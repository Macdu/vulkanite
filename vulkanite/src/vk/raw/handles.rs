use crate::private;
use crate::{vk::ObjectType, Handle};
use core::fmt;
use std::num::{NonZeroU64, NonZeroUsize};
#[macro_use]
mod macros;
handle_dispatchable! { Instance , Instance , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstance.html>" , "VkInstance" }
handle_dispatchable! { PhysicalDevice , PhysicalDevice , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice.html>" , "VkPhysicalDevice" }
handle_dispatchable! { Device , Device , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevice.html>" , "VkDevice" }
handle_dispatchable! { Queue , Queue , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueue.html>" , "VkQueue" }
handle_nondispatchable! { DeviceMemory , DeviceMemory , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemory.html>" , "VkDeviceMemory" }
handle_nondispatchable! { Fence , Fence , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFence.html>" , "VkFence" }
handle_nondispatchable! { Semaphore , Semaphore , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphore.html>" , "VkSemaphore" }
handle_nondispatchable! { Event , Event , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEvent.html>" , "VkEvent" }
handle_nondispatchable! { QueryPool , QueryPool , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPool.html>" , "VkQueryPool" }
handle_nondispatchable! { Buffer , Buffer , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuffer.html>" , "VkBuffer" }
handle_nondispatchable! { BufferView , BufferView , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferView.html>" , "VkBufferView" }
handle_nondispatchable! { Image , Image , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImage.html>" , "VkImage" }
handle_nondispatchable! { ImageView , ImageView , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageView.html>" , "VkImageView" }
handle_nondispatchable! { ShaderModule , ShaderModule , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModule.html>" , "VkShaderModule" }
handle_nondispatchable! { PipelineCache , PipelineCache , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html>" , "VkPipelineCache" }
handle_nondispatchable! { Pipeline , Pipeline , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipeline.html>" , "VkPipeline" }
handle_nondispatchable! { PipelineLayout , PipelineLayout , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html>" , "VkPipelineLayout" }
handle_nondispatchable! { Sampler , Sampler , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampler.html>" , "VkSampler" }
handle_nondispatchable! { DescriptorPool , DescriptorPool , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html>" , "VkDescriptorPool" }
handle_nondispatchable! { DescriptorSet , DescriptorSet , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html>" , "VkDescriptorSet" }
handle_nondispatchable! { DescriptorSetLayout , DescriptorSetLayout , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html>" , "VkDescriptorSetLayout" }
handle_nondispatchable! { Framebuffer , Framebuffer , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebuffer.html>" , "VkFramebuffer" }
handle_nondispatchable! { RenderPass , RenderPass , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html>" , "VkRenderPass" }
handle_nondispatchable! { CommandPool , CommandPool , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPool.html>" , "VkCommandPool" }
handle_dispatchable! { CommandBuffer , CommandBuffer , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBuffer.html>" , "VkCommandBuffer" }
handle_nondispatchable! { SamplerYcbcrConversion , SamplerYcbcrConversion , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversion.html>" , "VkSamplerYcbcrConversion" }
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionKHR.html>"]
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
handle_nondispatchable! { DescriptorUpdateTemplate , DescriptorUpdateTemplate , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html>" , "VkDescriptorUpdateTemplate" }
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateKHR.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
handle_nondispatchable! { PrivateDataSlot , PrivateDataSlot , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlot.html>" , "VkPrivateDataSlot" }
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotEXT.html>"]
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;
handle_nondispatchable! { SurfaceKHR , SurfaceKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html>" , "VkSurfaceKHR" }
handle_nondispatchable! { SwapchainKHR , SwapchainKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html>" , "VkSwapchainKHR" }
handle_nondispatchable! { DisplayKHR , DisplayKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayKHR.html>" , "VkDisplayKHR" }
handle_nondispatchable! { DisplayModeKHR , DisplayModeKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeKHR.html>" , "VkDisplayModeKHR" }
handle_nondispatchable! { DebugReportCallbackEXT , DebugReportCallbackEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackEXT.html>" , "VkDebugReportCallbackEXT" }
handle_nondispatchable! { CuModuleNVX , CuModuleNVX , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuModuleNVX.html>" , "VkCuModuleNVX" }
handle_nondispatchable! { CuFunctionNVX , CuFunctionNVX , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCuFunctionNVX.html>" , "VkCuFunctionNVX" }
handle_nondispatchable! { DebugUtilsMessengerEXT , DebugUtilsMessengerEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerEXT.html>" , "VkDebugUtilsMessengerEXT" }
handle_nondispatchable! { AccelerationStructureKHR , AccelerationStructureKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html>" , "VkAccelerationStructureKHR" }
handle_nondispatchable! { ValidationCacheEXT , ValidationCacheEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html>" , "VkValidationCacheEXT" }
handle_nondispatchable! { AccelerationStructureNV , AccelerationStructureNV , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureNV.html>" , "VkAccelerationStructureNV" }
handle_nondispatchable! { PerformanceConfigurationINTEL , PerformanceConfigurationINTEL , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html>" , "VkPerformanceConfigurationINTEL" }
handle_nondispatchable! { DeferredOperationKHR , DeferredOperationKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html>" , "VkDeferredOperationKHR" }
handle_nondispatchable! { IndirectCommandsLayoutNV , IndirectCommandsLayoutNV , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html>" , "VkIndirectCommandsLayoutNV" }
handle_nondispatchable! { CudaModuleNV , CudaModuleNV , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCudaModuleNV.html>" , "VkCudaModuleNV" }
handle_nondispatchable! { CudaFunctionNV , CudaFunctionNV , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCudaFunctionNV.html>" , "VkCudaFunctionNV" }
handle_nondispatchable! { BufferCollectionFUCHSIA , BufferCollectionFUCHSIA , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionFUCHSIA.html>" , "VkBufferCollectionFUCHSIA" }
handle_nondispatchable! { MicromapEXT , MicromapEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapEXT.html>" , "VkMicromapEXT" }
handle_nondispatchable! { OpticalFlowSessionNV , OpticalFlowSessionNV , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionNV.html>" , "VkOpticalFlowSessionNV" }
handle_nondispatchable! { ShaderEXT , ShaderEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderEXT.html>" , "VkShaderEXT" }
handle_nondispatchable! { PipelineBinaryKHR , PipelineBinaryKHR , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineBinaryKHR.html>" , "VkPipelineBinaryKHR" }
handle_nondispatchable! { IndirectCommandsLayoutEXT , IndirectCommandsLayoutEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutEXT.html>" , "VkIndirectCommandsLayoutEXT" }
handle_nondispatchable! { IndirectExecutionSetEXT , IndirectExecutionSetEXT , doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectExecutionSetEXT.html>" , "VkIndirectExecutionSetEXT" }
