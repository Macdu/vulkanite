use core::slice;
use std::{collections::HashSet, error::Error, ffi::CStr};

use anyhow::{anyhow, Result};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle};
use smallvec::{smallvec, SmallVec};
use vulkanite::{
    flagbits, include_spirv,
    vk::{self, PipelineVertexInputStateCreateInfo},
    window, DefaultAllocator, Dispatcher, DynamicDispatcher,
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct Application {
    window: Option<Window>,
    vulkan_app: Option<VulkanApplication>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Vulkan")
            .with_inner_size(LogicalSize::new(800, 600));
        let window = event_loop.create_window(window_attributes).unwrap();

        if self.vulkan_app.is_none() {
            // First time the window is created, initialize all the Vulkan parts
            self.vulkan_app = Some(VulkanApplication::init(&window).unwrap())
        }
        self.window = Some(window);
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        if self.window.is_none() {
            return;
        }

        if let Some(vulkan_app) = self.vulkan_app.as_mut() {
            vulkan_app.render_frame().unwrap();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window.is_none() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            //WindowEvent::RedrawRequested => {}
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    // Always try to render a frame, even if there are no events
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct VulkanApplication {
    instance: vk::rs::Instance,
    debug_messenger: Option<vk::rs::DebugUtilsMessengerEXT>,
    device: vk::rs::Device,
    queue: vk::rs::Queue,
    command_pool: vk::rs::CommandPool,
    cmd_buffers: SmallVec<[vk::rs::CommandBuffer; 3]>,
    fences: SmallVec<[vk::rs::Fence; 3]>,
    next_index: usize,
    image_available: SmallVec<[vk::rs::Semaphore; 3]>,
    render_finished: SmallVec<[vk::rs::Semaphore; 3]>,
    surface: vk::rs::SurfaceKHR,
    swapchain_objects: SwapchainObjects,
    render_pass: vk::rs::RenderPass,
    shader_modules: [vk::rs::ShaderModule; 2],
    pipeline_layout: vk::rs::PipelineLayout,
    pipeline: vk::rs::Pipeline,
}

extern "system" fn debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _ty: vk::DebugUtilsMessageTypeFlagsEXT,
    data: &vk::DebugUtilsMessengerCallbackDataEXT,
    _: *const (),
) -> vk::Bool32 {
    eprintln!("Validation layer: {:?}", unsafe {
        CStr::from_ptr(data.p_message)
    });
    vk::FALSE
}

impl VulkanApplication {
    pub fn init(window: &Window) -> Result<Self> {
        let display_handle = window.display_handle()?.as_raw();
        let window_handle = window.window_handle()?.as_raw();

        let dispatcher = unsafe { DynamicDispatcher::new_loaded()? };
        let entry = vk::rs::Entry::new(dispatcher, DefaultAllocator);
        let (instance, debug_messenger) = Self::create_instance(&entry, &display_handle)?;
        let surface = window::rs::create_surface(&instance, &display_handle, &window_handle)?;
        let (physical_device, device, queue, command_pool) =
            Self::create_device(&instance, &surface)?;
        let mut swapchain_objects =
            SwapchainObjects::create(&physical_device, &device, &surface, window.inner_size())?;

        let render_pass = Self::create_render_pass(&device, swapchain_objects.format)?;
        let (shader_modules, pipeline_layout, pipeline) =
            Self::create_graphics_pipelines(&device, &render_pass)?;

        swapchain_objects.create_framebuffers(&device, &render_pass)?;
        let cmd_buffers = Self::create_command_buffers(
            &device,
            &command_pool,
            swapchain_objects.swapchain_images.len(),
        )?;
        let (fences, [image_available, render_finished]) =
            Self::create_sync_objects(&device, swapchain_objects.swapchain_images.len())?;

        Ok(Self {
            instance,
            debug_messenger,
            device,
            queue,
            command_pool,
            cmd_buffers,
            fences,
            next_index: 0,
            image_available,
            render_finished,
            surface,
            swapchain_objects,
            render_pass,
            shader_modules,
            pipeline_layout,
            pipeline,
        })
    }

    pub fn create_instance(
        entry: &vk::rs::Entry,
        display_handle: &RawDisplayHandle,
    ) -> Result<(vk::rs::Instance, Option<vk::rs::DebugUtilsMessengerEXT>)> {
        // check for validation layers
        const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
        let layers: Vec<_> = entry.enumerate_instance_layer_properties()?;
        let has_validation = layers
            .into_iter()
            .any(|layer| layer.get_layer_name() == VALIDATION_LAYER);
        let enabled_layers = has_validation.then_some(VALIDATION_LAYER.as_ptr());

        // enable VK_EXT_debug_utils only if the validation layer is enabled
        let mut enabled_extensions =
            Vec::from(window::enumerate_required_extensions(display_handle)?);
        enabled_extensions.push(vk::EXT_DEBUG_UTILS.name);

        let app_info = vk::ApplicationInfo::default()
            .application_name(Some(c"Hello Triangle"))
            .engine_name(Some(c"No Engine"))
            .api_version(vk::API_VERSION_1_0);

        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(Some(&app_info))
            .enabled_extension(enabled_extensions.as_slice())
            .enabled_layer(enabled_layers.as_slice());

        let instance = entry.create_instance(&instance_info)?;

        let debug_messenger = if has_validation {
            // setup the debug callback
            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    flagbits!(vk::DebugUtilsMessageSeverityFlagsEXT::{Info | Warning | Error}),
                )
                .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                .pfn_user_callback(Some(debug_callback));
            Some(instance.create_debug_utils_messenger_ext(&debug_info)?)
        } else {
            None
        };

        Ok((instance, debug_messenger))
    }

    pub fn create_device(
        instance: &vk::rs::Instance,
        surface: &vk::rs::SurfaceKHR,
    ) -> Result<(
        vk::rs::PhysicalDevice,
        vk::rs::Device,
        vk::rs::Queue,
        vk::rs::CommandPool,
    )> {
        let physical_devices: SmallVec<[_; 3]> = instance.enumerate_physical_devices()?;

        let compute_device_score = |physical_device: &vk::rs::PhysicalDevice| {
            let properties = physical_device.get_properties();
            let is_discrete = properties.device_type == vk::PhysicalDeviceType::DiscreteGpu;
            let max_2d_dim = properties.limits.max_image_dimension2_d;

            // compute a score based on if the gpu is discrete and the maximal supported 2d image dimension
            (is_discrete as u32) * 10000 + max_2d_dim
        };

        let physical_device = physical_devices
            .into_iter()
            .max_by_key(compute_device_score)
            .ok_or_else(|| anyhow!("Failed to find a Vulkan compatible GPU"))?;

        let (queue_family, _) = physical_device
            .get_queue_family_properties::<Vec<_>>()
            .into_iter()
            .enumerate()
            .find(|(queue, props)| {
                props.queue_flags.contains(vk::QueueFlags::Graphics)
                    && physical_device
                        .get_surface_support_khr(*queue as u32, surface)
                        .is_ok_and(|supported| supported.into())
            })
            .ok_or_else(|| anyhow!("Failed to find a suitable GPU queue"))?;

        let features = vk::PhysicalDeviceFeatures::default();

        let required_extensions = [vk::KHR_SWAPCHAIN.name];
        let mut missing_extensions: HashSet<&CStr> =
            required_extensions.iter().map(|ext| ext.get()).collect();
        for extension_prop in
            physical_device.enumerate_device_extension_properties::<Vec<_>>(None)?
        {
            missing_extensions.remove(extension_prop.get_extension_name());
        }

        if !missing_extensions.is_empty() {
            return Err(anyhow!(
                "The following required device extensions are missing : {missing_extensions:?}"
            ));
        }

        let queue_prio = 1.0f32;
        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family as u32)
            .queue_priorities(slice::from_ref(&queue_prio));

        let device_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(slice::from_ref(&queue_info))
            .enabled_features(Some(&features))
            .enabled_extension(&required_extensions);

        let device = physical_device.create_device(&device_info)?;
        let queue = device.get_queue(queue_family as u32, 0);

        let command_pool = device.create_command_pool(
            &vk::CommandPoolCreateInfo::default()
                .flags(vk::CommandPoolCreateFlags::ResetCommandBuffer)
                .queue_family_index(queue_family as u32),
        )?;
        Ok((physical_device, device, queue, command_pool))
    }

    fn create_render_pass(
        device: &vk::rs::Device,
        format: vk::Format,
    ) -> Result<vk::rs::RenderPass> {
        let color_attachment = vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlags::Count1)
            .load_op(vk::AttachmentLoadOp::Clear)
            .store_op(vk::AttachmentStoreOp::Store)
            .stencil_load_op(vk::AttachmentLoadOp::DontCare)
            .stencil_store_op(vk::AttachmentStoreOp::DontCare)
            .initial_layout(vk::ImageLayout::Undefined)
            .final_layout(vk::ImageLayout::PresentSrcKHR);

        let color_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::ColorAttachmentOptimal);

        let subpass_dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::ColorAttachmentOutput)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::ColorAttachmentOutput)
            .dst_access_mask(vk::AccessFlags::ColorAttachmentWrite);

        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::Graphics)
            .color_attachment(slice::from_ref(&color_ref), None);

        let render_pass = device.create_render_pass(
            &vk::RenderPassCreateInfo::default()
                .attachments(slice::from_ref(&color_attachment))
                .subpasses(slice::from_ref(&subpass))
                .dependencies(slice::from_ref(&subpass_dependency)),
        )?;

        Ok(render_pass)
    }

    fn create_graphics_pipelines(
        device: &vk::rs::Device,
        render_pass: &vk::rs::RenderPass,
    ) -> Result<(
        [vk::rs::ShaderModule; 2],
        vk::rs::PipelineLayout,
        vk::rs::Pipeline,
    )> {
        /*
           Compiled with glslangValidator -V ./shader.vert

           #version 450

           vec2 positions[3] = vec2[](
               vec2(0.0, -0.5),
               vec2(0.5, 0.5),
               vec2(-0.5, 0.5)
           );

           vec3 colors[3] = vec3[](
               vec3(1.0, 0.0, 0.0),
               vec3(0.0, 1.0, 0.0),
               vec3(0.0, 0.0, 1.0)
           );


           layout(location = 0) out vec3 fragColor;

           void main() {
               gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
               fragColor = colors[gl_VertexIndex];
           }
        */
        let vertex_shader_code = include_spirv!("vert.spv");
        /*
           Compiled with glslangValidator -V ./shader.frag

           #version 450

           layout(location = 0) in vec3 fragColor;
           layout(location = 0) out vec4 outColor;

           void main() {
               outColor = vec4(fragColor, 1.0);
           }
        */
        let fragment_shader_code = include_spirv!("frag.spv");

        let vertex_module = device.create_shader_module(
            &vk::ShaderModuleCreateInfo::default().code(vertex_shader_code),
        )?;
        let fragment_module = device.create_shader_module(
            &vk::ShaderModuleCreateInfo::default().code(fragment_shader_code),
        )?;

        let shaders = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::Vertex)
                .module(Some(&vertex_module))
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::Fragment)
                .module(Some(&fragment_module))
                .name(c"main"),
        ];

        let dynamic_state = [vk::DynamicState::Viewport, vk::DynamicState::Scissor];
        let dynamic_info =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_state);

        let vertex_input = PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TriangleList);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::Fill)
            .line_width(1.0f32)
            .cull_mode(vk::CullModeFlags::Back)
            .front_face(vk::FrontFace::Clockwise);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples_with_mask(vk::SampleCountFlags::Count1, None);
        let blending_attach = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::all());
        let blending = vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(slice::from_ref(&blending_attach));

        let pipeline_layout =
            device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default())?;

        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(shaders.as_slice())
            .vertex_input_state(Some(&vertex_input))
            .input_assembly_state(Some(&input_assembly))
            .viewport_state(Some(&viewport_state))
            .rasterization_state(Some(&rasterizer))
            .multisample_state(Some(&multisampling))
            .color_blend_state(Some(&blending))
            .dynamic_state(Some(&dynamic_info))
            .layout(Some(&pipeline_layout))
            .render_pass(Some(&render_pass))
            .subpass(0);

        let (status, mut pipeline): (_, SmallVec<[_; 1]>) =
            device.create_graphics_pipelines(None, slice::from_ref(&pipeline_info))?;
        // Status can technically be vk::Status::PipelineCompileRequired
        assert!(status == vk::Status::Success);

        Ok((
            [vertex_module, fragment_module],
            pipeline_layout,
            pipeline.remove(0),
        ))
    }

    fn create_sync_objects(
        device: &vk::rs::Device,
        count: usize,
    ) -> Result<(
        SmallVec<[vk::rs::Fence; 3]>,
        [SmallVec<[vk::rs::Semaphore; 3]>; 2],
    )> {
        let fences = (0..count)
            .into_iter()
            .map(|_| {
                device.create_fence(
                    &vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::Signaled),
                )
            })
            .collect::<vk::Result<_>>()?;

        let create_semaphores = || {
            (0..count)
                .into_iter()
                .map(|_| device.create_semaphore(&Default::default()))
                .collect::<vk::Result<_>>()
        };

        Ok((fences, [create_semaphores()?, create_semaphores()?]))
    }

    fn create_command_buffers(
        device: &vk::rs::Device,
        command_pool: &vk::rs::CommandPool,
        count: usize,
    ) -> Result<SmallVec<[vk::rs::CommandBuffer; 3]>> {
        Ok(device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::Primary)
                .command_buffer_count(count as u32),
        )?)
    }

    fn record_command_buffer(
        &self,
        cmd_buffer: &vk::rs::CommandBuffer,
        image_idx: usize,
    ) -> Result<()> {
        cmd_buffer.reset(Default::default())?;
        cmd_buffer.begin(
            &vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::OneTimeSubmit),
        )?;

        let clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
            },
        };

        let extent = self.swapchain_objects.extent;
        cmd_buffer.begin_render_pass(
            &vk::RenderPassBeginInfo::default()
                .render_pass(&self.render_pass)
                .framebuffer(&self.swapchain_objects.framebuffers[image_idx])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent,
                })
                .clear_values(&[clear_value]),
            vk::SubpassContents::Inline,
        );

        cmd_buffer.bind_pipeline(vk::PipelineBindPoint::Graphics, &self.pipeline);
        cmd_buffer.set_viewport(
            0,
            &[vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: extent.width as f32,
                height: extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }],
        );
        cmd_buffer.set_scissor(
            0,
            &[vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent,
            }],
        );

        cmd_buffer.draw(3, 1, 0, 0);
        cmd_buffer.end_render_pass();

        cmd_buffer.end()?;
        Ok(())
    }

    fn render_frame(&mut self) -> Result<()> {
        let curr_index = self.next_index;
        self.next_index = (self.next_index + 1) % self.swapchain_objects.swapchain_images.len();

        let fence = &self.fences[curr_index];
        let image_available = &self.image_available[curr_index];

        self.device
            .wait_for_fences(slice::from_ref(fence), vk::TRUE, u64::MAX)?;

        self.device.reset_fences(slice::from_ref(fence))?;
        let (status, image_idx) = self.device.acquire_next_image_khr(
            &self.swapchain_objects.swapchain,
            u64::MAX,
            Some(image_available),
            None,
        )?;
        let image_idx = image_idx as usize;
        assert!(status == vk::Status::Success || status == vk::Status::SuboptimalKHR);

        let render_finished = &self.render_finished[curr_index];

        let cmd_buffer = &self.cmd_buffers[image_idx];
        self.record_command_buffer(cmd_buffer, image_idx)?;

        self.queue.submit(
            &[vk::SubmitInfo::default()
                .wait_semaphore(
                    slice::from_ref(image_available),
                    Some(&[vk::PipelineStageFlags::ColorAttachmentOutput]),
                )
                .command_buffers(slice::from_ref(cmd_buffer))
                .signal_semaphores(slice::from_ref(render_finished))],
            Some(fence),
        )?;

        self.queue.present_khr(
            &vk::PresentInfoKHR::default()
                .wait_semaphores(slice::from_ref(render_finished))
                .swapchain(
                    slice::from_ref(&self.swapchain_objects.swapchain),
                    &[image_idx as u32],
                    None,
                ),
        )?;
        Ok(())
    }
}

