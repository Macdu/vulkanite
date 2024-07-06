use std::ffi::CString;

use anyhow::{anyhow, Result};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::LitCStr;

use crate::xml;

use super::Generator;

pub fn generate(gen: &Generator) -> Result<String> {
    // also bundle the header version along with all the extensions
    let header_version = gen
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

    let extensions = gen
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
                (Some(xml::RequireContent::Enum(spec)), Some(xml::RequireContent::Enum(name))) => {
                    (name, spec)
                }
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

    let formatted_result = Generator::format_result(result)?;
    Ok(formatted_result)
}
