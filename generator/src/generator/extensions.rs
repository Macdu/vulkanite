use std::{collections::HashSet, ffi::CString};

use anyhow::{bail, Context, Result};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::LitCStr;

use crate::{structs::remove_featext_prefix, xml};

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
        .context("Failed to find VK_HEADER_VERSION")?;

    let mut extensions_deps_checked = HashSet::new();

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
                .with_context(|| format!("Extension {ext:?} should have a require block"))?
                .content
                .iter()
                .filter(|cnt| !matches!(cnt, xml::RequireContent::Comment(_)));

            let (name_enum, spec_enum) = match (req_block.next(), req_block.next()) {
                (Some(xml::RequireContent::Enum(spec)), Some(xml::RequireContent::Enum(name))) => {
                    (name, spec)
                }
                _ => bail!("Extension {ext:?} should start with two enums"),
            };

            if !spec_enum.name.ends_with("_SPEC_VERSION") {
                bail!("{spec_enum:?} should end with _SPEC_VERSION");
            }
            if !name_enum.name.ends_with("_EXTENSION_NAME") {
                bail!("{name_enum:?} should end with _EXTENSION_NAME");
            }

            let spec_version = spec_enum
                .value
                .as_ref()
                .map(|v| v.parse::<u32>().ok())
                .unwrap_or_default()
                .with_context(|| format!("{:?} should be an integer", spec_enum.value))?;
            let ext_name = name_enum
                .value
                .as_ref()
                .map(|v| (v.len() >= 2).then(|| &v[1..(v.len() - 1)]))
                .unwrap_or_default()
                .with_context(|| format!("{:?} should be in quotes", name_enum.value))?;
            let ext_name = LitCStr::new(&CString::new(ext_name).unwrap(), Span::call_site());

            let ext_ident = format_ident!("{}", ext.name["VK_".len()..].to_ascii_uppercase());
            let ext_feature = gen
                .extensions_features
                .get(remove_featext_prefix(&ext.name))
                .context("Failed to find extension")?;
            let feat_name = &ext_feature.name;
            let config = ext_feature.is_non_trivial.get().then(|| {
                quote! { #[cfg(feature = #feat_name)] }
            });

            let dep_check = (ext_feature.is_non_trivial.get()
                && extensions_deps_checked.insert(feat_name)
                && !gen.has_trivial_dep(&ext_feature.dependencies))
            .then(|| {
                let dep_inner = gen.get_config_feature_inner(&ext_feature.dependencies);
                let config_readable = gen
                    .get_config_readable(&ext_feature.dependencies)
                    .unwrap_or_default();
                let error_msg = format!(
                    "The feature {} requires {} to be enabled.",
                    feat_name, config_readable
                );
                quote! {
                    #[cfg(all(feature = #feat_name, not(#dep_inner)))]
                    compile_error!(#error_msg);
                }
            });

            Ok(quote! {
                #config
                pub const #ext_ident :#ext_class = #ext_class{
                    name: unsafe {#name_class::new(#ext_name)},
                    spec: #spec_version
                };
                #dep_check
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        use super::{DeviceExtension, DeviceExtensionName, InstanceExtension, InstanceExtensionName, ApiVersion};

        pub const HEADER_VERSION: ApiVersion = ApiVersion::new(0, 1, 4, #header_version);

        #(#extensions)*
    }
    .to_string();

    let formatted_result = Generator::format_result(result)?;
    Ok(formatted_result)
}
