use std::{error::Error, ffi::CStr};

use vk_headers::{vk, DynamicDispatcher, DYNAMIC_DISPATCHER};

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { DynamicDispatcher::load_lib()? };

    let app_info = vk::ApplicationInfo::default();
    let mut instance_info = vk::InstanceCreateInfo::default();
    instance_info.p_application_info(Some(&app_info));
    let instance = vk::raw::create_instance(&instance_info, None, &DYNAMIC_DISPATCHER).unwrap();

    unsafe { DynamicDispatcher::load_instance(&instance) };

    let devices = vk::raw::enumerate_physical_devices(&instance, &DYNAMIC_DISPATCHER)?;
    let device = &devices[0];
    let prop = vk::raw::get_physical_device_properties(device, &DYNAMIC_DISPATCHER);
    let name = unsafe {
        CStr::from_ptr(prop.device_name.as_ptr())
    };
    println!("GPU Name is {}", name.to_str()?);
    Ok(())
}
