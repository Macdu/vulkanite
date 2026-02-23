use super::{
    ApiVersion, DeviceExtension, DeviceExtensionName, InstanceExtension, InstanceExtensionName,
};
pub const HEADER_VERSION: ApiVersion = ApiVersion::new(0, 1, 4, 344u32);
#[cfg(feature = "ext_surface")]
pub const KHR_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_surface") },
    spec: 25u32,
};
#[cfg(feature = "ext_swapchain")]
pub const KHR_SWAPCHAIN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_swapchain") },
    spec: 70u32,
};
#[cfg(all(feature = "ext_swapchain", not(feature = "ext_surface")))]
compile_error!("The feature ext_swapchain requires ext_surface to be enabled.");
#[cfg(feature = "ext_display")]
pub const KHR_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_display") },
    spec: 23u32,
};
#[cfg(all(feature = "ext_display", not(feature = "ext_surface")))]
compile_error!("The feature ext_display requires ext_surface to be enabled.");
#[cfg(feature = "ext_display_swapchain")]
pub const KHR_DISPLAY_SWAPCHAIN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_display_swapchain") },
    spec: 10u32,
};
#[cfg(all(
    feature = "ext_display_swapchain",
    not(all(feature = "ext_swapchain", feature = "ext_display"))
))]
compile_error!(
    "The feature ext_display_swapchain requires (ext_swapchain and ext_display) to be enabled."
);
#[cfg(feature = "ext_xlib_surface")]
pub const KHR_XLIB_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_xlib_surface") },
    spec: 6u32,
};
#[cfg(all(feature = "ext_xlib_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_xlib_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_xcb_surface")]
pub const KHR_XCB_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_xcb_surface") },
    spec: 6u32,
};
#[cfg(all(feature = "ext_xcb_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_xcb_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_wayland_surface")]
pub const KHR_WAYLAND_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_wayland_surface") },
    spec: 6u32,
};
#[cfg(all(feature = "ext_wayland_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_wayland_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_android_surface")]
pub const KHR_ANDROID_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_android_surface") },
    spec: 6u32,
};
#[cfg(all(feature = "ext_android_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_android_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_win32_surface")]
pub const KHR_WIN32_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_win32_surface") },
    spec: 6u32,
};
#[cfg(all(feature = "ext_win32_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_win32_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_debug_report")]
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
#[cfg(feature = "ext_filter_cubic")]
pub const IMG_FILTER_CUBIC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_IMG_filter_cubic") },
    spec: 1u32,
};
#[cfg(feature = "ext_rasterization_order")]
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
#[cfg(feature = "ext_debug_marker")]
pub const EXT_DEBUG_MARKER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_debug_marker") },
    spec: 4u32,
};
#[cfg(all(feature = "ext_debug_marker", not(feature = "ext_debug_report")))]
compile_error!("The feature ext_debug_marker requires ext_debug_report to be enabled.");
pub const AMD_GCN_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_gcn_shader") },
    spec: 1u32,
};
#[cfg(feature = "ext_dedicated_allocation")]
pub const NV_DEDICATED_ALLOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_dedicated_allocation") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_dedicated_allocation",
    not(any(feature = "ext_get_memory_requirements2", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_dedicated_allocation requires (ext_get_memory_requirements2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_transform_feedback")]
pub const EXT_TRANSFORM_FEEDBACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_transform_feedback") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_transform_feedback",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_transform_feedback requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_binary_import")]
pub const NVX_BINARY_IMPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NVX_binary_import") },
    spec: 2u32,
};
#[cfg(feature = "ext_image_view_handle")]
pub const NVX_IMAGE_VIEW_HANDLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NVX_image_view_handle") },
    spec: 4u32,
};
#[cfg(feature = "ext_draw_indirect_count")]
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
#[cfg(feature = "ext_shader_info")]
pub const AMD_SHADER_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_info") },
    spec: 1u32,
};
#[cfg(feature = "ext_dynamic_rendering")]
pub const KHR_DYNAMIC_RENDERING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_dynamic_rendering") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_dynamic_rendering",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_depth_stencil_resolve"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_dynamic_rendering requires (((ext_get_physical_device_properties2 or version_1_1) and ext_depth_stencil_resolve) or version_1_2) to be enabled.") ;
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_image_load_store_lod") },
    spec: 1u32,
};
#[cfg(feature = "ext_stream_descriptor_surface")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_GGP_stream_descriptor_surface") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_stream_descriptor_surface",
    not(feature = "ext_surface")
))]
compile_error!("The feature ext_stream_descriptor_surface requires ext_surface to be enabled.");
pub const NV_CORNER_SAMPLED_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_corner_sampled_image") },
    spec: 2u32,
};
#[cfg(feature = "ext_multiview")]
pub const KHR_MULTIVIEW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_multiview") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_multiview",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_multiview requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const IMG_FORMAT_PVRTC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_IMG_format_pvrtc") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_capabilities")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_NV_external_memory_capabilities") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_capabilities",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_external_memory_capabilities requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_external_memory")]
pub const NV_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_memory") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory",
    not(any(feature = "ext_external_memory_capabilities", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory requires (ext_external_memory_capabilities or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_external_memory_win32")]
pub const NV_EXTERNAL_MEMORY_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_memory_win32") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_win32",
    not(any(feature = "ext_external_memory", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory_win32 requires (ext_external_memory or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_win32_keyed_mutex")]
pub const NV_WIN32_KEYED_MUTEX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_win32_keyed_mutex") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_win32_keyed_mutex",
    not(feature = "ext_external_memory_win32")
))]
compile_error!(
    "The feature ext_win32_keyed_mutex requires ext_external_memory_win32 to be enabled."
);
#[cfg(feature = "ext_get_physical_device_properties2")]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES2: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_get_physical_device_properties2") },
    spec: 2u32,
};
#[cfg(feature = "ext_device_group")]
pub const KHR_DEVICE_GROUP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_device_group") },
    spec: 4u32,
};
#[cfg(all(
    feature = "ext_device_group",
    not(feature = "ext_device_group_creation")
))]
compile_error!("The feature ext_device_group requires ext_device_group_creation to be enabled.");
#[cfg(feature = "ext_validation_flags")]
pub const EXT_VALIDATION_FLAGS: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_validation_flags") },
    spec: 3u32,
};
#[cfg(feature = "ext_vi_surface")]
pub const NN_VI_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_NN_vi_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_vi_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_vi_surface requires ext_surface to be enabled.");
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
#[cfg(feature = "ext_astc_decode_mode")]
pub const EXT_ASTC_DECODE_MODE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_astc_decode_mode") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_astc_decode_mode",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_astc_decode_mode requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_pipeline_robustness")]
pub const EXT_PIPELINE_ROBUSTNESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_robustness") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_pipeline_robustness",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_pipeline_robustness requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_maintenance1")]
pub const KHR_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance1") },
    spec: 2u32,
};
#[cfg(feature = "ext_device_group_creation")]
pub const KHR_DEVICE_GROUP_CREATION: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_device_group_creation") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_capabilities")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_external_memory_capabilities") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory")]
pub const KHR_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_memory") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_win32")]
pub const KHR_EXTERNAL_MEMORY_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_memory_win32") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_fd")]
pub const KHR_EXTERNAL_MEMORY_FD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_memory_fd") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_fd",
    not(any(feature = "ext_external_memory", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory_fd requires (ext_external_memory or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_win32_keyed_mutex")]
pub const KHR_WIN32_KEYED_MUTEX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_win32_keyed_mutex") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_semaphore_capabilities")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_external_semaphore_capabilities") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_semaphore_capabilities",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_external_semaphore_capabilities requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_external_semaphore")]
pub const KHR_EXTERNAL_SEMAPHORE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_semaphore") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_semaphore",
    not(feature = "ext_external_semaphore_capabilities")
))]
compile_error ! ("The feature ext_external_semaphore requires ext_external_semaphore_capabilities to be enabled.") ;
#[cfg(feature = "ext_external_semaphore_win32")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_semaphore_win32") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_semaphore_win32",
    not(feature = "ext_external_semaphore")
))]
compile_error!(
    "The feature ext_external_semaphore_win32 requires ext_external_semaphore to be enabled."
);
#[cfg(feature = "ext_external_semaphore_fd")]
pub const KHR_EXTERNAL_SEMAPHORE_FD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_semaphore_fd") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_semaphore_fd",
    not(any(feature = "ext_external_semaphore", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_semaphore_fd requires (ext_external_semaphore or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_push_descriptor")]
pub const KHR_PUSH_DESCRIPTOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_push_descriptor") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_push_descriptor",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_push_descriptor requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_conditional_rendering")]
pub const EXT_CONDITIONAL_RENDERING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_conditional_rendering") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_conditional_rendering",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_conditional_rendering requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_SHADER_FLOAT16_INT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_float16_int8") },
    spec: 1u32,
};
pub const KHR_16BIT_STORAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_16bit_storage") },
    spec: 1u32,
};
#[cfg(feature = "ext_incremental_present")]
pub const KHR_INCREMENTAL_PRESENT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_incremental_present") },
    spec: 2u32,
};
#[cfg(all(feature = "ext_incremental_present", not(feature = "ext_swapchain")))]
compile_error!("The feature ext_incremental_present requires ext_swapchain to be enabled.");
#[cfg(feature = "ext_descriptor_update_template")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_descriptor_update_template") },
    spec: 1u32,
};
#[cfg(feature = "ext_clip_space_w_scaling")]
pub const NV_CLIP_SPACE_W_SCALING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_clip_space_w_scaling") },
    spec: 1u32,
};
#[cfg(feature = "ext_direct_mode_display")]
pub const EXT_DIRECT_MODE_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_direct_mode_display") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_direct_mode_display", not(feature = "ext_display")))]
compile_error!("The feature ext_direct_mode_display requires ext_display to be enabled.");
#[cfg(feature = "ext_acquire_xlib_display")]
pub const EXT_ACQUIRE_XLIB_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_acquire_xlib_display") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_acquire_xlib_display",
    not(feature = "ext_direct_mode_display")
))]
compile_error!(
    "The feature ext_acquire_xlib_display requires ext_direct_mode_display to be enabled."
);
#[cfg(feature = "ext_display_surface_counter")]
pub const EXT_DISPLAY_SURFACE_COUNTER: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_display_surface_counter") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_display_surface_counter", not(feature = "ext_display")))]
compile_error!("The feature ext_display_surface_counter requires ext_display to be enabled.");
#[cfg(feature = "ext_display_control")]
pub const EXT_DISPLAY_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_display_control") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_display_control",
    not(all(feature = "ext_display_surface_counter", feature = "ext_swapchain"))
))]
compile_error ! ("The feature ext_display_control requires (ext_display_surface_counter and ext_swapchain) to be enabled.") ;
#[cfg(feature = "ext_display_timing")]
pub const GOOGLE_DISPLAY_TIMING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GOOGLE_display_timing") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_display_timing", not(feature = "ext_swapchain")))]
compile_error!("The feature ext_display_timing requires ext_swapchain to be enabled.");
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
#[cfg(feature = "ext_multiview_per_view_attributes")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NVX_multiview_per_view_attributes") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_multiview_per_view_attributes",
    not(any(feature = "ext_multiview", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_multiview_per_view_attributes requires (ext_multiview or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_viewport_swizzle")]
pub const NV_VIEWPORT_SWIZZLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_viewport_swizzle") },
    spec: 1u32,
};
#[cfg(feature = "ext_discard_rectangles")]
pub const EXT_DISCARD_RECTANGLES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_discard_rectangles") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_discard_rectangles",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_discard_rectangles requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_conservative_rasterization")]
pub const EXT_CONSERVATIVE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_conservative_rasterization") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_conservative_rasterization",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_conservative_rasterization requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_depth_clip_enable")]
pub const EXT_DEPTH_CLIP_ENABLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clip_enable") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_depth_clip_enable",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_depth_clip_enable requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_SWAPCHAIN_COLORSPACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_swapchain_colorspace") },
    spec: 5u32,
};
#[cfg(feature = "ext_hdr_metadata")]
pub const EXT_HDR_METADATA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_hdr_metadata") },
    spec: 3u32,
};
#[cfg(all(feature = "ext_hdr_metadata", not(feature = "ext_swapchain")))]
compile_error!("The feature ext_hdr_metadata requires ext_swapchain to be enabled.");
#[cfg(feature = "ext_imageless_framebuffer")]
pub const KHR_IMAGELESS_FRAMEBUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_imageless_framebuffer") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_imageless_framebuffer",
    not(any(
        all(
            any(
                all(
                    feature = "ext_get_physical_device_properties2",
                    feature = "ext_maintenance2"
                ),
                feature = "version_1_1"
            ),
            feature = "ext_image_format_list"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_imageless_framebuffer requires ((((ext_get_physical_device_properties2 and ext_maintenance2) or version_1_1) and ext_image_format_list) or version_1_2) to be enabled.") ;
#[cfg(feature = "ext_create_renderpass2")]
pub const KHR_CREATE_RENDERPASS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_create_renderpass2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_create_renderpass2",
    not(any(
        all(feature = "ext_multiview", feature = "ext_maintenance2"),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_create_renderpass2 requires ((ext_multiview and ext_maintenance2) or version_1_1) to be enabled.") ;
pub const IMG_RELAXED_LINE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_IMG_relaxed_line_rasterization") },
    spec: 1u32,
};
#[cfg(feature = "ext_shared_presentable_image")]
pub const KHR_SHARED_PRESENTABLE_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shared_presentable_image") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_shared_presentable_image",
    not(all(
        feature = "ext_swapchain",
        feature = "ext_get_surface_capabilities2",
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        )
    ))
))]
compile_error ! ("The feature ext_shared_presentable_image requires (ext_swapchain and ext_get_surface_capabilities2 and (ext_get_physical_device_properties2 or version_1_1)) to be enabled.") ;
#[cfg(feature = "ext_external_fence_capabilities")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_external_fence_capabilities") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_fence_capabilities",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_external_fence_capabilities requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_external_fence")]
pub const KHR_EXTERNAL_FENCE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_fence") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_fence",
    not(feature = "ext_external_fence_capabilities")
))]
compile_error!(
    "The feature ext_external_fence requires ext_external_fence_capabilities to be enabled."
);
#[cfg(feature = "ext_external_fence_win32")]
pub const KHR_EXTERNAL_FENCE_WIN32: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_fence_win32") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_fence_win32",
    not(feature = "ext_external_fence")
))]
compile_error!("The feature ext_external_fence_win32 requires ext_external_fence to be enabled.");
#[cfg(feature = "ext_external_fence_fd")]
pub const KHR_EXTERNAL_FENCE_FD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_external_fence_fd") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_fence_fd",
    not(any(feature = "ext_external_fence", feature = "version_1_1"))
))]
compile_error!(
    "The feature ext_external_fence_fd requires (ext_external_fence or version_1_1) to be enabled."
);
#[cfg(feature = "ext_performance_query")]
pub const KHR_PERFORMANCE_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_performance_query") },
    spec: 1u32,
};
#[cfg(feature = "ext_maintenance2")]
pub const KHR_MAINTENANCE2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance2") },
    spec: 1u32,
};
#[cfg(feature = "ext_get_surface_capabilities2")]
pub const KHR_GET_SURFACE_CAPABILITIES2: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_get_surface_capabilities2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_get_surface_capabilities2",
    not(feature = "ext_surface")
))]
compile_error!("The feature ext_get_surface_capabilities2 requires ext_surface to be enabled.");
pub const KHR_VARIABLE_POINTERS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_variable_pointers") },
    spec: 1u32,
};
#[cfg(feature = "ext_get_display_properties2")]
pub const KHR_GET_DISPLAY_PROPERTIES2: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_get_display_properties2") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_get_display_properties2", not(feature = "ext_display")))]
compile_error!("The feature ext_get_display_properties2 requires ext_display to be enabled.");
#[cfg(feature = "ext_ios_surface")]
pub const MVK_IOS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_MVK_ios_surface") },
    spec: 3u32,
};
#[cfg(all(feature = "ext_ios_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_ios_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_macos_surface")]
pub const MVK_MACOS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_MVK_macos_surface") },
    spec: 3u32,
};
#[cfg(all(feature = "ext_macos_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_macos_surface requires ext_surface to be enabled.");
pub const EXT_EXTERNAL_MEMORY_DMA_BUF: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_dma_buf") },
    spec: 1u32,
};
pub const EXT_QUEUE_FAMILY_FOREIGN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_queue_family_foreign") },
    spec: 1u32,
};
#[cfg(feature = "ext_dedicated_allocation")]
pub const KHR_DEDICATED_ALLOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_dedicated_allocation") },
    spec: 3u32,
};
#[cfg(feature = "ext_debug_utils")]
pub const EXT_DEBUG_UTILS: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_debug_utils") },
    spec: 2u32,
};
#[cfg(feature = "ext_external_memory_android_hardware_buffer")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe {
        DeviceExtensionName::new(c"VK_ANDROID_external_memory_android_hardware_buffer")
    },
    spec: 5u32,
};
#[cfg(all(
    feature = "ext_external_memory_android_hardware_buffer",
    not(all(any(
        all(
            feature = "ext_sampler_ycbcr_conversion",
            feature = "ext_external_memory",
            feature = "ext_dedicated_allocation"
        ),
        feature = "version_1_1"
    )))
))]
compile_error ! ("The feature ext_external_memory_android_hardware_buffer requires (((ext_sampler_ycbcr_conversion and ext_external_memory and ext_dedicated_allocation) or version_1_1) and ext_queue_family_foreign) to be enabled.") ;
#[cfg(feature = "ext_sampler_filter_minmax")]
pub const EXT_SAMPLER_FILTER_MINMAX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_sampler_filter_minmax") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_sampler_filter_minmax",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_sampler_filter_minmax requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_storage_buffer_storage_class") },
    spec: 1u32,
};
pub const AMD_GPU_SHADER_INT16: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_gpu_shader_int16") },
    spec: 2u32,
};
#[cfg(feature = "ext_shader_enqueue")]
pub const AMDX_SHADER_ENQUEUE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMDX_shader_enqueue") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_shader_enqueue",
    not(all(
        any(
            all(
                feature = "ext_synchronization2",
                feature = "ext_extended_dynamic_state"
            ),
            feature = "version_1_3"
        ),
        feature = "ext_maintenance5",
        feature = "ext_pipeline_library"
    ))
))]
compile_error ! ("The feature ext_shader_enqueue requires (((ext_synchronization2 and ext_spirv_1_4 and ext_extended_dynamic_state) or version_1_3) and ext_maintenance5 and ext_pipeline_library) to be enabled.") ;
#[cfg(feature = "ext_descriptor_heap")]
pub const EXT_DESCRIPTOR_HEAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_descriptor_heap") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_descriptor_heap",
    not(all(
        feature = "ext_maintenance5",
        any(feature = "ext_buffer_device_address", feature = "version_1_2")
    ))
))]
compile_error ! ("The feature ext_descriptor_heap requires (ext_maintenance5 and (ext_buffer_device_address or version_1_2)) to be enabled.") ;
#[cfg(feature = "ext_mixed_attachment_samples")]
pub const AMD_MIXED_ATTACHMENT_SAMPLES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_mixed_attachment_samples") },
    spec: 1u32,
};
pub const AMD_SHADER_FRAGMENT_MASK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_fragment_mask") },
    spec: 1u32,
};
#[cfg(feature = "ext_inline_uniform_block")]
pub const EXT_INLINE_UNIFORM_BLOCK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_inline_uniform_block") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_inline_uniform_block",
    not(any(
        all(
            feature = "ext_get_physical_device_properties2",
            feature = "ext_maintenance1"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_inline_uniform_block requires ((ext_get_physical_device_properties2 and ext_maintenance1) or version_1_1) to be enabled.") ;
pub const EXT_SHADER_STENCIL_EXPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_stencil_export") },
    spec: 1u32,
};
pub const KHR_SHADER_BFLOAT16: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_bfloat16") },
    spec: 1u32,
};
#[cfg(feature = "ext_sample_locations")]
pub const EXT_SAMPLE_LOCATIONS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_sample_locations") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_sample_locations",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_sample_locations requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_RELAXED_BLOCK_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_relaxed_block_layout") },
    spec: 1u32,
};
#[cfg(feature = "ext_get_memory_requirements2")]
pub const KHR_GET_MEMORY_REQUIREMENTS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_get_memory_requirements2") },
    spec: 1u32,
};
#[cfg(feature = "ext_image_format_list")]
pub const KHR_IMAGE_FORMAT_LIST: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_image_format_list") },
    spec: 1u32,
};
#[cfg(feature = "ext_blend_operation_advanced")]
pub const EXT_BLEND_OPERATION_ADVANCED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_blend_operation_advanced") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_blend_operation_advanced",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_blend_operation_advanced requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_fragment_coverage_to_color")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_fragment_coverage_to_color") },
    spec: 1u32,
};
#[cfg(feature = "ext_acceleration_structure")]
pub const KHR_ACCELERATION_STRUCTURE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_acceleration_structure") },
    spec: 13u32,
};
#[cfg(all(
    feature = "ext_acceleration_structure",
    not(all(
        any(
            all(
                feature = "version_1_1",
                feature = "ext_descriptor_indexing",
                feature = "ext_buffer_device_address"
            ),
            feature = "version_1_2"
        ),
        feature = "ext_deferred_host_operations"
    ))
))]
compile_error ! ("The feature ext_acceleration_structure requires (((version_1_1 and ext_descriptor_indexing and ext_buffer_device_address) or version_1_2) and ext_deferred_host_operations) to be enabled.") ;
#[cfg(feature = "ext_ray_tracing_pipeline")]
pub const KHR_RAY_TRACING_PIPELINE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_tracing_pipeline") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ray_tracing_pipeline",
    not(all(feature = "ext_acceleration_structure"))
))]
compile_error ! ("The feature ext_ray_tracing_pipeline requires ((ext_spirv_1_4 or version_1_2) and ext_acceleration_structure) to be enabled.") ;
pub const KHR_RAY_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_query") },
    spec: 1u32,
};
#[cfg(feature = "ext_framebuffer_mixed_samples")]
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
#[cfg(feature = "ext_sampler_ycbcr_conversion")]
pub const KHR_SAMPLER_YCBCR_CONVERSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_sampler_ycbcr_conversion") },
    spec: 14u32,
};
#[cfg(all(
    feature = "ext_sampler_ycbcr_conversion",
    not(any(
        all(
            feature = "ext_maintenance1",
            feature = "ext_bind_memory2",
            feature = "ext_get_memory_requirements2",
            feature = "ext_get_physical_device_properties2"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_sampler_ycbcr_conversion requires ((ext_maintenance1 and ext_bind_memory2 and ext_get_memory_requirements2 and ext_get_physical_device_properties2) or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_bind_memory2")]
pub const KHR_BIND_MEMORY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_bind_memory2") },
    spec: 1u32,
};
#[cfg(feature = "ext_image_drm_format_modifier")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_drm_format_modifier") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_image_drm_format_modifier",
    not(any(
        all(
            any(
                all(
                    feature = "ext_bind_memory2",
                    feature = "ext_get_physical_device_properties2",
                    feature = "ext_sampler_ycbcr_conversion"
                ),
                feature = "version_1_1"
            ),
            feature = "ext_image_format_list"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_image_drm_format_modifier requires ((((ext_bind_memory2 and ext_get_physical_device_properties2 and ext_sampler_ycbcr_conversion) or version_1_1) and ext_image_format_list) or version_1_2) to be enabled.") ;
#[cfg(feature = "ext_validation_cache")]
pub const EXT_VALIDATION_CACHE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_validation_cache") },
    spec: 1u32,
};
#[cfg(feature = "ext_descriptor_indexing")]
pub const EXT_DESCRIPTOR_INDEXING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_descriptor_indexing") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_descriptor_indexing",
    not(any(
        all(
            feature = "ext_get_physical_device_properties2",
            feature = "ext_maintenance3"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_descriptor_indexing requires ((ext_get_physical_device_properties2 and ext_maintenance3) or version_1_1) to be enabled.") ;
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_viewport_index_layer") },
    spec: 1u32,
};
pub const KHR_PORTABILITY_SUBSET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_portability_subset") },
    spec: 1u32,
};
#[cfg(feature = "ext_shading_rate_image")]
pub const NV_SHADING_RATE_IMAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shading_rate_image") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_shading_rate_image",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_shading_rate_image requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_ray_tracing")]
pub const NV_RAY_TRACING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_ray_tracing",
    not(any(
        all(
            feature = "ext_get_physical_device_properties2",
            feature = "ext_get_memory_requirements2"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_ray_tracing requires ((ext_get_physical_device_properties2 and ext_get_memory_requirements2) or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_representative_fragment_test")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_representative_fragment_test") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_representative_fragment_test",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_representative_fragment_test requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_maintenance3")]
pub const KHR_MAINTENANCE3: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance3") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_maintenance3",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_maintenance3 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_draw_indirect_count")]
pub const KHR_DRAW_INDIRECT_COUNT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_draw_indirect_count") },
    spec: 1u32,
};
#[cfg(feature = "ext_filter_cubic")]
pub const EXT_FILTER_CUBIC: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_filter_cubic") },
    spec: 3u32,
};
pub const QCOM_RENDER_PASS_SHADER_RESOLVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_render_pass_shader_resolve") },
    spec: 4u32,
};
pub const QCOM_COOPERATIVE_MATRIX_CONVERSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_cooperative_matrix_conversion") },
    spec: 1u32,
};
#[cfg(feature = "ext_global_priority")]
pub const EXT_GLOBAL_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_global_priority") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_global_priority",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_global_priority requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_subgroup_extended_types") },
    spec: 1u32,
};
pub const KHR_8BIT_STORAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_8bit_storage") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_host")]
pub const EXT_EXTERNAL_MEMORY_HOST: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_host") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_host",
    not(any(feature = "ext_external_memory", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory_host requires (ext_external_memory or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_buffer_marker")]
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
#[cfg(feature = "ext_pipeline_compiler_control")]
pub const AMD_PIPELINE_COMPILER_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_pipeline_compiler_control") },
    spec: 1u32,
};
#[cfg(feature = "ext_calibrated_timestamps")]
pub const EXT_CALIBRATED_TIMESTAMPS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_calibrated_timestamps") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_calibrated_timestamps",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_calibrated_timestamps requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const AMD_SHADER_CORE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_core_properties") },
    spec: 2u32,
};
#[cfg(feature = "ext_global_priority")]
pub const KHR_GLOBAL_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_global_priority") },
    spec: 1u32,
};
#[cfg(feature = "ext_memory_overallocation_behavior")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_memory_overallocation_behavior") },
    spec: 1u32,
};
#[cfg(feature = "ext_vertex_attribute_divisor")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_vertex_attribute_divisor") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_vertex_attribute_divisor",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_vertex_attribute_divisor requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_frame_token")]
pub const GGP_FRAME_TOKEN: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GGP_frame_token") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_frame_token",
    not(all(feature = "ext_swapchain", feature = "ext_stream_descriptor_surface"))
))]
compile_error ! ("The feature ext_frame_token requires (ext_swapchain and ext_stream_descriptor_surface) to be enabled.") ;
#[cfg(feature = "ext_pipeline_creation_feedback")]
pub const EXT_PIPELINE_CREATION_FEEDBACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_creation_feedback") },
    spec: 1u32,
};
#[cfg(feature = "ext_driver_properties")]
pub const KHR_DRIVER_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_driver_properties") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_driver_properties",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_driver_properties requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_shader_float_controls")]
pub const KHR_SHADER_FLOAT_CONTROLS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_float_controls") },
    spec: 4u32,
};
#[cfg(all(
    feature = "ext_shader_float_controls",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_shader_float_controls requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const NV_SHADER_SUBGROUP_PARTITIONED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shader_subgroup_partitioned") },
    spec: 1u32,
};
#[cfg(feature = "ext_depth_stencil_resolve")]
pub const KHR_DEPTH_STENCIL_RESOLVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_depth_stencil_resolve") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_depth_stencil_resolve",
    not(any(feature = "ext_create_renderpass2", feature = "version_1_2"))
))]
compile_error ! ("The feature ext_depth_stencil_resolve requires (ext_create_renderpass2 or version_1_2) to be enabled.") ;
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_swapchain_mutable_format") },
    spec: 1u32,
};
pub const NV_COMPUTE_SHADER_DERIVATIVES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_compute_shader_derivatives") },
    spec: 1u32,
};
#[cfg(feature = "ext_mesh_shader")]
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
#[cfg(feature = "ext_scissor_exclusive")]
pub const NV_SCISSOR_EXCLUSIVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_scissor_exclusive") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_scissor_exclusive",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_scissor_exclusive requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_device_diagnostic_checkpoints")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_diagnostic_checkpoints") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_device_diagnostic_checkpoints",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_device_diagnostic_checkpoints requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_timeline_semaphore")]
pub const KHR_TIMELINE_SEMAPHORE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_timeline_semaphore") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_timeline_semaphore",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_timeline_semaphore requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_present_timing")]
pub const EXT_PRESENT_TIMING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_present_timing") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_present_timing",
    not(all(
        feature = "ext_swapchain",
        feature = "ext_present_id2",
        feature = "ext_get_surface_capabilities2",
        feature = "ext_calibrated_timestamps"
    ))
))]
compile_error ! ("The feature ext_present_timing requires (ext_swapchain and ext_present_id2 and ext_get_surface_capabilities2 and ext_calibrated_timestamps) to be enabled.") ;
pub const INTEL_SHADER_INTEGER_FUNCTIONS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_INTEL_shader_integer_functions2") },
    spec: 1u32,
};
#[cfg(feature = "ext_performance_query")]
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
#[cfg(feature = "ext_display_native_hdr")]
pub const AMD_DISPLAY_NATIVE_HDR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_display_native_hdr") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_display_native_hdr",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_get_surface_capabilities2",
        feature = "ext_swapchain"
    ))
))]
compile_error ! ("The feature ext_display_native_hdr requires ((ext_get_physical_device_properties2 or version_1_1) and ext_get_surface_capabilities2 and ext_swapchain) to be enabled.") ;
#[cfg(feature = "ext_imagepipe_surface")]
pub const FUCHSIA_IMAGEPIPE_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_FUCHSIA_imagepipe_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_imagepipe_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_imagepipe_surface requires ext_surface to be enabled.");
pub const KHR_SHADER_TERMINATE_INVOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_terminate_invocation") },
    spec: 1u32,
};
#[cfg(feature = "ext_metal_surface")]
pub const EXT_METAL_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_metal_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_metal_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_metal_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_fragment_density_map")]
pub const EXT_FRAGMENT_DENSITY_MAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_fragment_density_map") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_fragment_density_map",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_fragment_density_map requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
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
#[cfg(feature = "ext_subgroup_size_control")]
pub const EXT_SUBGROUP_SIZE_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_subgroup_size_control") },
    spec: 2u32,
};
#[cfg(all(feature = "ext_subgroup_size_control", not(feature = "version_1_1")))]
compile_error!("The feature ext_subgroup_size_control requires version_1_1 to be enabled.");
#[cfg(feature = "ext_fragment_shading_rate")]
pub const KHR_FRAGMENT_SHADING_RATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_fragment_shading_rate") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_fragment_shading_rate",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_create_renderpass2"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_fragment_shading_rate requires (((ext_get_physical_device_properties2 or version_1_1) and ext_create_renderpass2) or version_1_2) to be enabled.") ;
#[cfg(feature = "ext_shader_core_properties2")]
pub const AMD_SHADER_CORE_PROPERTIES2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_shader_core_properties2") },
    spec: 1u32,
};
pub const AMD_DEVICE_COHERENT_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_device_coherent_memory") },
    spec: 1u32,
};
#[cfg(feature = "ext_dynamic_rendering_local_read")]
pub const KHR_DYNAMIC_RENDERING_LOCAL_READ: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_dynamic_rendering_local_read") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_dynamic_rendering_local_read",
    not(any(feature = "ext_dynamic_rendering", feature = "version_1_3"))
))]
compile_error ! ("The feature ext_dynamic_rendering_local_read requires (ext_dynamic_rendering or version_1_3) to be enabled.") ;
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
#[cfg(feature = "ext_memory_priority")]
pub const EXT_MEMORY_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_memory_priority") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_memory_priority",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_memory_priority requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_surface_protected_capabilities")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_surface_protected_capabilities") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_surface_protected_capabilities",
    not(all(feature = "version_1_1", feature = "ext_get_surface_capabilities2"))
))]
compile_error ! ("The feature ext_surface_protected_capabilities requires (version_1_1 and ext_get_surface_capabilities2) to be enabled.") ;
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_dedicated_allocation_image_aliasing") },
    spec: 1u32,
};
#[cfg(feature = "ext_separate_depth_stencil_layouts")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_separate_depth_stencil_layouts") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_separate_depth_stencil_layouts",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_create_renderpass2"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_separate_depth_stencil_layouts requires (((ext_get_physical_device_properties2 or version_1_1) and ext_create_renderpass2) or version_1_2) to be enabled.") ;
#[cfg(feature = "ext_buffer_device_address")]
pub const EXT_BUFFER_DEVICE_ADDRESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_buffer_device_address") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_buffer_device_address",
    not(any(
        all(
            feature = "ext_get_physical_device_properties2",
            feature = "ext_device_group"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_buffer_device_address requires ((ext_get_physical_device_properties2 and ext_device_group) or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_tooling_info")]
pub const EXT_TOOLING_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_tooling_info") },
    spec: 1u32,
};
#[cfg(feature = "ext_separate_stencil_usage")]
pub const EXT_SEPARATE_STENCIL_USAGE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_separate_stencil_usage") },
    spec: 1u32,
};
#[cfg(feature = "ext_validation_features")]
pub const EXT_VALIDATION_FEATURES: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_validation_features") },
    spec: 6u32,
};
#[cfg(feature = "ext_present_wait")]
pub const KHR_PRESENT_WAIT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_wait") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_present_wait",
    not(all(feature = "ext_swapchain", feature = "ext_present_id"))
))]
compile_error!(
    "The feature ext_present_wait requires (ext_swapchain and ext_present_id) to be enabled."
);
#[cfg(feature = "ext_cooperative_matrix")]
pub const NV_COOPERATIVE_MATRIX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cooperative_matrix") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_cooperative_matrix",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_cooperative_matrix requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_coverage_reduction_mode")]
pub const NV_COVERAGE_REDUCTION_MODE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_coverage_reduction_mode") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_coverage_reduction_mode",
    not(all(
        feature = "ext_framebuffer_mixed_samples",
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        )
    ))
))]
compile_error ! ("The feature ext_coverage_reduction_mode requires (ext_framebuffer_mixed_samples and (ext_get_physical_device_properties2 or version_1_1)) to be enabled.") ;
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
#[cfg(feature = "ext_provoking_vertex")]
pub const EXT_PROVOKING_VERTEX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_provoking_vertex") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_provoking_vertex",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_provoking_vertex requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_full_screen_exclusive")]
pub const EXT_FULL_SCREEN_EXCLUSIVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_full_screen_exclusive") },
    spec: 4u32,
};
#[cfg(all(
    feature = "ext_full_screen_exclusive",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_surface",
        feature = "ext_get_surface_capabilities2",
        feature = "ext_swapchain"
    ))
))]
compile_error ! ("The feature ext_full_screen_exclusive requires ((ext_get_physical_device_properties2 or version_1_1) and ext_surface and ext_get_surface_capabilities2 and ext_swapchain) to be enabled.") ;
#[cfg(feature = "ext_headless_surface")]
pub const EXT_HEADLESS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_headless_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_headless_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_headless_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_buffer_device_address")]
pub const KHR_BUFFER_DEVICE_ADDRESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_buffer_device_address") },
    spec: 1u32,
};
#[cfg(feature = "ext_line_rasterization")]
pub const EXT_LINE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_line_rasterization") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_line_rasterization",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_line_rasterization requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_SHADER_ATOMIC_FLOAT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_atomic_float") },
    spec: 1u32,
};
#[cfg(feature = "ext_host_query_reset")]
pub const EXT_HOST_QUERY_RESET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_host_query_reset") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_host_query_reset",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_host_query_reset requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_INDEX_TYPE_UINT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_index_type_uint8") },
    spec: 1u32,
};
#[cfg(feature = "ext_extended_dynamic_state")]
pub const EXT_EXTENDED_DYNAMIC_STATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_extended_dynamic_state") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_extended_dynamic_state",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_extended_dynamic_state requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_deferred_host_operations")]
pub const KHR_DEFERRED_HOST_OPERATIONS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_deferred_host_operations") },
    spec: 4u32,
};
#[cfg(feature = "ext_pipeline_executable_properties")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_pipeline_executable_properties") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_pipeline_executable_properties",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_pipeline_executable_properties requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_host_image_copy")]
pub const EXT_HOST_IMAGE_COPY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_host_image_copy") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_host_image_copy",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_copy_commands2",
            feature = "ext_format_feature_flags2"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_host_image_copy requires (((ext_get_physical_device_properties2 or version_1_1) and ext_copy_commands2 and ext_format_feature_flags2) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_map_memory2")]
pub const KHR_MAP_MEMORY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_map_memory2") },
    spec: 1u32,
};
#[cfg(feature = "ext_map_memory_placed")]
pub const EXT_MAP_MEMORY_PLACED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_map_memory_placed") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_map_memory_placed",
    not(any(feature = "ext_map_memory2", feature = "version_1_4"))
))]
compile_error!(
    "The feature ext_map_memory_placed requires (ext_map_memory2 or version_1_4) to be enabled."
);
pub const EXT_SHADER_ATOMIC_FLOAT2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_atomic_float2") },
    spec: 1u32,
};
#[cfg(feature = "ext_surface_maintenance1")]
pub const EXT_SURFACE_MAINTENANCE1: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_surface_maintenance1") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_surface_maintenance1",
    not(all(feature = "ext_surface", feature = "ext_get_surface_capabilities2"))
))]
compile_error ! ("The feature ext_surface_maintenance1 requires (ext_surface and ext_get_surface_capabilities2) to be enabled.") ;
#[cfg(feature = "ext_swapchain_maintenance1")]
pub const EXT_SWAPCHAIN_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_swapchain_maintenance1") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_swapchain_maintenance1",
    not(all(
        feature = "ext_swapchain",
        feature = "ext_surface_maintenance1",
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        )
    ))
))]
compile_error ! ("The feature ext_swapchain_maintenance1 requires (ext_swapchain and ext_surface_maintenance1 and (ext_get_physical_device_properties2 or version_1_1)) to be enabled.") ;
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_demote_to_helper_invocation") },
    spec: 1u32,
};
#[cfg(feature = "ext_device_generated_commands")]
pub const NV_DEVICE_GENERATED_COMMANDS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_generated_commands") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_device_generated_commands",
    not(any(
        all(
            any(feature = "ext_buffer_device_address", feature = "version_1_2"),
            feature = "ext_maintenance5"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_device_generated_commands requires (((ext_buffer_device_address or version_1_2) and ext_maintenance5) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_inherited_viewport_scissor")]
pub const NV_INHERITED_VIEWPORT_SCISSOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_inherited_viewport_scissor") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_inherited_viewport_scissor",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_inherited_viewport_scissor requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_SHADER_INTEGER_DOT_PRODUCT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_integer_dot_product") },
    spec: 1u32,
};
pub const EXT_TEXEL_BUFFER_ALIGNMENT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_texel_buffer_alignment") },
    spec: 1u32,
};
#[cfg(feature = "ext_render_pass_transform")]
pub const QCOM_RENDER_PASS_TRANSFORM: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_render_pass_transform") },
    spec: 5u32,
};
#[cfg(feature = "ext_depth_bias_control")]
pub const EXT_DEPTH_BIAS_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_bias_control") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_depth_bias_control",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_depth_bias_control requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_device_memory_report")]
pub const EXT_DEVICE_MEMORY_REPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_memory_report") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_device_memory_report",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_device_memory_report requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_acquire_drm_display")]
pub const EXT_ACQUIRE_DRM_DISPLAY: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_acquire_drm_display") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_acquire_drm_display",
    not(feature = "ext_direct_mode_display")
))]
compile_error!(
    "The feature ext_acquire_drm_display requires ext_direct_mode_display to be enabled."
);
pub const EXT_ROBUSTNESS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_robustness2") },
    spec: 1u32,
};
#[cfg(feature = "ext_custom_border_color")]
pub const EXT_CUSTOM_BORDER_COLOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_custom_border_color") },
    spec: 12u32,
};
#[cfg(all(
    feature = "ext_custom_border_color",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_custom_border_color requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_TEXTURE_COMPRESSION_ASTC_3D: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_texture_compression_astc_3d") },
    spec: 1u32,
};
pub const GOOGLE_USER_TYPE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_GOOGLE_user_type") },
    spec: 1u32,
};
#[cfg(feature = "ext_pipeline_library")]
pub const KHR_PIPELINE_LIBRARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_pipeline_library") },
    spec: 1u32,
};
#[cfg(feature = "ext_present_barrier")]
pub const NV_PRESENT_BARRIER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_present_barrier") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_present_barrier",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_surface",
        feature = "ext_get_surface_capabilities2",
        feature = "ext_swapchain"
    ))
))]
compile_error ! ("The feature ext_present_barrier requires ((ext_get_physical_device_properties2 or version_1_1) and ext_surface and ext_get_surface_capabilities2 and ext_swapchain) to be enabled.") ;
pub const KHR_SHADER_NON_SEMANTIC_INFO: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_non_semantic_info") },
    spec: 1u32,
};
#[cfg(feature = "ext_present_id")]
pub const KHR_PRESENT_ID: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_id") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_present_id",
    not(all(
        feature = "ext_swapchain",
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        )
    ))
))]
compile_error ! ("The feature ext_present_id requires (ext_swapchain and (ext_get_physical_device_properties2 or version_1_1)) to be enabled.") ;
#[cfg(feature = "ext_private_data")]
pub const EXT_PRIVATE_DATA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_private_data") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_private_data",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_private_data requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_pipeline_creation_cache_control")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_creation_cache_control") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_pipeline_creation_cache_control",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_pipeline_creation_cache_control requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_device_diagnostics_config")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_diagnostics_config") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_device_diagnostics_config",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_device_diagnostics_config requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const QCOM_RENDER_PASS_STORE_OPS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_render_pass_store_ops") },
    spec: 2u32,
};
#[cfg(feature = "ext_cuda_kernel_launch")]
pub const NV_CUDA_KERNEL_LAUNCH: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cuda_kernel_launch") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_cuda_kernel_launch",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_cuda_kernel_launch requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_tile_shading")]
pub const QCOM_TILE_SHADING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_tile_shading") },
    spec: 2u32,
};
#[cfg(all(feature = "ext_tile_shading", not(feature = "ext_tile_properties")))]
compile_error!("The feature ext_tile_shading requires ext_tile_properties to be enabled.");
#[cfg(feature = "ext_low_latency")]
pub const NV_LOW_LATENCY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_low_latency") },
    spec: 1u32,
};
#[cfg(feature = "ext_metal_objects")]
pub const EXT_METAL_OBJECTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_metal_objects") },
    spec: 2u32,
};
#[cfg(feature = "ext_synchronization2")]
pub const KHR_SYNCHRONIZATION2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_synchronization2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_synchronization2",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_synchronization2 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_descriptor_buffer")]
pub const EXT_DESCRIPTOR_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_descriptor_buffer") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_descriptor_buffer",
    not(any(
        all(
            any(
                all(
                    any(
                        feature = "ext_get_physical_device_properties2",
                        feature = "version_1_1"
                    ),
                    feature = "ext_buffer_device_address",
                    feature = "ext_descriptor_indexing"
                ),
                feature = "version_1_2"
            ),
            feature = "ext_synchronization2"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_descriptor_buffer requires (((((ext_get_physical_device_properties2 or version_1_1) and ext_buffer_device_address and ext_descriptor_indexing) or version_1_2) and ext_synchronization2) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_graphics_pipeline_library")]
pub const EXT_GRAPHICS_PIPELINE_LIBRARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_graphics_pipeline_library") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_graphics_pipeline_library",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_pipeline_library"
    ))
))]
compile_error ! ("The feature ext_graphics_pipeline_library requires ((ext_get_physical_device_properties2 or version_1_1) and ext_pipeline_library) to be enabled.") ;
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
#[cfg(feature = "ext_fragment_shading_rate_enums")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_fragment_shading_rate_enums") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_fragment_shading_rate_enums",
    not(feature = "ext_fragment_shading_rate")
))]
compile_error!(
    "The feature ext_fragment_shading_rate_enums requires ext_fragment_shading_rate to be enabled."
);
#[cfg(feature = "ext_ray_tracing_motion_blur")]
pub const NV_RAY_TRACING_MOTION_BLUR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_motion_blur") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ray_tracing_motion_blur",
    not(feature = "ext_ray_tracing_pipeline")
))]
compile_error!(
    "The feature ext_ray_tracing_motion_blur requires ext_ray_tracing_pipeline to be enabled."
);
#[cfg(feature = "ext_mesh_shader")]
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
#[cfg(feature = "ext_rotated_copy_commands")]
pub const QCOM_ROTATED_COPY_COMMANDS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_rotated_copy_commands") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_rotated_copy_commands",
    not(any(feature = "ext_copy_commands2", feature = "version_1_3"))
))]
compile_error ! ("The feature ext_rotated_copy_commands requires (ext_copy_commands2 or version_1_3) to be enabled.") ;
pub const EXT_IMAGE_ROBUSTNESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_robustness") },
    spec: 1u32,
};
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_workgroup_memory_explicit_layout") },
    spec: 1u32,
};
#[cfg(feature = "ext_copy_commands2")]
pub const KHR_COPY_COMMANDS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_copy_commands2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_copy_commands2",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_copy_commands2 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_image_compression_control")]
pub const EXT_IMAGE_COMPRESSION_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_compression_control") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_image_compression_control",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_image_compression_control requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_attachment_feedback_loop_layout") },
    spec: 2u32,
};
pub const EXT_4444_FORMATS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_4444_formats") },
    spec: 1u32,
};
#[cfg(feature = "ext_device_fault")]
pub const EXT_DEVICE_FAULT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_fault") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_device_fault",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_device_fault requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_rasterization_order_attachment_access")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_rasterization_order_attachment_access") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_rasterization_order_attachment_access",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_rasterization_order_attachment_access requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_RGBA10X6_FORMATS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_rgba10x6_formats") },
    spec: 1u32,
};
#[cfg(feature = "ext_acquire_winrt_display")]
pub const NV_ACQUIRE_WINRT_DISPLAY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_acquire_winrt_display") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_acquire_winrt_display",
    not(feature = "ext_direct_mode_display")
))]
compile_error!(
    "The feature ext_acquire_winrt_display requires ext_direct_mode_display to be enabled."
);
#[cfg(feature = "ext_directfb_surface")]
pub const EXT_DIRECTFB_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_EXT_directfb_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_directfb_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_directfb_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_mutable_descriptor_type")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_mutable_descriptor_type") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_mutable_descriptor_type",
    not(any(feature = "ext_maintenance3", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_mutable_descriptor_type requires (ext_maintenance3 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_vertex_input_dynamic_state")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_vertex_input_dynamic_state") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_vertex_input_dynamic_state",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_vertex_input_dynamic_state requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_PHYSICAL_DEVICE_DRM: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_physical_device_drm") },
    spec: 1u32,
};
#[cfg(feature = "ext_device_address_binding_report")]
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_address_binding_report") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_device_address_binding_report",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_debug_utils"
    ))
))]
compile_error ! ("The feature ext_device_address_binding_report requires ((ext_get_physical_device_properties2 or version_1_1) and ext_debug_utils) to be enabled.") ;
#[cfg(feature = "ext_depth_clip_control")]
pub const EXT_DEPTH_CLIP_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clip_control") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_depth_clip_control",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_depth_clip_control requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_primitive_topology_list_restart") },
    spec: 1u32,
};
#[cfg(feature = "ext_format_feature_flags2")]
pub const KHR_FORMAT_FEATURE_FLAGS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_format_feature_flags2") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_format_feature_flags2",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_format_feature_flags2 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_PRESENT_MODE_FIFO_LATEST_READY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_present_mode_fifo_latest_ready") },
    spec: 1u32,
};
#[cfg(feature = "ext_fuchsia_external_memory")]
pub const FUCHSIA_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_FUCHSIA_external_memory") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_fuchsia_external_memory",
    not(any(
        all(
            feature = "ext_external_memory_capabilities",
            feature = "ext_external_memory"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_fuchsia_external_memory requires ((ext_external_memory_capabilities and ext_external_memory) or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_fuchsia_external_semaphore")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_FUCHSIA_external_semaphore") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_fuchsia_external_semaphore",
    not(all(
        feature = "ext_external_semaphore_capabilities",
        feature = "ext_external_semaphore"
    ))
))]
compile_error ! ("The feature ext_fuchsia_external_semaphore requires (ext_external_semaphore_capabilities and ext_external_semaphore) to be enabled.") ;
#[cfg(feature = "ext_buffer_collection")]
pub const FUCHSIA_BUFFER_COLLECTION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_FUCHSIA_buffer_collection") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_buffer_collection",
    not(all(
        feature = "ext_fuchsia_external_memory",
        any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1")
    ))
))]
compile_error ! ("The feature ext_buffer_collection requires (ext_fuchsia_external_memory and (ext_sampler_ycbcr_conversion or version_1_1)) to be enabled.") ;
#[cfg(feature = "ext_subpass_shading")]
pub const HUAWEI_SUBPASS_SHADING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_subpass_shading") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_subpass_shading",
    not(any(
        all(
            any(feature = "ext_create_renderpass2", feature = "version_1_2"),
            feature = "ext_synchronization2"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_subpass_shading requires (((ext_create_renderpass2 or version_1_2) and ext_synchronization2) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_invocation_mask")]
pub const HUAWEI_INVOCATION_MASK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_invocation_mask") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_invocation_mask",
    not(all(
        feature = "ext_ray_tracing_pipeline",
        any(feature = "ext_synchronization2", feature = "version_1_3")
    ))
))]
compile_error ! ("The feature ext_invocation_mask requires (ext_ray_tracing_pipeline and (ext_synchronization2 or version_1_3)) to be enabled.") ;
#[cfg(feature = "ext_external_memory_rdma")]
pub const NV_EXTERNAL_MEMORY_RDMA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_memory_rdma") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_rdma",
    not(any(feature = "ext_external_memory", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory_rdma requires (ext_external_memory or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_pipeline_properties")]
pub const EXT_PIPELINE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pipeline_properties") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_pipeline_properties",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_pipeline_properties requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_frame_boundary")]
pub const EXT_FRAME_BOUNDARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_frame_boundary") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_frame_boundary",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_frame_boundary requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_multisampled_render_to_single_sampled")]
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_multisampled_render_to_single_sampled") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_multisampled_render_to_single_sampled",
    not(any(
        all(
            feature = "ext_create_renderpass2",
            feature = "ext_depth_stencil_resolve"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_multisampled_render_to_single_sampled requires ((ext_create_renderpass2 and ext_depth_stencil_resolve) or version_1_2) to be enabled.") ;
#[cfg(feature = "ext_extended_dynamic_state2")]
pub const EXT_EXTENDED_DYNAMIC_STATE2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_extended_dynamic_state2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_extended_dynamic_state2",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_extended_dynamic_state2 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_screen_surface")]
pub const QNX_SCREEN_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_QNX_screen_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_screen_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_screen_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_color_write_enable")]
pub const EXT_COLOR_WRITE_ENABLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_color_write_enable") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_color_write_enable",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_color_write_enable requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_PRIMITIVES_GENERATED_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_primitives_generated_query") },
    spec: 1u32,
};
#[cfg(feature = "ext_ray_tracing_maintenance1")]
pub const KHR_RAY_TRACING_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_tracing_maintenance1") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ray_tracing_maintenance1",
    not(feature = "ext_acceleration_structure")
))]
compile_error!(
    "The feature ext_ray_tracing_maintenance1 requires ext_acceleration_structure to be enabled."
);
pub const KHR_SHADER_UNTYPED_POINTERS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_untyped_pointers") },
    spec: 1u32,
};
pub const EXT_GLOBAL_PRIORITY_QUERY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_global_priority_query") },
    spec: 1u32,
};
#[cfg(feature = "ext_video_encode_rgb_conversion")]
pub const VALVE_VIDEO_ENCODE_RGB_CONVERSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_video_encode_rgb_conversion") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_video_encode_rgb_conversion",
    not(all(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1")))
))]
compile_error ! ("The feature ext_video_encode_rgb_conversion requires ((ext_sampler_ycbcr_conversion or version_1_1)) to be enabled.") ;
#[cfg(feature = "ext_image_view_min_lod")]
pub const EXT_IMAGE_VIEW_MIN_LOD: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_view_min_lod") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_image_view_min_lod",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_image_view_min_lod requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_multi_draw")]
pub const EXT_MULTI_DRAW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_multi_draw") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_multi_draw",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_multi_draw requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
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
#[cfg(feature = "ext_opacity_micromap")]
pub const EXT_OPACITY_MICROMAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_opacity_micromap") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_opacity_micromap",
    not(all(
        feature = "ext_acceleration_structure",
        any(feature = "ext_synchronization2", feature = "version_1_3")
    ))
))]
compile_error ! ("The feature ext_opacity_micromap requires (ext_acceleration_structure and (ext_synchronization2 or version_1_3)) to be enabled.") ;
#[cfg(feature = "ext_displacement_micromap")]
pub const NV_DISPLACEMENT_MICROMAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_displacement_micromap") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_displacement_micromap",
    not(feature = "ext_opacity_micromap")
))]
compile_error!(
    "The feature ext_displacement_micromap requires ext_opacity_micromap to be enabled."
);
pub const EXT_LOAD_STORE_OP_NONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_load_store_op_none") },
    spec: 1u32,
};
#[cfg(feature = "ext_cluster_culling_shader")]
pub const HUAWEI_CLUSTER_CULLING_SHADER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_cluster_culling_shader") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_cluster_culling_shader",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_cluster_culling_shader requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_border_color_swizzle")]
pub const EXT_BORDER_COLOR_SWIZZLE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_border_color_swizzle") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_border_color_swizzle",
    not(feature = "ext_custom_border_color")
))]
compile_error!(
    "The feature ext_border_color_swizzle requires ext_custom_border_color to be enabled."
);
#[cfg(feature = "ext_pageable_device_local_memory")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_pageable_device_local_memory") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_pageable_device_local_memory",
    not(feature = "ext_memory_priority")
))]
compile_error!(
    "The feature ext_pageable_device_local_memory requires ext_memory_priority to be enabled."
);
#[cfg(feature = "ext_maintenance4")]
pub const KHR_MAINTENANCE4: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance4") },
    spec: 2u32,
};
#[cfg(all(feature = "ext_maintenance4", not(feature = "version_1_1")))]
compile_error!("The feature ext_maintenance4 requires version_1_1 to be enabled.");
pub const ARM_SHADER_CORE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_shader_core_properties") },
    spec: 1u32,
};
pub const KHR_SHADER_SUBGROUP_ROTATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_subgroup_rotate") },
    spec: 2u32,
};
#[cfg(feature = "ext_scheduling_controls")]
pub const ARM_SCHEDULING_CONTROLS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_scheduling_controls") },
    spec: 1u32,
};
#[cfg(feature = "ext_image_sliced_view_of_3d")]
pub const EXT_IMAGE_SLICED_VIEW_OF_3D: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_image_sliced_view_of_3d") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_image_sliced_view_of_3d",
    not(any(
        all(
            feature = "ext_maintenance1",
            feature = "ext_get_physical_device_properties2"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_image_sliced_view_of_3d requires ((ext_maintenance1 and ext_get_physical_device_properties2) or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_descriptor_set_host_mapping")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_descriptor_set_host_mapping") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_descriptor_set_host_mapping",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_descriptor_set_host_mapping requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_DEPTH_CLAMP_ZERO_ONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clamp_zero_one") },
    spec: 1u32,
};
pub const EXT_NON_SEAMLESS_CUBE_MAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_non_seamless_cube_map") },
    spec: 1u32,
};
#[cfg(feature = "ext_render_pass_striped")]
pub const ARM_RENDER_PASS_STRIPED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_render_pass_striped") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_render_pass_striped",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_synchronization2"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_render_pass_striped requires (((ext_get_physical_device_properties2 or version_1_1) and ext_synchronization2) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_fragment_density_map_offset")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_fragment_density_map_offset") },
    spec: 3u32,
};
#[cfg(all(
    feature = "ext_fragment_density_map_offset",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_fragment_density_map",
        any(feature = "ext_create_renderpass2", feature = "version_1_2"),
        any(feature = "version_1_3", feature = "ext_dynamic_rendering")
    ))
))]
compile_error ! ("The feature ext_fragment_density_map_offset requires ((ext_get_physical_device_properties2 or version_1_1) and ext_fragment_density_map and (ext_create_renderpass2 or version_1_2) and (version_1_3 or ext_dynamic_rendering)) to be enabled.") ;
#[cfg(feature = "ext_copy_memory_indirect")]
pub const NV_COPY_MEMORY_INDIRECT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_copy_memory_indirect") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_copy_memory_indirect",
    not(any(
        all(
            feature = "ext_get_physical_device_properties2",
            feature = "ext_buffer_device_address"
        ),
        feature = "version_1_2"
    ))
))]
compile_error ! ("The feature ext_copy_memory_indirect requires ((ext_get_physical_device_properties2 and ext_buffer_device_address) or version_1_2) to be enabled.") ;
#[cfg(feature = "ext_memory_decompression")]
pub const NV_MEMORY_DECOMPRESSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_memory_decompression") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_memory_decompression",
    not(all(
        feature = "ext_get_physical_device_properties2",
        feature = "ext_buffer_device_address"
    ))
))]
compile_error ! ("The feature ext_memory_decompression requires (ext_get_physical_device_properties2 and ext_buffer_device_address) to be enabled.") ;
#[cfg(feature = "ext_device_generated_commands_compute")]
pub const NV_DEVICE_GENERATED_COMMANDS_COMPUTE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_device_generated_commands_compute") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_device_generated_commands_compute",
    not(feature = "ext_device_generated_commands")
))]
compile_error ! ("The feature ext_device_generated_commands_compute requires ext_device_generated_commands to be enabled.") ;
#[cfg(feature = "ext_ray_tracing_linear_swept_spheres")]
pub const NV_RAY_TRACING_LINEAR_SWEPT_SPHERES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_linear_swept_spheres") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ray_tracing_linear_swept_spheres",
    not(feature = "ext_ray_tracing_pipeline")
))]
compile_error ! ("The feature ext_ray_tracing_linear_swept_spheres requires ext_ray_tracing_pipeline to be enabled.") ;
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
#[cfg(feature = "ext_image_processing")]
pub const QCOM_IMAGE_PROCESSING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_image_processing") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_image_processing",
    not(any(feature = "ext_format_feature_flags2", feature = "version_1_3"))
))]
compile_error ! ("The feature ext_image_processing requires (ext_format_feature_flags2 or version_1_3) to be enabled.") ;
pub const EXT_NESTED_COMMAND_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_nested_command_buffer") },
    spec: 1u32,
};
#[cfg(feature = "ext_ohos_external_memory")]
pub const OHOS_EXTERNAL_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_OHOS_external_memory") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ohos_external_memory",
    not(all(any(
        all(
            feature = "ext_sampler_ycbcr_conversion",
            feature = "ext_external_memory",
            feature = "ext_dedicated_allocation"
        ),
        feature = "version_1_1"
    )))
))]
compile_error ! ("The feature ext_ohos_external_memory requires (((ext_sampler_ycbcr_conversion and ext_external_memory and ext_dedicated_allocation) or version_1_1) and ext_queue_family_foreign) to be enabled.") ;
#[cfg(feature = "ext_external_memory_acquire_unmodified")]
pub const EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_acquire_unmodified") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_acquire_unmodified",
    not(any(feature = "ext_external_memory", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory_acquire_unmodified requires (ext_external_memory or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_extended_dynamic_state3")]
pub const EXT_EXTENDED_DYNAMIC_STATE3: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_extended_dynamic_state3") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_extended_dynamic_state3",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_extended_dynamic_state3 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_subpass_merge_feedback")]
pub const EXT_SUBPASS_MERGE_FEEDBACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_subpass_merge_feedback") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_subpass_merge_feedback",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_subpass_merge_feedback requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_direct_driver_loading")]
pub const LUNARG_DIRECT_DRIVER_LOADING: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_LUNARG_direct_driver_loading") },
    spec: 1u32,
};
#[cfg(feature = "ext_tensors")]
pub const ARM_TENSORS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_tensors") },
    spec: 2u32,
};
#[cfg(all(feature = "ext_tensors", not(feature = "version_1_3")))]
compile_error!("The feature ext_tensors requires version_1_3 to be enabled.");
#[cfg(feature = "ext_shader_module_identifier")]
pub const EXT_SHADER_MODULE_IDENTIFIER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_module_identifier") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_shader_module_identifier",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_pipeline_creation_cache_control"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_shader_module_identifier requires (((ext_get_physical_device_properties2 or version_1_1) and ext_pipeline_creation_cache_control) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_rasterization_order_attachment_access")]
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_rasterization_order_attachment_access") },
    spec: 1u32,
};
#[cfg(feature = "ext_optical_flow")]
pub const NV_OPTICAL_FLOW: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_optical_flow") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_optical_flow",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_format_feature_flags2",
            feature = "ext_synchronization2"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_optical_flow requires (((ext_get_physical_device_properties2 or version_1_1) and ext_format_feature_flags2 and ext_synchronization2) or version_1_3) to be enabled.") ;
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
#[cfg(feature = "ext_maintenance5")]
pub const KHR_MAINTENANCE5: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance5") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_maintenance5",
    not(any(
        all(feature = "version_1_1", feature = "ext_dynamic_rendering"),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_maintenance5 requires ((version_1_1 and ext_dynamic_rendering) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_anti_lag")]
pub const AMD_ANTI_LAG: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMD_anti_lag") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_anti_lag",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_anti_lag requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_dense_geometry_format")]
pub const AMDX_DENSE_GEOMETRY_FORMAT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_AMDX_dense_geometry_format") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_dense_geometry_format",
    not(all(
        feature = "ext_acceleration_structure",
        any(feature = "ext_maintenance5", feature = "version_1_4")
    ))
))]
compile_error ! ("The feature ext_dense_geometry_format requires (ext_acceleration_structure and (ext_maintenance5 or version_1_4)) to be enabled.") ;
#[cfg(feature = "ext_present_id2")]
pub const KHR_PRESENT_ID2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_id2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_present_id2",
    not(all(
        feature = "ext_get_surface_capabilities2",
        feature = "ext_surface",
        feature = "ext_swapchain"
    ))
))]
compile_error ! ("The feature ext_present_id2 requires (ext_get_surface_capabilities2 and ext_surface and ext_swapchain) to be enabled.") ;
#[cfg(feature = "ext_present_wait2")]
pub const KHR_PRESENT_WAIT2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_wait2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_present_wait2",
    not(all(
        feature = "ext_get_surface_capabilities2",
        feature = "ext_surface",
        feature = "ext_swapchain",
        feature = "ext_present_id2"
    ))
))]
compile_error ! ("The feature ext_present_wait2 requires (ext_get_surface_capabilities2 and ext_surface and ext_swapchain and ext_present_id2) to be enabled.") ;
pub const KHR_RAY_TRACING_POSITION_FETCH: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_ray_tracing_position_fetch") },
    spec: 1u32,
};
#[cfg(feature = "ext_shader_object")]
pub const EXT_SHADER_OBJECT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_object") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_shader_object",
    not(any(
        all(
            any(
                feature = "ext_get_physical_device_properties2",
                feature = "version_1_1"
            ),
            feature = "ext_dynamic_rendering"
        ),
        feature = "version_1_3"
    ))
))]
compile_error ! ("The feature ext_shader_object requires (((ext_get_physical_device_properties2 or version_1_1) and ext_dynamic_rendering) or version_1_3) to be enabled.") ;
#[cfg(feature = "ext_pipeline_binary")]
pub const KHR_PIPELINE_BINARY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_pipeline_binary") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_pipeline_binary",
    not(any(feature = "ext_maintenance5", feature = "version_1_4"))
))]
compile_error!(
    "The feature ext_pipeline_binary requires (ext_maintenance5 or version_1_4) to be enabled."
);
#[cfg(feature = "ext_tile_properties")]
pub const QCOM_TILE_PROPERTIES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_tile_properties") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_tile_properties",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_tile_properties requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_amigo_profiling")]
pub const SEC_AMIGO_PROFILING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_SEC_amigo_profiling") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_amigo_profiling",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_amigo_profiling requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_surface_maintenance1")]
pub const KHR_SURFACE_MAINTENANCE1: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_KHR_surface_maintenance1") },
    spec: 1u32,
};
#[cfg(feature = "ext_swapchain_maintenance1")]
pub const KHR_SWAPCHAIN_MAINTENANCE1: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_swapchain_maintenance1") },
    spec: 1u32,
};
pub const QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_multiview_per_view_viewports") },
    spec: 1u32,
};
#[cfg(feature = "ext_ray_tracing_invocation_reorder")]
pub const NV_RAY_TRACING_INVOCATION_REORDER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_invocation_reorder") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ray_tracing_invocation_reorder",
    not(feature = "ext_ray_tracing_pipeline")
))]
compile_error ! ("The feature ext_ray_tracing_invocation_reorder requires ext_ray_tracing_pipeline to be enabled.") ;
#[cfg(feature = "ext_cooperative_vector")]
pub const NV_COOPERATIVE_VECTOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cooperative_vector") },
    spec: 4u32,
};
#[cfg(all(
    feature = "ext_cooperative_vector",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_cooperative_vector requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const NV_EXTENDED_SPARSE_ADDRESS_SPACE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_extended_sparse_address_space") },
    spec: 1u32,
};
#[cfg(feature = "ext_mutable_descriptor_type")]
pub const EXT_MUTABLE_DESCRIPTOR_TYPE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_mutable_descriptor_type") },
    spec: 1u32,
};
pub const EXT_LEGACY_VERTEX_ATTRIBUTES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_legacy_vertex_attributes") },
    spec: 1u32,
};
#[cfg(feature = "ext_layer_settings")]
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
pub const KHR_INTERNALLY_SYNCHRONIZED_QUEUES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_internally_synchronized_queues") },
    spec: 1u32,
};
#[cfg(feature = "ext_low_latency2")]
pub const NV_LOW_LATENCY2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_low_latency2") },
    spec: 2u32,
};
#[cfg(all(
    feature = "ext_low_latency2",
    not(all(
        any(feature = "version_1_2", feature = "ext_timeline_semaphore"),
        any(feature = "ext_present_id", feature = "ext_present_id2")
    ))
))]
compile_error ! ("The feature ext_low_latency2 requires ((version_1_2 or ext_timeline_semaphore) and (ext_present_id or ext_present_id2)) to be enabled.") ;
#[cfg(feature = "ext_cooperative_matrix")]
pub const KHR_COOPERATIVE_MATRIX: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_cooperative_matrix") },
    spec: 2u32,
};
#[cfg(feature = "ext_data_graph")]
pub const ARM_DATA_GRAPH: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_data_graph") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_data_graph",
    not(all(
        feature = "version_1_3",
        feature = "ext_maintenance5",
        feature = "ext_deferred_host_operations"
    ))
))]
compile_error ! ("The feature ext_data_graph requires (version_1_3 and ext_maintenance5 and ext_deferred_host_operations) to be enabled.") ;
#[cfg(feature = "ext_multiview_per_view_render_areas")]
pub const QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_multiview_per_view_render_areas") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_multiview_per_view_render_areas",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_multiview_per_view_render_areas requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_COMPUTE_SHADER_DERIVATIVES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_compute_shader_derivatives") },
    spec: 1u32,
};
pub const NV_PER_STAGE_DESCRIPTOR_SET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_per_stage_descriptor_set") },
    spec: 1u32,
};
#[cfg(feature = "ext_image_processing2")]
pub const QCOM_IMAGE_PROCESSING2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_image_processing2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_image_processing2",
    not(feature = "ext_image_processing")
))]
compile_error!("The feature ext_image_processing2 requires ext_image_processing to be enabled.");
#[cfg(feature = "ext_filter_cubic_weights")]
pub const QCOM_FILTER_CUBIC_WEIGHTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_filter_cubic_weights") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_filter_cubic_weights",
    not(feature = "ext_filter_cubic")
))]
compile_error!("The feature ext_filter_cubic_weights requires ext_filter_cubic to be enabled.");
#[cfg(feature = "ext_ycbcr_degamma")]
pub const QCOM_YCBCR_DEGAMMA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_ycbcr_degamma") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_ycbcr_degamma",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_ycbcr_degamma requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const QCOM_FILTER_CUBIC_CLAMP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_filter_cubic_clamp") },
    spec: 1u32,
};
#[cfg(feature = "ext_attachment_feedback_loop_dynamic_state")]
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_attachment_feedback_loop_dynamic_state") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_attachment_feedback_loop_dynamic_state",
    not(all(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    )))
))]
compile_error ! ("The feature ext_attachment_feedback_loop_dynamic_state requires ((ext_get_physical_device_properties2 or version_1_1) and ext_attachment_feedback_loop_layout) to be enabled.") ;
#[cfg(feature = "ext_vertex_attribute_divisor")]
pub const KHR_VERTEX_ATTRIBUTE_DIVISOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_vertex_attribute_divisor") },
    spec: 1u32,
};
pub const KHR_LOAD_STORE_OP_NONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_load_store_op_none") },
    spec: 1u32,
};
#[cfg(feature = "ext_unified_image_layouts")]
pub const KHR_UNIFIED_IMAGE_LAYOUTS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_unified_image_layouts") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_unified_image_layouts",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_unified_image_layouts requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_SHADER_FLOAT_CONTROLS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_float_controls2") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_screen_buffer")]
pub const QNX_EXTERNAL_MEMORY_SCREEN_BUFFER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QNX_external_memory_screen_buffer") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_screen_buffer",
    not(all(any(
        all(
            feature = "ext_sampler_ycbcr_conversion",
            feature = "ext_external_memory",
            feature = "ext_dedicated_allocation"
        ),
        feature = "version_1_1"
    )))
))]
compile_error ! ("The feature ext_external_memory_screen_buffer requires (((ext_sampler_ycbcr_conversion and ext_external_memory and ext_dedicated_allocation) or version_1_1) and ext_queue_family_foreign) to be enabled.") ;
#[cfg(feature = "ext_layered_driver")]
pub const MSFT_LAYERED_DRIVER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_MSFT_layered_driver") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_layered_driver",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_layered_driver requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_INDEX_TYPE_UINT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_index_type_uint8") },
    spec: 1u32,
};
#[cfg(feature = "ext_line_rasterization")]
pub const KHR_LINE_RASTERIZATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_line_rasterization") },
    spec: 1u32,
};
#[cfg(feature = "ext_calibrated_timestamps")]
pub const KHR_CALIBRATED_TIMESTAMPS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_calibrated_timestamps") },
    spec: 1u32,
};
pub const KHR_SHADER_EXPECT_ASSUME: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_expect_assume") },
    spec: 1u32,
};
#[cfg(feature = "ext_maintenance6")]
pub const KHR_MAINTENANCE6: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance6") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_maintenance6", not(feature = "version_1_1")))]
compile_error!("The feature ext_maintenance6 requires version_1_1 to be enabled.");
pub const NV_DESCRIPTOR_POOL_OVERALLOCATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_descriptor_pool_overallocation") },
    spec: 1u32,
};
#[cfg(feature = "ext_tile_memory_heap")]
pub const QCOM_TILE_MEMORY_HEAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_tile_memory_heap") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_tile_memory_heap",
    not(any(
        all(
            feature = "ext_get_memory_requirements2",
            feature = "ext_get_physical_device_properties2"
        ),
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_tile_memory_heap requires ((ext_get_memory_requirements2 and ext_get_physical_device_properties2) or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_copy_memory_indirect")]
pub const KHR_COPY_MEMORY_INDIRECT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_copy_memory_indirect") },
    spec: 1u32,
};
#[cfg(feature = "ext_memory_decompression")]
pub const EXT_MEMORY_DECOMPRESSION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_memory_decompression") },
    spec: 1u32,
};
#[cfg(feature = "ext_display_stereo")]
pub const NV_DISPLAY_STEREO: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_NV_display_stereo") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_display_stereo",
    not(all(feature = "ext_display", feature = "ext_get_display_properties2"))
))]
compile_error ! ("The feature ext_display_stereo requires (ext_display and ext_get_display_properties2) to be enabled.") ;
pub const NV_RAW_ACCESS_CHAINS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_raw_access_chains") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_compute_queue")]
pub const NV_EXTERNAL_COMPUTE_QUEUE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_external_compute_queue") },
    spec: 1u32,
};
pub const KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_relaxed_extended_instruction") },
    spec: 1u32,
};
pub const NV_COMMAND_BUFFER_INHERITANCE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_command_buffer_inheritance") },
    spec: 1u32,
};
#[cfg(feature = "ext_maintenance7")]
pub const KHR_MAINTENANCE7: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance7") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_maintenance7", not(feature = "version_1_1")))]
compile_error!("The feature ext_maintenance7 requires version_1_1 to be enabled.");
pub const NV_SHADER_ATOMIC_FLOAT16_VECTOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_shader_atomic_float16_vector") },
    spec: 1u32,
};
pub const EXT_SHADER_REPLICATED_COMPOSITES: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_replicated_composites") },
    spec: 1u32,
};
pub const EXT_SHADER_FLOAT8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_float8") },
    spec: 1u32,
};
pub const NV_RAY_TRACING_VALIDATION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_ray_tracing_validation") },
    spec: 1u32,
};
#[cfg(feature = "ext_cluster_acceleration_structure")]
pub const NV_CLUSTER_ACCELERATION_STRUCTURE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cluster_acceleration_structure") },
    spec: 4u32,
};
#[cfg(all(
    feature = "ext_cluster_acceleration_structure",
    not(feature = "ext_acceleration_structure")
))]
compile_error ! ("The feature ext_cluster_acceleration_structure requires ext_acceleration_structure to be enabled.") ;
#[cfg(feature = "ext_partitioned_acceleration_structure")]
pub const NV_PARTITIONED_ACCELERATION_STRUCTURE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_partitioned_acceleration_structure") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_partitioned_acceleration_structure",
    not(feature = "ext_acceleration_structure")
))]
compile_error ! ("The feature ext_partitioned_acceleration_structure requires ext_acceleration_structure to be enabled.") ;
#[cfg(feature = "ext_device_generated_commands")]
pub const EXT_DEVICE_GENERATED_COMMANDS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_device_generated_commands") },
    spec: 1u32,
};
#[cfg(feature = "ext_maintenance8")]
pub const KHR_MAINTENANCE8: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance8") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_maintenance8", not(feature = "version_1_1")))]
compile_error!("The feature ext_maintenance8 requires version_1_1 to be enabled.");
#[cfg(feature = "ext_image_alignment_control")]
pub const MESA_IMAGE_ALIGNMENT_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_MESA_image_alignment_control") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_image_alignment_control",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_image_alignment_control requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const KHR_SHADER_FMA: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_shader_fma") },
    spec: 1u32,
};
#[cfg(feature = "ext_push_constant_bank")]
pub const NV_PUSH_CONSTANT_BANK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_push_constant_bank") },
    spec: 1u32,
};
#[cfg(feature = "ext_ray_tracing_invocation_reorder")]
pub const EXT_RAY_TRACING_INVOCATION_REORDER: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_ray_tracing_invocation_reorder") },
    spec: 1u32,
};
#[cfg(feature = "ext_depth_clamp_control")]
pub const EXT_DEPTH_CLAMP_CONTROL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_depth_clamp_control") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_depth_clamp_control",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_depth_clamp_control requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_maintenance9")]
pub const KHR_MAINTENANCE9: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance9") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_maintenance9",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_maintenance9 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_ohos_surface")]
pub const OHOS_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_OHOS_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_ohos_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_ohos_surface requires ext_surface to be enabled.");
#[cfg(feature = "ext_hdr_vivid")]
pub const HUAWEI_HDR_VIVID: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_HUAWEI_hdr_vivid") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_hdr_vivid",
    not(all(
        any(
            feature = "ext_get_physical_device_properties2",
            feature = "version_1_1"
        ),
        feature = "ext_swapchain",
        feature = "ext_hdr_metadata"
    ))
))]
compile_error ! ("The feature ext_hdr_vivid requires ((ext_get_physical_device_properties2 or version_1_1) and ext_swapchain and ext_hdr_metadata) to be enabled.") ;
#[cfg(feature = "ext_cooperative_matrix2")]
pub const NV_COOPERATIVE_MATRIX2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_cooperative_matrix2") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_cooperative_matrix2",
    not(feature = "ext_cooperative_matrix")
))]
compile_error!(
    "The feature ext_cooperative_matrix2 requires ext_cooperative_matrix to be enabled."
);
pub const ARM_PIPELINE_OPACITY_MICROMAP: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_pipeline_opacity_micromap") },
    spec: 1u32,
};
#[cfg(feature = "ext_external_memory_metal")]
pub const EXT_EXTERNAL_MEMORY_METAL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_external_memory_metal") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_external_memory_metal",
    not(any(feature = "ext_external_memory", feature = "version_1_1"))
))]
compile_error ! ("The feature ext_external_memory_metal requires (ext_external_memory or version_1_1) to be enabled.") ;
pub const KHR_DEPTH_CLAMP_ZERO_ONE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_depth_clamp_zero_one") },
    spec: 1u32,
};
#[cfg(feature = "ext_performance_counters_by_region")]
pub const ARM_PERFORMANCE_COUNTERS_BY_REGION: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_performance_counters_by_region") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_performance_counters_by_region",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_performance_counters_by_region requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_VERTEX_ATTRIBUTE_ROBUSTNESS: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_vertex_attribute_robustness") },
    spec: 1u32,
};
pub const ARM_FORMAT_PACK: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_ARM_format_pack") },
    spec: 1u32,
};
#[cfg(feature = "ext_fragment_density_map_layered")]
pub const VALVE_FRAGMENT_DENSITY_MAP_LAYERED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_fragment_density_map_layered") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_fragment_density_map_layered",
    not(all(
        any(feature = "ext_maintenance5", feature = "version_1_4"),
        feature = "ext_fragment_density_map"
    ))
))]
compile_error ! ("The feature ext_fragment_density_map_layered requires ((ext_maintenance5 or version_1_4) and ext_fragment_density_map) to be enabled.") ;
pub const KHR_ROBUSTNESS2: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_robustness2") },
    spec: 1u32,
};
#[cfg(feature = "ext_present_metering")]
pub const NV_PRESENT_METERING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_present_metering") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_present_metering",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_present_metering requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_fragment_density_map_offset")]
pub const EXT_FRAGMENT_DENSITY_MAP_OFFSET: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_fragment_density_map_offset") },
    spec: 1u32,
};
pub const EXT_ZERO_INITIALIZE_DEVICE_MEMORY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_zero_initialize_device_memory") },
    spec: 1u32,
};
pub const KHR_PRESENT_MODE_FIFO_LATEST_READY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_present_mode_fifo_latest_ready") },
    spec: 1u32,
};
pub const EXT_SHADER_64BIT_INDEXING: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_64bit_indexing") },
    spec: 1u32,
};
#[cfg(feature = "ext_custom_resolve")]
pub const EXT_CUSTOM_RESOLVE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_custom_resolve") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_custom_resolve",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_custom_resolve requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
#[cfg(feature = "ext_data_graph_model")]
pub const QCOM_DATA_GRAPH_MODEL: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_QCOM_data_graph_model") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_data_graph_model", not(feature = "ext_data_graph")))]
compile_error!("The feature ext_data_graph_model requires ext_data_graph to be enabled.");
#[cfg(feature = "ext_maintenance10")]
pub const KHR_MAINTENANCE10: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_KHR_maintenance10") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_maintenance10",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_maintenance10 requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_SHADER_LONG_VECTOR: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_long_vector") },
    spec: 1u32,
};
pub const SEC_PIPELINE_CACHE_INCREMENTAL_MODE: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_SEC_pipeline_cache_incremental_mode") },
    spec: 1u32,
};
pub const EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_uniform_buffer_unsized_array") },
    spec: 1u32,
};
#[cfg(feature = "ext_compute_occupancy_priority")]
pub const NV_COMPUTE_OCCUPANCY_PRIORITY: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_NV_compute_occupancy_priority") },
    spec: 1u32,
};
#[cfg(all(
    feature = "ext_compute_occupancy_priority",
    not(any(
        feature = "ext_get_physical_device_properties2",
        feature = "version_1_1"
    ))
))]
compile_error ! ("The feature ext_compute_occupancy_priority requires (ext_get_physical_device_properties2 or version_1_1) to be enabled.") ;
pub const EXT_SHADER_SUBGROUP_PARTITIONED: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_EXT_shader_subgroup_partitioned") },
    spec: 1u32,
};
#[cfg(feature = "ext_ubm_surface")]
pub const SEC_UBM_SURFACE: InstanceExtension = InstanceExtension {
    name: unsafe { InstanceExtensionName::new(c"VK_SEC_ubm_surface") },
    spec: 1u32,
};
#[cfg(all(feature = "ext_ubm_surface", not(feature = "ext_surface")))]
compile_error!("The feature ext_ubm_surface requires ext_surface to be enabled.");
pub const VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT: DeviceExtension = DeviceExtension {
    name: unsafe { DeviceExtensionName::new(c"VK_VALVE_shader_mixed_float_dot_product") },
    spec: 1u32,
};
