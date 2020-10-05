module type WebGPU = {
  
  // variants
  type backendBit;
  type powerPreference;
  type window;
  type features;

  // custom types
  type instance;
  type adapter;
  type surface;
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

  // descriptor types
  type deviceDescriptor;
  type commandEncoderDescriptor;
  type renderPassDescriptor;
  type renderPassColorAttachmentDescriptor;
  type renderPassDepthStencilAttachmentDescriptor;
  type textureDataLayout;
  type limits;
  type extent3D;
  type origin3D;
  type color;
  type operations;
  type loadOp;

  // handle types
  type bufferCopyView;
  type textureCopyView;

  // faux-type constructors
  let Instance: backendBit => instance;

  // constants
  let defaultLimits: limits;

  // functions
  let requestAdapter: (instance, surface, powerPreference) => adapter;
  let enumerateAdapters: (instance, backendBit) => adapter list;
  let createSurface: (instance, window) => surface;

  let requestDevice: (adapter, deviceDescriptor, string option) => (device, queue);

  let createShaderModule: (device, shaderModuleSource) => shaderModule;
  let createCommandEncoder: (device, commandEncoderDescriptor) => commandEncoder;

  let copyBufferToBuffer: (
    commandEncoder,
    (buffer, int64),
    (buffer, int64),
    int64
    ) => ();
  let copyBufferToTexture: (
    commandEncoder,
    bufferCopyView,
    textureCopyView,
    extent3D
    ) => ();
  let copyTextureToBuffer: (
    commandEncoder,
    textureCopyView,
    bufferCopyView,
    extent3D
    ) => ();
  let copyTextureToTexture: (
    commandEncoder,
    textureCopyView,
    textureCopyView,
    extent3D
    ) => ();
  let beginRenderPass: (commandEncoder, renderPassDescriptor) => renderPass;
  let beginComputePass: commandEncoder => computePass;
  let finishRecording: commandEncoder => commandEncoder;

  let includeSPIRV: string => shaderModuleSource;
};