impl Drop for VulkanApplication {
    fn drop(&mut self) {
        self.device.wait_idle().unwrap();

        self.swapchain_objects.destroy(&self.device);

        unsafe {
            self.device.destroy_pipeline(Some(&self.pipeline));
            self.device
                .destroy_pipeline_layout(Some(&self.pipeline_layout));
            for module in &self.shader_modules {
                self.device.destroy_shader_module(Some(module));
            }
            self.device.destroy_render_pass(Some(&self.render_pass));
            self.device.destroy_command_pool(Some(&self.command_pool));

            for fence in &self.fences {
                self.device.destroy_fence(Some(fence));
            }
            for semaphore in self
                .image_available
                .iter()
                .chain(self.render_finished.iter())
            {
                self.device.destroy_semaphore(Some(&semaphore));
            }

            self.device.destroy();

            self.instance
                .destroy_debug_utils_messenger_ext(self.debug_messenger.as_ref());
            self.instance.destroy_surface_khr(Some(&self.surface));

            self.instance.destroy();
            DynamicDispatcher::unload();
        }
    }
}

pub struct SwapchainObjects {
    swapchain: vk::rs::SwapchainKHR,
    swapchain_images: SmallVec<[vk::rs::Image; 3]>,
    swapchain_views: SmallVec<[vk::rs::ImageView; 3]>,
    framebuffers: SmallVec<[vk::rs::Framebuffer; 3]>,
    format: vk::Format,
    extent: vk::Extent2D,
}

