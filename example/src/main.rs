use std::ptr;
use std::{error::Error, ffi::CStr};

use smallvec::SmallVec;
use vk_headers::{structure_chain, vk, DynamicDispatcher, Handle, StructureChain0, DYNAMIC_DISPATCHER};
use vk_headers::StructureChain;

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { DynamicDispatcher::load_lib()? };

    let mut app_info = vk::ApplicationInfo::default();
    app_info.api_version(vk::API_VERSION_1_1);
    let mut instance_info = vk::InstanceCreateInfo::default();
    instance_info.p_application_info(Some(&app_info));
    let instance = vk::raw::create_instance(&instance_info, None, &DYNAMIC_DISPATCHER).unwrap();

    unsafe { DynamicDispatcher::load_instance(&instance) };

    let devices: SmallVec<[_; 3]> =
        vk::raw::enumerate_physical_devices(&instance, &DYNAMIC_DISPATCHER)?;
    let device = &devices[0];
    let chain: structure_chain!(_, vk::PhysicalDeviceVulkan11Properties) = vk::raw::get_physical_device_properties2_chain(&device, &DYNAMIC_DISPATCHER);
    let vk11_props: &vk::PhysicalDeviceVulkan11Properties = chain.get();
    println!("Subgroup size is {}", vk11_props.subgroup_size);
    Ok(())
}
