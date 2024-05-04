use std::{error::Error, fs, io::BufReader};

use generator::Generator;
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
    fs::write("src/enums.rs", enums)?;

    let handles = generator.generate_handles()?;
    fs::write("src/raw/handles.rs", handles)?;

    let structs = generator.generate_structs()?;
    fs::write("src/structs.rs", structs)?;

    Ok(())
}
