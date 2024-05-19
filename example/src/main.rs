use std::error::Error;

use vk_headers::DynamicDispatcher;

fn main() -> Result<(), Box<dyn Error>>{
    unsafe {
        DynamicDispatcher::load_lib()?
    };
    
    Ok(())
}
