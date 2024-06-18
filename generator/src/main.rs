use std::{error::Error, fs, io::BufReader};

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

    let enums = generator.generate_enums()?;
    fs::write("src/vk/enums.rs", enums)?;

    let handles = generator.generate_handles()?;
    fs::write("src/vk/raw/handles.rs", handles)?;

    let structs = generator.generate_structs()?;
    fs::write("src/vk/structs.rs", structs)?;

    let dispatcher = generator.generate_dispatcher()?;
    fs::write("src/vk/dispatcher.rs", dispatcher)?;

    let raw_commands = generator.generate_raw_commands()?;
    fs::write("src/vk/raw/commands.rs", raw_commands)?;

    let basic_commands = generator.generate_advanced_commands(GeneratedCommandType::Basic)?;
    fs::write("src/vk/rs/commands.rs", basic_commands)?;

    Ok(())
}
