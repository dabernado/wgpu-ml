external Instance: backendBit => instance = "get_instance";
external requestAdapter: (instance, surface, powerPreference) => adapter = "request_adapter";
external enumerateAdapters: (instance, backendBit) => adapter list = "enumerate_adapters";
external createSurface: window => surface = "create_surface";
