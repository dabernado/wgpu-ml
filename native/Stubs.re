external requestAdapter: (
  instance,
  surface,
  powerPreference
  ) => adapter = "request_adapter";
external enumerateAdapters: (
  instance,
  backendBit
  ) => adapter list = "enumerate_adapters";
external Instance: backendBit => instance = "get_instance";
external createSurface: window => surface = "create_surface";

external requestDevice: (
  adapter,
  deviceDescriptor,
  string option
  ) => (device, queue) = "request_device";

external createShaderModule: (
  device,
  shaderModuleSource
  ) => shaderModule = "create_shader_module";
external createCommandEncoder: (
  device,
  commandEncoderDescriptor
  ) => commandEncoder = "create_command_encoder";

external copyBufferToBuffer: (
  commandEncoder,
  (buffer, int64),
  (buffer, int64),
  int64,
  ) => () = "copy_buffer_to_buffer";
external copyBufferToTexture: (
  commandEncoder,
  bufferCopyView,
  textureCopyView,
  extent3D,
  ) => () = "copy_buffer_to_texture";
external copyTextureToBuffer: (
  commandEncoder,
  bufferCopyView,
  textureCopyView,
  extent3D,
  ) => () = "copy_texture_to_buffer";
external copyTextureToTexture: (
  commandEncoder,
  textureCopyView,
  textureCopyView,
  extent3D,
  ) => () = "copy_texture_to_texture";
external beginRenderPass: (
  commandEncoder,
  renderPassDescriptor
  ) => renderPass = "begin_render_pass";
external beginComputePass: commandEncoder => computePass = "begin_compute_pass";
external finishRecording: commandEncoder => commandEncoder = "finish_recording";

external includeSPIRV: string => shaderModuleSource = "include_spirv";
