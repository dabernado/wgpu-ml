module type WebGPU = {
  
  // variants
  type backendBit;
  type powerPreference;
  type window;

  // custom types
  type instance;
  type adapter;
  type surface;

  // faux-type constructors
  let Instance: backendBit => instance;

  // functions
  let requestAdapter: (instance, surface, powerPreference) => adapter;
  let enumerateAdapters: (instance, backendBit) => adapter list;
  let createSurface: (instance, window) => surface;
};
