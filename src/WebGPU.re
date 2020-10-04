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
  | AllNative;

/*
 * Custom types
 */
type instance;
type surface;
type adapter;
type device;
type queue;

/*
 * Descriptor types
 */
type limits =
  | {}
  | {
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
