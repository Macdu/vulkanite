use crate::vk::raw::*;
use crate::vk::*;
use crate::*;
use std::cell::Cell;
use std::ffi::{c_char, c_int, c_void};
use std::mem;
#[derive(Default, Clone)]
pub struct CommandsDispatcher {
    pub create_instance: Cell<
        Option<
            unsafe extern "system" fn(
                *const InstanceCreateInfo,
                *const AllocationCallbacks,
                *const Instance,
            ) -> Status,
        >,
    >,
    pub destroy_instance:
        Cell<Option<unsafe extern "system" fn(Option<Instance>, *const AllocationCallbacks)>>,
    pub enumerate_physical_devices: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const u32,
                *const PhysicalDevice,
            ) -> Status,
        >,
    >,
    pub get_physical_device_features: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, *const PhysicalDeviceFeatures)>,
    >,
    pub get_physical_device_format_properties: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, Format, *const FormatProperties)>,
    >,
    pub get_physical_device_image_format_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Format,
                ImageType,
                ImageTiling,
                ImageUsageFlags,
                ImageCreateFlags,
                *const ImageFormatProperties,
            ) -> Status,
        >,
    >,
    pub get_physical_device_properties: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, *const PhysicalDeviceProperties)>,
    >,
    pub get_physical_device_queue_family_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const QueueFamilyProperties,
            ),
        >,
    >,
    pub get_physical_device_memory_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceMemoryProperties,
            ),
        >,
    >,
    pub get_instance_proc_addr:
        Cell<Option<unsafe extern "system" fn(Option<Instance>, *const c_char) -> FuncPtr>>,
    pub get_device_proc_addr:
        Cell<Option<unsafe extern "system" fn(Option<Device>, *const c_char) -> FuncPtr>>,
    pub create_device: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const DeviceCreateInfo,
                *const AllocationCallbacks,
                *const Device,
            ) -> Status,
        >,
    >,
    pub destroy_device:
        Cell<Option<unsafe extern "system" fn(Option<Device>, *const AllocationCallbacks)>>,
    pub enumerate_instance_extension_properties: Cell<
        Option<
            unsafe extern "system" fn(
                *const c_char,
                *const u32,
                *const ExtensionProperties,
            ) -> Status,
        >,
    >,
    pub enumerate_device_extension_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const c_char,
                *const u32,
                *const ExtensionProperties,
            ) -> Status,
        >,
    >,
    pub enumerate_instance_layer_properties:
        Cell<Option<unsafe extern "system" fn(*const u32, *const LayerProperties) -> Status>>,
    pub enumerate_device_layer_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const LayerProperties,
            ) -> Status,
        >,
    >,
    pub get_device_queue:
        Cell<Option<unsafe extern "system" fn(Option<Device>, u32, u32, *const Queue)>>,
    pub queue_submit: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Queue>,
                u32,
                *const SubmitInfo,
                Option<Fence>,
            ) -> Status,
        >,
    >,
    pub queue_wait_idle: Cell<Option<unsafe extern "system" fn(Option<Queue>) -> Status>>,
    pub device_wait_idle: Cell<Option<unsafe extern "system" fn(Option<Device>) -> Status>>,
    pub allocate_memory: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryAllocateInfo,
                *const AllocationCallbacks,
                *const DeviceMemory,
            ) -> Status,
        >,
    >,
    pub free_memory: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeviceMemory>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub map_memory: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeviceMemory>,
                DeviceSize,
                DeviceSize,
                MemoryMapFlags,
                *const *const c_void,
            ) -> Status,
        >,
    >,
    pub unmap_memory: Cell<Option<unsafe extern "system" fn(Option<Device>, Option<DeviceMemory>)>>,
    pub flush_mapped_memory_ranges: Cell<
        Option<unsafe extern "system" fn(Option<Device>, u32, *const MappedMemoryRange) -> Status>,
    >,
    pub invalidate_mapped_memory_ranges: Cell<
        Option<unsafe extern "system" fn(Option<Device>, u32, *const MappedMemoryRange) -> Status>,
    >,
    pub get_device_memory_commitment: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<DeviceMemory>, *const DeviceSize)>,
    >,
    pub bind_buffer_memory: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Buffer>,
                Option<DeviceMemory>,
                DeviceSize,
            ) -> Status,
        >,
    >,
    pub bind_image_memory: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Image>,
                Option<DeviceMemory>,
                DeviceSize,
            ) -> Status,
        >,
    >,
    pub get_buffer_memory_requirements: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Buffer>, *const MemoryRequirements),
        >,
    >,
    pub get_image_memory_requirements: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<Image>, *const MemoryRequirements)>,
    >,
    pub get_image_sparse_memory_requirements: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Image>,
                *const u32,
                *const SparseImageMemoryRequirements,
            ),
        >,
    >,
    pub get_physical_device_sparse_image_format_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Format,
                ImageType,
                SampleCountFlags,
                ImageUsageFlags,
                ImageTiling,
                *const u32,
                *const SparseImageFormatProperties,
            ),
        >,
    >,
    pub queue_bind_sparse: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Queue>,
                u32,
                *const BindSparseInfo,
                Option<Fence>,
            ) -> Status,
        >,
    >,
    pub create_fence: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const FenceCreateInfo,
                *const AllocationCallbacks,
                *const Fence,
            ) -> Status,
        >,
    >,
    pub destroy_fence: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Fence>, *const AllocationCallbacks),
        >,
    >,
    pub reset_fences:
        Cell<Option<unsafe extern "system" fn(Option<Device>, u32, *const Fence) -> Status>>,
    pub get_fence_status:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<Fence>) -> Status>>,
    pub wait_for_fences: Cell<
        Option<unsafe extern "system" fn(Option<Device>, u32, *const Fence, Bool32, u64) -> Status>,
    >,
    pub create_semaphore: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SemaphoreCreateInfo,
                *const AllocationCallbacks,
                *const Semaphore,
            ) -> Status,
        >,
    >,
    pub destroy_semaphore: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Semaphore>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_event: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const EventCreateInfo,
                *const AllocationCallbacks,
                *const Event,
            ) -> Status,
        >,
    >,
    pub destroy_event: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Event>, *const AllocationCallbacks),
        >,
    >,
    pub get_event_status:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<Event>) -> Status>>,
    pub set_event: Cell<Option<unsafe extern "system" fn(Option<Device>, Option<Event>) -> Status>>,
    pub reset_event:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<Event>) -> Status>>,
    pub create_query_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const QueryPoolCreateInfo,
                *const AllocationCallbacks,
                *const QueryPool,
            ) -> Status,
        >,
    >,
    pub destroy_query_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<QueryPool>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_query_pool_results: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<QueryPool>,
                u32,
                u32,
                usize,
                VoidPtr,
                DeviceSize,
                QueryResultFlags,
            ) -> Status,
        >,
    >,
    pub create_buffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferCreateInfo,
                *const AllocationCallbacks,
                *const Buffer,
            ) -> Status,
        >,
    >,
    pub destroy_buffer: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Buffer>, *const AllocationCallbacks),
        >,
    >,
    pub create_buffer_view: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferViewCreateInfo,
                *const AllocationCallbacks,
                *const BufferView,
            ) -> Status,
        >,
    >,
    pub destroy_buffer_view: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<BufferView>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageCreateInfo,
                *const AllocationCallbacks,
                *const Image,
            ) -> Status,
        >,
    >,
    pub destroy_image: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Image>, *const AllocationCallbacks),
        >,
    >,
    pub get_image_subresource_layout: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Image>,
                *const ImageSubresource,
                *const SubresourceLayout,
            ),
        >,
    >,
    pub create_image_view: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageViewCreateInfo,
                *const AllocationCallbacks,
                *const ImageView,
            ) -> Status,
        >,
    >,
    pub destroy_image_view: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ImageView>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_shader_module: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ShaderModuleCreateInfo,
                *const AllocationCallbacks,
                *const ShaderModule,
            ) -> Status,
        >,
    >,
    pub destroy_shader_module: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ShaderModule>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_pipeline_cache: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineCacheCreateInfo,
                *const AllocationCallbacks,
                *const PipelineCache,
            ) -> Status,
        >,
    >,
    pub destroy_pipeline_cache: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_pipeline_cache_data: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                *const usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub merge_pipeline_caches: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                u32,
                *const PipelineCache,
            ) -> Status,
        >,
    >,
    pub create_graphics_pipelines: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                u32,
                *const GraphicsPipelineCreateInfo,
                *const AllocationCallbacks,
                *const Pipeline,
            ) -> Status,
        >,
    >,
    pub create_compute_pipelines: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                u32,
                *const ComputePipelineCreateInfo,
                *const AllocationCallbacks,
                *const Pipeline,
            ) -> Status,
        >,
    >,
    pub destroy_pipeline: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Pipeline>, *const AllocationCallbacks),
        >,
    >,
    pub create_pipeline_layout: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineLayoutCreateInfo,
                *const AllocationCallbacks,
                *const PipelineLayout,
            ) -> Status,
        >,
    >,
    pub destroy_pipeline_layout: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineLayout>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_sampler: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SamplerCreateInfo,
                *const AllocationCallbacks,
                *const Sampler,
            ) -> Status,
        >,
    >,
    pub destroy_sampler: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<Sampler>, *const AllocationCallbacks),
        >,
    >,
    pub create_descriptor_set_layout: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorSetLayoutCreateInfo,
                *const AllocationCallbacks,
                *const DescriptorSetLayout,
            ) -> Status,
        >,
    >,
    pub destroy_descriptor_set_layout: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorSetLayout>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_descriptor_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorPoolCreateInfo,
                *const AllocationCallbacks,
                *const DescriptorPool,
            ) -> Status,
        >,
    >,
    pub destroy_descriptor_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorPool>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub reset_descriptor_pool: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<DescriptorPool>, u32) -> Status>,
    >,
    pub allocate_descriptor_sets: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorSetAllocateInfo,
                *const DescriptorSet,
            ) -> Status,
        >,
    >,
    pub free_descriptor_sets: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorPool>,
                u32,
                *const DescriptorSet,
            ) -> Status,
        >,
    >,
    pub update_descriptor_sets: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const WriteDescriptorSet,
                u32,
                *const CopyDescriptorSet,
            ),
        >,
    >,
    pub create_framebuffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const FramebufferCreateInfo,
                *const AllocationCallbacks,
                *const Framebuffer,
            ) -> Status,
        >,
    >,
    pub destroy_framebuffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Framebuffer>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_render_pass: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const RenderPassCreateInfo,
                *const AllocationCallbacks,
                *const RenderPass,
            ) -> Status,
        >,
    >,
    pub destroy_render_pass: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<RenderPass>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_render_area_granularity: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<RenderPass>, *const Extent2D)>,
    >,
    pub create_command_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const CommandPoolCreateInfo,
                *const AllocationCallbacks,
                *const CommandPool,
            ) -> Status,
        >,
    >,
    pub destroy_command_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CommandPool>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub reset_command_pool: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CommandPool>,
                CommandPoolResetFlags,
            ) -> Status,
        >,
    >,
    pub allocate_command_buffers: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const CommandBufferAllocateInfo,
                *const CommandBuffer,
            ) -> Status,
        >,
    >,
    pub free_command_buffers: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CommandPool>,
                u32,
                *const CommandBuffer,
            ),
        >,
    >,
    pub begin_command_buffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const CommandBufferBeginInfo,
            ) -> Status,
        >,
    >,
    pub end_command_buffer:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>) -> Status>>,
    pub reset_command_buffer: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, CommandBufferResetFlags) -> Status>,
    >,
    pub cmd_bind_pipeline: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, PipelineBindPoint, Option<Pipeline>),
        >,
    >,
    pub cmd_set_viewport:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const Viewport)>>,
    pub cmd_set_scissor:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const Rect2D)>>,
    pub cmd_set_line_width: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, f32)>>,
    pub cmd_set_depth_bias:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, f32, f32, f32)>>,
    pub cmd_set_blend_constants:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, [f32; 4u16 as _])>>,
    pub cmd_set_depth_bounds:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, f32, f32)>>,
    pub cmd_set_stencil_compare_mask:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, StencilFaceFlags, u32)>>,
    pub cmd_set_stencil_write_mask:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, StencilFaceFlags, u32)>>,
    pub cmd_set_stencil_reference:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, StencilFaceFlags, u32)>>,
    pub cmd_bind_descriptor_sets: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineBindPoint,
                Option<PipelineLayout>,
                u32,
                u32,
                *const DescriptorSet,
                u32,
                *const u32,
            ),
        >,
    >,
    pub cmd_bind_index_buffer: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize, IndexType),
        >,
    >,
    pub cmd_bind_vertex_buffers: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const Buffer,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_draw:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32, u32)>>,
    pub cmd_draw_indexed:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32, i32, u32)>>,
    pub cmd_draw_indirect: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize, u32, u32),
        >,
    >,
    pub cmd_draw_indexed_indirect: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize, u32, u32),
        >,
    >,
    pub cmd_dispatch: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32)>>,
    pub cmd_dispatch_indirect:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize)>>,
    pub cmd_copy_buffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                Option<Buffer>,
                u32,
                *const BufferCopy,
            ),
        >,
    >,
    pub cmd_copy_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Image>,
                ImageLayout,
                Option<Image>,
                ImageLayout,
                u32,
                *const ImageCopy,
            ),
        >,
    >,
    pub cmd_blit_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Image>,
                ImageLayout,
                Option<Image>,
                ImageLayout,
                u32,
                *const ImageBlit,
                Filter,
            ),
        >,
    >,
    pub cmd_copy_buffer_to_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                Option<Image>,
                ImageLayout,
                u32,
                *const BufferImageCopy,
            ),
        >,
    >,
    pub cmd_copy_image_to_buffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Image>,
                ImageLayout,
                Option<Buffer>,
                u32,
                *const BufferImageCopy,
            ),
        >,
    >,
    pub cmd_update_buffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                VoidPtr,
            ),
        >,
    >,
    pub cmd_fill_buffer: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                u32,
            ),
        >,
    >,
    pub cmd_clear_color_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Image>,
                ImageLayout,
                *const ClearColorValue,
                u32,
                *const ImageSubresourceRange,
            ),
        >,
    >,
    pub cmd_clear_depth_stencil_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Image>,
                ImageLayout,
                *const ClearDepthStencilValue,
                u32,
                *const ImageSubresourceRange,
            ),
        >,
    >,
    pub cmd_clear_attachments: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const ClearAttachment,
                u32,
                *const ClearRect,
            ),
        >,
    >,
    pub cmd_resolve_image: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Image>,
                ImageLayout,
                Option<Image>,
                ImageLayout,
                u32,
                *const ImageResolve,
            ),
        >,
    >,
    pub cmd_set_event: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<Event>, PipelineStageFlags)>,
    >,
    pub cmd_reset_event: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<Event>, PipelineStageFlags)>,
    >,
    pub cmd_wait_events: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const Event,
                PipelineStageFlags,
                PipelineStageFlags,
                u32,
                *const MemoryBarrier,
                u32,
                *const BufferMemoryBarrier,
                u32,
                *const ImageMemoryBarrier,
            ),
        >,
    >,
    pub cmd_pipeline_barrier: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineStageFlags,
                PipelineStageFlags,
                DependencyFlags,
                u32,
                *const MemoryBarrier,
                u32,
                *const BufferMemoryBarrier,
                u32,
                *const ImageMemoryBarrier,
            ),
        >,
    >,
    pub cmd_begin_query: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<QueryPool>,
                u32,
                QueryControlFlags,
            ),
        >,
    >,
    pub cmd_end_query:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<QueryPool>, u32)>>,
    pub cmd_reset_query_pool:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<QueryPool>, u32, u32)>>,
    pub cmd_write_timestamp: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineStageFlags,
                Option<QueryPool>,
                u32,
            ),
        >,
    >,
    pub cmd_copy_query_pool_results: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<QueryPool>,
                u32,
                u32,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                QueryResultFlags,
            ),
        >,
    >,
    pub cmd_push_constants: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<PipelineLayout>,
                ShaderStageFlags,
                u32,
                u32,
                VoidPtr,
            ),
        >,
    >,
    pub cmd_begin_render_pass: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const RenderPassBeginInfo,
                SubpassContents,
            ),
        >,
    >,
    pub cmd_next_subpass:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, SubpassContents)>>,
    pub cmd_end_render_pass: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_execute_commands:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const CommandBuffer)>>,
    pub enumerate_instance_version: Cell<Option<unsafe extern "system" fn(*const u32) -> Status>>,
    pub bind_buffer_memory2: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, u32, *const BindBufferMemoryInfo) -> Status,
        >,
    >,
    pub bind_buffer_memory2_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, u32, *const BindBufferMemoryInfo) -> Status,
        >,
    >,
    pub bind_image_memory2: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, u32, *const BindImageMemoryInfo) -> Status,
        >,
    >,
    pub bind_image_memory2_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, u32, *const BindImageMemoryInfo) -> Status,
        >,
    >,
    pub get_device_group_peer_memory_features: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, u32, u32, u32, *const PeerMemoryFeatureFlags),
        >,
    >,
    pub get_device_group_peer_memory_features_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, u32, u32, u32, *const PeerMemoryFeatureFlags),
        >,
    >,
    pub cmd_set_device_mask: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32)>>,
    pub cmd_set_device_mask_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32)>>,
    pub cmd_dispatch_base: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32, u32, u32, u32)>,
    >,
    pub cmd_dispatch_base_khr: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32, u32, u32, u32)>,
    >,
    pub enumerate_physical_device_groups: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const u32,
                *const PhysicalDeviceGroupProperties,
            ) -> Status,
        >,
    >,
    pub enumerate_physical_device_groups_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const u32,
                *const PhysicalDeviceGroupProperties,
            ) -> Status,
        >,
    >,
    pub get_image_memory_requirements2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageMemoryRequirementsInfo2,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_image_memory_requirements2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageMemoryRequirementsInfo2,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_buffer_memory_requirements2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferMemoryRequirementsInfo2,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_buffer_memory_requirements2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferMemoryRequirementsInfo2,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_image_sparse_memory_requirements2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageSparseMemoryRequirementsInfo2,
                *const u32,
                *const SparseImageMemoryRequirements2,
            ),
        >,
    >,
    pub get_image_sparse_memory_requirements2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageSparseMemoryRequirementsInfo2,
                *const u32,
                *const SparseImageMemoryRequirements2,
            ),
        >,
    >,
    pub get_physical_device_features2: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, *const PhysicalDeviceFeatures2)>,
    >,
    pub get_physical_device_features2_khr: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, *const PhysicalDeviceFeatures2)>,
    >,
    pub get_physical_device_properties2: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, *const PhysicalDeviceProperties2)>,
    >,
    pub get_physical_device_properties2_khr: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, *const PhysicalDeviceProperties2)>,
    >,
    pub get_physical_device_format_properties2: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, Format, *const FormatProperties2)>,
    >,
    pub get_physical_device_format_properties2_khr: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, Format, *const FormatProperties2)>,
    >,
    pub get_physical_device_image_format_properties2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceImageFormatInfo2,
                *const ImageFormatProperties2,
            ) -> Status,
        >,
    >,
    pub get_physical_device_image_format_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceImageFormatInfo2,
                *const ImageFormatProperties2,
            ) -> Status,
        >,
    >,
    pub get_physical_device_queue_family_properties2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const QueueFamilyProperties2,
            ),
        >,
    >,
    pub get_physical_device_queue_family_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const QueueFamilyProperties2,
            ),
        >,
    >,
    pub get_physical_device_memory_properties2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceMemoryProperties2,
            ),
        >,
    >,
    pub get_physical_device_memory_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceMemoryProperties2,
            ),
        >,
    >,
    pub get_physical_device_sparse_image_format_properties2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceSparseImageFormatInfo2,
                *const u32,
                *const SparseImageFormatProperties2,
            ),
        >,
    >,
    pub get_physical_device_sparse_image_format_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceSparseImageFormatInfo2,
                *const u32,
                *const SparseImageFormatProperties2,
            ),
        >,
    >,
    pub trim_command_pool:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<CommandPool>, u32)>>,
    pub trim_command_pool_khr:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<CommandPool>, u32)>>,
    pub get_device_queue2: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const DeviceQueueInfo2, *const Queue)>,
    >,
    pub create_sampler_ycbcr_conversion: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SamplerYcbcrConversionCreateInfo,
                *const AllocationCallbacks,
                *const SamplerYcbcrConversion,
            ) -> Status,
        >,
    >,
    pub create_sampler_ycbcr_conversion_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SamplerYcbcrConversionCreateInfo,
                *const AllocationCallbacks,
                *const SamplerYcbcrConversion,
            ) -> Status,
        >,
    >,
    pub destroy_sampler_ycbcr_conversion: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SamplerYcbcrConversion>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub destroy_sampler_ycbcr_conversion_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SamplerYcbcrConversion>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub create_descriptor_update_template: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorUpdateTemplateCreateInfo,
                *const AllocationCallbacks,
                *const DescriptorUpdateTemplate,
            ) -> Status,
        >,
    >,
    pub create_descriptor_update_template_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorUpdateTemplateCreateInfo,
                *const AllocationCallbacks,
                *const DescriptorUpdateTemplate,
            ) -> Status,
        >,
    >,
    pub destroy_descriptor_update_template: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorUpdateTemplate>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub destroy_descriptor_update_template_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorUpdateTemplate>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub update_descriptor_set_with_template: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorSet>,
                Option<DescriptorUpdateTemplate>,
                VoidPtr,
            ),
        >,
    >,
    pub update_descriptor_set_with_template_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorSet>,
                Option<DescriptorUpdateTemplate>,
                VoidPtr,
            ),
        >,
    >,
    pub get_physical_device_external_buffer_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceExternalBufferInfo,
                *const ExternalBufferProperties,
            ),
        >,
    >,
    pub get_physical_device_external_buffer_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceExternalBufferInfo,
                *const ExternalBufferProperties,
            ),
        >,
    >,
    pub get_physical_device_external_fence_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceExternalFenceInfo,
                *const ExternalFenceProperties,
            ),
        >,
    >,
    pub get_physical_device_external_fence_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceExternalFenceInfo,
                *const ExternalFenceProperties,
            ),
        >,
    >,
    pub get_physical_device_external_semaphore_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceExternalSemaphoreInfo,
                *const ExternalSemaphoreProperties,
            ),
        >,
    >,
    pub get_physical_device_external_semaphore_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceExternalSemaphoreInfo,
                *const ExternalSemaphoreProperties,
            ),
        >,
    >,
    pub get_descriptor_set_layout_support: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorSetLayoutCreateInfo,
                *const DescriptorSetLayoutSupport,
            ),
        >,
    >,
    pub get_descriptor_set_layout_support_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorSetLayoutCreateInfo,
                *const DescriptorSetLayoutSupport,
            ),
        >,
    >,
    pub cmd_draw_indirect_count: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_draw_indirect_count_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_draw_indirect_count_amd: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_draw_indexed_indirect_count: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_draw_indexed_indirect_count_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_draw_indexed_indirect_count_amd: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub create_render_pass2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const RenderPassCreateInfo2,
                *const AllocationCallbacks,
                *const RenderPass,
            ) -> Status,
        >,
    >,
    pub create_render_pass2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const RenderPassCreateInfo2,
                *const AllocationCallbacks,
                *const RenderPass,
            ) -> Status,
        >,
    >,
    pub cmd_begin_render_pass2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const RenderPassBeginInfo,
                *const SubpassBeginInfo,
            ),
        >,
    >,
    pub cmd_begin_render_pass2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const RenderPassBeginInfo,
                *const SubpassBeginInfo,
            ),
        >,
    >,
    pub cmd_next_subpass2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const SubpassBeginInfo,
                *const SubpassEndInfo,
            ),
        >,
    >,
    pub cmd_next_subpass2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const SubpassBeginInfo,
                *const SubpassEndInfo,
            ),
        >,
    >,
    pub cmd_end_render_pass2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const SubpassEndInfo)>>,
    pub cmd_end_render_pass2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const SubpassEndInfo)>>,
    pub reset_query_pool:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<QueryPool>, u32, u32)>>,
    pub reset_query_pool_ext:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<QueryPool>, u32, u32)>>,
    pub get_semaphore_counter_value: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<Semaphore>, *const u64) -> Status>,
    >,
    pub get_semaphore_counter_value_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<Semaphore>, *const u64) -> Status>,
    >,
    pub wait_semaphores: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const SemaphoreWaitInfo, u64) -> Status>,
    >,
    pub wait_semaphores_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const SemaphoreWaitInfo, u64) -> Status>,
    >,
    pub signal_semaphore: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const SemaphoreSignalInfo) -> Status>,
    >,
    pub signal_semaphore_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const SemaphoreSignalInfo) -> Status>,
    >,
    pub get_buffer_device_address: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferDeviceAddressInfo,
            ) -> DeviceAddress,
        >,
    >,
    pub get_buffer_device_address_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferDeviceAddressInfo,
            ) -> DeviceAddress,
        >,
    >,
    pub get_buffer_device_address_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferDeviceAddressInfo,
            ) -> DeviceAddress,
        >,
    >,
    pub get_buffer_opaque_capture_address: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const BufferDeviceAddressInfo) -> u64>,
    >,
    pub get_buffer_opaque_capture_address_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const BufferDeviceAddressInfo) -> u64>,
    >,
    pub get_device_memory_opaque_capture_address: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceMemoryOpaqueCaptureAddressInfo,
            ) -> u64,
        >,
    >,
    pub get_device_memory_opaque_capture_address_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceMemoryOpaqueCaptureAddressInfo,
            ) -> u64,
        >,
    >,
    pub get_physical_device_tool_properties: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const PhysicalDeviceToolProperties,
            ) -> Status,
        >,
    >,
    pub get_physical_device_tool_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const PhysicalDeviceToolProperties,
            ) -> Status,
        >,
    >,
    pub create_private_data_slot: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PrivateDataSlotCreateInfo,
                *const AllocationCallbacks,
                *const PrivateDataSlot,
            ) -> Status,
        >,
    >,
    pub create_private_data_slot_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PrivateDataSlotCreateInfo,
                *const AllocationCallbacks,
                *const PrivateDataSlot,
            ) -> Status,
        >,
    >,
    pub destroy_private_data_slot: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PrivateDataSlot>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub destroy_private_data_slot_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PrivateDataSlot>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub set_private_data: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ObjectType,
                u64,
                Option<PrivateDataSlot>,
                u64,
            ) -> Status,
        >,
    >,
    pub set_private_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ObjectType,
                u64,
                Option<PrivateDataSlot>,
                u64,
            ) -> Status,
        >,
    >,
    pub get_private_data: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ObjectType,
                u64,
                Option<PrivateDataSlot>,
                *const u64,
            ),
        >,
    >,
    pub get_private_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ObjectType,
                u64,
                Option<PrivateDataSlot>,
                *const u64,
            ),
        >,
    >,
    pub cmd_set_event2: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Event>, *const DependencyInfo),
        >,
    >,
    pub cmd_set_event2_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Event>, *const DependencyInfo),
        >,
    >,
    pub cmd_reset_event2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<Event>, u32)>>,
    pub cmd_reset_event2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<Event>, u32)>>,
    pub cmd_wait_events2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const Event,
                *const DependencyInfo,
            ),
        >,
    >,
    pub cmd_wait_events2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const Event,
                *const DependencyInfo,
            ),
        >,
    >,
    pub cmd_pipeline_barrier2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DependencyInfo)>>,
    pub cmd_pipeline_barrier2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DependencyInfo)>>,
    pub cmd_write_timestamp2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, Option<QueryPool>, u32)>>,
    pub cmd_write_timestamp2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, Option<QueryPool>, u32)>>,
    pub queue_submit2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Queue>,
                u32,
                *const SubmitInfo2,
                Option<Fence>,
            ) -> Status,
        >,
    >,
    pub queue_submit2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Queue>,
                u32,
                *const SubmitInfo2,
                Option<Fence>,
            ) -> Status,
        >,
    >,
    pub cmd_copy_buffer2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyBufferInfo2)>>,
    pub cmd_copy_buffer2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyBufferInfo2)>>,
    pub cmd_copy_image2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyImageInfo2)>>,
    pub cmd_copy_image2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyImageInfo2)>>,
    pub cmd_copy_buffer_to_image2: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyBufferToImageInfo2)>,
    >,
    pub cmd_copy_buffer_to_image2_khr: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyBufferToImageInfo2)>,
    >,
    pub cmd_copy_image_to_buffer2: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyImageToBufferInfo2)>,
    >,
    pub cmd_copy_image_to_buffer2_khr: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyImageToBufferInfo2)>,
    >,
    pub cmd_blit_image2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const BlitImageInfo2)>>,
    pub cmd_blit_image2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const BlitImageInfo2)>>,
    pub cmd_resolve_image2:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const ResolveImageInfo2)>>,
    pub cmd_resolve_image2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const ResolveImageInfo2)>>,
    pub cmd_begin_rendering:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const RenderingInfo)>>,
    pub cmd_begin_rendering_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const RenderingInfo)>>,
    pub cmd_end_rendering: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_end_rendering_khr: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_set_cull_mode:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, CullModeFlags)>>,
    pub cmd_set_cull_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, CullModeFlags)>>,
    pub cmd_set_front_face:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, FrontFace)>>,
    pub cmd_set_front_face_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, FrontFace)>>,
    pub cmd_set_primitive_topology:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, PrimitiveTopology)>>,
    pub cmd_set_primitive_topology_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, PrimitiveTopology)>>,
    pub cmd_set_viewport_with_count:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const Viewport)>>,
    pub cmd_set_viewport_with_count_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const Viewport)>>,
    pub cmd_set_scissor_with_count:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const Rect2D)>>,
    pub cmd_set_scissor_with_count_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const Rect2D)>>,
    pub cmd_bind_vertex_buffers2: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const Buffer,
                *const DeviceSize,
                *const DeviceSize,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_bind_vertex_buffers2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const Buffer,
                *const DeviceSize,
                *const DeviceSize,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_set_depth_test_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_test_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_write_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_write_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_compare_op:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, CompareOp)>>,
    pub cmd_set_depth_compare_op_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, CompareOp)>>,
    pub cmd_set_depth_bounds_test_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_bounds_test_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_stencil_test_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_stencil_test_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_stencil_op: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                StencilFaceFlags,
                StencilOp,
                StencilOp,
                StencilOp,
                CompareOp,
            ),
        >,
    >,
    pub cmd_set_stencil_op_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                StencilFaceFlags,
                StencilOp,
                StencilOp,
                StencilOp,
                CompareOp,
            ),
        >,
    >,
    pub cmd_set_rasterizer_discard_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_rasterizer_discard_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_bias_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_bias_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_primitive_restart_enable:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_primitive_restart_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub get_device_buffer_memory_requirements: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceBufferMemoryRequirements,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_device_buffer_memory_requirements_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceBufferMemoryRequirements,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_device_image_memory_requirements: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceImageMemoryRequirements,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_device_image_memory_requirements_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceImageMemoryRequirements,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub get_device_image_sparse_memory_requirements: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceImageMemoryRequirements,
                *const u32,
                *const SparseImageMemoryRequirements2,
            ),
        >,
    >,
    pub get_device_image_sparse_memory_requirements_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceImageMemoryRequirements,
                *const u32,
                *const SparseImageMemoryRequirements2,
            ),
        >,
    >,
    pub destroy_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                Option<SurfaceKHR>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_physical_device_surface_support_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                u32,
                Option<SurfaceKHR>,
                *const Bool32,
            ) -> Status,
        >,
    >,
    pub get_physical_device_surface_capabilities_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<SurfaceKHR>,
                *const SurfaceCapabilitiesKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_surface_formats_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<SurfaceKHR>,
                *const u32,
                *const SurfaceFormatKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_surface_present_modes_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<SurfaceKHR>,
                *const u32,
                *const PresentModeKHR,
            ) -> Status,
        >,
    >,
    pub create_swapchain_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SwapchainCreateInfoKHR,
                *const AllocationCallbacks,
                *const SwapchainKHR,
            ) -> Status,
        >,
    >,
    pub destroy_swapchain_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_swapchain_images_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const u32,
                *const Image,
            ) -> Status,
        >,
    >,
    pub acquire_next_image_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                u64,
                Option<Semaphore>,
                Option<Fence>,
                *const u32,
            ) -> Status,
        >,
    >,
    pub queue_present_khr:
        Cell<Option<unsafe extern "system" fn(Option<Queue>, *const PresentInfoKHR) -> Status>>,
    pub get_device_group_present_capabilities_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceGroupPresentCapabilitiesKHR,
            ) -> Status,
        >,
    >,
    pub get_device_group_surface_present_modes_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SurfaceKHR>,
                *const DeviceGroupPresentModeFlagsKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_present_rectangles_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<SurfaceKHR>,
                *const u32,
                *const Rect2D,
            ) -> Status,
        >,
    >,
    pub acquire_next_image2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AcquireNextImageInfoKHR,
                *const u32,
            ) -> Status,
        >,
    >,
    pub get_physical_device_display_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const DisplayPropertiesKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_display_plane_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const DisplayPlanePropertiesKHR,
            ) -> Status,
        >,
    >,
    pub get_display_plane_supported_displays_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                u32,
                *const u32,
                *const DisplayKHR,
            ) -> Status,
        >,
    >,
    pub get_display_mode_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<DisplayKHR>,
                *const u32,
                *const DisplayModePropertiesKHR,
            ) -> Status,
        >,
    >,
    pub create_display_mode_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<DisplayKHR>,
                *const DisplayModeCreateInfoKHR,
                *const AllocationCallbacks,
                *const DisplayModeKHR,
            ) -> Status,
        >,
    >,
    pub get_display_plane_capabilities_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<DisplayModeKHR>,
                u32,
                *const DisplayPlaneCapabilitiesKHR,
            ) -> Status,
        >,
    >,
    pub create_display_plane_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const DisplaySurfaceCreateInfoKHR,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub create_shared_swapchains_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const SwapchainCreateInfoKHR,
                *const AllocationCallbacks,
                *const SwapchainKHR,
            ) -> Status,
        >,
    >,
    pub create_xlib_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const XlibSurfaceCreateInfoKHR,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_xlib_presentation_support_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                u32,
                *const VoidPtr,
                VoidPtr,
            ) -> Bool32,
        >,
    >,
    pub create_xcb_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const XcbSurfaceCreateInfoKHR,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_xcb_presentation_support_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                u32,
                *const VoidPtr,
                VoidPtr,
            ) -> Bool32,
        >,
    >,
    pub create_wayland_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const WaylandSurfaceCreateInfoKHR,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_wayland_presentation_support_khr: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, u32, *const VoidPtr) -> Bool32>,
    >,
    pub create_android_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const AndroidSurfaceCreateInfoKHR,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub create_win32_surface_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const Win32SurfaceCreateInfoKHR,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_win32_presentation_support_khr:
        Cell<Option<unsafe extern "system" fn(Option<PhysicalDevice>, u32) -> Bool32>>,
    pub create_debug_report_callback_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const DebugReportCallbackCreateInfoEXT,
                *const AllocationCallbacks,
                *const DebugReportCallbackEXT,
            ) -> Status,
        >,
    >,
    pub destroy_debug_report_callback_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                Option<DebugReportCallbackEXT>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub debug_report_message_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                DebugReportFlagsEXT,
                DebugReportObjectTypeEXT,
                u64,
                usize,
                i32,
                *const c_char,
                *const c_char,
            ),
        >,
    >,
    pub debug_marker_set_object_tag_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const DebugMarkerObjectTagInfoEXT) -> Status,
        >,
    >,
    pub debug_marker_set_object_name_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DebugMarkerObjectNameInfoEXT,
            ) -> Status,
        >,
    >,
    pub cmd_debug_marker_begin_ext: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DebugMarkerMarkerInfoEXT)>,
    >,
    pub cmd_debug_marker_end_ext: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_debug_marker_insert_ext: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DebugMarkerMarkerInfoEXT)>,
    >,
    pub cmd_bind_transform_feedback_buffers_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const Buffer,
                *const DeviceSize,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_begin_transform_feedback_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const Buffer,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_end_transform_feedback_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const Buffer,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_begin_query_indexed_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<QueryPool>,
                u32,
                QueryControlFlags,
                u32,
            ),
        >,
    >,
    pub cmd_end_query_indexed_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<QueryPool>, u32, u32)>>,
    pub cmd_draw_indirect_byte_count_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub create_cu_module_nvx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const CuModuleCreateInfoNVX,
                *const AllocationCallbacks,
                *const CuModuleNVX,
            ) -> Status,
        >,
    >,
    pub create_cu_function_nvx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const CuFunctionCreateInfoNVX,
                *const AllocationCallbacks,
                *const CuFunctionNVX,
            ) -> Status,
        >,
    >,
    pub destroy_cu_module_nvx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CuModuleNVX>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub destroy_cu_function_nvx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CuFunctionNVX>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub cmd_cu_launch_kernel_nvx:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CuLaunchInfoNVX)>>,
    pub get_image_view_handle_nvx: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const ImageViewHandleInfoNVX) -> u32>,
    >,
    pub get_image_view_address_nvx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ImageView>,
                *const ImageViewAddressPropertiesNVX,
            ) -> Status,
        >,
    >,
    pub get_shader_info_amd: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                ShaderStageFlags,
                ShaderInfoTypeAMD,
                *const usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub create_stream_descriptor_surface_ggp: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const StreamDescriptorSurfaceCreateInfoGGP,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_external_image_format_properties_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Format,
                ImageType,
                ImageTiling,
                ImageUsageFlags,
                ImageCreateFlags,
                ExternalMemoryHandleTypeFlagsNV,
                *const ExternalImageFormatPropertiesNV,
            ) -> Status,
        >,
    >,
    pub get_memory_win32_handle_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeviceMemory>,
                ExternalMemoryHandleTypeFlagsNV,
                *const VoidPtr,
            ) -> Status,
        >,
    >,
    pub create_vi_surface_nn: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const ViSurfaceCreateInfoNN,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_memory_win32_handle_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryGetWin32HandleInfoKHR,
                *const VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_memory_win32_handle_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ExternalMemoryHandleTypeFlags,
                VoidPtr,
                *const MemoryWin32HandlePropertiesKHR,
            ) -> Status,
        >,
    >,
    pub get_memory_fd_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryGetFdInfoKHR,
                *const c_int,
            ) -> Status,
        >,
    >,
    pub get_memory_fd_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ExternalMemoryHandleTypeFlags,
                c_int,
                *const MemoryFdPropertiesKHR,
            ) -> Status,
        >,
    >,
    pub import_semaphore_win32_handle_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImportSemaphoreWin32HandleInfoKHR,
            ) -> Status,
        >,
    >,
    pub get_semaphore_win32_handle_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SemaphoreGetWin32HandleInfoKHR,
                *const VoidPtr,
            ) -> Status,
        >,
    >,
    pub import_semaphore_fd_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const ImportSemaphoreFdInfoKHR) -> Status,
        >,
    >,
    pub get_semaphore_fd_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SemaphoreGetFdInfoKHR,
                *const c_int,
            ) -> Status,
        >,
    >,
    pub cmd_push_descriptor_set_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineBindPoint,
                Option<PipelineLayout>,
                u32,
                u32,
                *const WriteDescriptorSet,
            ),
        >,
    >,
    pub cmd_push_descriptor_set_with_template_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<DescriptorUpdateTemplate>,
                Option<PipelineLayout>,
                u32,
                VoidPtr,
            ),
        >,
    >,
    pub cmd_begin_conditional_rendering_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const ConditionalRenderingBeginInfoEXT,
            ),
        >,
    >,
    pub cmd_end_conditional_rendering_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_set_viewport_wscaling_nv: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const ViewportWScalingNV),
        >,
    >,
    pub release_display_ext: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, Option<DisplayKHR>) -> Status>,
    >,
    pub acquire_xlib_display_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const VoidPtr,
                Option<DisplayKHR>,
            ) -> Status,
        >,
    >,
    pub get_rand_routput_display_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const VoidPtr,
                VoidPtr,
                *const DisplayKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_surface_capabilities2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<SurfaceKHR>,
                *const SurfaceCapabilities2EXT,
            ) -> Status,
        >,
    >,
    pub display_power_control_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DisplayKHR>,
                *const DisplayPowerInfoEXT,
            ) -> Status,
        >,
    >,
    pub register_device_event_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceEventInfoEXT,
                *const AllocationCallbacks,
                *const Fence,
            ) -> Status,
        >,
    >,
    pub register_display_event_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DisplayKHR>,
                *const DisplayEventInfoEXT,
                *const AllocationCallbacks,
                *const Fence,
            ) -> Status,
        >,
    >,
    pub get_swapchain_counter_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                SurfaceCounterFlagsEXT,
                *const u64,
            ) -> Status,
        >,
    >,
    pub get_refresh_cycle_duration_google: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const RefreshCycleDurationGOOGLE,
            ) -> Status,
        >,
    >,
    pub get_past_presentation_timing_google: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const u32,
                *const PastPresentationTimingGOOGLE,
            ) -> Status,
        >,
    >,
    pub cmd_set_discard_rectangle_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const Rect2D)>>,
    pub cmd_set_discard_rectangle_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_discard_rectangle_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, DiscardRectangleModeEXT)>>,
    pub set_hdr_metadata_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const SwapchainKHR,
                *const HdrMetadataEXT,
            ),
        >,
    >,
    pub get_swapchain_status_khr:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<SwapchainKHR>) -> Status>>,
    pub import_fence_win32_handle_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImportFenceWin32HandleInfoKHR,
            ) -> Status,
        >,
    >,
    pub get_fence_win32_handle_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const FenceGetWin32HandleInfoKHR,
                *const VoidPtr,
            ) -> Status,
        >,
    >,
    pub import_fence_fd_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const ImportFenceFdInfoKHR) -> Status>,
    >,
    pub get_fence_fd_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const FenceGetFdInfoKHR,
                *const c_int,
            ) -> Status,
        >,
    >,
    pub enumerate_physical_device_queue_family_performance_query_counters_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                u32,
                *const u32,
                *const PerformanceCounterKHR,
                *const PerformanceCounterDescriptionKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_queue_family_performance_query_passes_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const QueryPoolPerformanceCreateInfoKHR,
                *const u32,
            ),
        >,
    >,
    pub acquire_profiling_lock_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const AcquireProfilingLockInfoKHR) -> Status,
        >,
    >,
    pub release_profiling_lock_khr: Cell<Option<unsafe extern "system" fn(Option<Device>)>>,
    pub get_physical_device_surface_capabilities2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceSurfaceInfo2KHR,
                *const SurfaceCapabilities2KHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_surface_formats2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceSurfaceInfo2KHR,
                *const u32,
                *const SurfaceFormat2KHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_display_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const DisplayProperties2KHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_display_plane_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const DisplayPlaneProperties2KHR,
            ) -> Status,
        >,
    >,
    pub get_display_mode_properties2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                Option<DisplayKHR>,
                *const u32,
                *const DisplayModeProperties2KHR,
            ) -> Status,
        >,
    >,
    pub get_display_plane_capabilities2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const DisplayPlaneInfo2KHR,
                *const DisplayPlaneCapabilities2KHR,
            ) -> Status,
        >,
    >,
    pub create_iossurface_mvk: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const IOSSurfaceCreateInfoMVK,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub create_mac_ossurface_mvk: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const MacOSSurfaceCreateInfoMVK,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub set_debug_utils_object_name_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const DebugUtilsObjectNameInfoEXT) -> Status,
        >,
    >,
    pub set_debug_utils_object_tag_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const DebugUtilsObjectTagInfoEXT) -> Status,
        >,
    >,
    pub queue_begin_debug_utils_label_ext:
        Cell<Option<unsafe extern "system" fn(Option<Queue>, *const DebugUtilsLabelEXT)>>,
    pub queue_end_debug_utils_label_ext: Cell<Option<unsafe extern "system" fn(Option<Queue>)>>,
    pub queue_insert_debug_utils_label_ext:
        Cell<Option<unsafe extern "system" fn(Option<Queue>, *const DebugUtilsLabelEXT)>>,
    pub cmd_begin_debug_utils_label_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DebugUtilsLabelEXT)>>,
    pub cmd_end_debug_utils_label_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_insert_debug_utils_label_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DebugUtilsLabelEXT)>>,
    pub create_debug_utils_messenger_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const DebugUtilsMessengerCreateInfoEXT,
                *const AllocationCallbacks,
                *const DebugUtilsMessengerEXT,
            ) -> Status,
        >,
    >,
    pub destroy_debug_utils_messenger_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                Option<DebugUtilsMessengerEXT>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub submit_debug_utils_message_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                DebugUtilsMessageSeverityFlagsEXT,
                DebugUtilsMessageTypeFlagsEXT,
                *const DebugUtilsMessengerCallbackDataEXT,
            ),
        >,
    >,
    pub get_android_hardware_buffer_properties_android: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AHardwareBuffer,
                *const AndroidHardwareBufferPropertiesANDROID,
            ) -> Status,
        >,
    >,
    pub get_memory_android_hardware_buffer_android: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryGetAndroidHardwareBufferInfoANDROID,
                *const *const AHardwareBuffer,
            ) -> Status,
        >,
    >,
    pub create_execution_graph_pipelines_amdx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                u32,
                *const ExecutionGraphPipelineCreateInfoAMDX,
                *const AllocationCallbacks,
                *const Pipeline,
            ) -> Status,
        >,
    >,
    pub get_execution_graph_pipeline_scratch_size_amdx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                *const ExecutionGraphPipelineScratchSizeAMDX,
            ) -> Status,
        >,
    >,
    pub get_execution_graph_pipeline_node_index_amdx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                *const PipelineShaderStageNodeCreateInfoAMDX,
                *const u32,
            ) -> Status,
        >,
    >,
    pub cmd_initialize_graph_scratch_memory_amdx:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, DeviceAddress)>>,
    pub cmd_dispatch_graph_amdx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                DeviceAddress,
                *const DispatchGraphCountInfoAMDX,
            ),
        >,
    >,
    pub cmd_dispatch_graph_indirect_amdx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                DeviceAddress,
                *const DispatchGraphCountInfoAMDX,
            ),
        >,
    >,
    pub cmd_dispatch_graph_indirect_count_amdx: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, DeviceAddress, DeviceAddress)>,
    >,
    pub cmd_set_sample_locations_ext: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const SampleLocationsInfoEXT)>,
    >,
    pub get_physical_device_multisample_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                SampleCountFlags,
                *const MultisamplePropertiesEXT,
            ),
        >,
    >,
    pub create_acceleration_structure_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AccelerationStructureCreateInfoKHR,
                *const AllocationCallbacks,
                *const AccelerationStructureKHR,
            ) -> Status,
        >,
    >,
    pub destroy_acceleration_structure_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<AccelerationStructureKHR>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub cmd_build_acceleration_structures_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const AccelerationStructureBuildGeometryInfoKHR,
                *const *const AccelerationStructureBuildRangeInfoKHR,
            ),
        >,
    >,
    pub cmd_build_acceleration_structures_indirect_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const AccelerationStructureBuildGeometryInfoKHR,
                *const DeviceAddress,
                *const u32,
                *const *const u32,
            ),
        >,
    >,
    pub build_acceleration_structures_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                u32,
                *const AccelerationStructureBuildGeometryInfoKHR,
                *const *const AccelerationStructureBuildRangeInfoKHR,
            ) -> Status,
        >,
    >,
    pub copy_acceleration_structure_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const CopyAccelerationStructureInfoKHR,
            ) -> Status,
        >,
    >,
    pub copy_acceleration_structure_to_memory_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const CopyAccelerationStructureToMemoryInfoKHR,
            ) -> Status,
        >,
    >,
    pub copy_memory_to_acceleration_structure_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const CopyMemoryToAccelerationStructureInfoKHR,
            ) -> Status,
        >,
    >,
    pub write_acceleration_structures_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const AccelerationStructureKHR,
                QueryType,
                usize,
                VoidPtr,
                usize,
            ) -> Status,
        >,
    >,
    pub cmd_copy_acceleration_structure_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const CopyAccelerationStructureInfoKHR,
            ),
        >,
    >,
    pub cmd_copy_acceleration_structure_to_memory_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const CopyAccelerationStructureToMemoryInfoKHR,
            ),
        >,
    >,
    pub cmd_copy_memory_to_acceleration_structure_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const CopyMemoryToAccelerationStructureInfoKHR,
            ),
        >,
    >,
    pub get_acceleration_structure_device_address_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AccelerationStructureDeviceAddressInfoKHR,
            ) -> DeviceAddress,
        >,
    >,
    pub cmd_write_acceleration_structures_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const AccelerationStructureKHR,
                QueryType,
                Option<QueryPool>,
                u32,
            ),
        >,
    >,
    pub get_device_acceleration_structure_compatibility_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AccelerationStructureVersionInfoKHR,
                *const AccelerationStructureCompatibilityKHR,
            ),
        >,
    >,
    pub get_acceleration_structure_build_sizes_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                AccelerationStructureBuildTypeKHR,
                *const AccelerationStructureBuildGeometryInfoKHR,
                *const u32,
                *const AccelerationStructureBuildSizesInfoKHR,
            ),
        >,
    >,
    pub cmd_trace_rays_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const StridedDeviceAddressRegionKHR,
                *const StridedDeviceAddressRegionKHR,
                *const StridedDeviceAddressRegionKHR,
                *const StridedDeviceAddressRegionKHR,
                u32,
                u32,
                u32,
            ),
        >,
    >,
    pub create_ray_tracing_pipelines_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                Option<PipelineCache>,
                u32,
                *const RayTracingPipelineCreateInfoKHR,
                *const AllocationCallbacks,
                *const Pipeline,
            ) -> Status,
        >,
    >,
    pub get_ray_tracing_shader_group_handles_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                u32,
                u32,
                usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_ray_tracing_shader_group_handles_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                u32,
                u32,
                usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                u32,
                u32,
                usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub cmd_trace_rays_indirect_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const StridedDeviceAddressRegionKHR,
                *const StridedDeviceAddressRegionKHR,
                *const StridedDeviceAddressRegionKHR,
                *const StridedDeviceAddressRegionKHR,
                DeviceAddress,
            ),
        >,
    >,
    pub get_ray_tracing_shader_group_stack_size_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Pipeline>,
                u32,
                ShaderGroupShaderKHR,
            ) -> DeviceSize,
        >,
    >,
    pub cmd_set_ray_tracing_pipeline_stack_size_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32)>>,
    pub get_image_drm_format_modifier_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Image>,
                *const ImageDrmFormatModifierPropertiesEXT,
            ) -> Status,
        >,
    >,
    pub create_validation_cache_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ValidationCacheCreateInfoEXT,
                *const AllocationCallbacks,
                *const ValidationCacheEXT,
            ) -> Status,
        >,
    >,
    pub destroy_validation_cache_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ValidationCacheEXT>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub merge_validation_caches_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ValidationCacheEXT>,
                u32,
                *const ValidationCacheEXT,
            ) -> Status,
        >,
    >,
    pub get_validation_cache_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ValidationCacheEXT>,
                *const usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub cmd_bind_shading_rate_image_nv: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<ImageView>, ImageLayout)>,
    >,
    pub cmd_set_viewport_shading_rate_palette_nv: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const ShadingRatePaletteNV),
        >,
    >,
    pub cmd_set_coarse_sample_order_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                CoarseSampleOrderTypeNV,
                u32,
                *const CoarseSampleOrderCustomNV,
            ),
        >,
    >,
    pub create_acceleration_structure_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AccelerationStructureCreateInfoNV,
                *const AllocationCallbacks,
                *const AccelerationStructureNV,
            ) -> Status,
        >,
    >,
    pub destroy_acceleration_structure_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<AccelerationStructureNV>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_acceleration_structure_memory_requirements_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AccelerationStructureMemoryRequirementsInfoNV,
                *const MemoryRequirements2KHR,
            ),
        >,
    >,
    pub bind_acceleration_structure_memory_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const BindAccelerationStructureMemoryInfoNV,
            ) -> Status,
        >,
    >,
    pub cmd_build_acceleration_structure_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const AccelerationStructureInfoNV,
                Option<Buffer>,
                DeviceSize,
                Bool32,
                Option<AccelerationStructureNV>,
                Option<AccelerationStructureNV>,
                Option<Buffer>,
                DeviceSize,
            ),
        >,
    >,
    pub cmd_copy_acceleration_structure_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<AccelerationStructureNV>,
                Option<AccelerationStructureNV>,
                CopyAccelerationStructureModeKHR,
            ),
        >,
    >,
    pub cmd_trace_rays_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                u32,
                u32,
                u32,
            ),
        >,
    >,
    pub create_ray_tracing_pipelines_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PipelineCache>,
                u32,
                *const RayTracingPipelineCreateInfoNV,
                *const AllocationCallbacks,
                *const Pipeline,
            ) -> Status,
        >,
    >,
    pub get_acceleration_structure_handle_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<AccelerationStructureNV>,
                usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub cmd_write_acceleration_structures_properties_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const AccelerationStructureNV,
                QueryType,
                Option<QueryPool>,
                u32,
            ),
        >,
    >,
    pub compile_deferred_nv:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<Pipeline>, u32) -> Status>>,
    pub get_memory_host_pointer_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ExternalMemoryHandleTypeFlags,
                VoidPtr,
                *const MemoryHostPointerPropertiesEXT,
            ) -> Status,
        >,
    >,
    pub cmd_write_buffer_marker_amd: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineStageFlags,
                Option<Buffer>,
                DeviceSize,
                u32,
            ),
        >,
    >,
    pub cmd_draw_mesh_tasks_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32)>>,
    pub cmd_draw_mesh_tasks_indirect_nv: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize, u32, u32),
        >,
    >,
    pub cmd_draw_mesh_tasks_indirect_count_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_set_exclusive_scissor_enable_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const Bool32)>>,
    pub cmd_set_exclusive_scissor_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const Rect2D)>>,
    pub cmd_set_checkpoint_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, VoidPtr)>>,
    pub get_queue_checkpoint_data_nv:
        Cell<Option<unsafe extern "system" fn(Option<Queue>, *const u32, *const CheckpointDataNV)>>,
    pub initialize_performance_api_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const InitializePerformanceApiInfoINTEL,
            ) -> Status,
        >,
    >,
    pub uninitialize_performance_api_intel: Cell<Option<unsafe extern "system" fn(Option<Device>)>>,
    pub cmd_set_performance_marker_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const PerformanceMarkerInfoINTEL,
            ) -> Status,
        >,
    >,
    pub cmd_set_performance_stream_marker_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const PerformanceStreamMarkerInfoINTEL,
            ) -> Status,
        >,
    >,
    pub cmd_set_performance_override_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const PerformanceOverrideInfoINTEL,
            ) -> Status,
        >,
    >,
    pub acquire_performance_configuration_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PerformanceConfigurationAcquireInfoINTEL,
                *const PerformanceConfigurationINTEL,
            ) -> Status,
        >,
    >,
    pub release_performance_configuration_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<PerformanceConfigurationINTEL>,
            ) -> Status,
        >,
    >,
    pub queue_set_performance_configuration_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Queue>,
                Option<PerformanceConfigurationINTEL>,
            ) -> Status,
        >,
    >,
    pub get_performance_parameter_intel: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                PerformanceParameterTypeINTEL,
                *const PerformanceValueINTEL,
            ) -> Status,
        >,
    >,
    pub set_local_dimming_amd:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<SwapchainKHR>, Bool32)>>,
    pub create_image_pipe_surface_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const ImagePipeSurfaceCreateInfoFUCHSIA,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub create_metal_surface_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const MetalSurfaceCreateInfoEXT,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_fragment_shading_rates_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const PhysicalDeviceFragmentShadingRateKHR,
            ) -> Status,
        >,
    >,
    pub cmd_set_fragment_shading_rate_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const Extent2D,
                [FragmentShadingRateCombinerOpKHR; 2u16 as _],
            ),
        >,
    >,
    pub cmd_set_rendering_attachment_locations_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const RenderingAttachmentLocationInfoKHR,
            ),
        >,
    >,
    pub cmd_set_rendering_input_attachment_indices_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const RenderingInputAttachmentIndexInfoKHR,
            ),
        >,
    >,
    pub wait_for_present_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<SwapchainKHR>, u64, u64) -> Status>,
    >,
    pub get_physical_device_cooperative_matrix_properties_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const CooperativeMatrixPropertiesNV,
            ) -> Status,
        >,
    >,
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const FramebufferMixedSamplesCombinationNV,
            ) -> Status,
        >,
    >,
    pub get_physical_device_surface_present_modes2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const PhysicalDeviceSurfaceInfo2KHR,
                *const u32,
                *const PresentModeKHR,
            ) -> Status,
        >,
    >,
    pub acquire_full_screen_exclusive_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<SwapchainKHR>) -> Status>>,
    pub release_full_screen_exclusive_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<SwapchainKHR>) -> Status>>,
    pub get_device_group_surface_present_modes2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PhysicalDeviceSurfaceInfo2KHR,
                *const DeviceGroupPresentModeFlagsKHR,
            ) -> Status,
        >,
    >,
    pub create_headless_surface_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const HeadlessSurfaceCreateInfoEXT,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub create_deferred_operation_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AllocationCallbacks,
                *const DeferredOperationKHR,
            ) -> Status,
        >,
    >,
    pub destroy_deferred_operation_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_deferred_operation_max_concurrency_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<DeferredOperationKHR>) -> u32>,
    >,
    pub get_deferred_operation_result_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<DeferredOperationKHR>) -> Status>,
    >,
    pub deferred_operation_join_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, Option<DeferredOperationKHR>) -> Status>,
    >,
    pub get_pipeline_executable_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineInfoKHR,
                *const u32,
                *const PipelineExecutablePropertiesKHR,
            ) -> Status,
        >,
    >,
    pub get_pipeline_executable_statistics_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineExecutableInfoKHR,
                *const u32,
                *const PipelineExecutableStatisticKHR,
            ) -> Status,
        >,
    >,
    pub get_pipeline_executable_internal_representations_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineExecutableInfoKHR,
                *const u32,
                *const PipelineExecutableInternalRepresentationKHR,
            ) -> Status,
        >,
    >,
    pub copy_memory_to_image_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const CopyMemoryToImageInfoEXT) -> Status,
        >,
    >,
    pub copy_image_to_memory_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const CopyImageToMemoryInfoEXT) -> Status,
        >,
    >,
    pub copy_image_to_image_ext: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const CopyImageToImageInfoEXT) -> Status>,
    >,
    pub transition_image_layout_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const HostImageLayoutTransitionInfoEXT,
            ) -> Status,
        >,
    >,
    pub map_memory2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryMapInfoKHR,
                *const *const c_void,
            ) -> Status,
        >,
    >,
    pub unmap_memory2_khr: Cell<
        Option<unsafe extern "system" fn(Option<Device>, *const MemoryUnmapInfoKHR) -> Status>,
    >,
    pub release_swapchain_images_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ReleaseSwapchainImagesInfoEXT,
            ) -> Status,
        >,
    >,
    pub get_generated_commands_memory_requirements_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const GeneratedCommandsMemoryRequirementsInfoNV,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub cmd_preprocess_generated_commands_nv: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const GeneratedCommandsInfoNV)>,
    >,
    pub cmd_execute_generated_commands_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Bool32,
                *const GeneratedCommandsInfoNV,
            ),
        >,
    >,
    pub cmd_bind_pipeline_shader_group_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineBindPoint,
                Option<Pipeline>,
                u32,
            ),
        >,
    >,
    pub create_indirect_commands_layout_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const IndirectCommandsLayoutCreateInfoNV,
                *const AllocationCallbacks,
                *const IndirectCommandsLayoutNV,
            ) -> Status,
        >,
    >,
    pub destroy_indirect_commands_layout_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<IndirectCommandsLayoutNV>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub cmd_set_depth_bias2_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const DepthBiasInfoEXT)>>,
    pub acquire_drm_display_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<PhysicalDevice>, i32, Option<DisplayKHR>) -> Status,
        >,
    >,
    pub get_drm_display_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                i32,
                u32,
                *const DisplayKHR,
            ) -> Status,
        >,
    >,
    pub create_cuda_module_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const CudaModuleCreateInfoNV,
                *const AllocationCallbacks,
                *const CudaModuleNV,
            ) -> Status,
        >,
    >,
    pub get_cuda_module_cache_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CudaModuleNV>,
                *const usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub create_cuda_function_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const CudaFunctionCreateInfoNV,
                *const AllocationCallbacks,
                *const CudaFunctionNV,
            ) -> Status,
        >,
    >,
    pub destroy_cuda_module_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CudaModuleNV>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub destroy_cuda_function_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<CudaFunctionNV>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub cmd_cuda_launch_kernel_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CudaLaunchInfoNV)>>,
    pub export_metal_objects_ext:
        Cell<Option<unsafe extern "system" fn(Option<Device>, *const ExportMetalObjectsInfoEXT)>>,
    pub cmd_write_buffer_marker2_amd: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, u32, Option<Buffer>, DeviceSize, u32),
        >,
    >,
    pub get_queue_checkpoint_data2_nv: Cell<
        Option<unsafe extern "system" fn(Option<Queue>, *const u32, *const CheckpointData2NV)>,
    >,
    pub get_descriptor_set_layout_size_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorSetLayout>,
                *const DeviceSize,
            ),
        >,
    >,
    pub get_descriptor_set_layout_binding_offset_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DescriptorSetLayout>,
                u32,
                *const DeviceSize,
            ),
        >,
    >,
    pub get_descriptor_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const DescriptorGetInfoEXT, usize, VoidPtr),
        >,
    >,
    pub cmd_bind_descriptor_buffers_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const DescriptorBufferBindingInfoEXT,
            ),
        >,
    >,
    pub cmd_set_descriptor_buffer_offsets_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineBindPoint,
                Option<PipelineLayout>,
                u32,
                u32,
                *const u32,
                *const DeviceSize,
            ),
        >,
    >,
    pub cmd_bind_descriptor_buffer_embedded_samplers_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                PipelineBindPoint,
                Option<PipelineLayout>,
                u32,
            ),
        >,
    >,
    pub get_buffer_opaque_capture_descriptor_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferCaptureDescriptorDataInfoEXT,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_image_opaque_capture_descriptor_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageCaptureDescriptorDataInfoEXT,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_image_view_opaque_capture_descriptor_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImageViewCaptureDescriptorDataInfoEXT,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_sampler_opaque_capture_descriptor_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SamplerCaptureDescriptorDataInfoEXT,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_acceleration_structure_opaque_capture_descriptor_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const AccelerationStructureCaptureDescriptorDataInfoEXT,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub cmd_set_fragment_shading_rate_enum_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                FragmentShadingRateNV,
                [FragmentShadingRateCombinerOpKHR; 2u16 as _],
            ),
        >,
    >,
    pub cmd_draw_mesh_tasks_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32)>>,
    pub cmd_draw_mesh_tasks_indirect_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize, u32, u32),
        >,
    >,
    pub cmd_draw_mesh_tasks_indirect_count_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                Option<Buffer>,
                DeviceSize,
                u32,
                u32,
            ),
        >,
    >,
    pub get_device_fault_info_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceFaultCountsEXT,
                *const DeviceFaultInfoEXT,
            ) -> Status,
        >,
    >,
    pub acquire_winrt_display_nv: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, Option<DisplayKHR>) -> Status>,
    >,
    pub get_winrt_display_nv: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, u32, *const DisplayKHR) -> Status>,
    >,
    pub create_direct_fbsurface_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const DirectFBSurfaceCreateInfoEXT,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_direct_fbpresentation_support_ext: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, u32, *const VoidPtr) -> Bool32>,
    >,
    pub cmd_set_vertex_input_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const VertexInputBindingDescription2EXT,
                u32,
                *const VertexInputAttributeDescription2EXT,
            ),
        >,
    >,
    pub get_memory_zircon_handle_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryGetZirconHandleInfoFUCHSIA,
                *const VoidPtr,
            ) -> Status,
        >,
    >,
    pub get_memory_zircon_handle_properties_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                ExternalMemoryHandleTypeFlags,
                VoidPtr,
                *const MemoryZirconHandlePropertiesFUCHSIA,
            ) -> Status,
        >,
    >,
    pub import_semaphore_zircon_handle_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ImportSemaphoreZirconHandleInfoFUCHSIA,
            ) -> Status,
        >,
    >,
    pub get_semaphore_zircon_handle_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const SemaphoreGetZirconHandleInfoFUCHSIA,
                *const VoidPtr,
            ) -> Status,
        >,
    >,
    pub create_buffer_collection_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const BufferCollectionCreateInfoFUCHSIA,
                *const AllocationCallbacks,
                *const BufferCollectionFUCHSIA,
            ) -> Status,
        >,
    >,
    pub set_buffer_collection_image_constraints_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<BufferCollectionFUCHSIA>,
                *const ImageConstraintsInfoFUCHSIA,
            ) -> Status,
        >,
    >,
    pub set_buffer_collection_buffer_constraints_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<BufferCollectionFUCHSIA>,
                *const BufferConstraintsInfoFUCHSIA,
            ) -> Status,
        >,
    >,
    pub destroy_buffer_collection_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<BufferCollectionFUCHSIA>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_buffer_collection_properties_fuchsia: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<BufferCollectionFUCHSIA>,
                *const BufferCollectionPropertiesFUCHSIA,
            ) -> Status,
        >,
    >,
    pub get_device_subpass_shading_max_workgroup_size_huawei: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<RenderPass>,
                *const Extent2D,
            ) -> Status,
        >,
    >,
    pub cmd_subpass_shading_huawei: Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>)>>,
    pub cmd_bind_invocation_mask_huawei: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<ImageView>, ImageLayout)>,
    >,
    pub get_memory_remote_address_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MemoryGetRemoteAddressInfoNV,
                *const RemoteAddressNV,
            ) -> Status,
        >,
    >,
    pub get_pipeline_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineInfoEXT,
                *const BaseOutStructure,
            ) -> Status,
        >,
    >,
    pub cmd_set_patch_control_points_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32)>>,
    pub cmd_set_logic_op_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, LogicOp)>>,
    pub create_screen_surface_qnx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Instance>,
                *const ScreenSurfaceCreateInfoQNX,
                *const AllocationCallbacks,
                *const SurfaceKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_screen_presentation_support_qnx: Cell<
        Option<unsafe extern "system" fn(Option<PhysicalDevice>, u32, *const VoidPtr) -> Bool32>,
    >,
    pub cmd_set_color_write_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const Bool32)>>,
    pub cmd_trace_rays_indirect2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, DeviceAddress)>>,
    pub cmd_draw_multi_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const MultiDrawInfoEXT,
                u32,
                u32,
                u32,
            ),
        >,
    >,
    pub cmd_draw_multi_indexed_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const MultiDrawIndexedInfoEXT,
                u32,
                u32,
                u32,
                *const i32,
            ),
        >,
    >,
    pub create_micromap_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MicromapCreateInfoEXT,
                *const AllocationCallbacks,
                *const MicromapEXT,
            ) -> Status,
        >,
    >,
    pub destroy_micromap_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<MicromapEXT>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub cmd_build_micromaps_ext: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const MicromapBuildInfoEXT)>,
    >,
    pub build_micromaps_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                u32,
                *const MicromapBuildInfoEXT,
            ) -> Status,
        >,
    >,
    pub copy_micromap_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const CopyMicromapInfoEXT,
            ) -> Status,
        >,
    >,
    pub copy_micromap_to_memory_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const CopyMicromapToMemoryInfoEXT,
            ) -> Status,
        >,
    >,
    pub copy_memory_to_micromap_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<DeferredOperationKHR>,
                *const CopyMemoryToMicromapInfoEXT,
            ) -> Status,
        >,
    >,
    pub write_micromaps_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const MicromapEXT,
                QueryType,
                usize,
                VoidPtr,
                usize,
            ) -> Status,
        >,
    >,
    pub cmd_copy_micromap_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const CopyMicromapInfoEXT)>>,
    pub cmd_copy_micromap_to_memory_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, *const CopyMicromapToMemoryInfoEXT),
        >,
    >,
    pub cmd_copy_memory_to_micromap_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, *const CopyMemoryToMicromapInfoEXT),
        >,
    >,
    pub cmd_write_micromaps_properties_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const MicromapEXT,
                QueryType,
                Option<QueryPool>,
                u32,
            ),
        >,
    >,
    pub get_device_micromap_compatibility_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const MicromapVersionInfoEXT,
                *const AccelerationStructureCompatibilityKHR,
            ),
        >,
    >,
    pub get_micromap_build_sizes_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                AccelerationStructureBuildTypeKHR,
                *const MicromapBuildInfoEXT,
                *const MicromapBuildSizesInfoEXT,
            ),
        >,
    >,
    pub cmd_draw_cluster_huawei:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, u32)>>,
    pub cmd_draw_cluster_indirect_huawei:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Option<Buffer>, DeviceSize)>>,
    pub set_device_memory_priority_ext:
        Cell<Option<unsafe extern "system" fn(Option<Device>, Option<DeviceMemory>, f32)>>,
    pub get_descriptor_set_layout_host_mapping_info_valve: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DescriptorSetBindingReferenceVALVE,
                *const DescriptorSetLayoutHostMappingInfoVALVE,
            ),
        >,
    >,
    pub get_descriptor_set_host_mapping_valve: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, Option<DescriptorSet>, *const *const c_void),
        >,
    >,
    pub cmd_copy_memory_indirect_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, DeviceAddress, u32, u32)>>,
    pub cmd_copy_memory_to_image_indirect_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                DeviceAddress,
                u32,
                u32,
                Option<Image>,
                ImageLayout,
                *const ImageSubresourceLayers,
            ),
        >,
    >,
    pub cmd_decompress_memory_nv: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, u32, *const DecompressMemoryRegionNV),
        >,
    >,
    pub cmd_decompress_memory_indirect_count_nv: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, DeviceAddress, DeviceAddress, u32)>,
    >,
    pub get_pipeline_indirect_memory_requirements_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ComputePipelineCreateInfo,
                *const MemoryRequirements2,
            ),
        >,
    >,
    pub cmd_update_pipeline_indirect_buffer_nv: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, PipelineBindPoint, Option<Pipeline>),
        >,
    >,
    pub get_pipeline_indirect_device_address_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const PipelineIndirectDeviceAddressInfoNV,
            ) -> DeviceAddress,
        >,
    >,
    pub cmd_set_depth_clamp_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_polygon_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, PolygonMode)>>,
    pub cmd_set_rasterization_samples_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, SampleCountFlags)>>,
    pub cmd_set_sample_mask_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, SampleCountFlags, *const SampleMask),
        >,
    >,
    pub cmd_set_alpha_to_coverage_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_alpha_to_one_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_logic_op_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_color_blend_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const Bool32)>>,
    pub cmd_set_color_blend_equation_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const ColorBlendEquationEXT,
            ),
        >,
    >,
    pub cmd_set_color_write_mask_ext: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const ColorComponentFlags),
        >,
    >,
    pub cmd_set_tessellation_domain_origin_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, TessellationDomainOrigin)>>,
    pub cmd_set_rasterization_stream_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32)>>,
    pub cmd_set_conservative_rasterization_mode_ext: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, ConservativeRasterizationModeEXT)>,
    >,
    pub cmd_set_extra_primitive_overestimation_size_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, f32)>>,
    pub cmd_set_depth_clip_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_sample_locations_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_color_blend_advanced_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                u32,
                *const ColorBlendAdvancedEXT,
            ),
        >,
    >,
    pub cmd_set_provoking_vertex_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, ProvokingVertexModeEXT)>>,
    pub cmd_set_line_rasterization_mode_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, LineRasterizationModeEXT)>>,
    pub cmd_set_line_stipple_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_depth_clip_negative_one_to_one_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_viewport_wscaling_enable_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_viewport_swizzle_nv: Cell<
        Option<
            unsafe extern "system" fn(Option<CommandBuffer>, u32, u32, *const ViewportSwizzleNV),
        >,
    >,
    pub cmd_set_coverage_to_color_enable_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_coverage_to_color_location_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32)>>,
    pub cmd_set_coverage_modulation_mode_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, CoverageModulationModeNV)>>,
    pub cmd_set_coverage_modulation_table_enable_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_coverage_modulation_table_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, *const f32)>>,
    pub cmd_set_shading_rate_image_enable_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_representative_fragment_test_enable_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, Bool32)>>,
    pub cmd_set_coverage_reduction_mode_nv:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, CoverageReductionModeNV)>>,
    pub get_shader_module_identifier_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ShaderModule>,
                *const ShaderModuleIdentifierEXT,
            ),
        >,
    >,
    pub get_shader_module_create_info_identifier_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const ShaderModuleCreateInfo,
                *const ShaderModuleIdentifierEXT,
            ),
        >,
    >,
    pub get_physical_device_optical_flow_image_formats_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const OpticalFlowImageFormatInfoNV,
                *const u32,
                *const OpticalFlowImageFormatPropertiesNV,
            ) -> Status,
        >,
    >,
    pub create_optical_flow_session_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const OpticalFlowSessionCreateInfoNV,
                *const AllocationCallbacks,
                *const OpticalFlowSessionNV,
            ) -> Status,
        >,
    >,
    pub destroy_optical_flow_session_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<OpticalFlowSessionNV>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub bind_optical_flow_session_image_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<OpticalFlowSessionNV>,
                OpticalFlowSessionBindingPointNV,
                Option<ImageView>,
                ImageLayout,
            ) -> Status,
        >,
    >,
    pub cmd_optical_flow_execute_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<OpticalFlowSessionNV>,
                *const OpticalFlowExecuteInfoNV,
            ),
        >,
    >,
    pub cmd_bind_index_buffer2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                Option<Buffer>,
                DeviceSize,
                DeviceSize,
                IndexType,
            ),
        >,
    >,
    pub get_rendering_area_granularity_khr: Cell<
        Option<
            unsafe extern "system" fn(Option<Device>, *const RenderingAreaInfoKHR, *const Extent2D),
        >,
    >,
    pub get_device_image_subresource_layout_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const DeviceImageSubresourceInfoKHR,
                *const SubresourceLayout2KHR,
            ),
        >,
    >,
    pub get_image_subresource_layout2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Image>,
                *const ImageSubresource2KHR,
                *const SubresourceLayout2KHR,
            ),
        >,
    >,
    pub get_image_subresource_layout2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Image>,
                *const ImageSubresource2KHR,
                *const SubresourceLayout2KHR,
            ),
        >,
    >,
    pub create_shaders_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const ShaderCreateInfoEXT,
                *const AllocationCallbacks,
                *const ShaderEXT,
            ) -> Status,
        >,
    >,
    pub destroy_shader_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ShaderEXT>,
                *const AllocationCallbacks,
            ),
        >,
    >,
    pub get_shader_binary_data_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<ShaderEXT>,
                *const usize,
                VoidPtr,
            ) -> Status,
        >,
    >,
    pub cmd_bind_shaders_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                u32,
                *const ShaderStageFlags,
                *const ShaderEXT,
            ),
        >,
    >,
    pub get_framebuffer_tile_properties_qcom: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<Framebuffer>,
                *const u32,
                *const TilePropertiesQCOM,
            ) -> Status,
        >,
    >,
    pub get_dynamic_rendering_tile_properties_qcom: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const RenderingInfo,
                *const TilePropertiesQCOM,
            ) -> Status,
        >,
    >,
    pub set_latency_sleep_mode_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const LatencySleepModeInfoNV,
            ) -> Status,
        >,
    >,
    pub latency_sleep_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const LatencySleepInfoNV,
            ) -> Status,
        >,
    >,
    pub set_latency_marker_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const SetLatencyMarkerInfoNV,
            ),
        >,
    >,
    pub get_latency_timings_nv: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                Option<SwapchainKHR>,
                *const GetLatencyMarkerInfoNV,
            ),
        >,
    >,
    pub queue_notify_out_of_band_nv:
        Cell<Option<unsafe extern "system" fn(Option<Queue>, *const OutOfBandQueueTypeInfoNV)>>,
    pub get_physical_device_cooperative_matrix_properties_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const CooperativeMatrixPropertiesKHR,
            ) -> Status,
        >,
    >,
    pub cmd_set_attachment_feedback_loop_enable_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, ImageAspectFlags)>>,
    pub get_screen_buffer_properties_qnx: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                *const VoidPtr,
                *const ScreenBufferPropertiesQNX,
            ) -> Status,
        >,
    >,
    pub cmd_set_line_stipple_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u16)>>,
    pub cmd_set_line_stipple_ext:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, u32, u16)>>,
    pub get_physical_device_calibrateable_time_domains_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const TimeDomainKHR,
            ) -> Status,
        >,
    >,
    pub get_physical_device_calibrateable_time_domains_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<PhysicalDevice>,
                *const u32,
                *const TimeDomainKHR,
            ) -> Status,
        >,
    >,
    pub get_calibrated_timestamps_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const CalibratedTimestampInfoKHR,
                *const u64,
                *const u64,
            ) -> Status,
        >,
    >,
    pub get_calibrated_timestamps_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<Device>,
                u32,
                *const CalibratedTimestampInfoKHR,
                *const u64,
                *const u64,
            ) -> Status,
        >,
    >,
    pub cmd_bind_descriptor_sets2_khr: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const BindDescriptorSetsInfoKHR)>,
    >,
    pub cmd_push_constants2_khr:
        Cell<Option<unsafe extern "system" fn(Option<CommandBuffer>, *const PushConstantsInfoKHR)>>,
    pub cmd_push_descriptor_set2_khr: Cell<
        Option<unsafe extern "system" fn(Option<CommandBuffer>, *const PushDescriptorSetInfoKHR)>,
    >,
    pub cmd_push_descriptor_set_with_template2_khr: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const PushDescriptorSetWithTemplateInfoKHR,
            ),
        >,
    >,
    pub cmd_set_descriptor_buffer_offsets2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const SetDescriptorBufferOffsetsInfoEXT,
            ),
        >,
    >,
    pub cmd_bind_descriptor_buffer_embedded_samplers2_ext: Cell<
        Option<
            unsafe extern "system" fn(
                Option<CommandBuffer>,
                *const BindDescriptorBufferEmbeddedSamplersInfoEXT,
            ),
        >,
    >,
}
unsafe impl Send for CommandsDispatcher {}
unsafe impl Sync for CommandsDispatcher {}
impl CommandsDispatcher {
    pub unsafe fn load_proc_addr(
        &self,
        get_instance_proc_addr: unsafe extern "system" fn(
            Option<Instance>,
            *const c_char,
        ) -> FuncPtr,
    ) {
        self.get_instance_proc_addr
            .set(Some(get_instance_proc_addr));
        self.create_instance
            .set(mem::transmute(get_instance_proc_addr(
                None,
                c"vkCreateInstance".as_ptr(),
            )));
        self.enumerate_instance_extension_properties
            .set(mem::transmute(get_instance_proc_addr(
                None,
                c"vkEnumerateInstanceExtensionProperties".as_ptr(),
            )));
        self.enumerate_instance_layer_properties
            .set(mem::transmute(get_instance_proc_addr(
                None,
                c"vkEnumerateInstanceLayerProperties".as_ptr(),
            )));
        self.enumerate_instance_version
            .set(mem::transmute(get_instance_proc_addr(
                None,
                c"vkEnumerateInstanceVersion".as_ptr(),
            )));
    }
    pub unsafe fn load_instance(&self, instance: &Instance) {
        let get_instance_proc_addr = self
            .get_instance_proc_addr
            .get()
            .expect("load_proc_addr must be called before load_instance");
        let get_instance = || Some(instance.clone());
        self.destroy_instance
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyInstance".as_ptr(),
            )));
        self.enumerate_physical_devices
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEnumeratePhysicalDevices".as_ptr(),
            )));
        self.get_physical_device_features
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFeatures".as_ptr(),
            )));
        self.get_physical_device_format_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFormatProperties".as_ptr(),
            )));
        self.get_physical_device_image_format_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceImageFormatProperties".as_ptr(),
            )));
        self.get_physical_device_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceProperties".as_ptr(),
            )));
        self.get_physical_device_queue_family_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceQueueFamilyProperties".as_ptr(),
            )));
        self.get_physical_device_memory_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceMemoryProperties".as_ptr(),
            )));
        self.get_instance_proc_addr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetInstanceProcAddr".as_ptr(),
            )));
        self.get_device_proc_addr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceProcAddr".as_ptr(),
            )));
        self.create_device
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDevice".as_ptr(),
            )));
        self.destroy_device
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDevice".as_ptr(),
            )));
        self.enumerate_device_extension_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEnumerateDeviceExtensionProperties".as_ptr(),
            )));
        self.enumerate_device_layer_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEnumerateDeviceLayerProperties".as_ptr(),
            )));
        self.get_device_queue
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceQueue".as_ptr(),
            )));
        self.queue_submit.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkQueueSubmit".as_ptr(),
        )));
        self.queue_wait_idle
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueWaitIdle".as_ptr(),
            )));
        self.device_wait_idle
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDeviceWaitIdle".as_ptr(),
            )));
        self.allocate_memory
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAllocateMemory".as_ptr(),
            )));
        self.free_memory.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkFreeMemory".as_ptr(),
        )));
        self.map_memory.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkMapMemory".as_ptr(),
        )));
        self.unmap_memory.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkUnmapMemory".as_ptr(),
        )));
        self.flush_mapped_memory_ranges
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkFlushMappedMemoryRanges".as_ptr(),
            )));
        self.invalidate_mapped_memory_ranges
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkInvalidateMappedMemoryRanges".as_ptr(),
            )));
        self.get_device_memory_commitment
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceMemoryCommitment".as_ptr(),
            )));
        self.bind_buffer_memory
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindBufferMemory".as_ptr(),
            )));
        self.bind_image_memory
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindImageMemory".as_ptr(),
            )));
        self.get_buffer_memory_requirements
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferMemoryRequirements".as_ptr(),
            )));
        self.get_image_memory_requirements
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageMemoryRequirements".as_ptr(),
            )));
        self.get_image_sparse_memory_requirements
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageSparseMemoryRequirements".as_ptr(),
            )));
        self.get_physical_device_sparse_image_format_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSparseImageFormatProperties".as_ptr(),
            )));
        self.queue_bind_sparse
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueBindSparse".as_ptr(),
            )));
        self.create_fence.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkCreateFence".as_ptr(),
        )));
        self.destroy_fence
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyFence".as_ptr(),
            )));
        self.reset_fences.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkResetFences".as_ptr(),
        )));
        self.get_fence_status
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetFenceStatus".as_ptr(),
            )));
        self.wait_for_fences
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkWaitForFences".as_ptr(),
            )));
        self.create_semaphore
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateSemaphore".as_ptr(),
            )));
        self.destroy_semaphore
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroySemaphore".as_ptr(),
            )));
        self.create_event.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkCreateEvent".as_ptr(),
        )));
        self.destroy_event
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyEvent".as_ptr(),
            )));
        self.get_event_status
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetEventStatus".as_ptr(),
            )));
        self.set_event.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkSetEvent".as_ptr(),
        )));
        self.reset_event.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkResetEvent".as_ptr(),
        )));
        self.create_query_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateQueryPool".as_ptr(),
            )));
        self.destroy_query_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyQueryPool".as_ptr(),
            )));
        self.get_query_pool_results
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetQueryPoolResults".as_ptr(),
            )));
        self.create_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateBuffer".as_ptr(),
            )));
        self.destroy_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyBuffer".as_ptr(),
            )));
        self.create_buffer_view
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateBufferView".as_ptr(),
            )));
        self.destroy_buffer_view
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyBufferView".as_ptr(),
            )));
        self.create_image.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkCreateImage".as_ptr(),
        )));
        self.destroy_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyImage".as_ptr(),
            )));
        self.get_image_subresource_layout
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageSubresourceLayout".as_ptr(),
            )));
        self.create_image_view
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateImageView".as_ptr(),
            )));
        self.destroy_image_view
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyImageView".as_ptr(),
            )));
        self.create_shader_module
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateShaderModule".as_ptr(),
            )));
        self.destroy_shader_module
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyShaderModule".as_ptr(),
            )));
        self.create_pipeline_cache
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreatePipelineCache".as_ptr(),
            )));
        self.destroy_pipeline_cache
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyPipelineCache".as_ptr(),
            )));
        self.get_pipeline_cache_data
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelineCacheData".as_ptr(),
            )));
        self.merge_pipeline_caches
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkMergePipelineCaches".as_ptr(),
            )));
        self.create_graphics_pipelines
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateGraphicsPipelines".as_ptr(),
            )));
        self.create_compute_pipelines
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateComputePipelines".as_ptr(),
            )));
        self.destroy_pipeline
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyPipeline".as_ptr(),
            )));
        self.create_pipeline_layout
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreatePipelineLayout".as_ptr(),
            )));
        self.destroy_pipeline_layout
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyPipelineLayout".as_ptr(),
            )));
        self.create_sampler
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateSampler".as_ptr(),
            )));
        self.destroy_sampler
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroySampler".as_ptr(),
            )));
        self.create_descriptor_set_layout
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDescriptorSetLayout".as_ptr(),
            )));
        self.destroy_descriptor_set_layout
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDescriptorSetLayout".as_ptr(),
            )));
        self.create_descriptor_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDescriptorPool".as_ptr(),
            )));
        self.destroy_descriptor_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDescriptorPool".as_ptr(),
            )));
        self.reset_descriptor_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkResetDescriptorPool".as_ptr(),
            )));
        self.allocate_descriptor_sets
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAllocateDescriptorSets".as_ptr(),
            )));
        self.free_descriptor_sets
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkFreeDescriptorSets".as_ptr(),
            )));
        self.update_descriptor_sets
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkUpdateDescriptorSets".as_ptr(),
            )));
        self.create_framebuffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateFramebuffer".as_ptr(),
            )));
        self.destroy_framebuffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyFramebuffer".as_ptr(),
            )));
        self.create_render_pass
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateRenderPass".as_ptr(),
            )));
        self.destroy_render_pass
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyRenderPass".as_ptr(),
            )));
        self.get_render_area_granularity
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRenderAreaGranularity".as_ptr(),
            )));
        self.create_command_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateCommandPool".as_ptr(),
            )));
        self.destroy_command_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyCommandPool".as_ptr(),
            )));
        self.reset_command_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkResetCommandPool".as_ptr(),
            )));
        self.allocate_command_buffers
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAllocateCommandBuffers".as_ptr(),
            )));
        self.free_command_buffers
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkFreeCommandBuffers".as_ptr(),
            )));
        self.begin_command_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBeginCommandBuffer".as_ptr(),
            )));
        self.end_command_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEndCommandBuffer".as_ptr(),
            )));
        self.reset_command_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkResetCommandBuffer".as_ptr(),
            )));
        self.cmd_bind_pipeline
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindPipeline".as_ptr(),
            )));
        self.cmd_set_viewport
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewport".as_ptr(),
            )));
        self.cmd_set_scissor
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetScissor".as_ptr(),
            )));
        self.cmd_set_line_width
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLineWidth".as_ptr(),
            )));
        self.cmd_set_depth_bias
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBias".as_ptr(),
            )));
        self.cmd_set_blend_constants
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetBlendConstants".as_ptr(),
            )));
        self.cmd_set_depth_bounds
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBounds".as_ptr(),
            )));
        self.cmd_set_stencil_compare_mask
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilCompareMask".as_ptr(),
            )));
        self.cmd_set_stencil_write_mask
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilWriteMask".as_ptr(),
            )));
        self.cmd_set_stencil_reference
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilReference".as_ptr(),
            )));
        self.cmd_bind_descriptor_sets
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindDescriptorSets".as_ptr(),
            )));
        self.cmd_bind_index_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindIndexBuffer".as_ptr(),
            )));
        self.cmd_bind_vertex_buffers
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindVertexBuffers".as_ptr(),
            )));
        self.cmd_draw.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkCmdDraw".as_ptr(),
        )));
        self.cmd_draw_indexed
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndexed".as_ptr(),
            )));
        self.cmd_draw_indirect
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndirect".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndexedIndirect".as_ptr(),
            )));
        self.cmd_dispatch.set(mem::transmute(get_instance_proc_addr(
            get_instance(),
            c"vkCmdDispatch".as_ptr(),
        )));
        self.cmd_dispatch_indirect
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDispatchIndirect".as_ptr(),
            )));
        self.cmd_copy_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyBuffer".as_ptr(),
            )));
        self.cmd_copy_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyImage".as_ptr(),
            )));
        self.cmd_blit_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBlitImage".as_ptr(),
            )));
        self.cmd_copy_buffer_to_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyBufferToImage".as_ptr(),
            )));
        self.cmd_copy_image_to_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyImageToBuffer".as_ptr(),
            )));
        self.cmd_update_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdUpdateBuffer".as_ptr(),
            )));
        self.cmd_fill_buffer
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdFillBuffer".as_ptr(),
            )));
        self.cmd_clear_color_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdClearColorImage".as_ptr(),
            )));
        self.cmd_clear_depth_stencil_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdClearDepthStencilImage".as_ptr(),
            )));
        self.cmd_clear_attachments
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdClearAttachments".as_ptr(),
            )));
        self.cmd_resolve_image
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResolveImage".as_ptr(),
            )));
        self.cmd_set_event
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetEvent".as_ptr(),
            )));
        self.cmd_reset_event
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResetEvent".as_ptr(),
            )));
        self.cmd_wait_events
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWaitEvents".as_ptr(),
            )));
        self.cmd_pipeline_barrier
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPipelineBarrier".as_ptr(),
            )));
        self.cmd_begin_query
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginQuery".as_ptr(),
            )));
        self.cmd_end_query
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndQuery".as_ptr(),
            )));
        self.cmd_reset_query_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResetQueryPool".as_ptr(),
            )));
        self.cmd_write_timestamp
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteTimestamp".as_ptr(),
            )));
        self.cmd_copy_query_pool_results
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyQueryPoolResults".as_ptr(),
            )));
        self.cmd_push_constants
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPushConstants".as_ptr(),
            )));
        self.cmd_begin_render_pass
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginRenderPass".as_ptr(),
            )));
        self.cmd_next_subpass
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdNextSubpass".as_ptr(),
            )));
        self.cmd_end_render_pass
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndRenderPass".as_ptr(),
            )));
        self.cmd_execute_commands
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdExecuteCommands".as_ptr(),
            )));
        self.bind_buffer_memory2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindBufferMemory2".as_ptr(),
            )));
        self.bind_buffer_memory2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindBufferMemory2KHR".as_ptr(),
            )));
        self.bind_buffer_memory2.set(
            self.bind_buffer_memory2
                .get()
                .or(self.bind_buffer_memory2_khr.get()),
        );
        self.bind_image_memory2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindImageMemory2".as_ptr(),
            )));
        self.bind_image_memory2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindImageMemory2KHR".as_ptr(),
            )));
        self.bind_image_memory2.set(
            self.bind_image_memory2
                .get()
                .or(self.bind_image_memory2_khr.get()),
        );
        self.get_device_group_peer_memory_features
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceGroupPeerMemoryFeatures".as_ptr(),
            )));
        self.get_device_group_peer_memory_features_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceGroupPeerMemoryFeaturesKHR".as_ptr(),
            )));
        self.get_device_group_peer_memory_features.set(
            self.get_device_group_peer_memory_features
                .get()
                .or(self.get_device_group_peer_memory_features_khr.get()),
        );
        self.cmd_set_device_mask
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDeviceMask".as_ptr(),
            )));
        self.cmd_set_device_mask_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDeviceMaskKHR".as_ptr(),
            )));
        self.cmd_set_device_mask.set(
            self.cmd_set_device_mask
                .get()
                .or(self.cmd_set_device_mask_khr.get()),
        );
        self.cmd_dispatch_base
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDispatchBase".as_ptr(),
            )));
        self.cmd_dispatch_base_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDispatchBaseKHR".as_ptr(),
            )));
        self.cmd_dispatch_base.set(
            self.cmd_dispatch_base
                .get()
                .or(self.cmd_dispatch_base_khr.get()),
        );
        self.enumerate_physical_device_groups
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEnumeratePhysicalDeviceGroups".as_ptr(),
            )));
        self.enumerate_physical_device_groups_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEnumeratePhysicalDeviceGroupsKHR".as_ptr(),
            )));
        self.enumerate_physical_device_groups.set(
            self.enumerate_physical_device_groups
                .get()
                .or(self.enumerate_physical_device_groups_khr.get()),
        );
        self.get_image_memory_requirements2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageMemoryRequirements2".as_ptr(),
            )));
        self.get_image_memory_requirements2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageMemoryRequirements2KHR".as_ptr(),
            )));
        self.get_image_memory_requirements2.set(
            self.get_image_memory_requirements2
                .get()
                .or(self.get_image_memory_requirements2_khr.get()),
        );
        self.get_buffer_memory_requirements2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferMemoryRequirements2".as_ptr(),
            )));
        self.get_buffer_memory_requirements2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferMemoryRequirements2KHR".as_ptr(),
            )));
        self.get_buffer_memory_requirements2.set(
            self.get_buffer_memory_requirements2
                .get()
                .or(self.get_buffer_memory_requirements2_khr.get()),
        );
        self.get_image_sparse_memory_requirements2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageSparseMemoryRequirements2".as_ptr(),
            )));
        self.get_image_sparse_memory_requirements2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageSparseMemoryRequirements2KHR".as_ptr(),
            )));
        self.get_image_sparse_memory_requirements2.set(
            self.get_image_sparse_memory_requirements2
                .get()
                .or(self.get_image_sparse_memory_requirements2_khr.get()),
        );
        self.get_physical_device_features2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFeatures2".as_ptr(),
            )));
        self.get_physical_device_features2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFeatures2KHR".as_ptr(),
            )));
        self.get_physical_device_features2.set(
            self.get_physical_device_features2
                .get()
                .or(self.get_physical_device_features2_khr.get()),
        );
        self.get_physical_device_properties2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceProperties2".as_ptr(),
            )));
        self.get_physical_device_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_properties2.set(
            self.get_physical_device_properties2
                .get()
                .or(self.get_physical_device_properties2_khr.get()),
        );
        self.get_physical_device_format_properties2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFormatProperties2".as_ptr(),
            )));
        self.get_physical_device_format_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFormatProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_format_properties2.set(
            self.get_physical_device_format_properties2
                .get()
                .or(self.get_physical_device_format_properties2_khr.get()),
        );
        self.get_physical_device_image_format_properties2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceImageFormatProperties2".as_ptr(),
            )));
        self.get_physical_device_image_format_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceImageFormatProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_image_format_properties2.set(
            self.get_physical_device_image_format_properties2
                .get()
                .or(self.get_physical_device_image_format_properties2_khr.get()),
        );
        self.get_physical_device_queue_family_properties2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceQueueFamilyProperties2".as_ptr(),
            )));
        self.get_physical_device_queue_family_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceQueueFamilyProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_queue_family_properties2.set(
            self.get_physical_device_queue_family_properties2
                .get()
                .or(self.get_physical_device_queue_family_properties2_khr.get()),
        );
        self.get_physical_device_memory_properties2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceMemoryProperties2".as_ptr(),
            )));
        self.get_physical_device_memory_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceMemoryProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_memory_properties2.set(
            self.get_physical_device_memory_properties2
                .get()
                .or(self.get_physical_device_memory_properties2_khr.get()),
        );
        self.get_physical_device_sparse_image_format_properties2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSparseImageFormatProperties2".as_ptr(),
            )));
        self.get_physical_device_sparse_image_format_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_sparse_image_format_properties2
            .set(
                self.get_physical_device_sparse_image_format_properties2
                    .get()
                    .or(self
                        .get_physical_device_sparse_image_format_properties2_khr
                        .get()),
            );
        self.trim_command_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkTrimCommandPool".as_ptr(),
            )));
        self.trim_command_pool_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkTrimCommandPoolKHR".as_ptr(),
            )));
        self.trim_command_pool.set(
            self.trim_command_pool
                .get()
                .or(self.trim_command_pool_khr.get()),
        );
        self.get_device_queue2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceQueue2".as_ptr(),
            )));
        self.create_sampler_ycbcr_conversion
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateSamplerYcbcrConversion".as_ptr(),
            )));
        self.create_sampler_ycbcr_conversion_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateSamplerYcbcrConversionKHR".as_ptr(),
            )));
        self.create_sampler_ycbcr_conversion.set(
            self.create_sampler_ycbcr_conversion
                .get()
                .or(self.create_sampler_ycbcr_conversion_khr.get()),
        );
        self.destroy_sampler_ycbcr_conversion
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroySamplerYcbcrConversion".as_ptr(),
            )));
        self.destroy_sampler_ycbcr_conversion_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroySamplerYcbcrConversionKHR".as_ptr(),
            )));
        self.destroy_sampler_ycbcr_conversion.set(
            self.destroy_sampler_ycbcr_conversion
                .get()
                .or(self.destroy_sampler_ycbcr_conversion_khr.get()),
        );
        self.create_descriptor_update_template
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDescriptorUpdateTemplate".as_ptr(),
            )));
        self.create_descriptor_update_template_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDescriptorUpdateTemplateKHR".as_ptr(),
            )));
        self.create_descriptor_update_template.set(
            self.create_descriptor_update_template
                .get()
                .or(self.create_descriptor_update_template_khr.get()),
        );
        self.destroy_descriptor_update_template
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDescriptorUpdateTemplate".as_ptr(),
            )));
        self.destroy_descriptor_update_template_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDescriptorUpdateTemplateKHR".as_ptr(),
            )));
        self.destroy_descriptor_update_template.set(
            self.destroy_descriptor_update_template
                .get()
                .or(self.destroy_descriptor_update_template_khr.get()),
        );
        self.update_descriptor_set_with_template
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkUpdateDescriptorSetWithTemplate".as_ptr(),
            )));
        self.update_descriptor_set_with_template_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkUpdateDescriptorSetWithTemplateKHR".as_ptr(),
            )));
        self.update_descriptor_set_with_template.set(
            self.update_descriptor_set_with_template
                .get()
                .or(self.update_descriptor_set_with_template_khr.get()),
        );
        self.get_physical_device_external_buffer_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalBufferProperties".as_ptr(),
            )));
        self.get_physical_device_external_buffer_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalBufferPropertiesKHR".as_ptr(),
            )));
        self.get_physical_device_external_buffer_properties.set(
            self.get_physical_device_external_buffer_properties
                .get()
                .or(self
                    .get_physical_device_external_buffer_properties_khr
                    .get()),
        );
        self.get_physical_device_external_fence_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalFenceProperties".as_ptr(),
            )));
        self.get_physical_device_external_fence_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalFencePropertiesKHR".as_ptr(),
            )));
        self.get_physical_device_external_fence_properties.set(
            self.get_physical_device_external_fence_properties
                .get()
                .or(self.get_physical_device_external_fence_properties_khr.get()),
        );
        self.get_physical_device_external_semaphore_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalSemaphoreProperties".as_ptr(),
            )));
        self.get_physical_device_external_semaphore_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR".as_ptr(),
            )));
        self.get_physical_device_external_semaphore_properties.set(
            self.get_physical_device_external_semaphore_properties
                .get()
                .or(self
                    .get_physical_device_external_semaphore_properties_khr
                    .get()),
        );
        self.get_descriptor_set_layout_support
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorSetLayoutSupport".as_ptr(),
            )));
        self.get_descriptor_set_layout_support_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorSetLayoutSupportKHR".as_ptr(),
            )));
        self.get_descriptor_set_layout_support.set(
            self.get_descriptor_set_layout_support
                .get()
                .or(self.get_descriptor_set_layout_support_khr.get()),
        );
        self.cmd_draw_indirect_count
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndirectCount".as_ptr(),
            )));
        self.cmd_draw_indirect_count_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndirectCountKHR".as_ptr(),
            )));
        self.cmd_draw_indirect_count.set(
            self.cmd_draw_indirect_count
                .get()
                .or(self.cmd_draw_indirect_count_khr.get()),
        );
        self.cmd_draw_indirect_count_amd
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndirectCountAMD".as_ptr(),
            )));
        self.cmd_draw_indirect_count.set(
            self.cmd_draw_indirect_count
                .get()
                .or(self.cmd_draw_indirect_count_amd.get()),
        );
        self.cmd_draw_indexed_indirect_count
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndexedIndirectCount".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect_count_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndexedIndirectCountKHR".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect_count.set(
            self.cmd_draw_indexed_indirect_count
                .get()
                .or(self.cmd_draw_indexed_indirect_count_khr.get()),
        );
        self.cmd_draw_indexed_indirect_count_amd
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndexedIndirectCountAMD".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect_count.set(
            self.cmd_draw_indexed_indirect_count
                .get()
                .or(self.cmd_draw_indexed_indirect_count_amd.get()),
        );
        self.create_render_pass2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateRenderPass2".as_ptr(),
            )));
        self.create_render_pass2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateRenderPass2KHR".as_ptr(),
            )));
        self.create_render_pass2.set(
            self.create_render_pass2
                .get()
                .or(self.create_render_pass2_khr.get()),
        );
        self.cmd_begin_render_pass2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginRenderPass2".as_ptr(),
            )));
        self.cmd_begin_render_pass2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginRenderPass2KHR".as_ptr(),
            )));
        self.cmd_begin_render_pass2.set(
            self.cmd_begin_render_pass2
                .get()
                .or(self.cmd_begin_render_pass2_khr.get()),
        );
        self.cmd_next_subpass2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdNextSubpass2".as_ptr(),
            )));
        self.cmd_next_subpass2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdNextSubpass2KHR".as_ptr(),
            )));
        self.cmd_next_subpass2.set(
            self.cmd_next_subpass2
                .get()
                .or(self.cmd_next_subpass2_khr.get()),
        );
        self.cmd_end_render_pass2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndRenderPass2".as_ptr(),
            )));
        self.cmd_end_render_pass2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndRenderPass2KHR".as_ptr(),
            )));
        self.cmd_end_render_pass2.set(
            self.cmd_end_render_pass2
                .get()
                .or(self.cmd_end_render_pass2_khr.get()),
        );
        self.reset_query_pool
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkResetQueryPool".as_ptr(),
            )));
        self.reset_query_pool_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkResetQueryPoolEXT".as_ptr(),
            )));
        self.reset_query_pool.set(
            self.reset_query_pool
                .get()
                .or(self.reset_query_pool_ext.get()),
        );
        self.get_semaphore_counter_value
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSemaphoreCounterValue".as_ptr(),
            )));
        self.get_semaphore_counter_value_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSemaphoreCounterValueKHR".as_ptr(),
            )));
        self.get_semaphore_counter_value.set(
            self.get_semaphore_counter_value
                .get()
                .or(self.get_semaphore_counter_value_khr.get()),
        );
        self.wait_semaphores
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkWaitSemaphores".as_ptr(),
            )));
        self.wait_semaphores_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkWaitSemaphoresKHR".as_ptr(),
            )));
        self.wait_semaphores.set(
            self.wait_semaphores
                .get()
                .or(self.wait_semaphores_khr.get()),
        );
        self.signal_semaphore
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSignalSemaphore".as_ptr(),
            )));
        self.signal_semaphore_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSignalSemaphoreKHR".as_ptr(),
            )));
        self.signal_semaphore.set(
            self.signal_semaphore
                .get()
                .or(self.signal_semaphore_khr.get()),
        );
        self.get_buffer_device_address
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferDeviceAddress".as_ptr(),
            )));
        self.get_buffer_device_address_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferDeviceAddressKHR".as_ptr(),
            )));
        self.get_buffer_device_address.set(
            self.get_buffer_device_address
                .get()
                .or(self.get_buffer_device_address_khr.get()),
        );
        self.get_buffer_device_address_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferDeviceAddressEXT".as_ptr(),
            )));
        self.get_buffer_device_address.set(
            self.get_buffer_device_address
                .get()
                .or(self.get_buffer_device_address_ext.get()),
        );
        self.get_buffer_opaque_capture_address
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferOpaqueCaptureAddress".as_ptr(),
            )));
        self.get_buffer_opaque_capture_address_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferOpaqueCaptureAddressKHR".as_ptr(),
            )));
        self.get_buffer_opaque_capture_address.set(
            self.get_buffer_opaque_capture_address
                .get()
                .or(self.get_buffer_opaque_capture_address_khr.get()),
        );
        self.get_device_memory_opaque_capture_address
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceMemoryOpaqueCaptureAddress".as_ptr(),
            )));
        self.get_device_memory_opaque_capture_address_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceMemoryOpaqueCaptureAddressKHR".as_ptr(),
            )));
        self.get_device_memory_opaque_capture_address.set(
            self.get_device_memory_opaque_capture_address
                .get()
                .or(self.get_device_memory_opaque_capture_address_khr.get()),
        );
        self.get_physical_device_tool_properties
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceToolProperties".as_ptr(),
            )));
        self.get_physical_device_tool_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceToolPropertiesEXT".as_ptr(),
            )));
        self.get_physical_device_tool_properties.set(
            self.get_physical_device_tool_properties
                .get()
                .or(self.get_physical_device_tool_properties_ext.get()),
        );
        self.create_private_data_slot
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreatePrivateDataSlot".as_ptr(),
            )));
        self.create_private_data_slot_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreatePrivateDataSlotEXT".as_ptr(),
            )));
        self.create_private_data_slot.set(
            self.create_private_data_slot
                .get()
                .or(self.create_private_data_slot_ext.get()),
        );
        self.destroy_private_data_slot
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyPrivateDataSlot".as_ptr(),
            )));
        self.destroy_private_data_slot_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyPrivateDataSlotEXT".as_ptr(),
            )));
        self.destroy_private_data_slot.set(
            self.destroy_private_data_slot
                .get()
                .or(self.destroy_private_data_slot_ext.get()),
        );
        self.set_private_data
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetPrivateData".as_ptr(),
            )));
        self.set_private_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetPrivateDataEXT".as_ptr(),
            )));
        self.set_private_data.set(
            self.set_private_data
                .get()
                .or(self.set_private_data_ext.get()),
        );
        self.get_private_data
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPrivateData".as_ptr(),
            )));
        self.get_private_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPrivateDataEXT".as_ptr(),
            )));
        self.get_private_data.set(
            self.get_private_data
                .get()
                .or(self.get_private_data_ext.get()),
        );
        self.cmd_set_event2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetEvent2".as_ptr(),
            )));
        self.cmd_set_event2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetEvent2KHR".as_ptr(),
            )));
        self.cmd_set_event2
            .set(self.cmd_set_event2.get().or(self.cmd_set_event2_khr.get()));
        self.cmd_reset_event2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResetEvent2".as_ptr(),
            )));
        self.cmd_reset_event2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResetEvent2KHR".as_ptr(),
            )));
        self.cmd_reset_event2.set(
            self.cmd_reset_event2
                .get()
                .or(self.cmd_reset_event2_khr.get()),
        );
        self.cmd_wait_events2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWaitEvents2".as_ptr(),
            )));
        self.cmd_wait_events2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWaitEvents2KHR".as_ptr(),
            )));
        self.cmd_wait_events2.set(
            self.cmd_wait_events2
                .get()
                .or(self.cmd_wait_events2_khr.get()),
        );
        self.cmd_pipeline_barrier2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPipelineBarrier2".as_ptr(),
            )));
        self.cmd_pipeline_barrier2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPipelineBarrier2KHR".as_ptr(),
            )));
        self.cmd_pipeline_barrier2.set(
            self.cmd_pipeline_barrier2
                .get()
                .or(self.cmd_pipeline_barrier2_khr.get()),
        );
        self.cmd_write_timestamp2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteTimestamp2".as_ptr(),
            )));
        self.cmd_write_timestamp2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteTimestamp2KHR".as_ptr(),
            )));
        self.cmd_write_timestamp2.set(
            self.cmd_write_timestamp2
                .get()
                .or(self.cmd_write_timestamp2_khr.get()),
        );
        self.queue_submit2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueSubmit2".as_ptr(),
            )));
        self.queue_submit2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueSubmit2KHR".as_ptr(),
            )));
        self.queue_submit2
            .set(self.queue_submit2.get().or(self.queue_submit2_khr.get()));
        self.cmd_copy_buffer2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyBuffer2".as_ptr(),
            )));
        self.cmd_copy_buffer2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyBuffer2KHR".as_ptr(),
            )));
        self.cmd_copy_buffer2.set(
            self.cmd_copy_buffer2
                .get()
                .or(self.cmd_copy_buffer2_khr.get()),
        );
        self.cmd_copy_image2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyImage2".as_ptr(),
            )));
        self.cmd_copy_image2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyImage2KHR".as_ptr(),
            )));
        self.cmd_copy_image2.set(
            self.cmd_copy_image2
                .get()
                .or(self.cmd_copy_image2_khr.get()),
        );
        self.cmd_copy_buffer_to_image2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyBufferToImage2".as_ptr(),
            )));
        self.cmd_copy_buffer_to_image2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyBufferToImage2KHR".as_ptr(),
            )));
        self.cmd_copy_buffer_to_image2.set(
            self.cmd_copy_buffer_to_image2
                .get()
                .or(self.cmd_copy_buffer_to_image2_khr.get()),
        );
        self.cmd_copy_image_to_buffer2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyImageToBuffer2".as_ptr(),
            )));
        self.cmd_copy_image_to_buffer2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyImageToBuffer2KHR".as_ptr(),
            )));
        self.cmd_copy_image_to_buffer2.set(
            self.cmd_copy_image_to_buffer2
                .get()
                .or(self.cmd_copy_image_to_buffer2_khr.get()),
        );
        self.cmd_blit_image2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBlitImage2".as_ptr(),
            )));
        self.cmd_blit_image2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBlitImage2KHR".as_ptr(),
            )));
        self.cmd_blit_image2.set(
            self.cmd_blit_image2
                .get()
                .or(self.cmd_blit_image2_khr.get()),
        );
        self.cmd_resolve_image2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResolveImage2".as_ptr(),
            )));
        self.cmd_resolve_image2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdResolveImage2KHR".as_ptr(),
            )));
        self.cmd_resolve_image2.set(
            self.cmd_resolve_image2
                .get()
                .or(self.cmd_resolve_image2_khr.get()),
        );
        self.cmd_begin_rendering
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginRendering".as_ptr(),
            )));
        self.cmd_begin_rendering_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginRenderingKHR".as_ptr(),
            )));
        self.cmd_begin_rendering.set(
            self.cmd_begin_rendering
                .get()
                .or(self.cmd_begin_rendering_khr.get()),
        );
        self.cmd_end_rendering
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndRendering".as_ptr(),
            )));
        self.cmd_end_rendering_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndRenderingKHR".as_ptr(),
            )));
        self.cmd_end_rendering.set(
            self.cmd_end_rendering
                .get()
                .or(self.cmd_end_rendering_khr.get()),
        );
        self.cmd_set_cull_mode
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCullMode".as_ptr(),
            )));
        self.cmd_set_cull_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCullModeEXT".as_ptr(),
            )));
        self.cmd_set_cull_mode.set(
            self.cmd_set_cull_mode
                .get()
                .or(self.cmd_set_cull_mode_ext.get()),
        );
        self.cmd_set_front_face
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetFrontFace".as_ptr(),
            )));
        self.cmd_set_front_face_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetFrontFaceEXT".as_ptr(),
            )));
        self.cmd_set_front_face.set(
            self.cmd_set_front_face
                .get()
                .or(self.cmd_set_front_face_ext.get()),
        );
        self.cmd_set_primitive_topology
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPrimitiveTopology".as_ptr(),
            )));
        self.cmd_set_primitive_topology_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPrimitiveTopologyEXT".as_ptr(),
            )));
        self.cmd_set_primitive_topology.set(
            self.cmd_set_primitive_topology
                .get()
                .or(self.cmd_set_primitive_topology_ext.get()),
        );
        self.cmd_set_viewport_with_count
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewportWithCount".as_ptr(),
            )));
        self.cmd_set_viewport_with_count_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewportWithCountEXT".as_ptr(),
            )));
        self.cmd_set_viewport_with_count.set(
            self.cmd_set_viewport_with_count
                .get()
                .or(self.cmd_set_viewport_with_count_ext.get()),
        );
        self.cmd_set_scissor_with_count
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetScissorWithCount".as_ptr(),
            )));
        self.cmd_set_scissor_with_count_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetScissorWithCountEXT".as_ptr(),
            )));
        self.cmd_set_scissor_with_count.set(
            self.cmd_set_scissor_with_count
                .get()
                .or(self.cmd_set_scissor_with_count_ext.get()),
        );
        self.cmd_bind_vertex_buffers2
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindVertexBuffers2".as_ptr(),
            )));
        self.cmd_bind_vertex_buffers2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindVertexBuffers2EXT".as_ptr(),
            )));
        self.cmd_bind_vertex_buffers2.set(
            self.cmd_bind_vertex_buffers2
                .get()
                .or(self.cmd_bind_vertex_buffers2_ext.get()),
        );
        self.cmd_set_depth_test_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthTestEnable".as_ptr(),
            )));
        self.cmd_set_depth_test_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthTestEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_test_enable.set(
            self.cmd_set_depth_test_enable
                .get()
                .or(self.cmd_set_depth_test_enable_ext.get()),
        );
        self.cmd_set_depth_write_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthWriteEnable".as_ptr(),
            )));
        self.cmd_set_depth_write_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthWriteEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_write_enable.set(
            self.cmd_set_depth_write_enable
                .get()
                .or(self.cmd_set_depth_write_enable_ext.get()),
        );
        self.cmd_set_depth_compare_op
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthCompareOp".as_ptr(),
            )));
        self.cmd_set_depth_compare_op_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthCompareOpEXT".as_ptr(),
            )));
        self.cmd_set_depth_compare_op.set(
            self.cmd_set_depth_compare_op
                .get()
                .or(self.cmd_set_depth_compare_op_ext.get()),
        );
        self.cmd_set_depth_bounds_test_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBoundsTestEnable".as_ptr(),
            )));
        self.cmd_set_depth_bounds_test_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBoundsTestEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_bounds_test_enable.set(
            self.cmd_set_depth_bounds_test_enable
                .get()
                .or(self.cmd_set_depth_bounds_test_enable_ext.get()),
        );
        self.cmd_set_stencil_test_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilTestEnable".as_ptr(),
            )));
        self.cmd_set_stencil_test_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilTestEnableEXT".as_ptr(),
            )));
        self.cmd_set_stencil_test_enable.set(
            self.cmd_set_stencil_test_enable
                .get()
                .or(self.cmd_set_stencil_test_enable_ext.get()),
        );
        self.cmd_set_stencil_op
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilOp".as_ptr(),
            )));
        self.cmd_set_stencil_op_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetStencilOpEXT".as_ptr(),
            )));
        self.cmd_set_stencil_op.set(
            self.cmd_set_stencil_op
                .get()
                .or(self.cmd_set_stencil_op_ext.get()),
        );
        self.cmd_set_rasterizer_discard_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRasterizerDiscardEnable".as_ptr(),
            )));
        self.cmd_set_rasterizer_discard_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRasterizerDiscardEnableEXT".as_ptr(),
            )));
        self.cmd_set_rasterizer_discard_enable.set(
            self.cmd_set_rasterizer_discard_enable
                .get()
                .or(self.cmd_set_rasterizer_discard_enable_ext.get()),
        );
        self.cmd_set_depth_bias_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBiasEnable".as_ptr(),
            )));
        self.cmd_set_depth_bias_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBiasEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_bias_enable.set(
            self.cmd_set_depth_bias_enable
                .get()
                .or(self.cmd_set_depth_bias_enable_ext.get()),
        );
        self.cmd_set_primitive_restart_enable
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPrimitiveRestartEnable".as_ptr(),
            )));
        self.cmd_set_primitive_restart_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPrimitiveRestartEnableEXT".as_ptr(),
            )));
        self.cmd_set_primitive_restart_enable.set(
            self.cmd_set_primitive_restart_enable
                .get()
                .or(self.cmd_set_primitive_restart_enable_ext.get()),
        );
        self.get_device_buffer_memory_requirements
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceBufferMemoryRequirements".as_ptr(),
            )));
        self.get_device_buffer_memory_requirements_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceBufferMemoryRequirementsKHR".as_ptr(),
            )));
        self.get_device_buffer_memory_requirements.set(
            self.get_device_buffer_memory_requirements
                .get()
                .or(self.get_device_buffer_memory_requirements_khr.get()),
        );
        self.get_device_image_memory_requirements
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceImageMemoryRequirements".as_ptr(),
            )));
        self.get_device_image_memory_requirements_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceImageMemoryRequirementsKHR".as_ptr(),
            )));
        self.get_device_image_memory_requirements.set(
            self.get_device_image_memory_requirements
                .get()
                .or(self.get_device_image_memory_requirements_khr.get()),
        );
        self.get_device_image_sparse_memory_requirements
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceImageSparseMemoryRequirements".as_ptr(),
            )));
        self.get_device_image_sparse_memory_requirements_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceImageSparseMemoryRequirementsKHR".as_ptr(),
            )));
        self.get_device_image_sparse_memory_requirements.set(
            self.get_device_image_sparse_memory_requirements
                .get()
                .or(self.get_device_image_sparse_memory_requirements_khr.get()),
        );
        self.destroy_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroySurfaceKHR".as_ptr(),
            )));
        self.get_physical_device_surface_support_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfaceSupportKHR".as_ptr(),
            )));
        self.get_physical_device_surface_capabilities_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR".as_ptr(),
            )));
        self.get_physical_device_surface_formats_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfaceFormatsKHR".as_ptr(),
            )));
        self.get_physical_device_surface_present_modes_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfacePresentModesKHR".as_ptr(),
            )));
        self.create_swapchain_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateSwapchainKHR".as_ptr(),
            )));
        self.destroy_swapchain_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroySwapchainKHR".as_ptr(),
            )));
        self.get_swapchain_images_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSwapchainImagesKHR".as_ptr(),
            )));
        self.acquire_next_image_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireNextImageKHR".as_ptr(),
            )));
        self.queue_present_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueuePresentKHR".as_ptr(),
            )));
        self.get_device_group_present_capabilities_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceGroupPresentCapabilitiesKHR".as_ptr(),
            )));
        self.get_device_group_surface_present_modes_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceGroupSurfacePresentModesKHR".as_ptr(),
            )));
        self.get_physical_device_present_rectangles_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDevicePresentRectanglesKHR".as_ptr(),
            )));
        self.acquire_next_image2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireNextImage2KHR".as_ptr(),
            )));
        self.get_physical_device_display_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceDisplayPropertiesKHR".as_ptr(),
            )));
        self.get_physical_device_display_plane_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR".as_ptr(),
            )));
        self.get_display_plane_supported_displays_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDisplayPlaneSupportedDisplaysKHR".as_ptr(),
            )));
        self.get_display_mode_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDisplayModePropertiesKHR".as_ptr(),
            )));
        self.create_display_mode_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDisplayModeKHR".as_ptr(),
            )));
        self.get_display_plane_capabilities_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDisplayPlaneCapabilitiesKHR".as_ptr(),
            )));
        self.create_display_plane_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDisplayPlaneSurfaceKHR".as_ptr(),
            )));
        self.create_shared_swapchains_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateSharedSwapchainsKHR".as_ptr(),
            )));
        self.create_xlib_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateXlibSurfaceKHR".as_ptr(),
            )));
        self.get_physical_device_xlib_presentation_support_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceXlibPresentationSupportKHR".as_ptr(),
            )));
        self.create_xcb_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateXcbSurfaceKHR".as_ptr(),
            )));
        self.get_physical_device_xcb_presentation_support_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceXcbPresentationSupportKHR".as_ptr(),
            )));
        self.create_wayland_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateWaylandSurfaceKHR".as_ptr(),
            )));
        self.get_physical_device_wayland_presentation_support_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceWaylandPresentationSupportKHR".as_ptr(),
            )));
        self.create_android_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateAndroidSurfaceKHR".as_ptr(),
            )));
        self.create_win32_surface_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateWin32SurfaceKHR".as_ptr(),
            )));
        self.get_physical_device_win32_presentation_support_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceWin32PresentationSupportKHR".as_ptr(),
            )));
        self.create_debug_report_callback_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDebugReportCallbackEXT".as_ptr(),
            )));
        self.destroy_debug_report_callback_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDebugReportCallbackEXT".as_ptr(),
            )));
        self.debug_report_message_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDebugReportMessageEXT".as_ptr(),
            )));
        self.debug_marker_set_object_tag_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDebugMarkerSetObjectTagEXT".as_ptr(),
            )));
        self.debug_marker_set_object_name_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDebugMarkerSetObjectNameEXT".as_ptr(),
            )));
        self.cmd_debug_marker_begin_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDebugMarkerBeginEXT".as_ptr(),
            )));
        self.cmd_debug_marker_end_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDebugMarkerEndEXT".as_ptr(),
            )));
        self.cmd_debug_marker_insert_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDebugMarkerInsertEXT".as_ptr(),
            )));
        self.cmd_bind_transform_feedback_buffers_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindTransformFeedbackBuffersEXT".as_ptr(),
            )));
        self.cmd_begin_transform_feedback_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginTransformFeedbackEXT".as_ptr(),
            )));
        self.cmd_end_transform_feedback_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndTransformFeedbackEXT".as_ptr(),
            )));
        self.cmd_begin_query_indexed_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginQueryIndexedEXT".as_ptr(),
            )));
        self.cmd_end_query_indexed_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndQueryIndexedEXT".as_ptr(),
            )));
        self.cmd_draw_indirect_byte_count_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawIndirectByteCountEXT".as_ptr(),
            )));
        self.create_cu_module_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateCuModuleNVX".as_ptr(),
            )));
        self.create_cu_function_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateCuFunctionNVX".as_ptr(),
            )));
        self.destroy_cu_module_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyCuModuleNVX".as_ptr(),
            )));
        self.destroy_cu_function_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyCuFunctionNVX".as_ptr(),
            )));
        self.cmd_cu_launch_kernel_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCuLaunchKernelNVX".as_ptr(),
            )));
        self.get_image_view_handle_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageViewHandleNVX".as_ptr(),
            )));
        self.get_image_view_address_nvx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageViewAddressNVX".as_ptr(),
            )));
        self.get_shader_info_amd
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetShaderInfoAMD".as_ptr(),
            )));
        self.create_stream_descriptor_surface_ggp
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateStreamDescriptorSurfaceGGP".as_ptr(),
            )));
        self.get_physical_device_external_image_format_properties_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV".as_ptr(),
            )));
        self.get_memory_win32_handle_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryWin32HandleNV".as_ptr(),
            )));
        self.create_vi_surface_nn
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateViSurfaceNN".as_ptr(),
            )));
        self.get_memory_win32_handle_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryWin32HandleKHR".as_ptr(),
            )));
        self.get_memory_win32_handle_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryWin32HandlePropertiesKHR".as_ptr(),
            )));
        self.get_memory_fd_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryFdKHR".as_ptr(),
            )));
        self.get_memory_fd_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryFdPropertiesKHR".as_ptr(),
            )));
        self.import_semaphore_win32_handle_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkImportSemaphoreWin32HandleKHR".as_ptr(),
            )));
        self.get_semaphore_win32_handle_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSemaphoreWin32HandleKHR".as_ptr(),
            )));
        self.import_semaphore_fd_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkImportSemaphoreFdKHR".as_ptr(),
            )));
        self.get_semaphore_fd_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSemaphoreFdKHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPushDescriptorSetKHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set_with_template_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPushDescriptorSetWithTemplateKHR".as_ptr(),
            )));
        self.cmd_begin_conditional_rendering_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginConditionalRenderingEXT".as_ptr(),
            )));
        self.cmd_end_conditional_rendering_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndConditionalRenderingEXT".as_ptr(),
            )));
        self.cmd_set_viewport_wscaling_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewportWScalingNV".as_ptr(),
            )));
        self.release_display_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkReleaseDisplayEXT".as_ptr(),
            )));
        self.acquire_xlib_display_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireXlibDisplayEXT".as_ptr(),
            )));
        self.get_rand_routput_display_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRandROutputDisplayEXT".as_ptr(),
            )));
        self.get_physical_device_surface_capabilities2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfaceCapabilities2EXT".as_ptr(),
            )));
        self.display_power_control_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDisplayPowerControlEXT".as_ptr(),
            )));
        self.register_device_event_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkRegisterDeviceEventEXT".as_ptr(),
            )));
        self.register_display_event_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkRegisterDisplayEventEXT".as_ptr(),
            )));
        self.get_swapchain_counter_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSwapchainCounterEXT".as_ptr(),
            )));
        self.get_refresh_cycle_duration_google
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRefreshCycleDurationGOOGLE".as_ptr(),
            )));
        self.get_past_presentation_timing_google
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPastPresentationTimingGOOGLE".as_ptr(),
            )));
        self.cmd_set_discard_rectangle_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDiscardRectangleEXT".as_ptr(),
            )));
        self.cmd_set_discard_rectangle_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDiscardRectangleEnableEXT".as_ptr(),
            )));
        self.cmd_set_discard_rectangle_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDiscardRectangleModeEXT".as_ptr(),
            )));
        self.set_hdr_metadata_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetHdrMetadataEXT".as_ptr(),
            )));
        self.get_swapchain_status_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSwapchainStatusKHR".as_ptr(),
            )));
        self.import_fence_win32_handle_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkImportFenceWin32HandleKHR".as_ptr(),
            )));
        self.get_fence_win32_handle_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetFenceWin32HandleKHR".as_ptr(),
            )));
        self.import_fence_fd_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkImportFenceFdKHR".as_ptr(),
            )));
        self.get_fence_fd_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetFenceFdKHR".as_ptr(),
            )));
        self.enumerate_physical_device_queue_family_performance_query_counters_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR".as_ptr(),
            )));
        self.get_physical_device_queue_family_performance_query_passes_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR".as_ptr(),
            )));
        self.acquire_profiling_lock_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireProfilingLockKHR".as_ptr(),
            )));
        self.release_profiling_lock_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkReleaseProfilingLockKHR".as_ptr(),
            )));
        self.get_physical_device_surface_capabilities2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfaceCapabilities2KHR".as_ptr(),
            )));
        self.get_physical_device_surface_formats2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfaceFormats2KHR".as_ptr(),
            )));
        self.get_physical_device_display_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceDisplayProperties2KHR".as_ptr(),
            )));
        self.get_physical_device_display_plane_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR".as_ptr(),
            )));
        self.get_display_mode_properties2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDisplayModeProperties2KHR".as_ptr(),
            )));
        self.get_display_plane_capabilities2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDisplayPlaneCapabilities2KHR".as_ptr(),
            )));
        self.create_iossurface_mvk
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateIOSSurfaceMVK".as_ptr(),
            )));
        self.create_mac_ossurface_mvk
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateMacOSSurfaceMVK".as_ptr(),
            )));
        self.set_debug_utils_object_name_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetDebugUtilsObjectNameEXT".as_ptr(),
            )));
        self.set_debug_utils_object_tag_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetDebugUtilsObjectTagEXT".as_ptr(),
            )));
        self.queue_begin_debug_utils_label_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueBeginDebugUtilsLabelEXT".as_ptr(),
            )));
        self.queue_end_debug_utils_label_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueEndDebugUtilsLabelEXT".as_ptr(),
            )));
        self.queue_insert_debug_utils_label_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueInsertDebugUtilsLabelEXT".as_ptr(),
            )));
        self.cmd_begin_debug_utils_label_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBeginDebugUtilsLabelEXT".as_ptr(),
            )));
        self.cmd_end_debug_utils_label_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdEndDebugUtilsLabelEXT".as_ptr(),
            )));
        self.cmd_insert_debug_utils_label_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdInsertDebugUtilsLabelEXT".as_ptr(),
            )));
        self.create_debug_utils_messenger_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDebugUtilsMessengerEXT".as_ptr(),
            )));
        self.destroy_debug_utils_messenger_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDebugUtilsMessengerEXT".as_ptr(),
            )));
        self.submit_debug_utils_message_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSubmitDebugUtilsMessageEXT".as_ptr(),
            )));
        self.get_android_hardware_buffer_properties_android
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetAndroidHardwareBufferPropertiesANDROID".as_ptr(),
            )));
        self.get_memory_android_hardware_buffer_android
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryAndroidHardwareBufferANDROID".as_ptr(),
            )));
        self.create_execution_graph_pipelines_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateExecutionGraphPipelinesAMDX".as_ptr(),
            )));
        self.get_execution_graph_pipeline_scratch_size_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetExecutionGraphPipelineScratchSizeAMDX".as_ptr(),
            )));
        self.get_execution_graph_pipeline_node_index_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetExecutionGraphPipelineNodeIndexAMDX".as_ptr(),
            )));
        self.cmd_initialize_graph_scratch_memory_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdInitializeGraphScratchMemoryAMDX".as_ptr(),
            )));
        self.cmd_dispatch_graph_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDispatchGraphAMDX".as_ptr(),
            )));
        self.cmd_dispatch_graph_indirect_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDispatchGraphIndirectAMDX".as_ptr(),
            )));
        self.cmd_dispatch_graph_indirect_count_amdx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDispatchGraphIndirectCountAMDX".as_ptr(),
            )));
        self.cmd_set_sample_locations_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetSampleLocationsEXT".as_ptr(),
            )));
        self.get_physical_device_multisample_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceMultisamplePropertiesEXT".as_ptr(),
            )));
        self.create_acceleration_structure_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateAccelerationStructureKHR".as_ptr(),
            )));
        self.destroy_acceleration_structure_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyAccelerationStructureKHR".as_ptr(),
            )));
        self.cmd_build_acceleration_structures_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBuildAccelerationStructuresKHR".as_ptr(),
            )));
        self.cmd_build_acceleration_structures_indirect_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBuildAccelerationStructuresIndirectKHR".as_ptr(),
            )));
        self.build_acceleration_structures_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBuildAccelerationStructuresKHR".as_ptr(),
            )));
        self.copy_acceleration_structure_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyAccelerationStructureKHR".as_ptr(),
            )));
        self.copy_acceleration_structure_to_memory_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyAccelerationStructureToMemoryKHR".as_ptr(),
            )));
        self.copy_memory_to_acceleration_structure_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyMemoryToAccelerationStructureKHR".as_ptr(),
            )));
        self.write_acceleration_structures_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkWriteAccelerationStructuresPropertiesKHR".as_ptr(),
            )));
        self.cmd_copy_acceleration_structure_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyAccelerationStructureKHR".as_ptr(),
            )));
        self.cmd_copy_acceleration_structure_to_memory_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyAccelerationStructureToMemoryKHR".as_ptr(),
            )));
        self.cmd_copy_memory_to_acceleration_structure_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyMemoryToAccelerationStructureKHR".as_ptr(),
            )));
        self.get_acceleration_structure_device_address_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetAccelerationStructureDeviceAddressKHR".as_ptr(),
            )));
        self.cmd_write_acceleration_structures_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteAccelerationStructuresPropertiesKHR".as_ptr(),
            )));
        self.get_device_acceleration_structure_compatibility_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceAccelerationStructureCompatibilityKHR".as_ptr(),
            )));
        self.get_acceleration_structure_build_sizes_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetAccelerationStructureBuildSizesKHR".as_ptr(),
            )));
        self.cmd_trace_rays_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdTraceRaysKHR".as_ptr(),
            )));
        self.create_ray_tracing_pipelines_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateRayTracingPipelinesKHR".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_handles_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRayTracingShaderGroupHandlesKHR".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_handles_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRayTracingShaderGroupHandlesNV".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_handles_khr.set(
            self.get_ray_tracing_shader_group_handles_khr
                .get()
                .or(self.get_ray_tracing_shader_group_handles_nv.get()),
        );
        self.get_ray_tracing_capture_replay_shader_group_handles_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR".as_ptr(),
            )));
        self.cmd_trace_rays_indirect_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdTraceRaysIndirectKHR".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_stack_size_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRayTracingShaderGroupStackSizeKHR".as_ptr(),
            )));
        self.cmd_set_ray_tracing_pipeline_stack_size_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRayTracingPipelineStackSizeKHR".as_ptr(),
            )));
        self.get_image_drm_format_modifier_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageDrmFormatModifierPropertiesEXT".as_ptr(),
            )));
        self.create_validation_cache_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateValidationCacheEXT".as_ptr(),
            )));
        self.destroy_validation_cache_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyValidationCacheEXT".as_ptr(),
            )));
        self.merge_validation_caches_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkMergeValidationCachesEXT".as_ptr(),
            )));
        self.get_validation_cache_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetValidationCacheDataEXT".as_ptr(),
            )));
        self.cmd_bind_shading_rate_image_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindShadingRateImageNV".as_ptr(),
            )));
        self.cmd_set_viewport_shading_rate_palette_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewportShadingRatePaletteNV".as_ptr(),
            )));
        self.cmd_set_coarse_sample_order_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoarseSampleOrderNV".as_ptr(),
            )));
        self.create_acceleration_structure_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateAccelerationStructureNV".as_ptr(),
            )));
        self.destroy_acceleration_structure_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyAccelerationStructureNV".as_ptr(),
            )));
        self.get_acceleration_structure_memory_requirements_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetAccelerationStructureMemoryRequirementsNV".as_ptr(),
            )));
        self.bind_acceleration_structure_memory_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindAccelerationStructureMemoryNV".as_ptr(),
            )));
        self.cmd_build_acceleration_structure_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBuildAccelerationStructureNV".as_ptr(),
            )));
        self.cmd_copy_acceleration_structure_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyAccelerationStructureNV".as_ptr(),
            )));
        self.cmd_trace_rays_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdTraceRaysNV".as_ptr(),
            )));
        self.create_ray_tracing_pipelines_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateRayTracingPipelinesNV".as_ptr(),
            )));
        self.get_acceleration_structure_handle_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetAccelerationStructureHandleNV".as_ptr(),
            )));
        self.cmd_write_acceleration_structures_properties_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteAccelerationStructuresPropertiesNV".as_ptr(),
            )));
        self.compile_deferred_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCompileDeferredNV".as_ptr(),
            )));
        self.get_memory_host_pointer_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryHostPointerPropertiesEXT".as_ptr(),
            )));
        self.cmd_write_buffer_marker_amd
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteBufferMarkerAMD".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMeshTasksNV".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMeshTasksIndirectNV".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_count_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMeshTasksIndirectCountNV".as_ptr(),
            )));
        self.cmd_set_exclusive_scissor_enable_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetExclusiveScissorEnableNV".as_ptr(),
            )));
        self.cmd_set_exclusive_scissor_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetExclusiveScissorNV".as_ptr(),
            )));
        self.cmd_set_checkpoint_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCheckpointNV".as_ptr(),
            )));
        self.get_queue_checkpoint_data_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetQueueCheckpointDataNV".as_ptr(),
            )));
        self.initialize_performance_api_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkInitializePerformanceApiINTEL".as_ptr(),
            )));
        self.uninitialize_performance_api_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkUninitializePerformanceApiINTEL".as_ptr(),
            )));
        self.cmd_set_performance_marker_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPerformanceMarkerINTEL".as_ptr(),
            )));
        self.cmd_set_performance_stream_marker_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPerformanceStreamMarkerINTEL".as_ptr(),
            )));
        self.cmd_set_performance_override_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPerformanceOverrideINTEL".as_ptr(),
            )));
        self.acquire_performance_configuration_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquirePerformanceConfigurationINTEL".as_ptr(),
            )));
        self.release_performance_configuration_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkReleasePerformanceConfigurationINTEL".as_ptr(),
            )));
        self.queue_set_performance_configuration_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueSetPerformanceConfigurationINTEL".as_ptr(),
            )));
        self.get_performance_parameter_intel
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPerformanceParameterINTEL".as_ptr(),
            )));
        self.set_local_dimming_amd
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetLocalDimmingAMD".as_ptr(),
            )));
        self.create_image_pipe_surface_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateImagePipeSurfaceFUCHSIA".as_ptr(),
            )));
        self.create_metal_surface_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateMetalSurfaceEXT".as_ptr(),
            )));
        self.get_physical_device_fragment_shading_rates_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceFragmentShadingRatesKHR".as_ptr(),
            )));
        self.cmd_set_fragment_shading_rate_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetFragmentShadingRateKHR".as_ptr(),
            )));
        self.cmd_set_rendering_attachment_locations_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRenderingAttachmentLocationsKHR".as_ptr(),
            )));
        self.cmd_set_rendering_input_attachment_indices_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRenderingInputAttachmentIndicesKHR".as_ptr(),
            )));
        self.wait_for_present_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkWaitForPresentKHR".as_ptr(),
            )));
        self.get_physical_device_cooperative_matrix_properties_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV".as_ptr(),
            )));
        self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV".as_ptr(),
            )));
        self.get_physical_device_surface_present_modes2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceSurfacePresentModes2EXT".as_ptr(),
            )));
        self.acquire_full_screen_exclusive_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireFullScreenExclusiveModeEXT".as_ptr(),
            )));
        self.release_full_screen_exclusive_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkReleaseFullScreenExclusiveModeEXT".as_ptr(),
            )));
        self.get_device_group_surface_present_modes2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceGroupSurfacePresentModes2EXT".as_ptr(),
            )));
        self.create_headless_surface_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateHeadlessSurfaceEXT".as_ptr(),
            )));
        self.create_deferred_operation_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDeferredOperationKHR".as_ptr(),
            )));
        self.destroy_deferred_operation_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyDeferredOperationKHR".as_ptr(),
            )));
        self.get_deferred_operation_max_concurrency_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeferredOperationMaxConcurrencyKHR".as_ptr(),
            )));
        self.get_deferred_operation_result_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeferredOperationResultKHR".as_ptr(),
            )));
        self.deferred_operation_join_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDeferredOperationJoinKHR".as_ptr(),
            )));
        self.get_pipeline_executable_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelineExecutablePropertiesKHR".as_ptr(),
            )));
        self.get_pipeline_executable_statistics_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelineExecutableStatisticsKHR".as_ptr(),
            )));
        self.get_pipeline_executable_internal_representations_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelineExecutableInternalRepresentationsKHR".as_ptr(),
            )));
        self.copy_memory_to_image_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyMemoryToImageEXT".as_ptr(),
            )));
        self.copy_image_to_memory_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyImageToMemoryEXT".as_ptr(),
            )));
        self.copy_image_to_image_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyImageToImageEXT".as_ptr(),
            )));
        self.transition_image_layout_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkTransitionImageLayoutEXT".as_ptr(),
            )));
        self.map_memory2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkMapMemory2KHR".as_ptr(),
            )));
        self.unmap_memory2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkUnmapMemory2KHR".as_ptr(),
            )));
        self.release_swapchain_images_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkReleaseSwapchainImagesEXT".as_ptr(),
            )));
        self.get_generated_commands_memory_requirements_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetGeneratedCommandsMemoryRequirementsNV".as_ptr(),
            )));
        self.cmd_preprocess_generated_commands_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPreprocessGeneratedCommandsNV".as_ptr(),
            )));
        self.cmd_execute_generated_commands_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdExecuteGeneratedCommandsNV".as_ptr(),
            )));
        self.cmd_bind_pipeline_shader_group_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindPipelineShaderGroupNV".as_ptr(),
            )));
        self.create_indirect_commands_layout_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateIndirectCommandsLayoutNV".as_ptr(),
            )));
        self.destroy_indirect_commands_layout_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyIndirectCommandsLayoutNV".as_ptr(),
            )));
        self.cmd_set_depth_bias2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthBias2EXT".as_ptr(),
            )));
        self.acquire_drm_display_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireDrmDisplayEXT".as_ptr(),
            )));
        self.get_drm_display_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDrmDisplayEXT".as_ptr(),
            )));
        self.create_cuda_module_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateCudaModuleNV".as_ptr(),
            )));
        self.get_cuda_module_cache_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetCudaModuleCacheNV".as_ptr(),
            )));
        self.create_cuda_function_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateCudaFunctionNV".as_ptr(),
            )));
        self.destroy_cuda_module_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyCudaModuleNV".as_ptr(),
            )));
        self.destroy_cuda_function_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyCudaFunctionNV".as_ptr(),
            )));
        self.cmd_cuda_launch_kernel_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCudaLaunchKernelNV".as_ptr(),
            )));
        self.export_metal_objects_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkExportMetalObjectsEXT".as_ptr(),
            )));
        self.cmd_write_buffer_marker2_amd
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteBufferMarker2AMD".as_ptr(),
            )));
        self.get_queue_checkpoint_data2_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetQueueCheckpointData2NV".as_ptr(),
            )));
        self.get_descriptor_set_layout_size_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorSetLayoutSizeEXT".as_ptr(),
            )));
        self.get_descriptor_set_layout_binding_offset_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorSetLayoutBindingOffsetEXT".as_ptr(),
            )));
        self.get_descriptor_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorEXT".as_ptr(),
            )));
        self.cmd_bind_descriptor_buffers_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindDescriptorBuffersEXT".as_ptr(),
            )));
        self.cmd_set_descriptor_buffer_offsets_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDescriptorBufferOffsetsEXT".as_ptr(),
            )));
        self.cmd_bind_descriptor_buffer_embedded_samplers_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT".as_ptr(),
            )));
        self.get_buffer_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_image_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_image_view_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageViewOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_sampler_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSamplerOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_acceleration_structure_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.cmd_set_fragment_shading_rate_enum_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetFragmentShadingRateEnumNV".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMeshTasksEXT".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMeshTasksIndirectEXT".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_count_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMeshTasksIndirectCountEXT".as_ptr(),
            )));
        self.get_device_fault_info_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceFaultInfoEXT".as_ptr(),
            )));
        self.acquire_winrt_display_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkAcquireWinrtDisplayNV".as_ptr(),
            )));
        self.get_winrt_display_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetWinrtDisplayNV".as_ptr(),
            )));
        self.create_direct_fbsurface_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateDirectFBSurfaceEXT".as_ptr(),
            )));
        self.get_physical_device_direct_fbpresentation_support_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT".as_ptr(),
            )));
        self.cmd_set_vertex_input_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetVertexInputEXT".as_ptr(),
            )));
        self.get_memory_zircon_handle_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryZirconHandleFUCHSIA".as_ptr(),
            )));
        self.get_memory_zircon_handle_properties_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryZirconHandlePropertiesFUCHSIA".as_ptr(),
            )));
        self.import_semaphore_zircon_handle_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkImportSemaphoreZirconHandleFUCHSIA".as_ptr(),
            )));
        self.get_semaphore_zircon_handle_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetSemaphoreZirconHandleFUCHSIA".as_ptr(),
            )));
        self.create_buffer_collection_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateBufferCollectionFUCHSIA".as_ptr(),
            )));
        self.set_buffer_collection_image_constraints_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetBufferCollectionImageConstraintsFUCHSIA".as_ptr(),
            )));
        self.set_buffer_collection_buffer_constraints_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetBufferCollectionBufferConstraintsFUCHSIA".as_ptr(),
            )));
        self.destroy_buffer_collection_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyBufferCollectionFUCHSIA".as_ptr(),
            )));
        self.get_buffer_collection_properties_fuchsia
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetBufferCollectionPropertiesFUCHSIA".as_ptr(),
            )));
        self.get_device_subpass_shading_max_workgroup_size_huawei
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI".as_ptr(),
            )));
        self.cmd_subpass_shading_huawei
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSubpassShadingHUAWEI".as_ptr(),
            )));
        self.cmd_bind_invocation_mask_huawei
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindInvocationMaskHUAWEI".as_ptr(),
            )));
        self.get_memory_remote_address_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMemoryRemoteAddressNV".as_ptr(),
            )));
        self.get_pipeline_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelinePropertiesEXT".as_ptr(),
            )));
        self.cmd_set_patch_control_points_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPatchControlPointsEXT".as_ptr(),
            )));
        self.cmd_set_logic_op_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLogicOpEXT".as_ptr(),
            )));
        self.create_screen_surface_qnx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateScreenSurfaceQNX".as_ptr(),
            )));
        self.get_physical_device_screen_presentation_support_qnx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceScreenPresentationSupportQNX".as_ptr(),
            )));
        self.cmd_set_color_write_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetColorWriteEnableEXT".as_ptr(),
            )));
        self.cmd_trace_rays_indirect2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdTraceRaysIndirect2KHR".as_ptr(),
            )));
        self.cmd_draw_multi_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMultiEXT".as_ptr(),
            )));
        self.cmd_draw_multi_indexed_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawMultiIndexedEXT".as_ptr(),
            )));
        self.create_micromap_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateMicromapEXT".as_ptr(),
            )));
        self.destroy_micromap_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyMicromapEXT".as_ptr(),
            )));
        self.cmd_build_micromaps_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBuildMicromapsEXT".as_ptr(),
            )));
        self.build_micromaps_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBuildMicromapsEXT".as_ptr(),
            )));
        self.copy_micromap_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyMicromapEXT".as_ptr(),
            )));
        self.copy_micromap_to_memory_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyMicromapToMemoryEXT".as_ptr(),
            )));
        self.copy_memory_to_micromap_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCopyMemoryToMicromapEXT".as_ptr(),
            )));
        self.write_micromaps_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkWriteMicromapsPropertiesEXT".as_ptr(),
            )));
        self.cmd_copy_micromap_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyMicromapEXT".as_ptr(),
            )));
        self.cmd_copy_micromap_to_memory_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyMicromapToMemoryEXT".as_ptr(),
            )));
        self.cmd_copy_memory_to_micromap_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyMemoryToMicromapEXT".as_ptr(),
            )));
        self.cmd_write_micromaps_properties_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdWriteMicromapsPropertiesEXT".as_ptr(),
            )));
        self.get_device_micromap_compatibility_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceMicromapCompatibilityEXT".as_ptr(),
            )));
        self.get_micromap_build_sizes_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetMicromapBuildSizesEXT".as_ptr(),
            )));
        self.cmd_draw_cluster_huawei
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawClusterHUAWEI".as_ptr(),
            )));
        self.cmd_draw_cluster_indirect_huawei
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDrawClusterIndirectHUAWEI".as_ptr(),
            )));
        self.set_device_memory_priority_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetDeviceMemoryPriorityEXT".as_ptr(),
            )));
        self.get_descriptor_set_layout_host_mapping_info_valve
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorSetLayoutHostMappingInfoVALVE".as_ptr(),
            )));
        self.get_descriptor_set_host_mapping_valve
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDescriptorSetHostMappingVALVE".as_ptr(),
            )));
        self.cmd_copy_memory_indirect_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyMemoryIndirectNV".as_ptr(),
            )));
        self.cmd_copy_memory_to_image_indirect_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdCopyMemoryToImageIndirectNV".as_ptr(),
            )));
        self.cmd_decompress_memory_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDecompressMemoryNV".as_ptr(),
            )));
        self.cmd_decompress_memory_indirect_count_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdDecompressMemoryIndirectCountNV".as_ptr(),
            )));
        self.get_pipeline_indirect_memory_requirements_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelineIndirectMemoryRequirementsNV".as_ptr(),
            )));
        self.cmd_update_pipeline_indirect_buffer_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdUpdatePipelineIndirectBufferNV".as_ptr(),
            )));
        self.get_pipeline_indirect_device_address_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPipelineIndirectDeviceAddressNV".as_ptr(),
            )));
        self.cmd_set_depth_clamp_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthClampEnableEXT".as_ptr(),
            )));
        self.cmd_set_polygon_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetPolygonModeEXT".as_ptr(),
            )));
        self.cmd_set_rasterization_samples_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRasterizationSamplesEXT".as_ptr(),
            )));
        self.cmd_set_sample_mask_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetSampleMaskEXT".as_ptr(),
            )));
        self.cmd_set_alpha_to_coverage_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetAlphaToCoverageEnableEXT".as_ptr(),
            )));
        self.cmd_set_alpha_to_one_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetAlphaToOneEnableEXT".as_ptr(),
            )));
        self.cmd_set_logic_op_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLogicOpEnableEXT".as_ptr(),
            )));
        self.cmd_set_color_blend_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetColorBlendEnableEXT".as_ptr(),
            )));
        self.cmd_set_color_blend_equation_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetColorBlendEquationEXT".as_ptr(),
            )));
        self.cmd_set_color_write_mask_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetColorWriteMaskEXT".as_ptr(),
            )));
        self.cmd_set_tessellation_domain_origin_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetTessellationDomainOriginEXT".as_ptr(),
            )));
        self.cmd_set_rasterization_stream_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRasterizationStreamEXT".as_ptr(),
            )));
        self.cmd_set_conservative_rasterization_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetConservativeRasterizationModeEXT".as_ptr(),
            )));
        self.cmd_set_extra_primitive_overestimation_size_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetExtraPrimitiveOverestimationSizeEXT".as_ptr(),
            )));
        self.cmd_set_depth_clip_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthClipEnableEXT".as_ptr(),
            )));
        self.cmd_set_sample_locations_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetSampleLocationsEnableEXT".as_ptr(),
            )));
        self.cmd_set_color_blend_advanced_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetColorBlendAdvancedEXT".as_ptr(),
            )));
        self.cmd_set_provoking_vertex_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetProvokingVertexModeEXT".as_ptr(),
            )));
        self.cmd_set_line_rasterization_mode_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLineRasterizationModeEXT".as_ptr(),
            )));
        self.cmd_set_line_stipple_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLineStippleEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_clip_negative_one_to_one_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDepthClipNegativeOneToOneEXT".as_ptr(),
            )));
        self.cmd_set_viewport_wscaling_enable_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewportWScalingEnableNV".as_ptr(),
            )));
        self.cmd_set_viewport_swizzle_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetViewportSwizzleNV".as_ptr(),
            )));
        self.cmd_set_coverage_to_color_enable_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoverageToColorEnableNV".as_ptr(),
            )));
        self.cmd_set_coverage_to_color_location_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoverageToColorLocationNV".as_ptr(),
            )));
        self.cmd_set_coverage_modulation_mode_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoverageModulationModeNV".as_ptr(),
            )));
        self.cmd_set_coverage_modulation_table_enable_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoverageModulationTableEnableNV".as_ptr(),
            )));
        self.cmd_set_coverage_modulation_table_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoverageModulationTableNV".as_ptr(),
            )));
        self.cmd_set_shading_rate_image_enable_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetShadingRateImageEnableNV".as_ptr(),
            )));
        self.cmd_set_representative_fragment_test_enable_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetRepresentativeFragmentTestEnableNV".as_ptr(),
            )));
        self.cmd_set_coverage_reduction_mode_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetCoverageReductionModeNV".as_ptr(),
            )));
        self.get_shader_module_identifier_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetShaderModuleIdentifierEXT".as_ptr(),
            )));
        self.get_shader_module_create_info_identifier_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetShaderModuleCreateInfoIdentifierEXT".as_ptr(),
            )));
        self.get_physical_device_optical_flow_image_formats_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV".as_ptr(),
            )));
        self.create_optical_flow_session_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateOpticalFlowSessionNV".as_ptr(),
            )));
        self.destroy_optical_flow_session_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyOpticalFlowSessionNV".as_ptr(),
            )));
        self.bind_optical_flow_session_image_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkBindOpticalFlowSessionImageNV".as_ptr(),
            )));
        self.cmd_optical_flow_execute_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdOpticalFlowExecuteNV".as_ptr(),
            )));
        self.cmd_bind_index_buffer2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindIndexBuffer2KHR".as_ptr(),
            )));
        self.get_rendering_area_granularity_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetRenderingAreaGranularityKHR".as_ptr(),
            )));
        self.get_device_image_subresource_layout_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDeviceImageSubresourceLayoutKHR".as_ptr(),
            )));
        self.get_image_subresource_layout2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageSubresourceLayout2KHR".as_ptr(),
            )));
        self.get_image_subresource_layout2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetImageSubresourceLayout2EXT".as_ptr(),
            )));
        self.get_image_subresource_layout2_khr.set(
            self.get_image_subresource_layout2_khr
                .get()
                .or(self.get_image_subresource_layout2_ext.get()),
        );
        self.create_shaders_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCreateShadersEXT".as_ptr(),
            )));
        self.destroy_shader_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkDestroyShaderEXT".as_ptr(),
            )));
        self.get_shader_binary_data_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetShaderBinaryDataEXT".as_ptr(),
            )));
        self.cmd_bind_shaders_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindShadersEXT".as_ptr(),
            )));
        self.get_framebuffer_tile_properties_qcom
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetFramebufferTilePropertiesQCOM".as_ptr(),
            )));
        self.get_dynamic_rendering_tile_properties_qcom
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetDynamicRenderingTilePropertiesQCOM".as_ptr(),
            )));
        self.set_latency_sleep_mode_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetLatencySleepModeNV".as_ptr(),
            )));
        self.latency_sleep_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkLatencySleepNV".as_ptr(),
            )));
        self.set_latency_marker_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkSetLatencyMarkerNV".as_ptr(),
            )));
        self.get_latency_timings_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetLatencyTimingsNV".as_ptr(),
            )));
        self.queue_notify_out_of_band_nv
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkQueueNotifyOutOfBandNV".as_ptr(),
            )));
        self.get_physical_device_cooperative_matrix_properties_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR".as_ptr(),
            )));
        self.cmd_set_attachment_feedback_loop_enable_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetAttachmentFeedbackLoopEnableEXT".as_ptr(),
            )));
        self.get_screen_buffer_properties_qnx
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetScreenBufferPropertiesQNX".as_ptr(),
            )));
        self.cmd_set_line_stipple_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLineStippleKHR".as_ptr(),
            )));
        self.cmd_set_line_stipple_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetLineStippleEXT".as_ptr(),
            )));
        self.cmd_set_line_stipple_khr.set(
            self.cmd_set_line_stipple_khr
                .get()
                .or(self.cmd_set_line_stipple_ext.get()),
        );
        self.get_physical_device_calibrateable_time_domains_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR".as_ptr(),
            )));
        self.get_physical_device_calibrateable_time_domains_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT".as_ptr(),
            )));
        self.get_physical_device_calibrateable_time_domains_khr.set(
            self.get_physical_device_calibrateable_time_domains_khr
                .get()
                .or(self
                    .get_physical_device_calibrateable_time_domains_ext
                    .get()),
        );
        self.get_calibrated_timestamps_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetCalibratedTimestampsKHR".as_ptr(),
            )));
        self.get_calibrated_timestamps_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkGetCalibratedTimestampsEXT".as_ptr(),
            )));
        self.get_calibrated_timestamps_khr.set(
            self.get_calibrated_timestamps_khr
                .get()
                .or(self.get_calibrated_timestamps_ext.get()),
        );
        self.cmd_bind_descriptor_sets2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindDescriptorSets2KHR".as_ptr(),
            )));
        self.cmd_push_constants2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPushConstants2KHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPushDescriptorSet2KHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set_with_template2_khr
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdPushDescriptorSetWithTemplate2KHR".as_ptr(),
            )));
        self.cmd_set_descriptor_buffer_offsets2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdSetDescriptorBufferOffsets2EXT".as_ptr(),
            )));
        self.cmd_bind_descriptor_buffer_embedded_samplers2_ext
            .set(mem::transmute(get_instance_proc_addr(
                get_instance(),
                c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT".as_ptr(),
            )));
    }
    pub unsafe fn load_device(&self, device: &Device) {
        let get_device_proc_addr = self
            .get_device_proc_addr
            .get()
            .expect("load_instance must be called before load_device");
        let get_device = || Some(device.clone());
        self.get_device_proc_addr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceProcAddr".as_ptr(),
            )));
        self.destroy_device.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkDestroyDevice".as_ptr(),
        )));
        self.get_device_queue
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceQueue".as_ptr(),
            )));
        self.queue_submit.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkQueueSubmit".as_ptr(),
        )));
        self.queue_wait_idle
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueWaitIdle".as_ptr(),
            )));
        self.device_wait_idle
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDeviceWaitIdle".as_ptr(),
            )));
        self.allocate_memory
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAllocateMemory".as_ptr(),
            )));
        self.free_memory.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkFreeMemory".as_ptr(),
        )));
        self.map_memory.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkMapMemory".as_ptr(),
        )));
        self.unmap_memory.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkUnmapMemory".as_ptr(),
        )));
        self.flush_mapped_memory_ranges
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkFlushMappedMemoryRanges".as_ptr(),
            )));
        self.invalidate_mapped_memory_ranges
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkInvalidateMappedMemoryRanges".as_ptr(),
            )));
        self.get_device_memory_commitment
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceMemoryCommitment".as_ptr(),
            )));
        self.bind_buffer_memory
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindBufferMemory".as_ptr(),
            )));
        self.bind_image_memory
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindImageMemory".as_ptr(),
            )));
        self.get_buffer_memory_requirements
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferMemoryRequirements".as_ptr(),
            )));
        self.get_image_memory_requirements
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageMemoryRequirements".as_ptr(),
            )));
        self.get_image_sparse_memory_requirements
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageSparseMemoryRequirements".as_ptr(),
            )));
        self.queue_bind_sparse
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueBindSparse".as_ptr(),
            )));
        self.create_fence.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCreateFence".as_ptr(),
        )));
        self.destroy_fence.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkDestroyFence".as_ptr(),
        )));
        self.reset_fences.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkResetFences".as_ptr(),
        )));
        self.get_fence_status
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetFenceStatus".as_ptr(),
            )));
        self.wait_for_fences
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkWaitForFences".as_ptr(),
            )));
        self.create_semaphore
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateSemaphore".as_ptr(),
            )));
        self.destroy_semaphore
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroySemaphore".as_ptr(),
            )));
        self.create_event.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCreateEvent".as_ptr(),
        )));
        self.destroy_event.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkDestroyEvent".as_ptr(),
        )));
        self.get_event_status
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetEventStatus".as_ptr(),
            )));
        self.set_event.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkSetEvent".as_ptr(),
        )));
        self.reset_event.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkResetEvent".as_ptr(),
        )));
        self.create_query_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateQueryPool".as_ptr(),
            )));
        self.destroy_query_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyQueryPool".as_ptr(),
            )));
        self.get_query_pool_results
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetQueryPoolResults".as_ptr(),
            )));
        self.create_buffer.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCreateBuffer".as_ptr(),
        )));
        self.destroy_buffer.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkDestroyBuffer".as_ptr(),
        )));
        self.create_buffer_view
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateBufferView".as_ptr(),
            )));
        self.destroy_buffer_view
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyBufferView".as_ptr(),
            )));
        self.create_image.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCreateImage".as_ptr(),
        )));
        self.destroy_image.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkDestroyImage".as_ptr(),
        )));
        self.get_image_subresource_layout
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageSubresourceLayout".as_ptr(),
            )));
        self.create_image_view
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateImageView".as_ptr(),
            )));
        self.destroy_image_view
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyImageView".as_ptr(),
            )));
        self.create_shader_module
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateShaderModule".as_ptr(),
            )));
        self.destroy_shader_module
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyShaderModule".as_ptr(),
            )));
        self.create_pipeline_cache
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreatePipelineCache".as_ptr(),
            )));
        self.destroy_pipeline_cache
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyPipelineCache".as_ptr(),
            )));
        self.get_pipeline_cache_data
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelineCacheData".as_ptr(),
            )));
        self.merge_pipeline_caches
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkMergePipelineCaches".as_ptr(),
            )));
        self.create_graphics_pipelines
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateGraphicsPipelines".as_ptr(),
            )));
        self.create_compute_pipelines
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateComputePipelines".as_ptr(),
            )));
        self.destroy_pipeline
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyPipeline".as_ptr(),
            )));
        self.create_pipeline_layout
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreatePipelineLayout".as_ptr(),
            )));
        self.destroy_pipeline_layout
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyPipelineLayout".as_ptr(),
            )));
        self.create_sampler.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCreateSampler".as_ptr(),
        )));
        self.destroy_sampler
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroySampler".as_ptr(),
            )));
        self.create_descriptor_set_layout
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateDescriptorSetLayout".as_ptr(),
            )));
        self.destroy_descriptor_set_layout
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyDescriptorSetLayout".as_ptr(),
            )));
        self.create_descriptor_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateDescriptorPool".as_ptr(),
            )));
        self.destroy_descriptor_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyDescriptorPool".as_ptr(),
            )));
        self.reset_descriptor_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkResetDescriptorPool".as_ptr(),
            )));
        self.allocate_descriptor_sets
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAllocateDescriptorSets".as_ptr(),
            )));
        self.free_descriptor_sets
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkFreeDescriptorSets".as_ptr(),
            )));
        self.update_descriptor_sets
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkUpdateDescriptorSets".as_ptr(),
            )));
        self.create_framebuffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateFramebuffer".as_ptr(),
            )));
        self.destroy_framebuffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyFramebuffer".as_ptr(),
            )));
        self.create_render_pass
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateRenderPass".as_ptr(),
            )));
        self.destroy_render_pass
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyRenderPass".as_ptr(),
            )));
        self.get_render_area_granularity
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRenderAreaGranularity".as_ptr(),
            )));
        self.create_command_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateCommandPool".as_ptr(),
            )));
        self.destroy_command_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyCommandPool".as_ptr(),
            )));
        self.reset_command_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkResetCommandPool".as_ptr(),
            )));
        self.allocate_command_buffers
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAllocateCommandBuffers".as_ptr(),
            )));
        self.free_command_buffers
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkFreeCommandBuffers".as_ptr(),
            )));
        self.begin_command_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBeginCommandBuffer".as_ptr(),
            )));
        self.end_command_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkEndCommandBuffer".as_ptr(),
            )));
        self.reset_command_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkResetCommandBuffer".as_ptr(),
            )));
        self.cmd_bind_pipeline
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindPipeline".as_ptr(),
            )));
        self.cmd_set_viewport
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewport".as_ptr(),
            )));
        self.cmd_set_scissor
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetScissor".as_ptr(),
            )));
        self.cmd_set_line_width
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLineWidth".as_ptr(),
            )));
        self.cmd_set_depth_bias
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBias".as_ptr(),
            )));
        self.cmd_set_blend_constants
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetBlendConstants".as_ptr(),
            )));
        self.cmd_set_depth_bounds
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBounds".as_ptr(),
            )));
        self.cmd_set_stencil_compare_mask
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilCompareMask".as_ptr(),
            )));
        self.cmd_set_stencil_write_mask
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilWriteMask".as_ptr(),
            )));
        self.cmd_set_stencil_reference
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilReference".as_ptr(),
            )));
        self.cmd_bind_descriptor_sets
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindDescriptorSets".as_ptr(),
            )));
        self.cmd_bind_index_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindIndexBuffer".as_ptr(),
            )));
        self.cmd_bind_vertex_buffers
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindVertexBuffers".as_ptr(),
            )));
        self.cmd_draw.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdDraw".as_ptr(),
        )));
        self.cmd_draw_indexed
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndexed".as_ptr(),
            )));
        self.cmd_draw_indirect
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndirect".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndexedIndirect".as_ptr(),
            )));
        self.cmd_dispatch.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdDispatch".as_ptr(),
        )));
        self.cmd_dispatch_indirect
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDispatchIndirect".as_ptr(),
            )));
        self.cmd_copy_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyBuffer".as_ptr(),
            )));
        self.cmd_copy_image.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdCopyImage".as_ptr(),
        )));
        self.cmd_blit_image.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdBlitImage".as_ptr(),
        )));
        self.cmd_copy_buffer_to_image
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyBufferToImage".as_ptr(),
            )));
        self.cmd_copy_image_to_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyImageToBuffer".as_ptr(),
            )));
        self.cmd_update_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdUpdateBuffer".as_ptr(),
            )));
        self.cmd_fill_buffer
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdFillBuffer".as_ptr(),
            )));
        self.cmd_clear_color_image
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdClearColorImage".as_ptr(),
            )));
        self.cmd_clear_depth_stencil_image
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdClearDepthStencilImage".as_ptr(),
            )));
        self.cmd_clear_attachments
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdClearAttachments".as_ptr(),
            )));
        self.cmd_resolve_image
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResolveImage".as_ptr(),
            )));
        self.cmd_set_event.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdSetEvent".as_ptr(),
        )));
        self.cmd_reset_event
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResetEvent".as_ptr(),
            )));
        self.cmd_wait_events
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWaitEvents".as_ptr(),
            )));
        self.cmd_pipeline_barrier
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPipelineBarrier".as_ptr(),
            )));
        self.cmd_begin_query
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginQuery".as_ptr(),
            )));
        self.cmd_end_query.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdEndQuery".as_ptr(),
        )));
        self.cmd_reset_query_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResetQueryPool".as_ptr(),
            )));
        self.cmd_write_timestamp
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteTimestamp".as_ptr(),
            )));
        self.cmd_copy_query_pool_results
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyQueryPoolResults".as_ptr(),
            )));
        self.cmd_push_constants
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPushConstants".as_ptr(),
            )));
        self.cmd_begin_render_pass
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginRenderPass".as_ptr(),
            )));
        self.cmd_next_subpass
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdNextSubpass".as_ptr(),
            )));
        self.cmd_end_render_pass
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndRenderPass".as_ptr(),
            )));
        self.cmd_execute_commands
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdExecuteCommands".as_ptr(),
            )));
        self.bind_buffer_memory2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindBufferMemory2".as_ptr(),
            )));
        self.bind_buffer_memory2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindBufferMemory2KHR".as_ptr(),
            )));
        self.bind_buffer_memory2.set(
            self.bind_buffer_memory2
                .get()
                .or(self.bind_buffer_memory2_khr.get()),
        );
        self.bind_image_memory2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindImageMemory2".as_ptr(),
            )));
        self.bind_image_memory2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindImageMemory2KHR".as_ptr(),
            )));
        self.bind_image_memory2.set(
            self.bind_image_memory2
                .get()
                .or(self.bind_image_memory2_khr.get()),
        );
        self.get_device_group_peer_memory_features
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceGroupPeerMemoryFeatures".as_ptr(),
            )));
        self.get_device_group_peer_memory_features_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceGroupPeerMemoryFeaturesKHR".as_ptr(),
            )));
        self.get_device_group_peer_memory_features.set(
            self.get_device_group_peer_memory_features
                .get()
                .or(self.get_device_group_peer_memory_features_khr.get()),
        );
        self.cmd_set_device_mask
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDeviceMask".as_ptr(),
            )));
        self.cmd_set_device_mask_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDeviceMaskKHR".as_ptr(),
            )));
        self.cmd_set_device_mask.set(
            self.cmd_set_device_mask
                .get()
                .or(self.cmd_set_device_mask_khr.get()),
        );
        self.cmd_dispatch_base
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDispatchBase".as_ptr(),
            )));
        self.cmd_dispatch_base_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDispatchBaseKHR".as_ptr(),
            )));
        self.cmd_dispatch_base.set(
            self.cmd_dispatch_base
                .get()
                .or(self.cmd_dispatch_base_khr.get()),
        );
        self.get_image_memory_requirements2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageMemoryRequirements2".as_ptr(),
            )));
        self.get_image_memory_requirements2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageMemoryRequirements2KHR".as_ptr(),
            )));
        self.get_image_memory_requirements2.set(
            self.get_image_memory_requirements2
                .get()
                .or(self.get_image_memory_requirements2_khr.get()),
        );
        self.get_buffer_memory_requirements2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferMemoryRequirements2".as_ptr(),
            )));
        self.get_buffer_memory_requirements2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferMemoryRequirements2KHR".as_ptr(),
            )));
        self.get_buffer_memory_requirements2.set(
            self.get_buffer_memory_requirements2
                .get()
                .or(self.get_buffer_memory_requirements2_khr.get()),
        );
        self.get_image_sparse_memory_requirements2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageSparseMemoryRequirements2".as_ptr(),
            )));
        self.get_image_sparse_memory_requirements2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageSparseMemoryRequirements2KHR".as_ptr(),
            )));
        self.get_image_sparse_memory_requirements2.set(
            self.get_image_sparse_memory_requirements2
                .get()
                .or(self.get_image_sparse_memory_requirements2_khr.get()),
        );
        self.trim_command_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkTrimCommandPool".as_ptr(),
            )));
        self.trim_command_pool_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkTrimCommandPoolKHR".as_ptr(),
            )));
        self.trim_command_pool.set(
            self.trim_command_pool
                .get()
                .or(self.trim_command_pool_khr.get()),
        );
        self.get_device_queue2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceQueue2".as_ptr(),
            )));
        self.create_sampler_ycbcr_conversion
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateSamplerYcbcrConversion".as_ptr(),
            )));
        self.create_sampler_ycbcr_conversion_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateSamplerYcbcrConversionKHR".as_ptr(),
            )));
        self.create_sampler_ycbcr_conversion.set(
            self.create_sampler_ycbcr_conversion
                .get()
                .or(self.create_sampler_ycbcr_conversion_khr.get()),
        );
        self.destroy_sampler_ycbcr_conversion
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroySamplerYcbcrConversion".as_ptr(),
            )));
        self.destroy_sampler_ycbcr_conversion_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroySamplerYcbcrConversionKHR".as_ptr(),
            )));
        self.destroy_sampler_ycbcr_conversion.set(
            self.destroy_sampler_ycbcr_conversion
                .get()
                .or(self.destroy_sampler_ycbcr_conversion_khr.get()),
        );
        self.create_descriptor_update_template
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateDescriptorUpdateTemplate".as_ptr(),
            )));
        self.create_descriptor_update_template_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateDescriptorUpdateTemplateKHR".as_ptr(),
            )));
        self.create_descriptor_update_template.set(
            self.create_descriptor_update_template
                .get()
                .or(self.create_descriptor_update_template_khr.get()),
        );
        self.destroy_descriptor_update_template
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyDescriptorUpdateTemplate".as_ptr(),
            )));
        self.destroy_descriptor_update_template_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyDescriptorUpdateTemplateKHR".as_ptr(),
            )));
        self.destroy_descriptor_update_template.set(
            self.destroy_descriptor_update_template
                .get()
                .or(self.destroy_descriptor_update_template_khr.get()),
        );
        self.update_descriptor_set_with_template
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkUpdateDescriptorSetWithTemplate".as_ptr(),
            )));
        self.update_descriptor_set_with_template_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkUpdateDescriptorSetWithTemplateKHR".as_ptr(),
            )));
        self.update_descriptor_set_with_template.set(
            self.update_descriptor_set_with_template
                .get()
                .or(self.update_descriptor_set_with_template_khr.get()),
        );
        self.get_descriptor_set_layout_support
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorSetLayoutSupport".as_ptr(),
            )));
        self.get_descriptor_set_layout_support_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorSetLayoutSupportKHR".as_ptr(),
            )));
        self.get_descriptor_set_layout_support.set(
            self.get_descriptor_set_layout_support
                .get()
                .or(self.get_descriptor_set_layout_support_khr.get()),
        );
        self.cmd_draw_indirect_count
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndirectCount".as_ptr(),
            )));
        self.cmd_draw_indirect_count_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndirectCountKHR".as_ptr(),
            )));
        self.cmd_draw_indirect_count.set(
            self.cmd_draw_indirect_count
                .get()
                .or(self.cmd_draw_indirect_count_khr.get()),
        );
        self.cmd_draw_indirect_count_amd
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndirectCountAMD".as_ptr(),
            )));
        self.cmd_draw_indirect_count.set(
            self.cmd_draw_indirect_count
                .get()
                .or(self.cmd_draw_indirect_count_amd.get()),
        );
        self.cmd_draw_indexed_indirect_count
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndexedIndirectCount".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect_count_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndexedIndirectCountKHR".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect_count.set(
            self.cmd_draw_indexed_indirect_count
                .get()
                .or(self.cmd_draw_indexed_indirect_count_khr.get()),
        );
        self.cmd_draw_indexed_indirect_count_amd
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndexedIndirectCountAMD".as_ptr(),
            )));
        self.cmd_draw_indexed_indirect_count.set(
            self.cmd_draw_indexed_indirect_count
                .get()
                .or(self.cmd_draw_indexed_indirect_count_amd.get()),
        );
        self.create_render_pass2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateRenderPass2".as_ptr(),
            )));
        self.create_render_pass2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateRenderPass2KHR".as_ptr(),
            )));
        self.create_render_pass2.set(
            self.create_render_pass2
                .get()
                .or(self.create_render_pass2_khr.get()),
        );
        self.cmd_begin_render_pass2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginRenderPass2".as_ptr(),
            )));
        self.cmd_begin_render_pass2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginRenderPass2KHR".as_ptr(),
            )));
        self.cmd_begin_render_pass2.set(
            self.cmd_begin_render_pass2
                .get()
                .or(self.cmd_begin_render_pass2_khr.get()),
        );
        self.cmd_next_subpass2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdNextSubpass2".as_ptr(),
            )));
        self.cmd_next_subpass2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdNextSubpass2KHR".as_ptr(),
            )));
        self.cmd_next_subpass2.set(
            self.cmd_next_subpass2
                .get()
                .or(self.cmd_next_subpass2_khr.get()),
        );
        self.cmd_end_render_pass2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndRenderPass2".as_ptr(),
            )));
        self.cmd_end_render_pass2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndRenderPass2KHR".as_ptr(),
            )));
        self.cmd_end_render_pass2.set(
            self.cmd_end_render_pass2
                .get()
                .or(self.cmd_end_render_pass2_khr.get()),
        );
        self.reset_query_pool
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkResetQueryPool".as_ptr(),
            )));
        self.reset_query_pool_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkResetQueryPoolEXT".as_ptr(),
            )));
        self.reset_query_pool.set(
            self.reset_query_pool
                .get()
                .or(self.reset_query_pool_ext.get()),
        );
        self.get_semaphore_counter_value
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSemaphoreCounterValue".as_ptr(),
            )));
        self.get_semaphore_counter_value_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSemaphoreCounterValueKHR".as_ptr(),
            )));
        self.get_semaphore_counter_value.set(
            self.get_semaphore_counter_value
                .get()
                .or(self.get_semaphore_counter_value_khr.get()),
        );
        self.wait_semaphores
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkWaitSemaphores".as_ptr(),
            )));
        self.wait_semaphores_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkWaitSemaphoresKHR".as_ptr(),
            )));
        self.wait_semaphores.set(
            self.wait_semaphores
                .get()
                .or(self.wait_semaphores_khr.get()),
        );
        self.signal_semaphore
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSignalSemaphore".as_ptr(),
            )));
        self.signal_semaphore_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSignalSemaphoreKHR".as_ptr(),
            )));
        self.signal_semaphore.set(
            self.signal_semaphore
                .get()
                .or(self.signal_semaphore_khr.get()),
        );
        self.get_buffer_device_address
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferDeviceAddress".as_ptr(),
            )));
        self.get_buffer_device_address_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferDeviceAddressKHR".as_ptr(),
            )));
        self.get_buffer_device_address.set(
            self.get_buffer_device_address
                .get()
                .or(self.get_buffer_device_address_khr.get()),
        );
        self.get_buffer_device_address_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferDeviceAddressEXT".as_ptr(),
            )));
        self.get_buffer_device_address.set(
            self.get_buffer_device_address
                .get()
                .or(self.get_buffer_device_address_ext.get()),
        );
        self.get_buffer_opaque_capture_address
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferOpaqueCaptureAddress".as_ptr(),
            )));
        self.get_buffer_opaque_capture_address_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferOpaqueCaptureAddressKHR".as_ptr(),
            )));
        self.get_buffer_opaque_capture_address.set(
            self.get_buffer_opaque_capture_address
                .get()
                .or(self.get_buffer_opaque_capture_address_khr.get()),
        );
        self.get_device_memory_opaque_capture_address
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceMemoryOpaqueCaptureAddress".as_ptr(),
            )));
        self.get_device_memory_opaque_capture_address_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceMemoryOpaqueCaptureAddressKHR".as_ptr(),
            )));
        self.get_device_memory_opaque_capture_address.set(
            self.get_device_memory_opaque_capture_address
                .get()
                .or(self.get_device_memory_opaque_capture_address_khr.get()),
        );
        self.create_private_data_slot
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreatePrivateDataSlot".as_ptr(),
            )));
        self.create_private_data_slot_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreatePrivateDataSlotEXT".as_ptr(),
            )));
        self.create_private_data_slot.set(
            self.create_private_data_slot
                .get()
                .or(self.create_private_data_slot_ext.get()),
        );
        self.destroy_private_data_slot
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyPrivateDataSlot".as_ptr(),
            )));
        self.destroy_private_data_slot_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyPrivateDataSlotEXT".as_ptr(),
            )));
        self.destroy_private_data_slot.set(
            self.destroy_private_data_slot
                .get()
                .or(self.destroy_private_data_slot_ext.get()),
        );
        self.set_private_data
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetPrivateData".as_ptr(),
            )));
        self.set_private_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetPrivateDataEXT".as_ptr(),
            )));
        self.set_private_data.set(
            self.set_private_data
                .get()
                .or(self.set_private_data_ext.get()),
        );
        self.get_private_data
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPrivateData".as_ptr(),
            )));
        self.get_private_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPrivateDataEXT".as_ptr(),
            )));
        self.get_private_data.set(
            self.get_private_data
                .get()
                .or(self.get_private_data_ext.get()),
        );
        self.cmd_set_event2.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkCmdSetEvent2".as_ptr(),
        )));
        self.cmd_set_event2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetEvent2KHR".as_ptr(),
            )));
        self.cmd_set_event2
            .set(self.cmd_set_event2.get().or(self.cmd_set_event2_khr.get()));
        self.cmd_reset_event2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResetEvent2".as_ptr(),
            )));
        self.cmd_reset_event2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResetEvent2KHR".as_ptr(),
            )));
        self.cmd_reset_event2.set(
            self.cmd_reset_event2
                .get()
                .or(self.cmd_reset_event2_khr.get()),
        );
        self.cmd_wait_events2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWaitEvents2".as_ptr(),
            )));
        self.cmd_wait_events2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWaitEvents2KHR".as_ptr(),
            )));
        self.cmd_wait_events2.set(
            self.cmd_wait_events2
                .get()
                .or(self.cmd_wait_events2_khr.get()),
        );
        self.cmd_pipeline_barrier2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPipelineBarrier2".as_ptr(),
            )));
        self.cmd_pipeline_barrier2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPipelineBarrier2KHR".as_ptr(),
            )));
        self.cmd_pipeline_barrier2.set(
            self.cmd_pipeline_barrier2
                .get()
                .or(self.cmd_pipeline_barrier2_khr.get()),
        );
        self.cmd_write_timestamp2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteTimestamp2".as_ptr(),
            )));
        self.cmd_write_timestamp2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteTimestamp2KHR".as_ptr(),
            )));
        self.cmd_write_timestamp2.set(
            self.cmd_write_timestamp2
                .get()
                .or(self.cmd_write_timestamp2_khr.get()),
        );
        self.queue_submit2.set(mem::transmute(get_device_proc_addr(
            get_device(),
            c"vkQueueSubmit2".as_ptr(),
        )));
        self.queue_submit2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueSubmit2KHR".as_ptr(),
            )));
        self.queue_submit2
            .set(self.queue_submit2.get().or(self.queue_submit2_khr.get()));
        self.cmd_copy_buffer2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyBuffer2".as_ptr(),
            )));
        self.cmd_copy_buffer2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyBuffer2KHR".as_ptr(),
            )));
        self.cmd_copy_buffer2.set(
            self.cmd_copy_buffer2
                .get()
                .or(self.cmd_copy_buffer2_khr.get()),
        );
        self.cmd_copy_image2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyImage2".as_ptr(),
            )));
        self.cmd_copy_image2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyImage2KHR".as_ptr(),
            )));
        self.cmd_copy_image2.set(
            self.cmd_copy_image2
                .get()
                .or(self.cmd_copy_image2_khr.get()),
        );
        self.cmd_copy_buffer_to_image2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyBufferToImage2".as_ptr(),
            )));
        self.cmd_copy_buffer_to_image2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyBufferToImage2KHR".as_ptr(),
            )));
        self.cmd_copy_buffer_to_image2.set(
            self.cmd_copy_buffer_to_image2
                .get()
                .or(self.cmd_copy_buffer_to_image2_khr.get()),
        );
        self.cmd_copy_image_to_buffer2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyImageToBuffer2".as_ptr(),
            )));
        self.cmd_copy_image_to_buffer2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyImageToBuffer2KHR".as_ptr(),
            )));
        self.cmd_copy_image_to_buffer2.set(
            self.cmd_copy_image_to_buffer2
                .get()
                .or(self.cmd_copy_image_to_buffer2_khr.get()),
        );
        self.cmd_blit_image2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBlitImage2".as_ptr(),
            )));
        self.cmd_blit_image2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBlitImage2KHR".as_ptr(),
            )));
        self.cmd_blit_image2.set(
            self.cmd_blit_image2
                .get()
                .or(self.cmd_blit_image2_khr.get()),
        );
        self.cmd_resolve_image2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResolveImage2".as_ptr(),
            )));
        self.cmd_resolve_image2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdResolveImage2KHR".as_ptr(),
            )));
        self.cmd_resolve_image2.set(
            self.cmd_resolve_image2
                .get()
                .or(self.cmd_resolve_image2_khr.get()),
        );
        self.cmd_begin_rendering
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginRendering".as_ptr(),
            )));
        self.cmd_begin_rendering_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginRenderingKHR".as_ptr(),
            )));
        self.cmd_begin_rendering.set(
            self.cmd_begin_rendering
                .get()
                .or(self.cmd_begin_rendering_khr.get()),
        );
        self.cmd_end_rendering
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndRendering".as_ptr(),
            )));
        self.cmd_end_rendering_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndRenderingKHR".as_ptr(),
            )));
        self.cmd_end_rendering.set(
            self.cmd_end_rendering
                .get()
                .or(self.cmd_end_rendering_khr.get()),
        );
        self.cmd_set_cull_mode
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCullMode".as_ptr(),
            )));
        self.cmd_set_cull_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCullModeEXT".as_ptr(),
            )));
        self.cmd_set_cull_mode.set(
            self.cmd_set_cull_mode
                .get()
                .or(self.cmd_set_cull_mode_ext.get()),
        );
        self.cmd_set_front_face
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetFrontFace".as_ptr(),
            )));
        self.cmd_set_front_face_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetFrontFaceEXT".as_ptr(),
            )));
        self.cmd_set_front_face.set(
            self.cmd_set_front_face
                .get()
                .or(self.cmd_set_front_face_ext.get()),
        );
        self.cmd_set_primitive_topology
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPrimitiveTopology".as_ptr(),
            )));
        self.cmd_set_primitive_topology_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPrimitiveTopologyEXT".as_ptr(),
            )));
        self.cmd_set_primitive_topology.set(
            self.cmd_set_primitive_topology
                .get()
                .or(self.cmd_set_primitive_topology_ext.get()),
        );
        self.cmd_set_viewport_with_count
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewportWithCount".as_ptr(),
            )));
        self.cmd_set_viewport_with_count_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewportWithCountEXT".as_ptr(),
            )));
        self.cmd_set_viewport_with_count.set(
            self.cmd_set_viewport_with_count
                .get()
                .or(self.cmd_set_viewport_with_count_ext.get()),
        );
        self.cmd_set_scissor_with_count
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetScissorWithCount".as_ptr(),
            )));
        self.cmd_set_scissor_with_count_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetScissorWithCountEXT".as_ptr(),
            )));
        self.cmd_set_scissor_with_count.set(
            self.cmd_set_scissor_with_count
                .get()
                .or(self.cmd_set_scissor_with_count_ext.get()),
        );
        self.cmd_bind_vertex_buffers2
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindVertexBuffers2".as_ptr(),
            )));
        self.cmd_bind_vertex_buffers2_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindVertexBuffers2EXT".as_ptr(),
            )));
        self.cmd_bind_vertex_buffers2.set(
            self.cmd_bind_vertex_buffers2
                .get()
                .or(self.cmd_bind_vertex_buffers2_ext.get()),
        );
        self.cmd_set_depth_test_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthTestEnable".as_ptr(),
            )));
        self.cmd_set_depth_test_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthTestEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_test_enable.set(
            self.cmd_set_depth_test_enable
                .get()
                .or(self.cmd_set_depth_test_enable_ext.get()),
        );
        self.cmd_set_depth_write_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthWriteEnable".as_ptr(),
            )));
        self.cmd_set_depth_write_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthWriteEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_write_enable.set(
            self.cmd_set_depth_write_enable
                .get()
                .or(self.cmd_set_depth_write_enable_ext.get()),
        );
        self.cmd_set_depth_compare_op
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthCompareOp".as_ptr(),
            )));
        self.cmd_set_depth_compare_op_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthCompareOpEXT".as_ptr(),
            )));
        self.cmd_set_depth_compare_op.set(
            self.cmd_set_depth_compare_op
                .get()
                .or(self.cmd_set_depth_compare_op_ext.get()),
        );
        self.cmd_set_depth_bounds_test_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBoundsTestEnable".as_ptr(),
            )));
        self.cmd_set_depth_bounds_test_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBoundsTestEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_bounds_test_enable.set(
            self.cmd_set_depth_bounds_test_enable
                .get()
                .or(self.cmd_set_depth_bounds_test_enable_ext.get()),
        );
        self.cmd_set_stencil_test_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilTestEnable".as_ptr(),
            )));
        self.cmd_set_stencil_test_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilTestEnableEXT".as_ptr(),
            )));
        self.cmd_set_stencil_test_enable.set(
            self.cmd_set_stencil_test_enable
                .get()
                .or(self.cmd_set_stencil_test_enable_ext.get()),
        );
        self.cmd_set_stencil_op
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilOp".as_ptr(),
            )));
        self.cmd_set_stencil_op_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetStencilOpEXT".as_ptr(),
            )));
        self.cmd_set_stencil_op.set(
            self.cmd_set_stencil_op
                .get()
                .or(self.cmd_set_stencil_op_ext.get()),
        );
        self.cmd_set_rasterizer_discard_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRasterizerDiscardEnable".as_ptr(),
            )));
        self.cmd_set_rasterizer_discard_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRasterizerDiscardEnableEXT".as_ptr(),
            )));
        self.cmd_set_rasterizer_discard_enable.set(
            self.cmd_set_rasterizer_discard_enable
                .get()
                .or(self.cmd_set_rasterizer_discard_enable_ext.get()),
        );
        self.cmd_set_depth_bias_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBiasEnable".as_ptr(),
            )));
        self.cmd_set_depth_bias_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBiasEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_bias_enable.set(
            self.cmd_set_depth_bias_enable
                .get()
                .or(self.cmd_set_depth_bias_enable_ext.get()),
        );
        self.cmd_set_primitive_restart_enable
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPrimitiveRestartEnable".as_ptr(),
            )));
        self.cmd_set_primitive_restart_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPrimitiveRestartEnableEXT".as_ptr(),
            )));
        self.cmd_set_primitive_restart_enable.set(
            self.cmd_set_primitive_restart_enable
                .get()
                .or(self.cmd_set_primitive_restart_enable_ext.get()),
        );
        self.get_device_buffer_memory_requirements
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceBufferMemoryRequirements".as_ptr(),
            )));
        self.get_device_buffer_memory_requirements_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceBufferMemoryRequirementsKHR".as_ptr(),
            )));
        self.get_device_buffer_memory_requirements.set(
            self.get_device_buffer_memory_requirements
                .get()
                .or(self.get_device_buffer_memory_requirements_khr.get()),
        );
        self.get_device_image_memory_requirements
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceImageMemoryRequirements".as_ptr(),
            )));
        self.get_device_image_memory_requirements_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceImageMemoryRequirementsKHR".as_ptr(),
            )));
        self.get_device_image_memory_requirements.set(
            self.get_device_image_memory_requirements
                .get()
                .or(self.get_device_image_memory_requirements_khr.get()),
        );
        self.get_device_image_sparse_memory_requirements
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceImageSparseMemoryRequirements".as_ptr(),
            )));
        self.get_device_image_sparse_memory_requirements_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceImageSparseMemoryRequirementsKHR".as_ptr(),
            )));
        self.get_device_image_sparse_memory_requirements.set(
            self.get_device_image_sparse_memory_requirements
                .get()
                .or(self.get_device_image_sparse_memory_requirements_khr.get()),
        );
        self.create_swapchain_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateSwapchainKHR".as_ptr(),
            )));
        self.destroy_swapchain_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroySwapchainKHR".as_ptr(),
            )));
        self.get_swapchain_images_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSwapchainImagesKHR".as_ptr(),
            )));
        self.acquire_next_image_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAcquireNextImageKHR".as_ptr(),
            )));
        self.queue_present_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueuePresentKHR".as_ptr(),
            )));
        self.get_device_group_present_capabilities_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceGroupPresentCapabilitiesKHR".as_ptr(),
            )));
        self.get_device_group_surface_present_modes_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceGroupSurfacePresentModesKHR".as_ptr(),
            )));
        self.acquire_next_image2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAcquireNextImage2KHR".as_ptr(),
            )));
        self.create_shared_swapchains_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateSharedSwapchainsKHR".as_ptr(),
            )));
        self.debug_marker_set_object_tag_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDebugMarkerSetObjectTagEXT".as_ptr(),
            )));
        self.debug_marker_set_object_name_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDebugMarkerSetObjectNameEXT".as_ptr(),
            )));
        self.cmd_debug_marker_begin_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDebugMarkerBeginEXT".as_ptr(),
            )));
        self.cmd_debug_marker_end_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDebugMarkerEndEXT".as_ptr(),
            )));
        self.cmd_debug_marker_insert_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDebugMarkerInsertEXT".as_ptr(),
            )));
        self.cmd_bind_transform_feedback_buffers_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindTransformFeedbackBuffersEXT".as_ptr(),
            )));
        self.cmd_begin_transform_feedback_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginTransformFeedbackEXT".as_ptr(),
            )));
        self.cmd_end_transform_feedback_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndTransformFeedbackEXT".as_ptr(),
            )));
        self.cmd_begin_query_indexed_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginQueryIndexedEXT".as_ptr(),
            )));
        self.cmd_end_query_indexed_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndQueryIndexedEXT".as_ptr(),
            )));
        self.cmd_draw_indirect_byte_count_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawIndirectByteCountEXT".as_ptr(),
            )));
        self.create_cu_module_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateCuModuleNVX".as_ptr(),
            )));
        self.create_cu_function_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateCuFunctionNVX".as_ptr(),
            )));
        self.destroy_cu_module_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyCuModuleNVX".as_ptr(),
            )));
        self.destroy_cu_function_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyCuFunctionNVX".as_ptr(),
            )));
        self.cmd_cu_launch_kernel_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCuLaunchKernelNVX".as_ptr(),
            )));
        self.get_image_view_handle_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageViewHandleNVX".as_ptr(),
            )));
        self.get_image_view_address_nvx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageViewAddressNVX".as_ptr(),
            )));
        self.get_shader_info_amd
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetShaderInfoAMD".as_ptr(),
            )));
        self.get_memory_win32_handle_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryWin32HandleNV".as_ptr(),
            )));
        self.get_memory_win32_handle_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryWin32HandleKHR".as_ptr(),
            )));
        self.get_memory_win32_handle_properties_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryWin32HandlePropertiesKHR".as_ptr(),
            )));
        self.get_memory_fd_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryFdKHR".as_ptr(),
            )));
        self.get_memory_fd_properties_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryFdPropertiesKHR".as_ptr(),
            )));
        self.import_semaphore_win32_handle_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkImportSemaphoreWin32HandleKHR".as_ptr(),
            )));
        self.get_semaphore_win32_handle_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSemaphoreWin32HandleKHR".as_ptr(),
            )));
        self.import_semaphore_fd_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkImportSemaphoreFdKHR".as_ptr(),
            )));
        self.get_semaphore_fd_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSemaphoreFdKHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPushDescriptorSetKHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set_with_template_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPushDescriptorSetWithTemplateKHR".as_ptr(),
            )));
        self.cmd_begin_conditional_rendering_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginConditionalRenderingEXT".as_ptr(),
            )));
        self.cmd_end_conditional_rendering_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndConditionalRenderingEXT".as_ptr(),
            )));
        self.cmd_set_viewport_wscaling_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewportWScalingNV".as_ptr(),
            )));
        self.display_power_control_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDisplayPowerControlEXT".as_ptr(),
            )));
        self.register_device_event_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkRegisterDeviceEventEXT".as_ptr(),
            )));
        self.register_display_event_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkRegisterDisplayEventEXT".as_ptr(),
            )));
        self.get_swapchain_counter_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSwapchainCounterEXT".as_ptr(),
            )));
        self.get_refresh_cycle_duration_google
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRefreshCycleDurationGOOGLE".as_ptr(),
            )));
        self.get_past_presentation_timing_google
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPastPresentationTimingGOOGLE".as_ptr(),
            )));
        self.cmd_set_discard_rectangle_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDiscardRectangleEXT".as_ptr(),
            )));
        self.cmd_set_discard_rectangle_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDiscardRectangleEnableEXT".as_ptr(),
            )));
        self.cmd_set_discard_rectangle_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDiscardRectangleModeEXT".as_ptr(),
            )));
        self.set_hdr_metadata_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetHdrMetadataEXT".as_ptr(),
            )));
        self.get_swapchain_status_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSwapchainStatusKHR".as_ptr(),
            )));
        self.import_fence_win32_handle_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkImportFenceWin32HandleKHR".as_ptr(),
            )));
        self.get_fence_win32_handle_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetFenceWin32HandleKHR".as_ptr(),
            )));
        self.import_fence_fd_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkImportFenceFdKHR".as_ptr(),
            )));
        self.get_fence_fd_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetFenceFdKHR".as_ptr(),
            )));
        self.acquire_profiling_lock_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAcquireProfilingLockKHR".as_ptr(),
            )));
        self.release_profiling_lock_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkReleaseProfilingLockKHR".as_ptr(),
            )));
        self.set_debug_utils_object_name_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetDebugUtilsObjectNameEXT".as_ptr(),
            )));
        self.set_debug_utils_object_tag_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetDebugUtilsObjectTagEXT".as_ptr(),
            )));
        self.queue_begin_debug_utils_label_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueBeginDebugUtilsLabelEXT".as_ptr(),
            )));
        self.queue_end_debug_utils_label_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueEndDebugUtilsLabelEXT".as_ptr(),
            )));
        self.queue_insert_debug_utils_label_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueInsertDebugUtilsLabelEXT".as_ptr(),
            )));
        self.cmd_begin_debug_utils_label_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBeginDebugUtilsLabelEXT".as_ptr(),
            )));
        self.cmd_end_debug_utils_label_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdEndDebugUtilsLabelEXT".as_ptr(),
            )));
        self.cmd_insert_debug_utils_label_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdInsertDebugUtilsLabelEXT".as_ptr(),
            )));
        self.get_android_hardware_buffer_properties_android
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetAndroidHardwareBufferPropertiesANDROID".as_ptr(),
            )));
        self.get_memory_android_hardware_buffer_android
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryAndroidHardwareBufferANDROID".as_ptr(),
            )));
        self.create_execution_graph_pipelines_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateExecutionGraphPipelinesAMDX".as_ptr(),
            )));
        self.get_execution_graph_pipeline_scratch_size_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetExecutionGraphPipelineScratchSizeAMDX".as_ptr(),
            )));
        self.get_execution_graph_pipeline_node_index_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetExecutionGraphPipelineNodeIndexAMDX".as_ptr(),
            )));
        self.cmd_initialize_graph_scratch_memory_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdInitializeGraphScratchMemoryAMDX".as_ptr(),
            )));
        self.cmd_dispatch_graph_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDispatchGraphAMDX".as_ptr(),
            )));
        self.cmd_dispatch_graph_indirect_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDispatchGraphIndirectAMDX".as_ptr(),
            )));
        self.cmd_dispatch_graph_indirect_count_amdx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDispatchGraphIndirectCountAMDX".as_ptr(),
            )));
        self.cmd_set_sample_locations_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetSampleLocationsEXT".as_ptr(),
            )));
        self.create_acceleration_structure_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateAccelerationStructureKHR".as_ptr(),
            )));
        self.destroy_acceleration_structure_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyAccelerationStructureKHR".as_ptr(),
            )));
        self.cmd_build_acceleration_structures_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBuildAccelerationStructuresKHR".as_ptr(),
            )));
        self.cmd_build_acceleration_structures_indirect_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBuildAccelerationStructuresIndirectKHR".as_ptr(),
            )));
        self.build_acceleration_structures_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBuildAccelerationStructuresKHR".as_ptr(),
            )));
        self.copy_acceleration_structure_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyAccelerationStructureKHR".as_ptr(),
            )));
        self.copy_acceleration_structure_to_memory_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyAccelerationStructureToMemoryKHR".as_ptr(),
            )));
        self.copy_memory_to_acceleration_structure_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyMemoryToAccelerationStructureKHR".as_ptr(),
            )));
        self.write_acceleration_structures_properties_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkWriteAccelerationStructuresPropertiesKHR".as_ptr(),
            )));
        self.cmd_copy_acceleration_structure_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyAccelerationStructureKHR".as_ptr(),
            )));
        self.cmd_copy_acceleration_structure_to_memory_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyAccelerationStructureToMemoryKHR".as_ptr(),
            )));
        self.cmd_copy_memory_to_acceleration_structure_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyMemoryToAccelerationStructureKHR".as_ptr(),
            )));
        self.get_acceleration_structure_device_address_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetAccelerationStructureDeviceAddressKHR".as_ptr(),
            )));
        self.cmd_write_acceleration_structures_properties_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteAccelerationStructuresPropertiesKHR".as_ptr(),
            )));
        self.get_device_acceleration_structure_compatibility_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceAccelerationStructureCompatibilityKHR".as_ptr(),
            )));
        self.get_acceleration_structure_build_sizes_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetAccelerationStructureBuildSizesKHR".as_ptr(),
            )));
        self.cmd_trace_rays_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdTraceRaysKHR".as_ptr(),
            )));
        self.create_ray_tracing_pipelines_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateRayTracingPipelinesKHR".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_handles_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRayTracingShaderGroupHandlesKHR".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_handles_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRayTracingShaderGroupHandlesNV".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_handles_khr.set(
            self.get_ray_tracing_shader_group_handles_khr
                .get()
                .or(self.get_ray_tracing_shader_group_handles_nv.get()),
        );
        self.get_ray_tracing_capture_replay_shader_group_handles_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR".as_ptr(),
            )));
        self.cmd_trace_rays_indirect_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdTraceRaysIndirectKHR".as_ptr(),
            )));
        self.get_ray_tracing_shader_group_stack_size_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRayTracingShaderGroupStackSizeKHR".as_ptr(),
            )));
        self.cmd_set_ray_tracing_pipeline_stack_size_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRayTracingPipelineStackSizeKHR".as_ptr(),
            )));
        self.get_image_drm_format_modifier_properties_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageDrmFormatModifierPropertiesEXT".as_ptr(),
            )));
        self.create_validation_cache_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateValidationCacheEXT".as_ptr(),
            )));
        self.destroy_validation_cache_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyValidationCacheEXT".as_ptr(),
            )));
        self.merge_validation_caches_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkMergeValidationCachesEXT".as_ptr(),
            )));
        self.get_validation_cache_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetValidationCacheDataEXT".as_ptr(),
            )));
        self.cmd_bind_shading_rate_image_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindShadingRateImageNV".as_ptr(),
            )));
        self.cmd_set_viewport_shading_rate_palette_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewportShadingRatePaletteNV".as_ptr(),
            )));
        self.cmd_set_coarse_sample_order_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoarseSampleOrderNV".as_ptr(),
            )));
        self.create_acceleration_structure_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateAccelerationStructureNV".as_ptr(),
            )));
        self.destroy_acceleration_structure_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyAccelerationStructureNV".as_ptr(),
            )));
        self.get_acceleration_structure_memory_requirements_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetAccelerationStructureMemoryRequirementsNV".as_ptr(),
            )));
        self.bind_acceleration_structure_memory_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindAccelerationStructureMemoryNV".as_ptr(),
            )));
        self.cmd_build_acceleration_structure_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBuildAccelerationStructureNV".as_ptr(),
            )));
        self.cmd_copy_acceleration_structure_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyAccelerationStructureNV".as_ptr(),
            )));
        self.cmd_trace_rays_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdTraceRaysNV".as_ptr(),
            )));
        self.create_ray_tracing_pipelines_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateRayTracingPipelinesNV".as_ptr(),
            )));
        self.get_acceleration_structure_handle_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetAccelerationStructureHandleNV".as_ptr(),
            )));
        self.cmd_write_acceleration_structures_properties_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteAccelerationStructuresPropertiesNV".as_ptr(),
            )));
        self.compile_deferred_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCompileDeferredNV".as_ptr(),
            )));
        self.get_memory_host_pointer_properties_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryHostPointerPropertiesEXT".as_ptr(),
            )));
        self.cmd_write_buffer_marker_amd
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteBufferMarkerAMD".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMeshTasksNV".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMeshTasksIndirectNV".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_count_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMeshTasksIndirectCountNV".as_ptr(),
            )));
        self.cmd_set_exclusive_scissor_enable_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetExclusiveScissorEnableNV".as_ptr(),
            )));
        self.cmd_set_exclusive_scissor_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetExclusiveScissorNV".as_ptr(),
            )));
        self.cmd_set_checkpoint_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCheckpointNV".as_ptr(),
            )));
        self.get_queue_checkpoint_data_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetQueueCheckpointDataNV".as_ptr(),
            )));
        self.initialize_performance_api_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkInitializePerformanceApiINTEL".as_ptr(),
            )));
        self.uninitialize_performance_api_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkUninitializePerformanceApiINTEL".as_ptr(),
            )));
        self.cmd_set_performance_marker_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPerformanceMarkerINTEL".as_ptr(),
            )));
        self.cmd_set_performance_stream_marker_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPerformanceStreamMarkerINTEL".as_ptr(),
            )));
        self.cmd_set_performance_override_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPerformanceOverrideINTEL".as_ptr(),
            )));
        self.acquire_performance_configuration_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAcquirePerformanceConfigurationINTEL".as_ptr(),
            )));
        self.release_performance_configuration_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkReleasePerformanceConfigurationINTEL".as_ptr(),
            )));
        self.queue_set_performance_configuration_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueSetPerformanceConfigurationINTEL".as_ptr(),
            )));
        self.get_performance_parameter_intel
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPerformanceParameterINTEL".as_ptr(),
            )));
        self.set_local_dimming_amd
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetLocalDimmingAMD".as_ptr(),
            )));
        self.cmd_set_fragment_shading_rate_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetFragmentShadingRateKHR".as_ptr(),
            )));
        self.cmd_set_rendering_attachment_locations_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRenderingAttachmentLocationsKHR".as_ptr(),
            )));
        self.cmd_set_rendering_input_attachment_indices_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRenderingInputAttachmentIndicesKHR".as_ptr(),
            )));
        self.wait_for_present_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkWaitForPresentKHR".as_ptr(),
            )));
        self.acquire_full_screen_exclusive_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkAcquireFullScreenExclusiveModeEXT".as_ptr(),
            )));
        self.release_full_screen_exclusive_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkReleaseFullScreenExclusiveModeEXT".as_ptr(),
            )));
        self.get_device_group_surface_present_modes2_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceGroupSurfacePresentModes2EXT".as_ptr(),
            )));
        self.create_deferred_operation_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateDeferredOperationKHR".as_ptr(),
            )));
        self.destroy_deferred_operation_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyDeferredOperationKHR".as_ptr(),
            )));
        self.get_deferred_operation_max_concurrency_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeferredOperationMaxConcurrencyKHR".as_ptr(),
            )));
        self.get_deferred_operation_result_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeferredOperationResultKHR".as_ptr(),
            )));
        self.deferred_operation_join_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDeferredOperationJoinKHR".as_ptr(),
            )));
        self.get_pipeline_executable_properties_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelineExecutablePropertiesKHR".as_ptr(),
            )));
        self.get_pipeline_executable_statistics_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelineExecutableStatisticsKHR".as_ptr(),
            )));
        self.get_pipeline_executable_internal_representations_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelineExecutableInternalRepresentationsKHR".as_ptr(),
            )));
        self.copy_memory_to_image_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyMemoryToImageEXT".as_ptr(),
            )));
        self.copy_image_to_memory_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyImageToMemoryEXT".as_ptr(),
            )));
        self.copy_image_to_image_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyImageToImageEXT".as_ptr(),
            )));
        self.transition_image_layout_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkTransitionImageLayoutEXT".as_ptr(),
            )));
        self.map_memory2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkMapMemory2KHR".as_ptr(),
            )));
        self.unmap_memory2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkUnmapMemory2KHR".as_ptr(),
            )));
        self.release_swapchain_images_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkReleaseSwapchainImagesEXT".as_ptr(),
            )));
        self.get_generated_commands_memory_requirements_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetGeneratedCommandsMemoryRequirementsNV".as_ptr(),
            )));
        self.cmd_preprocess_generated_commands_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPreprocessGeneratedCommandsNV".as_ptr(),
            )));
        self.cmd_execute_generated_commands_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdExecuteGeneratedCommandsNV".as_ptr(),
            )));
        self.cmd_bind_pipeline_shader_group_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindPipelineShaderGroupNV".as_ptr(),
            )));
        self.create_indirect_commands_layout_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateIndirectCommandsLayoutNV".as_ptr(),
            )));
        self.destroy_indirect_commands_layout_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyIndirectCommandsLayoutNV".as_ptr(),
            )));
        self.cmd_set_depth_bias2_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthBias2EXT".as_ptr(),
            )));
        self.create_cuda_module_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateCudaModuleNV".as_ptr(),
            )));
        self.get_cuda_module_cache_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetCudaModuleCacheNV".as_ptr(),
            )));
        self.create_cuda_function_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateCudaFunctionNV".as_ptr(),
            )));
        self.destroy_cuda_module_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyCudaModuleNV".as_ptr(),
            )));
        self.destroy_cuda_function_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyCudaFunctionNV".as_ptr(),
            )));
        self.cmd_cuda_launch_kernel_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCudaLaunchKernelNV".as_ptr(),
            )));
        self.export_metal_objects_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkExportMetalObjectsEXT".as_ptr(),
            )));
        self.cmd_write_buffer_marker2_amd
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteBufferMarker2AMD".as_ptr(),
            )));
        self.get_queue_checkpoint_data2_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetQueueCheckpointData2NV".as_ptr(),
            )));
        self.get_descriptor_set_layout_size_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorSetLayoutSizeEXT".as_ptr(),
            )));
        self.get_descriptor_set_layout_binding_offset_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorSetLayoutBindingOffsetEXT".as_ptr(),
            )));
        self.get_descriptor_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorEXT".as_ptr(),
            )));
        self.cmd_bind_descriptor_buffers_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindDescriptorBuffersEXT".as_ptr(),
            )));
        self.cmd_set_descriptor_buffer_offsets_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDescriptorBufferOffsetsEXT".as_ptr(),
            )));
        self.cmd_bind_descriptor_buffer_embedded_samplers_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT".as_ptr(),
            )));
        self.get_buffer_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_image_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_image_view_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageViewOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_sampler_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSamplerOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.get_acceleration_structure_opaque_capture_descriptor_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT".as_ptr(),
            )));
        self.cmd_set_fragment_shading_rate_enum_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetFragmentShadingRateEnumNV".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMeshTasksEXT".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMeshTasksIndirectEXT".as_ptr(),
            )));
        self.cmd_draw_mesh_tasks_indirect_count_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMeshTasksIndirectCountEXT".as_ptr(),
            )));
        self.get_device_fault_info_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceFaultInfoEXT".as_ptr(),
            )));
        self.cmd_set_vertex_input_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetVertexInputEXT".as_ptr(),
            )));
        self.get_memory_zircon_handle_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryZirconHandleFUCHSIA".as_ptr(),
            )));
        self.get_memory_zircon_handle_properties_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryZirconHandlePropertiesFUCHSIA".as_ptr(),
            )));
        self.import_semaphore_zircon_handle_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkImportSemaphoreZirconHandleFUCHSIA".as_ptr(),
            )));
        self.get_semaphore_zircon_handle_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetSemaphoreZirconHandleFUCHSIA".as_ptr(),
            )));
        self.create_buffer_collection_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateBufferCollectionFUCHSIA".as_ptr(),
            )));
        self.set_buffer_collection_image_constraints_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetBufferCollectionImageConstraintsFUCHSIA".as_ptr(),
            )));
        self.set_buffer_collection_buffer_constraints_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetBufferCollectionBufferConstraintsFUCHSIA".as_ptr(),
            )));
        self.destroy_buffer_collection_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyBufferCollectionFUCHSIA".as_ptr(),
            )));
        self.get_buffer_collection_properties_fuchsia
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetBufferCollectionPropertiesFUCHSIA".as_ptr(),
            )));
        self.get_device_subpass_shading_max_workgroup_size_huawei
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI".as_ptr(),
            )));
        self.cmd_subpass_shading_huawei
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSubpassShadingHUAWEI".as_ptr(),
            )));
        self.cmd_bind_invocation_mask_huawei
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindInvocationMaskHUAWEI".as_ptr(),
            )));
        self.get_memory_remote_address_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMemoryRemoteAddressNV".as_ptr(),
            )));
        self.get_pipeline_properties_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelinePropertiesEXT".as_ptr(),
            )));
        self.cmd_set_patch_control_points_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPatchControlPointsEXT".as_ptr(),
            )));
        self.cmd_set_logic_op_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLogicOpEXT".as_ptr(),
            )));
        self.cmd_set_color_write_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetColorWriteEnableEXT".as_ptr(),
            )));
        self.cmd_trace_rays_indirect2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdTraceRaysIndirect2KHR".as_ptr(),
            )));
        self.cmd_draw_multi_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMultiEXT".as_ptr(),
            )));
        self.cmd_draw_multi_indexed_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawMultiIndexedEXT".as_ptr(),
            )));
        self.create_micromap_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateMicromapEXT".as_ptr(),
            )));
        self.destroy_micromap_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyMicromapEXT".as_ptr(),
            )));
        self.cmd_build_micromaps_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBuildMicromapsEXT".as_ptr(),
            )));
        self.build_micromaps_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBuildMicromapsEXT".as_ptr(),
            )));
        self.copy_micromap_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyMicromapEXT".as_ptr(),
            )));
        self.copy_micromap_to_memory_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyMicromapToMemoryEXT".as_ptr(),
            )));
        self.copy_memory_to_micromap_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCopyMemoryToMicromapEXT".as_ptr(),
            )));
        self.write_micromaps_properties_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkWriteMicromapsPropertiesEXT".as_ptr(),
            )));
        self.cmd_copy_micromap_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyMicromapEXT".as_ptr(),
            )));
        self.cmd_copy_micromap_to_memory_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyMicromapToMemoryEXT".as_ptr(),
            )));
        self.cmd_copy_memory_to_micromap_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyMemoryToMicromapEXT".as_ptr(),
            )));
        self.cmd_write_micromaps_properties_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdWriteMicromapsPropertiesEXT".as_ptr(),
            )));
        self.get_device_micromap_compatibility_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceMicromapCompatibilityEXT".as_ptr(),
            )));
        self.get_micromap_build_sizes_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetMicromapBuildSizesEXT".as_ptr(),
            )));
        self.cmd_draw_cluster_huawei
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawClusterHUAWEI".as_ptr(),
            )));
        self.cmd_draw_cluster_indirect_huawei
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDrawClusterIndirectHUAWEI".as_ptr(),
            )));
        self.set_device_memory_priority_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetDeviceMemoryPriorityEXT".as_ptr(),
            )));
        self.get_descriptor_set_layout_host_mapping_info_valve
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorSetLayoutHostMappingInfoVALVE".as_ptr(),
            )));
        self.get_descriptor_set_host_mapping_valve
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDescriptorSetHostMappingVALVE".as_ptr(),
            )));
        self.cmd_copy_memory_indirect_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyMemoryIndirectNV".as_ptr(),
            )));
        self.cmd_copy_memory_to_image_indirect_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdCopyMemoryToImageIndirectNV".as_ptr(),
            )));
        self.cmd_decompress_memory_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDecompressMemoryNV".as_ptr(),
            )));
        self.cmd_decompress_memory_indirect_count_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdDecompressMemoryIndirectCountNV".as_ptr(),
            )));
        self.get_pipeline_indirect_memory_requirements_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelineIndirectMemoryRequirementsNV".as_ptr(),
            )));
        self.cmd_update_pipeline_indirect_buffer_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdUpdatePipelineIndirectBufferNV".as_ptr(),
            )));
        self.get_pipeline_indirect_device_address_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetPipelineIndirectDeviceAddressNV".as_ptr(),
            )));
        self.cmd_set_depth_clamp_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthClampEnableEXT".as_ptr(),
            )));
        self.cmd_set_polygon_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetPolygonModeEXT".as_ptr(),
            )));
        self.cmd_set_rasterization_samples_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRasterizationSamplesEXT".as_ptr(),
            )));
        self.cmd_set_sample_mask_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetSampleMaskEXT".as_ptr(),
            )));
        self.cmd_set_alpha_to_coverage_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetAlphaToCoverageEnableEXT".as_ptr(),
            )));
        self.cmd_set_alpha_to_one_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetAlphaToOneEnableEXT".as_ptr(),
            )));
        self.cmd_set_logic_op_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLogicOpEnableEXT".as_ptr(),
            )));
        self.cmd_set_color_blend_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetColorBlendEnableEXT".as_ptr(),
            )));
        self.cmd_set_color_blend_equation_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetColorBlendEquationEXT".as_ptr(),
            )));
        self.cmd_set_color_write_mask_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetColorWriteMaskEXT".as_ptr(),
            )));
        self.cmd_set_tessellation_domain_origin_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetTessellationDomainOriginEXT".as_ptr(),
            )));
        self.cmd_set_rasterization_stream_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRasterizationStreamEXT".as_ptr(),
            )));
        self.cmd_set_conservative_rasterization_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetConservativeRasterizationModeEXT".as_ptr(),
            )));
        self.cmd_set_extra_primitive_overestimation_size_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetExtraPrimitiveOverestimationSizeEXT".as_ptr(),
            )));
        self.cmd_set_depth_clip_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthClipEnableEXT".as_ptr(),
            )));
        self.cmd_set_sample_locations_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetSampleLocationsEnableEXT".as_ptr(),
            )));
        self.cmd_set_color_blend_advanced_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetColorBlendAdvancedEXT".as_ptr(),
            )));
        self.cmd_set_provoking_vertex_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetProvokingVertexModeEXT".as_ptr(),
            )));
        self.cmd_set_line_rasterization_mode_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLineRasterizationModeEXT".as_ptr(),
            )));
        self.cmd_set_line_stipple_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLineStippleEnableEXT".as_ptr(),
            )));
        self.cmd_set_depth_clip_negative_one_to_one_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDepthClipNegativeOneToOneEXT".as_ptr(),
            )));
        self.cmd_set_viewport_wscaling_enable_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewportWScalingEnableNV".as_ptr(),
            )));
        self.cmd_set_viewport_swizzle_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetViewportSwizzleNV".as_ptr(),
            )));
        self.cmd_set_coverage_to_color_enable_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoverageToColorEnableNV".as_ptr(),
            )));
        self.cmd_set_coverage_to_color_location_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoverageToColorLocationNV".as_ptr(),
            )));
        self.cmd_set_coverage_modulation_mode_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoverageModulationModeNV".as_ptr(),
            )));
        self.cmd_set_coverage_modulation_table_enable_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoverageModulationTableEnableNV".as_ptr(),
            )));
        self.cmd_set_coverage_modulation_table_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoverageModulationTableNV".as_ptr(),
            )));
        self.cmd_set_shading_rate_image_enable_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetShadingRateImageEnableNV".as_ptr(),
            )));
        self.cmd_set_representative_fragment_test_enable_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetRepresentativeFragmentTestEnableNV".as_ptr(),
            )));
        self.cmd_set_coverage_reduction_mode_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetCoverageReductionModeNV".as_ptr(),
            )));
        self.get_shader_module_identifier_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetShaderModuleIdentifierEXT".as_ptr(),
            )));
        self.get_shader_module_create_info_identifier_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetShaderModuleCreateInfoIdentifierEXT".as_ptr(),
            )));
        self.create_optical_flow_session_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateOpticalFlowSessionNV".as_ptr(),
            )));
        self.destroy_optical_flow_session_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyOpticalFlowSessionNV".as_ptr(),
            )));
        self.bind_optical_flow_session_image_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkBindOpticalFlowSessionImageNV".as_ptr(),
            )));
        self.cmd_optical_flow_execute_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdOpticalFlowExecuteNV".as_ptr(),
            )));
        self.cmd_bind_index_buffer2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindIndexBuffer2KHR".as_ptr(),
            )));
        self.get_rendering_area_granularity_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetRenderingAreaGranularityKHR".as_ptr(),
            )));
        self.get_device_image_subresource_layout_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDeviceImageSubresourceLayoutKHR".as_ptr(),
            )));
        self.get_image_subresource_layout2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageSubresourceLayout2KHR".as_ptr(),
            )));
        self.get_image_subresource_layout2_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetImageSubresourceLayout2EXT".as_ptr(),
            )));
        self.get_image_subresource_layout2_khr.set(
            self.get_image_subresource_layout2_khr
                .get()
                .or(self.get_image_subresource_layout2_ext.get()),
        );
        self.create_shaders_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCreateShadersEXT".as_ptr(),
            )));
        self.destroy_shader_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkDestroyShaderEXT".as_ptr(),
            )));
        self.get_shader_binary_data_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetShaderBinaryDataEXT".as_ptr(),
            )));
        self.cmd_bind_shaders_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindShadersEXT".as_ptr(),
            )));
        self.get_framebuffer_tile_properties_qcom
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetFramebufferTilePropertiesQCOM".as_ptr(),
            )));
        self.get_dynamic_rendering_tile_properties_qcom
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetDynamicRenderingTilePropertiesQCOM".as_ptr(),
            )));
        self.set_latency_sleep_mode_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetLatencySleepModeNV".as_ptr(),
            )));
        self.latency_sleep_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkLatencySleepNV".as_ptr(),
            )));
        self.set_latency_marker_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkSetLatencyMarkerNV".as_ptr(),
            )));
        self.get_latency_timings_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetLatencyTimingsNV".as_ptr(),
            )));
        self.queue_notify_out_of_band_nv
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkQueueNotifyOutOfBandNV".as_ptr(),
            )));
        self.cmd_set_attachment_feedback_loop_enable_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetAttachmentFeedbackLoopEnableEXT".as_ptr(),
            )));
        self.get_screen_buffer_properties_qnx
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetScreenBufferPropertiesQNX".as_ptr(),
            )));
        self.cmd_set_line_stipple_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLineStippleKHR".as_ptr(),
            )));
        self.cmd_set_line_stipple_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetLineStippleEXT".as_ptr(),
            )));
        self.cmd_set_line_stipple_khr.set(
            self.cmd_set_line_stipple_khr
                .get()
                .or(self.cmd_set_line_stipple_ext.get()),
        );
        self.get_calibrated_timestamps_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetCalibratedTimestampsKHR".as_ptr(),
            )));
        self.get_calibrated_timestamps_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkGetCalibratedTimestampsEXT".as_ptr(),
            )));
        self.get_calibrated_timestamps_khr.set(
            self.get_calibrated_timestamps_khr
                .get()
                .or(self.get_calibrated_timestamps_ext.get()),
        );
        self.cmd_bind_descriptor_sets2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindDescriptorSets2KHR".as_ptr(),
            )));
        self.cmd_push_constants2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPushConstants2KHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPushDescriptorSet2KHR".as_ptr(),
            )));
        self.cmd_push_descriptor_set_with_template2_khr
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdPushDescriptorSetWithTemplate2KHR".as_ptr(),
            )));
        self.cmd_set_descriptor_buffer_offsets2_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdSetDescriptorBufferOffsets2EXT".as_ptr(),
            )));
        self.cmd_bind_descriptor_buffer_embedded_samplers2_ext
            .set(mem::transmute(get_device_proc_addr(
                get_device(),
                c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT".as_ptr(),
            )));
    }
}
