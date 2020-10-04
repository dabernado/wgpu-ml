include Stubs;

/*
 * Variants
 */
type backendBit =
  | Vulkan
  | GL
  | Metal
  | DX12
  | DX11
  | Browser
  | Primary
  | Secondary;

type window =
  | MacOS
  | Xlib
  | Xcb
  | Wayland
  | Windows
  | Web;

type powerPreference = Default | LowPower | HighPerformance;

type features =
  | DepthClamping
  | TextureCompressionBC
  | MappablePrimaryBuffers
  | SampledTextureBindingArray
  | SampledTextureArrayDynamicIndexing
  | SampledTextureArrayNonUniformIndexing
  | UnsizedBindingArray
  | MultiDrawIndirect
  | MultiDrawIndirectCount
  | PushConstants
  | AllWebGPU
  | AllNative
  | Empty;

/*
 * Custom types
 */
type instance;
type surface;
type adapter;
type device;
type queue;
type shaderModuleSource;

/*
 * Descriptor types
 */
type limits = {
  maxBindGroups: int,
  maxDynamicUniformBuffersPerPipelineLayout: int,
  maxDynamicStorageBuffersPerPipelineLayout: int,
  maxSampledTexturesPerShaderStage: int,
  maxSamplersPerShaderStage: int,
  maxStorageBuffersPerShaderStage: int,
  maxStorageTexturesPerShaderStage: int,
  maxUniformBuffersPerShaderStage: int,
  maxUniformBufferBindingSize: int,
  maxPushConstantSize: int,
};

type deviceDescriptor = {
  features: features,
  limits: limits,
  shader_validation: bool,
};

/*
 * Constants
 */
let defaultLimits = {
  maxBindGroups: 4,
  maxDynamicUniformBuffersPerPipelineLayout: 8,
  maxDynamicStorageBuffersPerPipelineLayout: 4,
  maxSampledTexturesPerShaderStage: 16,
  maxSamplersPerShaderStage: 16,
  maxStorageBuffersPerShaderStage: 4,
  maxStorageTexturesPerShaderStage: 4,
  maxUniformBuffersPerShaderStage: 12,
  maxUniformBufferBindingSize: 16384,
  maxPushConstantSize: 0,
};
