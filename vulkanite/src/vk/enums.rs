use bitflags::bitflags;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResult.html>"]
#[doc(alias = "VkResult")]
#[repr(i32)]
pub enum Status {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorUnknown = -13,
    ErrorOutOfPoolMemory = -1000069000,
    ErrorInvalidExternalHandle = -1000072003,
    ErrorFragmentation = -1000161000,
    ErrorInvalidOpaqueCaptureAddress = -1000257000,
    PipelineCompileRequired = 1000297000,
    ErrorSurfaceLostKHR = -1000000000,
    ErrorNativeWindowInUseKHR = -1000000001,
    SuboptimalKHR = 1000001003,
    ErrorOutOfDateKHR = -1000001004,
    ErrorIncompatibleDisplayKHR = -1000003001,
    ErrorValidationFailedEXT = -1000011001,
    ErrorInvalidShaderNV = -1000012000,
    ErrorInvalidDrmFormatModifierPlaneLayoutEXT = -1000158000,
    ErrorNotPermittedKHR = -1000174001,
    ErrorFullScreenExclusiveModeLostEXT = -1000255000,
    ThreadIdleKHR = 1000268000,
    ThreadDoneKHR = 1000268001,
    OperationDeferredKHR = 1000268002,
    OperationNotDeferredKHR = 1000268003,
    ErrorCompressionExhaustedEXT = -1000338000,
    IncompatibleShaderBinaryEXT = 1000482000,
}
#[allow(non_upper_case_globals)]
impl Status {
    pub const ErrorOutOfPoolMemoryKHR: Self = Self::ErrorOutOfPoolMemory;
    pub const ErrorInvalidExternalHandleKHR: Self = Self::ErrorInvalidExternalHandle;
    pub const ErrorFragmentationEXT: Self = Self::ErrorFragmentation;
    pub const ErrorNotPermittedEXT: Self = Self::ErrorNotPermittedKHR;
    pub const ErrorInvalidDeviceAddressEXT: Self = Self::ErrorInvalidOpaqueCaptureAddress;
    pub const ErrorInvalidOpaqueCaptureAddressKHR: Self = Self::ErrorInvalidOpaqueCaptureAddress;
    pub const PipelineCompileRequiredEXT: Self = Self::PipelineCompileRequired;
    pub const ErrorPipelineCompileRequiredEXT: Self = Self::PipelineCompileRequired;
    pub const ErrorIncompatibleShaderBinaryEXT: Self = Self::IncompatibleShaderBinaryEXT;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStructureType.html>"]
#[doc(alias = "VkStructureType")]
#[repr(u32)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    LoaderInstanceCreateInfo = 47,
    LoaderDeviceCreateInfo = 48,
    PhysicalDeviceSubgroupProperties = 1000094000,
    BindBufferMemoryInfo = 1000157000,
    BindImageMemoryInfo = 1000157001,
    PhysicalDevice16BitStorageFeatures = 1000083000,
    MemoryDedicatedRequirements = 1000127000,
    MemoryDedicatedAllocateInfo = 1000127001,
    MemoryAllocateFlagsInfo = 1000060000,
    DeviceGroupRenderPassBeginInfo = 1000060003,
    DeviceGroupCommandBufferBeginInfo = 1000060004,
    DeviceGroupSubmitInfo = 1000060005,
    DeviceGroupBindSparseInfo = 1000060006,
    BindBufferMemoryDeviceGroupInfo = 1000060013,
    BindImageMemoryDeviceGroupInfo = 1000060014,
    PhysicalDeviceGroupProperties = 1000070000,
    DeviceGroupDeviceCreateInfo = 1000070001,
    BufferMemoryRequirementsInfo2 = 1000146000,
    ImageMemoryRequirementsInfo2 = 1000146001,
    ImageSparseMemoryRequirementsInfo2 = 1000146002,
    MemoryRequirements2 = 1000146003,
    SparseImageMemoryRequirements2 = 1000146004,
    PhysicalDeviceFeatures2 = 1000059000,
    PhysicalDeviceProperties2 = 1000059001,
    FormatProperties2 = 1000059002,
    ImageFormatProperties2 = 1000059003,
    PhysicalDeviceImageFormatInfo2 = 1000059004,
    QueueFamilyProperties2 = 1000059005,
    PhysicalDeviceMemoryProperties2 = 1000059006,
    SparseImageFormatProperties2 = 1000059007,
    PhysicalDeviceSparseImageFormatInfo2 = 1000059008,
    PhysicalDevicePointClippingProperties = 1000117000,
    RenderPassInputAttachmentAspectCreateInfo = 1000117001,
    ImageViewUsageCreateInfo = 1000117002,
    PipelineTessellationDomainOriginStateCreateInfo = 1000117003,
    RenderPassMultiviewCreateInfo = 1000053000,
    PhysicalDeviceMultiviewFeatures = 1000053001,
    PhysicalDeviceMultiviewProperties = 1000053002,
    PhysicalDeviceVariablePointersFeatures = 1000120000,
    ProtectedSubmitInfo = 1000145000,
    PhysicalDeviceProtectedMemoryFeatures = 1000145001,
    PhysicalDeviceProtectedMemoryProperties = 1000145002,
    DeviceQueueInfo2 = 1000145003,
    SamplerYcbcrConversionCreateInfo = 1000156000,
    SamplerYcbcrConversionInfo = 1000156001,
    BindImagePlaneMemoryInfo = 1000156002,
    ImagePlaneMemoryRequirementsInfo = 1000156003,
    PhysicalDeviceSamplerYcbcrConversionFeatures = 1000156004,
    SamplerYcbcrConversionImageFormatProperties = 1000156005,
    DescriptorUpdateTemplateCreateInfo = 1000085000,
    PhysicalDeviceExternalImageFormatInfo = 1000071000,
    ExternalImageFormatProperties = 1000071001,
    PhysicalDeviceExternalBufferInfo = 1000071002,
    ExternalBufferProperties = 1000071003,
    PhysicalDeviceIdProperties = 1000071004,
    ExternalMemoryBufferCreateInfo = 1000072000,
    ExternalMemoryImageCreateInfo = 1000072001,
    ExportMemoryAllocateInfo = 1000072002,
    PhysicalDeviceExternalFenceInfo = 1000112000,
    ExternalFenceProperties = 1000112001,
    ExportFenceCreateInfo = 1000113000,
    ExportSemaphoreCreateInfo = 1000077000,
    PhysicalDeviceExternalSemaphoreInfo = 1000076000,
    ExternalSemaphoreProperties = 1000076001,
    PhysicalDeviceMaintenance3Properties = 1000168000,
    DescriptorSetLayoutSupport = 1000168001,
    PhysicalDeviceShaderDrawParametersFeatures = 1000063000,
    PhysicalDeviceVulkan11Features = 49,
    PhysicalDeviceVulkan11Properties = 50,
    PhysicalDeviceVulkan12Features = 51,
    PhysicalDeviceVulkan12Properties = 52,
    ImageFormatListCreateInfo = 1000147000,
    AttachmentDescription2 = 1000109000,
    AttachmentReference2 = 1000109001,
    SubpassDescription2 = 1000109002,
    SubpassDependency2 = 1000109003,
    RenderPassCreateInfo2 = 1000109004,
    SubpassBeginInfo = 1000109005,
    SubpassEndInfo = 1000109006,
    PhysicalDevice8BitStorageFeatures = 1000177000,
    PhysicalDeviceDriverProperties = 1000196000,
    PhysicalDeviceShaderAtomicInt64Features = 1000180000,
    PhysicalDeviceShaderFloat16Int8Features = 1000082000,
    PhysicalDeviceFloatControlsProperties = 1000197000,
    DescriptorSetLayoutBindingFlagsCreateInfo = 1000161000,
    PhysicalDeviceDescriptorIndexingFeatures = 1000161001,
    PhysicalDeviceDescriptorIndexingProperties = 1000161002,
    DescriptorSetVariableDescriptorCountAllocateInfo = 1000161003,
    DescriptorSetVariableDescriptorCountLayoutSupport = 1000161004,
    PhysicalDeviceDepthStencilResolveProperties = 1000199000,
    SubpassDescriptionDepthStencilResolve = 1000199001,
    PhysicalDeviceScalarBlockLayoutFeatures = 1000221000,
    ImageStencilUsageCreateInfo = 1000246000,
    PhysicalDeviceSamplerFilterMinmaxProperties = 1000130000,
    SamplerReductionModeCreateInfo = 1000130001,
    PhysicalDeviceVulkanMemoryModelFeatures = 1000211000,
    PhysicalDeviceImagelessFramebufferFeatures = 1000108000,
    FramebufferAttachmentsCreateInfo = 1000108001,
    FramebufferAttachmentImageInfo = 1000108002,
    RenderPassAttachmentBeginInfo = 1000108003,
    PhysicalDeviceUniformBufferStandardLayoutFeatures = 1000253000,
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures = 1000175000,
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures = 1000241000,
    AttachmentReferenceStencilLayout = 1000241001,
    AttachmentDescriptionStencilLayout = 1000241002,
    PhysicalDeviceHostQueryResetFeatures = 1000261000,
    PhysicalDeviceTimelineSemaphoreFeatures = 1000207000,
    PhysicalDeviceTimelineSemaphoreProperties = 1000207001,
    SemaphoreTypeCreateInfo = 1000207002,
    TimelineSemaphoreSubmitInfo = 1000207003,
    SemaphoreWaitInfo = 1000207004,
    SemaphoreSignalInfo = 1000207005,
    PhysicalDeviceBufferDeviceAddressFeatures = 1000257000,
    BufferDeviceAddressInfo = 1000244001,
    BufferOpaqueCaptureAddressCreateInfo = 1000257002,
    MemoryOpaqueCaptureAddressAllocateInfo = 1000257003,
    DeviceMemoryOpaqueCaptureAddressInfo = 1000257004,
    PhysicalDeviceVulkan13Features = 53,
    PhysicalDeviceVulkan13Properties = 54,
    PipelineCreationFeedbackCreateInfo = 1000192000,
    PhysicalDeviceShaderTerminateInvocationFeatures = 1000215000,
    PhysicalDeviceToolProperties = 1000245000,
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures = 1000276000,
    PhysicalDevicePrivateDataFeatures = 1000295000,
    DevicePrivateDataCreateInfo = 1000295001,
    PrivateDataSlotCreateInfo = 1000295002,
    PhysicalDevicePipelineCreationCacheControlFeatures = 1000297000,
    MemoryBarrier2 = 1000314000,
    BufferMemoryBarrier2 = 1000314001,
    ImageMemoryBarrier2 = 1000314002,
    DependencyInfo = 1000314003,
    SubmitInfo2 = 1000314004,
    SemaphoreSubmitInfo = 1000314005,
    CommandBufferSubmitInfo = 1000314006,
    PhysicalDeviceSynchronization2Features = 1000314007,
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures = 1000325000,
    PhysicalDeviceImageRobustnessFeatures = 1000335000,
    CopyBufferInfo2 = 1000337000,
    CopyImageInfo2 = 1000337001,
    CopyBufferToImageInfo2 = 1000337002,
    CopyImageToBufferInfo2 = 1000337003,
    BlitImageInfo2 = 1000337004,
    ResolveImageInfo2 = 1000337005,
    BufferCopy2 = 1000337006,
    ImageCopy2 = 1000337007,
    ImageBlit2 = 1000337008,
    BufferImageCopy2 = 1000337009,
    ImageResolve2 = 1000337010,
    PhysicalDeviceSubgroupSizeControlProperties = 1000225000,
    PipelineShaderStageRequiredSubgroupSizeCreateInfo = 1000225001,
    PhysicalDeviceSubgroupSizeControlFeatures = 1000225002,
    PhysicalDeviceInlineUniformBlockFeatures = 1000138000,
    PhysicalDeviceInlineUniformBlockProperties = 1000138001,
    WriteDescriptorSetInlineUniformBlock = 1000138002,
    DescriptorPoolInlineUniformBlockCreateInfo = 1000138003,
    PhysicalDeviceTextureCompressionAstcHdrFeatures = 1000066000,
    RenderingInfo = 1000044000,
    RenderingAttachmentInfo = 1000044001,
    PipelineRenderingCreateInfo = 1000044002,
    PhysicalDeviceDynamicRenderingFeatures = 1000044003,
    CommandBufferInheritanceRenderingInfo = 1000044004,
    PhysicalDeviceShaderIntegerDotProductFeatures = 1000280000,
    PhysicalDeviceShaderIntegerDotProductProperties = 1000280001,
    PhysicalDeviceTexelBufferAlignmentProperties = 1000281001,
    FormatProperties3 = 1000360000,
    PhysicalDeviceMaintenance4Features = 1000413000,
    PhysicalDeviceMaintenance4Properties = 1000413001,
    DeviceBufferMemoryRequirements = 1000413002,
    DeviceImageMemoryRequirements = 1000413003,
    SwapchainCreateInfoKHR = 1000001000,
    PresentInfoKHR = 1000001001,
    DeviceGroupPresentCapabilitiesKHR = 1000060007,
    ImageSwapchainCreateInfoKHR = 1000060008,
    BindImageMemorySwapchainInfoKHR = 1000060009,
    AcquireNextImageInfoKHR = 1000060010,
    DeviceGroupPresentInfoKHR = 1000060011,
    DeviceGroupSwapchainCreateInfoKHR = 1000060012,
    DisplayModeCreateInfoKHR = 1000002000,
    DisplaySurfaceCreateInfoKHR = 1000002001,
    DisplayPresentInfoKHR = 1000003000,
    XlibSurfaceCreateInfoKHR = 1000004000,
    XcbSurfaceCreateInfoKHR = 1000005000,
    WaylandSurfaceCreateInfoKHR = 1000006000,
    AndroidSurfaceCreateInfoKHR = 1000008000,
    Win32SurfaceCreateInfoKHR = 1000009000,
    DebugReportCallbackCreateInfoEXT = 1000011000,
    PipelineRasterizationStateRasterizationOrderAMD = 1000018000,
    DebugMarkerObjectNameInfoEXT = 1000022000,
    DebugMarkerObjectTagInfoEXT = 1000022001,
    DebugMarkerMarkerInfoEXT = 1000022002,
    DedicatedAllocationImageCreateInfoNV = 1000026000,
    DedicatedAllocationBufferCreateInfoNV = 1000026001,
    DedicatedAllocationMemoryAllocateInfoNV = 1000026002,
    PhysicalDeviceTransformFeedbackFeaturesEXT = 1000028000,
    PhysicalDeviceTransformFeedbackPropertiesEXT = 1000028001,
    PipelineRasterizationStateStreamCreateInfoEXT = 1000028002,
    CuModuleCreateInfoNVX = 1000029000,
    CuFunctionCreateInfoNVX = 1000029001,
    CuLaunchInfoNVX = 1000029002,
    ImageViewHandleInfoNVX = 1000030000,
    ImageViewAddressPropertiesNVX = 1000030001,
    TextureLodGatherFormatPropertiesAMD = 1000041000,
    RenderingFragmentShadingRateAttachmentInfoKHR = 1000044006,
    RenderingFragmentDensityMapAttachmentInfoEXT = 1000044007,
    AttachmentSampleCountInfoAMD = 1000044008,
    MultiviewPerViewAttributesInfoNVX = 1000044009,
    StreamDescriptorSurfaceCreateInfoGGP = 1000049000,
    PhysicalDeviceCornerSampledImageFeaturesNV = 1000050000,
    ExternalMemoryImageCreateInfoNV = 1000056000,
    ExportMemoryAllocateInfoNV = 1000056001,
    ImportMemoryWin32HandleInfoNV = 1000057000,
    ExportMemoryWin32HandleInfoNV = 1000057001,
    Win32KeyedMutexAcquireReleaseInfoNV = 1000058000,
    ValidationFlagsEXT = 1000061000,
    ViSurfaceCreateInfoNN = 1000062000,
    ImageViewAstcDecodeModeEXT = 1000067000,
    PhysicalDeviceAstcDecodeFeaturesEXT = 1000067001,
    PipelineRobustnessCreateInfoEXT = 1000068000,
    PhysicalDevicePipelineRobustnessFeaturesEXT = 1000068001,
    PhysicalDevicePipelineRobustnessPropertiesEXT = 1000068002,
    ImportMemoryWin32HandleInfoKHR = 1000073000,
    ExportMemoryWin32HandleInfoKHR = 1000073001,
    MemoryWin32HandlePropertiesKHR = 1000073002,
    MemoryGetWin32HandleInfoKHR = 1000073003,
    ImportMemoryFdInfoKHR = 1000074000,
    MemoryFdPropertiesKHR = 1000074001,
    MemoryGetFdInfoKHR = 1000074002,
    Win32KeyedMutexAcquireReleaseInfoKHR = 1000075000,
    ImportSemaphoreWin32HandleInfoKHR = 1000078000,
    ExportSemaphoreWin32HandleInfoKHR = 1000078001,
    D3D12FenceSubmitInfoKHR = 1000078002,
    SemaphoreGetWin32HandleInfoKHR = 1000078003,
    ImportSemaphoreFdInfoKHR = 1000079000,
    SemaphoreGetFdInfoKHR = 1000079001,
    PhysicalDevicePushDescriptorPropertiesKHR = 1000080000,
    CommandBufferInheritanceConditionalRenderingInfoEXT = 1000081000,
    PhysicalDeviceConditionalRenderingFeaturesEXT = 1000081001,
    ConditionalRenderingBeginInfoEXT = 1000081002,
    PresentRegionsKHR = 1000084000,
    PipelineViewportWScalingStateCreateInfoNV = 1000087000,
    SurfaceCapabilities2EXT = 1000090000,
    DisplayPowerInfoEXT = 1000091000,
    DeviceEventInfoEXT = 1000091001,
    DisplayEventInfoEXT = 1000091002,
    SwapchainCounterCreateInfoEXT = 1000091003,
    PresentTimesInfoGOOGLE = 1000092000,
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX = 1000097000,
    PipelineViewportSwizzleStateCreateInfoNV = 1000098000,
    PhysicalDeviceDiscardRectanglePropertiesEXT = 1000099000,
    PipelineDiscardRectangleStateCreateInfoEXT = 1000099001,
    PhysicalDeviceConservativeRasterizationPropertiesEXT = 1000101000,
    PipelineRasterizationConservativeStateCreateInfoEXT = 1000101001,
    PhysicalDeviceDepthClipEnableFeaturesEXT = 1000102000,
    PipelineRasterizationDepthClipStateCreateInfoEXT = 1000102001,
    HdrMetadataEXT = 1000105000,
    PhysicalDeviceRelaxedLineRasterizationFeaturesIMG = 1000110000,
    SharedPresentSurfaceCapabilitiesKHR = 1000111000,
    ImportFenceWin32HandleInfoKHR = 1000114000,
    ExportFenceWin32HandleInfoKHR = 1000114001,
    FenceGetWin32HandleInfoKHR = 1000114002,
    ImportFenceFdInfoKHR = 1000115000,
    FenceGetFdInfoKHR = 1000115001,
    PhysicalDevicePerformanceQueryFeaturesKHR = 1000116000,
    PhysicalDevicePerformanceQueryPropertiesKHR = 1000116001,
    QueryPoolPerformanceCreateInfoKHR = 1000116002,
    PerformanceQuerySubmitInfoKHR = 1000116003,
    AcquireProfilingLockInfoKHR = 1000116004,
    PerformanceCounterKHR = 1000116005,
    PerformanceCounterDescriptionKHR = 1000116006,
    PerformanceQueryReservationInfoKHR = 1000116007,
    PhysicalDeviceSurfaceInfo2KHR = 1000119000,
    SurfaceCapabilities2KHR = 1000119001,
    SurfaceFormat2KHR = 1000119002,
    DisplayProperties2KHR = 1000121000,
    DisplayPlaneProperties2KHR = 1000121001,
    DisplayModeProperties2KHR = 1000121002,
    DisplayPlaneInfo2KHR = 1000121003,
    DisplayPlaneCapabilities2KHR = 1000121004,
    IosSurfaceCreateInfoMVK = 1000122000,
    MacosSurfaceCreateInfoMVK = 1000123000,
    DebugUtilsObjectNameInfoEXT = 1000128000,
    DebugUtilsObjectTagInfoEXT = 1000128001,
    DebugUtilsLabelEXT = 1000128002,
    DebugUtilsMessengerCallbackDataEXT = 1000128003,
    DebugUtilsMessengerCreateInfoEXT = 1000128004,
    AndroidHardwareBufferUsageANDROID = 1000129000,
    AndroidHardwareBufferPropertiesANDROID = 1000129001,
    AndroidHardwareBufferFormatPropertiesANDROID = 1000129002,
    ImportAndroidHardwareBufferInfoANDROID = 1000129003,
    MemoryGetAndroidHardwareBufferInfoANDROID = 1000129004,
    ExternalFormatANDROID = 1000129005,
    AndroidHardwareBufferFormatProperties2ANDROID = 1000129006,
    PhysicalDeviceShaderEnqueueFeaturesAMDX = 1000134000,
    PhysicalDeviceShaderEnqueuePropertiesAMDX = 1000134001,
    ExecutionGraphPipelineScratchSizeAMDX = 1000134002,
    ExecutionGraphPipelineCreateInfoAMDX = 1000134003,
    PipelineShaderStageNodeCreateInfoAMDX = 1000134004,
    SampleLocationsInfoEXT = 1000143000,
    RenderPassSampleLocationsBeginInfoEXT = 1000143001,
    PipelineSampleLocationsStateCreateInfoEXT = 1000143002,
    PhysicalDeviceSampleLocationsPropertiesEXT = 1000143003,
    MultisamplePropertiesEXT = 1000143004,
    PhysicalDeviceBlendOperationAdvancedFeaturesEXT = 1000148000,
    PhysicalDeviceBlendOperationAdvancedPropertiesEXT = 1000148001,
    PipelineColorBlendAdvancedStateCreateInfoEXT = 1000148002,
    PipelineCoverageToColorStateCreateInfoNV = 1000149000,
    WriteDescriptorSetAccelerationStructureKHR = 1000150007,
    AccelerationStructureBuildGeometryInfoKHR = 1000150000,
    AccelerationStructureDeviceAddressInfoKHR = 1000150002,
    AccelerationStructureGeometryAabbsDataKHR = 1000150003,
    AccelerationStructureGeometryInstancesDataKHR = 1000150004,
    AccelerationStructureGeometryTrianglesDataKHR = 1000150005,
    AccelerationStructureGeometryKHR = 1000150006,
    AccelerationStructureVersionInfoKHR = 1000150009,
    CopyAccelerationStructureInfoKHR = 1000150010,
    CopyAccelerationStructureToMemoryInfoKHR = 1000150011,
    CopyMemoryToAccelerationStructureInfoKHR = 1000150012,
    PhysicalDeviceAccelerationStructureFeaturesKHR = 1000150013,
    PhysicalDeviceAccelerationStructurePropertiesKHR = 1000150014,
    AccelerationStructureCreateInfoKHR = 1000150017,
    AccelerationStructureBuildSizesInfoKHR = 1000150020,
    PhysicalDeviceRayTracingPipelineFeaturesKHR = 1000347000,
    PhysicalDeviceRayTracingPipelinePropertiesKHR = 1000347001,
    RayTracingPipelineCreateInfoKHR = 1000150015,
    RayTracingShaderGroupCreateInfoKHR = 1000150016,
    RayTracingPipelineInterfaceCreateInfoKHR = 1000150018,
    PhysicalDeviceRayQueryFeaturesKHR = 1000348013,
    PipelineCoverageModulationStateCreateInfoNV = 1000152000,
    PhysicalDeviceShaderSmBuiltinsFeaturesNV = 1000154000,
    PhysicalDeviceShaderSmBuiltinsPropertiesNV = 1000154001,
    DrmFormatModifierPropertiesListEXT = 1000158000,
    PhysicalDeviceImageDrmFormatModifierInfoEXT = 1000158002,
    ImageDrmFormatModifierListCreateInfoEXT = 1000158003,
    ImageDrmFormatModifierExplicitCreateInfoEXT = 1000158004,
    ImageDrmFormatModifierPropertiesEXT = 1000158005,
    DrmFormatModifierPropertiesList2EXT = 1000158006,
    ValidationCacheCreateInfoEXT = 1000160000,
    ShaderModuleValidationCacheCreateInfoEXT = 1000160001,
    PhysicalDevicePortabilitySubsetFeaturesKHR = 1000163000,
    PhysicalDevicePortabilitySubsetPropertiesKHR = 1000163001,
    PipelineViewportShadingRateImageStateCreateInfoNV = 1000164000,
    PhysicalDeviceShadingRateImageFeaturesNV = 1000164001,
    PhysicalDeviceShadingRateImagePropertiesNV = 1000164002,
    PipelineViewportCoarseSampleOrderStateCreateInfoNV = 1000164005,
    RayTracingPipelineCreateInfoNV = 1000165000,
    AccelerationStructureCreateInfoNV = 1000165001,
    GeometryNV = 1000165003,
    GeometryTrianglesNV = 1000165004,
    GeometryAabbNV = 1000165005,
    BindAccelerationStructureMemoryInfoNV = 1000165006,
    WriteDescriptorSetAccelerationStructureNV = 1000165007,
    AccelerationStructureMemoryRequirementsInfoNV = 1000165008,
    PhysicalDeviceRayTracingPropertiesNV = 1000165009,
    RayTracingShaderGroupCreateInfoNV = 1000165011,
    AccelerationStructureInfoNV = 1000165012,
    PhysicalDeviceRepresentativeFragmentTestFeaturesNV = 1000166000,
    PipelineRepresentativeFragmentTestStateCreateInfoNV = 1000166001,
    PhysicalDeviceImageViewImageFormatInfoEXT = 1000170000,
    FilterCubicImageViewImageFormatPropertiesEXT = 1000170001,
    ImportMemoryHostPointerInfoEXT = 1000178000,
    MemoryHostPointerPropertiesEXT = 1000178001,
    PhysicalDeviceExternalMemoryHostPropertiesEXT = 1000178002,
    PhysicalDeviceShaderClockFeaturesKHR = 1000181000,
    PipelineCompilerControlCreateInfoAMD = 1000183000,
    PhysicalDeviceShaderCorePropertiesAMD = 1000185000,
    DeviceQueueGlobalPriorityCreateInfoKHR = 1000174000,
    PhysicalDeviceGlobalPriorityQueryFeaturesKHR = 1000388000,
    QueueFamilyGlobalPriorityPropertiesKHR = 1000388001,
    DeviceMemoryOverallocationCreateInfoAMD = 1000189000,
    PhysicalDeviceVertexAttributeDivisorPropertiesEXT = 1000190000,
    PresentFrameTokenGGP = 1000191000,
    PhysicalDeviceComputeShaderDerivativesFeaturesNV = 1000201000,
    PhysicalDeviceMeshShaderFeaturesNV = 1000202000,
    PhysicalDeviceMeshShaderPropertiesNV = 1000202001,
    PhysicalDeviceShaderImageFootprintFeaturesNV = 1000204000,
    PipelineViewportExclusiveScissorStateCreateInfoNV = 1000205000,
    PhysicalDeviceExclusiveScissorFeaturesNV = 1000205002,
    CheckpointDataNV = 1000206000,
    QueueFamilyCheckpointPropertiesNV = 1000206001,
    PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL = 1000209000,
    QueryPoolPerformanceQueryCreateInfoINTEL = 1000210000,
    InitializePerformanceApiInfoINTEL = 1000210001,
    PerformanceMarkerInfoINTEL = 1000210002,
    PerformanceStreamMarkerInfoINTEL = 1000210003,
    PerformanceOverrideInfoINTEL = 1000210004,
    PerformanceConfigurationAcquireInfoINTEL = 1000210005,
    PhysicalDevicePciBusInfoPropertiesEXT = 1000212000,
    DisplayNativeHdrSurfaceCapabilitiesAMD = 1000213000,
    SwapchainDisplayNativeHdrCreateInfoAMD = 1000213001,
    ImagepipeSurfaceCreateInfoFUCHSIA = 1000214000,
    MetalSurfaceCreateInfoEXT = 1000217000,
    PhysicalDeviceFragmentDensityMapFeaturesEXT = 1000218000,
    PhysicalDeviceFragmentDensityMapPropertiesEXT = 1000218001,
    RenderPassFragmentDensityMapCreateInfoEXT = 1000218002,
    FragmentShadingRateAttachmentInfoKHR = 1000226000,
    PipelineFragmentShadingRateStateCreateInfoKHR = 1000226001,
    PhysicalDeviceFragmentShadingRatePropertiesKHR = 1000226002,
    PhysicalDeviceFragmentShadingRateFeaturesKHR = 1000226003,
    PhysicalDeviceFragmentShadingRateKHR = 1000226004,
    PhysicalDeviceShaderCoreProperties2AMD = 1000227000,
    PhysicalDeviceCoherentMemoryFeaturesAMD = 1000229000,
    PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR = 1000232000,
    RenderingAttachmentLocationInfoKHR = 1000232001,
    RenderingInputAttachmentIndexInfoKHR = 1000232002,
    PhysicalDeviceShaderImageAtomicInt64FeaturesEXT = 1000234000,
    PhysicalDeviceShaderQuadControlFeaturesKHR = 1000235000,
    PhysicalDeviceMemoryBudgetPropertiesEXT = 1000237000,
    PhysicalDeviceMemoryPriorityFeaturesEXT = 1000238000,
    MemoryPriorityAllocateInfoEXT = 1000238001,
    SurfaceProtectedCapabilitiesKHR = 1000239000,
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV = 1000240000,
    PhysicalDeviceBufferDeviceAddressFeaturesEXT = 1000244000,
    BufferDeviceAddressCreateInfoEXT = 1000244002,
    ValidationFeaturesEXT = 1000247000,
    PhysicalDevicePresentWaitFeaturesKHR = 1000248000,
    PhysicalDeviceCooperativeMatrixFeaturesNV = 1000249000,
    CooperativeMatrixPropertiesNV = 1000249001,
    PhysicalDeviceCooperativeMatrixPropertiesNV = 1000249002,
    PhysicalDeviceCoverageReductionModeFeaturesNV = 1000250000,
    PipelineCoverageReductionStateCreateInfoNV = 1000250001,
    FramebufferMixedSamplesCombinationNV = 1000250002,
    PhysicalDeviceFragmentShaderInterlockFeaturesEXT = 1000251000,
    PhysicalDeviceYcbcrImageArraysFeaturesEXT = 1000252000,
    PhysicalDeviceProvokingVertexFeaturesEXT = 1000254000,
    PipelineRasterizationProvokingVertexStateCreateInfoEXT = 1000254001,
    PhysicalDeviceProvokingVertexPropertiesEXT = 1000254002,
    SurfaceFullScreenExclusiveInfoEXT = 1000255000,
    SurfaceCapabilitiesFullScreenExclusiveEXT = 1000255002,
    SurfaceFullScreenExclusiveWin32InfoEXT = 1000255001,
    HeadlessSurfaceCreateInfoEXT = 1000256000,
    PhysicalDeviceShaderAtomicFloatFeaturesEXT = 1000260000,
    PhysicalDeviceExtendedDynamicStateFeaturesEXT = 1000267000,
    PhysicalDevicePipelineExecutablePropertiesFeaturesKHR = 1000269000,
    PipelineInfoKHR = 1000269001,
    PipelineExecutablePropertiesKHR = 1000269002,
    PipelineExecutableInfoKHR = 1000269003,
    PipelineExecutableStatisticKHR = 1000269004,
    PipelineExecutableInternalRepresentationKHR = 1000269005,
    PhysicalDeviceHostImageCopyFeaturesEXT = 1000270000,
    PhysicalDeviceHostImageCopyPropertiesEXT = 1000270001,
    MemoryToImageCopyEXT = 1000270002,
    ImageToMemoryCopyEXT = 1000270003,
    CopyImageToMemoryInfoEXT = 1000270004,
    CopyMemoryToImageInfoEXT = 1000270005,
    HostImageLayoutTransitionInfoEXT = 1000270006,
    CopyImageToImageInfoEXT = 1000270007,
    SubresourceHostMemcpySizeEXT = 1000270008,
    HostImageCopyDevicePerformanceQueryEXT = 1000270009,
    MemoryMapInfoKHR = 1000271000,
    MemoryUnmapInfoKHR = 1000271001,
    PhysicalDeviceMapMemoryPlacedFeaturesEXT = 1000272000,
    PhysicalDeviceMapMemoryPlacedPropertiesEXT = 1000272001,
    MemoryMapPlacedInfoEXT = 1000272002,
    PhysicalDeviceShaderAtomicFloat2FeaturesEXT = 1000273000,
    SurfacePresentModeEXT = 1000274000,
    SurfacePresentScalingCapabilitiesEXT = 1000274001,
    SurfacePresentModeCompatibilityEXT = 1000274002,
    PhysicalDeviceSwapchainMaintenance1FeaturesEXT = 1000275000,
    SwapchainPresentFenceInfoEXT = 1000275001,
    SwapchainPresentModesCreateInfoEXT = 1000275002,
    SwapchainPresentModeInfoEXT = 1000275003,
    SwapchainPresentScalingCreateInfoEXT = 1000275004,
    ReleaseSwapchainImagesInfoEXT = 1000275005,
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNV = 1000277000,
    GraphicsShaderGroupCreateInfoNV = 1000277001,
    GraphicsPipelineShaderGroupsCreateInfoNV = 1000277002,
    IndirectCommandsLayoutTokenNV = 1000277003,
    IndirectCommandsLayoutCreateInfoNV = 1000277004,
    GeneratedCommandsInfoNV = 1000277005,
    GeneratedCommandsMemoryRequirementsInfoNV = 1000277006,
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNV = 1000277007,
    PhysicalDeviceInheritedViewportScissorFeaturesNV = 1000278000,
    CommandBufferInheritanceViewportScissorInfoNV = 1000278001,
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT = 1000281000,
    CommandBufferInheritanceRenderPassTransformInfoQCOM = 1000282000,
    RenderPassTransformBeginInfoQCOM = 1000282001,
    PhysicalDeviceDepthBiasControlFeaturesEXT = 1000283000,
    DepthBiasInfoEXT = 1000283001,
    DepthBiasRepresentationInfoEXT = 1000283002,
    PhysicalDeviceDeviceMemoryReportFeaturesEXT = 1000284000,
    DeviceDeviceMemoryReportCreateInfoEXT = 1000284001,
    DeviceMemoryReportCallbackDataEXT = 1000284002,
    PhysicalDeviceRobustness2FeaturesEXT = 1000286000,
    PhysicalDeviceRobustness2PropertiesEXT = 1000286001,
    SamplerCustomBorderColorCreateInfoEXT = 1000287000,
    PhysicalDeviceCustomBorderColorPropertiesEXT = 1000287001,
    PhysicalDeviceCustomBorderColorFeaturesEXT = 1000287002,
    PipelineLibraryCreateInfoKHR = 1000290000,
    PhysicalDevicePresentBarrierFeaturesNV = 1000292000,
    SurfaceCapabilitiesPresentBarrierNV = 1000292001,
    SwapchainPresentBarrierCreateInfoNV = 1000292002,
    PresentIdKHR = 1000294000,
    PhysicalDevicePresentIdFeaturesKHR = 1000294001,
    PhysicalDeviceDiagnosticsConfigFeaturesNV = 1000300000,
    DeviceDiagnosticsConfigCreateInfoNV = 1000300001,
    CudaModuleCreateInfoNV = 1000307000,
    CudaFunctionCreateInfoNV = 1000307001,
    CudaLaunchInfoNV = 1000307002,
    PhysicalDeviceCudaKernelLaunchFeaturesNV = 1000307003,
    PhysicalDeviceCudaKernelLaunchPropertiesNV = 1000307004,
    QueryLowLatencySupportNV = 1000310000,
    ExportMetalObjectCreateInfoEXT = 1000311000,
    ExportMetalObjectsInfoEXT = 1000311001,
    ExportMetalDeviceInfoEXT = 1000311002,
    ExportMetalCommandQueueInfoEXT = 1000311003,
    ExportMetalBufferInfoEXT = 1000311004,
    ImportMetalBufferInfoEXT = 1000311005,
    ExportMetalTextureInfoEXT = 1000311006,
    ImportMetalTextureInfoEXT = 1000311007,
    ExportMetalIoSurfaceInfoEXT = 1000311008,
    ImportMetalIoSurfaceInfoEXT = 1000311009,
    ExportMetalSharedEventInfoEXT = 1000311010,
    ImportMetalSharedEventInfoEXT = 1000311011,
    QueueFamilyCheckpointProperties2NV = 1000314008,
    CheckpointData2NV = 1000314009,
    PhysicalDeviceDescriptorBufferPropertiesEXT = 1000316000,
    PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT = 1000316001,
    PhysicalDeviceDescriptorBufferFeaturesEXT = 1000316002,
    DescriptorAddressInfoEXT = 1000316003,
    DescriptorGetInfoEXT = 1000316004,
    BufferCaptureDescriptorDataInfoEXT = 1000316005,
    ImageCaptureDescriptorDataInfoEXT = 1000316006,
    ImageViewCaptureDescriptorDataInfoEXT = 1000316007,
    SamplerCaptureDescriptorDataInfoEXT = 1000316008,
    OpaqueCaptureDescriptorDataCreateInfoEXT = 1000316010,
    DescriptorBufferBindingInfoEXT = 1000316011,
    DescriptorBufferBindingPushDescriptorBufferHandleEXT = 1000316012,
    AccelerationStructureCaptureDescriptorDataInfoEXT = 1000316009,
    PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT = 1000320000,
    PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT = 1000320001,
    GraphicsPipelineLibraryCreateInfoEXT = 1000320002,
    PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD = 1000321000,
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR = 1000203000,
    PhysicalDeviceFragmentShaderBarycentricPropertiesKHR = 1000322000,
    PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR = 1000323000,
    PhysicalDeviceFragmentShadingRateEnumsPropertiesNV = 1000326000,
    PhysicalDeviceFragmentShadingRateEnumsFeaturesNV = 1000326001,
    PipelineFragmentShadingRateEnumStateCreateInfoNV = 1000326002,
    AccelerationStructureGeometryMotionTrianglesDataNV = 1000327000,
    PhysicalDeviceRayTracingMotionBlurFeaturesNV = 1000327001,
    AccelerationStructureMotionInfoNV = 1000327002,
    PhysicalDeviceMeshShaderFeaturesEXT = 1000328000,
    PhysicalDeviceMeshShaderPropertiesEXT = 1000328001,
    PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT = 1000330000,
    PhysicalDeviceFragmentDensityMap2FeaturesEXT = 1000332000,
    PhysicalDeviceFragmentDensityMap2PropertiesEXT = 1000332001,
    CopyCommandTransformInfoQCOM = 1000333000,
    PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR = 1000336000,
    PhysicalDeviceImageCompressionControlFeaturesEXT = 1000338000,
    ImageCompressionControlEXT = 1000338001,
    ImageCompressionPropertiesEXT = 1000338004,
    PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT = 1000339000,
    PhysicalDevice4444FormatsFeaturesEXT = 1000340000,
    PhysicalDeviceFaultFeaturesEXT = 1000341000,
    DeviceFaultCountsEXT = 1000341001,
    DeviceFaultInfoEXT = 1000341002,
    PhysicalDeviceRgba10X6FormatsFeaturesEXT = 1000344000,
    DirectfbSurfaceCreateInfoEXT = 1000346000,
    PhysicalDeviceVertexInputDynamicStateFeaturesEXT = 1000352000,
    VertexInputBindingDescription2EXT = 1000352001,
    VertexInputAttributeDescription2EXT = 1000352002,
    PhysicalDeviceDrmPropertiesEXT = 1000353000,
    PhysicalDeviceAddressBindingReportFeaturesEXT = 1000354000,
    DeviceAddressBindingCallbackDataEXT = 1000354001,
    PhysicalDeviceDepthClipControlFeaturesEXT = 1000355000,
    PipelineViewportDepthClipControlCreateInfoEXT = 1000355001,
    PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT = 1000356000,
    ImportMemoryZirconHandleInfoFUCHSIA = 1000364000,
    MemoryZirconHandlePropertiesFUCHSIA = 1000364001,
    MemoryGetZirconHandleInfoFUCHSIA = 1000364002,
    ImportSemaphoreZirconHandleInfoFUCHSIA = 1000365000,
    SemaphoreGetZirconHandleInfoFUCHSIA = 1000365001,
    BufferCollectionCreateInfoFUCHSIA = 1000366000,
    ImportMemoryBufferCollectionFUCHSIA = 1000366001,
    BufferCollectionImageCreateInfoFUCHSIA = 1000366002,
    BufferCollectionPropertiesFUCHSIA = 1000366003,
    BufferConstraintsInfoFUCHSIA = 1000366004,
    BufferCollectionBufferCreateInfoFUCHSIA = 1000366005,
    ImageConstraintsInfoFUCHSIA = 1000366006,
    ImageFormatConstraintsInfoFUCHSIA = 1000366007,
    SysmemColorSpaceFUCHSIA = 1000366008,
    BufferCollectionConstraintsInfoFUCHSIA = 1000366009,
    SubpassShadingPipelineCreateInfoHUAWEI = 1000369000,
    PhysicalDeviceSubpassShadingFeaturesHUAWEI = 1000369001,
    PhysicalDeviceSubpassShadingPropertiesHUAWEI = 1000369002,
    PhysicalDeviceInvocationMaskFeaturesHUAWEI = 1000370000,
    MemoryGetRemoteAddressInfoNV = 1000371000,
    PhysicalDeviceExternalMemoryRdmaFeaturesNV = 1000371001,
    PipelinePropertiesIdentifierEXT = 1000372000,
    PhysicalDevicePipelinePropertiesFeaturesEXT = 1000372001,
    PhysicalDeviceFrameBoundaryFeaturesEXT = 1000375000,
    FrameBoundaryEXT = 1000375001,
    PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT = 1000376000,
    SubpassResolvePerformanceQueryEXT = 1000376001,
    MultisampledRenderToSingleSampledInfoEXT = 1000376002,
    PhysicalDeviceExtendedDynamicState2FeaturesEXT = 1000377000,
    ScreenSurfaceCreateInfoQNX = 1000378000,
    PhysicalDeviceColorWriteEnableFeaturesEXT = 1000381000,
    PipelineColorWriteCreateInfoEXT = 1000381001,
    PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT = 1000382000,
    PhysicalDeviceRayTracingMaintenance1FeaturesKHR = 1000386000,
    PhysicalDeviceImageViewMinLodFeaturesEXT = 1000391000,
    ImageViewMinLodCreateInfoEXT = 1000391001,
    PhysicalDeviceMultiDrawFeaturesEXT = 1000392000,
    PhysicalDeviceMultiDrawPropertiesEXT = 1000392001,
    PhysicalDeviceImage2DViewOf3DFeaturesEXT = 1000393000,
    PhysicalDeviceShaderTileImageFeaturesEXT = 1000395000,
    PhysicalDeviceShaderTileImagePropertiesEXT = 1000395001,
    MicromapBuildInfoEXT = 1000396000,
    MicromapVersionInfoEXT = 1000396001,
    CopyMicromapInfoEXT = 1000396002,
    CopyMicromapToMemoryInfoEXT = 1000396003,
    CopyMemoryToMicromapInfoEXT = 1000396004,
    PhysicalDeviceOpacityMicromapFeaturesEXT = 1000396005,
    PhysicalDeviceOpacityMicromapPropertiesEXT = 1000396006,
    MicromapCreateInfoEXT = 1000396007,
    MicromapBuildSizesInfoEXT = 1000396008,
    AccelerationStructureTrianglesOpacityMicromapEXT = 1000396009,
    PhysicalDeviceDisplacementMicromapFeaturesNV = 1000397000,
    PhysicalDeviceDisplacementMicromapPropertiesNV = 1000397001,
    AccelerationStructureTrianglesDisplacementMicromapNV = 1000397002,
    PhysicalDeviceClusterCullingShaderFeaturesHUAWEI = 1000404000,
    PhysicalDeviceClusterCullingShaderPropertiesHUAWEI = 1000404001,
    PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI = 1000404002,
    PhysicalDeviceBorderColorSwizzleFeaturesEXT = 1000411000,
    SamplerBorderColorComponentMappingCreateInfoEXT = 1000411001,
    PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT = 1000412000,
    PhysicalDeviceShaderCorePropertiesARM = 1000415000,
    PhysicalDeviceShaderSubgroupRotateFeaturesKHR = 1000416000,
    DeviceQueueShaderCoreControlCreateInfoARM = 1000417000,
    PhysicalDeviceSchedulingControlsFeaturesARM = 1000417001,
    PhysicalDeviceSchedulingControlsPropertiesARM = 1000417002,
    PhysicalDeviceImageSlicedViewOf3DFeaturesEXT = 1000418000,
    ImageViewSlicedCreateInfoEXT = 1000418001,
    PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE = 1000420000,
    DescriptorSetBindingReferenceVALVE = 1000420001,
    DescriptorSetLayoutHostMappingInfoVALVE = 1000420002,
    PhysicalDeviceDepthClampZeroOneFeaturesEXT = 1000421000,
    PhysicalDeviceNonSeamlessCubeMapFeaturesEXT = 1000422000,
    PhysicalDeviceRenderPassStripedFeaturesARM = 1000424000,
    PhysicalDeviceRenderPassStripedPropertiesARM = 1000424001,
    RenderPassStripeBeginInfoARM = 1000424002,
    RenderPassStripeInfoARM = 1000424003,
    RenderPassStripeSubmitInfoARM = 1000424004,
    PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM = 1000425000,
    PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM = 1000425001,
    SubpassFragmentDensityMapOffsetEndInfoQCOM = 1000425002,
    PhysicalDeviceCopyMemoryIndirectFeaturesNV = 1000426000,
    PhysicalDeviceCopyMemoryIndirectPropertiesNV = 1000426001,
    PhysicalDeviceMemoryDecompressionFeaturesNV = 1000427000,
    PhysicalDeviceMemoryDecompressionPropertiesNV = 1000427001,
    PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV = 1000428000,
    ComputePipelineIndirectBufferInfoNV = 1000428001,
    PipelineIndirectDeviceAddressInfoNV = 1000428002,
    PhysicalDeviceLinearColorAttachmentFeaturesNV = 1000430000,
    PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR = 1000434000,
    PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT = 1000437000,
    PhysicalDeviceImageProcessingFeaturesQCOM = 1000440000,
    PhysicalDeviceImageProcessingPropertiesQCOM = 1000440001,
    ImageViewSampleWeightCreateInfoQCOM = 1000440002,
    PhysicalDeviceNestedCommandBufferFeaturesEXT = 1000451000,
    PhysicalDeviceNestedCommandBufferPropertiesEXT = 1000451001,
    ExternalMemoryAcquireUnmodifiedEXT = 1000453000,
    PhysicalDeviceExtendedDynamicState3FeaturesEXT = 1000455000,
    PhysicalDeviceExtendedDynamicState3PropertiesEXT = 1000455001,
    PhysicalDeviceSubpassMergeFeedbackFeaturesEXT = 1000458000,
    RenderPassCreationControlEXT = 1000458001,
    RenderPassCreationFeedbackCreateInfoEXT = 1000458002,
    RenderPassSubpassFeedbackCreateInfoEXT = 1000458003,
    DirectDriverLoadingInfoLUNARG = 1000459000,
    DirectDriverLoadingListLUNARG = 1000459001,
    PhysicalDeviceShaderModuleIdentifierFeaturesEXT = 1000462000,
    PhysicalDeviceShaderModuleIdentifierPropertiesEXT = 1000462001,
    PipelineShaderStageModuleIdentifierCreateInfoEXT = 1000462002,
    ShaderModuleIdentifierEXT = 1000462003,
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT = 1000342000,
    PhysicalDeviceOpticalFlowFeaturesNV = 1000464000,
    PhysicalDeviceOpticalFlowPropertiesNV = 1000464001,
    OpticalFlowImageFormatInfoNV = 1000464002,
    OpticalFlowImageFormatPropertiesNV = 1000464003,
    OpticalFlowSessionCreateInfoNV = 1000464004,
    OpticalFlowExecuteInfoNV = 1000464005,
    OpticalFlowSessionCreatePrivateDataInfoNV = 1000464010,
    PhysicalDeviceLegacyDitheringFeaturesEXT = 1000465000,
    PhysicalDevicePipelineProtectedAccessFeaturesEXT = 1000466000,
    PhysicalDeviceExternalFormatResolveFeaturesANDROID = 1000468000,
    PhysicalDeviceExternalFormatResolvePropertiesANDROID = 1000468001,
    AndroidHardwareBufferFormatResolvePropertiesANDROID = 1000468002,
    PhysicalDeviceMaintenance5FeaturesKHR = 1000470000,
    PhysicalDeviceMaintenance5PropertiesKHR = 1000470001,
    RenderingAreaInfoKHR = 1000470003,
    DeviceImageSubresourceInfoKHR = 1000470004,
    SubresourceLayout2KHR = 1000338002,
    ImageSubresource2KHR = 1000338003,
    PipelineCreateFlags2CreateInfoKHR = 1000470005,
    BufferUsageFlags2CreateInfoKHR = 1000470006,
    PhysicalDeviceAntiLagFeaturesAMD = 1000476000,
    AntiLagDataAMD = 1000476001,
    AntiLagPresentationInfoAMD = 1000476002,
    PhysicalDeviceRayTracingPositionFetchFeaturesKHR = 1000481000,
    PhysicalDeviceShaderObjectFeaturesEXT = 1000482000,
    PhysicalDeviceShaderObjectPropertiesEXT = 1000482001,
    ShaderCreateInfoEXT = 1000482002,
    PhysicalDeviceTilePropertiesFeaturesQCOM = 1000484000,
    TilePropertiesQCOM = 1000484001,
    PhysicalDeviceAmigoProfilingFeaturesSEC = 1000485000,
    AmigoProfilingSubmitInfoSEC = 1000485001,
    PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM = 1000488000,
    PhysicalDeviceRayTracingInvocationReorderFeaturesNV = 1000490000,
    PhysicalDeviceRayTracingInvocationReorderPropertiesNV = 1000490001,
    PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV = 1000492000,
    PhysicalDeviceExtendedSparseAddressSpacePropertiesNV = 1000492001,
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT = 1000351000,
    MutableDescriptorTypeCreateInfoEXT = 1000351002,
    PhysicalDeviceLegacyVertexAttributesFeaturesEXT = 1000495000,
    PhysicalDeviceLegacyVertexAttributesPropertiesEXT = 1000495001,
    LayerSettingsCreateInfoEXT = 1000496000,
    PhysicalDeviceShaderCoreBuiltinsFeaturesARM = 1000497000,
    PhysicalDeviceShaderCoreBuiltinsPropertiesARM = 1000497001,
    PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT = 1000498000,
    PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT = 1000499000,
    LatencySleepModeInfoNV = 1000505000,
    LatencySleepInfoNV = 1000505001,
    SetLatencyMarkerInfoNV = 1000505002,
    GetLatencyMarkerInfoNV = 1000505003,
    LatencyTimingsFrameReportNV = 1000505004,
    LatencySubmissionPresentIdNV = 1000505005,
    OutOfBandQueueTypeInfoNV = 1000505006,
    SwapchainLatencyCreateInfoNV = 1000505007,
    LatencySurfaceCapabilitiesNV = 1000505008,
    PhysicalDeviceCooperativeMatrixFeaturesKHR = 1000506000,
    CooperativeMatrixPropertiesKHR = 1000506001,
    PhysicalDeviceCooperativeMatrixPropertiesKHR = 1000506002,
    PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM = 1000510000,
    MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM = 1000510001,
    PhysicalDevicePerStageDescriptorSetFeaturesNV = 1000516000,
    PhysicalDeviceImageProcessing2FeaturesQCOM = 1000518000,
    PhysicalDeviceImageProcessing2PropertiesQCOM = 1000518001,
    SamplerBlockMatchWindowCreateInfoQCOM = 1000518002,
    SamplerCubicWeightsCreateInfoQCOM = 1000519000,
    PhysicalDeviceCubicWeightsFeaturesQCOM = 1000519001,
    BlitImageCubicWeightsInfoQCOM = 1000519002,
    PhysicalDeviceYcbcrDegammaFeaturesQCOM = 1000520000,
    SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM = 1000520001,
    PhysicalDeviceCubicClampFeaturesQCOM = 1000521000,
    PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT = 1000524000,
    PhysicalDeviceVertexAttributeDivisorPropertiesKHR = 1000525000,
    PipelineVertexInputDivisorStateCreateInfoKHR = 1000190001,
    PhysicalDeviceVertexAttributeDivisorFeaturesKHR = 1000190002,
    PhysicalDeviceShaderFloatControls2FeaturesKHR = 1000528000,
    ScreenBufferPropertiesQNX = 1000529000,
    ScreenBufferFormatPropertiesQNX = 1000529001,
    ImportScreenBufferInfoQNX = 1000529002,
    ExternalFormatQNX = 1000529003,
    PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX = 1000529004,
    PhysicalDeviceLayeredDriverPropertiesMSFT = 1000530000,
    PhysicalDeviceIndexTypeUint8FeaturesKHR = 1000265000,
    PhysicalDeviceLineRasterizationFeaturesKHR = 1000259000,
    PipelineRasterizationLineStateCreateInfoKHR = 1000259001,
    PhysicalDeviceLineRasterizationPropertiesKHR = 1000259002,
    CalibratedTimestampInfoKHR = 1000184000,
    PhysicalDeviceShaderExpectAssumeFeaturesKHR = 1000544000,
    PhysicalDeviceMaintenance6FeaturesKHR = 1000545000,
    PhysicalDeviceMaintenance6PropertiesKHR = 1000545001,
    BindMemoryStatusKHR = 1000545002,
    BindDescriptorSetsInfoKHR = 1000545003,
    PushConstantsInfoKHR = 1000545004,
    PushDescriptorSetInfoKHR = 1000545005,
    PushDescriptorSetWithTemplateInfoKHR = 1000545006,
    SetDescriptorBufferOffsetsInfoEXT = 1000545007,
    BindDescriptorBufferEmbeddedSamplersInfoEXT = 1000545008,
    PhysicalDeviceDescriptorPoolOverallocationFeaturesNV = 1000546000,
    PhysicalDeviceRawAccessChainsFeaturesNV = 1000555000,
    PhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR = 1000558000,
    PhysicalDeviceMaintenance7FeaturesKHR = 1000562000,
    PhysicalDeviceMaintenance7PropertiesKHR = 1000562001,
    PhysicalDeviceLayeredApiPropertiesListKHR = 1000562002,
    PhysicalDeviceLayeredApiPropertiesKHR = 1000562003,
    PhysicalDeviceLayeredApiVulkanPropertiesKHR = 1000562004,
    PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV = 1000563000,
    PhysicalDeviceShaderReplicatedCompositesFeaturesEXT = 1000564000,
    PhysicalDeviceRayTracingValidationFeaturesNV = 1000568000,
    PhysicalDeviceImageAlignmentControlFeaturesMESA = 1000575000,
    PhysicalDeviceImageAlignmentControlPropertiesMESA = 1000575001,
    ImageAlignmentControlCreateInfoMESA = 1000575002,
}
#[allow(non_upper_case_globals)]
impl StructureType {
    pub const PhysicalDeviceVariablePointerFeatures: Self =
        Self::PhysicalDeviceVariablePointersFeatures;
    pub const PhysicalDeviceShaderDrawParameterFeatures: Self =
        Self::PhysicalDeviceShaderDrawParametersFeatures;
    pub const DebugReportCreateInfoEXT: Self = Self::DebugReportCallbackCreateInfoEXT;
    pub const RenderingInfoKHR: Self = Self::RenderingInfo;
    pub const RenderingAttachmentInfoKHR: Self = Self::RenderingAttachmentInfo;
    pub const PipelineRenderingCreateInfoKHR: Self = Self::PipelineRenderingCreateInfo;
    pub const PhysicalDeviceDynamicRenderingFeaturesKHR: Self =
        Self::PhysicalDeviceDynamicRenderingFeatures;
    pub const CommandBufferInheritanceRenderingInfoKHR: Self =
        Self::CommandBufferInheritanceRenderingInfo;
    pub const AttachmentSampleCountInfoNV: Self = Self::AttachmentSampleCountInfoAMD;
    pub const RenderPassMultiviewCreateInfoKHR: Self = Self::RenderPassMultiviewCreateInfo;
    pub const PhysicalDeviceMultiviewFeaturesKHR: Self = Self::PhysicalDeviceMultiviewFeatures;
    pub const PhysicalDeviceMultiviewPropertiesKHR: Self = Self::PhysicalDeviceMultiviewProperties;
    pub const PhysicalDeviceFeatures2KHR: Self = Self::PhysicalDeviceFeatures2;
    pub const PhysicalDeviceProperties2KHR: Self = Self::PhysicalDeviceProperties2;
    pub const FormatProperties2KHR: Self = Self::FormatProperties2;
    pub const ImageFormatProperties2KHR: Self = Self::ImageFormatProperties2;
    pub const PhysicalDeviceImageFormatInfo2KHR: Self = Self::PhysicalDeviceImageFormatInfo2;
    pub const QueueFamilyProperties2KHR: Self = Self::QueueFamilyProperties2;
    pub const PhysicalDeviceMemoryProperties2KHR: Self = Self::PhysicalDeviceMemoryProperties2;
    pub const SparseImageFormatProperties2KHR: Self = Self::SparseImageFormatProperties2;
    pub const PhysicalDeviceSparseImageFormatInfo2KHR: Self =
        Self::PhysicalDeviceSparseImageFormatInfo2;
    pub const MemoryAllocateFlagsInfoKHR: Self = Self::MemoryAllocateFlagsInfo;
    pub const DeviceGroupRenderPassBeginInfoKHR: Self = Self::DeviceGroupRenderPassBeginInfo;
    pub const DeviceGroupCommandBufferBeginInfoKHR: Self = Self::DeviceGroupCommandBufferBeginInfo;
    pub const DeviceGroupSubmitInfoKHR: Self = Self::DeviceGroupSubmitInfo;
    pub const DeviceGroupBindSparseInfoKHR: Self = Self::DeviceGroupBindSparseInfo;
    pub const BindBufferMemoryDeviceGroupInfoKHR: Self = Self::BindBufferMemoryDeviceGroupInfo;
    pub const BindImageMemoryDeviceGroupInfoKHR: Self = Self::BindImageMemoryDeviceGroupInfo;
    pub const PhysicalDeviceTextureCompressionAstcHdrFeaturesEXT: Self =
        Self::PhysicalDeviceTextureCompressionAstcHdrFeatures;
    pub const PhysicalDeviceGroupPropertiesKHR: Self = Self::PhysicalDeviceGroupProperties;
    pub const DeviceGroupDeviceCreateInfoKHR: Self = Self::DeviceGroupDeviceCreateInfo;
    pub const PhysicalDeviceExternalImageFormatInfoKHR: Self =
        Self::PhysicalDeviceExternalImageFormatInfo;
    pub const ExternalImageFormatPropertiesKHR: Self = Self::ExternalImageFormatProperties;
    pub const PhysicalDeviceExternalBufferInfoKHR: Self = Self::PhysicalDeviceExternalBufferInfo;
    pub const ExternalBufferPropertiesKHR: Self = Self::ExternalBufferProperties;
    pub const PhysicalDeviceIdPropertiesKHR: Self = Self::PhysicalDeviceIdProperties;
    pub const ExternalMemoryBufferCreateInfoKHR: Self = Self::ExternalMemoryBufferCreateInfo;
    pub const ExternalMemoryImageCreateInfoKHR: Self = Self::ExternalMemoryImageCreateInfo;
    pub const ExportMemoryAllocateInfoKHR: Self = Self::ExportMemoryAllocateInfo;
    pub const PhysicalDeviceExternalSemaphoreInfoKHR: Self =
        Self::PhysicalDeviceExternalSemaphoreInfo;
    pub const ExternalSemaphorePropertiesKHR: Self = Self::ExternalSemaphoreProperties;
    pub const ExportSemaphoreCreateInfoKHR: Self = Self::ExportSemaphoreCreateInfo;
    pub const PhysicalDeviceShaderFloat16Int8FeaturesKHR: Self =
        Self::PhysicalDeviceShaderFloat16Int8Features;
    pub const PhysicalDeviceFloat16Int8FeaturesKHR: Self =
        Self::PhysicalDeviceShaderFloat16Int8Features;
    pub const PhysicalDevice16BitStorageFeaturesKHR: Self =
        Self::PhysicalDevice16BitStorageFeatures;
    pub const DescriptorUpdateTemplateCreateInfoKHR: Self =
        Self::DescriptorUpdateTemplateCreateInfo;
    pub const PhysicalDeviceImagelessFramebufferFeaturesKHR: Self =
        Self::PhysicalDeviceImagelessFramebufferFeatures;
    pub const FramebufferAttachmentsCreateInfoKHR: Self = Self::FramebufferAttachmentsCreateInfo;
    pub const FramebufferAttachmentImageInfoKHR: Self = Self::FramebufferAttachmentImageInfo;
    pub const RenderPassAttachmentBeginInfoKHR: Self = Self::RenderPassAttachmentBeginInfo;
    pub const AttachmentDescription2KHR: Self = Self::AttachmentDescription2;
    pub const AttachmentReference2KHR: Self = Self::AttachmentReference2;
    pub const SubpassDescription2KHR: Self = Self::SubpassDescription2;
    pub const SubpassDependency2KHR: Self = Self::SubpassDependency2;
    pub const RenderPassCreateInfo2KHR: Self = Self::RenderPassCreateInfo2;
    pub const SubpassBeginInfoKHR: Self = Self::SubpassBeginInfo;
    pub const SubpassEndInfoKHR: Self = Self::SubpassEndInfo;
    pub const PhysicalDeviceExternalFenceInfoKHR: Self = Self::PhysicalDeviceExternalFenceInfo;
    pub const ExternalFencePropertiesKHR: Self = Self::ExternalFenceProperties;
    pub const ExportFenceCreateInfoKHR: Self = Self::ExportFenceCreateInfo;
    pub const PhysicalDevicePointClippingPropertiesKHR: Self =
        Self::PhysicalDevicePointClippingProperties;
    pub const RenderPassInputAttachmentAspectCreateInfoKHR: Self =
        Self::RenderPassInputAttachmentAspectCreateInfo;
    pub const ImageViewUsageCreateInfoKHR: Self = Self::ImageViewUsageCreateInfo;
    pub const PipelineTessellationDomainOriginStateCreateInfoKHR: Self =
        Self::PipelineTessellationDomainOriginStateCreateInfo;
    pub const PhysicalDeviceVariablePointersFeaturesKHR: Self =
        Self::PhysicalDeviceVariablePointersFeatures;
    pub const PhysicalDeviceVariablePointerFeaturesKHR: Self =
        Self::PhysicalDeviceVariablePointersFeaturesKHR;
    pub const MemoryDedicatedRequirementsKHR: Self = Self::MemoryDedicatedRequirements;
    pub const MemoryDedicatedAllocateInfoKHR: Self = Self::MemoryDedicatedAllocateInfo;
    pub const PhysicalDeviceSamplerFilterMinmaxPropertiesEXT: Self =
        Self::PhysicalDeviceSamplerFilterMinmaxProperties;
    pub const SamplerReductionModeCreateInfoEXT: Self = Self::SamplerReductionModeCreateInfo;
    pub const PhysicalDeviceInlineUniformBlockFeaturesEXT: Self =
        Self::PhysicalDeviceInlineUniformBlockFeatures;
    pub const PhysicalDeviceInlineUniformBlockPropertiesEXT: Self =
        Self::PhysicalDeviceInlineUniformBlockProperties;
    pub const WriteDescriptorSetInlineUniformBlockEXT: Self =
        Self::WriteDescriptorSetInlineUniformBlock;
    pub const DescriptorPoolInlineUniformBlockCreateInfoEXT: Self =
        Self::DescriptorPoolInlineUniformBlockCreateInfo;
    pub const BufferMemoryRequirementsInfo2KHR: Self = Self::BufferMemoryRequirementsInfo2;
    pub const ImageMemoryRequirementsInfo2KHR: Self = Self::ImageMemoryRequirementsInfo2;
    pub const ImageSparseMemoryRequirementsInfo2KHR: Self =
        Self::ImageSparseMemoryRequirementsInfo2;
    pub const MemoryRequirements2KHR: Self = Self::MemoryRequirements2;
    pub const SparseImageMemoryRequirements2KHR: Self = Self::SparseImageMemoryRequirements2;
    pub const ImageFormatListCreateInfoKHR: Self = Self::ImageFormatListCreateInfo;
    pub const SamplerYcbcrConversionCreateInfoKHR: Self = Self::SamplerYcbcrConversionCreateInfo;
    pub const SamplerYcbcrConversionInfoKHR: Self = Self::SamplerYcbcrConversionInfo;
    pub const BindImagePlaneMemoryInfoKHR: Self = Self::BindImagePlaneMemoryInfo;
    pub const ImagePlaneMemoryRequirementsInfoKHR: Self = Self::ImagePlaneMemoryRequirementsInfo;
    pub const PhysicalDeviceSamplerYcbcrConversionFeaturesKHR: Self =
        Self::PhysicalDeviceSamplerYcbcrConversionFeatures;
    pub const SamplerYcbcrConversionImageFormatPropertiesKHR: Self =
        Self::SamplerYcbcrConversionImageFormatProperties;
    pub const BindBufferMemoryInfoKHR: Self = Self::BindBufferMemoryInfo;
    pub const BindImageMemoryInfoKHR: Self = Self::BindImageMemoryInfo;
    pub const DescriptorSetLayoutBindingFlagsCreateInfoEXT: Self =
        Self::DescriptorSetLayoutBindingFlagsCreateInfo;
    pub const PhysicalDeviceDescriptorIndexingFeaturesEXT: Self =
        Self::PhysicalDeviceDescriptorIndexingFeatures;
    pub const PhysicalDeviceDescriptorIndexingPropertiesEXT: Self =
        Self::PhysicalDeviceDescriptorIndexingProperties;
    pub const DescriptorSetVariableDescriptorCountAllocateInfoEXT: Self =
        Self::DescriptorSetVariableDescriptorCountAllocateInfo;
    pub const DescriptorSetVariableDescriptorCountLayoutSupportEXT: Self =
        Self::DescriptorSetVariableDescriptorCountLayoutSupport;
    pub const PhysicalDeviceMaintenance3PropertiesKHR: Self =
        Self::PhysicalDeviceMaintenance3Properties;
    pub const DescriptorSetLayoutSupportKHR: Self = Self::DescriptorSetLayoutSupport;
    pub const DeviceQueueGlobalPriorityCreateInfoEXT: Self =
        Self::DeviceQueueGlobalPriorityCreateInfoKHR;
    pub const PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR: Self =
        Self::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    pub const PhysicalDevice8BitStorageFeaturesKHR: Self = Self::PhysicalDevice8BitStorageFeatures;
    pub const PhysicalDeviceShaderAtomicInt64FeaturesKHR: Self =
        Self::PhysicalDeviceShaderAtomicInt64Features;
    pub const CalibratedTimestampInfoEXT: Self = Self::CalibratedTimestampInfoKHR;
    pub const PipelineVertexInputDivisorStateCreateInfoEXT: Self =
        Self::PipelineVertexInputDivisorStateCreateInfoKHR;
    pub const PhysicalDeviceVertexAttributeDivisorFeaturesEXT: Self =
        Self::PhysicalDeviceVertexAttributeDivisorFeaturesKHR;
    pub const PipelineCreationFeedbackCreateInfoEXT: Self =
        Self::PipelineCreationFeedbackCreateInfo;
    pub const PhysicalDeviceDriverPropertiesKHR: Self = Self::PhysicalDeviceDriverProperties;
    pub const PhysicalDeviceFloatControlsPropertiesKHR: Self =
        Self::PhysicalDeviceFloatControlsProperties;
    pub const PhysicalDeviceDepthStencilResolvePropertiesKHR: Self =
        Self::PhysicalDeviceDepthStencilResolveProperties;
    pub const SubpassDescriptionDepthStencilResolveKHR: Self =
        Self::SubpassDescriptionDepthStencilResolve;
    pub const PhysicalDeviceFragmentShaderBarycentricFeaturesNV: Self =
        Self::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR;
    pub const PhysicalDeviceTimelineSemaphoreFeaturesKHR: Self =
        Self::PhysicalDeviceTimelineSemaphoreFeatures;
    pub const PhysicalDeviceTimelineSemaphorePropertiesKHR: Self =
        Self::PhysicalDeviceTimelineSemaphoreProperties;
    pub const SemaphoreTypeCreateInfoKHR: Self = Self::SemaphoreTypeCreateInfo;
    pub const TimelineSemaphoreSubmitInfoKHR: Self = Self::TimelineSemaphoreSubmitInfo;
    pub const SemaphoreWaitInfoKHR: Self = Self::SemaphoreWaitInfo;
    pub const SemaphoreSignalInfoKHR: Self = Self::SemaphoreSignalInfo;
    pub const QueryPoolCreateInfoINTEL: Self = Self::QueryPoolPerformanceQueryCreateInfoINTEL;
    pub const PhysicalDeviceVulkanMemoryModelFeaturesKHR: Self =
        Self::PhysicalDeviceVulkanMemoryModelFeatures;
    pub const PhysicalDeviceShaderTerminateInvocationFeaturesKHR: Self =
        Self::PhysicalDeviceShaderTerminateInvocationFeatures;
    pub const PhysicalDeviceScalarBlockLayoutFeaturesEXT: Self =
        Self::PhysicalDeviceScalarBlockLayoutFeatures;
    pub const PhysicalDeviceSubgroupSizeControlPropertiesEXT: Self =
        Self::PhysicalDeviceSubgroupSizeControlProperties;
    pub const PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT: Self =
        Self::PipelineShaderStageRequiredSubgroupSizeCreateInfo;
    pub const PhysicalDeviceSubgroupSizeControlFeaturesEXT: Self =
        Self::PhysicalDeviceSubgroupSizeControlFeatures;
    pub const PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR: Self =
        Self::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    pub const AttachmentReferenceStencilLayoutKHR: Self = Self::AttachmentReferenceStencilLayout;
    pub const AttachmentDescriptionStencilLayoutKHR: Self =
        Self::AttachmentDescriptionStencilLayout;
    pub const PhysicalDeviceBufferAddressFeaturesEXT: Self =
        Self::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    pub const BufferDeviceAddressInfoEXT: Self = Self::BufferDeviceAddressInfo;
    pub const PhysicalDeviceToolPropertiesEXT: Self = Self::PhysicalDeviceToolProperties;
    pub const ImageStencilUsageCreateInfoEXT: Self = Self::ImageStencilUsageCreateInfo;
    pub const PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR: Self =
        Self::PhysicalDeviceUniformBufferStandardLayoutFeatures;
    pub const PhysicalDeviceBufferDeviceAddressFeaturesKHR: Self =
        Self::PhysicalDeviceBufferDeviceAddressFeatures;
    pub const BufferDeviceAddressInfoKHR: Self = Self::BufferDeviceAddressInfo;
    pub const BufferOpaqueCaptureAddressCreateInfoKHR: Self =
        Self::BufferOpaqueCaptureAddressCreateInfo;
    pub const MemoryOpaqueCaptureAddressAllocateInfoKHR: Self =
        Self::MemoryOpaqueCaptureAddressAllocateInfo;
    pub const DeviceMemoryOpaqueCaptureAddressInfoKHR: Self =
        Self::DeviceMemoryOpaqueCaptureAddressInfo;
    pub const PhysicalDeviceLineRasterizationFeaturesEXT: Self =
        Self::PhysicalDeviceLineRasterizationFeaturesKHR;
    pub const PipelineRasterizationLineStateCreateInfoEXT: Self =
        Self::PipelineRasterizationLineStateCreateInfoKHR;
    pub const PhysicalDeviceLineRasterizationPropertiesEXT: Self =
        Self::PhysicalDeviceLineRasterizationPropertiesKHR;
    pub const PhysicalDeviceHostQueryResetFeaturesEXT: Self =
        Self::PhysicalDeviceHostQueryResetFeatures;
    pub const PhysicalDeviceIndexTypeUint8FeaturesEXT: Self =
        Self::PhysicalDeviceIndexTypeUint8FeaturesKHR;
    pub const PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT: Self =
        Self::PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
    pub const PhysicalDeviceShaderIntegerDotProductFeaturesKHR: Self =
        Self::PhysicalDeviceShaderIntegerDotProductFeatures;
    pub const PhysicalDeviceShaderIntegerDotProductPropertiesKHR: Self =
        Self::PhysicalDeviceShaderIntegerDotProductProperties;
    pub const PhysicalDeviceTexelBufferAlignmentPropertiesEXT: Self =
        Self::PhysicalDeviceTexelBufferAlignmentProperties;
    pub const PhysicalDevicePrivateDataFeaturesEXT: Self = Self::PhysicalDevicePrivateDataFeatures;
    pub const DevicePrivateDataCreateInfoEXT: Self = Self::DevicePrivateDataCreateInfo;
    pub const PrivateDataSlotCreateInfoEXT: Self = Self::PrivateDataSlotCreateInfo;
    pub const PhysicalDevicePipelineCreationCacheControlFeaturesEXT: Self =
        Self::PhysicalDevicePipelineCreationCacheControlFeatures;
    pub const MemoryBarrier2KHR: Self = Self::MemoryBarrier2;
    pub const BufferMemoryBarrier2KHR: Self = Self::BufferMemoryBarrier2;
    pub const ImageMemoryBarrier2KHR: Self = Self::ImageMemoryBarrier2;
    pub const DependencyInfoKHR: Self = Self::DependencyInfo;
    pub const SubmitInfo2KHR: Self = Self::SubmitInfo2;
    pub const SemaphoreSubmitInfoKHR: Self = Self::SemaphoreSubmitInfo;
    pub const CommandBufferSubmitInfoKHR: Self = Self::CommandBufferSubmitInfo;
    pub const PhysicalDeviceSynchronization2FeaturesKHR: Self =
        Self::PhysicalDeviceSynchronization2Features;
    pub const PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR: Self =
        Self::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
    pub const PhysicalDeviceImageRobustnessFeaturesEXT: Self =
        Self::PhysicalDeviceImageRobustnessFeatures;
    pub const CopyBufferInfo2KHR: Self = Self::CopyBufferInfo2;
    pub const CopyImageInfo2KHR: Self = Self::CopyImageInfo2;
    pub const CopyBufferToImageInfo2KHR: Self = Self::CopyBufferToImageInfo2;
    pub const CopyImageToBufferInfo2KHR: Self = Self::CopyImageToBufferInfo2;
    pub const BlitImageInfo2KHR: Self = Self::BlitImageInfo2;
    pub const ResolveImageInfo2KHR: Self = Self::ResolveImageInfo2;
    pub const BufferCopy2KHR: Self = Self::BufferCopy2;
    pub const ImageCopy2KHR: Self = Self::ImageCopy2;
    pub const ImageBlit2KHR: Self = Self::ImageBlit2;
    pub const BufferImageCopy2KHR: Self = Self::BufferImageCopy2;
    pub const ImageResolve2KHR: Self = Self::ImageResolve2;
    pub const SubresourceLayout2EXT: Self = Self::SubresourceLayout2KHR;
    pub const ImageSubresource2EXT: Self = Self::ImageSubresource2KHR;
    pub const PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM: Self =
        Self::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT;
    pub const PhysicalDeviceMutableDescriptorTypeFeaturesVALVE: Self =
        Self::PhysicalDeviceMutableDescriptorTypeFeaturesEXT;
    pub const MutableDescriptorTypeCreateInfoVALVE: Self = Self::MutableDescriptorTypeCreateInfoEXT;
    pub const FormatProperties3KHR: Self = Self::FormatProperties3;
    pub const PipelineInfoEXT: Self = Self::PipelineInfoKHR;
    pub const PhysicalDeviceGlobalPriorityQueryFeaturesEXT: Self =
        Self::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
    pub const QueueFamilyGlobalPriorityPropertiesEXT: Self =
        Self::QueueFamilyGlobalPriorityPropertiesKHR;
    pub const PhysicalDeviceMaintenance4FeaturesKHR: Self =
        Self::PhysicalDeviceMaintenance4Features;
    pub const PhysicalDeviceMaintenance4PropertiesKHR: Self =
        Self::PhysicalDeviceMaintenance4Properties;
    pub const DeviceBufferMemoryRequirementsKHR: Self = Self::DeviceBufferMemoryRequirements;
    pub const DeviceImageMemoryRequirementsKHR: Self = Self::DeviceImageMemoryRequirements;
    pub const ShaderRequiredSubgroupSizeCreateInfoEXT: Self =
        Self::PipelineShaderStageRequiredSubgroupSizeCreateInfo;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ATTACHMENT_UNUSED.html>"]
#[doc(alias = "VK_ATTACHMENT_UNUSED")]
pub const ATTACHMENT_UNUSED: u32 = !0u32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LOD_CLAMP_NONE.html>"]
#[doc(alias = "VK_LOD_CLAMP_NONE")]
pub const LOD_CLAMP_NONE: f32 = 1000.0f32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_IGNORED.html>"]
#[doc(alias = "VK_QUEUE_FAMILY_IGNORED")]
pub const QUEUE_FAMILY_IGNORED: u32 = !0u32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_ARRAY_LAYERS.html>"]
#[doc(alias = "VK_REMAINING_ARRAY_LAYERS")]
pub const REMAINING_ARRAY_LAYERS: u32 = !0u32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_MIP_LEVELS.html>"]
#[doc(alias = "VK_REMAINING_MIP_LEVELS")]
pub const REMAINING_MIP_LEVELS: u32 = !0u32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SUBPASS_EXTERNAL.html>"]
#[doc(alias = "VK_SUBPASS_EXTERNAL")]
pub const SUBPASS_EXTERNAL: u32 = !0u32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_WHOLE_SIZE.html>"]
#[doc(alias = "VK_WHOLE_SIZE")]
pub const WHOLE_SIZE: u64 = !0u64;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_MEMORY_TYPES.html>"]
#[doc(alias = "VK_MAX_MEMORY_TYPES")]
pub const MAX_MEMORY_TYPES: u32 = 32;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html>"]
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")]
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_UUID_SIZE.html>"]
#[doc(alias = "VK_UUID_SIZE")]
pub const UUID_SIZE: u32 = 16;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_EXTENSION_NAME_SIZE.html>"]
#[doc(alias = "VK_MAX_EXTENSION_NAME_SIZE")]
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DESCRIPTION_SIZE.html>"]
#[doc(alias = "VK_MAX_DESCRIPTION_SIZE")]
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_MEMORY_HEAPS.html>"]
#[doc(alias = "VK_MAX_MEMORY_HEAPS")]
pub const MAX_MEMORY_HEAPS: u32 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheHeaderVersion.html>"]
#[doc(alias = "VkPipelineCacheHeaderVersion")]
#[repr(u32)]
pub enum PipelineCacheHeaderVersion {
    One = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkObjectType.html>"]
#[doc(alias = "VkObjectType")]
#[repr(u32)]
pub enum ObjectType {
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,
    SamplerYcbcrConversion = 1000156000,
    DescriptorUpdateTemplate = 1000085000,
    PrivateDataSlot = 1000295000,
    SurfaceKHR = 1000000000,
    SwapchainKHR = 1000001000,
    DisplayKHR = 1000002000,
    DisplayModeKHR = 1000002001,
    DebugReportCallbackEXT = 1000011000,
    CuModuleNVX = 1000029000,
    CuFunctionNVX = 1000029001,
    DebugUtilsMessengerEXT = 1000128000,
    AccelerationStructureKHR = 1000150000,
    ValidationCacheEXT = 1000160000,
    AccelerationStructureNV = 1000165000,
    PerformanceConfigurationINTEL = 1000210000,
    DeferredOperationKHR = 1000268000,
    IndirectCommandsLayoutNV = 1000277000,
    CudaModuleNV = 1000307000,
    CudaFunctionNV = 1000307001,
    BufferCollectionFUCHSIA = 1000366000,
    MicromapEXT = 1000396000,
    OpticalFlowSessionNV = 1000464000,
    ShaderEXT = 1000482000,
}
#[allow(non_upper_case_globals)]
impl ObjectType {
    pub const DescriptorUpdateTemplateKHR: Self = Self::DescriptorUpdateTemplate;
    pub const SamplerYcbcrConversionKHR: Self = Self::SamplerYcbcrConversion;
    pub const PrivateDataSlotEXT: Self = Self::PrivateDataSlot;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVendorId.html>"]
#[doc(alias = "VkVendorId")]
#[repr(u32)]
pub enum VendorId {
    Khronos = 0x10000,
    VIV = 0x10001,
    VSI = 0x10002,
    Kazan = 0x10003,
    Codeplay = 0x10004,
    MESA = 0x10005,
    Pocl = 0x10006,
    Mobileye = 0x10007,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormat.html>"]
#[doc(alias = "VkFormat")]
#[repr(u32)]
pub enum Format {
    Undefined = 0,
    R4G4UnormPack8 = 1,
    R4G4B4A4UnormPack16 = 2,
    B4G4R4A4UnormPack16 = 3,
    R5G6B5UnormPack16 = 4,
    B5G6R5UnormPack16 = 5,
    R5G5B5A1UnormPack16 = 6,
    B5G5R5A1UnormPack16 = 7,
    A1R5G5B5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8Uscaled = 11,
    R8Sscaled = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8G8Unorm = 16,
    R8G8Snorm = 17,
    R8G8Uscaled = 18,
    R8G8Sscaled = 19,
    R8G8Uint = 20,
    R8G8Sint = 21,
    R8G8Srgb = 22,
    R8G8B8Unorm = 23,
    R8G8B8Snorm = 24,
    R8G8B8Uscaled = 25,
    R8G8B8Sscaled = 26,
    R8G8B8Uint = 27,
    R8G8B8Sint = 28,
    R8G8B8Srgb = 29,
    B8G8R8Unorm = 30,
    B8G8R8Snorm = 31,
    B8G8R8Uscaled = 32,
    B8G8R8Sscaled = 33,
    B8G8R8Uint = 34,
    B8G8R8Sint = 35,
    B8G8R8Srgb = 36,
    R8G8B8A8Unorm = 37,
    R8G8B8A8Snorm = 38,
    R8G8B8A8Uscaled = 39,
    R8G8B8A8Sscaled = 40,
    R8G8B8A8Uint = 41,
    R8G8B8A8Sint = 42,
    R8G8B8A8Srgb = 43,
    B8G8R8A8Unorm = 44,
    B8G8R8A8Snorm = 45,
    B8G8R8A8Uscaled = 46,
    B8G8R8A8Sscaled = 47,
    B8G8R8A8Uint = 48,
    B8G8R8A8Sint = 49,
    B8G8R8A8Srgb = 50,
    A8B8G8R8UnormPack32 = 51,
    A8B8G8R8SnormPack32 = 52,
    A8B8G8R8UscaledPack32 = 53,
    A8B8G8R8SscaledPack32 = 54,
    A8B8G8R8UintPack32 = 55,
    A8B8G8R8SintPack32 = 56,
    A8B8G8R8SrgbPack32 = 57,
    A2R10G10B10UnormPack32 = 58,
    A2R10G10B10SnormPack32 = 59,
    A2R10G10B10UscaledPack32 = 60,
    A2R10G10B10SscaledPack32 = 61,
    A2R10G10B10UintPack32 = 62,
    A2R10G10B10SintPack32 = 63,
    A2B10G10R10UnormPack32 = 64,
    A2B10G10R10SnormPack32 = 65,
    A2B10G10R10UscaledPack32 = 66,
    A2B10G10R10SscaledPack32 = 67,
    A2B10G10R10UintPack32 = 68,
    A2B10G10R10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16G16Unorm = 77,
    R16G16Snorm = 78,
    R16G16Uscaled = 79,
    R16G16Sscaled = 80,
    R16G16Uint = 81,
    R16G16Sint = 82,
    R16G16Sfloat = 83,
    R16G16B16Unorm = 84,
    R16G16B16Snorm = 85,
    R16G16B16Uscaled = 86,
    R16G16B16Sscaled = 87,
    R16G16B16Uint = 88,
    R16G16B16Sint = 89,
    R16G16B16Sfloat = 90,
    R16G16B16A16Unorm = 91,
    R16G16B16A16Snorm = 92,
    R16G16B16A16Uscaled = 93,
    R16G16B16A16Sscaled = 94,
    R16G16B16A16Uint = 95,
    R16G16B16A16Sint = 96,
    R16G16B16A16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32G32Uint = 101,
    R32G32Sint = 102,
    R32G32Sfloat = 103,
    R32G32B32Uint = 104,
    R32G32B32Sint = 105,
    R32G32B32Sfloat = 106,
    R32G32B32A32Uint = 107,
    R32G32B32A32Sint = 108,
    R32G32B32A32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64G64Uint = 113,
    R64G64Sint = 114,
    R64G64Sfloat = 115,
    R64G64B64Uint = 116,
    R64G64B64Sint = 117,
    R64G64B64Sfloat = 118,
    R64G64B64A64Uint = 119,
    R64G64B64A64Sint = 120,
    R64G64B64A64Sfloat = 121,
    B10G11R11UfloatPack32 = 122,
    E5B9G9R9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    Bc1RgbUnormBlock = 131,
    Bc1RgbSrgbBlock = 132,
    Bc1RgbaUnormBlock = 133,
    Bc1RgbaSrgbBlock = 134,
    Bc2UnormBlock = 135,
    Bc2SrgbBlock = 136,
    Bc3UnormBlock = 137,
    Bc3SrgbBlock = 138,
    Bc4UnormBlock = 139,
    Bc4SnormBlock = 140,
    Bc5UnormBlock = 141,
    Bc5SnormBlock = 142,
    Bc6HUfloatBlock = 143,
    Bc6HSfloatBlock = 144,
    Bc7UnormBlock = 145,
    Bc7SrgbBlock = 146,
    Etc2R8G8B8UnormBlock = 147,
    Etc2R8G8B8SrgbBlock = 148,
    Etc2R8G8B8A1UnormBlock = 149,
    Etc2R8G8B8A1SrgbBlock = 150,
    Etc2R8G8B8A8UnormBlock = 151,
    Etc2R8G8B8A8SrgbBlock = 152,
    EacR11UnormBlock = 153,
    EacR11SnormBlock = 154,
    EacR11G11UnormBlock = 155,
    EacR11G11SnormBlock = 156,
    Astc4x4UnormBlock = 157,
    Astc4x4SrgbBlock = 158,
    Astc5x4UnormBlock = 159,
    Astc5x4SrgbBlock = 160,
    Astc5x5UnormBlock = 161,
    Astc5x5SrgbBlock = 162,
    Astc6x5UnormBlock = 163,
    Astc6x5SrgbBlock = 164,
    Astc6x6UnormBlock = 165,
    Astc6x6SrgbBlock = 166,
    Astc8x5UnormBlock = 167,
    Astc8x5SrgbBlock = 168,
    Astc8x6UnormBlock = 169,
    Astc8x6SrgbBlock = 170,
    Astc8x8UnormBlock = 171,
    Astc8x8SrgbBlock = 172,
    Astc10x5UnormBlock = 173,
    Astc10x5SrgbBlock = 174,
    Astc10x6UnormBlock = 175,
    Astc10x6SrgbBlock = 176,
    Astc10x8UnormBlock = 177,
    Astc10x8SrgbBlock = 178,
    Astc10x10UnormBlock = 179,
    Astc10x10SrgbBlock = 180,
    Astc12x10UnormBlock = 181,
    Astc12x10SrgbBlock = 182,
    Astc12x12UnormBlock = 183,
    Astc12x12SrgbBlock = 184,
    G8B8G8R8422Unorm = 1000156000,
    B8G8R8G8422Unorm = 1000156001,
    G8B8R83Plane420Unorm = 1000156002,
    G8B8R82Plane420Unorm = 1000156003,
    G8B8R83Plane422Unorm = 1000156004,
    G8B8R82Plane422Unorm = 1000156005,
    G8B8R83Plane444Unorm = 1000156006,
    R10X6UnormPack16 = 1000156007,
    R10X6G10X6Unorm2Pack16 = 1000156008,
    R10X6G10X6B10X6A10X6Unorm4Pack16 = 1000156009,
    G10X6B10X6G10X6R10X6422Unorm4Pack16 = 1000156010,
    B10X6G10X6R10X6G10X6422Unorm4Pack16 = 1000156011,
    G10X6B10X6R10X63Plane420Unorm3Pack16 = 1000156012,
    G10X6B10X6R10X62Plane420Unorm3Pack16 = 1000156013,
    G10X6B10X6R10X63Plane422Unorm3Pack16 = 1000156014,
    G10X6B10X6R10X62Plane422Unorm3Pack16 = 1000156015,
    G10X6B10X6R10X63Plane444Unorm3Pack16 = 1000156016,
    R12X4UnormPack16 = 1000156017,
    R12X4G12X4Unorm2Pack16 = 1000156018,
    R12X4G12X4B12X4A12X4Unorm4Pack16 = 1000156019,
    G12X4B12X4G12X4R12X4422Unorm4Pack16 = 1000156020,
    B12X4G12X4R12X4G12X4422Unorm4Pack16 = 1000156021,
    G12X4B12X4R12X43Plane420Unorm3Pack16 = 1000156022,
    G12X4B12X4R12X42Plane420Unorm3Pack16 = 1000156023,
    G12X4B12X4R12X43Plane422Unorm3Pack16 = 1000156024,
    G12X4B12X4R12X42Plane422Unorm3Pack16 = 1000156025,
    G12X4B12X4R12X43Plane444Unorm3Pack16 = 1000156026,
    G16B16G16R16422Unorm = 1000156027,
    B16G16R16G16422Unorm = 1000156028,
    G16B16R163Plane420Unorm = 1000156029,
    G16B16R162Plane420Unorm = 1000156030,
    G16B16R163Plane422Unorm = 1000156031,
    G16B16R162Plane422Unorm = 1000156032,
    G16B16R163Plane444Unorm = 1000156033,
    G8B8R82Plane444Unorm = 1000330000,
    G10X6B10X6R10X62Plane444Unorm3Pack16 = 1000330001,
    G12X4B12X4R12X42Plane444Unorm3Pack16 = 1000330002,
    G16B16R162Plane444Unorm = 1000330003,
    A4R4G4B4UnormPack16 = 1000340000,
    A4B4G4R4UnormPack16 = 1000340001,
    Astc4x4SfloatBlock = 1000066000,
    Astc5x4SfloatBlock = 1000066001,
    Astc5x5SfloatBlock = 1000066002,
    Astc6x5SfloatBlock = 1000066003,
    Astc6x6SfloatBlock = 1000066004,
    Astc8x5SfloatBlock = 1000066005,
    Astc8x6SfloatBlock = 1000066006,
    Astc8x8SfloatBlock = 1000066007,
    Astc10x5SfloatBlock = 1000066008,
    Astc10x6SfloatBlock = 1000066009,
    Astc10x8SfloatBlock = 1000066010,
    Astc10x10SfloatBlock = 1000066011,
    Astc12x10SfloatBlock = 1000066012,
    Astc12x12SfloatBlock = 1000066013,
    Pvrtc12BppUnormBlockIMG = 1000054000,
    Pvrtc14BppUnormBlockIMG = 1000054001,
    Pvrtc22BppUnormBlockIMG = 1000054002,
    Pvrtc24BppUnormBlockIMG = 1000054003,
    Pvrtc12BppSrgbBlockIMG = 1000054004,
    Pvrtc14BppSrgbBlockIMG = 1000054005,
    Pvrtc22BppSrgbBlockIMG = 1000054006,
    Pvrtc24BppSrgbBlockIMG = 1000054007,
    R16G16Sfixed5NV = 1000464000,
    A1B5G5R5UnormPack16KHR = 1000470000,
    A8UnormKHR = 1000470001,
}
#[allow(non_upper_case_globals)]
impl Format {
    pub const Astc4x4SfloatBlockEXT: Self = Self::Astc4x4SfloatBlock;
    pub const Astc5x4SfloatBlockEXT: Self = Self::Astc5x4SfloatBlock;
    pub const Astc5x5SfloatBlockEXT: Self = Self::Astc5x5SfloatBlock;
    pub const Astc6x5SfloatBlockEXT: Self = Self::Astc6x5SfloatBlock;
    pub const Astc6x6SfloatBlockEXT: Self = Self::Astc6x6SfloatBlock;
    pub const Astc8x5SfloatBlockEXT: Self = Self::Astc8x5SfloatBlock;
    pub const Astc8x6SfloatBlockEXT: Self = Self::Astc8x6SfloatBlock;
    pub const Astc8x8SfloatBlockEXT: Self = Self::Astc8x8SfloatBlock;
    pub const Astc10x5SfloatBlockEXT: Self = Self::Astc10x5SfloatBlock;
    pub const Astc10x6SfloatBlockEXT: Self = Self::Astc10x6SfloatBlock;
    pub const Astc10x8SfloatBlockEXT: Self = Self::Astc10x8SfloatBlock;
    pub const Astc10x10SfloatBlockEXT: Self = Self::Astc10x10SfloatBlock;
    pub const Astc12x10SfloatBlockEXT: Self = Self::Astc12x10SfloatBlock;
    pub const Astc12x12SfloatBlockEXT: Self = Self::Astc12x12SfloatBlock;
    pub const G8B8G8R8422UnormKHR: Self = Self::G8B8G8R8422Unorm;
    pub const B8G8R8G8422UnormKHR: Self = Self::B8G8R8G8422Unorm;
    pub const G8B8R83Plane420UnormKHR: Self = Self::G8B8R83Plane420Unorm;
    pub const G8B8R82Plane420UnormKHR: Self = Self::G8B8R82Plane420Unorm;
    pub const G8B8R83Plane422UnormKHR: Self = Self::G8B8R83Plane422Unorm;
    pub const G8B8R82Plane422UnormKHR: Self = Self::G8B8R82Plane422Unorm;
    pub const G8B8R83Plane444UnormKHR: Self = Self::G8B8R83Plane444Unorm;
    pub const R10X6UnormPack16KHR: Self = Self::R10X6UnormPack16;
    pub const R10X6G10X6Unorm2Pack16KHR: Self = Self::R10X6G10X6Unorm2Pack16;
    pub const R10X6G10X6B10X6A10X6Unorm4Pack16KHR: Self = Self::R10X6G10X6B10X6A10X6Unorm4Pack16;
    pub const G10X6B10X6G10X6R10X6422Unorm4Pack16KHR: Self =
        Self::G10X6B10X6G10X6R10X6422Unorm4Pack16;
    pub const B10X6G10X6R10X6G10X6422Unorm4Pack16KHR: Self =
        Self::B10X6G10X6R10X6G10X6422Unorm4Pack16;
    pub const G10X6B10X6R10X63Plane420Unorm3Pack16KHR: Self =
        Self::G10X6B10X6R10X63Plane420Unorm3Pack16;
    pub const G10X6B10X6R10X62Plane420Unorm3Pack16KHR: Self =
        Self::G10X6B10X6R10X62Plane420Unorm3Pack16;
    pub const G10X6B10X6R10X63Plane422Unorm3Pack16KHR: Self =
        Self::G10X6B10X6R10X63Plane422Unorm3Pack16;
    pub const G10X6B10X6R10X62Plane422Unorm3Pack16KHR: Self =
        Self::G10X6B10X6R10X62Plane422Unorm3Pack16;
    pub const G10X6B10X6R10X63Plane444Unorm3Pack16KHR: Self =
        Self::G10X6B10X6R10X63Plane444Unorm3Pack16;
    pub const R12X4UnormPack16KHR: Self = Self::R12X4UnormPack16;
    pub const R12X4G12X4Unorm2Pack16KHR: Self = Self::R12X4G12X4Unorm2Pack16;
    pub const R12X4G12X4B12X4A12X4Unorm4Pack16KHR: Self = Self::R12X4G12X4B12X4A12X4Unorm4Pack16;
    pub const G12X4B12X4G12X4R12X4422Unorm4Pack16KHR: Self =
        Self::G12X4B12X4G12X4R12X4422Unorm4Pack16;
    pub const B12X4G12X4R12X4G12X4422Unorm4Pack16KHR: Self =
        Self::B12X4G12X4R12X4G12X4422Unorm4Pack16;
    pub const G12X4B12X4R12X43Plane420Unorm3Pack16KHR: Self =
        Self::G12X4B12X4R12X43Plane420Unorm3Pack16;
    pub const G12X4B12X4R12X42Plane420Unorm3Pack16KHR: Self =
        Self::G12X4B12X4R12X42Plane420Unorm3Pack16;
    pub const G12X4B12X4R12X43Plane422Unorm3Pack16KHR: Self =
        Self::G12X4B12X4R12X43Plane422Unorm3Pack16;
    pub const G12X4B12X4R12X42Plane422Unorm3Pack16KHR: Self =
        Self::G12X4B12X4R12X42Plane422Unorm3Pack16;
    pub const G12X4B12X4R12X43Plane444Unorm3Pack16KHR: Self =
        Self::G12X4B12X4R12X43Plane444Unorm3Pack16;
    pub const G16B16G16R16422UnormKHR: Self = Self::G16B16G16R16422Unorm;
    pub const B16G16R16G16422UnormKHR: Self = Self::B16G16R16G16422Unorm;
    pub const G16B16R163Plane420UnormKHR: Self = Self::G16B16R163Plane420Unorm;
    pub const G16B16R162Plane420UnormKHR: Self = Self::G16B16R162Plane420Unorm;
    pub const G16B16R163Plane422UnormKHR: Self = Self::G16B16R163Plane422Unorm;
    pub const G16B16R162Plane422UnormKHR: Self = Self::G16B16R162Plane422Unorm;
    pub const G16B16R163Plane444UnormKHR: Self = Self::G16B16R163Plane444Unorm;
    pub const G8B8R82Plane444UnormEXT: Self = Self::G8B8R82Plane444Unorm;
    pub const G10X6B10X6R10X62Plane444Unorm3Pack16EXT: Self =
        Self::G10X6B10X6R10X62Plane444Unorm3Pack16;
    pub const G12X4B12X4R12X42Plane444Unorm3Pack16EXT: Self =
        Self::G12X4B12X4R12X42Plane444Unorm3Pack16;
    pub const G16B16R162Plane444UnormEXT: Self = Self::G16B16R162Plane444Unorm;
    pub const A4R4G4B4UnormPack16EXT: Self = Self::A4R4G4B4UnormPack16;
    pub const A4B4G4R4UnormPack16EXT: Self = Self::A4B4G4R4UnormPack16;
    pub const R16G16S105NV: Self = Self::R16G16Sfixed5NV;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html>"]
    #[doc(alias = "VkFormatFeatureFlagBits")]
    pub struct FormatFeatureFlags : u32 {
        const SampledImage = 1u32 << 0;
        const StorageImage = 1u32 << 1;
        const StorageImageAtomic = 1u32 << 2;
        const UniformTexelBuffer = 1u32 << 3;
        const StorageTexelBuffer = 1u32 << 4;
        const StorageTexelBufferAtomic = 1u32 << 5;
        const VertexBuffer = 1u32 << 6;
        const ColorAttachment = 1u32 << 7;
        const ColorAttachmentBlend = 1u32 << 8;
        const DepthStencilAttachment = 1u32 << 9;
        const BlitSrc = 1u32 << 10;
        const BlitDst = 1u32 << 11;
        const SampledImageFilterLinear = 1u32 << 12;
        const TransferSrc = 1u32 << 14;
        const TransferDst = 1u32 << 15;
        const MidpointChromaSamples = 1u32 << 17;
        const SampledImageYcbcrConversionLinearFilter = 1u32 << 18;
        const SampledImageYcbcrConversionSeparateReconstructionFilter = 1u32 << 19;
        const SampledImageYcbcrConversionChromaReconstructionExplicit = 1u32 << 20;
        const SampledImageYcbcrConversionChromaReconstructionExplicitForceable = 1u32 << 21;
        const Disjoint = 1u32 << 22;
        const CositedChromaSamples = 1u32 << 23;
        const SampledImageFilterMinmax = 1u32 << 16;
        const SampledImageFilterCubicIMG = Self::SampledImageFilterCubicEXT.bits();
        const TransferSrcKHR = Self::TransferSrc.bits();
        const TransferDstKHR = Self::TransferDst.bits();
        const SampledImageFilterMinmaxEXT = Self::SampledImageFilterMinmax.bits();
        const AccelerationStructureVertexBufferKHR = 1u32 << 29;
        const MidpointChromaSamplesKHR = Self::MidpointChromaSamples.bits();
        const SampledImageYcbcrConversionLinearFilterKHR = Self::SampledImageYcbcrConversionLinearFilter.bits();
        const SampledImageYcbcrConversionSeparateReconstructionFilterKHR = Self::SampledImageYcbcrConversionSeparateReconstructionFilter.bits();
        const SampledImageYcbcrConversionChromaReconstructionExplicitKHR = Self::SampledImageYcbcrConversionChromaReconstructionExplicit.bits();
        const SampledImageYcbcrConversionChromaReconstructionExplicitForceableKHR = Self::SampledImageYcbcrConversionChromaReconstructionExplicitForceable.bits();
        const DisjointKHR = Self::Disjoint.bits();
        const CositedChromaSamplesKHR = Self::CositedChromaSamples.bits();
        const SampledImageFilterCubicEXT = 1u32 << 13;
        const FragmentDensityMapEXT = 1u32 << 24;
        const FragmentShadingRateAttachmentKHR = 1u32 << 30;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCreateFlagBits.html>"]
    #[doc(alias = "VkImageCreateFlagBits")]
    pub struct ImageCreateFlags : u32 {
        const SparseBinding = 1u32 << 0;
        const SparseResidency = 1u32 << 1;
        const SparseAliased = 1u32 << 2;
        const MutableFormat = 1u32 << 3;
        const CubeCompatible = 1u32 << 4;
        const Alias = 1u32 << 10;
        const SplitInstanceBindRegions = 1u32 << 6;
        const Image2DArrayCompatible = 1u32 << 5;
        const BlockTexelViewCompatible = 1u32 << 7;
        const ExtendedUsage = 1u32 << 8;
        const Protected = 1u32 << 11;
        const Disjoint = 1u32 << 9;
        const CornerSampledNV = 1u32 << 13;
        const SplitInstanceBindRegionsKHR = Self::SplitInstanceBindRegions.bits();
        const Image2DArrayCompatibleKHR = Self::Image2DArrayCompatible.bits();
        const BlockTexelViewCompatibleKHR = Self::BlockTexelViewCompatible.bits();
        const ExtendedUsageKHR = Self::ExtendedUsage.bits();
        const SampleLocationsCompatibleDepthEXT = 1u32 << 12;
        const DisjointKHR = Self::Disjoint.bits();
        const AliasKHR = Self::Alias.bits();
        const SubsampledEXT = 1u32 << 14;
        const DescriptorBufferCaptureReplayEXT = 1u32 << 16;
        const MultisampledRenderToSingleSampledEXT = 1u32 << 18;
        const Image2DViewCompatibleEXT = 1u32 << 17;
        const FragmentDensityMapOffsetQCOM = 1u32 << 15;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html>"]
#[doc(alias = "VkImageTiling")]
#[repr(u32)]
pub enum ImageTiling {
    Optimal = 0,
    Linear = 1,
    DrmFormatModifierEXT = 1000158000,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageType.html>"]
#[doc(alias = "VkImageType")]
#[repr(u32)]
pub enum ImageType {
    Type1D = 0,
    Type2D = 1,
    Type3D = 2,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageUsageFlagBits.html>"]
    #[doc(alias = "VkImageUsageFlagBits")]
    pub struct ImageUsageFlags : u32 {
        const TransferSrc = 1u32 << 0;
        const TransferDst = 1u32 << 1;
        const Sampled = 1u32 << 2;
        const Storage = 1u32 << 3;
        const ColorAttachment = 1u32 << 4;
        const DepthStencilAttachment = 1u32 << 5;
        const TransientAttachment = 1u32 << 6;
        const InputAttachment = 1u32 << 7;
        const ShadingRateImageNV = Self::FragmentShadingRateAttachmentKHR.bits();
        const FragmentDensityMapEXT = 1u32 << 9;
        const FragmentShadingRateAttachmentKHR = 1u32 << 8;
        const HostTransferEXT = 1u32 << 22;
        const AttachmentFeedbackLoopEXT = 1u32 << 19;
        const InvocationMaskHUAWEI = 1u32 << 18;
        const SampleWeightQCOM = 1u32 << 20;
        const SampleBlockMatchQCOM = 1u32 << 21;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateFlagBits.html>"]
    #[doc(alias = "VkInstanceCreateFlagBits")]
    pub struct InstanceCreateFlags : u32 {
        const EnumeratePortabilityKHR = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInternalAllocationType.html>"]
#[doc(alias = "VkInternalAllocationType")]
#[repr(u32)]
pub enum InternalAllocationType {
    Executable = 0,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHeapFlagBits.html>"]
    #[doc(alias = "VkMemoryHeapFlagBits")]
    pub struct MemoryHeapFlags : u32 {
        const DeviceLocal = 1u32 << 0;
        const MultiInstance = 1u32 << 1;
        const MultiInstanceKHR = Self::MultiInstance.bits();
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPropertyFlagBits.html>"]
    #[doc(alias = "VkMemoryPropertyFlagBits")]
    pub struct MemoryPropertyFlags : u32 {
        const DeviceLocal = 1u32 << 0;
        const HostVisible = 1u32 << 1;
        const HostCoherent = 1u32 << 2;
        const HostCached = 1u32 << 3;
        const LazilyAllocated = 1u32 << 4;
        const Protected = 1u32 << 5;
        const DeviceCoherentAMD = 1u32 << 6;
        const DeviceUncachedAMD = 1u32 << 7;
        const RdmaCapableNV = 1u32 << 8;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceType.html>"]
#[doc(alias = "VkPhysicalDeviceType")]
#[repr(u32)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFlagBits.html>"]
    #[doc(alias = "VkQueueFlagBits")]
    pub struct QueueFlags : u32 {
        const Graphics = 1u32 << 0;
        const Compute = 1u32 << 1;
        const Transfer = 1u32 << 2;
        const SparseBinding = 1u32 << 3;
        const Protected = 1u32 << 4;
        const OpticalFlowNV = 1u32 << 8;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleCountFlagBits.html>"]
    #[doc(alias = "VkSampleCountFlagBits")]
    pub struct SampleCountFlags : u32 {
        const Count1 = 1u32 << 0;
        const Count2 = 1u32 << 1;
        const Count4 = 1u32 << 2;
        const Count8 = 1u32 << 3;
        const Count16 = 1u32 << 4;
        const Count32 = 1u32 << 5;
        const Count64 = 1u32 << 6;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSystemAllocationScope.html>"]
#[doc(alias = "VkSystemAllocationScope")]
#[repr(u32)]
pub enum SystemAllocationScope {
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html>"]
    #[doc(alias = "VkPipelineStageFlagBits")]
    pub struct PipelineStageFlags : u32 {
        const TopOfPipe = 1u32 << 0;
        const DrawIndirect = 1u32 << 1;
        const VertexInput = 1u32 << 2;
        const VertexShader = 1u32 << 3;
        const TessellationControlShader = 1u32 << 4;
        const TessellationEvaluationShader = 1u32 << 5;
        const GeometryShader = 1u32 << 6;
        const FragmentShader = 1u32 << 7;
        const EarlyFragmentTests = 1u32 << 8;
        const LateFragmentTests = 1u32 << 9;
        const ColorAttachmentOutput = 1u32 << 10;
        const ComputeShader = 1u32 << 11;
        const Transfer = 1u32 << 12;
        const BottomOfPipe = 1u32 << 13;
        const Host = 1u32 << 14;
        const AllGraphics = 1u32 << 15;
        const AllCommands = 1u32 << 16;
        const None = 0;
        const TransformFeedbackEXT = 1u32 << 24;
        const ConditionalRenderingEXT = 1u32 << 18;
        const AccelerationStructureBuildKHR = 1u32 << 25;
        const RayTracingShaderKHR = 1u32 << 21;
        const ShadingRateImageNV = Self::FragmentShadingRateAttachmentKHR.bits();
        const RayTracingShaderNV = Self::RayTracingShaderKHR.bits();
        const AccelerationStructureBuildNV = Self::AccelerationStructureBuildKHR.bits();
        const TaskShaderNV = Self::TaskShaderEXT.bits();
        const MeshShaderNV = Self::MeshShaderEXT.bits();
        const FragmentDensityProcessEXT = 1u32 << 23;
        const FragmentShadingRateAttachmentKHR = 1u32 << 22;
        const CommandPreprocessNV = 1u32 << 17;
        const NoneKHR = Self::None.bits();
        const TaskShaderEXT = 1u32 << 19;
        const MeshShaderEXT = 1u32 << 20;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryMapFlagBits.html>"]
    #[doc(alias = "VkMemoryMapFlagBits")]
    pub struct MemoryMapFlags : u32 {
        const PlacedEXT = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html>"]
    #[doc(alias = "VkImageAspectFlagBits")]
    pub struct ImageAspectFlags : u32 {
        const Color = 1u32 << 0;
        const Depth = 1u32 << 1;
        const Stencil = 1u32 << 2;
        const Metadata = 1u32 << 3;
        const Plane0 = 1u32 << 4;
        const Plane1 = 1u32 << 5;
        const Plane2 = 1u32 << 6;
        const None = 0;
        const Plane0KHR = Self::Plane0.bits();
        const Plane1KHR = Self::Plane1.bits();
        const Plane2KHR = Self::Plane2.bits();
        const MemoryPlane0EXT = 1u32 << 7;
        const MemoryPlane1EXT = 1u32 << 8;
        const MemoryPlane2EXT = 1u32 << 9;
        const MemoryPlane3EXT = 1u32 << 10;
        const NoneKHR = Self::None.bits();
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html>"]
    #[doc(alias = "VkSparseImageFormatFlagBits")]
    pub struct SparseImageFormatFlags : u32 {
        const SingleMiptail = 1u32 << 0;
        const AlignedMipSize = 1u32 << 1;
        const NonstandardBlockSize = 1u32 << 2;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseMemoryBindFlagBits.html>"]
    #[doc(alias = "VkSparseMemoryBindFlagBits")]
    pub struct SparseMemoryBindFlags : u32 {
        const Metadata = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceCreateFlagBits.html>"]
    #[doc(alias = "VkFenceCreateFlagBits")]
    pub struct FenceCreateFlags : u32 {
        const Signaled = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkEventCreateFlagBits.html>"]
    #[doc(alias = "VkEventCreateFlagBits")]
    pub struct EventCreateFlags : u32 {
        const DeviceOnly = 1u32 << 0;
        const DeviceOnlyKHR = Self::DeviceOnly.bits();
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPipelineStatisticFlagBits.html>"]
    #[doc(alias = "VkQueryPipelineStatisticFlagBits")]
    pub struct QueryPipelineStatisticFlags : u32 {
        const InputAssemblyVertices = 1u32 << 0;
        const InputAssemblyPrimitives = 1u32 << 1;
        const VertexShaderInvocations = 1u32 << 2;
        const GeometryShaderInvocations = 1u32 << 3;
        const GeometryShaderPrimitives = 1u32 << 4;
        const ClippingInvocations = 1u32 << 5;
        const ClippingPrimitives = 1u32 << 6;
        const FragmentShaderInvocations = 1u32 << 7;
        const TessellationControlShaderPatches = 1u32 << 8;
        const TessellationEvaluationShaderInvocations = 1u32 << 9;
        const ComputeShaderInvocations = 1u32 << 10;
        const TaskShaderInvocationsEXT = 1u32 << 11;
        const MeshShaderInvocationsEXT = 1u32 << 12;
        const ClusterCullingShaderInvocationsHUAWEI = 1u32 << 13;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultFlagBits.html>"]
    #[doc(alias = "VkQueryResultFlagBits")]
    pub struct QueryResultFlags : u32 {
        const Result64 = 1u32 << 0;
        const Wait = 1u32 << 1;
        const WithAvailability = 1u32 << 2;
        const Partial = 1u32 << 3;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryType.html>"]
#[doc(alias = "VkQueryType")]
#[repr(u32)]
pub enum QueryType {
    Occlusion = 0,
    PipelineStatistics = 1,
    Timestamp = 2,
    TransformFeedbackStreamEXT = 1000028004,
    PerformanceQueryKHR = 1000116000,
    AccelerationStructureCompactedSizeKHR = 1000150000,
    AccelerationStructureSerializationSizeKHR = 1000150001,
    AccelerationStructureCompactedSizeNV = 1000165000,
    PerformanceQueryINTEL = 1000210000,
    MeshPrimitivesGeneratedEXT = 1000328000,
    PrimitivesGeneratedEXT = 1000382000,
    AccelerationStructureSerializationBottomLevelPointersKHR = 1000386000,
    AccelerationStructureSizeKHR = 1000386001,
    MicromapSerializationSizeEXT = 1000396000,
    MicromapCompactedSizeEXT = 1000396001,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCreateFlagBits.html>"]
    #[doc(alias = "VkBufferCreateFlagBits")]
    pub struct BufferCreateFlags : u32 {
        const SparseBinding = 1u32 << 0;
        const SparseResidency = 1u32 << 1;
        const SparseAliased = 1u32 << 2;
        const Protected = 1u32 << 3;
        const DeviceAddressCaptureReplay = 1u32 << 4;
        const DeviceAddressCaptureReplayEXT = Self::DeviceAddressCaptureReplay.bits();
        const DeviceAddressCaptureReplayKHR = Self::DeviceAddressCaptureReplay.bits();
        const DescriptorBufferCaptureReplayEXT = 1u32 << 5;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html>"]
    #[doc(alias = "VkBufferUsageFlagBits")]
    pub struct BufferUsageFlags : u32 {
        const TransferSrc = 1u32 << 0;
        const TransferDst = 1u32 << 1;
        const UniformTexelBuffer = 1u32 << 2;
        const StorageTexelBuffer = 1u32 << 3;
        const UniformBuffer = 1u32 << 4;
        const StorageBuffer = 1u32 << 5;
        const IndexBuffer = 1u32 << 6;
        const VertexBuffer = 1u32 << 7;
        const IndirectBuffer = 1u32 << 8;
        const ShaderDeviceAddress = 1u32 << 17;
        const TransformFeedbackBufferEXT = 1u32 << 11;
        const TransformFeedbackCounterBufferEXT = 1u32 << 12;
        const ConditionalRenderingEXT = 1u32 << 9;
        const ExecutionGraphScratchAMDX = 1u32 << 25;
        const AccelerationStructureBuildInputReadOnlyKHR = 1u32 << 19;
        const AccelerationStructureStorageKHR = 1u32 << 20;
        const ShaderBindingTableKHR = 1u32 << 10;
        const RayTracingNV = Self::ShaderBindingTableKHR.bits();
        const ShaderDeviceAddressEXT = Self::ShaderDeviceAddress.bits();
        const ShaderDeviceAddressKHR = Self::ShaderDeviceAddress.bits();
        const SamplerDescriptorBufferEXT = 1u32 << 21;
        const ResourceDescriptorBufferEXT = 1u32 << 22;
        const PushDescriptorsDescriptorBufferEXT = 1u32 << 26;
        const MicromapBuildInputReadOnlyEXT = 1u32 << 23;
        const MicromapStorageEXT = 1u32 << 24;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharingMode.html>"]
#[doc(alias = "VkSharingMode")]
#[repr(u32)]
pub enum SharingMode {
    Exclusive = 0,
    Concurrent = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageLayout.html>"]
#[doc(alias = "VkImageLayout")]
#[repr(u32)]
pub enum ImageLayout {
    Undefined = 0,
    General = 1,
    ColorAttachmentOptimal = 2,
    DepthStencilAttachmentOptimal = 3,
    DepthStencilReadOnlyOptimal = 4,
    ShaderReadOnlyOptimal = 5,
    TransferSrcOptimal = 6,
    TransferDstOptimal = 7,
    Preinitialized = 8,
    DepthReadOnlyStencilAttachmentOptimal = 1000117000,
    DepthAttachmentStencilReadOnlyOptimal = 1000117001,
    DepthAttachmentOptimal = 1000241000,
    DepthReadOnlyOptimal = 1000241001,
    StencilAttachmentOptimal = 1000241002,
    StencilReadOnlyOptimal = 1000241003,
    ReadOnlyOptimal = 1000314000,
    AttachmentOptimal = 1000314001,
    PresentSrcKHR = 1000001002,
    SharedPresentKHR = 1000111000,
    FragmentDensityMapOptimalEXT = 1000218000,
    FragmentShadingRateAttachmentOptimalKHR = 1000164003,
    RenderingLocalReadKHR = 1000232000,
    AttachmentFeedbackLoopOptimalEXT = 1000339000,
}
#[allow(non_upper_case_globals)]
impl ImageLayout {
    pub const DepthReadOnlyStencilAttachmentOptimalKHR: Self =
        Self::DepthReadOnlyStencilAttachmentOptimal;
    pub const DepthAttachmentStencilReadOnlyOptimalKHR: Self =
        Self::DepthAttachmentStencilReadOnlyOptimal;
    pub const ShadingRateOptimalNV: Self = Self::FragmentShadingRateAttachmentOptimalKHR;
    pub const DepthAttachmentOptimalKHR: Self = Self::DepthAttachmentOptimal;
    pub const DepthReadOnlyOptimalKHR: Self = Self::DepthReadOnlyOptimal;
    pub const StencilAttachmentOptimalKHR: Self = Self::StencilAttachmentOptimal;
    pub const StencilReadOnlyOptimalKHR: Self = Self::StencilReadOnlyOptimal;
    pub const ReadOnlyOptimalKHR: Self = Self::ReadOnlyOptimal;
    pub const AttachmentOptimalKHR: Self = Self::AttachmentOptimal;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentSwizzle.html>"]
#[doc(alias = "VkComponentSwizzle")]
#[repr(u32)]
pub enum ComponentSwizzle {
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewCreateFlagBits.html>"]
    #[doc(alias = "VkImageViewCreateFlagBits")]
    pub struct ImageViewCreateFlags : u32 {
        const FragmentDensityMapDynamicEXT = 1u32 << 0;
        const DescriptorBufferCaptureReplayEXT = 1u32 << 2;
        const FragmentDensityMapDeferredEXT = 1u32 << 1;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewType.html>"]
#[doc(alias = "VkImageViewType")]
#[repr(u32)]
pub enum ImageViewType {
    Type1D = 0,
    Type2D = 1,
    Type3D = 2,
    Cube = 3,
    Type1DArray = 4,
    Type2DArray = 5,
    CubeArray = 6,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html>"]
#[doc(alias = "VkBlendFactor")]
#[repr(u32)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    DstColor = 4,
    OneMinusDstColor = 5,
    SrcAlpha = 6,
    OneMinusSrcAlpha = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
    ConstantColor = 10,
    OneMinusConstantColor = 11,
    ConstantAlpha = 12,
    OneMinusConstantAlpha = 13,
    SrcAlphaSaturate = 14,
    Src1Color = 15,
    OneMinusSrc1Color = 16,
    Src1Alpha = 17,
    OneMinusSrc1Alpha = 18,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html>"]
#[doc(alias = "VkBlendOp")]
#[repr(u32)]
pub enum BlendOp {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    ZeroEXT = 1000148000,
    SrcEXT = 1000148001,
    DstEXT = 1000148002,
    SrcOverEXT = 1000148003,
    DstOverEXT = 1000148004,
    SrcInEXT = 1000148005,
    DstInEXT = 1000148006,
    SrcOutEXT = 1000148007,
    DstOutEXT = 1000148008,
    SrcAtopEXT = 1000148009,
    DstAtopEXT = 1000148010,
    XorEXT = 1000148011,
    MultiplyEXT = 1000148012,
    ScreenEXT = 1000148013,
    OverlayEXT = 1000148014,
    DarkenEXT = 1000148015,
    LightenEXT = 1000148016,
    ColordodgeEXT = 1000148017,
    ColorburnEXT = 1000148018,
    HardlightEXT = 1000148019,
    SoftlightEXT = 1000148020,
    DifferenceEXT = 1000148021,
    ExclusionEXT = 1000148022,
    InvertEXT = 1000148023,
    InvertRgbEXT = 1000148024,
    LineardodgeEXT = 1000148025,
    LinearburnEXT = 1000148026,
    VividlightEXT = 1000148027,
    LinearlightEXT = 1000148028,
    PinlightEXT = 1000148029,
    HardmixEXT = 1000148030,
    HslHueEXT = 1000148031,
    HslSaturationEXT = 1000148032,
    HslColorEXT = 1000148033,
    HslLuminosityEXT = 1000148034,
    PlusEXT = 1000148035,
    PlusClampedEXT = 1000148036,
    PlusClampedAlphaEXT = 1000148037,
    PlusDarkerEXT = 1000148038,
    MinusEXT = 1000148039,
    MinusClampedEXT = 1000148040,
    ContrastEXT = 1000148041,
    InvertOvgEXT = 1000148042,
    RedEXT = 1000148043,
    GreenEXT = 1000148044,
    BlueEXT = 1000148045,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html>"]
    #[doc(alias = "VkColorComponentFlagBits")]
    pub struct ColorComponentFlags : u32 {
        const R = 1u32 << 0;
        const G = 1u32 << 1;
        const B = 1u32 << 2;
        const A = 1u32 << 3;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html>"]
#[doc(alias = "VkCompareOp")]
#[repr(u32)]
pub enum CompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCullModeFlagBits.html>"]
    #[doc(alias = "VkCullModeFlagBits")]
    pub struct CullModeFlags : u32 {
        const None = 0;
        const Front = 1u32 << 0;
        const Back = 1u32 << 1;
        const FrontAndBack = 0x00000003;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html>"]
#[doc(alias = "VkDynamicState")]
#[repr(u32)]
pub enum DynamicState {
    Viewport = 0,
    Scissor = 1,
    LineWidth = 2,
    DepthBias = 3,
    BlendConstants = 4,
    DepthBounds = 5,
    StencilCompareMask = 6,
    StencilWriteMask = 7,
    StencilReference = 8,
    CullMode = 1000267000,
    FrontFace = 1000267001,
    PrimitiveTopology = 1000267002,
    ViewportWithCount = 1000267003,
    ScissorWithCount = 1000267004,
    VertexInputBindingStride = 1000267005,
    DepthTestEnable = 1000267006,
    DepthWriteEnable = 1000267007,
    DepthCompareOp = 1000267008,
    DepthBoundsTestEnable = 1000267009,
    StencilTestEnable = 1000267010,
    StencilOp = 1000267011,
    RasterizerDiscardEnable = 1000377001,
    DepthBiasEnable = 1000377002,
    PrimitiveRestartEnable = 1000377004,
    ViewportWScalingNV = 1000087000,
    DiscardRectangleEXT = 1000099000,
    DiscardRectangleEnableEXT = 1000099001,
    DiscardRectangleModeEXT = 1000099002,
    SampleLocationsEXT = 1000143000,
    RayTracingPipelineStackSizeKHR = 1000347000,
    ViewportShadingRatePaletteNV = 1000164004,
    ViewportCoarseSampleOrderNV = 1000164006,
    ExclusiveScissorEnableNV = 1000205000,
    ExclusiveScissorNV = 1000205001,
    FragmentShadingRateKHR = 1000226000,
    VertexInputEXT = 1000352000,
    PatchControlPointsEXT = 1000377000,
    LogicOpEXT = 1000377003,
    ColorWriteEnableEXT = 1000381000,
    DepthClampEnableEXT = 1000455003,
    PolygonModeEXT = 1000455004,
    RasterizationSamplesEXT = 1000455005,
    SampleMaskEXT = 1000455006,
    AlphaToCoverageEnableEXT = 1000455007,
    AlphaToOneEnableEXT = 1000455008,
    LogicOpEnableEXT = 1000455009,
    ColorBlendEnableEXT = 1000455010,
    ColorBlendEquationEXT = 1000455011,
    ColorWriteMaskEXT = 1000455012,
    TessellationDomainOriginEXT = 1000455002,
    RasterizationStreamEXT = 1000455013,
    ConservativeRasterizationModeEXT = 1000455014,
    ExtraPrimitiveOverestimationSizeEXT = 1000455015,
    DepthClipEnableEXT = 1000455016,
    SampleLocationsEnableEXT = 1000455017,
    ColorBlendAdvancedEXT = 1000455018,
    ProvokingVertexModeEXT = 1000455019,
    LineRasterizationModeEXT = 1000455020,
    LineStippleEnableEXT = 1000455021,
    DepthClipNegativeOneToOneEXT = 1000455022,
    ViewportWScalingEnableNV = 1000455023,
    ViewportSwizzleNV = 1000455024,
    CoverageToColorEnableNV = 1000455025,
    CoverageToColorLocationNV = 1000455026,
    CoverageModulationModeNV = 1000455027,
    CoverageModulationTableEnableNV = 1000455028,
    CoverageModulationTableNV = 1000455029,
    ShadingRateImageEnableNV = 1000455030,
    RepresentativeFragmentTestEnableNV = 1000455031,
    CoverageReductionModeNV = 1000455032,
    AttachmentFeedbackLoopEnableEXT = 1000524000,
    LineStippleKHR = 1000259000,
}
#[allow(non_upper_case_globals)]
impl DynamicState {
    pub const LineStippleEXT: Self = Self::LineStippleKHR;
    pub const CullModeEXT: Self = Self::CullMode;
    pub const FrontFaceEXT: Self = Self::FrontFace;
    pub const PrimitiveTopologyEXT: Self = Self::PrimitiveTopology;
    pub const ViewportWithCountEXT: Self = Self::ViewportWithCount;
    pub const ScissorWithCountEXT: Self = Self::ScissorWithCount;
    pub const VertexInputBindingStrideEXT: Self = Self::VertexInputBindingStride;
    pub const DepthTestEnableEXT: Self = Self::DepthTestEnable;
    pub const DepthWriteEnableEXT: Self = Self::DepthWriteEnable;
    pub const DepthCompareOpEXT: Self = Self::DepthCompareOp;
    pub const DepthBoundsTestEnableEXT: Self = Self::DepthBoundsTestEnable;
    pub const StencilTestEnableEXT: Self = Self::StencilTestEnable;
    pub const StencilOpEXT: Self = Self::StencilOp;
    pub const RasterizerDiscardEnableEXT: Self = Self::RasterizerDiscardEnable;
    pub const DepthBiasEnableEXT: Self = Self::DepthBiasEnable;
    pub const PrimitiveRestartEnableEXT: Self = Self::PrimitiveRestartEnable;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFrontFace.html>"]
#[doc(alias = "VkFrontFace")]
#[repr(u32)]
pub enum FrontFace {
    CounterClockwise = 0,
    Clockwise = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html>"]
#[doc(alias = "VkLogicOp")]
#[repr(u32)]
pub enum LogicOp {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    NoOp = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equivalent = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html>"]
    #[doc(alias = "VkPipelineCreateFlagBits")]
    pub struct PipelineCreateFlags : u32 {
        const DisableOptimization = 1u32 << 0;
        const AllowDerivatives = 1u32 << 1;
        const Derivative = 1u32 << 2;
        const ViewIndexFromDeviceIndex = 1u32 << 3;
        const DispatchBase = 1u32 << 4;
        const FailOnPipelineCompileRequired = 1u32 << 8;
        const EarlyReturnOnFailure = 1u32 << 9;
        const RenderingFragmentShadingRateAttachmentKHR = 1u32 << 21;
        const RasterizationStateCreateFragmentShadingRateAttachmentKHR = Self::RenderingFragmentShadingRateAttachmentKHR.bits();
        const RenderingFragmentDensityMapAttachmentEXT = 1u32 << 22;
        const RasterizationStateCreateFragmentDensityMapAttachmentEXT = Self::RenderingFragmentDensityMapAttachmentEXT.bits();
        const ViewIndexFromDeviceIndexKHR = Self::ViewIndexFromDeviceIndex.bits();
        const DispatchBaseKHR = Self::DispatchBase.bits();
        const RayTracingNoNullAnyHitShadersKHR = 1u32 << 14;
        const RayTracingNoNullClosestHitShadersKHR = 1u32 << 15;
        const RayTracingNoNullMissShadersKHR = 1u32 << 16;
        const RayTracingNoNullIntersectionShadersKHR = 1u32 << 17;
        const RayTracingSkipTrianglesKHR = 1u32 << 12;
        const RayTracingSkipAabbsKHR = 1u32 << 13;
        const RayTracingShaderGroupHandleCaptureReplayKHR = 1u32 << 19;
        const DeferCompileNV = 1u32 << 5;
        const CaptureStatisticsKHR = 1u32 << 6;
        const CaptureInternalRepresentationsKHR = 1u32 << 7;
        const IndirectBindableNV = 1u32 << 18;
        const LibraryKHR = 1u32 << 11;
        const FailOnPipelineCompileRequiredEXT = Self::FailOnPipelineCompileRequired.bits();
        const EarlyReturnOnFailureEXT = Self::EarlyReturnOnFailure.bits();
        const DescriptorBufferEXT = 1u32 << 29;
        const RetainLinkTimeOptimizationInfoEXT = 1u32 << 23;
        const LinkTimeOptimizationEXT = 1u32 << 10;
        const RayTracingAllowMotionNV = 1u32 << 20;
        const ColorAttachmentFeedbackLoopEXT = 1u32 << 25;
        const DepthStencilAttachmentFeedbackLoopEXT = 1u32 << 26;
        const RayTracingOpacityMicromapEXT = 1u32 << 24;
        const RayTracingDisplacementMicromapNV = 1u32 << 28;
        const NoProtectedAccessEXT = 1u32 << 27;
        const ProtectedAccessOnlyEXT = 1u32 << 30;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html>"]
    #[doc(alias = "VkPipelineShaderStageCreateFlagBits")]
    pub struct PipelineShaderStageCreateFlags : u32 {
        const AllowVaryingSubgroupSize = 1u32 << 0;
        const RequireFullSubgroups = 1u32 << 1;
        const AllowVaryingSubgroupSizeEXT = Self::AllowVaryingSubgroupSize.bits();
        const RequireFullSubgroupsEXT = Self::RequireFullSubgroups.bits();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html>"]
#[doc(alias = "VkPolygonMode")]
#[repr(u32)]
pub enum PolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
    FillRectangleNV = 1000153000,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrimitiveTopology.html>"]
#[doc(alias = "VkPrimitiveTopology")]
#[repr(u32)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    LineListWithAdjacency = 6,
    LineStripWithAdjacency = 7,
    TriangleListWithAdjacency = 8,
    TriangleStripWithAdjacency = 9,
    PatchList = 10,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html>"]
    #[doc(alias = "VkShaderStageFlagBits")]
    pub struct ShaderStageFlags : u32 {
        const Vertex = 1u32 << 0;
        const TessellationControl = 1u32 << 1;
        const TessellationEvaluation = 1u32 << 2;
        const Geometry = 1u32 << 3;
        const Fragment = 1u32 << 4;
        const Compute = 1u32 << 5;
        const AllGraphics = 0x0000001F;
        const All = 0x7FFFFFFF;
        const RaygenKHR = 1u32 << 8;
        const AnyHitKHR = 1u32 << 9;
        const ClosestHitKHR = 1u32 << 10;
        const MissKHR = 1u32 << 11;
        const IntersectionKHR = 1u32 << 12;
        const CallableKHR = 1u32 << 13;
        const RaygenNV = Self::RaygenKHR.bits();
        const AnyHitNV = Self::AnyHitKHR.bits();
        const ClosestHitNV = Self::ClosestHitKHR.bits();
        const MissNV = Self::MissKHR.bits();
        const IntersectionNV = Self::IntersectionKHR.bits();
        const CallableNV = Self::CallableKHR.bits();
        const TaskNV = Self::TaskEXT.bits();
        const MeshNV = Self::MeshEXT.bits();
        const TaskEXT = 1u32 << 6;
        const MeshEXT = 1u32 << 7;
        const SubpassShadingHUAWEI = 1u32 << 14;
        const ClusterCullingHUAWEI = 1u32 << 19;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilOp.html>"]
#[doc(alias = "VkStencilOp")]
#[repr(u32)]
pub enum StencilOp {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementAndClamp = 3,
    DecrementAndClamp = 4,
    Invert = 5,
    IncrementAndWrap = 6,
    DecrementAndWrap = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputRate.html>"]
#[doc(alias = "VkVertexInputRate")]
#[repr(u32)]
pub enum VertexInputRate {
    Vertex = 0,
    Instance = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBorderColor.html>"]
#[doc(alias = "VkBorderColor")]
#[repr(u32)]
pub enum BorderColor {
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,
    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5,
    FloatCustomEXT = 1000287003,
    IntCustomEXT = 1000287004,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilter.html>"]
#[doc(alias = "VkFilter")]
#[repr(u32)]
pub enum Filter {
    Nearest = 0,
    Linear = 1,
    CubicEXT = 1000015000,
}
#[allow(non_upper_case_globals)]
impl Filter {
    pub const CubicIMG: Self = Self::CubicEXT;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html>"]
#[doc(alias = "VkSamplerAddressMode")]
#[repr(u32)]
pub enum SamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    MirrorClampToEdge = 4,
}
#[allow(non_upper_case_globals)]
impl SamplerAddressMode {
    pub const MirrorClampToEdgeKHR: Self = Self::MirrorClampToEdge;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html>"]
    #[doc(alias = "VkSamplerCreateFlagBits")]
    pub struct SamplerCreateFlags : u32 {
        const SubsampledEXT = 1u32 << 0;
        const SubsampledCoarseReconstructionEXT = 1u32 << 1;
        const DescriptorBufferCaptureReplayEXT = 1u32 << 3;
        const NonSeamlessCubeMapEXT = 1u32 << 2;
        const ImageProcessingQCOM = 1u32 << 4;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html>"]
#[doc(alias = "VkSamplerMipmapMode")]
#[repr(u32)]
pub enum SamplerMipmapMode {
    Nearest = 0,
    Linear = 1,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolCreateFlagBits.html>"]
    #[doc(alias = "VkDescriptorPoolCreateFlagBits")]
    pub struct DescriptorPoolCreateFlags : u32 {
        const FreeDescriptorSet = 1u32 << 0;
        const UpdateAfterBind = 1u32 << 1;
        const UpdateAfterBindEXT = Self::UpdateAfterBind.bits();
        const HostOnlyVALVE = Self::HostOnlyEXT.bits();
        const HostOnlyEXT = 1u32 << 2;
        const AllowOverallocationSetsNV = 1u32 << 3;
        const AllowOverallocationPoolsNV = 1u32 << 4;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html>"]
    #[doc(alias = "VkDescriptorSetLayoutCreateFlagBits")]
    pub struct DescriptorSetLayoutCreateFlags : u32 {
        const UpdateAfterBindPool = 1u32 << 1;
        const PushDescriptorKHR = 1u32 << 0;
        const UpdateAfterBindPoolEXT = Self::UpdateAfterBindPool.bits();
        const DescriptorBufferEXT = 1u32 << 4;
        const EmbeddedImmutableSamplersEXT = 1u32 << 5;
        const HostOnlyPoolVALVE = Self::HostOnlyPoolEXT.bits();
        const IndirectBindableNV = 1u32 << 7;
        const HostOnlyPoolEXT = 1u32 << 2;
        const PerStageNV = 1u32 << 6;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorType.html>"]
#[doc(alias = "VkDescriptorType")]
#[repr(u32)]
pub enum DescriptorType {
    Sampler = 0,
    CombinedImageSampler = 1,
    SampledImage = 2,
    StorageImage = 3,
    UniformTexelBuffer = 4,
    StorageTexelBuffer = 5,
    UniformBuffer = 6,
    StorageBuffer = 7,
    UniformBufferDynamic = 8,
    StorageBufferDynamic = 9,
    InputAttachment = 10,
    InlineUniformBlock = 1000138000,
    AccelerationStructureKHR = 1000150000,
    AccelerationStructureNV = 1000165000,
    SampleWeightImageQCOM = 1000440000,
    BlockMatchImageQCOM = 1000440001,
    MutableEXT = 1000351000,
}
#[allow(non_upper_case_globals)]
impl DescriptorType {
    pub const InlineUniformBlockEXT: Self = Self::InlineUniformBlock;
    pub const MutableVALVE: Self = Self::MutableEXT;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html>"]
    #[doc(alias = "VkAccessFlagBits")]
    pub struct AccessFlags : u32 {
        const IndirectCommandRead = 1u32 << 0;
        const IndexRead = 1u32 << 1;
        const VertexAttributeRead = 1u32 << 2;
        const UniformRead = 1u32 << 3;
        const InputAttachmentRead = 1u32 << 4;
        const ShaderRead = 1u32 << 5;
        const ShaderWrite = 1u32 << 6;
        const ColorAttachmentRead = 1u32 << 7;
        const ColorAttachmentWrite = 1u32 << 8;
        const DepthStencilAttachmentRead = 1u32 << 9;
        const DepthStencilAttachmentWrite = 1u32 << 10;
        const TransferRead = 1u32 << 11;
        const TransferWrite = 1u32 << 12;
        const HostRead = 1u32 << 13;
        const HostWrite = 1u32 << 14;
        const MemoryRead = 1u32 << 15;
        const MemoryWrite = 1u32 << 16;
        const None = 0;
        const TransformFeedbackWriteEXT = 1u32 << 25;
        const TransformFeedbackCounterReadEXT = 1u32 << 26;
        const TransformFeedbackCounterWriteEXT = 1u32 << 27;
        const ConditionalRenderingReadEXT = 1u32 << 20;
        const ColorAttachmentReadNoncoherentEXT = 1u32 << 19;
        const AccelerationStructureReadKHR = 1u32 << 21;
        const AccelerationStructureWriteKHR = 1u32 << 22;
        const ShadingRateImageReadNV = Self::FragmentShadingRateAttachmentReadKHR.bits();
        const AccelerationStructureReadNV = Self::AccelerationStructureReadKHR.bits();
        const AccelerationStructureWriteNV = Self::AccelerationStructureWriteKHR.bits();
        const FragmentDensityMapReadEXT = 1u32 << 24;
        const FragmentShadingRateAttachmentReadKHR = 1u32 << 23;
        const CommandPreprocessReadNV = 1u32 << 17;
        const CommandPreprocessWriteNV = 1u32 << 18;
        const NoneKHR = Self::None.bits();
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html>"]
    #[doc(alias = "VkAttachmentDescriptionFlagBits")]
    pub struct AttachmentDescriptionFlags : u32 {
        const MayAlias = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html>"]
#[doc(alias = "VkAttachmentLoadOp")]
#[repr(u32)]
pub enum AttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2,
    NoneKHR = 1000400000,
}
#[allow(non_upper_case_globals)]
impl AttachmentLoadOp {
    pub const NoneEXT: Self = Self::NoneKHR;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentStoreOp.html>"]
#[doc(alias = "VkAttachmentStoreOp")]
#[repr(u32)]
pub enum AttachmentStoreOp {
    Store = 0,
    DontCare = 1,
    None = 1000301000,
}
#[allow(non_upper_case_globals)]
impl AttachmentStoreOp {
    pub const NoneKHR: Self = Self::None;
    pub const NoneQCOM: Self = Self::None;
    pub const NoneEXT: Self = Self::None;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyFlagBits.html>"]
    #[doc(alias = "VkDependencyFlagBits")]
    pub struct DependencyFlags : u32 {
        const ByRegion = 1u32 << 0;
        const DeviceGroup = 1u32 << 2;
        const ViewLocal = 1u32 << 1;
        const ViewLocalKHR = Self::ViewLocal.bits();
        const DeviceGroupKHR = Self::DeviceGroup.bits();
        const FeedbackLoopEXT = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html>"]
    #[doc(alias = "VkFramebufferCreateFlagBits")]
    pub struct FramebufferCreateFlags : u32 {
        const Imageless = 1u32 << 0;
        const ImagelessKHR = Self::Imageless.bits();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html>"]
#[doc(alias = "VkPipelineBindPoint")]
#[repr(u32)]
pub enum PipelineBindPoint {
    Graphics = 0,
    Compute = 1,
    ExecutionGraphAMDX = 1000134000,
    RayTracingKHR = 1000165000,
    SubpassShadingHUAWEI = 1000369003,
}
#[allow(non_upper_case_globals)]
impl PipelineBindPoint {
    pub const RayTracingNV: Self = Self::RayTracingKHR;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html>"]
    #[doc(alias = "VkRenderPassCreateFlagBits")]
    pub struct RenderPassCreateFlags : u32 {
        const TransformQCOM = 1u32 << 1;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html>"]
    #[doc(alias = "VkSubpassDescriptionFlagBits")]
    pub struct SubpassDescriptionFlags : u32 {
        const PerViewAttributesNVX = 1u32 << 0;
        const PerViewPositionXOnlyNVX = 1u32 << 1;
        const FragmentRegionQCOM = 1u32 << 2;
        const ShaderResolveQCOM = 1u32 << 3;
        const RasterizationOrderAttachmentColorAccessARM = Self::RasterizationOrderAttachmentColorAccessEXT.bits();
        const RasterizationOrderAttachmentDepthAccessARM = Self::RasterizationOrderAttachmentDepthAccessEXT.bits();
        const RasterizationOrderAttachmentStencilAccessARM = Self::RasterizationOrderAttachmentStencilAccessEXT.bits();
        const RasterizationOrderAttachmentColorAccessEXT = 1u32 << 4;
        const RasterizationOrderAttachmentDepthAccessEXT = 1u32 << 5;
        const RasterizationOrderAttachmentStencilAccessEXT = 1u32 << 6;
        const EnableLegacyDitheringEXT = 1u32 << 7;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html>"]
    #[doc(alias = "VkCommandPoolCreateFlagBits")]
    pub struct CommandPoolCreateFlags : u32 {
        const Transient = 1u32 << 0;
        const ResetCommandBuffer = 1u32 << 1;
        const Protected = 1u32 << 2;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolResetFlagBits.html>"]
    #[doc(alias = "VkCommandPoolResetFlagBits")]
    pub struct CommandPoolResetFlags : u32 {
        const ReleaseResources = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html>"]
#[doc(alias = "VkCommandBufferLevel")]
#[repr(u32)]
pub enum CommandBufferLevel {
    Primary = 0,
    Secondary = 1,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html>"]
    #[doc(alias = "VkCommandBufferResetFlagBits")]
    pub struct CommandBufferResetFlags : u32 {
        const ReleaseResources = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html>"]
    #[doc(alias = "VkCommandBufferUsageFlagBits")]
    pub struct CommandBufferUsageFlags : u32 {
        const OneTimeSubmit = 1u32 << 0;
        const RenderPassContinue = 1u32 << 1;
        const SimultaneousUse = 1u32 << 2;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryControlFlagBits.html>"]
    #[doc(alias = "VkQueryControlFlagBits")]
    pub struct QueryControlFlags : u32 {
        const Precise = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndexType.html>"]
#[doc(alias = "VkIndexType")]
#[repr(u32)]
pub enum IndexType {
    Uint16 = 0,
    Uint32 = 1,
    NoneKHR = 1000165000,
    Uint8KHR = 1000265000,
}
#[allow(non_upper_case_globals)]
impl IndexType {
    pub const NoneNV: Self = Self::NoneKHR;
    pub const Uint8EXT: Self = Self::Uint8KHR;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilFaceFlagBits.html>"]
    #[doc(alias = "VkStencilFaceFlagBits")]
    pub struct StencilFaceFlags : u32 {
        const Front = 1u32 << 0;
        const Back = 1u32 << 1;
        const FrontAndBack = 0x00000003;
        const rontAndBack = Self::FrontAndBack.bits();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassContents.html>"]
#[doc(alias = "VkSubpassContents")]
#[repr(u32)]
pub enum SubpassContents {
    Inline = 0,
    SecondaryCommandBuffers = 1,
    InlineAndSecondaryCommandBuffersKHR = 1000451000,
}
#[allow(non_upper_case_globals)]
impl SubpassContents {
    pub const InlineAndSecondaryCommandBuffersEXT: Self = Self::InlineAndSecondaryCommandBuffersKHR;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html>"]
    #[doc(alias = "VkSubgroupFeatureFlagBits")]
    pub struct SubgroupFeatureFlags : u32 {
        const Basic = 1u32 << 0;
        const Vote = 1u32 << 1;
        const Arithmetic = 1u32 << 2;
        const Ballot = 1u32 << 3;
        const Shuffle = 1u32 << 4;
        const ShuffleRelative = 1u32 << 5;
        const Clustered = 1u32 << 6;
        const Quad = 1u32 << 7;
        const PartitionedNV = 1u32 << 8;
        const RotateKHR = 1u32 << 9;
        const RotateClusteredKHR = 1u32 << 10;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html>"]
    #[doc(alias = "VkPeerMemoryFeatureFlagBits")]
    pub struct PeerMemoryFeatureFlags : u32 {
        const CopySrc = 1u32 << 0;
        const CopyDst = 1u32 << 1;
        const GenericSrc = 1u32 << 2;
        const GenericDst = 1u32 << 3;
        const CopySrcKHR = Self::CopySrc.bits();
        const CopyDstKHR = Self::CopyDst.bits();
        const GenericSrcKHR = Self::GenericSrc.bits();
        const GenericDstKHR = Self::GenericDst.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBitsKHR.html>"]
#[doc(alias = "VkPeerMemoryFeatureFlagBitsKHR")]
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html>"]
    #[doc(alias = "VkMemoryAllocateFlagBits")]
    pub struct MemoryAllocateFlags : u32 {
        const DeviceMask = 1u32 << 0;
        const DeviceAddress = 1u32 << 1;
        const DeviceAddressCaptureReplay = 1u32 << 2;
        const DeviceMaskKHR = Self::DeviceMask.bits();
        const DeviceAddressKHR = Self::DeviceAddress.bits();
        const DeviceAddressCaptureReplayKHR = Self::DeviceAddressCaptureReplay.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBitsKHR.html>"]
#[doc(alias = "VkMemoryAllocateFlagBitsKHR")]
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DEVICE_GROUP_SIZE.html>"]
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE")]
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehavior.html>"]
#[doc(alias = "VkPointClippingBehavior")]
#[repr(u32)]
pub enum PointClippingBehavior {
    AllClipPlanes = 0,
    UserClipPlanesOnly = 1,
}
#[allow(non_upper_case_globals)]
impl PointClippingBehavior {
    pub const AllClipPlanesKHR: Self = Self::AllClipPlanes;
    pub const UserClipPlanesOnlyKHR: Self = Self::UserClipPlanesOnly;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehaviorKHR.html>"]
#[doc(alias = "VkPointClippingBehaviorKHR")]
pub type PointClippingBehaviorKHR = PointClippingBehavior;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOrigin.html>"]
#[doc(alias = "VkTessellationDomainOrigin")]
#[repr(u32)]
pub enum TessellationDomainOrigin {
    UpperLeft = 0,
    LowerLeft = 1,
}
#[allow(non_upper_case_globals)]
impl TessellationDomainOrigin {
    pub const UpperLeftKHR: Self = Self::UpperLeft;
    pub const LowerLeftKHR: Self = Self::LowerLeft;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOriginKHR.html>"]
#[doc(alias = "VkTessellationDomainOriginKHR")]
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html>"]
    #[doc(alias = "VkDeviceQueueCreateFlagBits")]
    pub struct DeviceQueueCreateFlags : u32 {
        const Protected = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversion.html>"]
#[doc(alias = "VkSamplerYcbcrModelConversion")]
#[repr(u32)]
pub enum SamplerYcbcrModelConversion {
    RgbIdentity = 0,
    YcbcrIdentity = 1,
    Ycbcr709 = 2,
    Ycbcr601 = 3,
    Ycbcr2020 = 4,
}
#[allow(non_upper_case_globals)]
impl SamplerYcbcrModelConversion {
    pub const RgbIdentityKHR: Self = Self::RgbIdentity;
    pub const YcbcrIdentityKHR: Self = Self::YcbcrIdentity;
    pub const Ycbcr709KHR: Self = Self::Ycbcr709;
    pub const Ycbcr601KHR: Self = Self::Ycbcr601;
    pub const Ycbcr2020KHR: Self = Self::Ycbcr2020;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversionKHR.html>"]
#[doc(alias = "VkSamplerYcbcrModelConversionKHR")]
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRange.html>"]
#[doc(alias = "VkSamplerYcbcrRange")]
#[repr(u32)]
pub enum SamplerYcbcrRange {
    ItuFull = 0,
    ItuNarrow = 1,
}
#[allow(non_upper_case_globals)]
impl SamplerYcbcrRange {
    pub const ItuFullKHR: Self = Self::ItuFull;
    pub const ItuNarrowKHR: Self = Self::ItuNarrow;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRangeKHR.html>"]
#[doc(alias = "VkSamplerYcbcrRangeKHR")]
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkChromaLocation.html>"]
#[doc(alias = "VkChromaLocation")]
#[repr(u32)]
pub enum ChromaLocation {
    CositedEven = 0,
    Midpoint = 1,
}
#[allow(non_upper_case_globals)]
impl ChromaLocation {
    pub const CositedEvenKHR: Self = Self::CositedEven;
    pub const MidpointKHR: Self = Self::Midpoint;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkChromaLocationKHR.html>"]
#[doc(alias = "VkChromaLocationKHR")]
pub type ChromaLocationKHR = ChromaLocation;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateType.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateType")]
#[repr(u32)]
pub enum DescriptorUpdateTemplateType {
    DescriptorSet = 0,
    PushDescriptorsKHR = 1,
}
#[allow(non_upper_case_globals)]
impl DescriptorUpdateTemplateType {
    pub const DescriptorSetKHR: Self = Self::DescriptorSet;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateTypeKHR.html>"]
#[doc(alias = "VkDescriptorUpdateTemplateTypeKHR")]
pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE.html>"]
#[doc(alias = "VK_LUID_SIZE")]
pub const LUID_SIZE: u32 = 8;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html>"]
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
    pub struct ExternalMemoryHandleTypeFlags : u32 {
        const OpaqueFd = 1u32 << 0;
        const OpaqueWin32 = 1u32 << 1;
        const OpaqueWin32Kmt = 1u32 << 2;
        const D3D11Texture = 1u32 << 3;
        const D3D11TextureKmt = 1u32 << 4;
        const D3D12Heap = 1u32 << 5;
        const D3D12Resource = 1u32 << 6;
        const OpaqueFdKHR = Self::OpaqueFd.bits();
        const OpaqueWin32KHR = Self::OpaqueWin32.bits();
        const OpaqueWin32KmtKHR = Self::OpaqueWin32Kmt.bits();
        const D3D11TextureKHR = Self::D3D11Texture.bits();
        const D3D11TextureKmtKHR = Self::D3D11TextureKmt.bits();
        const D3D12HeapKHR = Self::D3D12Heap.bits();
        const D3D12ResourceKHR = Self::D3D12Resource.bits();
        const DmaBufEXT = 1u32 << 9;
        const AndroidHardwareBufferANDROID = 1u32 << 10;
        const HostAllocationEXT = 1u32 << 7;
        const HostMappedForeignMemoryEXT = 1u32 << 8;
        const ZirconVmoFUCHSIA = 1u32 << 11;
        const RdmaAddressNV = 1u32 << 12;
        const ScreenBufferQNX = 1u32 << 14;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsKHR.html>"]
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsKHR")]
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html>"]
    #[doc(alias = "VkExternalMemoryFeatureFlagBits")]
    pub struct ExternalMemoryFeatureFlags : u32 {
        const DedicatedOnly = 1u32 << 0;
        const Exportable = 1u32 << 1;
        const Importable = 1u32 << 2;
        const DedicatedOnlyKHR = Self::DedicatedOnly.bits();
        const ExportableKHR = Self::Exportable.bits();
        const ImportableKHR = Self::Importable.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsKHR.html>"]
#[doc(alias = "VkExternalMemoryFeatureFlagBitsKHR")]
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_EXTERNAL.html>"]
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL")]
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1u32;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html>"]
    #[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
    pub struct ExternalFenceHandleTypeFlags : u32 {
        const OpaqueFd = 1u32 << 0;
        const OpaqueWin32 = 1u32 << 1;
        const OpaqueWin32Kmt = 1u32 << 2;
        const SyncFd = 1u32 << 3;
        const OpaqueFdKHR = Self::OpaqueFd.bits();
        const OpaqueWin32KHR = Self::OpaqueWin32.bits();
        const OpaqueWin32KmtKHR = Self::OpaqueWin32Kmt.bits();
        const SyncFdKHR = Self::SyncFd.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBitsKHR.html>"]
#[doc(alias = "VkExternalFenceHandleTypeFlagBitsKHR")]
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html>"]
    #[doc(alias = "VkExternalFenceFeatureFlagBits")]
    pub struct ExternalFenceFeatureFlags : u32 {
        const Exportable = 1u32 << 0;
        const Importable = 1u32 << 1;
        const ExportableKHR = Self::Exportable.bits();
        const ImportableKHR = Self::Importable.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBitsKHR.html>"]
#[doc(alias = "VkExternalFenceFeatureFlagBitsKHR")]
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html>"]
    #[doc(alias = "VkFenceImportFlagBits")]
    pub struct FenceImportFlags : u32 {
        const Temporary = 1u32 << 0;
        const TemporaryKHR = Self::Temporary.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBitsKHR.html>"]
#[doc(alias = "VkFenceImportFlagBitsKHR")]
pub type FenceImportFlagsKHR = FenceImportFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html>"]
    #[doc(alias = "VkSemaphoreImportFlagBits")]
    pub struct SemaphoreImportFlags : u32 {
        const Temporary = 1u32 << 0;
        const TemporaryKHR = Self::Temporary.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBitsKHR.html>"]
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html>"]
    #[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
    pub struct ExternalSemaphoreHandleTypeFlags : u32 {
        const OpaqueFd = 1u32 << 0;
        const OpaqueWin32 = 1u32 << 1;
        const OpaqueWin32Kmt = 1u32 << 2;
        const D3D12Fence = 1u32 << 3;
        const D3D11Fence = Self::D3D12Fence.bits();
        const SyncFd = 1u32 << 4;
        const OpaqueFdKHR = Self::OpaqueFd.bits();
        const OpaqueWin32KHR = Self::OpaqueWin32.bits();
        const OpaqueWin32KmtKHR = Self::OpaqueWin32Kmt.bits();
        const D3D12FenceKHR = Self::D3D12Fence.bits();
        const SyncFdKHR = Self::SyncFd.bits();
        const ZirconEventFUCHSIA = 1u32 << 7;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBitsKHR.html>"]
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBitsKHR")]
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html>"]
    #[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
    pub struct ExternalSemaphoreFeatureFlags : u32 {
        const Exportable = 1u32 << 0;
        const Importable = 1u32 << 1;
        const ExportableKHR = Self::Exportable.bits();
        const ImportableKHR = Self::Importable.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBitsKHR.html>"]
#[doc(alias = "VkExternalSemaphoreFeatureFlagBitsKHR")]
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_NAME_SIZE.html>"]
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE")]
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE.html>"]
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE")]
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDriverId.html>"]
#[doc(alias = "VkDriverId")]
#[repr(u32)]
pub enum DriverId {
    AmdProprietary = 1,
    AmdOpenSource = 2,
    MesaRadv = 3,
    NvidiaProprietary = 4,
    IntelProprietaryWindows = 5,
    IntelOpenSourceMESA = 6,
    ImaginationProprietary = 7,
    QualcommProprietary = 8,
    ArmProprietary = 9,
    GoogleSwiftshader = 10,
    GgpProprietary = 11,
    BroadcomProprietary = 12,
    MesaLlvmpipe = 13,
    Moltenvk = 14,
    CoreaviProprietary = 15,
    JuiceProprietary = 16,
    VerisiliconProprietary = 17,
    MesaTurnip = 18,
    MesaV3Dv = 19,
    MesaPanvk = 20,
    SamsungProprietary = 21,
    MesaVenus = 22,
    MesaDozen = 23,
    MesaNvk = 24,
    ImaginationOpenSourceMESA = 25,
    MesaHoneykrisp = 26,
    Reserved27 = 27,
}
#[allow(non_upper_case_globals)]
impl DriverId {
    pub const AmdProprietaryKHR: Self = Self::AmdProprietary;
    pub const AmdOpenSourceKHR: Self = Self::AmdOpenSource;
    pub const MesaRadvKHR: Self = Self::MesaRadv;
    pub const NvidiaProprietaryKHR: Self = Self::NvidiaProprietary;
    pub const IntelProprietaryWindowsKHR: Self = Self::IntelProprietaryWindows;
    pub const IntelOpenSourceMesaKHR: Self = Self::IntelOpenSourceMESA;
    pub const ImaginationProprietaryKHR: Self = Self::ImaginationProprietary;
    pub const QualcommProprietaryKHR: Self = Self::QualcommProprietary;
    pub const ArmProprietaryKHR: Self = Self::ArmProprietary;
    pub const GoogleSwiftshaderKHR: Self = Self::GoogleSwiftshader;
    pub const GgpProprietaryKHR: Self = Self::GgpProprietary;
    pub const BroadcomProprietaryKHR: Self = Self::BroadcomProprietary;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDriverIdKHR.html>"]
#[doc(alias = "VkDriverIdKHR")]
pub type DriverIdKHR = DriverId;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependence.html>"]
#[doc(alias = "VkShaderFloatControlsIndependence")]
#[repr(u32)]
pub enum ShaderFloatControlsIndependence {
    Controls32BitOnly = 0,
    All = 1,
    None = 2,
}
#[allow(non_upper_case_globals)]
impl ShaderFloatControlsIndependence {
    pub const Controls32BitOnlyKHR: Self = Self::Controls32BitOnly;
    pub const AllKHR: Self = Self::All;
    pub const NoneKHR: Self = Self::None;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependenceKHR.html>"]
#[doc(alias = "VkShaderFloatControlsIndependenceKHR")]
pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html>"]
    #[doc(alias = "VkDescriptorBindingFlagBits")]
    pub struct DescriptorBindingFlags : u32 {
        const UpdateAfterBind = 1u32 << 0;
        const UpdateUnusedWhilePending = 1u32 << 1;
        const PartiallyBound = 1u32 << 2;
        const VariableDescriptorCount = 1u32 << 3;
        const UpdateAfterBindEXT = Self::UpdateAfterBind.bits();
        const UpdateUnusedWhilePendingEXT = Self::UpdateUnusedWhilePending.bits();
        const PartiallyBoundEXT = Self::PartiallyBound.bits();
        const VariableDescriptorCountEXT = Self::VariableDescriptorCount.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBitsEXT.html>"]
#[doc(alias = "VkDescriptorBindingFlagBitsEXT")]
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html>"]
    #[doc(alias = "VkResolveModeFlagBits")]
    pub struct ResolveModeFlags : u32 {
        const None = 0;
        const SampleZero = 1u32 << 0;
        const Average = 1u32 << 1;
        const Min = 1u32 << 2;
        const Max = 1u32 << 3;
        const NoneKHR = Self::None.bits();
        const SampleZeroKHR = Self::SampleZero.bits();
        const AverageKHR = Self::Average.bits();
        const MinKHR = Self::Min.bits();
        const MaxKHR = Self::Max.bits();
        const ExternalFormatDownsampleANDROID = 1u32 << 4;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBitsKHR.html>"]
#[doc(alias = "VkResolveModeFlagBitsKHR")]
pub type ResolveModeFlagsKHR = ResolveModeFlags;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionMode.html>"]
#[doc(alias = "VkSamplerReductionMode")]
#[repr(u32)]
pub enum SamplerReductionMode {
    WeightedAverage = 0,
    Min = 1,
    Max = 2,
    WeightedAverageRangeclampQCOM = 1000521000,
}
#[allow(non_upper_case_globals)]
impl SamplerReductionMode {
    pub const WeightedAverageEXT: Self = Self::WeightedAverage;
    pub const MinEXT: Self = Self::Min;
    pub const MaxEXT: Self = Self::Max;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionModeEXT.html>"]
#[doc(alias = "VkSamplerReductionModeEXT")]
pub type SamplerReductionModeEXT = SamplerReductionMode;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreType.html>"]
#[doc(alias = "VkSemaphoreType")]
#[repr(u32)]
pub enum SemaphoreType {
    Binary = 0,
    Timeline = 1,
}
#[allow(non_upper_case_globals)]
impl SemaphoreType {
    pub const BinaryKHR: Self = Self::Binary;
    pub const TimelineKHR: Self = Self::Timeline;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeKHR.html>"]
#[doc(alias = "VkSemaphoreTypeKHR")]
pub type SemaphoreTypeKHR = SemaphoreType;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html>"]
    #[doc(alias = "VkSemaphoreWaitFlagBits")]
    pub struct SemaphoreWaitFlags : u32 {
        const Any = 1u32 << 0;
        const AnyKHR = Self::Any.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBitsKHR.html>"]
#[doc(alias = "VkSemaphoreWaitFlagBitsKHR")]
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html>"]
    #[doc(alias = "VkPipelineCreationFeedbackFlagBits")]
    pub struct PipelineCreationFeedbackFlags : u32 {
        const Valid = 1u32 << 0;
        const ValidEXT = Self::Valid.bits();
        const ApplicationPipelineCacheHit = 1u32 << 1;
        const ApplicationPipelineCacheHitEXT = Self::ApplicationPipelineCacheHit.bits();
        const BasePipelineAcceleration = 1u32 << 2;
        const BasePipelineAccelerationEXT = Self::BasePipelineAcceleration.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBitsEXT.html>"]
#[doc(alias = "VkPipelineCreationFeedbackFlagBitsEXT")]
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBits.html>"]
    #[doc(alias = "VkToolPurposeFlagBits")]
    pub struct ToolPurposeFlags : u32 {
        const Validation = 1u32 << 0;
        const ValidationEXT = Self::Validation.bits();
        const Profiling = 1u32 << 1;
        const ProfilingEXT = Self::Profiling.bits();
        const Tracing = 1u32 << 2;
        const TracingEXT = Self::Tracing.bits();
        const AdditionalFeatures = 1u32 << 3;
        const AdditionalFeaturesEXT = Self::AdditionalFeatures.bits();
        const ModifyingFeatures = 1u32 << 4;
        const ModifyingFeaturesEXT = Self::ModifyingFeatures.bits();
        const DebugReportingEXT = 1u32 << 5;
        const DebugMarkersEXT = 1u32 << 6;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkToolPurposeFlagBitsEXT.html>"]
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html>"]
    #[doc(alias = "VkPipelineStageFlagBits2")]
    pub struct PipelineStageFlags2 : u64 {
        const None = 0;
        const NoneKHR = Self::None.bits();
        const TopOfPipe = 1u64 << 0;
        const TopOfPipeKHR = Self::TopOfPipe.bits();
        const DrawIndirect = 1u64 << 1;
        const DrawIndirectKHR = Self::DrawIndirect.bits();
        const VertexInput = 1u64 << 2;
        const VertexInputKHR = Self::VertexInput.bits();
        const VertexShader = 1u64 << 3;
        const VertexShaderKHR = Self::VertexShader.bits();
        const TessellationControlShader = 1u64 << 4;
        const TessellationControlShaderKHR = Self::TessellationControlShader.bits();
        const TessellationEvaluationShader = 1u64 << 5;
        const TessellationEvaluationShaderKHR = Self::TessellationEvaluationShader.bits();
        const GeometryShader = 1u64 << 6;
        const GeometryShaderKHR = Self::GeometryShader.bits();
        const FragmentShader = 1u64 << 7;
        const FragmentShaderKHR = Self::FragmentShader.bits();
        const EarlyFragmentTests = 1u64 << 8;
        const EarlyFragmentTestsKHR = Self::EarlyFragmentTests.bits();
        const LateFragmentTests = 1u64 << 9;
        const LateFragmentTestsKHR = Self::LateFragmentTests.bits();
        const ColorAttachmentOutput = 1u64 << 10;
        const ColorAttachmentOutputKHR = Self::ColorAttachmentOutput.bits();
        const ComputeShader = 1u64 << 11;
        const ComputeShaderKHR = Self::ComputeShader.bits();
        const AllTransfer = 1u64 << 12;
        const AllTransferKHR = Self::AllTransfer.bits();
        const Transfer = Self::AllTransferKHR.bits();
        const TransferKHR = Self::AllTransfer.bits();
        const BottomOfPipe = 1u64 << 13;
        const BottomOfPipeKHR = Self::BottomOfPipe.bits();
        const Host = 1u64 << 14;
        const HostKHR = Self::Host.bits();
        const AllGraphics = 1u64 << 15;
        const AllGraphicsKHR = Self::AllGraphics.bits();
        const AllCommands = 1u64 << 16;
        const AllCommandsKHR = Self::AllCommands.bits();
        const Copy = 1u64 << 32;
        const CopyKHR = Self::Copy.bits();
        const Resolve = 1u64 << 33;
        const ResolveKHR = Self::Resolve.bits();
        const Blit = 1u64 << 34;
        const BlitKHR = Self::Blit.bits();
        const Clear = 1u64 << 35;
        const ClearKHR = Self::Clear.bits();
        const IndexInput = 1u64 << 36;
        const IndexInputKHR = Self::IndexInput.bits();
        const VertexAttributeInput = 1u64 << 37;
        const VertexAttributeInputKHR = Self::VertexAttributeInput.bits();
        const PreRasterizationShaders = 1u64 << 38;
        const PreRasterizationShadersKHR = Self::PreRasterizationShaders.bits();
        const TransformFeedbackEXT = 1u64 << 24;
        const ConditionalRenderingEXT = 1u64 << 18;
        const CommandPreprocessNV = 1u64 << 17;
        const FragmentShadingRateAttachmentKHR = 1u64 << 22;
        const ShadingRateImageNV = Self::FragmentShadingRateAttachmentKHR.bits();
        const AccelerationStructureBuildKHR = 1u64 << 25;
        const RayTracingShaderKHR = 1u64 << 21;
        const RayTracingShaderNV = Self::RayTracingShaderKHR.bits();
        const AccelerationStructureBuildNV = Self::AccelerationStructureBuildKHR.bits();
        const FragmentDensityProcessEXT = 1u64 << 23;
        const TaskShaderNV = Self::TaskShaderEXT.bits();
        const MeshShaderNV = Self::MeshShaderEXT.bits();
        const TaskShaderEXT = 1u64 << 19;
        const MeshShaderEXT = 1u64 << 20;
        const SubpassShaderHUAWEI = 1u64 << 39;
        const SubpassShadingHUAWEI = Self::SubpassShaderHUAWEI.bits();
        const InvocationMaskHUAWEI = 1u64 << 40;
        const AccelerationStructureCopyKHR = 1u64 << 28;
        const MicromapBuildEXT = 1u64 << 30;
        const ClusterCullingShaderHUAWEI = 1u64 << 41;
        const OpticalFlowNV = 1u64 << 29;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2KHR.html>"]
#[doc(alias = "VkPipelineStageFlagBits2KHR")]
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html>"]
    #[doc(alias = "VkAccessFlagBits2")]
    pub struct AccessFlags2 : u64 {
        const None = 0;
        const NoneKHR = Self::None.bits();
        const IndirectCommandRead = 1u64 << 0;
        const IndirectCommandReadKHR = Self::IndirectCommandRead.bits();
        const IndexRead = 1u64 << 1;
        const IndexReadKHR = Self::IndexRead.bits();
        const VertexAttributeRead = 1u64 << 2;
        const VertexAttributeReadKHR = Self::VertexAttributeRead.bits();
        const UniformRead = 1u64 << 3;
        const UniformReadKHR = Self::UniformRead.bits();
        const InputAttachmentRead = 1u64 << 4;
        const InputAttachmentReadKHR = Self::InputAttachmentRead.bits();
        const ShaderRead = 1u64 << 5;
        const ShaderReadKHR = Self::ShaderRead.bits();
        const ShaderWrite = 1u64 << 6;
        const ShaderWriteKHR = Self::ShaderWrite.bits();
        const ColorAttachmentRead = 1u64 << 7;
        const ColorAttachmentReadKHR = Self::ColorAttachmentRead.bits();
        const ColorAttachmentWrite = 1u64 << 8;
        const ColorAttachmentWriteKHR = Self::ColorAttachmentWrite.bits();
        const DepthStencilAttachmentRead = 1u64 << 9;
        const DepthStencilAttachmentReadKHR = Self::DepthStencilAttachmentRead.bits();
        const DepthStencilAttachmentWrite = 1u64 << 10;
        const DepthStencilAttachmentWriteKHR = Self::DepthStencilAttachmentWrite.bits();
        const TransferRead = 1u64 << 11;
        const TransferReadKHR = Self::TransferRead.bits();
        const TransferWrite = 1u64 << 12;
        const TransferWriteKHR = Self::TransferWrite.bits();
        const HostRead = 1u64 << 13;
        const HostReadKHR = Self::HostRead.bits();
        const HostWrite = 1u64 << 14;
        const HostWriteKHR = Self::HostWrite.bits();
        const MemoryRead = 1u64 << 15;
        const MemoryReadKHR = Self::MemoryRead.bits();
        const MemoryWrite = 1u64 << 16;
        const MemoryWriteKHR = Self::MemoryWrite.bits();
        const ShaderSampledRead = 1u64 << 32;
        const ShaderSampledReadKHR = Self::ShaderSampledRead.bits();
        const ShaderStorageRead = 1u64 << 33;
        const ShaderStorageReadKHR = Self::ShaderStorageRead.bits();
        const ShaderStorageWrite = 1u64 << 34;
        const ShaderStorageWriteKHR = Self::ShaderStorageWrite.bits();
        const TransformFeedbackWriteEXT = 1u64 << 25;
        const TransformFeedbackCounterReadEXT = 1u64 << 26;
        const TransformFeedbackCounterWriteEXT = 1u64 << 27;
        const ConditionalRenderingReadEXT = 1u64 << 20;
        const CommandPreprocessReadNV = 1u64 << 17;
        const CommandPreprocessWriteNV = 1u64 << 18;
        const FragmentShadingRateAttachmentReadKHR = 1u64 << 23;
        const ShadingRateImageReadNV = Self::FragmentShadingRateAttachmentReadKHR.bits();
        const AccelerationStructureReadKHR = 1u64 << 21;
        const AccelerationStructureWriteKHR = 1u64 << 22;
        const AccelerationStructureReadNV = Self::AccelerationStructureReadKHR.bits();
        const AccelerationStructureWriteNV = Self::AccelerationStructureWriteKHR.bits();
        const FragmentDensityMapReadEXT = 1u64 << 24;
        const ColorAttachmentReadNoncoherentEXT = 1u64 << 19;
        const DescriptorBufferReadEXT = 1u64 << 41;
        const InvocationMaskReadHUAWEI = 1u64 << 39;
        const ShaderBindingTableReadKHR = 1u64 << 40;
        const MicromapReadEXT = 1u64 << 44;
        const MicromapWriteEXT = 1u64 << 45;
        const OpticalFlowReadNV = 1u64 << 42;
        const OpticalFlowWriteNV = 1u64 << 43;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2KHR.html>"]
#[doc(alias = "VkAccessFlagBits2KHR")]
pub type AccessFlags2KHR = AccessFlags2;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBits.html>"]
    #[doc(alias = "VkSubmitFlagBits")]
    pub struct SubmitFlags : u32 {
        const Protected = 1u32 << 0;
        const ProtectedKHR = Self::Protected.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitFlagBitsKHR.html>"]
#[doc(alias = "VkSubmitFlagBitsKHR")]
pub type SubmitFlagsKHR = SubmitFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html>"]
    #[doc(alias = "VkRenderingFlagBits")]
    pub struct RenderingFlags : u32 {
        const ContentsSecondaryCommandBuffers = 1u32 << 0;
        const ContentsSecondaryCommandBuffersKHR = Self::ContentsSecondaryCommandBuffers.bits();
        const Suspending = 1u32 << 1;
        const SuspendingKHR = Self::Suspending.bits();
        const Resuming = 1u32 << 2;
        const ResumingKHR = Self::Resuming.bits();
        const ContentsInlineEXT = Self::ContentsInlineKHR.bits();
        const EnableLegacyDitheringEXT = 1u32 << 3;
        const ContentsInlineKHR = 1u32 << 4;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBitsKHR.html>"]
#[doc(alias = "VkRenderingFlagBitsKHR")]
pub type RenderingFlagsKHR = RenderingFlags;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html>"]
    #[doc(alias = "VkFormatFeatureFlagBits2")]
    pub struct FormatFeatureFlags2 : u64 {
        const SampledImage = 1u64 << 0;
        const SampledImageKHR = Self::SampledImage.bits();
        const StorageImage = 1u64 << 1;
        const StorageImageKHR = Self::StorageImage.bits();
        const StorageImageAtomic = 1u64 << 2;
        const StorageImageAtomicKHR = Self::StorageImageAtomic.bits();
        const UniformTexelBuffer = 1u64 << 3;
        const UniformTexelBufferKHR = Self::UniformTexelBuffer.bits();
        const StorageTexelBuffer = 1u64 << 4;
        const StorageTexelBufferKHR = Self::StorageTexelBuffer.bits();
        const StorageTexelBufferAtomic = 1u64 << 5;
        const StorageTexelBufferAtomicKHR = Self::StorageTexelBufferAtomic.bits();
        const VertexBuffer = 1u64 << 6;
        const VertexBufferKHR = Self::VertexBuffer.bits();
        const ColorAttachment = 1u64 << 7;
        const ColorAttachmentKHR = Self::ColorAttachment.bits();
        const ColorAttachmentBlend = 1u64 << 8;
        const ColorAttachmentBlendKHR = Self::ColorAttachmentBlend.bits();
        const DepthStencilAttachment = 1u64 << 9;
        const DepthStencilAttachmentKHR = Self::DepthStencilAttachment.bits();
        const BlitSrc = 1u64 << 10;
        const BlitSrcKHR = Self::BlitSrc.bits();
        const BlitDst = 1u64 << 11;
        const BlitDstKHR = Self::BlitDst.bits();
        const SampledImageFilterLinear = 1u64 << 12;
        const SampledImageFilterLinearKHR = Self::SampledImageFilterLinear.bits();
        const SampledImageFilterCubic = 1u64 << 13;
        const SampledImageFilterCubicEXT = Self::SampledImageFilterCubic.bits();
        const TransferSrc = 1u64 << 14;
        const TransferSrcKHR = Self::TransferSrc.bits();
        const TransferDst = 1u64 << 15;
        const TransferDstKHR = Self::TransferDst.bits();
        const SampledImageFilterMinmax = 1u64 << 16;
        const SampledImageFilterMinmaxKHR = Self::SampledImageFilterMinmax.bits();
        const MidpointChromaSamples = 1u64 << 17;
        const MidpointChromaSamplesKHR = Self::MidpointChromaSamples.bits();
        const SampledImageYcbcrConversionLinearFilter = 1u64 << 18;
        const SampledImageYcbcrConversionLinearFilterKHR = Self::SampledImageYcbcrConversionLinearFilter.bits();
        const SampledImageYcbcrConversionSeparateReconstructionFilter = 1u64 << 19;
        const SampledImageYcbcrConversionSeparateReconstructionFilterKHR = Self::SampledImageYcbcrConversionSeparateReconstructionFilter.bits();
        const SampledImageYcbcrConversionChromaReconstructionExplicit = 1u64 << 20;
        const SampledImageYcbcrConversionChromaReconstructionExplicitKHR = Self::SampledImageYcbcrConversionChromaReconstructionExplicit.bits();
        const SampledImageYcbcrConversionChromaReconstructionExplicitForceable = 1u64 << 21;
        const SampledImageYcbcrConversionChromaReconstructionExplicitForceableKHR = Self::SampledImageYcbcrConversionChromaReconstructionExplicitForceable.bits();
        const Disjoint = 1u64 << 22;
        const DisjointKHR = Self::Disjoint.bits();
        const CositedChromaSamples = 1u64 << 23;
        const CositedChromaSamplesKHR = Self::CositedChromaSamples.bits();
        const StorageReadWithoutFormat = 1u64 << 31;
        const StorageReadWithoutFormatKHR = Self::StorageReadWithoutFormat.bits();
        const StorageWriteWithoutFormat = 1u64 << 32;
        const StorageWriteWithoutFormatKHR = Self::StorageWriteWithoutFormat.bits();
        const SampledImageDepthComparison = 1u64 << 33;
        const SampledImageDepthComparisonKHR = Self::SampledImageDepthComparison.bits();
        const AccelerationStructureVertexBufferKHR = 1u64 << 29;
        const FragmentDensityMapEXT = 1u64 << 24;
        const FragmentShadingRateAttachmentKHR = 1u64 << 30;
        const HostImageTransferEXT = 1u64 << 46;
        const LinearColorAttachmentNV = 1u64 << 38;
        const WeightImageQCOM = 1u64 << 34;
        const WeightSampledImageQCOM = 1u64 << 35;
        const BlockMatchingQCOM = 1u64 << 36;
        const BoxFilterSampledQCOM = 1u64 << 37;
        const OpticalFlowImageNV = 1u64 << 40;
        const OpticalFlowVectorNV = 1u64 << 41;
        const OpticalFlowCostNV = 1u64 << 42;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2KHR.html>"]
#[doc(alias = "VkFormatFeatureFlagBits2KHR")]
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html>"]
    #[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
    pub struct SurfaceTransformFlagsKHR : u32 {
        const Identity = 1u32 << 0;
        const Rotate90 = 1u32 << 1;
        const Rotate180 = 1u32 << 2;
        const Rotate270 = 1u32 << 3;
        const HorizontalMirror = 1u32 << 4;
        const HorizontalMirrorRotate90 = 1u32 << 5;
        const HorizontalMirrorRotate180 = 1u32 << 6;
        const HorizontalMirrorRotate270 = 1u32 << 7;
        const Inherit = 1u32 << 8;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html>"]
#[doc(alias = "VkPresentModeKHR")]
#[repr(u32)]
pub enum PresentModeKHR {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    SharedDemandRefresh = 1000111000,
    SharedContinuousRefresh = 1000111001,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html>"]
#[doc(alias = "VkColorSpaceKHR")]
#[repr(u32)]
pub enum ColorSpaceKHR {
    SrgbNonlinear = 0,
    DisplayP3NonlinearEXT = 1000104001,
    ExtendedSrgbLinearEXT = 1000104002,
    DisplayP3LinearEXT = 1000104003,
    DciP3NonlinearEXT = 1000104004,
    Bt709LinearEXT = 1000104005,
    Bt709NonlinearEXT = 1000104006,
    Bt2020LinearEXT = 1000104007,
    Hdr10St2084EXT = 1000104008,
    DolbyvisionEXT = 1000104009,
    Hdr10HlgEXT = 1000104010,
    AdobergbLinearEXT = 1000104011,
    AdobergbNonlinearEXT = 1000104012,
    PassThroughEXT = 1000104013,
    ExtendedSrgbNonlinearEXT = 1000104014,
    DisplayNativeAMD = 1000213000,
}
#[allow(non_upper_case_globals)]
impl ColorSpaceKHR {
    pub const spaceSrgbNonlinear: Self = Self::SrgbNonlinear;
    pub const DciP3LinearEXT: Self = Self::DisplayP3LinearEXT;
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html>"]
    #[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
    pub struct CompositeAlphaFlagsKHR : u32 {
        const Opaque = 1u32 << 0;
        const PreMultiplied = 1u32 << 1;
        const PostMultiplied = 1u32 << 2;
        const Inherit = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html>"]
    #[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
    pub struct SwapchainCreateFlagsKHR : u32 {
        const SplitInstanceBindRegions = 1u32 << 0;
        const Protected = 1u32 << 1;
        const MutableFormat = 1u32 << 2;
        const DeferredMemoryAllocationEXT = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html>"]
    #[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
    pub struct DeviceGroupPresentModeFlagsKHR : u32 {
        const Local = 1u32 << 0;
        const Remote = 1u32 << 1;
        const Sum = 1u32 << 2;
        const LocalMultiDevice = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html>"]
    #[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
    pub struct DisplayPlaneAlphaFlagsKHR : u32 {
        const Opaque = 1u32 << 0;
        const Global = 1u32 << 1;
        const PerPixel = 1u32 << 2;
        const PerPixelPremultiplied = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportFlagBitsEXT.html>"]
    #[doc(alias = "VkDebugReportFlagBitsEXT")]
    pub struct DebugReportFlagsEXT : u32 {
        const Information = 1u32 << 0;
        const Warning = 1u32 << 1;
        const PerformanceWarning = 1u32 << 2;
        const Error = 1u32 << 3;
        const Debug = 1u32 << 4;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportObjectTypeEXT.html>"]
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[repr(u32)]
pub enum DebugReportObjectTypeEXT {
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,
    SurfaceKHR = 26,
    SwapchainKHR = 27,
    DebugReportCallbackExt = 28,
    DisplayKHR = 29,
    DisplayModeKHR = 30,
    ValidationCacheExt = 33,
    SamplerYcbcrConversion = 1000156000,
    DescriptorUpdateTemplate = 1000085000,
    CuModuleNVX = 1000029000,
    CuFunctionNVX = 1000029001,
    AccelerationStructureKHR = 1000150000,
    AccelerationStructureNV = 1000165000,
    CudaModuleNV = 1000307000,
    CudaFunctionNV = 1000307001,
    BufferCollectionFUCHSIA = 1000366000,
}
#[allow(non_upper_case_globals)]
impl DebugReportObjectTypeEXT {
    pub const DebugReport: Self = Self::DebugReportCallbackExt;
    pub const ValidationCache: Self = Self::ValidationCacheExt;
    pub const DescriptorUpdateTemplateKHR: Self = Self::DescriptorUpdateTemplate;
    pub const SamplerYcbcrConversionKHR: Self = Self::SamplerYcbcrConversion;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRasterizationOrderAMD.html>"]
#[doc(alias = "VkRasterizationOrderAMD")]
#[repr(u32)]
pub enum RasterizationOrderAMD {
    Strict = 0,
    Relaxed = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html>"]
#[doc(alias = "VkShaderInfoTypeAMD")]
#[repr(u32)]
pub enum ShaderInfoTypeAMD {
    Statistics = 0,
    Binary = 1,
    Disassembly = 2,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html>"]
    #[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
    pub struct ExternalMemoryHandleTypeFlagsNV : u32 {
        const OpaqueWin32 = 1u32 << 0;
        const OpaqueWin32Kmt = 1u32 << 1;
        const D3D11Image = 1u32 << 2;
        const D3D11ImageKmt = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html>"]
    #[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
    pub struct ExternalMemoryFeatureFlagsNV : u32 {
        const DedicatedOnly = 1u32 << 0;
        const Exportable = 1u32 << 1;
        const Importable = 1u32 << 2;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCheckEXT.html>"]
#[doc(alias = "VkValidationCheckEXT")]
#[repr(u32)]
pub enum ValidationCheckEXT {
    All = 0,
    Shaders = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessBufferBehaviorEXT.html>"]
#[doc(alias = "VkPipelineRobustnessBufferBehaviorEXT")]
#[repr(u32)]
pub enum PipelineRobustnessBufferBehaviorEXT {
    DeviceDefault = 0,
    Disabled = 1,
    RobustBufferAccess = 2,
    RobustBufferAccess2 = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRobustnessImageBehaviorEXT.html>"]
#[doc(alias = "VkPipelineRobustnessImageBehaviorEXT")]
#[repr(u32)]
pub enum PipelineRobustnessImageBehaviorEXT {
    DeviceDefault = 0,
    Disabled = 1,
    RobustImageAccess = 2,
    RobustImageAccess2 = 3,
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DEVICE_GROUP_SIZE_KHR.html>"]
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE_KHR")]
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = MAX_DEVICE_GROUP_SIZE;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE_KHR.html>"]
#[doc(alias = "VK_LUID_SIZE_KHR")]
pub const LUID_SIZE_KHR: u32 = LUID_SIZE;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_EXTERNAL_KHR.html>"]
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL_KHR")]
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = QUEUE_FAMILY_EXTERNAL;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html>"]
    #[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
    pub struct ConditionalRenderingFlagsEXT : u32 {
        const Inverted = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html>"]
    #[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
    pub struct SurfaceCounterFlagsEXT : u32 {
        const Vblank = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerStateEXT.html>"]
#[doc(alias = "VkDisplayPowerStateEXT")]
#[repr(u32)]
pub enum DisplayPowerStateEXT {
    Off = 0,
    Suspend = 1,
    On = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceEventTypeEXT.html>"]
#[doc(alias = "VkDeviceEventTypeEXT")]
#[repr(u32)]
pub enum DeviceEventTypeEXT {
    DisplayHotplug = 0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventTypeEXT.html>"]
#[doc(alias = "VkDisplayEventTypeEXT")]
#[repr(u32)]
pub enum DisplayEventTypeEXT {
    FirstPixelOut = 0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportCoordinateSwizzleNV.html>"]
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[repr(u32)]
pub enum ViewportCoordinateSwizzleNV {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
    PositiveW = 6,
    NegativeW = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html>"]
#[doc(alias = "VkDiscardRectangleModeEXT")]
#[repr(u32)]
pub enum DiscardRectangleModeEXT {
    Inclusive = 0,
    Exclusive = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html>"]
#[doc(alias = "VkConservativeRasterizationModeEXT")]
#[repr(u32)]
pub enum ConservativeRasterizationModeEXT {
    Disabled = 0,
    Overestimate = 1,
    Underestimate = 2,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html>"]
    #[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
    pub struct PerformanceCounterDescriptionFlagsKHR : u32 {
        const PerformanceImpacting = 1u32 << 0;
        const ConcurrentlyImpacted = 1u32 << 1;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html>"]
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[repr(u32)]
pub enum PerformanceCounterScopeKHR {
    CommandBuffer = 0,
    RenderPass = 1,
    Command = 2,
}
#[allow(non_upper_case_globals)]
impl PerformanceCounterScopeKHR {
    pub const QueryScopeCommandBuffer: Self = Self::CommandBuffer;
    pub const QueryScopeRenderPass: Self = Self::RenderPass;
    pub const QueryScopeCommand: Self = Self::Command;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html>"]
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[repr(u32)]
pub enum PerformanceCounterStorageKHR {
    Int32 = 0,
    Int64 = 1,
    Uint32 = 2,
    Uint64 = 3,
    Float32 = 4,
    Float64 = 5,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html>"]
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[repr(u32)]
pub enum PerformanceCounterUnitKHR {
    Generic = 0,
    Percentage = 1,
    Nanoseconds = 2,
    Bytes = 3,
    BytesPerSecond = 4,
    Kelvin = 5,
    Watts = 6,
    Volts = 7,
    Amps = 8,
    Hertz = 9,
    Cycles = 10,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html>"]
    #[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
    pub struct AcquireProfilingLockFlagsKHR : u32 {
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_FOREIGN_EXT.html>"]
#[doc(alias = "VK_QUEUE_FAMILY_FOREIGN_EXT")]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2u32;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html>"]
    #[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
    pub struct DebugUtilsMessageSeverityFlagsEXT : u32 {
        const Verbose = 1u32 << 0;
        const Info = 1u32 << 4;
        const Warning = 1u32 << 8;
        const Error = 1u32 << 12;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html>"]
    #[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
    pub struct DebugUtilsMessageTypeFlagsEXT : u32 {
        const General = 1u32 << 0;
        const Validation = 1u32 << 1;
        const Performance = 1u32 << 2;
        const DeviceAddressBinding = 1u32 << 3;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_INDEX_UNUSED_AMDX.html>"]
#[doc(alias = "VK_SHADER_INDEX_UNUSED_AMDX")]
pub const SHADER_INDEX_UNUSED_AMDX: u32 = !0u32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html>"]
#[doc(alias = "VkBlendOverlapEXT")]
#[repr(u32)]
pub enum BlendOverlapEXT {
    Uncorrelated = 0,
    Disjoint = 1,
    Conjoint = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html>"]
#[doc(alias = "VkAccelerationStructureTypeKHR")]
#[repr(u32)]
pub enum AccelerationStructureTypeKHR {
    TopLevel = 0,
    BottomLevel = 1,
    Generic = 2,
}
#[allow(non_upper_case_globals)]
impl AccelerationStructureTypeKHR {
    pub const TopLevelNV: Self = Self::TopLevel;
    pub const BottomLevelNV: Self = Self::BottomLevel;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeNV.html>"]
#[doc(alias = "VkAccelerationStructureTypeNV")]
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html>"]
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
#[repr(u32)]
pub enum AccelerationStructureBuildTypeKHR {
    Host = 0,
    Device = 1,
    HostOrDevice = 2,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html>"]
    #[doc(alias = "VkGeometryFlagBitsKHR")]
    pub struct GeometryFlagsKHR : u32 {
        const Opaque = 1u32 << 0;
        const NoDuplicateAnyHitInvocation = 1u32 << 1;
        const OpaqueNV = Self::Opaque.bits();
        const NoDuplicateAnyHitInvocationNV = Self::NoDuplicateAnyHitInvocation.bits();
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsNV.html>"]
#[doc(alias = "VkGeometryFlagBitsNV")]
pub type GeometryFlagsNV = GeometryFlagsKHR;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html>"]
    #[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
    pub struct GeometryInstanceFlagsKHR : u32 {
        const TriangleFacingCullDisable = 1u32 << 0;
        const TriangleFlipFacing = 1u32 << 1;
        const ForceOpaque = 1u32 << 2;
        const ForceNoOpaque = 1u32 << 3;
        const TriangleFrontCounterclockwise = Self::TriangleFlipFacing.bits();
        const TriangleCullDisableNV = Self::TriangleFacingCullDisable.bits();
        const TriangleFrontCounterclockwiseNV = Self::TriangleFrontCounterclockwise.bits();
        const ForceOpaqueNV = Self::ForceOpaque.bits();
        const ForceNoOpaqueNV = Self::ForceNoOpaque.bits();
        const ForceOpacityMicromap2StateEXT = 1u32 << 4;
        const DisableOpacityMicromapsEXT = 1u32 << 5;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsNV.html>"]
#[doc(alias = "VkGeometryInstanceFlagBitsNV")]
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html>"]
    #[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
    pub struct BuildAccelerationStructureFlagsKHR : u32 {
        const AllowUpdate = 1u32 << 0;
        const AllowCompaction = 1u32 << 1;
        const PreferFastTrace = 1u32 << 2;
        const PreferFastBuild = 1u32 << 3;
        const LowMemory = 1u32 << 4;
        const AllowUpdateNV = Self::AllowUpdate.bits();
        const AllowCompactionNV = Self::AllowCompaction.bits();
        const PreferFastTraceNV = Self::PreferFastTrace.bits();
        const PreferFastBuildNV = Self::PreferFastBuild.bits();
        const LowMemoryNV = Self::LowMemory.bits();
        const MotionNV = 1u32 << 5;
        const AllowOpacityMicromapUpdateEXT = 1u32 << 6;
        const AllowDisableOpacityMicromapsEXT = 1u32 << 7;
        const AllowOpacityMicromapDataUpdateEXT = 1u32 << 8;
        const AllowDisplacementMicromapUpdateNV = 1u32 << 9;
        const AllowDataAccess = 1u32 << 11;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsNV.html>"]
#[doc(alias = "VkBuildAccelerationStructureFlagBitsNV")]
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html>"]
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
#[repr(u32)]
pub enum CopyAccelerationStructureModeKHR {
    Clone = 0,
    Compact = 1,
    Serialize = 2,
    Deserialize = 3,
}
#[allow(non_upper_case_globals)]
impl CopyAccelerationStructureModeKHR {
    pub const CloneNV: Self = Self::Clone;
    pub const CompactNV: Self = Self::Compact;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeNV.html>"]
#[doc(alias = "VkCopyAccelerationStructureModeNV")]
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html>"]
#[doc(alias = "VkGeometryTypeKHR")]
#[repr(u32)]
pub enum GeometryTypeKHR {
    Triangles = 0,
    Aabbs = 1,
    Instances = 2,
}
#[allow(non_upper_case_globals)]
impl GeometryTypeKHR {
    pub const TrianglesNV: Self = Self::Triangles;
    pub const AabbsNV: Self = Self::Aabbs;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeNV.html>"]
#[doc(alias = "VkGeometryTypeNV")]
pub type GeometryTypeNV = GeometryTypeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html>"]
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
#[repr(u32)]
pub enum AccelerationStructureCompatibilityKHR {
    Compatible = 0,
    Incompatible = 1,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html>"]
    #[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
    pub struct AccelerationStructureCreateFlagsKHR : u32 {
        const DeviceAddressCaptureReplay = 1u32 << 0;
        const DescriptorBufferCaptureReplayEXT = 1u32 << 3;
        const MotionNV = 1u32 << 2;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html>"]
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
#[repr(u32)]
pub enum BuildAccelerationStructureModeKHR {
    Build = 0,
    Update = 1,
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_KHR.html>"]
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = !0u32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html>"]
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
#[repr(u32)]
pub enum RayTracingShaderGroupTypeKHR {
    General = 0,
    TrianglesHitGroup = 1,
    ProceduralHitGroup = 2,
}
#[allow(non_upper_case_globals)]
impl RayTracingShaderGroupTypeKHR {
    pub const GeneralNV: Self = Self::General;
    pub const TrianglesHitGroupNV: Self = Self::TrianglesHitGroup;
    pub const ProceduralHitGroupNV: Self = Self::ProceduralHitGroup;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeNV.html>"]
#[doc(alias = "VkRayTracingShaderGroupTypeNV")]
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html>"]
#[doc(alias = "VkShaderGroupShaderKHR")]
#[repr(u32)]
pub enum ShaderGroupShaderKHR {
    General = 0,
    ClosestHit = 1,
    AnyHit = 2,
    Intersection = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html>"]
#[doc(alias = "VkCoverageModulationModeNV")]
#[repr(u32)]
pub enum CoverageModulationModeNV {
    None = 0,
    Rgb = 1,
    Alpha = 2,
    Rgba = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html>"]
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
#[repr(u32)]
pub enum ValidationCacheHeaderVersionEXT {
    One = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteEntryNV.html>"]
#[doc(alias = "VkShadingRatePaletteEntryNV")]
#[repr(u32)]
pub enum ShadingRatePaletteEntryNV {
    NoInvocations = 0,
    Rate16InvocationsPerPixel = 1,
    Rate8InvocationsPerPixel = 2,
    Rate4InvocationsPerPixel = 3,
    Rate2InvocationsPerPixel = 4,
    Rate1InvocationPerPixel = 5,
    Rate1InvocationPer2X1Pixels = 6,
    Rate1InvocationPer1X2Pixels = 7,
    Rate1InvocationPer2X2Pixels = 8,
    Rate1InvocationPer4X2Pixels = 9,
    Rate1InvocationPer2X4Pixels = 10,
    Rate1InvocationPer4X4Pixels = 11,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderTypeNV.html>"]
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
#[repr(u32)]
pub enum CoarseSampleOrderTypeNV {
    Default = 0,
    Custom = 1,
    PixelMajor = 2,
    SampleMajor = 3,
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_SHADER_UNUSED_NV.html>"]
#[doc(alias = "VK_SHADER_UNUSED_NV")]
pub const SHADER_UNUSED_NV: u32 = SHADER_UNUSED_KHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html>"]
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
#[repr(u32)]
pub enum AccelerationStructureMemoryRequirementsTypeNV {
    Object = 0,
    BuildScratch = 1,
    UpdateScratch = 2,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html>"]
    #[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
    pub struct PipelineCompilerControlFlagsAMD : u32 {
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_GLOBAL_PRIORITY_SIZE_KHR.html>"]
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_KHR")]
pub const MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityKHR.html>"]
#[doc(alias = "VkQueueGlobalPriorityKHR")]
#[repr(u32)]
pub enum QueueGlobalPriorityKHR {
    Low = 128,
    Medium = 256,
    High = 512,
    Realtime = 1024,
}
#[allow(non_upper_case_globals)]
impl QueueGlobalPriorityKHR {
    pub const LowEXT: Self = Self::Low;
    pub const MediumEXT: Self = Self::Medium;
    pub const HighEXT: Self = Self::High;
    pub const RealtimeEXT: Self = Self::Realtime;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityEXT.html>"]
#[doc(alias = "VkQueueGlobalPriorityEXT")]
pub type QueueGlobalPriorityEXT = QueueGlobalPriorityKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html>"]
#[doc(alias = "VkMemoryOverallocationBehaviorAMD")]
#[repr(u32)]
pub enum MemoryOverallocationBehaviorAMD {
    Default = 0,
    Allowed = 1,
    Disallowed = 2,
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_NAME_SIZE_KHR.html>"]
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE_KHR")]
pub const MAX_DRIVER_NAME_SIZE_KHR: u32 = MAX_DRIVER_NAME_SIZE;
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE_KHR.html>"]
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE_KHR")]
pub const MAX_DRIVER_INFO_SIZE_KHR: u32 = MAX_DRIVER_INFO_SIZE;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html>"]
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
#[repr(u32)]
pub enum PerformanceConfigurationTypeINTEL {
    CommandQueueMetricsDiscoveryActivated = 0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html>"]
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
#[repr(u32)]
pub enum QueryPoolSamplingModeINTEL {
    Manual = 0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html>"]
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
#[repr(u32)]
pub enum PerformanceOverrideTypeINTEL {
    NullHardware = 0,
    FlushGpuCaches = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html>"]
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
#[repr(u32)]
pub enum PerformanceParameterTypeINTEL {
    HwCountersSupported = 0,
    StreamMarkerValidBits = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html>"]
#[doc(alias = "VkPerformanceValueTypeINTEL")]
#[repr(u32)]
pub enum PerformanceValueTypeINTEL {
    Uint32 = 0,
    Uint64 = 1,
    Float = 2,
    Bool = 3,
    String = 4,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html>"]
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[repr(u32)]
pub enum FragmentShadingRateCombinerOpKHR {
    Keep = 0,
    Replace = 1,
    Min = 2,
    Max = 3,
    Mul = 4,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html>"]
    #[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
    pub struct ShaderCorePropertiesFlagsAMD : u32 {
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureEnableEXT.html>"]
#[doc(alias = "VkValidationFeatureEnableEXT")]
#[repr(u32)]
pub enum ValidationFeatureEnableEXT {
    GpuAssisted = 0,
    GpuAssistedReserveBindingSlot = 1,
    BestPractices = 2,
    DebugPrintf = 3,
    SynchronizationValidation = 4,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html>"]
#[doc(alias = "VkValidationFeatureDisableEXT")]
#[repr(u32)]
pub enum ValidationFeatureDisableEXT {
    All = 0,
    Shaders = 1,
    ThreadSafety = 2,
    ApiParameters = 3,
    ObjectLifetimes = 4,
    CoreChecks = 5,
    UniqueHandles = 6,
    ShaderValidationCache = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageReductionModeNV.html>"]
#[doc(alias = "VkCoverageReductionModeNV")]
#[repr(u32)]
pub enum CoverageReductionModeNV {
    Merge = 0,
    Truncate = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html>"]
#[doc(alias = "VkProvokingVertexModeEXT")]
#[repr(u32)]
pub enum ProvokingVertexModeEXT {
    FirstVertex = 0,
    LastVertex = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html>"]
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[repr(u32)]
pub enum FullScreenExclusiveEXT {
    Default = 0,
    Allowed = 1,
    Disallowed = 2,
    ApplicationControlled = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html>"]
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[repr(u32)]
pub enum PipelineExecutableStatisticFormatKHR {
    Bool32 = 0,
    Int64 = 1,
    Uint64 = 2,
    Float64 = 3,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHostImageCopyFlagBitsEXT.html>"]
    #[doc(alias = "VkHostImageCopyFlagBitsEXT")]
    pub struct HostImageCopyFlagsEXT : u32 {
        const Memcpy = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryUnmapFlagBitsKHR.html>"]
    #[doc(alias = "VkMemoryUnmapFlagBitsKHR")]
    pub struct MemoryUnmapFlagsKHR : u32 {
        const ReserveEXT = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentScalingFlagBitsEXT.html>"]
    #[doc(alias = "VkPresentScalingFlagBitsEXT")]
    pub struct PresentScalingFlagsEXT : u32 {
        const OneToOne = 1u32 << 0;
        const AspectRatioStretch = 1u32 << 1;
        const Stretch = 1u32 << 2;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentGravityFlagBitsEXT.html>"]
    #[doc(alias = "VkPresentGravityFlagBitsEXT")]
    pub struct PresentGravityFlagsEXT : u32 {
        const Min = 1u32 << 0;
        const Max = 1u32 << 1;
        const Centered = 1u32 << 2;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html>"]
    #[doc(alias = "VkIndirectStateFlagBitsNV")]
    pub struct IndirectStateFlagsNV : u32 {
        const FlagFrontface = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html>"]
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
#[repr(u32)]
pub enum IndirectCommandsTokenTypeNV {
    ShaderGroup = 0,
    StateFlags = 1,
    IndexBuffer = 2,
    VertexBuffer = 3,
    PushConstant = 4,
    DrawIndexed = 5,
    Draw = 6,
    DrawTasks = 7,
    DrawMeshTasks = 1000328000,
    Pipeline = 1000428003,
    Dispatch = 1000428004,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html>"]
    #[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
    pub struct IndirectCommandsLayoutUsageFlagsNV : u32 {
        const ExplicitPreprocess = 1u32 << 0;
        const IndexedSequences = 1u32 << 1;
        const UnorderedSequences = 1u32 << 2;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDepthBiasRepresentationEXT.html>"]
#[doc(alias = "VkDepthBiasRepresentationEXT")]
#[repr(u32)]
pub enum DepthBiasRepresentationEXT {
    LeastRepresentableValueFormat = 0,
    LeastRepresentableValueForceUnorm = 1,
    Float = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html>"]
#[doc(alias = "VkDeviceMemoryReportEventTypeEXT")]
#[repr(u32)]
pub enum DeviceMemoryReportEventTypeEXT {
    Allocate = 0,
    Free = 1,
    Import = 2,
    Unimport = 3,
    AllocationFailed = 4,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html>"]
    #[doc(alias = "VkPipelineCacheCreateFlagBits")]
    pub struct PipelineCacheCreateFlags : u32 {
        const ExternallySynchronized = 1u32 << 0;
        const ExternallySynchronizedEXT = Self::ExternallySynchronized.bits();
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html>"]
    #[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
    pub struct DeviceDiagnosticsConfigFlagsNV : u32 {
        const EnableShaderDebugInfo = 1u32 << 0;
        const EnableResourceTracking = 1u32 << 1;
        const EnableAutomaticCheckpoints = 1u32 << 2;
        const EnableShaderErrorReporting = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMetalObjectTypeFlagBitsEXT.html>"]
    #[doc(alias = "VkExportMetalObjectTypeFlagBitsEXT")]
    pub struct ExportMetalObjectTypeFlagsEXT : u32 {
        const MetalDevice = 1u32 << 0;
        const MetalCommandQueue = 1u32 << 1;
        const MetalBuffer = 1u32 << 2;
        const MetalTexture = 1u32 << 3;
        const MetalIosurface = 1u32 << 4;
        const MetalSharedEvent = 1u32 << 5;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html>"]
    #[doc(alias = "VkGraphicsPipelineLibraryFlagBitsEXT")]
    pub struct GraphicsPipelineLibraryFlagsEXT : u32 {
        const VertexInputInterface = 1u32 << 0;
        const PreRasterizationShaders = 1u32 << 1;
        const FragmentShader = 1u32 << 2;
        const FragmentOutputInterface = 1u32 << 3;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateFlagBits.html>"]
    #[doc(alias = "VkPipelineLayoutCreateFlagBits")]
    pub struct PipelineLayoutCreateFlags : u32 {
        const IndependentSetsEXT = 1u32 << 1;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html>"]
#[doc(alias = "VkFragmentShadingRateNV")]
#[repr(u32)]
pub enum FragmentShadingRateNV {
    Rate1InvocationPerPixel = 0,
    Rate1InvocationPer1X2Pixels = 1,
    Rate1InvocationPer2X1Pixels = 4,
    Rate1InvocationPer2X2Pixels = 5,
    Rate1InvocationPer2X4Pixels = 6,
    Rate1InvocationPer4X2Pixels = 9,
    Rate1InvocationPer4X4Pixels = 10,
    Rate2InvocationsPerPixel = 11,
    Rate4InvocationsPerPixel = 12,
    Rate8InvocationsPerPixel = 13,
    Rate16InvocationsPerPixel = 14,
    NoInvocations = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html>"]
#[doc(alias = "VkFragmentShadingRateTypeNV")]
#[repr(u32)]
pub enum FragmentShadingRateTypeNV {
    FragmentSize = 0,
    Enums = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceTypeNV.html>"]
#[doc(alias = "VkAccelerationStructureMotionInstanceTypeNV")]
#[repr(u32)]
pub enum AccelerationStructureMotionInstanceTypeNV {
    Static = 0,
    MatrixMotion = 1,
    SrtMotion = 2,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFlagBitsEXT.html>"]
    #[doc(alias = "VkImageCompressionFlagBitsEXT")]
    pub struct ImageCompressionFlagsEXT : u32 {
        const Default = 0;
        const FixedRateDefault = 1u32 << 0;
        const FixedRateExplicit = 1u32 << 1;
        const Disabled = 1u32 << 2;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCompressionFixedRateFlagBitsEXT.html>"]
    #[doc(alias = "VkImageCompressionFixedRateFlagBitsEXT")]
    pub struct ImageCompressionFixedRateFlagsEXT : u32 {
        const None = 0;
        const Rate1Bpc = 1u32 << 0;
        const Rate2Bpc = 1u32 << 1;
        const Rate3Bpc = 1u32 << 2;
        const Rate4Bpc = 1u32 << 3;
        const Rate5Bpc = 1u32 << 4;
        const Rate6Bpc = 1u32 << 5;
        const Rate7Bpc = 1u32 << 6;
        const Rate8Bpc = 1u32 << 7;
        const Rate9Bpc = 1u32 << 8;
        const Rate10Bpc = 1u32 << 9;
        const Rate11Bpc = 1u32 << 10;
        const Rate12Bpc = 1u32 << 11;
        const Rate13Bpc = 1u32 << 12;
        const Rate14Bpc = 1u32 << 13;
        const Rate15Bpc = 1u32 << 14;
        const Rate16Bpc = 1u32 << 15;
        const Rate17Bpc = 1u32 << 16;
        const Rate18Bpc = 1u32 << 17;
        const Rate19Bpc = 1u32 << 18;
        const Rate20Bpc = 1u32 << 19;
        const Rate21Bpc = 1u32 << 20;
        const Rate22Bpc = 1u32 << 21;
        const Rate23Bpc = 1u32 << 22;
        const Rate24Bpc = 1u32 << 23;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultAddressTypeEXT.html>"]
#[doc(alias = "VkDeviceFaultAddressTypeEXT")]
#[repr(u32)]
pub enum DeviceFaultAddressTypeEXT {
    None = 0,
    ReadInvalid = 1,
    WriteInvalid = 2,
    ExecuteInvalid = 3,
    InstructionPointerUnknown = 4,
    InstructionPointerInvalid = 5,
    InstructionPointerFault = 6,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceFaultVendorBinaryHeaderVersionEXT.html>"]
#[doc(alias = "VkDeviceFaultVendorBinaryHeaderVersionEXT")]
#[repr(u32)]
pub enum DeviceFaultVendorBinaryHeaderVersionEXT {
    One = 1,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingFlagBitsEXT.html>"]
    #[doc(alias = "VkDeviceAddressBindingFlagBitsEXT")]
    pub struct DeviceAddressBindingFlagsEXT : u32 {
        const InternalObject = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceAddressBindingTypeEXT.html>"]
#[doc(alias = "VkDeviceAddressBindingTypeEXT")]
#[repr(u32)]
pub enum DeviceAddressBindingTypeEXT {
    Bind = 0,
    Unbind = 1,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html>"]
    #[doc(alias = "VkImageConstraintsInfoFlagBitsFUCHSIA")]
    pub struct ImageConstraintsInfoFlagsFUCHSIA : u32 {
        const CpuReadRarely = 1u32 << 0;
        const CpuReadOften = 1u32 << 1;
        const CpuWriteRarely = 1u32 << 2;
        const CpuWriteOften = 1u32 << 3;
        const ProtectedOptional = 1u32 << 4;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFrameBoundaryFlagBitsEXT.html>"]
    #[doc(alias = "VkFrameBoundaryFlagBitsEXT")]
    pub struct FrameBoundaryFlagsEXT : u32 {
        const FrameEnd = 1u32 << 0;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_GLOBAL_PRIORITY_SIZE_EXT.html>"]
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_EXT")]
pub const MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = MAX_GLOBAL_PRIORITY_SIZE_KHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapTypeEXT.html>"]
#[doc(alias = "VkMicromapTypeEXT")]
#[repr(u32)]
pub enum MicromapTypeEXT {
    OpacityMicromap = 0,
    DisplacementMicromapNV = 1000397000,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapFlagBitsEXT.html>"]
    #[doc(alias = "VkBuildMicromapFlagBitsEXT")]
    pub struct BuildMicromapFlagsEXT : u32 {
        const PreferFastTrace = 1u32 << 0;
        const PreferFastBuild = 1u32 << 1;
        const AllowCompaction = 1u32 << 2;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMicromapModeEXT.html>"]
#[doc(alias = "VkCopyMicromapModeEXT")]
#[repr(u32)]
pub enum CopyMicromapModeEXT {
    Clone = 0,
    Serialize = 1,
    Deserialize = 2,
    Compact = 3,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMicromapCreateFlagBitsEXT.html>"]
    #[doc(alias = "VkMicromapCreateFlagBitsEXT")]
    pub struct MicromapCreateFlagsEXT : u32 {
        const DeviceAddressCaptureReplay = 1u32 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildMicromapModeEXT.html>"]
#[doc(alias = "VkBuildMicromapModeEXT")]
#[repr(u32)]
pub enum BuildMicromapModeEXT {
    Build = 0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpacityMicromapFormatEXT.html>"]
#[doc(alias = "VkOpacityMicromapFormatEXT")]
#[repr(u32)]
pub enum OpacityMicromapFormatEXT {
    Format2State = 1,
    Format4State = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpacityMicromapSpecialIndexEXT.html>"]
#[doc(alias = "VkOpacityMicromapSpecialIndexEXT")]
#[repr(i32)]
pub enum OpacityMicromapSpecialIndexEXT {
    FullyTransparent = -1,
    FullyOpaque = -2,
    FullyUnknownTransparent = -3,
    FullyUnknownOpaque = -4,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplacementMicromapFormatNV.html>"]
#[doc(alias = "VkDisplacementMicromapFormatNV")]
#[repr(u32)]
pub enum DisplacementMicromapFormatNV {
    Format64Triangles64Bytes = 1,
    Format256Triangles128Bytes = 2,
    Format1024Triangles128Bytes = 3,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>"]
    #[doc(alias = "VkPhysicalDeviceSchedulingControlsFlagBitsARM")]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM : u64 {
        const ShaderCoreCount = 1u64 << 0;
    }
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_REMAINING_3D_SLICES_EXT.html>"]
#[doc(alias = "VK_REMAINING_3D_SLICES_EXT")]
pub const REMAINING_3D_SLICES_EXT: u32 = !0u32;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDecompressionMethodFlagBitsNV.html>"]
    #[doc(alias = "VkMemoryDecompressionMethodFlagBitsNV")]
    pub struct MemoryDecompressionMethodFlagsNV : u64 {
        const Gdeflate10 = 1u64 << 0;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassMergeStatusEXT.html>"]
#[doc(alias = "VkSubpassMergeStatusEXT")]
#[repr(u32)]
pub enum SubpassMergeStatusEXT {
    Merged = 0,
    Disallowed = 1,
    NotMergedSideEffects = 2,
    NotMergedSamplesMismatch = 3,
    NotMergedViewsMismatch = 4,
    NotMergedAliasing = 5,
    NotMergedDependencies = 6,
    NotMergedIncompatibleInputAttachment = 7,
    NotMergedTooManyAttachments = 8,
    NotMergedInsufficientStorage = 9,
    NotMergedDepthStencilCount = 10,
    NotMergedResolveAttachmentReuse = 11,
    NotMergedSingleSubpass = 12,
    NotMergedUnspecified = 13,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectDriverLoadingModeLUNARG.html>"]
#[doc(alias = "VkDirectDriverLoadingModeLUNARG")]
#[repr(u32)]
pub enum DirectDriverLoadingModeLUNARG {
    Exclusive = 0,
    Inclusive = 1,
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT.html>"]
#[doc(alias = "VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT")]
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html>"]
    #[doc(alias = "VkPipelineColorBlendStateCreateFlagBits")]
    pub struct PipelineColorBlendStateCreateFlags : u32 {
        const RasterizationOrderAttachmentAccessARM = Self::RasterizationOrderAttachmentAccessEXT.bits();
        const RasterizationOrderAttachmentAccessEXT = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html>"]
    #[doc(alias = "VkPipelineDepthStencilStateCreateFlagBits")]
    pub struct PipelineDepthStencilStateCreateFlags : u32 {
        const RasterizationOrderAttachmentDepthAccessARM = Self::RasterizationOrderAttachmentDepthAccessEXT.bits();
        const RasterizationOrderAttachmentStencilAccessARM = Self::RasterizationOrderAttachmentStencilAccessEXT.bits();
        const RasterizationOrderAttachmentDepthAccessEXT = 1u32 << 0;
        const RasterizationOrderAttachmentStencilAccessEXT = 1u32 << 1;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowUsageFlagBitsNV.html>"]
    #[doc(alias = "VkOpticalFlowUsageFlagBitsNV")]
    pub struct OpticalFlowUsageFlagsNV : u32 {
        const Unknown = 0;
        const Input = 1u32 << 0;
        const Output = 1u32 << 1;
        const Hint = 1u32 << 2;
        const Cost = 1u32 << 3;
        const GlobalFlow = 1u32 << 4;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowGridSizeFlagBitsNV.html>"]
    #[doc(alias = "VkOpticalFlowGridSizeFlagBitsNV")]
    pub struct OpticalFlowGridSizeFlagsNV : u32 {
        const Unknown = 0;
        const Size1X1 = 1u32 << 0;
        const Size2X2 = 1u32 << 1;
        const Size4X4 = 1u32 << 2;
        const Size8X8 = 1u32 << 3;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowPerformanceLevelNV.html>"]
#[doc(alias = "VkOpticalFlowPerformanceLevelNV")]
#[repr(u32)]
pub enum OpticalFlowPerformanceLevelNV {
    Unknown = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionBindingPointNV.html>"]
#[doc(alias = "VkOpticalFlowSessionBindingPointNV")]
#[repr(u32)]
pub enum OpticalFlowSessionBindingPointNV {
    Unknown = 0,
    Input = 1,
    Reference = 2,
    Hint = 3,
    FlowVector = 4,
    BackwardFlowVector = 5,
    Cost = 6,
    BackwardCost = 7,
    GlobalFlow = 8,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html>"]
    #[doc(alias = "VkOpticalFlowSessionCreateFlagBitsNV")]
    pub struct OpticalFlowSessionCreateFlagsNV : u32 {
        const EnableHint = 1u32 << 0;
        const EnableCost = 1u32 << 1;
        const EnableGlobalFlow = 1u32 << 2;
        const AllowRegions = 1u32 << 3;
        const BothDirections = 1u32 << 4;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOpticalFlowExecuteFlagBitsNV.html>"]
    #[doc(alias = "VkOpticalFlowExecuteFlagBitsNV")]
    pub struct OpticalFlowExecuteFlagsNV : u32 {
        const DisableTemporalHints = 1u32 << 0;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits2KHR.html>"]
    #[doc(alias = "VkPipelineCreateFlagBits2KHR")]
    pub struct PipelineCreateFlags2KHR : u64 {
        const DisableOptimization = 1u64 << 0;
        const AllowDerivatives = 1u64 << 1;
        const Derivative = 1u64 << 2;
        const EnableLegacyDitheringEXT = 1u64 << 34;
        const ViewIndexFromDeviceIndex = 1u64 << 3;
        const DispatchBase = 1u64 << 4;
        const DeferCompileNV = 1u64 << 5;
        const CaptureStatistics = 1u64 << 6;
        const CaptureInternalRepresentations = 1u64 << 7;
        const FailOnPipelineCompileRequired = 1u64 << 8;
        const EarlyReturnOnFailure = 1u64 << 9;
        const LinkTimeOptimizationEXT = 1u64 << 10;
        const RetainLinkTimeOptimizationInfoEXT = 1u64 << 23;
        const Library = 1u64 << 11;
        const RayTracingSkipTriangles = 1u64 << 12;
        const RayTracingSkipAabbs = 1u64 << 13;
        const RayTracingNoNullAnyHitShaders = 1u64 << 14;
        const RayTracingNoNullClosestHitShaders = 1u64 << 15;
        const RayTracingNoNullMissShaders = 1u64 << 16;
        const RayTracingNoNullIntersectionShaders = 1u64 << 17;
        const RayTracingShaderGroupHandleCaptureReplay = 1u64 << 19;
        const IndirectBindableNV = 1u64 << 18;
        const RayTracingAllowMotionNV = 1u64 << 20;
        const RenderingFragmentShadingRateAttachment = 1u64 << 21;
        const RenderingFragmentDensityMapAttachmentEXT = 1u64 << 22;
        const RayTracingOpacityMicromapEXT = 1u64 << 24;
        const ColorAttachmentFeedbackLoopEXT = 1u64 << 25;
        const DepthStencilAttachmentFeedbackLoopEXT = 1u64 << 26;
        const NoProtectedAccessEXT = 1u64 << 27;
        const ProtectedAccessOnlyEXT = 1u64 << 30;
        const RayTracingDisplacementMicromapNV = 1u64 << 28;
        const DescriptorBufferEXT = 1u64 << 29;
    }
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits2KHR.html>"]
    #[doc(alias = "VkBufferUsageFlagBits2KHR")]
    pub struct BufferUsageFlags2KHR : u64 {
        const TransferSrc = 1u64 << 0;
        const TransferDst = 1u64 << 1;
        const UniformTexelBuffer = 1u64 << 2;
        const StorageTexelBuffer = 1u64 << 3;
        const UniformBuffer = 1u64 << 4;
        const StorageBuffer = 1u64 << 5;
        const IndexBuffer = 1u64 << 6;
        const VertexBuffer = 1u64 << 7;
        const IndirectBuffer = 1u64 << 8;
        const ExecutionGraphScratchAMDX = 1u64 << 25;
        const ConditionalRenderingEXT = 1u64 << 9;
        const ShaderBindingTable = 1u64 << 10;
        const RayTracingNV = Self::ShaderBindingTable.bits();
        const TransformFeedbackBufferEXT = 1u64 << 11;
        const TransformFeedbackCounterBufferEXT = 1u64 << 12;
        const VideoDecodeSrc = 1u64 << 13;
        const VideoDecodeDst = 1u64 << 14;
        const VideoEncodeDst = 1u64 << 15;
        const VideoEncodeSrc = 1u64 << 16;
        const ShaderDeviceAddress = 1u64 << 17;
        const AccelerationStructureBuildInputReadOnly = 1u64 << 19;
        const AccelerationStructureStorage = 1u64 << 20;
        const SamplerDescriptorBufferEXT = 1u64 << 21;
        const ResourceDescriptorBufferEXT = 1u64 << 22;
        const PushDescriptorsDescriptorBufferEXT = 1u64 << 26;
        const MicromapBuildInputReadOnlyEXT = 1u64 << 23;
        const MicromapStorageEXT = 1u64 << 24;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAntiLagModeAMD.html>"]
#[doc(alias = "VkAntiLagModeAMD")]
#[repr(u32)]
pub enum AntiLagModeAMD {
    DriverControl = 0,
    On = 1,
    Off = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAntiLagStageAMD.html>"]
#[doc(alias = "VkAntiLagStageAMD")]
#[repr(u32)]
pub enum AntiLagStageAMD {
    Input = 0,
    Present = 1,
}
bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCreateFlagBitsEXT.html>"]
    #[doc(alias = "VkShaderCreateFlagBitsEXT")]
    pub struct ShaderCreateFlagsEXT : u32 {
        const LinkStage = 1u32 << 0;
        const AllowVaryingSubgroupSize = 1u32 << 1;
        const RequireFullSubgroups = 1u32 << 2;
        const NoTaskShader = 1u32 << 3;
        const DispatchBase = 1u32 << 4;
        const FragmentShadingRateAttachment = 1u32 << 5;
        const FragmentDensityMapAttachment = 1u32 << 6;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCodeTypeEXT.html>"]
#[doc(alias = "VkShaderCodeTypeEXT")]
#[repr(u32)]
pub enum ShaderCodeTypeEXT {
    Binary = 0,
    Spirv = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingInvocationReorderModeNV.html>"]
#[doc(alias = "VkRayTracingInvocationReorderModeNV")]
#[repr(u32)]
pub enum RayTracingInvocationReorderModeNV {
    None = 0,
    Reorder = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLayerSettingTypeEXT.html>"]
#[doc(alias = "VkLayerSettingTypeEXT")]
#[repr(u32)]
pub enum LayerSettingTypeEXT {
    Bool32 = 0,
    Int32 = 1,
    Int64 = 2,
    Uint32 = 3,
    Uint64 = 4,
    Float32 = 5,
    Float64 = 6,
    String = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLatencyMarkerNV.html>"]
#[doc(alias = "VkLatencyMarkerNV")]
#[repr(u32)]
pub enum LatencyMarkerNV {
    SimulationStart = 0,
    SimulationEnd = 1,
    RendersubmitStart = 2,
    RendersubmitEnd = 3,
    PresentStart = 4,
    PresentEnd = 5,
    InputSample = 6,
    TriggerFlash = 7,
    OutOfBandRendersubmitStart = 8,
    OutOfBandRendersubmitEnd = 9,
    OutOfBandPresentStart = 10,
    OutOfBandPresentEnd = 11,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkOutOfBandQueueTypeNV.html>"]
#[doc(alias = "VkOutOfBandQueueTypeNV")]
#[repr(u32)]
pub enum OutOfBandQueueTypeNV {
    Render = 0,
    Present = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScopeKHR.html>"]
#[doc(alias = "VkScopeKHR")]
#[repr(u32)]
pub enum ScopeKHR {
    Device = 1,
    Workgroup = 2,
    Subgroup = 3,
    QueueFamily = 5,
}
#[allow(non_upper_case_globals)]
impl ScopeKHR {
    pub const DeviceNV: Self = Self::Device;
    pub const WorkgroupNV: Self = Self::Workgroup;
    pub const SubgroupNV: Self = Self::Subgroup;
    pub const QueueFamilyNV: Self = Self::QueueFamily;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScopeNV.html>"]
#[doc(alias = "VkScopeNV")]
pub type ScopeNV = ScopeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentTypeKHR.html>"]
#[doc(alias = "VkComponentTypeKHR")]
#[repr(u32)]
pub enum ComponentTypeKHR {
    Float16 = 0,
    Float32 = 1,
    Float64 = 2,
    Sint8 = 3,
    Sint16 = 4,
    Sint32 = 5,
    Sint64 = 6,
    Uint8 = 7,
    Uint16 = 8,
    Uint32 = 9,
    Uint64 = 10,
}
#[allow(non_upper_case_globals)]
impl ComponentTypeKHR {
    pub const Float16NV: Self = Self::Float16;
    pub const Float32NV: Self = Self::Float32;
    pub const Float64NV: Self = Self::Float64;
    pub const Sint8NV: Self = Self::Sint8;
    pub const Sint16NV: Self = Self::Sint16;
    pub const Sint32NV: Self = Self::Sint32;
    pub const Sint64NV: Self = Self::Sint64;
    pub const Uint8NV: Self = Self::Uint8;
    pub const Uint16NV: Self = Self::Uint16;
    pub const Uint32NV: Self = Self::Uint32;
    pub const Uint64NV: Self = Self::Uint64;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentTypeNV.html>"]
#[doc(alias = "VkComponentTypeNV")]
pub type ComponentTypeNV = ComponentTypeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlockMatchWindowCompareModeQCOM.html>"]
#[doc(alias = "VkBlockMatchWindowCompareModeQCOM")]
#[repr(u32)]
pub enum BlockMatchWindowCompareModeQCOM {
    Min = 0,
    Max = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCubicFilterWeightsQCOM.html>"]
#[doc(alias = "VkCubicFilterWeightsQCOM")]
#[repr(u32)]
pub enum CubicFilterWeightsQCOM {
    CatmullRom = 0,
    ZeroTangentCardinal = 1,
    BSpline = 2,
    MitchellNetravali = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLayeredDriverUnderlyingApiMSFT.html>"]
#[doc(alias = "VkLayeredDriverUnderlyingApiMSFT")]
#[repr(u32)]
pub enum LayeredDriverUnderlyingApiMSFT {
    None = 0,
    D3D12 = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLineRasterizationModeKHR.html>"]
#[doc(alias = "VkLineRasterizationModeKHR")]
#[repr(u32)]
pub enum LineRasterizationModeKHR {
    Default = 0,
    Rectangular = 1,
    Bresenham = 2,
    RectangularSmooth = 3,
}
#[allow(non_upper_case_globals)]
impl LineRasterizationModeKHR {
    pub const DefaultEXT: Self = Self::Default;
    pub const RectangularEXT: Self = Self::Rectangular;
    pub const BresenhamEXT: Self = Self::Bresenham;
    pub const RectangularSmoothEXT: Self = Self::RectangularSmooth;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLineRasterizationModeEXT.html>"]
#[doc(alias = "VkLineRasterizationModeEXT")]
pub type LineRasterizationModeEXT = LineRasterizationModeKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimeDomainKHR.html>"]
#[doc(alias = "VkTimeDomainKHR")]
#[repr(u32)]
pub enum TimeDomainKHR {
    Device = 0,
    ClockMonotonic = 1,
    ClockMonotonicRaw = 2,
    QueryPerformanceCounter = 3,
}
#[allow(non_upper_case_globals)]
impl TimeDomainKHR {
    pub const DeviceEXT: Self = Self::Device;
    pub const ClockMonotonicEXT: Self = Self::ClockMonotonic;
    pub const ClockMonotonicRawEXT: Self = Self::ClockMonotonicRaw;
    pub const QueryPerformanceCounterEXT: Self = Self::QueryPerformanceCounter;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimeDomainEXT.html>"]
#[doc(alias = "VkTimeDomainEXT")]
pub type TimeDomainEXT = TimeDomainKHR;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLayeredApiKHR.html>"]
#[doc(alias = "VkPhysicalDeviceLayeredApiKHR")]
#[repr(u32)]
pub enum PhysicalDeviceLayeredApiKHR {
    Vulkan = 0,
    D3D12 = 1,
    Metal = 2,
    Opengl = 3,
    Opengles = 4,
}
