use std::error::Error;

use smallvec::SmallVec;
use vk_headers::{vk, DefaultAllocator, DynamicDispatcher};

fn main() -> Result<(), Box<dyn Error>> {
    let dispatcher = unsafe { DynamicDispatcher::new_loaded()? };
    let entry = vk::rs::Entry::new(dispatcher, DefaultAllocator);

    let mut app_info = vk::ApplicationInfo::default();
    app_info.api_version(vk::API_VERSION_1_1);
    let mut instance_info = vk::InstanceCreateInfo::default();
    instance_info.p_application_info(Some(&app_info));
    let instance = entry.create_instance(&instance_info)?;

    let devices: SmallVec<[_; 3]> = instance.enumerate_physical_devices()?;
    let device = &devices[0];
    let (_, vk11_props): (_, vk::PhysicalDeviceVulkan11Properties) = device.get_properties2_chain();
    println!("Subgroup size is {}", vk11_props.subgroup_size);
    Ok(())
}