impl SwapchainObjects {
    pub fn create(
        physical_device: &vk::rs::PhysicalDevice,
        device: &vk::rs::Device,
        surface: &vk::rs::SurfaceKHR,
        window_size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let capabilities = physical_device.get_surface_capabilities_khr(surface)?;

        let format = physical_device
            .get_surface_formats_khr::<Vec<_>>(Some(surface))?
            .into_iter()
            .max_by_key(|fmt| match fmt {
                // we have one pair of format/color_space that we prefer
                vk::SurfaceFormatKHR {
                    format: vk::Format::B8G8R8A8Srgb,
                    color_space: vk::ColorSpaceKHR::SrgbNonlinear,
                } => 1,
                _ => 0,
            })
            .ok_or_else(|| anyhow!("No swapchain format is available"))?;

        let present_mode = physical_device
            .get_surface_present_modes_khr::<Vec<_>>(Some(surface))?
            .into_iter()
            .max_by_key(|mode| match mode {
                vk::PresentModeKHR::Mailbox => 3,
                vk::PresentModeKHR::FifoRelaxed => 2,
                vk::PresentModeKHR::Fifo => 1,
                vk::PresentModeKHR::Immediate => 0,
                _ => -1,
            })
            .ok_or_else(|| anyhow!("No swapchain present mode is available"))?;

        let extent = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let min_ex = capabilities.min_image_extent;
            let max_ex = capabilities.max_image_extent;
            vk::Extent2D {
                width: window_size.width.clamp(min_ex.width, max_ex.width),
                height: window_size.height.clamp(min_ex.height, max_ex.height),
            }
        };

