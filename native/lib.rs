extern crate wgpu;
extern crate ocaml;
extern crate raw_window_handle;
use raw_window_handle::RawWindowHandle;

#[cfg(target_os = "macos")]
use raw_window_handle::macos::MacOSHandle;
#[cfg(target_os = "windows")]
use raw_window_handle::windows::WindowsHandle;
#[cfg(target_os = "linux")]
use raw_window_handle::unix::{XlibHandle, XcbHandle, WaylandHandle};

struct Instance(wgpu::Instance);
struct Surface(wgpu::Surface);
ocaml::custom! (Instance);
ocaml::custom! (Surface);

#[derive(ocaml::ToValue, ocaml::FromValue)]
enum BackendBit {
    Vulkan,
    GL,
    Metal,
    DX12,
    DX11,
    Browser,
    Primary,
    Secondary,
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
enum Window {
    MacOS,
    Xlib,
    Xcb,
    Wayland,
    Windows,
}

unsafe impl raw_window_handle::HasRawWindowHandle for Window {
    #[cfg(target_os = "macos")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        match self {
            MacOS => RawWindowHandle::MacOS(
                    MacOSHandle { ..MacOSHandle::empty() }
                ),
            _ => panic!("Unsupported windowing platform")
        }
    }
    
    #[cfg(target_os = "windows")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        match self {
            Windows => RawWindowHandle::Windows(
                    WindowsHandle { ..WindowsHandle::empty() }
                ),
            _ => panic!("Unsupported windowing platform")
        }
    }
    
    #[cfg(target_os = "linux")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        match self {
            Window::Xlib => RawWindowHandle::Xlib(
                    XlibHandle { ..XlibHandle::empty() }
                ),
            Window::Xcb => RawWindowHandle::Xcb(
                    XcbHandle { ..XcbHandle::empty() }
                ),
            Window::Wayland => RawWindowHandle::Wayland(
                    WaylandHandle { ..WaylandHandle::empty() }
                ),
            _ => panic!("Unsupported windowing platform")
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
enum PowerPreference {
    Default,
    LowPower,
    HighPerformance,
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn get_instance(backend: BackendBit) -> Instance {
    let wgpu_backend = match backend {
        Vulkan => wgpu::BackendBit::VULKAN,
        GL => wgpu::BackendBit::GL,
        Metal => wgpu::BackendBit::METAL,
        DX12 => wgpu::BackendBit::DX12,
        DX11 => wgpu::BackendBit::DX11,
        Browser => wgpu::BackendBit::BROWSER_WEBGPU,
        Primary => wgpu::BackendBit::PRIMARY,
        Secondary => wgpu::BackendBit::SECONDARY,
    };

    Instance(wgpu::Instance::new(wgpu_backend))
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn create_surface(instance: ocaml::Pointer<Instance>, window: Window) -> wgpu::Surface {
    Surface(instance.as_ref().0.create_surface(&window))
}
