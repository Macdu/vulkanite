use std::{cell::RefCell, collections::HashSet, iter};

use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    generator::make_doc_link,
    helpers::convert_len_case,
    structs::{CommandParamsParsed, ReturnType, Type},
    xml,
};

use super::Generator;

pub fn generate<'a>(gen: &Generator<'a>) -> Result<String> {
    let listed_commands = RefCell::new(HashSet::new());
    let generate_group_commands = |require: &'a xml::Require| -> Result<TokenStream> {
        let cmds = require
            .content
            .iter()
            .filter_map(|req| match req {
                xml::RequireContent::Command(cmd) => gen
                    .commands
                    .get(cmd.name.as_str())
                    .filter(|_| listed_commands.borrow_mut().insert(&cmd.name))
                    .map(|cmd| {
                        let cmd_params = gen.parse_cmd_params(cmd)?;
                        // generate the command for the main name and all the possible aliases
                        let aliases = cmd.aliases.borrow();
                        let raw_cmds = iter::once((cmd.vk_name, cmd.name.as_str()))
                            .chain(
                                aliases
                                    .iter()
                                    .map(|(vk_name, name)| (*vk_name, name.as_str())),
                            )
                            .map(|(vk_name, name)| {
                                generate_raw_command(gen, &cmd_params, vk_name, name)
                            })
                            .collect::<Result<Vec<_>>>()?;
                        Ok(quote! (#(#raw_cmds)*))
                    }),
                _ => None,
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! (#(#cmds)*))
    };

    let raw_cmd_features = gen.filtered_features().flat_map(|feat| &feat.require);
    let raw_cmd_extensions = gen
        .filtered_extensions()
        .flat_map(|ext: &xml::Extension| &ext.require);

    let raw_cmd_impl = raw_cmd_features
        .chain(raw_cmd_extensions)
        .map(generate_group_commands)
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        // TODO: remove
        #![allow(unused_unsafe)]
        #![allow(unused_mut)]

        use std::ffi::{c_int, CStr};

        use crate::*;
        use crate::vk::*;
        use crate::vk::raw::{self,*};

        #(#raw_cmd_impl)*
    }
    .to_string();

    Generator::format_result(result)
}

fn generate_raw_command<'a, 'b>(
    gen: &'b Generator<'a>,
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
                let outer_to_inner_type =
                    gen.generate_type_outer_to_inner(&advanced_ty, param.optional, name.clone())?;
                Ok(outer_to_inner_type)
            } else if vec_fields
                .iter()
                .any(|(vk_name, _)| *vk_name == param.vk_name)
            {
                let slice_ty = gen.generate_slice_type(
                    advanced_ty,
                    idx as u32,
                    &name,
                    None,
                    false,
                    param.optional,
                )?;
                Ok(slice_ty.affectation)
            } else if output_fields
                .iter()
                .any(|(vk_name, _)| *vk_name == param.vk_name)
            {
                let param_name = match param.ty {
                    Type::Ptr(name) => name,
                    Type::DoublePtr("void") => "VoidPtr",
                    _ => return Err(anyhow!("Param {} should be a pointer", param.vk_name)),
                };
                let is_structure_type = gen
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
                Ok(quote! (#vec_field_name.as_slice().len() as _))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let templates = &parsed_arg_templates;
    let args_outer_name = parsed_args_in.iter().map(|(x, _)| x);
    let args_outer_type = parsed_args_in.iter().map(|(_, y)| y);

    let (ret_type, pre_call, post_call, ret_template, inner_call) = match cmd.return_ty {
        ReturnType::BaseType(name) => {
            let ty_name = gen.get_ident_name(name)?;
            if name == "VkBool32" {
                (quote! (-> bool), None, Some(quote!(.into())), None, None)
            } else {
                (quote! (-> #ty_name), None, None, None, None)
            }
        }
        ReturnType::Result { nb_successes, .. } if output_fields.is_empty() => (
            if nb_successes > 1 {
                quote! (-> Result<Status>)
            } else {
                quote! (-> Result<()>)
            },
            None,
            if nb_successes > 1 {
                Some(quote! (.into_result()))
            } else {
                Some(quote! (.map_success(|| ())))
            },
            None,
            None,
        ),
        _ if !output_fields.is_empty() => {
            let has_status = matches!(cmd.return_ty, ReturnType::Result { .. });
            let has_many_successes = matches!(cmd.return_ty, ReturnType::Result { nb_successes, .. } if nb_successes > 1);
            let (_, field) = output_fields[0];
            let ret_type = match field.ty {
                Type::Ptr(name) => name,
                Type::DoublePtr("void") => "VoidPtr",
                _ => return Err(anyhow!("Could not use return field for {name}")),
            };
            let mut ret_name = gen.get_ident_name(ret_type)?;
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
                        Ok(quote! (#param_name.as_slice().len()))
                    } else {
                        convert_len_case(len).parse::<TokenStream>()
                    }
                })
                .transpose()
                .map_err(|_| anyhow!("Failed to parse length of {}", field.vk_name))?;

            let is_vec = external_length.is_some() || internal_length.is_some();
            let is_structure_type = gen
                .get_struct(ret_type)
                .is_some_and(|my_struct| my_struct.s_type.is_some());
            let is_handle = gen.get_handle(ret_type).is_some();

            let lifetime =
                (!is_handle && gen.compute_name_lifetime(ret_type)).then(|| quote! (<'static>));

            if !is_vec && ret_type == "VkBool32" {
                assert!(has_status);
                ret_name = format_ident!("bool");
            }

            let mut result_quote = quote! (#ret_name #lifetime);
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
            let prev_affectation = has_status.then(|| quote! (let vk_status = ));
            let return_cast = if let Some(internal_length) = &internal_length {
                quote! (#field_name.resize_with_len(#internal_length as _); #field_name)
            } else if external_length.is_some() {
                quote!(vk_vec.resize_with_len(vk_len as _); vk_vec)
            } else if is_structure_type {
                quote! (S::setup_cleanup(#field_name.as_mut_ptr());#field_name.assume_init())
            } else if ret_type == "VkBool32" {
                quote! (#field_name.assume_init().into())
            } else {
                quote! (#field_name.assume_init())
            };
            let return_result = match (has_status, has_many_successes) {
                (true, true) => quote! (; vk_status.map_successes(|| {#return_cast})),
                (true, false) => quote! (; vk_status.map_success(|| {#return_cast})),
                _ => quote! (; #return_cast),
            };
            let param_init = if let Some(internal_length) = &internal_length {
                Some(
                    quote! (let mut #field_name = R::create_with_capacity(#internal_length as _); #prev_affectation),
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
                    let mut vk_vec = R::create_with_capacity(vk_len as _);
                    let mut #external_length = ptr::from_mut(&mut vk_len);
                    let mut #field_name = vk_vec.get_content_mut_ptr();
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
            let inner_call = match (&cmd.return_ty, external_length) {
                (ReturnType::Result { has_incomplete, .. }, Some(external_length))
                    if *has_incomplete =>
                {
                    Some(quote! {
                        loop {
                            let status = vulkan_command(#(#args_inner),*);
                            if status != Status::Incomplete {
                                break status;
                            }
                            vk_vec.update_with_capacity(vk_len as _);
                            #external_length = ptr::from_mut(&mut vk_len);
                            #field_name = vk_vec.get_content_mut_ptr();
                        }
                    })
                }
                _ => None,
            };
            (
                quote! (-> #result_quote),
                Some(param_init),
                Some(return_result),
                ret_template,
                inner_call,
            )
        }
        _ => (quote!(), None, None, None, None),
    };

    let inner_call = inner_call.unwrap_or_else(|| quote! (vulkan_command(#(#args_inner),*)));

    let func_name = format_ident!("{name}");
    let doc = make_doc_link(vk_name);
    let lifetime = (!vec_fields.is_empty()).then(|| quote! ('a, ));
    Ok(quote! {
        #doc
        pub unsafe fn #func_name<#lifetime #ret_template #(#templates),*>(#(#args_outer_name: #args_outer_type,)* dispatcher: &CommandsDispatcher ) #ret_type {
            let vulkan_command = dispatcher.#func_name.get().expect("Vulkan command not loaded.");
            #pre_call
            #inner_call
            #post_call
        }
    })
}
