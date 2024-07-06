use std::{cell::RefCell, collections::HashSet, ffi::CString, iter};

use anyhow::{anyhow, Result};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, LitCStr};

use crate::{
    structs::{Command, ReturnType},
    xml,
};

use super::Generator;

pub fn generate<'a>(gen: &Generator<'a>) -> Result<String> {
    let listed_commands = RefCell::new(HashSet::new());
    let mut proc_addr_loader = Vec::new();
    let mut instance_loader = Vec::new();
    let mut device_loader = Vec::new();

    let generate_group_dispatcher = |require: &'a xml::Require| -> Result<TokenStream> {
        let cmds = require
            .content
            .iter()
            .filter_map(|req| match req {
                xml::RequireContent::Command(cmd) => gen
                    .commands
                    .get(cmd.name.as_str())
                    .filter(|_| listed_commands.borrow_mut().insert(&cmd.name))
                    .map(|cmd| {
                        generate_dispatch_command(
                            gen,
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

    let dispatcher_features = gen.filtered_features().flat_map(|feat| &feat.require);
    let dispatcher_extensions = gen
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
        use std::ffi::{c_char, c_int, c_void};

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

    Generator::format_result(result)
}

fn generate_dispatch_command<'a>(
    gen: &Generator<'a>,
    cmd: &Command<'a>,
    proc_addr_loader: &mut Vec<TokenStream>,
    instance_loader: &mut Vec<TokenStream>,
    device_loader: &mut Vec<TokenStream>,
) -> Result<TokenStream> {
    let ret_type = match cmd.return_ty {
        ReturnType::Void => quote!(),
        ReturnType::Result { .. } => quote! (-> Status),
        ReturnType::BaseType(name) => {
            let name = gen
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
        .map(|param| gen.generate_type_inner(&param.advanced_ty.get().unwrap(), false))
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
