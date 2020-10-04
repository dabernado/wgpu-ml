external Instance: backendBit => instance = "get_instance";
external requestAdapter: (instance, surface, powerPreference) => adapter = "request_adapter";
external enumerateAdapters: (instance, backendBit) => adapter list = "enumerate_adapters";
external createSurface: window => surface = "create_surface";

external requestDevice: (adapter, deviceDescriptor, string option) => (device, queue) = "request_device";

external createShaderModule: (device, shaderModuleSource) => shaderModule = "create_shader_module";

external includeSPIRV: string => shaderModuleSource = "include_spirv";
