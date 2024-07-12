use std::{
    collections::{BTreeMap, HashMap, HashSet},
    iter,
};

use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    helpers::camel_case_to_snake_case,
    structs::{CommandParamsParsed, ReturnType, Type},
    xml,
};

use super::{make_doc_link, GeneratedCommandType, Generator};

pub fn generate<'a, 'b>(gen: &'b Generator<'a>, gen_ty: GeneratedCommandType) -> Result<String> {
    let mut listed_cmds = HashSet::new();
    let mut listed_handles = HashSet::new();
    let mut handle_cmds: HashMap<&str, BTreeMap<usize, CommandParamsParsed>> = HashMap::new();
    let mut handles_order = Vec::new();
    let mut cmd_nb = 0usize;

    let mut result = Vec::new();

    for req_cnt in gen
        .filtered_features()
        .flat_map(|feat| &feat.require)
        .flat_map(|cnt| &cnt.content)
        .chain(
            gen.filtered_extensions()
                .flat_map(|ext| &ext.require)
                .flat_map(|cnt| &cnt.content),
        )
    {
        let req_cmd = match req_cnt {
            xml::RequireContent::Command(req_cmd) => req_cmd,
            xml::RequireContent::Type(xml::RequireType { name, .. }) => {
                if let Some(handle) = gen.handles.get(name.as_str()) {
                    if listed_handles.insert(name.as_str()) {
                        handles_order.push((name.as_str(), handle));
                    }
                }
                continue;
            }
            _ => continue,
        };
        let cmd = match gen.commands.get(req_cmd.name.as_str()) {
            Some(cmd) => cmd,
            None => continue,
        };
        if !listed_cmds.insert(cmd.vk_name) {
            continue;
        }
        let parsed_params = gen.parse_cmd_params(cmd)?;
        handle_cmds
            .entry(parsed_params.handle)
            .or_default()
            .insert(cmd_nb, parsed_params);
        cmd_nb += 1;
    }

    let is_complex_handle = |name: &str| handle_cmds.contains_key(name);

    let create_methods = |cmds: &BTreeMap<usize, CommandParamsParsed>| -> Result<Vec<TokenStream>> {
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
                        generate_advanced_command(
                            gen,
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
        #[derive(Clone)]
        pub struct Entry<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
            disp: D,
            alloc: A,
        }

        impl<D: Dispatcher, A: Allocator> Copy for Entry<D, A>
        where
            D: Copy,
            A: Copy,
        {
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

    for (handle_name, handle) in handles_order {
        if let Some(cmds) = handle_cmds.get(handle_name) {
            let id_name = format_ident!("{}", handle.name);
            let doc_tag = make_doc_link(handle_name);

            let methods = create_methods(cmds)?;
            let alias_impl = (gen_ty == GeneratedCommandType::Basic).then(|| {
                quote! {
                    unsafe impl Alias<raw::#id_name> for #id_name {}
                }
            });

            result.push(quote! {
                #[repr(C)]
                #[derive(Clone)]
                #doc_tag
                pub struct #id_name<D: Dispatcher = DynamicDispatcher, A: Allocator = DefaultAllocator> {
                    inner: <raw::#id_name as Handle>::InnerType,
                    disp: D,
                    alloc: A,
                }

                #alias_impl
                impl<D: Dispatcher, A:Allocator> Copy for #id_name<D,A> where
                D: Copy,
                A: Copy,
                {
                }

                impl<D: Dispatcher, A: Allocator> Deref for #id_name<D,A> {
                    type Target = raw::#id_name;

                    fn deref(&self) -> &Self::Target {
                        // Safety: raw::#id_name is repr(transparent) of raw::#id_name::InnerType
                        unsafe{ std::mem::transmute(&self.inner) }
                    }
                }

                impl<D: Dispatcher, A: Allocator> #id_name<D, A> {
                    pub unsafe fn from_inner(handle: raw::#id_name, disp: D, alloc: A) -> Self {
                        Self {
                            inner: handle.as_raw(),
                            disp,
                            alloc,
                        }
                    }

                    pub unsafe fn clone(&self) -> Self {
                        Self {
                            inner: self.inner.clone(),
                            disp: self.disp.clone(),
                            alloc: self.alloc.clone(),
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
            for alias_name in iter::once(handle.name).chain(handle.aliases.borrow().iter().copied())
            {
                let id_name = format_ident!("{alias_name}");
                let doc_tag = make_doc_link(&format!("Vk{alias_name}"));
                result.push(quote! (#doc_tag pub type #id_name = raw::#id_name;))
            }
        }
    }

    let result = quote! {
        use std::{
            ffi::{c_int, CStr},
            ops::Deref,
        };
        use crate::{vk::*, Alias, Allocator, AdvancedDynamicArray, AsSlice, DefaultAllocator, Dispatcher, DynamicArray, DynamicDispatcher, Handle, StructureChainOut};

        #(#result)*
    }
    .to_string();

    Generator::format_result(result)
}

fn generate_advanced_command<'a, 'b, F>(
    gen: &'b Generator<'a>,
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
            let ty_name = gen.get_ident_name(name)?;
            if name == "VkBool32" {
                (quote! (-> bool), None, None, None)
            } else {
                (quote! (-> #ty_name), None, None, None)
            }
        }
        ReturnType::Result { nb_successes, .. } if cmd_parsed.output_fields.is_empty() => (
            if nb_successes > 1 {
                quote! (-> Result<Status>)
            } else {
                quote! (-> Result<()>)
            },
            None,
            None,
            None,
        ),
        _ if !cmd_parsed.output_fields.is_empty() => {
            let has_status = matches!(cmd.return_ty, ReturnType::Result { .. });
            let has_many_successes = matches!(cmd.return_ty, ReturnType::Result { nb_successes, .. } if nb_successes > 1);
            let (_, field) = cmd_parsed.output_fields[0];
            let ret_type = match field.ty {
                Type::Ptr(name) => name,
                Type::DoublePtr("void") => "VoidPtr",
                _ => return Err(anyhow!("Could not use return field for {name}")),
            };
            let mut ret_name = gen.get_ident_name(ret_type)?;
            let needs_transformation = is_complex_handle(ret_type);

            let is_vec = cmd_parsed.output_length.is_some()
                || field.xml.altlen.is_some()
                || field.xml.len.is_some();
            let is_structure_type = gen
                .get_struct(ret_type)
                .is_some_and(|my_struct| my_struct.s_type.is_some());
            let is_handle = gen.get_handle(ret_type).is_some();

            if !is_vec && ret_type == "VkBool32" {
                ret_name = format_ident!("bool");
            }

            let lifetime =
                (!is_handle && gen.compute_name_lifetime(ret_type)).then(|| quote! (<'static>));
            let ret_param = if needs_transformation {
                Some(quote! (<D,A>))
            } else {
                lifetime
            };

            let pre_call = needs_transformation.then(|| {
                let type_descr = is_vec.then(|| {
                    if has_status {
                        if has_many_successes {
                            quote! (: Result<(Status, R::InnerArrayType)>)
                        } else {
                            quote! (: Result<R::InnerArrayType>)
                        }
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
                            unsafe { Instance::from_inner(instance, disp, self.alloc.clone()) }
                        })}
                    }
                    "vkCreateDevice" => {
                        return quote! {; vk_result.map(|device| {
                            let disp = self.disp.clone_with_device(&device);
                            unsafe { Device::from_inner(device, disp, self.alloc.clone()) }
                        })}
                    }
                    _ => {}
                };
                let mapping_fn = if is_vec {
                    quote!(vk_result
                        .into_iter()
                        .map(|el| unsafe{#ret_name::from_inner(el, self.disp.clone(), self.alloc.clone())})
                        .collect())
                } else {
                    quote!(unsafe {#ret_name::from_inner(vk_result, self.disp.clone(), self.alloc.clone())})
                };
                if has_status {
                    if has_many_successes {
                        quote! (; vk_result.map(|(status, vk_result)| (status, #mapping_fn)))
                    } else {
                        quote! (; vk_result.map(|vk_result| #mapping_fn))
                    }
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
                if has_many_successes {
                    result_quote = quote! ((Status, #result_quote))
                }
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
    let unsafe_tag = name.starts_with("destroy").then(|| quote!(unsafe));
    let lifetime = (!cmd_parsed.vec_fields.is_empty()).then(|| quote! ('a, ));

    Ok(quote! {
        #doc_tag
        pub #unsafe_tag fn #fn_name<#lifetime #ret_template #(#arg_template),*>(&self, #(#arg_outer_name: #arg_outer_type),*) #ret_type {
            #pre_call
            unsafe {
                raw::#raw_fn_name(#caller #(#arg_outer_name,)* #allocator_param self.disp.get_command_dispatcher())
            }
            #post_call
        }
    })
}
