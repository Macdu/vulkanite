use std::error::Error;

use vk_headers::{raw, ApplicationInfo, DynamicDispatcher, InstanceCreateInfo, API_VERSION_1_0, DYNAMIC_DISPATCHER};

fn main() -> Result<(), Box<dyn Error>>{
    unsafe {
        DynamicDispatcher::load_lib()?
    };

    let app_info = ApplicationInfo::default();
    let mut instance_info = InstanceCreateInfo::default();
    instance_info.p_application_info(Some(&app_info));
    let _instance = raw::create_instance(&instance_info, None, &DYNAMIC_DISPATCHER).unwrap();
    
    Ok(())
}
