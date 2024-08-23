use std::{cell::RefCell, collections::HashSet};

use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{generator::make_doc_link, structs::Handle, xml};

use super::{make_doc_link_inner, Generator};

pub fn generate<'a>(gen: &Generator<'a>) -> Result<String> {
    let listed_handles = RefCell::new(HashSet::new());

    let generate_group_handle = |require: &'a xml::Require| -> Result<TokenStream> {
        let handles = require
            .content
            .iter()
            .filter_map(|item| match item {
                xml::RequireContent::Type(xml::RequireType { name, .. }) => Some(name),
                _ => None,
            })
            .filter_map(|name| gen.handles.get(name.as_str()).map(|handle| (name, handle)))
            .filter(|(name, _)| listed_handles.borrow_mut().insert(*name))
            .map(|(name, handle)| generate_handle(gen, handle, name))
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! { #(#handles)*})
    };

    let feature_handles = gen
        .filtered_features()
        .flat_map(|feature| &feature.require)
        .map(|req| generate_group_handle(req))
        .collect::<Result<Vec<_>>>()?;
    let feature_extensions = gen
        .filtered_extensions()
        .flat_map(|ext| &ext.require)
        .map(|req| generate_group_handle(req))
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        use crate::private;
        use crate::{vk::ObjectType, Handle};

        use core::fmt;
        use std::num::{NonZeroU64, NonZeroUsize};

        #[macro_use]
        mod macros;

        #(#feature_handles)*
        #(#feature_extensions)*
    }
    .to_string();

    let formatted_result = Generator::format_result(result)?;
    Ok(formatted_result)
}

fn generate_handle<'a>(
    gen: &Generator<'a>,
    handle: &Handle<'a>,
    vk_name: &str,
) -> Result<TokenStream> {
    let name = format_ident!("{}", handle.name);
    let dispatch_macro = if handle.is_dispatchable {
        "handle_dispatchable"
    } else {
        "handle_nondispatchable"
    };
    let dispatch_macro = format_ident!("{dispatch_macro}");
    let doc_tag = make_doc_link_inner(vk_name);
    let mapping = gen.mapping.borrow();
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
        #dispatch_macro !{#name, #object_type, #doc_tag, #vk_name}
        #(#aliases)*
    })
}
