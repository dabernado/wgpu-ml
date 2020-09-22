include Stubs;

type backendBit =
    Vulkan
  | GL
  | Metal
  | DX12
  | DX11
  | Browser
  | Primary
  | Secondary;

type powerPreference = Default | LowPower | HighPerformance;

type instance = Instance backendBit;

type adapter;
