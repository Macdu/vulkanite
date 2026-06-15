use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::HashMap,
    io::Write,
};

use anyhow::{anyhow, bail, Context, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    helpers::camel_case_to_snake_case,
    structs::{
        convert_field_to_snake_case, is_subfeature, remove_featext_prefix, AdvancedType, Api,
        CType, Command, CommandParam, CommandParamsParsed, Constant, Dependencies, Enum,
        EnumAliased, EnumFlag, EnumValue, EnumVariant, ExtFeature, Handle, MappingEntry,
        MappingType, SliceType, Struct, StructBasetype, StructStandard, Type,
    },
    xml,
};

mod advanced_commands;
mod dispatcher;
mod enums;
mod extensions;
mod features;
mod formats;
mod handles;
mod raw_commands;
mod structs;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GeneratedCommandType {
    Basic,
}

pub struct Generator<'a> {
    registry: &'a xml::Registry,
    vendor_names: Vec<&'a str>,
    extensions_features: HashMap<&'a str, ExtFeature<'a>>,
    enums: HashMap<&'a str, Enum<'a>>,
    constants: HashMap<&'a str, Constant<'a>>,
    handles: HashMap<&'a str, Handle<'a>>,
    structs: HashMap<&'a str, Struct<'a>>,
    commands: HashMap<&'a str, Command<'a>>,
    mapping: RefCell<HashMap<&'a str, MappingEntry<'a>>>,
}

impl<'a> Generator<'a> {
    pub fn new(_api: Api, registry: &'a xml::Registry) -> Result<Self> {
        if registry
            .enums
            .get(0)
            .context("Registry enum is empty")?
            .name
            != "API Constants"
        {
            return Err(anyhow!(
                "First field of registry enums should be API Constants, instead got {}",
                registry.enums[0].name
            ));
        }

        let features_list = Self::filter_features(&registry.features).filter_map(|feat| {
            if is_subfeature(&feat.name) {
                // Only keep the main version as a dependency
                return None;
            }
            let feat_name = remove_featext_prefix(&feat.name);
            Some(Ok((
                feat_name,
                ExtFeature {
                    name: feat_name.to_ascii_lowercase(),
                    is_non_trivial: false.into(),
                    dependencies: Dependencies::None,
                },
            )))
        });
        let extensions_list = Self::filter_extensions(&registry.extensions).map(|ext| {
            let ext_simplified = remove_featext_prefix(&ext.name);
            let dependencies = ext
                .depends
                .as_ref()
                .map(|dep| Dependencies::parse(&dep))
                .transpose()?
                .unwrap_or_default();
            Ok((
                ext_simplified,
                ExtFeature {
                    name: format!("ext_{ext_simplified}"),
                    is_non_trivial: false.into(),
                    dependencies,
                },
            ))
        });
        let extensions_features = features_list
            .chain(extensions_list)
            .collect::<Result<_>>()?;

        let vendor_names: Vec<_> = registry
            .tags
            .iter()
            .flat_map(|tags| &tags.tag)
            .map(|tag| tag.name.as_str())
            .collect();

        let constants = registry.enums[0]
            .enums
            .iter()
            .map(|cst| Ok((cst.name.as_str(), Constant::try_from(cst)?)))
            .collect::<Result<_>>()?;

        let enums = registry
            .enums
            .iter()
            // Skip the constants from above
            .skip(1)
            .map(|it| {
                let mut my_enum = Enum::try_from(it, &vendor_names)?;
                if it.name == "VkResult" {
                    // Rename Result as Status so that vk::Result<A> = std::Result<A, vk::Status>
                    my_enum.name = "Status".to_owned();
                }
                Ok((it.name.as_str(), my_enum))
            })
            .collect::<Result<_>>()?;

        let all_types = registry.types.iter().flat_map(|tys| &tys.types);

        let find_name = |ty: &'a xml::TypeContent| match ty {
            xml::TypeContent::Name(name) => Some(name.as_str()),
            _ => None,
        };

        let handles = all_types
            .clone()
            .filter(|ty| {
                ty.category.as_ref().is_some_and(|cat| cat == "handle") && ty.alias.is_none()
            })
            .map(|ty| {
                let handle = Handle::try_from(ty)?;
                let name = ty.content.iter().find_map(find_name).unwrap();
                Ok((name, handle))
            })
            .collect::<Result<_>>()?;

        let structs = all_types
            .clone()
            .filter(|ty| {
                ty.category
                    .as_ref()
                    .is_some_and(|cat| cat == "basetype" || cat == "struct" || cat == "union")
                    && ty.alias.is_none()
            })
            .filter(|ty| !matches!(&ty.apis[..], [xml::Api::Vulkansc]))
            .map(|ty| {
                let my_struct = Struct::try_from(ty)?;
                let name = ty
                    .name_attr
                    .as_ref()
                    .map(|n| n.as_str())
                    // the fact try_from did not return error means this unwrap won't fail
                    .unwrap_or_else(|| ty.content.iter().find_map(find_name).unwrap());
                Ok((name, my_struct))
            })
            .collect::<Result<_>>()?;

        let commands = registry
            .commands
            .iter()
            .flat_map(|cmds| &cmds.command)
            .filter(|cmd| cmd.alias.is_none() && !matches!(&cmd.apis[..], [xml::Api::Vulkansc]))
            .map(|cmd| Command::try_from(cmd).map(|cmd| (cmd.vk_name, cmd)))
            .collect::<Result<_>>()?;

        let mapping: HashMap<&str, MappingEntry> = [
            ("uint8_t", "u8"),
            ("int8_t", "i8"),
            ("char", "c_char"),
            ("uint16_t", "u16"),
            ("int16_t", "i16"),
            ("uint32_t", "u32"),
            ("int32_t", "i32"),
            ("int", "c_int"),
            ("uint32_t", "u32"),
            ("int32_t", "i32"),
            ("uint64_t", "u64"),
            ("int64_t", "i64"),
            ("size_t", "usize"),
            ("float", "f32"),
            ("double", "f64"),
            ("void", "c_void"),
            // Types from external headers
            ("Window", "c_ulong"),
            ("xcb_window_t", "u32"),
            // Custom types for QoL improvements
            ("VoidPtr", "VoidPtr"),
            ("ApiVersion", "ApiVersion"),
            ("InstanceExtensionName", "InstanceExtensionName"),
            ("DeviceExtensionName", "DeviceExtensionName"),
            (
                "DebugUtilsMessengerCallbackEXT",
                "DebugUtilsMessengerCallbackEXT",
            ),
        ]
        .into_iter()
        .map(|(key, val)| {
            (
                key,
                MappingEntry {
                    name: val.to_owned(),
                    ty: MappingType::BaseType,
                },
            )
        })
        .collect();

        let gen = Generator {
            registry,
            vendor_names,
            extensions_features,
            enums,
            constants,
            handles,
            structs,
            commands,
            mapping: RefCell::new(mapping),
        };

        gen.extend_enums()?;
        gen.extend_handles()?;
        gen.extend_structs()?;
        gen.extend_commands()?;
        gen.find_dependencies()?;

