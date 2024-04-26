use std::{error::Error, fs, io::BufReader};

use generator::{Api, Generator};
use quick_xml::de::from_reader;
use xml::Registry;

mod generator;
mod xml;
mod helpers;

fn main() -> Result<(), Box<dyn Error>>{
    let file = std::fs::File::open("../vk.xml")?;
    let reader = BufReader::new(file);
    let registry: Registry = from_reader(reader)?;
    // print!("{:?}", registry);
    let generator = Generator::new(Api::Vulkan, &registry)?;
    let enums = generator.generate_enums()?;
    //println!("{}", enums);
    fs::write("src/enums.rs", enums)?;

    let handles = generator.generate_handles()?;
    fs::write("src/raw/handles.rs", handles)?;
    Ok(())
}
