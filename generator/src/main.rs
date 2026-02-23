use std::{error::Error, fs, io::BufReader, path::PathBuf};

use anyhow::Context;
use generator::{GeneratedCommandType, Generator};
use quick_xml::de::from_reader;
use structs::Api;
use xml::Registry;

mod generator;
mod helpers;
mod structs;
mod xml;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(xml_path) = std::env::args().nth(1) else {
        eprintln!(
            "Usage: {} <path/to/vk.xml>",
            std::env::args()
                .next()
                .unwrap_or_else(|| "generator".to_string())
        );
        return Ok(());
    };

    println!("Parsing vk.xml from \"{xml_path}\"");
    let file = std::fs::File::open(&xml_path).context("Failed to open vk.xml file")?;
    let reader = BufReader::new(file);
    let registry: Registry = from_reader(reader)?;

    let generator = Generator::new(Api::Vulkan, &registry)?;

    let main_crate_name = "vulkanite";
    let crate_vk = PathBuf::from(&format!("{main_crate_name}/src/vk"));

    println!("Generating code");
    let cargo_path = PathBuf::from(&format!("{main_crate_name}/Cargo.toml"));
    let cargo_file = fs::read_to_string(&cargo_path).context("Failed to read Cargo.toml file")?;
    fs::write(cargo_path, generator.generate_features(cargo_file)?)?;

    let extensions = generator.generate_extensions()?;
    fs::write(crate_vk.join("extensions.rs"), extensions)?;

    let enums = generator.generate_enums()?;
    fs::write(crate_vk.join("enums.rs"), enums)?;

    let handles = generator.generate_handles()?;
    fs::write(crate_vk.join("raw/handles.rs"), handles)?;

    let structs = generator.generate_structs()?;
    fs::write(crate_vk.join("structs.rs"), structs)?;

    let formats = generator.generate_formats()?;
    fs::write(crate_vk.join("formats.rs"), formats)?;

    let dispatcher = generator.generate_dispatcher()?;
    fs::write(crate_vk.join("dispatcher.rs"), dispatcher)?;

    let raw_commands = generator.generate_raw_commands()?;
    fs::write(crate_vk.join("raw/commands.rs"), raw_commands)?;

    let basic_commands = generator.generate_advanced_commands(GeneratedCommandType::Basic)?;
    fs::write(crate_vk.join("rs/commands.rs"), basic_commands)?;

    println!("Code generation completed successfully");
    Ok(())
}
