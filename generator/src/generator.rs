use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::Write,
};

use anyhow::{anyhow, Result};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    helpers::{longuest_common_prefix, screaming_snake_to_pascal_case},
    xml::{self, RequireEnum, RequireType},
};

pub struct Generator<'a> {
    registry: &'a xml::Registry,
    enums: HashMap<&'a str, Enum<'a>>,
    constants: HashMap<&'a str, Constant<'a>>,
    handles: HashMap<&'a str, Handle<'a>>,
    listed_enum: RefCell<HashSet<String>>,
    mapping: RefCell<HashMap<&'a str, String>>,
}

impl<'a> Generator<'a> {
    pub fn new(_api: Api, registry: &xml::Registry) -> Result<Generator> {
        if registry
            .enums
            .get(0)
            .ok_or_else(|| anyhow!("Registry enum is empty"))?
            .name
            != "API Constants"
        {
            return Err(anyhow!(
                "First field of registry enums should be API Constants, instead got {}",
                registry.enums[0].name
            ));
        }

        let constants = registry.enums[0]
            .enums
            .iter()
            .map(|cst| Ok((cst.name.as_str(), Constant::try_from(cst)?)))
            .collect::<Result<_>>()?;

        let enums = registry
            .enums
            .iter()
            .skip(1)
            .map(|it| Ok((it.name.as_str(), Enum::try_from(it)?)))
            .collect::<Result<_>>()?;

        let handles = registry
            .types
            .iter()
            .map(|tys| &tys.types)
            .flatten()
            .filter(|ty| {
                ty.category.as_ref().is_some_and(|cat| cat == "handle") && ty.alias.is_none()
            })
            .map(|ty| {
                let handle = Handle::try_from(ty)?;
                Ok((handle.vk_name, handle))
            })
            .collect::<Result<_>>()?;

        let gen = Generator {
            registry,
            enums,
            constants,
            handles,
            listed_enum: RefCell::new(HashSet::new()),
            mapping: RefCell::new(HashMap::new()),
        };
        gen.extend_enums()?;
        gen.extend_handles()?;

        Ok(gen)
    }

    pub fn generate_enums(&self) -> Result<String> {
        self.listed_enum.borrow_mut().clear();

        let feature_enums = self
            .registry
            .features
            .iter()
            .map(|feature| self.generate_group_enums(&feature.name, &feature.require))
            .collect::<Result<Vec<_>>>()?;

        let extension_enums = self
            .registry
            .extensions
            .iter()
            .map(|exts| &exts.extension)
            .flatten()
            .map(|ext| self.generate_group_enums(&ext.name, &ext.require))
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            use bitflags::bitflags;

            #(#feature_enums)*
            #(#extension_enums)*
        }
        .to_string();