        Ok(gen)
    }

    pub fn generate_features(&self, cargo_file: String) -> Result<String> {
        features::generate(self, cargo_file)
    }

    pub fn generate_extensions(&self) -> Result<String> {
        extensions::generate(self)
    }

    pub fn generate_enums(&self) -> Result<String> {
        enums::generate(self)
    }

    pub fn generate_handles(&self) -> Result<String> {
        handles::generate(self)
    }

    pub fn generate_structs(&self) -> Result<String> {
        structs::generate(self)
    }

    pub fn generate_formats(&self) -> Result<String> {
        formats::generate(self)
    }

    pub fn generate_dispatcher(&self) -> Result<String> {
        dispatcher::generate(self)
    }

    pub fn generate_raw_commands(&self) -> Result<String> {
        raw_commands::generate(self)
    }

    pub fn generate_advanced_commands<'b>(
        &'b self,
        gen_ty: GeneratedCommandType,
    ) -> Result<String> {
        advanced_commands::generate(self, gen_ty)
    }

    fn format_result(input: String) -> Result<String> {
        // This comes from a mix of looking at ash (https://github.com/ash-rs/ash/blob/660553c9184997c805c5a9f990395eab6d5e8dd4/generator/src/lib.rs#L3458)
        // and  (https://docs.rs/bindgen/0.51.1bindgen/src/bindgen/lib.rs.html#1968)

        let mut cmd = std::process::Command::new("rustfmt")
            .args(["--edition", "2021"])
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
            Some(2) => bail!("Rustfmt parsing error"),
            Some(3) => bail!("Rustfmt could not parse some lines"),
            _ => bail!("Rustfmt internal error"),
        }
    }

    fn extend_enums(&self) -> Result<()> {
        // add constants to the mapping
        let mut mapping = self.mapping.borrow_mut();
        for (vk_name, constant) in &self.constants {
            let name = match constant {
                Constant::Aliased { name, .. } | Constant::Field { name, .. } => name,
            };
            mapping.insert(
                &vk_name,
                MappingEntry {
                    name: name.clone(),
                    ty: MappingType::Constant,
                },
            );
        }

        // add all aliases
        for ty in self.all_types() {
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
            .flat_map(|ext| ext.require.iter().map(|req| (Some(ext.number), req)));
        let requires = self
            .filtered_features()
            .flat_map(|feat| feat.requires().map(|req| (None, req)))
            .chain(requires_ext);

        let enum_extends = requires
            .flat_map(|(nb, req)| req.content.iter().map(move |cnt| (nb.clone(), cnt)))
            .filter_map(|(nb, el)| match el {
                xml::RequireContent::Enum(req_enum) if req_enum.extends.is_some() => {
                    Some((nb, req_enum))
                }
                _ => None,
            });

        for (ext_number, ext) in enum_extends {
            // we checked above that extends.is_some()
            let parent_name = ext.extends.as_ref().unwrap();
            let name = convert_field_to_snake_case(&parent_name, &ext.name, &self.vendor_names)?;

            let parent = self
                .enums
                .borrow()
                .get(parent_name.as_str())
                .ok_or_else(|| anyhow!("Parent {parent_name} for {} does not exists", ext.name))?;

            let mut values = parent.values.borrow_mut();
            // in case a same field is redefined by multiple extensions (can happen)
            if values.iter().any(|(ext_name, _)| ext_name == &ext.name) {
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
                values.push((
                    &ext.name,
                    EnumValue::Variant(EnumVariant {
                        name,
                        value: Cow::Owned(value),
                    }),
                ));
            } else if let Some(value) = &ext.value {
                values.push((
                    &ext.name,
                    EnumValue::Variant(EnumVariant {
                        name,
                        value: Cow::Borrowed(value),
                    }),
                ));
            } else if let Some(bitpos) = ext.bitpos {
                values.push((&ext.name, EnumValue::Flag(EnumFlag { name, bitpos })));
            } else if let Some(alias) = &ext.alias {
                if values
                    .iter()
                    .any(|(_, val)| matches!(val, EnumValue::Aliased(EnumAliased { name: other, .. }) if other == &name))
                {
                    continue;
                }

                values.push((&ext.name, EnumValue::Aliased(EnumAliased { name, alias })));
            }
        }

        // add all enum fields / aliases to the mapping
        for (enum_vk_name, enum_decl) in self.enums.iter() {
            assert!(mapping
                .insert(
                    enum_vk_name,
                    MappingEntry {
                        name: enum_decl.name.clone(),
                        ty: MappingType::Enum
                    }
                )
                .is_none());

            // add all fields
            for (field_vk_name, field) in enum_decl.values.borrow().iter() {
                let name = match field {
                    EnumValue::Aliased(EnumAliased { name, .. })
                    | EnumValue::Variant(EnumVariant { name, .. })
                    | EnumValue::Flag(EnumFlag { name, .. }) => name,
                };
                let full_name = format!("{}::{name}", enum_decl.name);
                assert!(mapping
                    .insert(
                        field_vk_name,
                        MappingEntry {
                            name: full_name,
                            ty: MappingType::EnumValue
                        }
                    )
                    .is_none());
            }

            // add all aliases
            for (alias_vk_name, alias_name) in enum_decl.aliases.borrow().iter() {
                assert!(mapping
                    .insert(
                        alias_vk_name,
                        MappingEntry {
                            name: alias_name.clone(),
                            ty: MappingType::EnumAlias(enum_vk_name)
                        }
                    )
                    .is_none());
            }
        }

        // add bitmask to the mapping (VkAccessFlagBits enum => VkAccessFlags bitmask)
        for bitmask in self
            .all_types()
            .filter(|ty| ty.category.as_ref().is_some_and(|cat| cat == "bitmask"))
        {
            if let Some(alias) = &bitmask.alias {
                let vk_name = bitmask
                    .name_attr
                    .as_ref()
                    .ok_or_else(|| anyhow!("Failed to get name for {alias}"))?;
                let alias_bits = alias.replace("Flags", "FlagBits");
                if let Some(enum_base) = self.enums.get(alias_bits.as_str()) {
                    // add the alias only if it does not exists already as the FlagBits version
                    let mut aliases = enum_base.aliases.borrow_mut();
                    // Remove the "Vk" prefix
                    let name = &vk_name[2..];
                    if !aliases.iter().any(|(_, alias_name)| alias_name == name) {
                        aliases.push((vk_name.as_str(), name.to_string()));
                    }
                    assert!(mapping
                        .insert(
                            vk_name,
                            MappingEntry {
                                name: name.to_owned(),
                                ty: MappingType::EnumAlias(alias.as_str())
                            }
                        )
                        .is_none());
                } else {
                    // alias of an empty bitmask
                    assert!(mapping
                        .insert(
                            vk_name,
                            MappingEntry {
                                name: "u32".to_owned(),
                                ty: MappingType::BaseType
                            }
                        )
                        .is_none());
                }
            } else {
                let name = bitmask
                    .content
                    .iter()
                    .find_map(|cnt| match cnt {
                        xml::TypeContent::Name(name) => Some(name.as_str()),
                        _ => None,
                    })
                    .ok_or_else(|| anyhow!("Failed to find name for bimask"))?;
                let value =
                    if let Some(req) = bitmask.requires.as_ref().or(bitmask.bitvalues.as_ref()) {
                        mapping
                            .get(req.as_str())
                            .ok_or_else(|| anyhow!("Failed to find bitflag {req}"))?
                            .clone()
                    } else {
                        // bitflag with no value defined yet (can only be 0)
                        let is_64 = bitmask.content.iter().any(
                            |cnt| matches!(cnt, xml::TypeContent::Type(ty) if ty == "VkFlags64"),
                        );
                        MappingEntry {
                            name: if is_64 { "u64" } else { "u32" }.to_owned(),
                            ty: MappingType::BaseType,
                        }
                    };
                assert!(mapping.insert(name, value).is_none());
            }
        }

        // workaround for the time being
        for funcptr in self.all_types().filter_map(|ty| match &ty {
            &xml::Type {
                category: Some(category),
                ..
            } if category == "funcpointer" => Some(ty),
            _ => None,
        }) {
            let name = funcptr
                .content
                .iter()
                .find_map(|cnt| match cnt {
                    xml::TypeContent::Proto(param) => Some(param),
                    _ => None,
                })
                .and_then(|param| {
                    param.content.iter().find_map(|cnt| match cnt {
                        xml::ParamContent::Name(name) => Some(name.as_str()),
                        _ => None,
                    })
                })
                .context("Failed to find name for funcptr")?;
            assert!(mapping
                .insert(
                    name,
                    MappingEntry {
                        name: "FuncPtr".to_owned(),
                        ty: MappingType::FunctionPtr
                    }
                )
                .is_none());
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
                .with_context(|| format!("Expected a name for {:?}", handle))?
                .as_str();

            // Remove the Vk prefix
            let name = &vk_name[2..];

            // add the alias to the mapping
            assert!(mapping
                .insert(
                    vk_name,
                    MappingEntry {
                        name: name.to_owned(),
                        ty: MappingType::HandleAlias(alias)
                    }
                )
                .is_none());

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
            assert!(mapping
                .insert(
                    vk_name,
                    MappingEntry {
                        name: handle.name.to_owned(),
                        ty: MappingType::Handle
                    }
                )
                .is_none());
        }

        Ok(())
    }

    fn extend_structs(&self) -> Result<()> {
        let mut mapping = self.mapping.borrow_mut();

        // add include types
        for ty in self.all_types() {
            match (&ty.requires, &ty.name_attr, &ty.category, &ty.alias) {
                (Some(_), Some(name), ..) => {
                    // only insert if it is not already custom defined
                    mapping.entry(name).or_insert_with(|| MappingEntry {
                        name: "VoidPtr".to_string(),
                        ty: MappingType::BaseType,
                    });
                }
                (_, Some(name), Some(cat), Some(alias)) if cat == "struct" => {
                    if let Struct::Standard(base_struct) = self
                        .structs
                        .get(alias.as_str())
                        .ok_or_else(|| anyhow!("Failed to find alias {alias} for {name}"))?
                    {
                        base_struct.aliases.borrow_mut().push(&name);
                    } else {
                        return Err(anyhow!("Defining an alias for a basetype is not handled"));
                    }
                }
                _ => {}
            }
        }

        // No need to put the fields, only put the definitions
        for (vk_name, my_struct) in &self.structs {
            let (name, ty) = match my_struct {
                Struct::BaseType(StructBasetype { name, .. }) => (name, MappingType::BaseType),
                Struct::Standard(StructStandard { name, .. }) => (name, MappingType::Struct),
            };
            assert!(mapping
                .insert(
                    vk_name,
                    MappingEntry {
                        name: name.clone(),
                        ty
                    }
                )
                .is_none());

            if let Struct::Standard(my_struct) = my_struct {
                for alias in my_struct.aliases.borrow().iter() {
                    // Remove the Vk prefix
                    let real_name = &alias[2..];
                    assert!(mapping
                        .insert(
                            alias,
                            MappingEntry {
                                name: real_name.to_owned(),
                                ty: MappingType::StructAlias(vk_name)
                            }
                        )
                        .is_none())
                }
            }
        }

        // we must drop the mutable reference before the next part
        drop(mapping);

        // register all lifetimes and field aspect
        for (vk_name, my_struct) in &self.structs {
            if my_struct.lifetime().is_none() {
                self.compute_name_lifetime(vk_name);
                assert!(my_struct.lifetime().is_some());
            }

            if let Struct::Standard(my_struct) = &my_struct {
                for field in &my_struct.fields {
                    field
                        .advanced_ty
                        .set(Some(self.compute_advanced_type(&field.ty)));
                }
            }
        }

        Ok(())
    }

    fn extend_commands(&self) -> Result<()> {
        // first register aliases
        for cmd in self.all_commands().filter(|cmd| cmd.alias.is_some()) {
            let original = self
                .commands
                .get(cmd.alias.as_ref().unwrap().as_str())
                .ok_or_else(|| anyhow!("Failed to find command {}", cmd.alias.as_ref().unwrap()))?;

            let vk_name = cmd.name.as_ref().unwrap().as_str();
            assert!(vk_name.starts_with("vk"));
            let name = camel_case_to_snake_case(&vk_name[2..]);
            original.aliases.borrow_mut().push((vk_name, name));
        }

        // then update the mapping
        let mut mapping = self.mapping.borrow_mut();
        for (vk_name, cmd) in &self.commands {
            assert!(mapping
                .insert(
                    vk_name,
                    MappingEntry {
                        name: cmd.name.clone(),
                        ty: MappingType::Command
                    }
                )
                .is_none());

            for (alias_vk_name, alias_name) in cmd.aliases.borrow().iter() {
                assert!(mapping
                    .insert(
                        alias_vk_name,
                        MappingEntry {
                            name: alias_name.clone(),
                            ty: MappingType::CommandAlias(vk_name)
                        }
                    )
                    .is_none());
            }
        }

        // we need to borrow the mapping in the next part
        std::mem::drop(mapping);

        // compute all advanced types
        for (_, cmd) in &self.commands {
            // no need to compute an advanced type for the return type (can only be void, result or a basetype)

            for param in &cmd.params {
                param
                    .advanced_ty
                    .set(Some(self.compute_advanced_type(&param.ty)));
            }

            if let Some(param) = cmd.params.first() {
                if let Some(AdvancedType::Handle(handle)) = param.advanced_ty.get() {
                    cmd.handle.set(Some(handle))
                }
            }
        }

        Ok(())
    }

    fn find_dependencies(&self) -> Result<()> {
        let mapping = self.mapping.borrow();
        let requires = self
            .filtered_extensions()
            .flat_map(|ext| {
                let name = remove_featext_prefix(&ext.name);
                ext.require.iter().map(move |req| (name, req))
            })
            .chain(self.filtered_features().flat_map(|feat| {
                feat.requires()
                    .map(|req| (remove_featext_prefix(&feat.name), req))
            }))
            .flat_map(|(name, req)| {
                let mut deps = Dependencies::Single(name);
                if let Some(depends) = &req.depends {
                    // TODO: don't use expect
                    let additional_deps =
                        Dependencies::parse(&depends).expect("Failed to parse dependencies");
                    deps.add_and(additional_deps);
                };
                req.content.iter().map(move |req| (name, deps.clone(), req))
            });

        for (ext_name, deps, req) in requires {
            let Some(ext_feat) = self.extensions_features.get(ext_name) else {
                bail!("Failed to find extension/feature {ext_name}");
            };
            let name = match req {
                xml::RequireContent::Type(req_ty) => &req_ty.name,
                xml::RequireContent::Command(command) => &command.name,
                _ => continue,
            };

            match mapping.get(name.as_str()) {
                Some(MappingEntry {
                    ty: MappingType::Struct | MappingType::StructAlias(_),
                    ..
                }) => {
                    let Some(my_struct) = self.get_struct(name) else {
                        bail!("Failed to find struct {name}")
                    };
                    my_struct.dependencies.borrow_mut().add_or(deps.clone());
                    if !name.contains("Features") && !name.contains("Properties") {
                        // An extension can be trivial if it only contains a features and properties definition
                        ext_feat.is_non_trivial.set(true);
                    }
                }
                Some(MappingEntry {
                    ty: MappingType::Handle | MappingType::HandleAlias(_),
                    ..
                }) => {
                    let Some(handle) = self.get_handle(name) else {
                        bail!("Failed to find handle {name}")
                    };
                    handle.dependencies.borrow_mut().add_or(deps.clone());
                    ext_feat.is_non_trivial.set(true);
                }
                Some(MappingEntry {
                    ty: MappingType::Enum | MappingType::EnumAlias(_),
                    ..
                }) => {
                    let my_enum = self
                        .get_enum(name)
                        .or_else(|| self.get_enum(&name.replace("Flags", "FlagBits")))
                        .with_context(|| format!("Failed to find enum {name}"))?;

                    my_enum.dependencies.borrow_mut().add_or(deps.clone());
                    ext_feat.is_non_trivial.set(true);
                }
                Some(MappingEntry {
                    ty: MappingType::Command | MappingType::CommandAlias(_),
                    ..
                }) => {
                    let Some(command) = self.get_command(name) else {
                        bail!("Failed to find command {name}")
                    };
                    command.dependencies.borrow_mut().add_or(deps.clone());
                    ext_feat.is_non_trivial.set(true);
                }
                _ => (),
            }
        }

        // We always have vulkan 1.0 enabled by default
        // so set this extension as trivial
        self.extensions_features
            .get("VERSION_1_0")
            .context("Failed to find vulkan 1.0")?
            .is_non_trivial
            .set(false);

        let get_dep = |struct_name: &str| -> Result<_> {
            Ok(self
                .get_struct(struct_name)
                .context("Failed to find struct")?
                .dependencies
                .borrow_mut())
        };

        // The VK_EXT_global_priority_query extension is trivial, but its properties struct
        // uses QueueGlobalPriority which is defined by one of its dependencies
        for struct_name in [
            "VkPhysicalDeviceGlobalPriorityQueryFeatures",
            "VkQueueFamilyGlobalPriorityProperties",
        ] {
            let mut my_struct = get_dep(struct_name)?;
            my_struct.clear();
            my_struct.add_or(Dependencies::Single("global_priority"));
            my_struct.add_or(Dependencies::Single("VK_VERSION_1_4"));
        }

        // Same for VkChromaLocation and VK_ANDROID_external_format_resolve
        for struct_name in [
            "VkPhysicalDeviceExternalFormatResolveFeaturesANDROID",
            "VkPhysicalDeviceExternalFormatResolvePropertiesANDROID",
        ] {
            let mut my_struct = get_dep(struct_name)?;
            my_struct.clear();
            my_struct.add_or(Dependencies::Single("sampler_ycbcr_conversion"));
            my_struct.add_or(Dependencies::Single("VK_VERSION_1_1"));
        }

        // VkBlendOverlapEXT is defined by VK_EXT_blend_operation_advanced but used by VK_EXT_extended_dynamic_state3 / VK_EXT_shader_object
        // I believe this is a bug in the spec..
        {
            let my_enum = self
                .get_enum("VkBlendOverlapEXT")
                .context("Failed to find enum")?;
            let mut deps = my_enum.dependencies.borrow_mut();
            deps.add_or(Dependencies::Single("extended_dynamic_state3"));
            deps.add_or(Dependencies::Single("shader_object"));
        }

        Ok(())
    }

    fn get_enum(&self, name: &str) -> Option<&Enum<'a>> {
        let enum_name = match self.mapping.borrow().get(name)?.ty {
            MappingType::Enum => name,
            MappingType::EnumAlias(alias) => alias,
            _ => return None,
        };
        self.enums.get(enum_name)
    }

    fn get_handle(&self, name: &str) -> Option<&Handle<'a>> {
        let handle_name = match self.mapping.borrow().get(name)?.ty {
            MappingType::Handle => name,
            MappingType::HandleAlias(alias) => alias,
            _ => return None,
        };
        self.handles.get(handle_name)
    }

    fn get_struct(&self, name: &str) -> Option<&StructStandard<'a>> {
        let struct_name = match self.mapping.borrow().get(name)?.ty {
            MappingType::Struct => name,
            MappingType::StructAlias(alias) => alias,
            _ => return None,
        };
        match self.structs.get(struct_name) {
            Some(Struct::Standard(my_struct)) => Some(my_struct),
            _ => None,
        }
    }

    fn get_command(&self, name: &str) -> Option<&Command<'a>> {
        let command_name = match self.mapping.borrow().get(name)?.ty {
            MappingType::Command => name,
            MappingType::CommandAlias(alias) => alias,
            _ => return None,
        };
        self.commands.get(command_name)
    }

    /// Return if the type ty needs to have a lifetime parameter
    /// i.e if the type is pointer to a sized type of a struct with a lifetime
    fn compute_type_lifetime(&self, ty: &Type) -> bool {
        match ty {
            Type::Void | Type::VoidPtr | Type::Bitfield { .. } => false,
            Type::Ptr(_) | Type::DoublePtr(_) | Type::CStrArr => true,
            Type::Path(ty)
            | Type::ArrayEnum { ty, .. }
            | Type::ArrayCst { ty, .. }
            | Type::ArrayDoubleCst { ty, .. } => self.compute_name_lifetime(ty),
        }
    }

    /// Return if the type name needs a lifetime
    /// i.e if the type is a base type whose type has a lifetime
    /// Or a standard struct and one of its type requires a lifetime
    fn compute_name_lifetime(&self, name: &str) -> bool {
        let struct_name = match self.mapping.borrow().get(name).map(|mp| mp.ty) {
            Some(MappingType::Struct | MappingType::BaseType) => name,
            Some(MappingType::StructAlias(alias)) => alias,
            Some(MappingType::Handle | MappingType::HandleAlias(_)) => return true,
            _ => return false,
        };
        let my_struct = match self.structs.get(struct_name) {
            Some(my_struct) => my_struct,
            None => return false,
        };

        if let Some(lifetime) = my_struct.lifetime() {
            return lifetime;
        }

        match my_struct {
            Struct::BaseType(base_type) => {
                let lifetime = self.compute_type_lifetime(&base_type.ty);
                base_type.has_lifetime.set(Some(lifetime));
                lifetime
            }
            Struct::Standard(union_type) if union_type.is_union => {
                let lifetime = union_type.fields.iter().any(|field| match field.ty {
                    // Unions use raw pointers, so for them add a lifetime only if the pointee requires one
                    Type::Ptr(name) => self.compute_name_lifetime(name),
                    _ => self.compute_type_lifetime(&field.ty),
                });
                union_type.has_lifetime.set(Some(lifetime));
                lifetime
            }
            Struct::Standard(standard_type) => {
                let lifetime = standard_type.s_type.is_some()
                    || standard_type.fields.iter().any(|field| {
                        let ty_has_lifetime = self.compute_type_lifetime(&field.ty);
                        // void* pData with a given length must also be given a lifetime
                        let is_data_ptr =
                            matches!(field.ty, Type::VoidPtr) && field.xml.len.is_some();
                        ty_has_lifetime || is_data_ptr
                    });
                standard_type.has_lifetime.set(Some(lifetime));
                lifetime
            }
        }
    }

    fn compute_advanced_type(&self, ty: &Type<'a>) -> AdvancedType<'a> {
        type AT<'a> = AdvancedType<'a>;
        match ty {
            Type::Void => AT::Void,
            Type::VoidPtr => AT::VoidPtr,
            Type::Path(name) => match self.mapping.borrow().get(name).map(|entry| entry.ty) {
                Some(MappingType::Struct | MappingType::StructAlias(_)) => AT::Struct(name),
                Some(MappingType::Handle | MappingType::HandleAlias(_)) => AT::Handle(name),
                Some(MappingType::Enum | MappingType::EnumAlias(_)) => AT::Enum(name),
                Some(MappingType::FunctionPtr) => AT::Func(name),
                Some(MappingType::BaseType) if *name == "VoidPtr" => AT::VoidPtr,
                Some(MappingType::BaseType) if *name == "VkBool32" => AT::Bool32,
                _ => AT::Other(name),
            },
            Type::Ptr(name) => {
                if self.mapping.borrow().get(name).is_some_and(|entry| {
                    matches!(entry.ty, MappingType::Handle | MappingType::HandleAlias(_))
                }) {
                    AT::HandlePtr(name)
                } else if *name == "char" {
                    AT::CString
                } else if *name == "void" {
                    AT::VoidPtr
                } else {
                    AT::OtherPtr(name)
                }
            }
            Type::DoublePtr(name) => AT::OtherDoublePtr(name),
            Type::CStrArr => AT::CStringPtr,
            Type::ArrayEnum { ty, size } => {
                if self.handles.contains_key(ty) {
                    AT::HandleArray(ty, size)
                } else if *ty == "char" {
                    AT::CharArray(size)
                } else {
                    AT::OtherArrayWithEnum(ty, size)
                }
            }
            Type::ArrayCst { ty, size } => AT::OtherArrayWithCst(ty, *size),
            Type::ArrayDoubleCst { ty, size1, size2 } => AT::OtherDoubleArray(ty, *size1, *size2),
            Type::Bitfield { ty, bitsize } => AT::Bitfield(ty, *bitsize),
        }
    }

    // remove VulkanSC only features
    fn filter_features(feats: &'a Vec<xml::Feature>) -> impl Iterator<Item = &'a xml::Feature> {
        feats
            .iter()
            .filter(|feat| feat.api.is_empty() || feat.api.contains(&xml::Api::Vulkan))
    }

    fn filtered_features(&self) -> impl Iterator<Item = &'a xml::Feature> {
        Self::filter_features(&self.registry.features)
    }

    // remove VulkanSC only extensions
    fn filter_extensions(
        exts: &'a Vec<xml::Extensions>,
    ) -> impl Iterator<Item = &'a xml::Extension> {
        exts.iter()
            .flat_map(|exts| &exts.extension)
            .filter(|ext| ext.supported.contains(&xml::ExtensionSupported::Vulkan))
            // for the time being, ignore video extensions
            .filter(|ext| {
                !ext.name.starts_with("VK_KHR_video") && !ext.name.starts_with("VK_VALVE_video")
            })
    }

    fn filtered_extensions(&self) -> impl Iterator<Item = &'a xml::Extension> {
        Self::filter_extensions(&self.registry.extensions)
    }

    fn all_types(&self) -> impl Iterator<Item = &'a xml::Type> {
        self.registry
            .types
            .iter()
            .flat_map(|ty| &ty.types)
            .filter(|ty| !matches!(&ty.apis[..], [xml::Api::Vulkansc]))
    }

    fn all_commands(&self) -> impl Iterator<Item = &'a xml::Command> {
        self.registry
            .commands
            .iter()
            .flat_map(|cmd| &cmd.command)
            .filter(|cmd| !matches!(&cmd.apis[..], [xml::Api::Vulkansc]))
    }

    fn parse_cmd_params<'b>(&'b self, cmd: &'b Command<'a>) -> Result<CommandParamsParsed<'a, 'b>> {
        let mut output_length: Option<&CommandParam> = None;
        let mut output_fields = Vec::new();

        let mut simple_fields: Vec<_> = cmd
            .params
            .iter()
            .map(|param| (param.vk_name, param))
            .collect();
        let mut vec_fields = Vec::new();

        fn erase_from_vec<T>(vec: &mut Vec<(&str, T)>, el: &str) {
            if let Some(pos) = vec.iter().position(|field| field.0 == el) {
                vec.remove(pos);
            }
        }

        // find return types
        for param in &cmd.params {
            let ptr_content = match param.ty {
                Type::Ptr(name) => name,
                Type::DoublePtr("void") => "VoidPtr",
                _ => continue,
            };

            if param.is_const {
                if let Some(len) = &param.xml.len {
                    if len != "null-terminated" {
                        if !param.optional {
                            erase_from_vec(&mut simple_fields, len);
                        }
                        erase_from_vec(&mut simple_fields, param.vk_name);

                        vec_fields.push((param.vk_name, param));
                    }
                }
                continue;
            }

            // special types written as non-const but which still need to be considered as const
            if [
                "Display",
                "IDirectFB",
                "wl_display",
                "xcb_connection_t",
                "_screen_window",
                "ubm_device",
            ]
            .contains(&ptr_content)
            {
                continue;
            }

            output_fields.push((param.vk_name, param));
            erase_from_vec(&mut simple_fields, param.vk_name);

            if let Some(len) = &param.xml.len {
                // if the length is a used provided input, there is a ->
                // if it has fixed size, a number is used
                if len.chars().all(|c| c.is_ascii_alphabetic()) {
                    let len_field = cmd
                        .params
                        .iter()
                        .find(|other| other.vk_name == len)
                        .ok_or_else(|| anyhow!("Failed to find param {len} for {}", cmd.vk_name))?;

                    let len_field_is_unknown =
                        !len_field.is_const && matches!(len_field.ty, Type::Ptr(_));

                    if len_field_is_unknown
                        && output_length.is_some_and(|out_len| out_len.vk_name != len_field.vk_name)
                    {
                        return Err(anyhow!(
                            "Two output lengths used in {}: {} and {}",
                            cmd.vk_name,
                            output_length.unwrap().vk_name,
                            len_field.vk_name
                        ));
                    }
                    if len_field_is_unknown {
                        output_length = Some(len_field);
                    }
                }
            }
        }

        if let Some(output_len) = output_length {
            // remove it from output_field
            let len_idx = output_fields
                .iter()
                .position(|el| el.1.vk_name == output_len.vk_name)
                .ok_or_else(|| {
                    anyhow!(
                        "Element {} from {} should be considered an output",
                        output_len.vk_name,
                        cmd.vk_name,
                    )
                })?;
            output_fields.remove(len_idx);
        }

        let length_mappings: HashMap<&str, &CommandParam> = vec_fields
            .iter()
            .filter_map(|(_, field)| {
                if field.optional {
                    return None;
                }
                let len = field.xml.len.as_ref()?;
                cmd.params
                    .iter()
                    .find(|param| param.vk_name == len)
                    .map(|param| (param.vk_name, *field))
            })
            .collect();

        let handle = cmd
            .params
            .first()
            .map(|param| match param.advanced_ty.get() {
                Some(AdvancedType::Handle(name)) => name,
                _ => "",
            })
            .unwrap_or("");

        let mut has_extended_lifetime = false;
        let result_params = cmd
            .params
            .iter()
            .enumerate()
            .map(|(idx, param)| {
                let advanced_ty = param.advanced_ty.get().unwrap();
                let name = format_ident!("{}", param.name);
                if simple_fields
                    .iter()
                    .any(|(vk_name, _)| *vk_name == param.vk_name)
                {
                    let outer_type =
                        self.generate_type_outer(&advanced_ty, param.optional, false)?;
                    Ok((None, (name.to_token_stream(), outer_type)))
                } else if vec_fields
                    .iter()
                    .any(|(vk_name, _)| *vk_name == param.vk_name)
                {
                    let SliceType {
                        template_param,
                        input_ty,
                        need_input_lifetime,
                        ..
                    } = self.generate_slice_type(
                        advanced_ty,
                        idx as u32,
                        &name,
                        None,
                        false,
                        param.optional,
                    )?;
                    has_extended_lifetime |= need_input_lifetime;
                    Ok((template_param, (name.to_token_stream(), input_ty)))
                } else {
                    Ok((None, (quote!(), quote!())))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let parsed_arg_templates = result_params
            .iter()
            .filter_map(|(x, _)| x.clone())
            .collect();
        let parsed_args_in = result_params
            .iter()
            .map(|(_, y)| y.clone())
            .filter(|(name, _)| !name.is_empty())
            .collect();

        Ok(CommandParamsParsed {
            handle,
            output_length,
            output_fields,
            simple_fields,
            vec_fields,
            length_mappings,
            parsed_arg_templates,
            parsed_args_in,
            command: cmd,
            has_extended_lifetime,
        })
    }

    /// Generate the raw type, as it is stored
    fn generate_type_inner(
        &self,
        ty: &AdvancedType<'a>,
        add_lifetime: bool,
    ) -> Result<TokenStream> {
        let get_lifetime = |ty: &str| {
            if add_lifetime {
                self.compute_name_lifetime(ty).then(|| quote! (<'a>))
            } else {
                None
            }
        };

        type AT<'a> = AdvancedType<'a>;
        let result = match ty {
            AT::Void => quote!(c_void),
            AT::VoidPtr => quote!(VoidPtr),
            AT::Bool32 => quote!(Bool32),
            AT::Enum(ty) | AT::Other(ty) => {
                let ty_ident = self.get_ident_name(ty)?;
                quote! (#ty_ident)
            }
            AT::Handle(ty) => {
                let lifetime = if add_lifetime {
                    quote! ('a)
                } else {
                    quote! ('_)
                };
                let ty_ident = self.get_ident_name(ty)?;
                quote! (Option<BorrowedHandle<#lifetime, #ty_ident>>)
            }
            AT::Func(_ty) => {
                quote!(FuncPtr)
            }
            AT::Struct(ty) => {
                let lifetime = get_lifetime(ty);
                let ty = self.get_ident_name(ty)?;
                quote! (#ty #lifetime)
            }
            AT::HandlePtr(ty) => {
                let ty = self.get_ident_name(ty)?;
                quote! (*const #ty)
            }
            AT::OtherPtr(ty) => {
                let lifetime = get_lifetime(ty);
                let ty = self.get_ident_name(ty)?;
                quote! (*const #ty #lifetime)
            }
            AT::OtherDoublePtr(ty) => {
                let lifetime = get_lifetime(ty);
                let ty = if *ty == "void" {
                    quote!(c_void)
                } else {
                    self.get_ident_name(ty)?.into_token_stream()
                };
                quote! (*const *const #ty #lifetime)
            }
            AT::HandleArray(ty, size) => {
                let ty: Ident = self.get_ident_name(ty)?;
                let size: Ident = self.get_ident_name(size)?;
                quote! ([Option<#ty>; #size as _])
            }
            AT::OtherArrayWithEnum(ty, size) => {
                let lifetime = get_lifetime(ty);
                let ty: Ident = self.get_ident_name(ty)?;
                let size: Ident = self.get_ident_name(size)?;
                quote! ([#ty #lifetime; #size as _])
            }
            AT::OtherArrayWithCst(ty, size) => {
                let lifetime = get_lifetime(ty);
                let ty: Ident = self.get_ident_name(ty)?;
                quote! ([#ty #lifetime; #size as _])
            }
            AT::OtherDoubleArray(ty, size1, size2) => {
                let lifetime = get_lifetime(ty);
                let ty: Ident = self.get_ident_name(ty)?;
                quote! ([[#ty #lifetime; #size2 as _]; #size1 as _])
            }
            AT::CharArray(size) => {
                let size: Ident = self.get_ident_name(size)?;
                quote! ([c_char; #size as _])
            }
            AT::CString => {
                quote!(*const c_char)
            }
            AT::CStringPtr => {
                quote!(*const *const c_char)
            }
            AT::Bitfield(_ty, bitsize) => {
                // Note: this may not be correct if there are padding bytes before
                let nb_bytes = bitsize / 8;
                quote! ([u8; #nb_bytes as _])
            }
        };
        Ok(result)
    }

    /// Generate a nice representation of the type which is a lot easier to interact with
    /// For example using references instead of raw pointers
    fn generate_type_outer(
        &self,
        ty: &AdvancedType<'a>,
        optional: bool,
        with_lifetime: bool,
    ) -> Result<TokenStream> {
        let get_lifetime =
            |ty: &str| (self.compute_name_lifetime(ty) && with_lifetime).then(|| quote! (<'a>));
        let life_a = with_lifetime.then_some(quote! ('a));

        let mut can_option = true;
        let result = match ty {
            AdvancedType::Handle(name) => {
                let name = self.get_ident_name(name)?;
                Ok(quote!(&#life_a raw::#name))
            }
            AdvancedType::HandlePtr(name) => {
                let name = self.get_ident_name(name)?;
                Ok(quote! (&#life_a raw::#name))
            }
            AdvancedType::HandleArray(_, _) => {
                Err(anyhow!("Trying to generate outer type of a handle array"))
            }
            AdvancedType::OtherPtr(name) => {
                let lifetime = get_lifetime(name);
                let name = self.get_ident_name(name)?;
                Ok(quote! (&#life_a #name #lifetime))
            }
            AdvancedType::OtherDoublePtr(name) => {
                let lifetime = get_lifetime(name);
                let name = self.get_ident_name(name)?;
                Ok(quote! (&#life_a &#life_a #name #lifetime))
            }
            AdvancedType::CString => Ok(quote!(&#life_a CStr)),
            AdvancedType::CStringPtr => Err(anyhow!(
                "Trying to generate outer type of a cstring pointer"
            )),
            AdvancedType::Bitfield(name, _) => {
                can_option = false;
                let name = self.get_ident_name(name)?;
                Ok(quote! (#name))
            }
            AdvancedType::Bool32 => {
                can_option = false;
                Ok(quote!(impl Into<Bool32>))
            }
            _ => {
                can_option = false;
                self.generate_type_inner(ty, true)
            }
        };
        if optional && can_option {
            result.map(|tk| quote! (Option<#tk>))
        } else {
            result
        }
    }

    fn generate_type_outer_to_inner(
        &self,
        ty: &AdvancedType<'a>,
        optional: bool,
        name: Ident,
    ) -> Result<TokenStream> {
        let mut add_option = false;
        let handle_ptr_option = || {
            if optional {
                quote!(#name.map(|v| ptr::from_ref(v)).unwrap_or(ptr::null()))
            } else {
                quote!(ptr::from_ref(#name))
            }
        };
        let result = match ty {
            AdvancedType::Handle(_) => {
                add_option = true;
                if optional {
                    Ok(quote!(#name.map(|v| v.borrow())))
                } else {
                    Ok(quote!(#name.borrow() ))
                }
            }
            AdvancedType::HandlePtr(_) => Ok(handle_ptr_option()),
            AdvancedType::HandleArray(_, _) => {
                Err(anyhow!("Trying to get outer type of a handle array"))
            }
            AdvancedType::OtherPtr(_) => Ok(handle_ptr_option()),
            AdvancedType::OtherDoublePtr(_) => {
                let ptr = handle_ptr_option();
                Ok(quote! (#ptr.cast()))
            }
            AdvancedType::CString => {
                if optional {
                    Ok(quote!(#name.map(|v| v.as_ptr()).unwrap_or(ptr::null())))
                } else {
                    Ok(quote!(#name.as_ptr()))
                }
            }
            AdvancedType::CStringPtr => Err(anyhow!(
                "Trying to generate outer type of a cstring pointer"
            )),
            AdvancedType::Bool32 => Ok(quote! (#name.into())),
            AdvancedType::Bitfield(bit_name, bitsize) => {
                let nb_bytes = *bitsize as usize / 8;
                assert!(nb_bytes <= 4);
                // we must use the .bits() method if it is a bitflag
                let value = self
                    .enums
                    .get(bit_name.replace("Flags", "FlagBits").as_str())
                    .filter(|en| en.bitflag)
                    .map(|_| quote!(#name.bits()))
                    .unwrap_or_else(|| quote!((#name as u32)));
                Ok(quote! {
                    #value.to_ne_bytes()[..#nb_bytes].try_into().unwrap()
                })
            }
            _ => Ok(quote!(#name)),
        };

        if add_option && !optional {
            result.map(|tk| quote!(Some(#tk)))
        } else {
            result
        }
    }

    fn generate_slice_type(
        &self,
        ty: AdvancedType<'a>,
        index: u32,
        name: &Ident,
        len_field: Option<&Ident>,
        is_assignment: bool,
        is_optional: bool,
    ) -> Result<SliceType> {
        let get_lifetimes = |ty: &str| {
            self.compute_name_lifetime(ty)
                .then(|| (quote! (<'a>), quote! (<'b>)))
                .unzip()
        };
        let template_ty = format_ident!("V{}", index);
        let mut simple_affectation = if is_optional {
            quote! (#name.map(|p| p.as_slice().as_ptr().cast()).unwrap_or(ptr::null()))
        } else {
            quote! (#name.as_slice().as_ptr().cast())
        };
        let mut access_name = quote!(#name);
        if matches!(ty, AdvancedType::VoidPtr) {
            access_name = quote! (#name.cast::<u8>())
        }
        let mut access = quote! ( unsafe { slice::from_raw_parts(self.#access_name.cast(), self.#len_field as _) });
        // TODO: add an if is_optional check once non-optional type pointers cannot be set to null (otherwise UB)
        // if is_optional {
        access = quote! ( (!self.#name.is_null()).then(|| #access).unwrap_or(&[]) );

        if is_assignment {
            simple_affectation = quote! (self.#name = #simple_affectation)
        };

        let wrap_ty = |ty: TokenStream| {
            if is_optional {
                quote! (Option<#ty>)
            } else {
                ty
            }
        };

        let field_name = name;
        match ty {
            AdvancedType::HandlePtr(name) => {
                let name = self.get_ident_name(name)?;
                Ok(SliceType {
                    template_param: Some(quote! (#template_ty: Alias<raw::#name> + 'a)),
                    input_ty: wrap_ty(quote! (impl AsSlice<'a, #template_ty>)),
                    affectation: simple_affectation,
                    output_ty: quote! (&'a [BorrowedHandle<'a, raw::#name>]),
                    access,
                    need_input_lifetime: false,
                    need_output_lifetime: false,
                })
            }
            AdvancedType::HandleArray(name, _) => {
                // output only type
                let name = self.get_ident_name(name)?;
                Ok(SliceType {
                    template_param: None,
                    input_ty: quote!(),
                    affectation: quote!(),
                    output_ty: quote! (&'b [BorrowedHandle<'a, raw::#name>]),
                    access: quote! {
                        // first perform the size check (safer although shouldn't be needed)
                        let handle_array = &self.#field_name[..(self.#len_field as _)];
                        unsafe { slice::from_raw_parts(handle_array.as_ptr().cast(), handle_array.len()) }
                    },
                    need_input_lifetime: false,
                    need_output_lifetime: true,
                })
            }
            AdvancedType::OtherPtr(name) => {
                let (lifetime_a, lifetime_b) = get_lifetimes(name);
                let name = self.get_ident_name(name)?;
                Ok(SliceType {
                    template_param: None,
                    input_ty: wrap_ty(quote! (impl AsSlice<'a,#name #lifetime_b>)),
                    affectation: simple_affectation,
                    output_ty: quote! (&'a [#name #lifetime_a]),
                    access,
                    need_input_lifetime: lifetime_b.is_some(),
                    need_output_lifetime: false,
                })
            }
            AdvancedType::VoidPtr => {
                // binary data
                Ok(SliceType {
                    template_param: None,
                    input_ty: wrap_ty(quote!(impl AsSlice<'a, u8>)),
                    affectation: simple_affectation,
                    output_ty: quote!(&'a [u8]),
                    access,
                    need_input_lifetime: false,
                    need_output_lifetime: false,
                })
            }
            AdvancedType::OtherDoublePtr(ty) => {
                let (lifetime_a, lifetime_b) = get_lifetimes(ty);
                let ty = if ty == "void" {
                    quote!(())
                } else {
                    format_ident!("{}", self.get_ident_name(ty)?).into_token_stream()
                };
                Ok(SliceType {
                    template_param: None,
                    input_ty: wrap_ty(quote! (impl AsSlice<'a, &'a #ty #lifetime_b>)),
                    affectation: simple_affectation,
                    output_ty: quote! (&'a [&'a #ty #lifetime_a]),
                    access,
                    need_input_lifetime: lifetime_b.is_some(),
                    need_output_lifetime: false,
                })
            }
            AdvancedType::CStringPtr => Ok(SliceType {
                template_param: None,
                input_ty: wrap_ty(quote!(impl AsSlice<'a, *const c_char>)),
                affectation: simple_affectation,
                output_ty: quote!(&'a [*const c_char]),
                access,
                need_input_lifetime: false,
                need_output_lifetime: false,
            }),
            AdvancedType::OtherArrayWithCst(name, _)
            | AdvancedType::OtherArrayWithEnum(name, _) => {
                // output only type
                assert!(get_lifetimes(name).0.is_none());
                let name = self.get_ident_name(name)?;
                Ok(SliceType {
                    template_param: None,
                    input_ty: quote!(),
                    affectation: quote!(),
                    output_ty: quote!(&'b [#name]),
                    access: quote! (&self.#field_name[..(self.#len_field as _)]),
                    need_input_lifetime: false,
                    need_output_lifetime: true,
                })
            }
            _ => Err(anyhow!(
                "Trying to get array with unexpected type for {name}"
            )),
        }
    }

    fn generate_default(&self, ty: &AdvancedType<'a>) -> Result<TokenStream> {
        match ty {
            AdvancedType::Void => Err(anyhow!(
                "Void type should never have to be constructed directly"
            )),
            AdvancedType::VoidPtr
            | AdvancedType::HandlePtr(_)
            | AdvancedType::OtherPtr(_)
            | AdvancedType::OtherDoublePtr(_)
            | AdvancedType::Func(_)
            | AdvancedType::CStringPtr
            | AdvancedType::CString => Ok(quote!(ptr::null())),
            AdvancedType::Handle(_)
            | AdvancedType::HandleArray(_, _)
            | AdvancedType::Struct(_)
            | AdvancedType::Bool32
            | AdvancedType::OtherDoubleArray(_, _, _) => Ok(quote!(Default::default())),
            AdvancedType::Other(name) => {
                // We can't use default if the basetype is the alias of a pointer
                let is_struct_voidptr = self.structs.get(name).is_some_and(|my_struct| {
                    matches!(
                        my_struct,
                        Struct::BaseType(StructBasetype {
                            ty: Type::VoidPtr,
                            ..
                        })
                    )
                });

                // imported structs are not in self.structs, but we also need to check for them
                let is_imported_voidptr = self.mapping.borrow().get(name).is_some_and(|entry| {
                    matches!(
                        (entry.ty, entry.name.as_str()),
                        (MappingType::BaseType, "VoidPtr")
                    )
                });

                if is_struct_voidptr || is_imported_voidptr {
                    Ok(quote!(ptr::null()))
                } else {
                    Ok(quote!(Default::default()))
                }
            }
            AdvancedType::Enum(name) => {
                // There is no default for regular enums
                let name = match self.mapping.borrow().get(name).map(|entry| entry.ty) {
                    Some(MappingType::Enum) => name,
                    Some(MappingType::EnumAlias(alias_name)) => alias_name,
                    _ => return Err(anyhow!("Type {name} was not found as an enum")),
                };
                Ok(self
                    .enums
                    .get(name)
                    .filter(|my_enum| !my_enum.bitflag)
                    .and_then(|my_enum| {
                        self.mapping
                            .borrow()
                            .get(my_enum.values.borrow()[0].0)
                            .map(|entry| entry.name.parse().unwrap())
                    })
                    .unwrap_or_else(|| quote!(Default::default())))
            }
            AdvancedType::OtherArrayWithEnum(name, _)
            | AdvancedType::OtherArrayWithCst(name, _) => {
                let value_default = self
                    .mapping
                    .borrow()
                    .get(name)
                    .is_some_and(|entry| {
                        matches!(entry.ty, MappingType::EnumAlias(_) | MappingType::Enum)
                    })
                    .then(|| self.generate_default(&AdvancedType::Enum(name)))
                    .unwrap_or_else(|| Ok(quote!(Default::default())))?;
                Ok(quote!(array::from_fn(|_| #value_default)))
            }
            AdvancedType::CharArray(_) => Ok(quote!(array::from_fn(|_| Default::default()))),
            AdvancedType::Bitfield(_, _bitsize) => Ok(quote!(Default::default())),
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
            // remove whitespace at the end of a line
            bitflag = bitflag.replace(" \n", "\n");

            // remove the indentation for the bitflags! closing bracket
            if bitflag.ends_with('\r') {
                // On Windows, lines end with CRLF and not LF
                bitflag.pop();
            }
            bitflag.replace_range((bitflag.len() - 5)..(bitflag.len() - 1), "");

            *line = Cow::Owned(bitflag);
        }

        lines.join("\n")
    }

    fn get_mapping_name(&self, name: &str) -> Result<String> {
        self.mapping
            .borrow()
            .get(name)
            .map(|entry| entry.name.clone())
            .ok_or_else(|| anyhow!("Failed to find key {name}"))
    }

    fn get_ident_name(&self, name: &str) -> Result<Ident> {
        self.get_mapping_name(name)
            .map(|name| format_ident!("{name}"))
    }

    fn has_trivial_dep(&self, deps: &Dependencies) -> bool {
        match deps {
            Dependencies::None => true,
            Dependencies::Single(dep) => self
                .extensions_features
                .borrow()
                .get(dep)
                .map_or(false, |f| !f.is_non_trivial.get()),
            Dependencies::Or(deps) => deps.iter().any(|dep| self.has_trivial_dep(dep)),
            Dependencies::And(deps) => deps.iter().all(|dep| self.has_trivial_dep(dep)),
        }
    }

    fn get_config_readable(&self, dependencies: &Dependencies) -> Option<String> {
        match dependencies {
            Dependencies::None => None,
            Dependencies::Single(dep) => self
                .extensions_features
                .borrow()
                .get(dep)
                .map(|feat| feat.name.clone()),
            Dependencies::Or(deps) => {
                let deps_cnf = deps.iter().filter_map(|dep| self.get_config_readable(dep));
                let res: String = deps_cnf.collect::<Vec<_>>().join(" or ").into();
                Some(format!("({res})"))
            }
            Dependencies::And(deps) => {
                let deps_cnf = deps.iter().filter_map(|dep| self.get_config_readable(dep));
                let res: String = deps_cnf.collect::<Vec<_>>().join(" and ").into();
                Some(format!("({res})"))
            }
        }
    }

    fn get_config_feature_inner(&self, dependencies: &Dependencies) -> Option<TokenStream> {
        // Note: This is really inefficient (quadratic complexity)
        if self.has_trivial_dep(dependencies) {
            return None;
        }

        match dependencies {
            Dependencies::None => None,
            Dependencies::Single(dep) => self.extensions_features.borrow().get(dep).map(|feat| {
                let name = feat.name.as_str();
                quote! {
                    feature = #name
                }
            }),
            Dependencies::Or(deps) => {
                let deps_cnf = deps
                    .iter()
                    .filter_map(|dep| self.get_config_feature_inner(dep));
                Some(quote! {
                    any(
                        #(#deps_cnf),*
                    )
                })
            }
            Dependencies::And(deps) => {
                let deps_cnf = deps
                    .iter()
                    .filter_map(|dep| self.get_config_feature_inner(dep));
                Some(quote! {
                    all(
                        #(#deps_cnf),*
                    )
                })
            }
        }
    }

    fn get_config_feature(&self, dependencies: &Dependencies) -> Result<Option<TokenStream>> {
        self.get_config_feature_inner(dependencies)
            .map(|cnf| {
                Ok(quote! {
                    #[cfg(#cnf)]
                })
            })
            .transpose()
    }
}

fn parse_value(value: &str, ty: CType) -> TokenStream {
    // bitwise negation is different
    let value: String = value.replace("~", "!");
    let mut rust_value = match ty {
        CType::Uint32 => value.replace("U", "u32"),
        CType::Uint64 => value.replace("ULL", "u64"),
        CType::Float => value.to_ascii_uppercase().replace("F", "f32"),
        CType::Int32 | CType::Int64 => value,
    };
    if rust_value.starts_with("(") && rust_value.ends_with(")") {
        rust_value = rust_value[1..(rust_value.len() - 1)].to_owned();
    }
    rust_value.parse().unwrap()
}

fn get_doc_url(item_name: &str) -> String {
    format!("<https://docs.vulkan.org/refpages/latest/refpages/source/{item_name}.html>")
}

fn make_doc_link(item_name: &str) -> TokenStream {
    let doc_name = get_doc_url(item_name);
    let alias_name = item_name
        .to_ascii_lowercase()
        .starts_with("vk")
        .then(|| quote! (#[doc(alias = #item_name)]));
    quote! (#[doc = #doc_name] #alias_name)
}

fn make_doc_link_inner(item_name: &str) -> TokenStream {
    let doc_name = get_doc_url(item_name);
    quote! (doc = #doc_name)
}
