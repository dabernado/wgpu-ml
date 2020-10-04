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
struct Adapter(wgpu::Adapter);
ocaml::custom! (Instance);
ocaml::custom! (Surface);
ocaml::custom! (Adapter);

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

impl BackendBit {
    fn as_wgpu(&self) -> wgpu::BackendBit {
        match self {
            BackendBit::Vulkan => wgpu::BackendBit::VULKAN,
            BackendBit::GL => wgpu::BackendBit::GL,
            BackendBit::Metal => wgpu::BackendBit::METAL,
            BackendBit::DX12 => wgpu::BackendBit::DX12,
            BackendBit::DX11 => wgpu::BackendBit::DX11,
            BackendBit::Browser => wgpu::BackendBit::BROWSER_WEBGPU,
            BackendBit::Primary => wgpu::BackendBit::PRIMARY,
            BackendBit::Secondary => wgpu::BackendBit::SECONDARY,
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
enum Window {
    MacOS,
    Xlib,
    Xcb,
    Wayland,
    Windows,
    Web,
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

impl PowerPreference {
    fn as_wgpu(&self) -> wgpu::PowerPreference {
        match self {
            PowerPreference::Default => wgpu::PowerPreference::Default,
            PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn get_instance(backend: BackendBit) -> Instance {
    Instance(wgpu::Instance::new(wgpu_backend.as_wgpu()))
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn create_surface(instance: ocaml::Pointer<Instance>, window: Window) -> wgpu::Surface {
    Surface(instance.as_ref().0.create_surface(&window))
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn request_adapter(
    instance: ocaml::Pointer<Instance>,
    surface: ocaml::Pointer<Surface>,
    power: PowerPreference
) -> Adapter {
    let w_inst = instance.as_ref().0;
    let w_surf = surface.as_ref().0;

    Adapter(
        w_inst.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: power.as_wgpu(),
            compatible_surface: Some(&w_surf),
        })
        .await
        .expect("Failed to find an appropriate adapter")
    )
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn enumerate_adapters(instance: ocaml::Pointer<Instance>, backend: BackendBit) -> ocaml::List<Adapter> {
    let w_inst = instance.as_ref().0;
    let list = ocaml::List::empty();
    let adapters = w_inst.enumerate_adapters(backend.as_wgpu());

    for x in adapters { list.add(Adapter(x)); };
    list
}
