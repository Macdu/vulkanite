use std::{error::Error, fs, io::BufReader, path::PathBuf};

use generator::{GeneratedCommandType, Generator};
use quick_xml::de::from_reader;
use structs::Api;
use xml::Registry;

mod generator;
mod helpers;
mod structs;
mod xml;

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("../vk.xml")?;
    let reader = BufReader::new(file);
    let registry: Registry = from_reader(reader)?;

    let generator = Generator::new(Api::Vulkan, &registry)?;

    let main_crate_name = "vulkanite";
    let crate_vk = PathBuf::from(&format!("{main_crate_name}/src/vk"));

    let extensions = generator.generate_extensions()?;
    fs::write(crate_vk.join("extensions.rs"), extensions)?;

    let enums = generator.generate_enums()?;
    fs::write(crate_vk.join("enums.rs"), enums)?;

    let handles = generator.generate_handles()?;
    fs::write(crate_vk.join("raw/handles.rs"), handles)?;

    let structs = generator.generate_structs()?;
    fs::write(crate_vk.join("structs.rs"), structs)?;

    let dispatcher = generator.generate_dispatcher()?;
    fs::write(crate_vk.join("dispatcher.rs"), dispatcher)?;

    let raw_commands = generator.generate_raw_commands()?;
    fs::write(crate_vk.join("raw/commands.rs"), raw_commands)?;

    let basic_commands = generator.generate_advanced_commands(GeneratedCommandType::Basic)?;
    fs::write(crate_vk.join("rs/commands.rs"), basic_commands)?;

    Ok(())
}
