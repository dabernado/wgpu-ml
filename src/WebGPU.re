include Stubs;

/*
 * Variants
 */
type backendBit =
  | Vulkan
  | GL
  | Metal
  | DX12
  | DX11
  | Browser
  | Primary
  | Secondary;

type window =
  | MacOS
  | Xlib
  | Xcb
  | Wayland
  | Windows
  | Web;

type powerPreference = Default | LowPower | HighPerformance;

/*
 * Custom types
 */
type instance;
type surface;
//type adapter;