        let formatted_result = Self::format_result(result)?;
        Ok(Self::bitflag_format_fixup(formatted_result))
    }

    pub fn generate_handles(&self) -> Result<String> {
        let mapping = self.mapping.borrow();
        let handles = self
            .handles
            .iter()
            .map(|(_, handle)| {
                if handle.object_type == "VK_OBJECT_TYPE_SEMAPHORE_SCI_SYNC_POOL_NV" {
                    // annoying VulkanSC handle (we don't have its object type defined)
                    return Ok(quote! {});
                }

                let name = format_ident!("{}", handle.name);
                let dispatch_macro = if handle.is_dispatchable {
                    "handle_dispatchable"
                } else {
                    "handle_nondispatchable"
                };
                let dispatch_macro = format_ident!("{dispatch_macro}");
                let doc_tag = make_doc_link_inner(handle.vk_name);
                let object_type = &mapping
                        .borrow()
                        .get(handle.object_type)
                        .ok_or_else(|| {
                            anyhow!(
                                "Failed to find matching object type for {}",
                                handle.object_type
                            )
                        })?
                        // Remove the prefix
                        [("ObjectType::".len())..];
                let object_type = format_ident!("{object_type}");
                let aliases = handle
                    .aliases
                    .borrow()
                    .iter()
                    .map(|alias| {
                        let doc_tag = make_doc_link(&format!("Vk{alias}"));
                        let alias = format_ident!("{alias}");
                        quote! ( #doc_tag pub type #alias = #name; )
                    })
                    .collect::<Vec<_>>();

                Ok(quote! {
                    #dispatch_macro !{#name, #object_type, #doc_tag}
                    #(#aliases)*
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            use core::fmt;
            use std::num::{NonZeroUsize, NonZeroU64};

            use crate::{handle_dispatchable, handle_nondispatchable, vk_handle, private};
            use crate::{Handle, ObjectType};

            #(#handles)*
        }
        .to_string();

        let formatted_result = Self::format_result(result)?;
        Ok(formatted_result)
    }

    fn format_result(input: String) -> Result<String> {
        // This comes from a mix of looking at ash (https://github.com/ash-rs/ash/blob/660553c9184997c805c5a9f990395eab6d5e8dd4/generator/src/lib.rs#L3458)
        // and bindgen (https://docs.rs/bindgen/0.51.1/src/bindgen/lib.rs.html#1968)

        let mut cmd = std::process::Command::new("rustfmt")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let mut child_stdin = cmd
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Failed to take stdin"))?;
        let mut child_stdout = cmd
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to take stdout"))?;

        // Write to stdin in a new thread, so that we can read from stdout on this
        // thread. This keeps the child from blocking on writing to its stdout which
        // might block us from writing to its stdin.
        let stdin_handle = std::thread::spawn(move || {
            let _ = child_stdin.write_all(input.as_bytes());
            input
        });

        let mut output = vec![];
        std::io::copy(&mut child_stdout, &mut output)?;

        let status = cmd.wait()?;
        let _ = stdin_handle
            .join()
            .map_err(|_| anyhow!("Failed to join Rustfmt stdin handle"))?;

        let result = String::from_utf8(output)?;
        match status.code() {
            Some(0) => Ok(result),
            Some(2) => Err(anyhow!("Rustfmt parsing error")),
            Some(3) => Err(anyhow!("Rustfmt could not parse some lines")),
            _ => Err(anyhow!("Rustfmt internal error")),
        }
    }

    fn extend_enums(&self) -> Result<()> {
        // add constants to the mapping
        let mut mapping = self.mapping.borrow_mut();
        for (vk_name, constant) in &self.constants {
            let name = match constant {
                Constant::Aliased { name, .. } | Constant::Field { name, .. } => name,
            };
            mapping.insert(&vk_name, name.clone());
        }

        // add all aliases
        for ty in self.registry.types.iter().map(|t| &t.types).flatten() {
            if ty.name_attr.is_none() {
                continue;
            }

            match ty
                .category
                .as_ref()
                .map(|c| c.as_str())
                .zip(ty.alias.as_ref())
            {
                Some(("enum", alias)) => {
                    let parent = self
                        .enums
                        .borrow()
                        .get(alias.as_str())
                        .ok_or_else(|| anyhow!("Failed to find aliased enum {alias}"))?;
                    // check for ty.name_attr emptiness has been done above
                    let ty_name = ty.name_attr.as_ref().unwrap().as_str();
                    let parsed_name = Enum::parse_name(ty_name);
                    parent.aliases.borrow_mut().push((ty_name, parsed_name));
                }
                _ => {}
            }
        }

        // Add missing enum fields from the new features and extensions back to the enums
        // we also need while flattening everything to keep track of the extension number, which complicates everything
        let requires_ext = self
            .filtered_extensions()
            .map(|ext| ext.require.iter().map(|req| (Some(ext.number), req)))
            .flatten();
        let requires = self
            .filtered_features()
            .map(|feat| feat.require.iter().map(|req| (None, req)))
            .flatten()
            .chain(requires_ext);

        let enum_extends = requires
            .map(|(nb, req)| req.content.iter().map(move |cnt| (nb.clone(), cnt)))
            .flatten()
            .filter_map(|(nb, el)| match el {
                xml::RequireContent::Enum(req_enum) if req_enum.extends.is_some() => {
                    Some((nb, req_enum))
                }
                _ => None,
            });

        for (ext_number, ext) in enum_extends {
            // we checked above that extends.is_some()
            let parent_name = ext.extends.as_ref().unwrap();
            let name = convert_field_to_snake_case(&parent_name, &ext.name)?;

            let parent = self
                .enums
                .borrow()
                .get(parent_name.as_str())
                .ok_or_else(|| anyhow!("Parent {parent_name} for {} does not exists", ext.name))?;

            // in case a same field is redefined by multiple extensions (can happen)
            if parent
                .values
                .borrow()
                .iter()
                .any(|(ext_name, _)| ext_name == &ext.name)
            {
                continue;
            }

            if let Some(offset) = &ext.offset {
                let ext_number = match (ext.extnumber, ext_number) {
                    (Some(nb), _) | (_, Some(nb)) if nb > 0 => nb,
                    _ => {
                        return Err(anyhow!(
                            "Failed to find an extension number for {}",
                            ext.name
                        ))
                    }
                };

                let value = 1_000_000_000 + (ext_number - 1) * 1000 + offset;
                let value = if ext.dir.is_some() {
                    // dir = "-", negative value
                    format!("-{value}")
                } else {
                    value.to_string()
                };
                parent.values.borrow_mut().push((
                    &ext.name,
                    EnumValue::Variant(EnumVariant {
                        name,
                        value: Cow::Owned(value),
                    }),
                ));
            } else if let Some(value) = &ext.value {
                parent.values.borrow_mut().push((
                    &ext.name,
                    EnumValue::Variant(EnumVariant {
                        name,
                        value: Cow::Borrowed(value),
                    }),
                ));
            } else if let Some(bitpos) = ext.bitpos {
                parent
                    .values
                    .borrow_mut()
                    .push((&ext.name, EnumValue::Flag(EnumFlag { name, bitpos })))
            } else if let Some(alias) = &ext.alias {
                parent
                    .values
                    .borrow_mut()
                    .push((&ext.name, EnumValue::Aliased(EnumAliased { name, alias })));
            }
        }

        // add all enum fields / eliases to the mapping
        for (enum_vk_name, enum_decl) in self.enums.iter() {
            mapping.insert(enum_vk_name, enum_decl.name.clone());

            // add all fields
            for (field_vk_name, field) in enum_decl.values.borrow().iter() {
                let name = match field {
                    EnumValue::Aliased(EnumAliased { name, .. })
                    | EnumValue::Variant(EnumVariant { name, .. })
                    | EnumValue::Flag(EnumFlag { name, .. }) => name,
                };
                let full_name = format!("{}::{name}", enum_decl.name);
                mapping.insert(field_vk_name, full_name);
            }

            // add all aliases
            for (alias_vk_name, alias_name) in enum_decl.aliases.borrow().iter() {
                mapping.insert(alias_vk_name, alias_name.clone());
            }
        }

        Ok(())
    }

    fn extend_handles(&self) -> Result<()> {
        let mut mapping = self.mapping.borrow_mut();

        for handle in self.all_types().filter(|ty| {
            ty.category.as_ref().is_some_and(|cat| cat == "handle") && ty.alias.is_some()
        }) {
            // checked above
            let alias = handle.alias.as_ref().unwrap().as_str();
            let vk_name = &handle
                .name_attr
                .as_ref()
                .ok_or_else(|| anyhow!("Expected a name for {:?}", handle))?
                .as_str();

            // Remove the Vk prefix
            let name = &vk_name[2..];

            // add the alias to the mapping
            mapping.insert(vk_name, name.to_string());

            self.handles
                .borrow()
                .get(alias)
                .ok_or_else(|| anyhow!("Failed to find aliased enum for {:?}", handle))?
                .aliases
                .borrow_mut()
                .push(name);
        }

        // add all handles to the mapping
        for (vk_name, handle) in &self.handles {
            mapping.insert(vk_name, handle.name.to_string());
        }

        Ok(())
    }

    // remove VulkanSC only features
    fn filtered_features(&self) -> impl Iterator<Item = &'a xml::Feature> {
        self.registry
            .features
            .iter()
            .filter(|feat| feat.api.is_empty() || feat.api.contains(&xml::Api::Vulkan))
    }

    // remove VulkanSC only extensions
    fn filtered_extensions(&self) -> impl Iterator<Item = &'a xml::Extension> {
        self.registry
            .extensions
            .iter()
            .map(|exts| &exts.extension)
            .flatten()
            .filter(|ext| ext.supported.contains(&xml::ExtensionSupported::Vulkan))
    }

    fn all_types(&self) -> impl Iterator<Item = &'a xml::Type> {
        self.registry.types.iter().map(|ty| &ty.types).flatten()
    }

    fn generate_group_enums(
        &self,
        _group_name: &str,
        require: &Vec<xml::Require>,
    ) -> Result<TokenStream> {
        let content = require
            .iter()
            .map(|req| &req.content)
            .flatten()
            .collect::<Vec<_>>();
        let group_enums = content
            .into_iter()
            .filter_map(|cnt| match cnt {
                xml::RequireContent::Enum(RequireEnum { name, .. })
                | xml::RequireContent::Type(RequireType { name, .. }) => {
                    if !self.listed_enum.borrow().contains(name) {
                        Some(name.as_str())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .filter_map(|name| {
                let result = if let Some(constant) = self.constants.borrow().get(name) {
                    Some(self.generate_constant(name, constant))
                } else if let Some(value) = self.enums.borrow().get(name) {
                    Some(self.generate_enum(name, value))
                } else {
                    None
                };
                if result.is_some() {
                    self.listed_enum.borrow_mut().insert(name.to_string());
                }
                result
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            #(#group_enums)*
        })
    }

    fn generate_constant(&self, const_name: &str, value: &Constant) -> Result<TokenStream> {
        let doc_tag = make_doc_link(const_name);
        match value {
            Constant::Aliased { name, alias } => {
                let (alias, ty) = self
                    .constants
                    .borrow()
                    .get(alias)
                    .map(|c| {
                        if let Constant::Field { ref name, ty, .. } = c {
                            Some((name, ty))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(None)
                    .ok_or_else(|| anyhow!("Aliased constant {alias} does not exist"))?;
                let name = format_ident!("{name}");
                let value = format_ident!("{alias}");
                let ty: Ident = ty.into();

                Ok(quote! { #doc_tag pub const #name: #ty = #value; })
            }
            Constant::Field { name, ty, value } => {
                let name = format_ident!("{name}");
                let value = parse_value(value, *ty);
                let ty: Ident = ty.into();

                Ok(quote! { #doc_tag pub const #name: #ty = #value; })
            }
        }
    }

    fn generate_enum(&self, enum_name: &str, value: &Enum) -> Result<TokenStream> {
        let has_negative = value.values.borrow().iter().any(|(_, field)| match field {
            EnumValue::Variant(EnumVariant { value, .. }) => value.starts_with("-"),
            _ => false,
        });
        let ty: CType = match (value.width_is_64, has_negative) {
            (false, false) => CType::Uint32,
            (false, true) => CType::Int32,
            (true, false) => CType::Uint64,
            (true, true) => CType::Int64,
        };
        let ty_token: Ident = (&ty).into();
        let is_bitflag = value.bitflag;
        let enum_fields = value.values.borrow();

        // Make sure an enum value is not declared twice
        // Can happen with VK_SURFACE_COUNTER_VBLANK_BIT_EXT and VK_SURFACE_COUNTER_VBLANK_EXT both resolving to Vblank
        let mut found_values = HashSet::new();

        let fields = enum_fields
            .iter()
            .map(|(_, field)| match field {
                EnumValue::Aliased(EnumAliased { name, alias }) => {
                    if found_values.contains(name) {
                        // duplicate field
                        return Ok((true, quote! {}));
                    }

                    let alias = enum_fields
                        .iter()
                        .find(|(name, _)| name == alias)
                        .map(|(_, e)| match e {
                            EnumValue::Variant(EnumVariant { name, .. })
                            | EnumValue::Flag(EnumFlag { name, .. })
                            | EnumValue::Aliased(EnumAliased { name, .. }) => Some(name),
                        })
                        .unwrap_or(None)
                        .ok_or_else(|| anyhow!("Aliased enum {alias} does not exist"))?;

                    let name = format_ident!("{name}");
                    let value = format_ident!("{alias}");
                    if is_bitflag {
                        Ok((true, quote! {const #name = Self::#value.bits(); }))
                    } else {
                        Ok((false, quote! {pub const #name : Self = Self::#value;}))
                    }
                }
                EnumValue::Variant(EnumVariant { name, value }) => {
                    found_values.insert(name);
                    let name = format_ident!("{name}");
                    let value = parse_value(value, ty);

                    if is_bitflag {
                        Ok((true, quote! {const #name = #value;}))
                    } else {
                        Ok((true, quote! {#name = #value,}))
                    }
                }
                EnumValue::Flag(EnumFlag { name, bitpos }) => {
                    found_values.insert(name);
                    let name = format_ident!("{name}");
                    let value: TokenStream =
                        format!("1{} << {}", ty_token, bitpos).parse().unwrap();

                    Ok((true, quote! {const #name = #value;}))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let (in_decls, in_impls): (Vec<_>, Vec<_>) =
            fields.into_iter().partition(|(in_decl, _)| *in_decl);
        let (_, in_decls): (Vec<_>, Vec<_>) = in_decls.into_iter().unzip();
        let (_, in_impls): (Vec<_>, Vec<_>) = in_impls.into_iter().unzip();

        let (struct_ty, pre_qualifier, post_qualifier) = if is_bitflag {
            (quote! {struct}, None, Some(quote! { : #ty_token }))
        } else {
            (quote! {enum}, Some(quote! {#[repr(#ty_token)]}), None)
        };

        let name = format_ident!("{}", value.name);
        let aliases = value
            .aliases
            .borrow()
            .iter()
            .map(|(alias_name, alias)| {
                let alias = format_ident!("{alias}");
                let doc_tag = make_doc_link(alias_name);
                quote! (#doc_tag pub type #alias = #name;)
            })
            .collect::<Vec<_>>();
        let doc_tag = make_doc_link(enum_name);
        let result = quote! {
            #[derive(Debug, Clone, Copy)]
            #doc_tag
            #pre_qualifier
            pub #struct_ty #name #post_qualifier {
                #(#in_decls)*
            }
        };

        if is_bitflag {
            Ok(quote! {
                bitflags! {
                    #result
                }
                #(#aliases)*
            })
        } else if !in_impls.is_empty() {
            Ok(quote! {
                #result

                #[allow(non_upper_case_globals)]
                impl #name {
                    #(#in_impls)*
                }
                #(#aliases)*
            })
        } else {
            Ok(result)
        }
    }

    fn bitflag_format_fixup(formatted_result: String) -> String {
        // Rustfmt does not support formatting macros like bitflags! {..}
        // so do it ourself
        // the result is already formatted, so each bitflags struct occupies exactly one line
        let mut lines = formatted_result
            .split('\n')
            .map(|l| Cow::Borrowed(l))
            .collect::<Vec<_>>();
        for line in &mut lines {
            if !line.starts_with("bitflags!") {
                continue;
            }

            let mut bitflag = line.clone().into_owned();
            // remove the spaces before a lot of tokens
            for token in [",", ";", "(", ".", "const", "[", "::", "bits", "pub"] {
                bitflag = bitflag.replace(&format!(" {token}"), token);
            }
            // remove the spaces after some other tokens
            for token in ["::", ";", "{", "}"] {
                bitflag = bitflag.replace(&format!("{token} "), token);
            }
            // add newlines at the correct location, before these tokens (along with one indentation)
            for token in ["#[", "pub ", "const ", "}"] {
                bitflag = bitflag.replace(token, &format!("\n    {token}"));
            }
            // add another indentation for fields
            bitflag = bitflag.replace("const ", "    const ");
            // remove the indentation for the bitflags! closing bracket
            bitflag.replace_range((bitflag.len() - 6)..(bitflag.len() - 2), "");

            let _ = std::mem::replace(line, Cow::Owned(bitflag));
        }

        lines.join("\n")
    }
}

pub enum Api {
    Vulkan,
    //VulkanSc
}

struct Enum<'a> {
    name: String,
    bitflag: bool,
    width_is_64: bool,
    values: RefCell<Vec<(&'a str, EnumValue<'a>)>>,
    aliases: RefCell<Vec<(&'a str, String)>>,
}

impl<'a> Enum<'a> {
    fn parse_name(name: &str) -> String {
        // remove the Vk prefix
        // and replace FlagBits by Flags
        // VkMemoryAllocateFlagBitsKHR -> MemoryAllocateFlagsKHR
        name[2..].replace("FlagBits", "Flags")
    }
}

impl<'a> TryFrom<&'a xml::Enums> for Enum<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Enums) -> Result<Self> {
        let values = value
            .enums
            .iter()
            .map(|val| Ok((val.name.as_str(), EnumValue::try_from(val, &value.name)?)))
            .collect::<Result<_>>()?;

        let bitflag: bool = matches!(value.ty, Some(xml::EnumsType::Bitmask));

        let name = Self::parse_name(&value.name);
        Ok(Enum {
            name,
            bitflag,
            width_is_64: value.bitwidth_is_64.is_some(),
            values: RefCell::new(values),
            aliases: RefCell::new(Vec::new()),
        })
    }
}

enum EnumValue<'a> {
    Aliased(EnumAliased<'a>),
    Variant(EnumVariant<'a>),
    Flag(EnumFlag),
}

impl<'a> EnumValue<'a> {
    fn try_from(value: &'a xml::Enum, container_name: &'a str) -> Result<Self> {
        match &value {
            xml::Enum {
                name,
                alias: Some(alias),
                ..
            } => {
                let name = convert_field_to_snake_case(&container_name, name)?;
                Ok(EnumValue::Aliased(EnumAliased { name, alias }))
            }
            xml::Enum {
                name,
                value: Some(value),
                ..
            } => {
                let name = convert_field_to_snake_case(&container_name, name)?;
                Ok(EnumValue::Variant(EnumVariant {
                    name,
                    value: Cow::Borrowed(value),
                }))
            }
            xml::Enum {
                name,
                bitpos: Some(bitpos),
                ..
            } => {
                let name = convert_field_to_snake_case(&container_name, name)?;
                Ok(EnumValue::Flag(EnumFlag {
                    name,
                    bitpos: *bitpos,
                }))
            }
            _ => Err(anyhow!("XML enum {:?} is ill-formed for an enum", value)),
        }
    }
}

struct EnumAliased<'a> {
    name: String,
    alias: &'a str,
}

struct EnumVariant<'a> {
    name: String,
    value: Cow<'a, str>,
}

struct EnumFlag {
    name: String,
    bitpos: u8,
}

enum Constant<'a> {
    Aliased {
        name: String,
        alias: &'a str,
    },
    Field {
        name: String,
        ty: CType,
        value: &'a str,
    },
}

impl<'a> TryFrom<&'a xml::Enum> for Constant<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Enum) -> Result<Self, Self::Error> {
        match &value {
            xml::Enum {
                name,
                alias: Some(alias),
                ..
            } => {
                // same as below
                let name = name[("VK_".len())..].to_owned();
                Ok(Constant::Aliased { name, alias })
            }
            xml::Enum {
                name,
                ty: Some(ty),
                value: Some(value),
                ..
            } => {
                // this is a real constant, use the appropriate rust case
                let name = name[("VK_".len())..].to_owned();
                let ty: CType = (ty as &str).try_into()?;
                Ok(Constant::Field { name, ty, value })
            }
            _ => Err(anyhow!("XML enum {:?} is ill-formed for a constant", value)),
        }
    }
}

struct Handle<'a> {
    name: &'a str,
    vk_name: &'a str,
    is_dispatchable: bool,
    parent: Option<&'a str>,
    object_type: &'a str,
    aliases: RefCell<Vec<&'a str>>,
}

impl<'a> TryFrom<&'a xml::Type> for Handle<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Type) -> Result<Self> {
        match &value {
            xml::Type {
                category: Some(cat),
                parent,
                alias: None,
                content,
                object_type_enum: Some(object_type),
                ..
            } if cat == "handle" => {
                match &content[..] {
                    // content should be
                    // <type>VK_DEFINE_HANDLE</type>(<name>VkInstance</name>)
                    // or
                    // <type>VK_DEFINE_NON_DISPATCHABLE_HANDLE</type>(<name>VkBuffer</name>)
                    [xml::TypeContent::Type(ty), xml::TypeContent::Text(_), xml::TypeContent::Name(name), xml::TypeContent::Text(_)] =>
                    {
                        let is_dispatchable = match ty.as_str() {
                            "VK_DEFINE_HANDLE" => true,
                            "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => false,
                            _ => return Err(anyhow!("Unknown type {ty} for {:?}", value)),
                        };
                        let vk_name = name.as_str();
                        // remove the Vk Prefix
                        let name = &name[2..];
                        Ok(Handle {
                            name,
                            vk_name,
                            is_dispatchable,
                            object_type: object_type.as_str(),
                            parent: parent.as_ref().map(|p| p.as_str()),
                            aliases: RefCell::new(Vec::new()),
                        })
                    }
                    _ => Err(anyhow!(
                        "Failed to parse content as handle from {:?}",
                        value
                    )),
                }
            }
            _ => Err(anyhow!("Failed to extract handle from {:?}", value)),
        }
    }
}

#[derive(Clone, Copy)]
enum CType {
    Uint32,
    Int32,
    Uint64,
    Int64,
    Float,
}

impl TryFrom<&str> for CType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "uint32_t" => Ok(CType::Uint32),
            "int32_t" => Ok(CType::Int32),
            "uint64_t" => Ok(CType::Uint64),
            "int64_t" => Ok(CType::Int64),
            "float" => Ok(CType::Float),
            _ => Err(anyhow!("Unknown c type {value}")),
        }
    }
}

impl Into<Ident> for &CType {
    fn into(self) -> Ident {
        let ident_str = match self {
            CType::Uint32 => "u32",
            CType::Int32 => "i32",
            CType::Uint64 => "u64",
            CType::Int64 => "i64",
            CType::Float => "f32",
        };
        Ident::new(ident_str, Span::call_site())
    }
}

/// Performs screaming snake case to pascal case conversion
/// We only keep the part of the field not already in the container:
/// VK_PRIMITIVE_TOPOLOGY_POINT_LIST => PointList
///
/// If the container name has an extension name, it is stripped of the field name:
/// VK_BLEND_OVERLAP_UNCORRELATED_EXT => Uncorrelated (container name is VkBlendOverlapEXT)
/// VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR => RayTracingKHR (Khr is kept because the container name VkPipelineBindPoint has no Khr)
///
/// For bitflags, the _BIT part is removed
/// VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT => SampledImage
///
/// Because rust enums can't start with a number, it will be removed and instead some prefix based
/// on the container name will be added:
/// VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV => Rate16InvocationsPerPixel
/// VK_SAMPLE_COUNT_4_BIT => Count4
fn convert_field_to_snake_case(container_name: &str, field_name: &str) -> Result<String> {
    // try to detect extensions NV, KHR, HUAWEI...
    let post_extension_size = container_name
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_uppercase())
        .count();
    let post_extension = &container_name[(container_name.len() - post_extension_size)..];
    let field_name = if post_extension.len() >= 2 && field_name.ends_with(post_extension) {
        // No extension has one letter
        // also removing the underscore before the extension
        &field_name[..(field_name.len() - post_extension.len() - 1)]
    } else {
        field_name
    };
    let result = screaming_snake_to_pascal_case(field_name);

    let container_simplified = if let Some(pos) = container_name.find("FlagBits") {
        &container_name[..pos]
    } else if post_extension_size >= 2 {
        &container_name[..(container_name.len() - post_extension_size)]
    } else {
        &container_name[..]
    };
    let prefix = longuest_common_prefix(container_simplified, &result);

    // remove the prefix
    let mut result = result[prefix.len()..].to_string();
    if post_extension_size >= 2 {}
    if let Some(pos) = container_name.find("FlagBits") {
        // remove the 'Bit' part
        if let Some(pos) = result.rfind("Bit") {
            result.replace_range(pos..(pos + "Bit".len()), "");
        }

        // the enum number is a bit all over the place (VkBufferUsageFlagBits2KHR -> VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR)
        // handle it here
        let nb_between = container_name.len() - (pos + "FlagBits".len() + post_extension_size);
        if nb_between == 1
            && container_name
                .chars()
                .nth_back(post_extension_size)
                .unwrap()
                .is_numeric()
        {
            //go from 2TransferDst to TransferDst
            result = result[1..].to_string();
        } else if nb_between >= 2 {
            return Err(anyhow!(
                "Unexpected name for {field_name} and {container_name}"
            ));
        }
    }

    let prefix = if result
        .chars()
        .next()
        .ok_or_else(|| anyhow!("The resulting field from {field_name} is an empty string"))?
        .is_numeric()
    {
        // we can't let it start with a number
        // add something relevant in front
        [
            "Type", "Count", "Depth", "Size", "Rate", "Format", "Result", "Controls", "Chroma",
            "Image",
        ]
        .into_iter()
        .find(|kw| container_name.contains(kw))
        .ok_or_else(|| {
            anyhow!(
                "Failed to find a relevant keyword to add in front of {result} for {field_name}"
            )
        })?
    } else {
        ""
    };

    Ok(format!("{prefix}{result}"))
}

fn parse_value(value: &str, ty: CType) -> TokenStream {
    // bitwise negation is different
    let value: String = value.replace("~", "!");
    let mut rust_value = match ty {
        CType::Uint32 => value.replace("U", "u32"),
        CType::Uint64 => value.replace("ULL", "u64"),
        CType::Float => value.replace("F", "f32"),
        CType::Int32 | CType::Int64 => value,
    };
    if rust_value.starts_with("(") && rust_value.ends_with(")") {
        rust_value = rust_value[1..(rust_value.len() - 1)].to_owned();
    }
    rust_value.parse().unwrap()
}

fn get_doc_url(item_name: &str) -> String {
    format!(
        "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{item_name}.html>"
    )
}

fn make_doc_link(item_name: &str) -> TokenStream {
    let doc_name = get_doc_url(item_name);
    quote! (#[doc = #doc_name])
}

fn make_doc_link_inner(item_name: &str) -> TokenStream {
    let doc_name = get_doc_url(item_name);
    quote! (doc = #doc_name)
}
