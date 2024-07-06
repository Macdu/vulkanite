use std::{cell::RefCell, collections::HashSet};

use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    structs::{CType, Constant, Enum, EnumAliased, EnumFlag, EnumValue, EnumVariant},
    xml,
};

use super::{make_doc_link, parse_value, Generator};

pub fn generate(gen: &Generator) -> Result<String> {
    let listed_enums = RefCell::new(HashSet::from(
        // we define TRUE and FALSE in lib.rs
        ["VK_TRUE", "VK_FALSE"],
    ));

    let feature_enums = gen
        .filtered_features()
        .map(|feature| generate_group_enums(gen, &feature.name, &feature.require, &listed_enums))
        .collect::<Result<Vec<_>>>()?;

    let extension_enums = gen
        .filtered_extensions()
        .map(|ext: &crate::xml::Extension| {
            generate_group_enums(gen, &ext.name, &ext.require, &listed_enums)
        })
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        use bitflags::bitflags;

        #(#feature_enums)*
        #(#extension_enums)*
    }
    .to_string();

    let formatted_result = Generator::format_result(result)?;
    Ok(Generator::bitflag_format_fixup(formatted_result))
}

fn generate_group_enums<'a>(
    gen: &Generator<'a>,
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
            let result = if let Some(constant) = gen.constants.get(name) {
                Some(generate_constant(gen, name, constant))
            } else if let Some(value) = gen.enums.get(name) {
                Some(generate_enum(gen, name, value))
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

fn generate_constant(gen: &Generator, const_name: &str, value: &Constant) -> Result<TokenStream> {
    let doc_tag = make_doc_link(const_name);
    match value {
        Constant::Aliased { name, alias } => {
            let (alias, ty) = gen
                .constants
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

fn generate_enum(_gen: &Generator, enum_name: &str, value: &Enum) -> Result<TokenStream> {
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
                let value: TokenStream = format!("1{} << {}", ty_token, bitpos).parse().unwrap();

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
