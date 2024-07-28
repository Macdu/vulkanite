use super::{
    ApiVersion, DeviceExtension, DeviceExtensionName, InstanceExtension, InstanceExtensionName,
};
pub const HEADER_VERSION: ApiVersion = ApiVersion::new(0, 1, 3, 292u32);
pub const KHR_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_surface") },
    spec: 25u32,
};
pub const KHR_SWAPCHAIN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_swapchain") },
    spec: 70u32,
};
pub const KHR_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_display") },
    spec: 23u32,
};
pub const KHR_DISPLAY_SWAPCHAIN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_display_swapchain") },
    spec: 10u32,
};
pub const KHR_XLIB_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_xlib_surface") },
    spec: 6u32,
};
pub const KHR_XCB_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_xcb_surface") },
    spec: 6u32,
};
pub const KHR_WAYLAND_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_wayland_surface") },
    spec: 6u32,
};
pub const KHR_ANDROID_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_android_surface") },
    spec: 6u32,
};
pub const KHR_WIN32_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_win32_surface") },
    spec: 6u32,
};
pub const EXT_DEBUG_REPORT: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_debug_report") },
    spec: 10u32,
};
pub const NV_GLSL_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_glsl_shader") },
    spec: 1u32,
};
pub const EXT_DEPTH_RANGE_UNRESTRICTED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_range_unrestricted") },
    spec: 1u32,
};
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_sampler_mirror_clamp_to_edge") },
    spec: 3u32,
};
pub const IMG_FILTER_CUBIC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_IMG_filter_cubic") },
    spec: 1u32,
};
pub const AMD_RASTERIZATION_ORDER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_rasterization_order") },
    spec: 1u32,
};
pub const AMD_SHADER_TRINARY_MINMAX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_trinary_minmax") },
    spec: 1u32,
};
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_explicit_vertex_parameter") },
    spec: 1u32,
};
pub const EXT_DEBUG_MARKER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_debug_marker") },
    spec: 4u32,
};
pub const AMD_GCN_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_gcn_shader") },
    spec: 1u32,
};
pub const NV_DEDICATED_ALLOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_dedicated_allocation") },
    spec: 1u32,
};
pub const EXT_TRANSFORM_FEEDBACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_transform_feedback") },
    spec: 1u32,
};
pub const NVX_BINARY_IMPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NVX_binary_import") },
    spec: 1u32,
};
pub const NVX_IMAGE_VIEW_HANDLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NVX_image_view_handle") },
    spec: 2u32,
};
pub const AMD_DRAW_INDIRECT_COUNT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_draw_indirect_count") },
    spec: 2u32,
};
pub const AMD_NEGATIVE_VIEWPORT_HEIGHT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_negative_viewport_height") },
    spec: 1u32,
};
pub const AMD_GPU_SHADER_HALF_FLOAT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_gpu_shader_half_float") },
    spec: 2u32,
};
pub const AMD_SHADER_BALLOT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_ballot") },
    spec: 1u32,
};
pub const AMD_TEXTURE_GATHER_BIAS_LOD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_texture_gather_bias_lod") },
    spec: 1u32,
};
pub const AMD_SHADER_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_info") },
    spec: 1u32,
};
pub const KHR_DYNAMIC_RENDERING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_dynamic_rendering") },
    spec: 1u32,
};
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_image_load_store_lod") },
    spec: 1u32,
};
pub const GGP_STREAM_DESCRIPTOR_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_GGP_stream_descriptor_surface") },
    spec: 1u32,
};
pub const NV_CORNER_SAMPLED_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_corner_sampled_image") },
    spec: 2u32,
};
pub const KHR_MULTIVIEW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_multiview") },
    spec: 1u32,
};
pub const IMG_FORMAT_PVRTC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_IMG_format_pvrtc") },
    spec: 1u32,
};
pub const NV_EXTERNAL_MEMORY_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_NV_external_memory_capabilities") },
    spec: 1u32,
};
pub const NV_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_memory") },
    spec: 1u32,
};
pub const NV_EXTERNAL_MEMORY_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_memory_win32") },
    spec: 1u32,
};
pub const NV_WIN32_KEYED_MUTEX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_win32_keyed_mutex") },
    spec: 2u32,
};
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES2: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_get_physical_device_properties2") },
    spec: 2u32,
};
pub const KHR_DEVICE_GROUP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_device_group") },
    spec: 4u32,
};
pub const EXT_VALIDATION_FLAGS: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_validation_flags") },
    spec: 3u32,
};
pub const NN_VI_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_NN_vi_surface") },
    spec: 1u32,
};
pub const KHR_SHADER_DRAW_PARAMETERS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_draw_parameters") },
    spec: 1u32,
};
pub const EXT_SHADER_SUBGROUP_BALLOT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_subgroup_ballot") },
    spec: 1u32,
};
pub const EXT_SHADER_SUBGROUP_VOTE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_subgroup_vote") },
    spec: 1u32,
};
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_texture_compression_astc_hdr") },
    spec: 1u32,
};
pub const EXT_ASTC_DECODE_MODE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_astc_decode_mode") },
    spec: 1u32,
};
pub const EXT_PIPELINE_ROBUSTNESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_robustness") },
    spec: 1u32,
};
pub const KHR_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance1") },
    spec: 2u32,
};
pub const KHR_DEVICE_GROUP_CREATION: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_device_group_creation") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_external_memory_capabilities") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_memory") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_MEMORY_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_memory_win32") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_MEMORY_FD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_memory_fd") },
    spec: 1u32,
};
pub const KHR_WIN32_KEYED_MUTEX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_win32_keyed_mutex") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_external_semaphore_capabilities") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_SEMAPHORE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_semaphore") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_SEMAPHORE_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_semaphore_win32") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_SEMAPHORE_FD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_semaphore_fd") },
    spec: 1u32,
};
pub const KHR_PUSH_DESCRIPTOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_push_descriptor") },
    spec: 2u32,
};
pub const EXT_CONDITIONAL_RENDERING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_conditional_rendering") },
    spec: 2u32,
};
pub const KHR_SHADER_FLOAT16_INT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_float16_int8") },
    spec: 1u32,
};
pub const KHR_16BIT_STORAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_16bit_storage") },
    spec: 1u32,
};
pub const KHR_INCREMENTAL_PRESENT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_incremental_present") },
    spec: 2u32,
};
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_descriptor_update_template") },
    spec: 1u32,
};
pub const NV_CLIP_SPACE_W_SCALING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_clip_space_w_scaling") },
    spec: 1u32,
};
pub const EXT_DIRECT_MODE_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_direct_mode_display") },
    spec: 1u32,
};
pub const EXT_ACQUIRE_XLIB_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_acquire_xlib_display") },
    spec: 1u32,
};
pub const EXT_DISPLAY_SURFACE_COUNTER: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_display_surface_counter") },
    spec: 1u32,
};
pub const EXT_DISPLAY_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_display_control") },
    spec: 1u32,
};
pub const GOOGLE_DISPLAY_TIMING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GOOGLE_display_timing") },
    spec: 1u32,
};
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_sample_mask_override_coverage") },
    spec: 1u32,
};
pub const NV_GEOMETRY_SHADER_PASSTHROUGH: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_geometry_shader_passthrough") },
    spec: 1u32,
};
pub const NV_VIEWPORT_ARRAY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_viewport_array2") },
    spec: 1u32,
};
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NVX_multiview_per_view_attributes") },
    spec: 1u32,
};
pub const NV_VIEWPORT_SWIZZLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_viewport_swizzle") },
    spec: 1u32,
};
pub const EXT_DISCARD_RECTANGLES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_discard_rectangles") },
    spec: 2u32,
};
pub const EXT_CONSERVATIVE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_conservative_rasterization") },
    spec: 1u32,
};
pub const EXT_DEPTH_CLIP_ENABLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clip_enable") },
    spec: 1u32,
};
pub const EXT_SWAPCHAIN_COLORSPACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_swapchain_colorspace") },
    spec: 5u32,
};
pub const EXT_HDR_METADATA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_hdr_metadata") },
    spec: 3u32,
};
pub const KHR_IMAGELESS_FRAMEBUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_imageless_framebuffer") },
    spec: 1u32,
};
pub const KHR_CREATE_RENDERPASS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_create_renderpass2") },
    spec: 1u32,
};
pub const IMG_RELAXED_LINE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_IMG_relaxed_line_rasterization") },
    spec: 1u32,
};
pub const KHR_SHARED_PRESENTABLE_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shared_presentable_image") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_FENCE_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_external_fence_capabilities") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_FENCE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_fence") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_FENCE_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_fence_win32") },
    spec: 1u32,
};
pub const KHR_EXTERNAL_FENCE_FD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_fence_fd") },
    spec: 1u32,
};
pub const KHR_PERFORMANCE_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_performance_query") },
    spec: 1u32,
};
pub const KHR_MAINTENANCE2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance2") },
    spec: 1u32,
};
pub const KHR_GET_SURFACE_CAPABILITIES2: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_get_surface_capabilities2") },
    spec: 1u32,
};
pub const KHR_VARIABLE_POINTERS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_variable_pointers") },
    spec: 1u32,
};
pub const KHR_GET_DISPLAY_PROPERTIES2: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_get_display_properties2") },
    spec: 1u32,
};
pub const MVK_IOS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_MVK_ios_surface") },
    spec: 3u32,
};
pub const MVK_MACOS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_MVK_macos_surface") },
    spec: 3u32,
};
pub const EXT_EXTERNAL_MEMORY_DMA_BUF: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_dma_buf") },
    spec: 1u32,
};
pub const EXT_QUEUE_FAMILY_FOREIGN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_queue_family_foreign") },
    spec: 1u32,
};
pub const KHR_DEDICATED_ALLOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_dedicated_allocation") },
    spec: 3u32,
};
pub const EXT_DEBUG_UTILS: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_debug_utils") },
    spec: 2u32,
};
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe {
        DeviceExtensionName::new(c"VK_ANDROID_external_memory_android_hardware_buffer")
    },
    spec: 5u32,
};
pub const EXT_SAMPLER_FILTER_MINMAX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_sampler_filter_minmax") },
    spec: 2u32,
};
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_storage_buffer_storage_class") },
    spec: 1u32,
};
pub const AMD_GPU_SHADER_INT16: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_gpu_shader_int16") },
    spec: 2u32,
};
pub const AMDX_SHADER_ENQUEUE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMDX_shader_enqueue") },
    spec: 1u32,
};
pub const AMD_MIXED_ATTACHMENT_SAMPLES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_mixed_attachment_samples") },
    spec: 1u32,
};
pub const AMD_SHADER_FRAGMENT_MASK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_fragment_mask") },
    spec: 1u32,
};
pub const EXT_INLINE_UNIFORM_BLOCK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_inline_uniform_block") },
    spec: 1u32,
};
pub const EXT_SHADER_STENCIL_EXPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_stencil_export") },
    spec: 1u32,
};
pub const EXT_SAMPLE_LOCATIONS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_sample_locations") },
    spec: 1u32,
};
pub const KHR_RELAXED_BLOCK_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_relaxed_block_layout") },
    spec: 1u32,
};
pub const KHR_GET_MEMORY_REQUIREMENTS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_get_memory_requirements2") },
    spec: 1u32,
};
pub const KHR_IMAGE_FORMAT_LIST: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_image_format_list") },
    spec: 1u32,
};
pub const EXT_BLEND_OPERATION_ADVANCED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_blend_operation_advanced") },
    spec: 2u32,
};
pub const NV_FRAGMENT_COVERAGE_TO_COLOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_fragment_coverage_to_color") },
    spec: 1u32,
};
pub const KHR_ACCELERATION_STRUCTURE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_acceleration_structure") },
    spec: 13u32,
};
pub const KHR_RAY_TRACING_PIPELINE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_tracing_pipeline") },
    spec: 1u32,
};
pub const KHR_RAY_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_query") },
    spec: 1u32,
};
pub const NV_FRAMEBUFFER_MIXED_SAMPLES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_framebuffer_mixed_samples") },
    spec: 1u32,
};
pub const NV_FILL_RECTANGLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_fill_rectangle") },
    spec: 1u32,
};
pub const NV_SHADER_SM_BUILTINS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shader_sm_builtins") },
    spec: 1u32,
};
pub const EXT_POST_DEPTH_COVERAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_post_depth_coverage") },
    spec: 1u32,
};
pub const KHR_SAMPLER_YCBCR_CONVERSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_sampler_ycbcr_conversion") },
    spec: 14u32,
};
pub const KHR_BIND_MEMORY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_bind_memory2") },
    spec: 1u32,
};
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_drm_format_modifier") },
    spec: 2u32,
};
pub const EXT_VALIDATION_CACHE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_validation_cache") },
    spec: 1u32,
};
pub const EXT_DESCRIPTOR_INDEXING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_descriptor_indexing") },
    spec: 2u32,
};
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_viewport_index_layer") },
    spec: 1u32,
};
pub const KHR_PORTABILITY_SUBSET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_portability_subset") },
    spec: 1u32,
};
pub const NV_SHADING_RATE_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shading_rate_image") },
    spec: 3u32,
};
pub const NV_RAY_TRACING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing") },
    spec: 3u32,
};
pub const NV_REPRESENTATIVE_FRAGMENT_TEST: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_representative_fragment_test") },
    spec: 2u32,
};
pub const KHR_MAINTENANCE3: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance3") },
    spec: 1u32,
};
pub const KHR_DRAW_INDIRECT_COUNT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_draw_indirect_count") },
    spec: 1u32,
};
pub const EXT_FILTER_CUBIC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_filter_cubic") },
    spec: 3u32,
};
pub const QCOM_RENDER_PASS_SHADER_RESOLVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_render_pass_shader_resolve") },
    spec: 4u32,
};
pub const EXT_GLOBAL_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_global_priority") },
    spec: 2u32,
};
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_subgroup_extended_types") },
    spec: 1u32,
};
pub const KHR_8BIT_STORAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_8bit_storage") },
    spec: 1u32,
};
pub const EXT_EXTERNAL_MEMORY_HOST: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_host") },
    spec: 1u32,
};
pub const AMD_BUFFER_MARKER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_buffer_marker") },
    spec: 1u32,
};
pub const KHR_SHADER_ATOMIC_INT64: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_atomic_int64") },
    spec: 1u32,
};
pub const KHR_SHADER_CLOCK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_clock") },
    spec: 1u32,
};
pub const AMD_PIPELINE_COMPILER_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_pipeline_compiler_control") },
    spec: 1u32,
};
pub const EXT_CALIBRATED_TIMESTAMPS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_calibrated_timestamps") },
    spec: 2u32,
};
pub const AMD_SHADER_CORE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_core_properties") },
    spec: 2u32,
};
pub const KHR_GLOBAL_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_global_priority") },
    spec: 1u32,
};
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_memory_overallocation_behavior") },
    spec: 1u32,
};
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_vertex_attribute_divisor") },
    spec: 3u32,
};
pub const GGP_FRAME_TOKEN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GGP_frame_token") },
    spec: 1u32,
};
pub const EXT_PIPELINE_CREATION_FEEDBACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_creation_feedback") },
    spec: 1u32,
};
pub const KHR_DRIVER_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_driver_properties") },
    spec: 1u32,
};
pub const KHR_SHADER_FLOAT_CONTROLS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_float_controls") },
    spec: 4u32,
};
pub const NV_SHADER_SUBGROUP_PARTITIONED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shader_subgroup_partitioned") },
    spec: 1u32,
};
pub const KHR_DEPTH_STENCIL_RESOLVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_depth_stencil_resolve") },
    spec: 1u32,
};
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_swapchain_mutable_format") },
    spec: 1u32,
};
pub const NV_COMPUTE_SHADER_DERIVATIVES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_compute_shader_derivatives") },
    spec: 1u32,
};
pub const NV_MESH_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_mesh_shader") },
    spec: 1u32,
};
pub const NV_FRAGMENT_SHADER_BARYCENTRIC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_fragment_shader_barycentric") },
    spec: 1u32,
};
pub const NV_SHADER_IMAGE_FOOTPRINT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shader_image_footprint") },
    spec: 2u32,
};
pub const NV_SCISSOR_EXCLUSIVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_scissor_exclusive") },
    spec: 2u32,
};
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_diagnostic_checkpoints") },
    spec: 2u32,
};
pub const KHR_TIMELINE_SEMAPHORE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_timeline_semaphore") },
    spec: 2u32,
};
pub const INTEL_SHADER_INTEGER_FUNCTIONS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_INTEL_shader_integer_functions2") },
    spec: 1u32,
};
pub const INTEL_PERFORMANCE_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_INTEL_performance_query") },
    spec: 2u32,
};
pub const KHR_VULKAN_MEMORY_MODEL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_vulkan_memory_model") },
    spec: 3u32,
};
pub const EXT_PCI_BUS_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pci_bus_info") },
    spec: 2u32,
};
pub const AMD_DISPLAY_NATIVE_HDR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_display_native_hdr") },
    spec: 1u32,
};
pub const FUCHSIA_IMAGEPIPE_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_FUCHSIA_imagepipe_surface") },
    spec: 1u32,
};
pub const KHR_SHADER_TERMINATE_INVOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_terminate_invocation") },
    spec: 1u32,
};
pub const EXT_METAL_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_metal_surface") },
    spec: 1u32,
};
pub const EXT_FRAGMENT_DENSITY_MAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_fragment_density_map") },
    spec: 2u32,
};
pub const EXT_SCALAR_BLOCK_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_scalar_block_layout") },
    spec: 1u32,
};
pub const GOOGLE_HLSL_FUNCTIONALITY1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GOOGLE_hlsl_functionality1") },
    spec: 1u32,
};
pub const GOOGLE_DECORATE_STRING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GOOGLE_decorate_string") },
    spec: 1u32,
};
pub const EXT_SUBGROUP_SIZE_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_subgroup_size_control") },
    spec: 2u32,
};
pub const KHR_FRAGMENT_SHADING_RATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_fragment_shading_rate") },
    spec: 2u32,
};
pub const AMD_SHADER_CORE_PROPERTIES2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_core_properties2") },
    spec: 1u32,
};
pub const AMD_DEVICE_COHERENT_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_device_coherent_memory") },
    spec: 1u32,
};
pub const KHR_DYNAMIC_RENDERING_LOCAL_READ: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_dynamic_rendering_local_read") },
    spec: 1u32,
};
pub const EXT_SHADER_IMAGE_ATOMIC_INT64: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_image_atomic_int64") },
    spec: 1u32,
};
pub const KHR_SHADER_QUAD_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_quad_control") },
    spec: 1u32,
};
pub const KHR_SPIRV_1_4: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_spirv_1_4") },
    spec: 1u32,
};
pub const EXT_MEMORY_BUDGET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_memory_budget") },
    spec: 1u32,
};
pub const EXT_MEMORY_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_memory_priority") },
    spec: 1u32,
};
pub const KHR_SURFACE_PROTECTED_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_surface_protected_capabilities") },
    spec: 1u32,
};
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_dedicated_allocation_image_aliasing") },
    spec: 1u32,
};
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_separate_depth_stencil_layouts") },
    spec: 1u32,
};
pub const EXT_BUFFER_DEVICE_ADDRESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_buffer_device_address") },
    spec: 2u32,
};
pub const EXT_TOOLING_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_tooling_info") },
    spec: 1u32,
};
pub const EXT_SEPARATE_STENCIL_USAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_separate_stencil_usage") },
    spec: 1u32,
};
pub const EXT_VALIDATION_FEATURES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_validation_features") },
    spec: 6u32,
};
pub const KHR_PRESENT_WAIT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_wait") },
    spec: 1u32,
};
pub const NV_COOPERATIVE_MATRIX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cooperative_matrix") },
    spec: 1u32,
};
pub const NV_COVERAGE_REDUCTION_MODE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_coverage_reduction_mode") },
    spec: 1u32,
};
pub const EXT_FRAGMENT_SHADER_INTERLOCK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_fragment_shader_interlock") },
    spec: 1u32,
};
pub const EXT_YCBCR_IMAGE_ARRAYS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_ycbcr_image_arrays") },
    spec: 1u32,
};
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_uniform_buffer_standard_layout") },
    spec: 1u32,
};
pub const EXT_PROVOKING_VERTEX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_provoking_vertex") },
    spec: 1u32,
};
pub const EXT_FULL_SCREEN_EXCLUSIVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_full_screen_exclusive") },
    spec: 4u32,
};
pub const EXT_HEADLESS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_headless_surface") },
    spec: 1u32,
};
pub const KHR_BUFFER_DEVICE_ADDRESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_buffer_device_address") },
    spec: 1u32,
};
pub const EXT_LINE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_line_rasterization") },
    spec: 1u32,
};
pub const EXT_SHADER_ATOMIC_FLOAT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_atomic_float") },
    spec: 1u32,
};
pub const EXT_HOST_QUERY_RESET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_host_query_reset") },
    spec: 1u32,
};
pub const EXT_INDEX_TYPE_UINT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_index_type_uint8") },
    spec: 1u32,
};
pub const EXT_EXTENDED_DYNAMIC_STATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_extended_dynamic_state") },
    spec: 1u32,
};
pub const KHR_DEFERRED_HOST_OPERATIONS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_deferred_host_operations") },
    spec: 4u32,
};
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_pipeline_executable_properties") },
    spec: 1u32,
};
pub const EXT_HOST_IMAGE_COPY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_host_image_copy") },
    spec: 1u32,
};
pub const KHR_MAP_MEMORY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_map_memory2") },
    spec: 1u32,
};
pub const EXT_MAP_MEMORY_PLACED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_map_memory_placed") },
    spec: 1u32,
};
pub const EXT_SHADER_ATOMIC_FLOAT2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_atomic_float2") },
    spec: 1u32,
};
pub const EXT_SURFACE_MAINTENANCE1: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_surface_maintenance1") },
    spec: 1u32,
};
pub const EXT_SWAPCHAIN_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_swapchain_maintenance1") },
    spec: 1u32,
};
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_demote_to_helper_invocation") },
    spec: 1u32,
};
pub const NV_DEVICE_GENERATED_COMMANDS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_generated_commands") },
    spec: 3u32,
};
pub const NV_INHERITED_VIEWPORT_SCISSOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_inherited_viewport_scissor") },
    spec: 1u32,
};
pub const KHR_SHADER_INTEGER_DOT_PRODUCT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_integer_dot_product") },
    spec: 1u32,
};
pub const EXT_TEXEL_BUFFER_ALIGNMENT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_texel_buffer_alignment") },
    spec: 1u32,
};
pub const QCOM_RENDER_PASS_TRANSFORM: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_render_pass_transform") },
    spec: 4u32,
};
pub const EXT_DEPTH_BIAS_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_bias_control") },
    spec: 1u32,
};
pub const EXT_DEVICE_MEMORY_REPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_memory_report") },
    spec: 2u32,
};
pub const EXT_ACQUIRE_DRM_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_acquire_drm_display") },
    spec: 1u32,
};
pub const EXT_ROBUSTNESS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_robustness2") },
    spec: 1u32,
};
pub const EXT_CUSTOM_BORDER_COLOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_custom_border_color") },
    spec: 12u32,
};
pub const GOOGLE_USER_TYPE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GOOGLE_user_type") },
    spec: 1u32,
};
pub const KHR_PIPELINE_LIBRARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_pipeline_library") },
    spec: 1u32,
};
pub const NV_PRESENT_BARRIER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_present_barrier") },
    spec: 1u32,
};
pub const KHR_SHADER_NON_SEMANTIC_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_non_semantic_info") },
    spec: 1u32,
};
pub const KHR_PRESENT_ID: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_id") },
    spec: 1u32,
};
pub const EXT_PRIVATE_DATA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_private_data") },
    spec: 1u32,
};
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_creation_cache_control") },
    spec: 3u32,
};
pub const NV_DEVICE_DIAGNOSTICS_CONFIG: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_diagnostics_config") },
    spec: 2u32,
};
pub const QCOM_RENDER_PASS_STORE_OPS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_render_pass_store_ops") },
    spec: 2u32,
};
pub const NV_CUDA_KERNEL_LAUNCH: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cuda_kernel_launch") },
    spec: 2u32,
};
pub const NV_LOW_LATENCY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_low_latency") },
    spec: 1u32,
};
pub const EXT_METAL_OBJECTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_metal_objects") },
    spec: 2u32,
};
pub const KHR_SYNCHRONIZATION2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_synchronization2") },
    spec: 1u32,
};
pub const EXT_DESCRIPTOR_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_descriptor_buffer") },
    spec: 1u32,
};
pub const EXT_GRAPHICS_PIPELINE_LIBRARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_graphics_pipeline_library") },
    spec: 1u32,
};
pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_early_and_late_fragment_tests") },
    spec: 1u32,
};
pub const KHR_FRAGMENT_SHADER_BARYCENTRIC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_fragment_shader_barycentric") },
    spec: 1u32,
};
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_subgroup_uniform_control_flow") },
    spec: 1u32,
};
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_zero_initialize_workgroup_memory") },
    spec: 1u32,
};
pub const NV_FRAGMENT_SHADING_RATE_ENUMS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_fragment_shading_rate_enums") },
    spec: 1u32,
};
pub const NV_RAY_TRACING_MOTION_BLUR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_motion_blur") },
    spec: 1u32,
};
pub const EXT_MESH_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_mesh_shader") },
    spec: 1u32,
};
pub const EXT_YCBCR_2PLANE_444_FORMATS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_ycbcr_2plane_444_formats") },
    spec: 1u32,
};
pub const EXT_FRAGMENT_DENSITY_MAP2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_fragment_density_map2") },
    spec: 1u32,
};
pub const QCOM_ROTATED_COPY_COMMANDS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_rotated_copy_commands") },
    spec: 2u32,
};
pub const EXT_IMAGE_ROBUSTNESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_robustness") },
    spec: 1u32,
};
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_workgroup_memory_explicit_layout") },
    spec: 1u32,
};
pub const KHR_COPY_COMMANDS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_copy_commands2") },
    spec: 1u32,
};
pub const EXT_IMAGE_COMPRESSION_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_compression_control") },
    spec: 1u32,
};
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_attachment_feedback_loop_layout") },
    spec: 2u32,
};
pub const EXT_4444_FORMATS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_4444_formats") },
    spec: 1u32,
};
pub const EXT_DEVICE_FAULT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_fault") },
    spec: 2u32,
};
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_rasterization_order_attachment_access") },
    spec: 1u32,
};
pub const EXT_RGBA10X6_FORMATS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_rgba10x6_formats") },
    spec: 1u32,
};
pub const NV_ACQUIRE_WINRT_DISPLAY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_acquire_winrt_display") },
    spec: 1u32,
};
pub const EXT_DIRECTFB_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_directfb_surface") },
    spec: 1u32,
};
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_mutable_descriptor_type") },
    spec: 1u32,
};
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_vertex_input_dynamic_state") },
    spec: 2u32,
};
pub const EXT_PHYSICAL_DEVICE_DRM: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_physical_device_drm") },
    spec: 1u32,
};
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_address_binding_report") },
    spec: 1u32,
};
pub const EXT_DEPTH_CLIP_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clip_control") },
    spec: 1u32,
};
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_primitive_topology_list_restart") },
    spec: 1u32,
};
pub const KHR_FORMAT_FEATURE_FLAGS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_format_feature_flags2") },
    spec: 2u32,
};
pub const FUCHSIA_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_FUCHSIA_external_memory") },
    spec: 1u32,
};
pub const FUCHSIA_EXTERNAL_SEMAPHORE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_FUCHSIA_external_semaphore") },
    spec: 1u32,
};
pub const FUCHSIA_BUFFER_COLLECTION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_FUCHSIA_buffer_collection") },
    spec: 2u32,
};
pub const HUAWEI_SUBPASS_SHADING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_subpass_shading") },
    spec: 3u32,
};
pub const HUAWEI_INVOCATION_MASK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_invocation_mask") },
    spec: 1u32,
};
pub const NV_EXTERNAL_MEMORY_RDMA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_memory_rdma") },
    spec: 1u32,
};
pub const EXT_PIPELINE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_properties") },
    spec: 1u32,
};
pub const EXT_FRAME_BOUNDARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_frame_boundary") },
    spec: 1u32,
};
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_multisampled_render_to_single_sampled") },
    spec: 1u32,
};
pub const EXT_EXTENDED_DYNAMIC_STATE2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_extended_dynamic_state2") },
    spec: 1u32,
};
pub const QNX_SCREEN_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_QNX_screen_surface") },
    spec: 1u32,
};
pub const EXT_COLOR_WRITE_ENABLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_color_write_enable") },
    spec: 1u32,
};
pub const EXT_PRIMITIVES_GENERATED_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_primitives_generated_query") },
    spec: 1u32,
};
pub const KHR_RAY_TRACING_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_tracing_maintenance1") },
    spec: 1u32,
};
pub const EXT_GLOBAL_PRIORITY_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_global_priority_query") },
    spec: 1u32,
};
pub const EXT_IMAGE_VIEW_MIN_LOD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_view_min_lod") },
    spec: 1u32,
};
pub const EXT_MULTI_DRAW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_multi_draw") },
    spec: 1u32,
};
pub const EXT_IMAGE_2D_VIEW_OF_3D: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_2d_view_of_3d") },
    spec: 1u32,
};
pub const KHR_PORTABILITY_ENUMERATION: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_portability_enumeration") },
    spec: 1u32,
};
pub const EXT_SHADER_TILE_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_tile_image") },
    spec: 1u32,
};
pub const EXT_OPACITY_MICROMAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_opacity_micromap") },
    spec: 2u32,
};
pub const NV_DISPLACEMENT_MICROMAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_displacement_micromap") },
    spec: 2u32,
};
pub const EXT_LOAD_STORE_OP_NONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_load_store_op_none") },
    spec: 1u32,
};
pub const HUAWEI_CLUSTER_CULLING_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_cluster_culling_shader") },
    spec: 3u32,
};
pub const EXT_BORDER_COLOR_SWIZZLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_border_color_swizzle") },
    spec: 1u32,
};
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pageable_device_local_memory") },
    spec: 1u32,
};
pub const KHR_MAINTENANCE4: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance4") },
    spec: 2u32,
};
pub const ARM_SHADER_CORE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_shader_core_properties") },
    spec: 1u32,
};
pub const KHR_SHADER_SUBGROUP_ROTATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_subgroup_rotate") },
    spec: 2u32,
};
pub const ARM_SCHEDULING_CONTROLS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_scheduling_controls") },
    spec: 1u32,
};
pub const EXT_IMAGE_SLICED_VIEW_OF_3D: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_sliced_view_of_3d") },
    spec: 1u32,
};
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_descriptor_set_host_mapping") },
    spec: 1u32,
};
pub const EXT_DEPTH_CLAMP_ZERO_ONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clamp_zero_one") },
    spec: 1u32,
};
pub const EXT_NON_SEAMLESS_CUBE_MAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_non_seamless_cube_map") },
    spec: 1u32,
};
pub const ARM_RENDER_PASS_STRIPED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_render_pass_striped") },
    spec: 1u32,
};
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_fragment_density_map_offset") },
    spec: 2u32,
};
pub const NV_COPY_MEMORY_INDIRECT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_copy_memory_indirect") },
    spec: 1u32,
};
pub const NV_MEMORY_DECOMPRESSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_memory_decompression") },
    spec: 1u32,
};
pub const NV_DEVICE_GENERATED_COMMANDS_COMPUTE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_generated_commands_compute") },
    spec: 2u32,
};
pub const NV_LINEAR_COLOR_ATTACHMENT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_linear_color_attachment") },
    spec: 1u32,
};
pub const GOOGLE_SURFACELESS_QUERY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_GOOGLE_surfaceless_query") },
    spec: 2u32,
};
pub const KHR_SHADER_MAXIMAL_RECONVERGENCE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_maximal_reconvergence") },
    spec: 1u32,
};
pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_compression_control_swapchain") },
    spec: 1u32,
};
pub const QCOM_IMAGE_PROCESSING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_image_processing") },
    spec: 1u32,
};
pub const EXT_NESTED_COMMAND_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_nested_command_buffer") },
    spec: 1u32,
};
pub const EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_acquire_unmodified") },
    spec: 1u32,
};
pub const EXT_EXTENDED_DYNAMIC_STATE3: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_extended_dynamic_state3") },
    spec: 2u32,
};
pub const EXT_SUBPASS_MERGE_FEEDBACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_subpass_merge_feedback") },
    spec: 2u32,
};
pub const LUNARG_DIRECT_DRIVER_LOADING: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_LUNARG_direct_driver_loading") },
    spec: 1u32,
};
pub const EXT_SHADER_MODULE_IDENTIFIER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_module_identifier") },
    spec: 1u32,
};
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_rasterization_order_attachment_access") },
    spec: 1u32,
};
pub const NV_OPTICAL_FLOW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_optical_flow") },
    spec: 1u32,
};
pub const EXT_LEGACY_DITHERING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_legacy_dithering") },
    spec: 2u32,
};
pub const EXT_PIPELINE_PROTECTED_ACCESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_protected_access") },
    spec: 1u32,
};
pub const ANDROID_EXTERNAL_FORMAT_RESOLVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ANDROID_external_format_resolve") },
    spec: 1u32,
};
pub const KHR_MAINTENANCE5: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance5") },
    spec: 1u32,
};
pub const AMD_ANTI_LAG: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_anti_lag") },
    spec: 1u32,
};
pub const KHR_RAY_TRACING_POSITION_FETCH: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_tracing_position_fetch") },
    spec: 1u32,
};
pub const EXT_SHADER_OBJECT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_object") },
    spec: 1u32,
};
pub const QCOM_TILE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_tile_properties") },
    spec: 1u32,
};
pub const SEC_AMIGO_PROFILING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_SEC_amigo_profiling") },
    spec: 1u32,
};
pub const QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_multiview_per_view_viewports") },
    spec: 1u32,
};
pub const NV_RAY_TRACING_INVOCATION_REORDER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_invocation_reorder") },
    spec: 1u32,
};
pub const NV_EXTENDED_SPARSE_ADDRESS_SPACE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_extended_sparse_address_space") },
    spec: 1u32,
};
pub const EXT_MUTABLE_DESCRIPTOR_TYPE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_mutable_descriptor_type") },
    spec: 1u32,
};
pub const EXT_LEGACY_VERTEX_ATTRIBUTES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_legacy_vertex_attributes") },
    spec: 1u32,
};
pub const EXT_LAYER_SETTINGS: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_layer_settings") },
    spec: 2u32,
};
pub const ARM_SHADER_CORE_BUILTINS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_shader_core_builtins") },
    spec: 2u32,
};
pub const EXT_PIPELINE_LIBRARY_GROUP_HANDLES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_library_group_handles") },
    spec: 1u32,
};
pub const EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_dynamic_rendering_unused_attachments") },
    spec: 1u32,
};
pub const NV_LOW_LATENCY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_low_latency2") },
    spec: 2u32,
};
pub const KHR_COOPERATIVE_MATRIX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_cooperative_matrix") },
    spec: 2u32,
};
pub const QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_multiview_per_view_render_areas") },
    spec: 1u32,
};
pub const NV_PER_STAGE_DESCRIPTOR_SET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_per_stage_descriptor_set") },
    spec: 1u32,
};
pub const QCOM_IMAGE_PROCESSING2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_image_processing2") },
    spec: 1u32,
};
pub const QCOM_FILTER_CUBIC_WEIGHTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_filter_cubic_weights") },
    spec: 1u32,
};
pub const QCOM_YCBCR_DEGAMMA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_ycbcr_degamma") },
    spec: 1u32,
};
pub const QCOM_FILTER_CUBIC_CLAMP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_filter_cubic_clamp") },
    spec: 1u32,
};
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_attachment_feedback_loop_dynamic_state") },
    spec: 1u32,
};
pub const KHR_VERTEX_ATTRIBUTE_DIVISOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_vertex_attribute_divisor") },
    spec: 1u32,
};
pub const KHR_LOAD_STORE_OP_NONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_load_store_op_none") },
    spec: 1u32,
};
pub const KHR_SHADER_FLOAT_CONTROLS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_float_controls2") },
    spec: 1u32,
};
pub const QNX_EXTERNAL_MEMORY_SCREEN_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QNX_external_memory_screen_buffer") },
    spec: 1u32,
};
pub const MSFT_LAYERED_DRIVER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_MSFT_layered_driver") },
    spec: 1u32,
};
pub const KHR_INDEX_TYPE_UINT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_index_type_uint8") },
    spec: 1u32,
};
pub const KHR_LINE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_line_rasterization") },
    spec: 1u32,
};
pub const KHR_CALIBRATED_TIMESTAMPS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_calibrated_timestamps") },
    spec: 1u32,
};
pub const KHR_SHADER_EXPECT_ASSUME: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_expect_assume") },
    spec: 1u32,
};
pub const KHR_MAINTENANCE6: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance6") },
    spec: 1u32,
};
pub const NV_DESCRIPTOR_POOL_OVERALLOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_descriptor_pool_overallocation") },
    spec: 1u32,
};
pub const NV_RAW_ACCESS_CHAINS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_raw_access_chains") },
    spec: 1u32,
};
pub const KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_relaxed_extended_instruction") },
    spec: 1u32,
};
pub const KHR_MAINTENANCE7: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance7") },
    spec: 1u32,
};
pub const NV_SHADER_ATOMIC_FLOAT16_VECTOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shader_atomic_float16_vector") },
    spec: 1u32,
};
pub const EXT_SHADER_REPLICATED_COMPOSITES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_replicated_composites") },
    spec: 1u32,
};
pub const NV_RAY_TRACING_VALIDATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_validation") },
    spec: 1u32,
};
pub const MESA_IMAGE_ALIGNMENT_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_MESA_image_alignment_control") },
    spec: 1u32,
};
