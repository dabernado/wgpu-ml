[@bs.module "./wgpu"] external Instance: backendBit => instance = "getInstance";
[@bs.module "./wgpu"] external requestAdapter: (instance, surface, powerPreference) => adapter = "requestAdapter";
[@bs.module "./wgpu"] external enumerateAdapters: (instance, backendBit) => adapter list = "enumerateAdapters";
[@bs.module "./wgpu"] external createSurface: window => surface = "createSurface";

[@bs.module "./wgpu"] external requestDevice: (adapter, deviceDescriptor, string option) => (device, queue) = "requestDevice";
