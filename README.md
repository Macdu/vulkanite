<!-- cargo-rdme start -->

Unofficial bindings for the Vulkan Graphics API

The goal is to provide a nice and safer way to use Vulkan with Rust
while having in most cases no overhead.

These bindings try to replicate an experience similar to using the official Vulkan C++ bindings on Rust
which makes it differ from the popular Vulkan crate ash.

# Features
## Safety
Vulkan handles can be mostly be seen as references to Vulkan object. As such, using this crate, Vulkan handles **cannot be NULL**
(there is no vk::NULL_HANDLE value), being given a handle means you can assume it is not null and valid.
All vulkan functions which take as parameters handles which can be null now instead take as parameter an [`Option<Handle>`].

Concerning command safety, I decided to take an approach similar to the `cxx` crate: guarantee safety at the boundary between rust and Vulkan API/driver code (likely written in C). These bindings also try to ensure that anything that can be checked to be according to the Vulkan Specification while being mostly cost-free / ran at compile time is done this way.

When using the Vulkan API, driver code will be called which is possibly proprietary and on which you have no control. Even if you completely follow the Vulkan Specification and have no validation error, you might still get some surprise segfault when running your program on some GPUs/drivers (I speak from experience). As such the first solution would be to make every vulkan command or function calling a vulkan command unsafe, but this is from my point of view counter-productive. I chose to keep most Vulkan commands safe. The exceptions are destroy commands for which you must ensure everything created by what you are about to destroyed have already been destroyed.

Note that these bindings assume the driver implementation complies, at least minimally, with the Vulkan Specification. In particular if the driver returns a completely unkown `VkStatus` code (which is not allowed by the specification), this
will lead to undefined behavior in the rust code.

## Smart handles
Similar to the C++ bindings, this binding groups vulkan commands by the handle which 'executes' it. Therefore the owing code on ash:
```rust
unsafe {
    device.begin_command_buffer(
        &vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
    )
    ...
    swapchain_khr.queue_present_khr(queue, &present_info)?;
}
```
becomes:
```rust
cmd_buffer.begin(
    &vk::CommandBufferBeginInfo::default()
       .flags(vk::CommandBufferUsageFlags::OneTimeSubmit)
)?;

queue.present_khr(&present_info)?;
```

## Result
The Vulkan `VkResult` enum as been rename as [vk::Status] (this is the only enum/structure whose name is different ared to the C bindings).
Instead [`vk::Result<A>`] is defined as [Result<A, vk::Status>] and all Vulkan commands which return a Result in the inal specification instead
return a [vk::Result] now:
```rust
let device = instance.create_device(&device_info)?;
```
Moreover, if a Vulkan command can return multiple success codes, the status will be part of the result:
```rust
let (status, image_idx) = self.device.acquire_next_image_khr(
   &self.swapchain_objects.swapchain,
   u64::MAX,
   Some(semaphore),
   None, // not signaling any fence
)?;
if status == vk::Status::vk::Status::SuboptimalKHR {
    ...
}
```

## Slices
Every vulkan command or structure that takes as input a length along a raw pointer now takes as input a slice.
If a length is used for multiple raw pointers, the matching slices must be entered together and it is checked
that they have the same length.
```rust
impl CommandBuffer {
    pub fn set_viewport(&self, first_viewport: u32, viewports: impl AsSlice<vk::Viewport>);
}

cmd_buffer.set_viewport(0, &[vk::Viewport{..}, vk::Viewport{..}])

let color_attachment = vec![...];
let resolve_attachment = vec![...];
let subpass_description = vk::SubpassDescription::default()
    // the resolve attachment field is optional for SubpassDescription
    // but if it is supplied it must have the same length as color_attachment
    .color_attachment(&color_attachment, Some(&resolve_attachment))
    ....;
```

## Arrays as command outputs
For Vulkan commands that return an array (a length pointer and data pointer), it is instead possible
to use as output any structure implementing the [DynamicArray] (and [AdvancedDynamicArray] for the case of
smart handles). This is the case of [Vec]:
```rust
let surface_formats : Vec<_> = physical_device.get_surface_formats_khr(Some(surface))?;
```
The reason `: Vec<_>` has to be added is when using the `smallvec` feature, it is possible to do the following:
```rust
// won't make any heap allocation if you have less than 3 GPUs
let physical_devices: SmallVec<[_; 3]> = instance.enumerate_physical_devices()?;
```
Note that the case of [vk::Status::Incomplete] is handled by the implementation: you can assume [vk::Status::Incomplete]
is never returned

## Structure chain as command outputs
In case a vulkan command returns a structure that can be extended, a tuple can be used to specify which structure to ieve:
```rust
let (vk_props, vk11_props) : (_, vk::PhysicalDeviceVulkan11Properties) = physical_device.get_properties2();
println!("Max supported API is {}, Subgroup size is {}", vk_props.properties.api_version,  vk11_props.subgroup_size);
```
In case you don't want to use a structure chain, you have the 2 following choices (the type cannot be deduced explicitely in the default case):
```rust
let vk_props: vk::PhysicalDeviceProperties2 = physical_device.get_properties2();
let (vk_props,) = physical_device.get_properties2();
```

Note that the structure chain integrity is checked as compile time: the following code will lead to a compile error ::PhysicalDeviceVulkan11Features] cannot
be used in a structure chain whose head is [vk::PhysicalDeviceFeatures]):
```rust
let (_, _) : (_, vk::PhysicalDeviceVulkan11Features) = physical_device.get_properties2();
```
This compile-time check applies also to the next part:

## Structure chain as command inputs
The first possible way to build a structure chain is to use `structure.push_next(&mut next_structure)`. There are also iple
macros provided to do the same in a way similar to the C++ bindings:

```rust
let mut device_info = vk_headers::structure_chain!(
    vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_features(Some(&features))
        .enabled_extension(&required_extensions),
    vk::PhysicalDeviceShaderObjectFeaturesEXT::default().shader_object(true)
);

if does_not_support_extension {
    device_info.unlink::<vk::PhysicalDeviceShaderObjectFeaturesEXT>()
}

if does_not_support_feature {
    device_info.get_mut::<vk::PhysicalDeviceShaderObjectFeaturesEXT>().shader_object(false);
}

let device = physical_device.create_device(device_info.as_ref())?;
```

# Features

The following features are available:
- `loaded`: Allow the crate to dynamically load the vulkan library using the `libloading` crate, see [Dispatcher::new_loaded]
- `smallvec`: Add support for the smallvec crate to minimize heap allocations, enabling this feature allows the following: `let physical_devices: SmallVec<[_; 3]> = instance.enumerate_physical_devices()?;`.
- `raw-window-handle`: Add interoperability with the raw-window-handle crate, to create surfaces from raw handles, see the [window] module

# MSRV
The current MSRV for this crate is Rust 1.77 (C-String literals are heavily used). It is not planned to increase
this version anytime soon and if this happens a reasonable (at least 6 months old) MSRV will be chosen.

Note that these bindings are still a Work-In-Process, the public API may see breaking changes if this improves
safety or how nice to use the code is.

Please be aware that this crate should not be considered production ready yet, breaking changes are to be expected in the future versions.

<!-- cargo-rdme end -->
