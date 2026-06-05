use crate::private;
use crate::{vk::ObjectType, Handle};
use core::fmt;
use std::num::{NonZeroU64, NonZeroUsize};
#[macro_use]
mod macros;
handle_dispatchable! { Instance , Instance , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html>" , "VkInstance" }
handle_dispatchable! { PhysicalDevice , PhysicalDevice , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html>" , "VkPhysicalDevice" }
handle_dispatchable! { Device , Device , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html>" , "VkDevice" }
handle_dispatchable! { Queue , Queue , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html>" , "VkQueue" }
handle_nondispatchable! { DeviceMemory , DeviceMemory , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html>" , "VkDeviceMemory" }
handle_nondispatchable! { Fence , Fence , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html>" , "VkFence" }
handle_nondispatchable! { Semaphore , Semaphore , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html>" , "VkSemaphore" }
handle_nondispatchable! { QueryPool , QueryPool , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html>" , "VkQueryPool" }
handle_nondispatchable! { Buffer , Buffer , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html>" , "VkBuffer" }
handle_nondispatchable! { Image , Image , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html>" , "VkImage" }
handle_nondispatchable! { ImageView , ImageView , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html>" , "VkImageView" }
handle_nondispatchable! { CommandPool , CommandPool , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html>" , "VkCommandPool" }
handle_dispatchable! { CommandBuffer , CommandBuffer , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html>" , "VkCommandBuffer" }
handle_nondispatchable! { Event , Event , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html>" , "VkEvent" }
handle_nondispatchable! { BufferView , BufferView , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html>" , "VkBufferView" }
handle_nondispatchable! { ShaderModule , ShaderModule , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html>" , "VkShaderModule" }
handle_nondispatchable! { PipelineCache , PipelineCache , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html>" , "VkPipelineCache" }
handle_nondispatchable! { Pipeline , Pipeline , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html>" , "VkPipeline" }
handle_nondispatchable! { PipelineLayout , PipelineLayout , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html>" , "VkPipelineLayout" }
handle_nondispatchable! { Sampler , Sampler , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html>" , "VkSampler" }
handle_nondispatchable! { DescriptorPool , DescriptorPool , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html>" , "VkDescriptorPool" }
handle_nondispatchable! { DescriptorSet , DescriptorSet , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html>" , "VkDescriptorSet" }
handle_nondispatchable! { DescriptorSetLayout , DescriptorSetLayout , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html>" , "VkDescriptorSetLayout" }
handle_nondispatchable! { Framebuffer , Framebuffer , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html>" , "VkFramebuffer" }
handle_nondispatchable! { RenderPass , RenderPass , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html>" , "VkRenderPass" }
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
handle_nondispatchable! { DescriptorUpdateTemplate , DescriptorUpdateTemplate , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html>" , "VkDescriptorUpdateTemplate" }
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
handle_nondispatchable! { SamplerYcbcrConversion , SamplerYcbcrConversion , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html>" , "VkSamplerYcbcrConversion" }
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
handle_nondispatchable! { PrivateDataSlot , PrivateDataSlot , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html>" , "VkPrivateDataSlot" }
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotEXT.html>"]
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;
#[cfg(feature = "ext_surface")]
handle_nondispatchable! { SurfaceKHR , SurfaceKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html>" , "VkSurfaceKHR" }
#[cfg(feature = "ext_swapchain")]
handle_nondispatchable! { SwapchainKHR , SwapchainKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html>" , "VkSwapchainKHR" }
#[cfg(feature = "ext_display")]
handle_nondispatchable! { DisplayKHR , DisplayKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html>" , "VkDisplayKHR" }
#[cfg(feature = "ext_display")]
handle_nondispatchable! { DisplayModeKHR , DisplayModeKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html>" , "VkDisplayModeKHR" }
#[cfg(feature = "ext_debug_report")]
handle_nondispatchable! { DebugReportCallbackEXT , DebugReportCallbackEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html>" , "VkDebugReportCallbackEXT" }
#[cfg(feature = "ext_binary_import")]
handle_nondispatchable! { CuModuleNVX , CuModuleNVX , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html>" , "VkCuModuleNVX" }
#[cfg(feature = "ext_binary_import")]
handle_nondispatchable! { CuFunctionNVX , CuFunctionNVX , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html>" , "VkCuFunctionNVX" }
#[cfg(feature = "ext_debug_utils")]
handle_nondispatchable! { DebugUtilsMessengerEXT , DebugUtilsMessengerEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html>" , "VkDebugUtilsMessengerEXT" }
#[cfg(feature = "ext_gpa_interface")]
handle_nondispatchable! { GpaSessionAMD , GpaSessionAMD , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html>" , "VkGpaSessionAMD" }
#[cfg(any(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
handle_nondispatchable! { TensorARM , TensorARM , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html>" , "VkTensorARM" }
#[cfg(feature = "ext_acceleration_structure")]
handle_nondispatchable! { AccelerationStructureKHR , AccelerationStructureKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html>" , "VkAccelerationStructureKHR" }
#[cfg(feature = "ext_validation_cache")]
handle_nondispatchable! { ValidationCacheEXT , ValidationCacheEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html>" , "VkValidationCacheEXT" }
#[cfg(feature = "ext_ray_tracing")]
handle_nondispatchable! { AccelerationStructureNV , AccelerationStructureNV , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html>" , "VkAccelerationStructureNV" }
#[cfg(feature = "ext_performance_query")]
handle_nondispatchable! { PerformanceConfigurationINTEL , PerformanceConfigurationINTEL , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html>" , "VkPerformanceConfigurationINTEL" }
#[cfg(feature = "ext_deferred_host_operations")]
handle_nondispatchable! { DeferredOperationKHR , DeferredOperationKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html>" , "VkDeferredOperationKHR" }
#[cfg(feature = "ext_device_generated_commands")]
handle_nondispatchable! { IndirectCommandsLayoutNV , IndirectCommandsLayoutNV , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html>" , "VkIndirectCommandsLayoutNV" }
#[cfg(feature = "ext_cuda_kernel_launch")]
handle_nondispatchable! { CudaModuleNV , CudaModuleNV , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html>" , "VkCudaModuleNV" }
#[cfg(feature = "ext_cuda_kernel_launch")]
handle_nondispatchable! { CudaFunctionNV , CudaFunctionNV , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html>" , "VkCudaFunctionNV" }
#[cfg(feature = "ext_buffer_collection")]
handle_nondispatchable! { BufferCollectionFUCHSIA , BufferCollectionFUCHSIA , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html>" , "VkBufferCollectionFUCHSIA" }
#[cfg(feature = "ext_opacity_micromap")]
handle_nondispatchable! { MicromapEXT , MicromapEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html>" , "VkMicromapEXT" }
#[cfg(feature = "ext_tensors")]
handle_nondispatchable! { TensorViewARM , TensorViewARM , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html>" , "VkTensorViewARM" }
#[cfg(feature = "ext_optical_flow")]
handle_nondispatchable! { OpticalFlowSessionNV , OpticalFlowSessionNV , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html>" , "VkOpticalFlowSessionNV" }
#[cfg(feature = "ext_shader_object")]
handle_nondispatchable! { ShaderEXT , ShaderEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html>" , "VkShaderEXT" }
#[cfg(feature = "ext_pipeline_binary")]
handle_nondispatchable! { PipelineBinaryKHR , PipelineBinaryKHR , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html>" , "VkPipelineBinaryKHR" }
#[cfg(feature = "ext_data_graph")]
handle_nondispatchable! { DataGraphPipelineSessionARM , DataGraphPipelineSessionARM , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html>" , "VkDataGraphPipelineSessionARM" }
#[cfg(feature = "ext_external_compute_queue")]
handle_dispatchable! { ExternalComputeQueueNV , ExternalComputeQueueNV , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html>" , "VkExternalComputeQueueNV" }
#[cfg(feature = "ext_device_generated_commands")]
handle_nondispatchable! { IndirectCommandsLayoutEXT , IndirectCommandsLayoutEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html>" , "VkIndirectCommandsLayoutEXT" }
#[cfg(feature = "ext_device_generated_commands")]
handle_nondispatchable! { IndirectExecutionSetEXT , IndirectExecutionSetEXT , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html>" , "VkIndirectExecutionSetEXT" }
#[cfg(feature = "ext_shader_instrumentation")]
handle_nondispatchable! { ShaderInstrumentationARM , ShaderInstrumentationARM , doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html>" , "VkShaderInstrumentationARM" }
