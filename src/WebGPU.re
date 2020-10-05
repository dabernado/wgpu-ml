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
type shaderModule;
type shaderModuleSource;
type commandEncoder;
type renderPass;
type computePass;
type buffer;
type texture;
type textureView;

/*
 * Descriptor types
 */
type limits = {
  maxBindGroups: int32,
  maxDynamicUniformBuffersPerPipelineLayout: int32,
  maxDynamicStorageBuffersPerPipelineLayout: int32,
  maxSampledTexturesPerShaderStage: int32,
  maxSamplersPerShaderStage: int32,
  maxStorageBuffersPerShaderStage: int32,
  maxStorageTexturesPerShaderStage: int32,
  maxUniformBuffersPerShaderStage: int32,
  maxUniformBufferBindingSize: int32,
  maxPushConstantSize: int32,
};

type deviceDescriptor = {
  features: features,
  limits: limits,
  shader_validation: bool,
};

type commandEncoderDescriptor = string option;

type renderPassDescriptor = {
  colorAttachments: renderPassColorAttachmentDescriptor list,
  depthStencilAttachment: renderPassDepthStencilAttachmentDescriptor option,
};

type renderPassColorAttachmentDescriptor = {
  attachment: textureView,
  resolveTarget: textureView option,
  ops: operations(color),
};

type renderPassDepthStencilAttachmentDescriptor = {
  attachment: textureView,
  depth_ops: operations(float) option,
  stencil_ops: operations(int32) option,
};

type color = (float, float, float, float);

type extent3D = (int32, int32, int32);

type origin3D = (int32, int32, int32);

type operations('a) = {
  load: loadOp('a),
  store: bool,
};

type loadOp('a) = Clear('a) | Load;

type textureDataLayout = {
  offset: int64,
  bytesPerRow: int32,
  rowsPerImage: int32,
};

/*
 * Handle types
 */
type bufferCopyView = {
  buffer: buffer,
  layout: textureDataLayout,
};

type textureCopyView = {
  texture: texture,
  mipLevel: int32,
  origin: origin3D,
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
