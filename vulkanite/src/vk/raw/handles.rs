use crate::private;
use crate::{vk::ObjectType, Handle};
use core::fmt;
use std::num::{NonZeroU64, NonZeroUsize};
#[macro_use]
mod macros;
handle_dispatchable! { Instance , Instance , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkInstance.html>" , "VkInstance" }
handle_dispatchable! { PhysicalDevice , PhysicalDevice , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPhysicalDevice.html>" , "VkPhysicalDevice" }
handle_dispatchable! { Device , Device , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDevice.html>" , "VkDevice" }
handle_dispatchable! { Queue , Queue , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkQueue.html>" , "VkQueue" }
handle_nondispatchable! { DeviceMemory , DeviceMemory , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDeviceMemory.html>" , "VkDeviceMemory" }
handle_nondispatchable! { Fence , Fence , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkFence.html>" , "VkFence" }
handle_nondispatchable! { Semaphore , Semaphore , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkSemaphore.html>" , "VkSemaphore" }
handle_nondispatchable! { QueryPool , QueryPool , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkQueryPool.html>" , "VkQueryPool" }
handle_nondispatchable! { Buffer , Buffer , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkBuffer.html>" , "VkBuffer" }
handle_nondispatchable! { Image , Image , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkImage.html>" , "VkImage" }
handle_nondispatchable! { ImageView , ImageView , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkImageView.html>" , "VkImageView" }
handle_nondispatchable! { CommandPool , CommandPool , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkCommandPool.html>" , "VkCommandPool" }
handle_dispatchable! { CommandBuffer , CommandBuffer , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkCommandBuffer.html>" , "VkCommandBuffer" }
handle_nondispatchable! { Event , Event , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkEvent.html>" , "VkEvent" }
handle_nondispatchable! { BufferView , BufferView , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkBufferView.html>" , "VkBufferView" }
handle_nondispatchable! { ShaderModule , ShaderModule , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkShaderModule.html>" , "VkShaderModule" }
handle_nondispatchable! { PipelineCache , PipelineCache , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPipelineCache.html>" , "VkPipelineCache" }
handle_nondispatchable! { Pipeline , Pipeline , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPipeline.html>" , "VkPipeline" }
handle_nondispatchable! { PipelineLayout , PipelineLayout , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPipelineLayout.html>" , "VkPipelineLayout" }
handle_nondispatchable! { Sampler , Sampler , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkSampler.html>" , "VkSampler" }
handle_nondispatchable! { DescriptorPool , DescriptorPool , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDescriptorPool.html>" , "VkDescriptorPool" }
handle_nondispatchable! { DescriptorSet , DescriptorSet , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDescriptorSet.html>" , "VkDescriptorSet" }
handle_nondispatchable! { DescriptorSetLayout , DescriptorSetLayout , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDescriptorSetLayout.html>" , "VkDescriptorSetLayout" }
handle_nondispatchable! { Framebuffer , Framebuffer , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkFramebuffer.html>" , "VkFramebuffer" }
handle_nondispatchable! { RenderPass , RenderPass , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkRenderPass.html>" , "VkRenderPass" }
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
handle_nondispatchable! { DescriptorUpdateTemplate , DescriptorUpdateTemplate , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplate.html>" , "VkDescriptorUpdateTemplate" }
#[cfg(any(feature = "ext_descriptor_update_template", feature = "version_1_1"))]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
handle_nondispatchable! { SamplerYcbcrConversion , SamplerYcbcrConversion , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkSamplerYcbcrConversion.html>" , "VkSamplerYcbcrConversion" }
#[cfg(any(feature = "ext_sampler_ycbcr_conversion", feature = "version_1_1"))]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
handle_nondispatchable! { PrivateDataSlot , PrivateDataSlot , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPrivateDataSlot.html>" , "VkPrivateDataSlot" }
#[cfg(any(feature = "ext_private_data", feature = "version_1_3"))]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPrivateDataSlotEXT.html>"]
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;
#[cfg(feature = "ext_surface")]
handle_nondispatchable! { SurfaceKHR , SurfaceKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkSurfaceKHR.html>" , "VkSurfaceKHR" }
#[cfg(feature = "ext_swapchain")]
handle_nondispatchable! { SwapchainKHR , SwapchainKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkSwapchainKHR.html>" , "VkSwapchainKHR" }
#[cfg(feature = "ext_display")]
handle_nondispatchable! { DisplayKHR , DisplayKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDisplayKHR.html>" , "VkDisplayKHR" }
#[cfg(feature = "ext_display")]
handle_nondispatchable! { DisplayModeKHR , DisplayModeKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDisplayModeKHR.html>" , "VkDisplayModeKHR" }
#[cfg(feature = "ext_debug_report")]
handle_nondispatchable! { DebugReportCallbackEXT , DebugReportCallbackEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDebugReportCallbackEXT.html>" , "VkDebugReportCallbackEXT" }
#[cfg(feature = "ext_binary_import")]
handle_nondispatchable! { CuModuleNVX , CuModuleNVX , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkCuModuleNVX.html>" , "VkCuModuleNVX" }
#[cfg(feature = "ext_binary_import")]
handle_nondispatchable! { CuFunctionNVX , CuFunctionNVX , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkCuFunctionNVX.html>" , "VkCuFunctionNVX" }
#[cfg(feature = "ext_debug_utils")]
handle_nondispatchable! { DebugUtilsMessengerEXT , DebugUtilsMessengerEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDebugUtilsMessengerEXT.html>" , "VkDebugUtilsMessengerEXT" }
#[cfg(any(feature = "ext_descriptor_heap", feature = "ext_tensors"))]
handle_nondispatchable! { TensorARM , TensorARM , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkTensorARM.html>" , "VkTensorARM" }
#[cfg(feature = "ext_acceleration_structure")]
handle_nondispatchable! { AccelerationStructureKHR , AccelerationStructureKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkAccelerationStructureKHR.html>" , "VkAccelerationStructureKHR" }
#[cfg(feature = "ext_validation_cache")]
handle_nondispatchable! { ValidationCacheEXT , ValidationCacheEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkValidationCacheEXT.html>" , "VkValidationCacheEXT" }
#[cfg(feature = "ext_ray_tracing")]
handle_nondispatchable! { AccelerationStructureNV , AccelerationStructureNV , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkAccelerationStructureNV.html>" , "VkAccelerationStructureNV" }
#[cfg(feature = "ext_performance_query")]
handle_nondispatchable! { PerformanceConfigurationINTEL , PerformanceConfigurationINTEL , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPerformanceConfigurationINTEL.html>" , "VkPerformanceConfigurationINTEL" }
#[cfg(feature = "ext_deferred_host_operations")]
handle_nondispatchable! { DeferredOperationKHR , DeferredOperationKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDeferredOperationKHR.html>" , "VkDeferredOperationKHR" }
#[cfg(feature = "ext_device_generated_commands")]
handle_nondispatchable! { IndirectCommandsLayoutNV , IndirectCommandsLayoutNV , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutNV.html>" , "VkIndirectCommandsLayoutNV" }
#[cfg(feature = "ext_cuda_kernel_launch")]
handle_nondispatchable! { CudaModuleNV , CudaModuleNV , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkCudaModuleNV.html>" , "VkCudaModuleNV" }
#[cfg(feature = "ext_cuda_kernel_launch")]
handle_nondispatchable! { CudaFunctionNV , CudaFunctionNV , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkCudaFunctionNV.html>" , "VkCudaFunctionNV" }
#[cfg(feature = "ext_buffer_collection")]
handle_nondispatchable! { BufferCollectionFUCHSIA , BufferCollectionFUCHSIA , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkBufferCollectionFUCHSIA.html>" , "VkBufferCollectionFUCHSIA" }
#[cfg(feature = "ext_opacity_micromap")]
handle_nondispatchable! { MicromapEXT , MicromapEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkMicromapEXT.html>" , "VkMicromapEXT" }
#[cfg(feature = "ext_tensors")]
handle_nondispatchable! { TensorViewARM , TensorViewARM , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkTensorViewARM.html>" , "VkTensorViewARM" }
#[cfg(feature = "ext_optical_flow")]
handle_nondispatchable! { OpticalFlowSessionNV , OpticalFlowSessionNV , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkOpticalFlowSessionNV.html>" , "VkOpticalFlowSessionNV" }
#[cfg(feature = "ext_shader_object")]
handle_nondispatchable! { ShaderEXT , ShaderEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkShaderEXT.html>" , "VkShaderEXT" }
#[cfg(feature = "ext_pipeline_binary")]
handle_nondispatchable! { PipelineBinaryKHR , PipelineBinaryKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkPipelineBinaryKHR.html>" , "VkPipelineBinaryKHR" }
#[cfg(feature = "ext_data_graph")]
handle_nondispatchable! { DataGraphPipelineSessionARM , DataGraphPipelineSessionARM , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkDataGraphPipelineSessionARM.html>" , "VkDataGraphPipelineSessionARM" }
#[cfg(feature = "ext_external_compute_queue")]
handle_dispatchable! { ExternalComputeQueueNV , ExternalComputeQueueNV , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkExternalComputeQueueNV.html>" , "VkExternalComputeQueueNV" }
#[cfg(feature = "ext_device_generated_commands")]
handle_nondispatchable! { IndirectCommandsLayoutEXT , IndirectCommandsLayoutEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutEXT.html>" , "VkIndirectCommandsLayoutEXT" }
#[cfg(feature = "ext_device_generated_commands")]
handle_nondispatchable! { IndirectExecutionSetEXT , IndirectExecutionSetEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkIndirectExecutionSetEXT.html>" , "VkIndirectExecutionSetEXT" }
#[cfg(feature = "ext_shader_instrumentation")]
handle_nondispatchable! { ShaderInstrumentationARM , ShaderInstrumentationARM , doc = "<https://www.khronos.org/registry/vulkan/specs/latest/man/html/VkShaderInstrumentationARM.html>" , "VkShaderInstrumentationARM" }
