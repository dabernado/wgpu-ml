[@bs.module "./wgpu"] external requestAdapter: (
  instance,
  surface,
  powerPreference
  ) => adapter = "requestAdapter";
[@bs.module "./wgpu"] external enumerateAdapters: (
  instance,
  backendBit
  ) => adapter list = "enumerateAdapters";
[@bs.module "./wgpu"] external Instance: backendBit => instance = "getInstance";
[@bs.module "./wgpu"] external createSurface: window => surface = "createSurface";

[@bs.module "./wgpu"] external requestDevice: (
  adapter,
  deviceDescriptor,
  string option
  ) => (device, queue) = "requestDevice";

[@bs.module "./wgpu"] external createShaderModule: (
  device,
  shaderModuleSource
  ) => shaderModule = "createShaderModule";
[@bs.module "./wgpu"] external createCommandEncoder: (
  device,
  commandEncoderDescriptor
  ) => commandEncoder = "createCommandEncoder";

[@bs.module "./wgpu"] external copyBufferToBuffer: (
  commandEncoder,
  (buffer, int64),
  (buffer, int64),
  int64,
  ) => () = "copyBufferToBuffer";
[@bs.module "./wgpu"] external copyBufferToTexture: (
  commandEncoder,
  bufferCopyView,
  textureCopyView,
  extent3D,
  ) => () = "copyBufferToTexture";
[@bs.module "./wgpu"] external copyTextureToBuffer: (
  commandEncoder,
  bufferCopyView,
  textureCopyView,
  extent3D,
  ) => () = "copyTextureToBuffer";
[@bs.module "./wgpu"] external copyTextureToTexture: (
  commandEncoder,
  textureCopyView,
  textureCopyView,
  extent3D,
  ) => () = "copyTextureToTexture";
[@bs.module "./wgpu"] external beginRenderPass: (
  commandEncoder,
  renderPassDescriptor
  ) => renderPass = "beginRenderPass";
[@bs.module "./wgpu"] external beginComputePass: commandEncoder => computePass = "beginComputePass";
[@bs.module "./wgpu"] external finishRecording: commandEncoder => commandEncoder = "finishRecording";

[@bs.module "./wgpu"] external includeSPIRV: string => shaderModuleSource = "includeSPIRV";
