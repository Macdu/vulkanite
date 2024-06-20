use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    ffi::CString,
    io::Write,
    iter,
};

use anyhow::{anyhow, Result};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, LitCStr};

use crate::{
    helpers::{camel_case_to_snake_case, convert_len_case},
    structs::{
        convert_field_to_snake_case, AdvancedType, Api, CType, Command, CommandParam,
        CommandParamsParsed, Constant, Enum, EnumAliased, EnumFlag, EnumValue, EnumVariant, Handle,
        MappingEntry, MappingType, ReturnType, Struct, StructBasetype, StructField, StructStandard,
        Type,
    },
    xml,
};

#[derive(Clone, Copy)]
pub enum GeneratedCommandType {
    Basic,
}

pub struct Generator<'a> {
    registry: &'a xml::Registry,
    ext_names: Vec<&'a str>,
    enums: HashMap<&'a str, Enum<'a>>,
    constants: HashMap<&'a str, Constant<'a>>,
    handles: HashMap<&'a str, Handle<'a>>,
    structs: HashMap<&'a str, Struct<'a>>,
    commands: HashMap<&'a str, Command<'a>>,
    mapping: RefCell<HashMap<&'a str, MappingEntry<'a>>>,
}

impl<'a> Generator<'a> {
    pub fn new(_api: Api, registry: &'a xml::Registry) -> Result<Generator> {
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

        let ext_names: Vec<_> = registry
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
                let mut my_enum = Enum::try_from(it, &ext_names)?;
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
            .filter(|ty| ty.api != Some(xml::Api::Vulkansc))
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
            .filter(|cmd| cmd.alias.is_none() && cmd.api != Some(xml::Api::Vulkansc))
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
            ("Window", "u32"),
            ("xcb_window_t", "u32"),
            // Custom types for QoL improvements
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
            ext_names,
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

        Ok(gen)
    }

