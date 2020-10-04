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

  // descriptor types
  type deviceDescriptor;
  type limits;

  // faux-type constructors
  let Instance: backendBit => instance;

  // constants
  let defaultLimits: limits;

  // functions
  let requestAdapter: (instance, surface, powerPreference) => adapter;
  let enumerateAdapters: (instance, backendBit) => adapter list;
  let createSurface: (instance, window) => surface;

  let requestDevice: (adapter, deviceDescriptor) => (device, queue);
};
