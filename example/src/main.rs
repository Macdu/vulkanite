use std::error::Error;

use vk_headers::{vk, DynamicDispatcher, DYNAMIC_DISPATCHER};

fn main() -> Result<(), Box<dyn Error>>{
    unsafe {
        DynamicDispatcher::load_lib()?
    };

    let app_info = vk::ApplicationInfo::default();
    let mut instance_info = vk::InstanceCreateInfo::default();
    instance_info.p_application_info(Some(&app_info));
    let _instance = vk::raw::create_instance(&instance_info, None, &DYNAMIC_DISPATCHER).unwrap();
    
    Ok(())
}