    pub fn generate_extensions(&self) -> Result<String> {
        // also bundle the header version along with all the extensions
        let header_version = self
            .all_types()
            .find_map(|ty| {
                // we want to find the following:
                // <type api="vulkan" category="define">// Version of this file #define <name>VK_HEADER_VERSION</name> 281</type>
                match (ty, ty.content.as_slice()) {
                    (
                        xml::Type {
                            category: Some(cat),
                            ..
                        },
                        [xml::TypeContent::Text(_), xml::TypeContent::Name(name), xml::TypeContent::Text(value)],
                    ) if cat == "define" && name == "VK_HEADER_VERSION" => Some(value),
                    _ => None,
                }
            })
            .map(|v| v.parse::<u32>().ok())
            .unwrap_or_default()
            .ok_or_else(|| anyhow!("Failed to find VK_HEADER_VERSION"))?;

        let extensions = self
            .filtered_extensions()
            .map(|ext| {
                let is_device = matches!(ext.ty, Some(xml::ExtensionType::Device));
                let (ext_class, name_class) = if is_device {
                    (quote!(DeviceExtension), quote!(DeviceExtensionName))
                } else {
                    (quote!(InstanceExtension), quote!(InstanceExtensionName))
                };

                let mut req_block = ext
                    .require
                    .first()
                    .ok_or_else(|| anyhow!("Extension {ext:?} should have a require block"))?
                    .content
                    .iter()
                    .filter(|cnt| !matches!(cnt, xml::RequireContent::Comment(_)));

                let (name_enum, spec_enum) = match (req_block.next(), req_block.next()) {
                    (
                        Some(xml::RequireContent::Enum(spec)),
                        Some(xml::RequireContent::Enum(name)),
                    ) => (name, spec),
                    _ => return Err(anyhow!("Extension {ext:?} should start with two enums")),
                };

                if !spec_enum.name.ends_with("_SPEC_VERSION") {
                    return Err(anyhow!("{spec_enum:?} should end with _SPEC_VERSION"));
                }
                if !name_enum.name.ends_with("_EXTENSION_NAME") {
                    return Err(anyhow!("{name_enum:?} should end with _EXTENSION_NAME"));
                }

                let spec_version = spec_enum
                    .value
                    .as_ref()
                    .map(|v| v.parse::<u32>().ok())
                    .unwrap_or_default()
                    .ok_or_else(|| anyhow!("{:?} should be an integer", spec_enum.value))?;
                let ext_name = name_enum
                    .value
                    .as_ref()
                    .map(|v| (v.len() >= 2).then(|| &v[1..(v.len() - 1)]))
                    .unwrap_or_default()
                    .ok_or_else(|| anyhow!("{:?} should be in quotes", name_enum.value))?;
                let ext_name = LitCStr::new(&CString::new(ext_name).unwrap(), Span::call_site());

                let ext_ident = format_ident!("{}", ext.name["VK_".len()..].to_ascii_uppercase());

                Ok(quote! {
                    pub const #ext_ident :#ext_class = #ext_class{
                        name: unsafe {#name_class::new(#ext_name)},
                        spec: #spec_version
                    };
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            use super::{DeviceExtension, DeviceExtensionName, InstanceExtension, InstanceExtensionName, ApiVersion};

            pub const HEADER_VERSION: ApiVersion = ApiVersion::new(0, 1, 3, #header_version);

            #(#extensions)*
        }
        .to_string();

        let formatted_result = Self::format_result(result)?;
        Ok(formatted_result)
    }

    pub fn generate_enums(&self) -> Result<String> {
        let listed_enums = RefCell::new(HashSet::from(
            // we define TRUE and FALSE in lib.rs
            ["VK_TRUE", "VK_FALSE"],
        ));

        let feature_enums = self
            .filtered_features()
            .map(|feature| {
                self.generate_group_enums(&feature.name, &feature.require, &listed_enums)
            })
            .collect::<Result<Vec<_>>>()?;

        let extension_enums = self
            .filtered_extensions()
            .map(|ext| self.generate_group_enums(&ext.name, &ext.require, &listed_enums))
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
        let listed_handles = RefCell::new(HashSet::new());

        let generate_group_handle = |require: &'a xml::Require| -> Result<TokenStream> {
            let handles = require
                .content
                .iter()
                .filter_map(|item| match item {
                    xml::RequireContent::Type(xml::RequireType { name, .. }) => Some(name),
                    _ => None,
                })
                .filter_map(|name| self.handles.get(name.as_str()).map(|handle| (name, handle)))
                .filter(|(name, _)| listed_handles.borrow_mut().insert(*name))
                .map(|(name, handle)| self.generate_handle(handle, name))
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! { #(#handles)*})
        };

        let feature_handles = self
            .filtered_features()
            .flat_map(|feature| &feature.require)
            .map(|req| generate_group_handle(req))
            .collect::<Result<Vec<_>>>()?;
        let feature_extensions = self
            .filtered_extensions()
            .flat_map(|ext| &ext.require)
            .map(|req| generate_group_handle(req))
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            use core::fmt;
            use std::num::{NonZeroUsize, NonZeroU64};

            use crate::{handle_dispatchable, handle_nondispatchable, vk_handle, private};
            use crate::{Handle, vk::ObjectType};

            #(#feature_handles)*
            #(#feature_extensions)*
        }
        .to_string();

        let formatted_result = Self::format_result(result)?;
        Ok(formatted_result)
    }

    pub fn generate_structs(&self) -> Result<String> {
        let listed_structs = RefCell::new(HashSet::from(
            // We use our own Bool32 type
            ["VkBool32"],
        ));

        let generate_group_struct = |require: &'a xml::Require| -> Result<TokenStream> {
            let structs = require
                .content
                .iter()
                .filter_map(|item| match item {
                    xml::RequireContent::Type(xml::RequireType { name: ty_name, .. }) => self
                        .structs
                        .get(ty_name.as_str())
                        .filter(|_| listed_structs.borrow_mut().insert(&ty_name))
                        .map(|my_struct| match my_struct {
                            Struct::BaseType(StructBasetype {
                                name,
                                ty,
                                has_lifetime,
                            }) => {
                                let advanced_type = self.compute_advanced_type(ty);
                                let ty = self
                                    .generate_type_inner(&advanced_type, true)
                                    .expect("Failed to get type");
                                let name = format_ident!("{name}");
                                let doc_tag = make_doc_link(ty_name);
                                let lifetime = has_lifetime.get().unwrap().then(|| quote! (<'a>));
                                Ok(quote! (#doc_tag pub type #name #lifetime = #ty;))
                            }
                            Struct::Standard(my_struct) => self.generate_struct(my_struct, ty_name),
                        }),
                    _ => None,
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! { #(#structs)*})
        };

        let struct_features = self
            .filtered_features()
            .flat_map(|feature| &feature.require)
            .map(|req| generate_group_struct(req))
            .collect::<Result<Vec<_>>>()?;
        let struct_extensions = self
            .filtered_extensions()
            .flat_map(|ext| &ext.require)
            .map(|req| generate_group_struct(req))
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            use crate::*;
            use crate::vk::*;
            use crate::vk::raw::*;
            use core::slice;
            use std::ptr::{self, NonNull};
            use std::array;
            use std::marker::PhantomData;
            use std::ffi::{c_char, c_int};
            use std::mem::ManuallyDrop;

            #(#struct_features)*
            #(#struct_extensions)*
        }
        .to_string();

        let formatted_result = Self::format_result(result)?;
        Ok(formatted_result)
    }

    pub fn generate_dispatcher(&self) -> Result<String> {
        let listed_commands = RefCell::new(HashSet::new());
        let mut proc_addr_loader = Vec::new();
        let mut instance_loader = Vec::new();
        let mut device_loader = Vec::new();

        let generate_group_dispatcher = |require: &'a xml::Require| -> Result<TokenStream> {
            let cmds = require
                .content
                .iter()
                .filter_map(|req| match req {
                    xml::RequireContent::Command(cmd) => self
                        .commands
                        .get(cmd.name.as_str())
                        .filter(|_| listed_commands.borrow_mut().insert(&cmd.name))
                        .map(|cmd| {
                            self.generate_dispatch_command(
                                &cmd,
                                &mut proc_addr_loader,
                                &mut instance_loader,
                                &mut device_loader,
                            )
                        }),
                    _ => None,
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! (#(#cmds)*))
        };

        let dispatcher_features = self.filtered_features().flat_map(|feat| &feat.require);
        let dispatcher_extensions = self
            .filtered_extensions()
            .flat_map(|ext: &xml::Extension| &ext.require);

        let dispatcher_impl = dispatcher_features
            .chain(dispatcher_extensions)
            .map(generate_group_dispatcher)
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            use crate::*;
            use crate::vk::*;
            use crate::vk::raw::*;

            use std::mem;
            use std::cell::Cell;
            use std::ffi::{c_char, c_int};

            #[derive(Default, Clone)]
            pub struct CommandsDispatcher {
                #(#dispatcher_impl)*
            }
            unsafe impl Send for CommandsDispatcher{}
            unsafe impl Sync for CommandsDispatcher{}

            impl CommandsDispatcher {
                pub unsafe fn load_proc_addr(&self, get_instance_proc_addr: unsafe extern "system" fn(Option<Instance>, *const c_char) -> FuncPtr) {
                    self.get_instance_proc_addr.set(Some(get_instance_proc_addr));

                    #(#proc_addr_loader)*
                }

                pub unsafe fn load_instance(&self, instance: &Instance) {
                    let get_instance_proc_addr = self.get_instance_proc_addr.get().expect("load_proc_addr must be called before load_instance");
                    let get_instance = || Some(instance.clone());

                    #(#instance_loader)*
                }

                pub unsafe fn load_device(&self, device: &Device) {
                    let get_device_proc_addr = self.get_device_proc_addr.get().expect("load_instance must be called before load_device");
                    let get_device = || Some(device.clone());

                    #(#device_loader)*;
                }
            }
        }
        .to_string();

        Self::format_result(result)
    }

    pub fn generate_raw_commands(&self) -> Result<String> {
        let listed_commands = RefCell::new(HashSet::new());
        let generate_group_commands = |require: &'a xml::Require| -> Result<TokenStream> {
            let cmds = require
                .content
                .iter()
                .filter_map(|req| match req {
                    xml::RequireContent::Command(cmd) => self
                        .commands
                        .get(cmd.name.as_str())
                        .filter(|_| listed_commands.borrow_mut().insert(&cmd.name))
                        .map(|cmd| {
                            let cmd_params = self.parse_cmd_params(cmd)?;
                            // generate the command for the main name and all the possible aliases
                            let aliases = cmd.aliases.borrow();
                            let raw_cmds = iter::once((cmd.vk_name, cmd.name.as_str()))
                                .chain(
                                    aliases
                                        .iter()
                                        .map(|(vk_name, name)| (*vk_name, name.as_str())),
                                )
                                .map(|(vk_name, name)| {
                                    self.generate_raw_command(&cmd_params, vk_name, name)
                                })
                                .collect::<Result<Vec<_>>>()?;
                            Ok(quote! (#(#raw_cmds)*))
                        }),
                    _ => None,
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! (#(#cmds)*))
        };

        let raw_cmd_features = self.filtered_features().flat_map(|feat| &feat.require);
        let raw_cmd_extensions = self
            .filtered_extensions()
            .flat_map(|ext: &xml::Extension| &ext.require);

        let raw_cmd_impl = raw_cmd_features
            .chain(raw_cmd_extensions)
            .map(generate_group_commands)
            .collect::<Result<Vec<_>>>()?;

        let result = quote! {
            // TODO: remove
            #![allow(unused_unsafe)]

            use std::ffi::{c_int, c_void, CStr};

            use crate::*;
            use crate::vk::*;
            use crate::vk::raw::{self,*};

            #(#raw_cmd_impl)*
        }
        .to_string();

        Self::format_result(result)
    }

    pub fn generate_advanced_commands<'b>(
        &'b self,
        gen_ty: GeneratedCommandType,
    ) -> Result<String> {
        let mut listed_cmds = HashSet::new();
        let mut handle_cmds: HashMap<&str, BTreeMap<usize, CommandParamsParsed>> = HashMap::new();
        let mut possible_handles = HashSet::new();
        let mut cmd_nb = 0usize;

        let mut result = Vec::new();

        for req_cnt in self
            .filtered_features()
            .flat_map(|feat| &feat.require)
            .flat_map(|cnt| &cnt.content)
            .chain(
                self.filtered_extensions()
                    .flat_map(|ext| &ext.require)
                    .flat_map(|cnt| &cnt.content),
            )
        {
            let req_cmd = match req_cnt {
                xml::RequireContent::Command(req_cmd) => req_cmd,
                xml::RequireContent::Type(xml::RequireType { name, .. }) => {
                    if self.handles.contains_key(name.as_str()) {
                        possible_handles.insert(name.as_str());
                    }
                    continue;
                }
                _ => continue,
            };
            let cmd = match self.commands.get(req_cmd.name.as_str()) {
                Some(cmd) => cmd,
                None => continue,
            };
            if !listed_cmds.insert(cmd.vk_name) {
                continue;
            }
            let parsed_params = self.parse_cmd_params(cmd)?;
            handle_cmds
                .entry(parsed_params.handle)
                .or_default()
                .insert(cmd_nb, parsed_params);
            cmd_nb += 1;
        }

        let is_complex_handle = |name: &str| handle_cmds.contains_key(name);

        let create_methods =
            |cmds: &BTreeMap<usize, CommandParamsParsed>| -> Result<Vec<TokenStream>> {
                cmds.values()
                    .map(|cmd_parsed| {
                        let cmd = cmd_parsed.command;
                        let all_variants = iter::once((cmd.vk_name, cmd.name.as_str()))
                            .chain(
                                cmd.aliases
                                    .borrow()
                                    .iter()
                                    .map(|(vk_name, name)| (*vk_name, name.as_str())),
                            )
                            .map(|(vk_name, name)| {
                                self.generate_advanced_command(
                                    name,
                                    vk_name,
                                    cmd_parsed,
                                    gen_ty,
                                    is_complex_handle,
                                )
                            })
                            .collect::<Result<Vec<_>>>()?;
                        Ok(quote! (#(#all_variants)*))
                    })
                    .collect::<Result<Vec<_>>>()
            };

        let entry_cmds = handle_cmds
            .get("")
            .expect("vkCreateInstance should be here");
        let entry_methods = create_methods(entry_cmds)?;
        result.push(quote! {
            pub struct Entry<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
                disp: D,
                alloc: A,
            }

            impl<D: Dispatcher, A: Allocator> Entry<D, A> {
                pub fn new(disp: D, alloc: A) -> Self {
                    Self {
                        disp,
                        alloc,
                    }
                }

                pub fn get_dispatcher(&self) -> &D {
                    &self.disp
                }

                pub fn get_allocator(&self) -> &A {
                    &self.alloc
                }

                #(#entry_methods)*
            }
        });

        for (handle_name, handle) in &self.handles {
            if !possible_handles.contains(handle_name) {
                continue;
            }

            if let Some(cmds) = handle_cmds.get(handle_name) {
                let id_name = format_ident!("{}", handle.name);
                let doc_tag = make_doc_link(handle_name);

                let methods = create_methods(cmds)?;

                result.push(quote! {
                    #[repr(C)]
                    #doc_tag
                    pub struct #id_name<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
                        inner: raw::#id_name,
                        disp: D,
                        alloc: A,
                    }

                    impl<D: Dispatcher, A: Allocator> Deref for #id_name<D,A> {
                        type Target = raw::#id_name;

                        fn deref(&self) -> &Self::Target {
                            &self.inner
                        }
                    }

                    impl #id_name {
                        pub fn from_inner(handle: raw::#id_name) -> Self {
                            Self {
                                inner: handle,
                                disp: DynamicDispatcher(()),
                                alloc: DefaultAllocator,
                            }
                        }
                    }

                    impl<D: Dispatcher, A: Allocator> #id_name<D, A> {
                        pub fn from_inner_with(handle: raw::#id_name, disp: D, alloc: A) -> Self {
                            Self {
                                inner: handle,
                                disp,
                                alloc,
                            }
                        }

                        pub fn get_dispatcher(&self) -> &D {
                            &self.disp
                        }

                        pub fn get_allocator(&self) -> &A {
                            &self.alloc
                        }

                        #(#methods)*
                    }
                });
            } else {
                for alias_name in
                    iter::once(handle.name).chain(handle.aliases.borrow().iter().copied())
                {
                    let id_name = format_ident!("{alias_name}");
                    let doc_tag = make_doc_link(&format!("Vk{alias_name}"));
                    result.push(quote! (#doc_tag pub type #id_name = raw::#id_name;))
                }
            }
        }

        let result = quote! {
            use std::{
                ffi::{c_int, c_void, CStr},
                ops::Deref,
            };
            use crate::{vk::*, Alias, Allocator, AdvancedDynamicArray, DefaultAllocator, Dispatcher, DynamicArray, DynamicDispatcher, StructureChainOut};

            #(#result)*
        }
        .to_string();

        Self::format_result(result)
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
            .flat_map(|feat| feat.require.iter().map(|req| (None, req)))
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
            let name = convert_field_to_snake_case(&parent_name, &ext.name, &self.ext_names)?;

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
                            ty: MappingType::AliasedEnum(enum_vk_name)
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
                                ty: MappingType::AliasedEnum(alias.as_str())
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
                let value = if let Some(req) = &bitmask.requires {
                    mapping
                        .get(req.as_str())
                        .ok_or_else(|| anyhow!("Failed to find bitflag {req}"))?
                        .clone()
                } else {
                    // bitflag with no value defined yet (can only be 0)
                    MappingEntry {
                        name: "u32".to_owned(),
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
                    xml::TypeContent::Name(name) => Some(name.as_str()),
                    _ => None,
                })
                .ok_or_else(|| anyhow!("Failed to find name for funcptr"))?;
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
                .ok_or_else(|| anyhow!("Expected a name for {:?}", handle))?
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
                                ty: MappingType::AliasedStruct(vk_name)
                            }
                        )
                        .is_none())
                }
            }
        }

        // we must drop the mutable reference before the next part
        std::mem::drop(mapping);

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
            MappingType::AliasedStruct(alias) => alias,
            _ => return None,
        };
        match self.structs.get(struct_name) {
            Some(Struct::Standard(my_struct)) => Some(my_struct),
            _ => None,
        }
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
            Some(MappingType::AliasedStruct(alias)) => alias,
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
                let lifetime = union_type.fields.iter().any(|field| {
                    self.compute_type_lifetime(&field.ty)
                        // we must have an internal lifetime for unions (pointer to basetypes do not count)
                        // we assume the basetype has already been added to the mapping
                        && match field.ty {
                            Type::Ptr(name) => !self
                                .mapping
                                .borrow()
                                .get(name)
                                .is_some_and(|entry| matches!(entry.ty, MappingType::BaseType)),
                            _ => true
                        }
                });
                union_type.has_lifetime.set(Some(lifetime));
                lifetime
            }
            Struct::Standard(standard_type) => {
                let lifetime = standard_type.s_type.is_some()
                    || standard_type
                        .fields
                        .iter()
                        .any(|field| self.compute_type_lifetime(&field.ty));
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
                Some(MappingType::Struct | MappingType::AliasedStruct(_)) => AT::Struct(name),
                Some(MappingType::Handle | MappingType::HandleAlias(_)) => AT::Handle(name),
                Some(MappingType::Enum | MappingType::AliasedEnum(_)) => AT::Enum(name),
                Some(MappingType::FunctionPtr) => AT::Func(name),
                _ => AT::Other(name),
            },
            Type::Ptr(name) => {
                if self.mapping.borrow().get(name).is_some_and(|entry| {
                    matches!(entry.ty, MappingType::Handle | MappingType::HandleAlias(_))
                }) {
                    AT::HandlePtr(name)
                } else if *name == "char" {
                    AT::CString
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
            .flat_map(|exts| &exts.extension)
            .filter(|ext| ext.supported.contains(&xml::ExtensionSupported::Vulkan))
            // for the time being, ignore video extensions
            .filter(|ext| !ext.name.starts_with("VK_KHR_video"))
    }

    fn all_types(&self) -> impl Iterator<Item = &'a xml::Type> {
        self.registry
            .types
            .iter()
            .flat_map(|ty| &ty.types)
            .filter(|ty| ty.api != Some(xml::Api::Vulkansc))
    }

    fn all_commands(&self) -> impl Iterator<Item = &'a xml::Command> {
        self.registry
            .commands
            .iter()
            .flat_map(|cmd| &cmd.command)
            .filter(|cmd| cmd.api != Some(xml::Api::Vulkansc))
    }

    fn generate_group_enums(
        &self,
        _group_name: &str,
        require: &'a Vec<xml::Require>,
        listed_enums: &RefCell<HashSet<&'a str>>,
    ) -> Result<TokenStream> {
        let content = require
            .iter()
            .flat_map(|req| &req.content)
            .collect::<Vec<_>>();
        let group_enums = content
            .into_iter()
            .filter_map(|cnt| match cnt {
                xml::RequireContent::Enum(xml::RequireEnum { name, .. })
                | xml::RequireContent::Type(xml::RequireType { name, .. }) => {
                    if !listed_enums.borrow().contains(name.as_str()) {
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
                    listed_enums.borrow_mut().insert(name);
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
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #doc_tag
            #pre_qualifier
            pub #struct_ty #name #post_qualifier {
                #(#in_decls)*
            }
        };

        if is_bitflag {
            Ok(quote! {
                bitflags! {
                    #[derive(Default)]
                    #[repr(transparent)]
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

    fn generate_handle(&self, handle: &Handle<'a>, vk_name: &str) -> Result<TokenStream> {
        let name = format_ident!("{}", handle.name);
        let dispatch_macro = if handle.is_dispatchable {
            "handle_dispatchable"
        } else {
            "handle_nondispatchable"
        };
        let dispatch_macro = format_ident!("{dispatch_macro}");
        let doc_tag = make_doc_link_inner(vk_name);
        let mapping = self.mapping.borrow();
        let object_type = &mapping.get(handle.object_type)
            .ok_or_else(|| anyhow!( "Failed to find matching object type for {}", handle.object_type))?.name
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
    }

    fn generate_struct(
        &self,
        my_struct: &StructStandard<'a>,
        struct_vk_name: &str,
    ) -> Result<TokenStream> {
        let mapping = self.mapping.borrow();
        let all_fields: HashMap<_, _> = my_struct.fields.iter().map(|f| (f.vk_name, f)).collect();
        let mut simple_fields = all_fields.clone();
        let mut char_arr_fields = Vec::new();
        if my_struct.s_type.is_some() {
            // remove preemptively s_type and p_next
            simple_fields.remove("sType");
            simple_fields.remove("pNext");
        }

        struct FieldWithLen<'a, 'b> {
            len_field: &'b StructField<'a>,
            array_fields: Vec<&'b StructField<'a>>,
        }
        let mut length_fields = HashMap::new();

        // retrieve all arrays with a len
        for field in &my_struct.fields {
            let mut len = match &field.xml.len {
                Some(len) => len.as_str(),
                _ => continue,
            };

            if len == "null-terminated" {
                if let Some(AdvancedType::CharArray(size)) = field.advanced_ty.get() {
                    simple_fields.remove(field.vk_name);
                    char_arr_fields.push((field, size));
                }
                continue;
            }

            if len.ends_with(",null-terminated") {
                // just ask for an array of const char* for the time being
                len = &len[..(len.len() - ",null-terminated".len())]
            }

            if field.xml.alt_len.is_some() {
                // TODO: handle
                simple_fields.remove(field.vk_name);
                continue;
            }

            if len.contains(',') {
                // TODO: handle
                len = len.split(',').next().unwrap();
            }

            let len_field = *all_fields
                .get(len)
                .ok_or_else(|| anyhow!("Failed to find length field {len} for {struct_vk_name}"))?;

            simple_fields.remove(field.vk_name);
            simple_fields.remove(len_field.vk_name);

            length_fields
                .entry(len)
                .or_insert_with(|| FieldWithLen {
                    len_field,
                    array_fields: Vec::new(),
                })
                .array_fields
                .push(field);
        }

        let iter = my_struct
            .fields
            .iter()
            .map(|field| {
                let field_name = format_ident!("{}", field.name);
                let (ty_name, vis, default_impl) = match field.vk_name {
                    "sType" if my_struct.s_type.is_some() => (
                        quote!(StructureType),
                        quote! (pub(crate)),
                        quote! {#field_name: Self::STRUCTURE_TYPE},
                    ),
                    "pNext" if my_struct.s_type.is_some() => (
                        quote!(Cell<*const Header>),
                        quote! (pub(crate)),
                        quote! {p_next: Cell::new(ptr::null())},
                    ),
                    _ => {
                        let default_value =
                            self.generate_default(&field.advanced_ty.get().unwrap())?;
                        let ty_inner =
                            self.generate_type_inner(&field.advanced_ty.get().unwrap(), true)?;
                        let ty_inner = if my_struct.is_union {
                            quote! (ManuallyDrop<#ty_inner>)
                        } else {
                            ty_inner
                        };
                        (
                            ty_inner,
                            if my_struct.is_union || simple_fields.get(field.vk_name).is_some() {
                                quote!(pub)
                            } else {
                                quote! (pub(crate))
                            },
                            quote! (#field_name: #default_value),
                        )
                    }
                };
                Ok((quote! {#vis #field_name: #ty_name,}, default_impl))
            })
            .collect::<Result<Vec<_>>>()?;

        let (fields, default_impl): (Vec<_>, Vec<_>) = iter.into_iter().unzip();

        let simple_accessors = simple_fields
            .into_iter()
            .map(|(_, field)| {
                let name = format_ident!("{}", field.name);
                let fn_name = if field.name.starts_with("p_") {
                    &field.name[2..]
                } else if field.name.starts_with("pp_") {
                    &field.name[3..]
                } else {
                    &field.name
                };
                let fn_name = format_ident!("{fn_name}");
                let ty = field.advanced_ty.get().unwrap();
                let ty_name = self.generate_type_outer(&ty, field.optional, true)?;
                let value =
                    self.generate_type_outer_to_inner(&ty, field.optional, format_ident!("value"))?;
                Ok(quote! {
                    #[inline]
                    pub fn #fn_name(mut self, value: #ty_name) -> Self {
                        self.#name = #value;
                        self
                    }
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let char_arr_setters = char_arr_fields.into_iter().map(|(field, size)| {
            let name = format_ident!("{}", field.name);
            let fn_name = format_ident!("get_{}", field.name);
            let size_cst = format_ident!("{}", &size["VK_".len()..]);

            quote! {
                pub fn #fn_name(&self) -> &CStr {
                    CStr::from_bytes_until_nul(
                        unsafe {
                            mem::transmute::<_, &[u8; #size_cst as _]>(&self.#name)
                        }
                        .as_slice(),
                    )
                    .unwrap()
                }
            }
        });

        let array_accessors = length_fields
            .into_iter()
            .map(|(_, length_field)| {
                if my_struct.return_only {
                    // TODO: still add getters
                    return Ok(quote! ());
                }

                let len_field = length_field.len_field;
                let var_name = if length_field.array_fields.len() == 1 && length_field.array_fields[0].name.starts_with("p_") {
                    &length_field.array_fields[0].name[("p_".len())..]
                } else if len_field.name.ends_with("_count") {
                     &len_field.name[..(len_field.name.len() - "_count".len())]
                } else if len_field.name.ends_with("_size") {
                    &len_field.name[..(len_field.name.len() - "_size".len())]
                } else if len_field.name == "size" {
                    "values"  
                } else {
                    return Err(anyhow!("field length name not expected: {}", len_field.name))
                };
                let setter_name = format_ident!("{}", var_name);
                //let getter_name = format_ident!("get_{}", field.name[2..]);
                let length_name = format_ident!("{}", length_field.len_field.name);
                let ty_tokens = length_field.array_fields.iter().enumerate().map(|(idx, field)|{
                    self.generate_slice_type(field.advanced_ty.get().unwrap(), idx as u32, format_ident!("{}", field.name), true, true)
                }).collect::<Result<Vec<_>>>()?;
                let field_names = length_field.array_fields.iter().map(|field| {
                    format_ident!("{}",field.name)
                }).collect::<Vec<_>>();
                let first_field_name = &field_names[0];
                let template_arg = ty_tokens.iter().map(|(x,_,_)| x).filter(|x| !x.is_empty());
                let slice_ty = ty_tokens.iter().map(|(_,y,_)| y);
                let attr = ty_tokens.iter().map(|(_,_,z)| z);
                Ok(quote! {
                    #[inline]
                    pub fn #setter_name<#(#template_arg),*>(mut self, #(#field_names: #slice_ty),*) -> Self {
                        #(#attr;)*
                        self.#length_name = #first_field_name.len() as _;
                        self
                    }
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let aliases = my_struct
            .aliases
            .borrow()
            .iter()
            .map(|name| format_ident!("{}", mapping.get(name).unwrap().name))
            .collect::<Vec<_>>();

        let name = format_ident!("{}", my_struct.name);
        let doc_tag = make_doc_link(struct_vk_name);

        let has_lifetime = my_struct.has_lifetime.get().unwrap();
        let lifetime = has_lifetime.then(|| quote! (<'a>));
        let require_phantom = has_lifetime;
        let phantom_decl = require_phantom.then(|| quote! (phantom: PhantomData<&'a ()>,));
        let phantom_default = require_phantom.then(|| quote! (phantom: PhantomData,));
        let (s_type_impl, p_next_impl) = if let Some(s_type) = my_struct.s_type {
            let s_type_value: TokenStream = mapping
                .get(s_type)
                .ok_or_else(|| anyhow!("Failed to find structure type for {s_type}"))?
                .name
                .parse()
                .unwrap();
            (
                Some(quote! {
                    unsafe impl #lifetime ExtendableStructure for #name #lifetime {
                        const STRUCTURE_TYPE: StructureType = #s_type_value;
                    }
                }),
                Some(quote! {
                    pub fn push_next<T: ExtendingStructure<Self>>(&mut self, ext: &'a mut T) {
                        unsafe { self.push_next_unchecked(ext) }
                    }
                }),
            )
        } else {
            (None, None)
        };
        let struct_extensions = my_struct
            .extends
            .iter()
            .map(|name| {
                mapping
                    .get(name.as_str())
                    .map(|entry| format_ident!("{}", entry.name))
                    .ok_or_else(|| anyhow!("Failed to find extension {}", name))
            })
            .collect::<Result<Vec<_>>>()?;

        if my_struct.is_union {
            // union are much more lightweight
            // we should have wrapper around them (as they are unsafe)

            // For the default implementation, take the first field and use its default value
            let first_field = my_struct.fields.first().unwrap();
            let first_field_name = format_ident!("{}", first_field.name);
            let first_field_default =
                self.generate_default(&first_field.advanced_ty.get().unwrap())?;
            return Ok(quote! {
                #[repr(C)]
                #doc_tag
                pub union #name #lifetime {
                    #(#fields)*
                }

                impl #lifetime Default for #name #lifetime {
                    fn default() -> Self {
                        Self {
                            #first_field_name: ManuallyDrop::new(#first_field_default),
                        }
                    }
                }

                #(pub type #aliases #lifetime = #name #lifetime;)*
            });
        }

        Ok(quote! {
            #[repr(C)]
            #doc_tag
            pub struct #name #lifetime {
                #(#fields)*
                #phantom_decl
            }
            #s_type_impl
            unsafe impl #lifetime Send for #name #lifetime {}
            unsafe impl #lifetime Sync for #name #lifetime {}
            #(unsafe impl<'a, 'b> ExtendingStructure<#struct_extensions<'b>> for #name<'a> {})*

            impl #lifetime Default for #name #lifetime {
                fn default() -> Self {
                    Self {
                        #(#default_impl,)*
                        #phantom_default
                    }
                }
            }

            impl #lifetime #name #lifetime {
                #(#simple_accessors)*
                #(#char_arr_setters)*
                #(#array_accessors)*
                #p_next_impl
            }

            #(pub type #aliases #lifetime = #name #lifetime;)*
        })
    }

    /// First is the field type and name, second
    fn generate_dispatch_command(
        &self,
        cmd: &Command<'a>,
        proc_addr_loader: &mut Vec<TokenStream>,
        instance_loader: &mut Vec<TokenStream>,
        device_loader: &mut Vec<TokenStream>,
    ) -> Result<TokenStream> {
        let ret_type = match cmd.return_ty {
            ReturnType::Void => quote!(),
            ReturnType::Result => quote! (-> Status),
            ReturnType::BaseType(name) => {
                let name = self
                    .mapping
                    .borrow()
                    .get(name)
                    .map(|entry| format_ident!("{}", entry.name))
                    .ok_or_else(|| anyhow!("Failed to find type {name}"))?;
                quote! (-> #name)
            }
        };
        let params = cmd
            .params
            .iter()
            .map(|param| self.generate_type_inner(&param.advanced_ty.get().unwrap(), false))
            .collect::<Result<Vec<_>>>()?;

        let aliases = cmd.aliases.borrow();
        let names = iter::once(cmd.name.as_str())
            .chain(aliases.iter().map(|(_, alias)| alias.as_str()))
            .map(|name| {
                let name = format_ident!("{name}");
                quote! (pub #name: Cell<Option<unsafe extern "system" fn(#(#params),*) #ret_type>>,)
            });

        let main_name = format_ident!("{}", cmd.name);
        let update_fallback = |loader: &mut Vec<TokenStream>, alias: &Ident| {
            if alias == &main_name {
                return;
            }

            loader.push(quote! (self.#main_name.set(self.#main_name.get().or(self.#alias.get()));));
        };

        for (vk_name, name) in iter::once((cmd.vk_name, cmd.name.as_str())).chain(
            aliases
                .iter()
                .map(|(vk_name, alias)| (*vk_name, alias.as_str())),
        ) {
            let name = format_ident!("{name}");
            let name_cstr = LitCStr::new(&CString::new(vk_name).unwrap(), Span::call_site());
            if let Some(handle_name) = cmd.handle.get() {
                instance_loader.push(quote! (self.#name.set(mem::transmute(get_instance_proc_addr(get_instance(), #name_cstr.as_ptr())));));
                update_fallback(instance_loader, &name);

                if handle_name != "VkInstance" && handle_name != "VkPhysicalDevice" {
                    device_loader.push(quote! (self.#name.set(mem::transmute(get_device_proc_addr(get_device(), #name_cstr.as_ptr())));));
                    update_fallback(device_loader, &name);
                }
            } else {
                proc_addr_loader.push(quote! (self.#name.set(mem::transmute(get_instance_proc_addr(None, #name_cstr.as_ptr())));));
                update_fallback(proc_addr_loader, &name);
            }
        }

        Ok(quote! (#(#names)*))
    }

    fn generate_raw_command<'b>(
        &'b self,
        parsed_cmd: &CommandParamsParsed<'a, 'b>,
        vk_name: &str,
        name: &str,
    ) -> Result<TokenStream> {
        let CommandParamsParsed {
            output_length,
            output_fields,
            simple_fields,
            vec_fields,
            length_mappings,
            command: cmd,
            parsed_arg_templates,
            parsed_args_in,
            ..
        } = parsed_cmd;

        if output_fields.len() > 1 {
            // TODO
            return Ok(quote!());
        }

        assert!(output_fields.is_empty() || !matches!(cmd.return_ty, ReturnType::BaseType(_)));

        let args_inner = cmd
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
                    let outer_to_inner_type = self.generate_type_outer_to_inner(
                        &advanced_ty,
                        param.optional,
                        name.clone(),
                    )?;
                    Ok(outer_to_inner_type)
                } else if vec_fields
                    .iter()
                    .any(|(vk_name, _)| *vk_name == param.vk_name)
                {
                    let (_, _, inner) = self.generate_slice_type(
                        advanced_ty,
                        idx as u32,
                        name.clone(),
                        false,
                        false,
                    )?;
                    Ok(inner)
                } else if output_fields
                    .iter()
                    .any(|(vk_name, _)| *vk_name == param.vk_name)
                {
                    let param_name = match param.ty {
                        Type::Ptr(name) => name,
                        _ => return Err(anyhow!("Param {} should be a pointer", param.vk_name)),
                    };
                    let is_structure_type = self
                        .get_struct(param_name)
                        .is_some_and(|my_struct| my_struct.s_type.is_some());
                    if output_length.is_some() {
                        Ok(quote! (#name))
                    } else if param.xml.len.is_some() {
                        Ok(quote! (#name.get_content_mut_ptr()))
                    } else if is_structure_type {
                        Ok(quote! (S::get_uninit_head_ptr(#name.as_mut_ptr())))
                    } else {
                        Ok(quote! (#name.as_mut_ptr()))
                    }
                } else if output_length.is_some_and(|len| len.vk_name == param.vk_name) {
                    Ok(quote! (#name))
                } else {
                    // it is the length of a vec field
                    let vec_field = length_mappings
                        .get(param.vk_name)
                        .ok_or_else(|| anyhow!("Failed to find field for {}", param.vk_name))?;
                    let vec_field_name = format_ident!("{}", vec_field.name);
                    Ok(quote! (#vec_field_name.len() as _))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let templates = &parsed_arg_templates;
        let args_outer_name = parsed_args_in.iter().map(|(x, _)| x);
        let args_outer_type = parsed_args_in.iter().map(|(_, y)| y);

        let (ret_type, pre_call, post_call, ret_template) = match cmd.return_ty {
            ReturnType::BaseType(name) => {
                let ty_name = self.get_ident_name(name)?;
                (quote! (-> #ty_name), None, None, None)
            }
            ReturnType::Result if output_fields.is_empty() => {
                (quote! (-> Status), None, None, None)
            }
            _ if !output_fields.is_empty() => {
                let has_status = cmd.return_ty == ReturnType::Result;
                let (_, field) = output_fields[0];
                let ret_type = match field.ty {
                    Type::Ptr(name) => name,
                    _ => return Err(anyhow!("Could not use return field for {name}")),
                };
                let ret_name = self.get_ident_name(ret_type)?;
                let field_name = format_ident!("{}", field.name);

                let external_length = output_length.map(|param| format_ident!("{}", param.name));
                let internal_length = field
                    .xml
                    .altlen
                    .as_ref()
                    .or(field.xml.len.as_ref())
                    .filter(|_| external_length.is_none())
                    .map(|len| {
                        if let Some(param) = length_mappings.get(len.as_str()) {
                            let param_name = format_ident!("{}", param.name);
                            Ok(quote! (#param_name.len()))
                        } else {
                            convert_len_case(len).parse::<TokenStream>()
                        }
                    })
                    .transpose()
                    .map_err(|_| anyhow!("Failed to parse length of {}", field.vk_name))?;

                let is_structure_type = self
                    .get_struct(ret_type)
                    .is_some_and(|my_struct| my_struct.s_type.is_some());
                let is_handle = self.get_handle(ret_type).is_some();

                let lifetime = (!is_handle && self.compute_name_lifetime(ret_type))
                    .then(|| quote! (<'static>));

                let mut result_quote = quote! (#ret_name #lifetime);
                if internal_length.is_some() || external_length.is_some() {
                    result_quote = quote!(R)
                } else if is_structure_type {
                    result_quote = quote!(S)
                }
                if has_status {
                    result_quote = quote! (Result<#result_quote>)
                }
                let prev_affectation = has_status.then(|| quote! (let vk_status = ));
                let return_cast = if let Some(internal_length) = &internal_length {
                    quote! (#field_name.resize_with_len(#internal_length as _); #field_name)
                } else if external_length.is_some() {
                    quote!(vk_vec.resize_with_len(vk_len as _); vk_vec)
                } else if is_structure_type {
                    quote! (S::setup_cleanup(#field_name.as_mut_ptr());#field_name.assume_init())
                } else {
                    quote! (#field_name.assume_init())
                };
                let return_result = if has_status {
                    quote! (; vk_status.map_success(|| {#return_cast}))
                } else {
                    quote! (; #return_cast)
                };
                let param_init = if let Some(internal_length) = &internal_length {
                    Some(
                        quote! (let mut #field_name = R::allocate_with_capacity(#internal_length as _); #prev_affectation),
                    )
                } else if let Some(external_length) = &external_length {
                    let first_call_args = args_inner.clone();
                    let map_success = has_status.then(|| quote! (.map_success(|| ())?));
                    Some(quote! {
                        let mut vk_len = MaybeUninit::uninit();
                        let #external_length = vk_len.as_mut_ptr();
                        let #field_name = ptr::null_mut();
                        vulkan_command(#(#first_call_args),*)#map_success;
                        let mut vk_len = vk_len.assume_init();
                        let mut vk_vec = R::allocate_with_capacity(vk_len as _);
                        let #external_length = ptr::from_mut(&mut vk_len);
                        let #field_name = vk_vec.get_content_mut_ptr();
                        #prev_affectation
                    })
                } else if is_structure_type {
                    Some(
                        quote! (let mut #field_name = MaybeUninit::uninit(); S::setup_uninit(&mut #field_name); #prev_affectation),
                    )
                } else {
                    Some(quote! (let mut #field_name = MaybeUninit::uninit(); #prev_affectation))
                };
                let ret_template = if internal_length.is_some() || external_length.is_some() {
                    Some(quote! (R: DynamicArray<#ret_name #lifetime>,))
                } else if is_structure_type {
                    Some(quote! (S: StructureChainOut<#ret_name #lifetime> ))
                } else {
                    None
                };
                (
                    quote! (-> #result_quote),
                    Some(param_init),
                    Some(return_result),
                    ret_template,
                )
            }
            _ => (quote!(), None, None, None),
        };

        let func_name = format_ident!("{name}");
        let doc = make_doc_link(vk_name);
        Ok(quote! {
            #doc
            pub fn #func_name<#ret_template #(#templates),*>(#(#args_outer_name: #args_outer_type,)* dispatcher: &CommandsDispatcher ) #ret_type {
                let vulkan_command = dispatcher.#func_name.get().expect("Vulkan command not loaded.");
                unsafe {
                    #pre_call
                    vulkan_command(#(#args_inner),*)
                    #post_call
                }
            }
        })
    }

    fn generate_advanced_command<F>(
        &self,
        name: &str,
        vk_name: &str,
        cmd_parsed: &CommandParamsParsed,
        _gen_ty: GeneratedCommandType,
        is_complex_handle: F,
    ) -> Result<TokenStream>
    where
        F: Fn(&str) -> bool,
    {
        let cmd = cmd_parsed.command;
        let arg_template = &cmd_parsed.parsed_arg_templates;

        if cmd_parsed.output_fields.len() > 1 {
            return Ok(quote!());
        }

        // the first element is usually the handle, skip it
        let nb_to_skip = if cmd_parsed.handle.is_empty() { 0 } else { 1 };

        // the user does not have to specify his own allocator
        let has_allocator = cmd_parsed
            .parsed_args_in
            .last()
            .is_some_and(|(name, _)| name.to_string() == "p_allocator");

        let nb_to_take =
            cmd_parsed.parsed_args_in.len() - nb_to_skip - if has_allocator { 1 } else { 0 };

        let arg_outer_name: Vec<_> = cmd_parsed
            .parsed_args_in
            .iter()
            .skip(nb_to_skip)
            .take(nb_to_take)
            .map(|(x, _)| x)
            .collect();
        let arg_outer_type = cmd_parsed
            .parsed_args_in
            .iter()
            .skip(nb_to_skip)
            .take(nb_to_take)
            .map(|(_, y)| y);

        // remove the handle name from the function name
        let mut new_name = name.to_string();
        // vkGetDeviceImageMemoryRequirements and vkGetImageMemoryRequirements would resolve to the same name without the last check
        // this is also the case for vkGetDeviceSparseImageMemoryRequirements and vkGetDeviceBufferMemoryRequirements
        if !cmd_parsed.handle.is_empty() && !new_name.ends_with("memory_requirements") {
            // remove the Vk prefix
            let snake_case_handle = camel_case_to_snake_case(&cmd_parsed.handle["Vk".len()..]);
            new_name = new_name.replace(&snake_case_handle, "");

            if cmd_parsed.handle == "VkCommandBuffer" && new_name.starts_with("cmd_") {
                new_name = new_name["cmd_".len()..].to_owned();
            }
            new_name = new_name.replace("__", "_");
            if new_name.starts_with('_') {
                new_name = new_name[1..].to_owned();
            }
            if new_name.ends_with('_') {
                new_name = new_name[..(new_name.len() - 1)].to_owned();
            }
        }

        let fn_name = format_ident!("{new_name}");
        let raw_fn_name = format_ident!("{name}");

        let (ret_type, ret_template, pre_call, post_call) = match cmd.return_ty {
            ReturnType::BaseType(name) => {
                let ty_name = self.get_ident_name(name)?;
                (quote! (-> #ty_name), None, None, None)
            }
            ReturnType::Result if cmd_parsed.output_fields.is_empty() => {
                (quote! (-> Status), None, None, None)
            }
            _ if !cmd_parsed.output_fields.is_empty() => {
                let has_status = cmd.return_ty == ReturnType::Result;
                let (_, field) = cmd_parsed.output_fields[0];
                let ret_type = match field.ty {
                    Type::Ptr(name) => name,
                    _ => return Err(anyhow!("Could not use return field for {name}")),
                };
                let ret_name = self.get_ident_name(ret_type)?;
                let needs_transformation = is_complex_handle(ret_type);

                let is_vec = cmd_parsed.output_length.is_some()
                    || field.xml.altlen.is_some()
                    || field.xml.len.is_some();
                let is_structure_type = self
                    .get_struct(ret_type)
                    .is_some_and(|my_struct| my_struct.s_type.is_some());
                let is_handle = self.get_handle(ret_type).is_some();

                let lifetime = (!is_handle && self.compute_name_lifetime(ret_type))
                    .then(|| quote! (<'static>));
                let ret_param = if needs_transformation {
                    Some(quote! (<D,A>))
                } else {
                    lifetime
                };

                let pre_call = needs_transformation.then(|| {
                    let type_descr = is_vec.then(|| {
                        if has_status {
                            quote! (: Result<R::InnerArrayType>)
                        } else {
                            quote! (: R::InnerArrayType)
                        }
                    });
                    quote! (let vk_result #type_descr = )
                });
                let post_call = needs_transformation.then(|| {
                    match vk_name {
                        "vkCreateInstance" => {
                            return quote! {; vk_result.map(|instance| {
                                let disp = self.disp.clone_with_instance(&instance);
                                Instance::from_inner_with(instance, disp, self.alloc.clone())
                            })}
                        }
                        "vkCreateDevice" => {
                            return quote! {; vk_result.map(|device| {
                                let disp = self.disp.clone_with_device(&device);
                                Device::from_inner_with(device, disp, self.alloc.clone())
                            })}
                        }
                        _ => {}
                    };
                    let mapping_fn = if is_vec {
                        quote!(vk_result
                            .into_iter()
                            .map(|el| #ret_name::from_inner_with(el, self.disp.clone(), self.alloc.clone()))
                            .collect())
                    } else {
                        quote!(#ret_name::from_inner_with(vk_result, self.disp.clone(), self.alloc.clone()))
                    };
                    if has_status {
                        quote! (; vk_result.map(|vk_result| #mapping_fn))
                    } else {
                        quote! (; #mapping_fn)
                    }
                });

                let mut result_quote = quote! (#ret_name #ret_param);
                if is_vec {
                    result_quote = quote!(R)
                } else if is_structure_type {
                    result_quote = quote!(S)
                }
                if has_status {
                    result_quote = quote! (Result<#result_quote>)
                }

                let ret_template = if is_vec && needs_transformation {
                    Some(quote! (R: AdvancedDynamicArray<#ret_name #ret_param, raw::#ret_name>,))
                } else if is_vec {
                    Some(quote! (R: DynamicArray<#ret_name #ret_param>,))
                } else if is_structure_type {
                    Some(quote! (S: StructureChainOut<#ret_name #ret_param> ))
                } else {
                    None
                };
                (quote! (-> #result_quote), ret_template, pre_call, post_call)
            }
            _ => (quote!(), None, None, None),
        };

        let caller = if cmd_parsed.handle.is_empty() {
            quote!()
        } else if cmd.params[0].optional {
            quote!(Some(self),)
        } else {
            quote!(self,)
        };
        let allocator_param =
            has_allocator.then(|| quote!(self.alloc.get_allocation_callbacks().as_ref(),));

        let doc_tag = make_doc_link(vk_name);

        Ok(quote! {
            #doc_tag
            pub fn #fn_name<#ret_template #(#arg_template),*>(&self, #(#arg_outer_name: #arg_outer_type),*) #ret_type {
                #pre_call
                raw::#raw_fn_name(#caller #(#arg_outer_name,)* #allocator_param self.disp.get_command_dispatcher())
                #post_call
            }
        })
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
                _ => continue,
            };

            if param.is_const {
                if let Some(len) = &param.xml.len {
                    if len != "null-terminated" {
                        erase_from_vec(&mut simple_fields, len);
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
                    Ok((quote!(), (name.to_token_stream(), outer_type)))
                } else if vec_fields
                    .iter()
                    .any(|(vk_name, _)| *vk_name == param.vk_name)
                {
                    let (templ, outer, _) = self.generate_slice_type(
                        advanced_ty,
                        idx as u32,
                        name.clone(),
                        false,
                        false,
                    )?;
                    Ok((templ, (name.to_token_stream(), outer)))
                } else {
                    Ok((quote!(), (quote!(), quote!())))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let parsed_arg_templates = result_params
            .iter()
            .map(|(x, _)| x.clone())
            .filter(|t| !t.is_empty())
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
            AT::Void => quote!(()),
            AT::VoidPtr => quote!(*const ()),
            AT::Enum(ty) | AT::Other(ty) => {
                let ty_ident = self.get_ident_name(ty)?;
                quote! (#ty_ident)
            }
            AT::Handle(ty) => {
                let ty_ident = self.get_ident_name(ty)?;
                quote! (Option<#ty_ident>)
            }
            AT::Func(_ty) => {
                quote!(*const ())
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
                    quote!(())
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
                Ok(quote!(&#life_a #name))
            }
            AdvancedType::HandlePtr(name) => {
                let name = self.get_ident_name(name)?;
                Ok(quote! (&#life_a #name))
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
                    Ok(quote!(#name.map(|v| unsafe { v.clone() })))
                } else {
                    Ok(quote!(unsafe { #name.clone() }))
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

    /// First returned argument is the template parameter, second is the slice type, third is the affectation
    fn generate_slice_type(
        &self,
        ty: AdvancedType<'a>,
        index: u32,
        name: Ident,
        is_assignment: bool,
        with_lifetime: bool,
    ) -> Result<(TokenStream, TokenStream, TokenStream)> {
        let get_lifetime =
            |ty: &str| (with_lifetime && self.compute_name_lifetime(ty)).then(|| quote! (<'a>));
        let life_a = with_lifetime.then_some(quote! ('a));
        let template_ty = format_ident!("V{}", index);
        let simple_affectation = if is_assignment {
            quote! (self.#name = ptr::from_ref(#name).cast())
        } else {
            quote! (ptr::from_ref(#name).cast())
        };

        match ty {
            AdvancedType::HandlePtr(name) | AdvancedType::HandleArray(name, _) => {
                let name = self.get_ident_name(name)?;
                Ok((
                    quote! (#template_ty: Alias<raw::#name>),
                    quote! (&#life_a [#template_ty]),
                    simple_affectation,
                ))
            }
            AdvancedType::OtherPtr(name) => {
                let lifetime = get_lifetime(name);
                let name = self.get_ident_name(name)?;
                Ok((
                    quote!(),
                    quote! (&#life_a [#name #lifetime]),
                    simple_affectation,
                ))
            }
            AdvancedType::VoidPtr => {
                // binary data
                Ok((quote!(), quote!(&#life_a [u8]), simple_affectation))
            }
            AdvancedType::OtherDoublePtr(ty) => {
                let lifetime = get_lifetime(ty);
                let ty = if ty == "void" {
                    quote!(())
                } else {
                    format_ident!("{}", self.get_ident_name(ty)?).into_token_stream()
                };
                Ok((
                    quote!(),
                    quote! (&#life_a [&#life_a #ty #lifetime]),
                    simple_affectation,
                ))
            }
            AdvancedType::CStringPtr => Ok((
                quote!(),
                quote!(&#life_a [*const c_char]),
                simple_affectation,
            )),
            _ => Err(anyhow!("Trying to get array with unexpected type")),
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
            | AdvancedType::OtherDoubleArray(_, _, _) => Ok(quote!(Default::default())),
            AdvancedType::Other(name) => {
                // We can't use default if the basetype is the alias of a pointer
                self.structs
                    .get(name)
                    .filter(|my_struct| {
                        matches!(
                            my_struct,
                            Struct::BaseType(StructBasetype {
                                ty: Type::VoidPtr,
                                ..
                            })
                        )
                    })
                    .map_or_else(
                        || Ok(quote!(Default::default())),
                        |_| Ok(quote!(ptr::null())),
                    )
            }
            AdvancedType::Enum(name) => {
                // There is no default for regular enums
                let name = match self.mapping.borrow().get(name).map(|entry| entry.ty) {
                    Some(MappingType::Enum) => name,
                    Some(MappingType::AliasedEnum(alias_name)) => alias_name,
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
                        matches!(entry.ty, MappingType::AliasedEnum(_) | MappingType::Enum)
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
            // remove the indentation for the bitflags! closing bracket
            bitflag.replace_range((bitflag.len() - 6)..(bitflag.len() - 2), "");

            let _ = std::mem::replace(line, Cow::Owned(bitflag));
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
