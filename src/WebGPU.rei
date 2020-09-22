module type WebGPU = {
  type backendBit;
  type powerPreference;
  type instance;
  type adapter;

  let requestAdapter: instance * powerPreference => adapter;
};
