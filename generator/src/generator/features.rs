use std::{collections::HashSet, convert::identity};

use anyhow::{Context, Result};

use crate::{
    generator::Generator,
    structs::{is_subfeature, remove_featext_prefix},
};

const CONFIG_DELIMITER: &str =
    "# Features below are automatically generated, do not edit them manually";

pub fn generate(gen: &Generator<'_>, cargo_file: String) -> Result<String> {
    let features_start = cargo_file
        .find(CONFIG_DELIMITER)
        .context("Failed to find the features section in Cargo.toml")?;

    let cargo_start = &cargo_file[..features_start + CONFIG_DELIMITER.len()];
    let mut last_feature = None;
    let versions = gen
        .filtered_features()
        .filter(|feat| !is_subfeature(&feat.name))
        .map(|feat| {
            gen.extensions_features
                .get(remove_featext_prefix(&feat.name))
        })
        .filter_map(identity)
        .filter(|feat| feat.is_non_trivial.get())
        .map(|feat| {
            let val = format!(
                "{} = [{}]",
                feat.name,
                last_feature.as_deref().unwrap_or("")
            );
            last_feature = Some(format!("\"{}\"", feat.name));
            val
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut ext_seen = HashSet::new();
    let extensions = gen
        .filtered_extensions()
        .map(|ext| {
            gen.extensions_features
                .get(remove_featext_prefix(&ext.name))
        })
        .filter_map(identity)
        .filter(|feat| feat.is_non_trivial.get())
        .filter(|feat| ext_seen.insert(&feat.name))
        .map(|feat| format!("{} = []", feat.name))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!("{cargo_start}\n{versions}\n\n{extensions}\n"))
}