        let max_swap_count = if capabilities.max_image_count != 0 {
            capabilities.max_image_count
        } else {
            u32::MAX
        };
        let swapchain_count = (capabilities.min_image_count + 1).min(max_swap_count);

        let swapchain_info = vk::SwapchainCreateInfoKHR::default()
            .surface(&surface)
            .min_image_count(swapchain_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent.clone())
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::ColorAttachment)
            .image_sharing_mode(vk::SharingMode::Exclusive)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::Opaque)
            .present_mode(present_mode)
            .clipped(vk::TRUE);

        let swapchain = device.create_swapchain_khr(&swapchain_info)?;
        let swapchain_images: SmallVec<[_; 3]> = device.get_swapchain_images_khr(&swapchain)?;
        let swapchain_views: SmallVec<[_; 3]> = swapchain_images
            .iter()
            .map(|img| {
                device.create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .image(img)
                        .view_type(vk::ImageViewType::Type2D)
                        .format(format.format)
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::Color,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        }),
                )
            })
            .collect::<vk::Result<_>>()?;

        Ok(SwapchainObjects {
            swapchain,
            swapchain_images,
            swapchain_views,
            framebuffers: smallvec![],
            format: format.format,
            extent,
        })
    }

    fn create_framebuffers(
        &mut self,
        device: &vk::rs::Device,
        render_pass: &vk::rs::RenderPass,
    ) -> Result<()> {
        self.framebuffers = self
            .swapchain_views
            .iter()
            .map(|view| {
                device.create_framebuffer(
                    &vk::FramebufferCreateInfo::default()
                        .render_pass(render_pass)
                        .attachments(slice::from_ref(view))
                        .width(self.extent.width)
                        .height(self.extent.height)
                        .layers(1),
                )
            })
            .collect::<vk::Result<_>>()?;
        Ok(())
    }

    fn destroy(&mut self, device: &vk::rs::Device) {
        unsafe {
            for framebuffer in &self.framebuffers {
                device.destroy_framebuffer(Some(framebuffer));
            }
            for view in &self.swapchain_views {
                device.destroy_image_view(Some(view));
            }
            device.destroy_swapchain_khr(Some(&self.swapchain));
        }
    }
}
