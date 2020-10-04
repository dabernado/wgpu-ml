extern crate wgpu;
extern crate ocaml;
extern crate raw_window_handle;
use raw_window_handle::RawWindowHandle;
use std::convert::TryFrom;

#[cfg(target_os = "macos")]
use raw_window_handle::macos::MacOSHandle;
#[cfg(target_os = "windows")]
use raw_window_handle::windows::WindowsHandle;
#[cfg(target_os = "linux")]
use raw_window_handle::unix::{XlibHandle, XcbHandle, WaylandHandle};

struct Instance(wgpu::Instance);
struct Surface(wgpu::Surface);
struct Adapter(wgpu::Adapter);
struct Device(wgpu::Device);
struct Queue(wgpu::Queue);
ocaml::custom! (Instance);
ocaml::custom! (Surface);
ocaml::custom! (Adapter);
ocaml::custom! (Device);
ocaml::custom! (Queue);

trait ToRust<T> {
    fn as_rust(&self) -> T;
}

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

impl ToRust<wgpu::BackendBit> for BackendBit {
    fn as_rust(&self) -> wgpu::BackendBit {
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

impl ToRust<wgpu::PowerPreference> for PowerPreference {
    fn as_rust(&self) -> wgpu::PowerPreference {
        match self {
            PowerPreference::Default => wgpu::PowerPreference::Default,
            PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
enum Features {
  DepthClamping,
  TextureCompressionBC,
  MappablePrimaryBuffers,
  SampledTextureBindingArray,
  SampledTextureArrayDynamicIndexing,
  SampledTextureArrayNonUniformIndexing,
  UnsizedBindingArray,
  MultiDrawIndirect,
  MultiDrawIndirectCount,
  PushConstants,
  AllWebGPU,
  AllNative,
  Empty,
}

impl ToRust<wgpu::Features> for Features {
    fn as_rust(&self) -> wgpu::Features {
        match self {
            Features::DepthClamping => wgpu::Features::DEPTH_CLAMPING,
            Features::TextureCompressionBC => wgpu::Features::TEXTURE_COMPRESSION_BC,
            Features::MappablePrimaryBuffers => wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
            Features::SampledTextureBindingArray => wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY,
            Features::SampledTextureArrayDynamicIndexing => wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING,
            Features::SampledTextureArrayNonUniformIndexing => wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING,
            Features::UnsizedBindingArray => wgpu::Features::UNSIZED_BINDING_ARRAY,
            Features::MultiDrawIndirect => wgpu::Features::MULTI_DRAW_INDIRECT,
            Features::MultiDrawIndirectCount => wgpu::Features::MULTI_DRAW_INDIRECT_COUNT,
            Features::PushConstants => wgpu::Features::PUSH_CONSTANTS,
            Features::AllWebGPU => wgpu::Features::ALL_WEBGPU,
            Features::AllNative => wgpu::Features::ALL_NATIVE,
            Features::Empty => wgpu::Features::empty(),
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct Limits {
  max_bind_groups: ocaml::Int,
  max_dynamic_uniform_buffers_per_pipeline_layout: ocaml::Int,
  max_dynamic_storage_buffers_per_pipeline_layout: ocaml::Int,
  max_sampled_textures_per_shader_stage: ocaml::Int,
  max_samplers_per_shader_stage: ocaml::Int,
  max_storage_buffers_per_shader_stage: ocaml::Int,
  max_storage_textures_per_shader_stage: ocaml::Int,
  max_uniform_buffers_per_shader_stage: ocaml::Int,
  max_uniform_buffer_binding_size: ocaml::Int,
  max_push_constant_size: ocaml::Int,
}

impl ToRust<wgpu::Limits> for Limits {
    fn as_rust(&self) -> wgpu::Limits {
        wgpu::Limits {
            max_bind_groups:
                u32::try_from(self.max_bind_groups).unwrap(),
            max_dynamic_uniform_buffers_per_pipeline_layout:
                u32::try_from(self.max_dynamic_uniform_buffers_per_pipeline_layout).unwrap(),
            max_dynamic_storage_buffers_per_pipeline_layout:
                u32::try_from(self.max_dynamic_storage_buffers_per_pipeline_layout).unwrap(),
            max_sampled_textures_per_shader_stage:
                u32::try_from(self.max_sampled_textures_per_shader_stage).unwrap(),
            max_samplers_per_shader_stage:
                u32::try_from(self.max_samplers_per_shader_stage).unwrap(),
            max_storage_buffers_per_shader_stage:
                u32::try_from(self.max_storage_buffers_per_shader_stage).unwrap(),
            max_storage_textures_per_shader_stage:
                u32::try_from(self.max_storage_textures_per_shader_stage).unwrap(),
            max_uniform_buffers_per_shader_stage:
                u32::try_from(self.max_uniform_buffers_per_shader_stage).unwrap(),
            max_uniform_buffer_binding_size:
                u32::try_from(self.max_uniform_buffer_binding_size).unwrap(),
            max_push_constant_size:
                u32::try_from(self.max_push_constant_size).unwrap(),
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct DeviceDescriptor {
    features: Features,
    limits: Limits,
    shader_validation: bool,
}

impl ToRust<wgpu::DeviceDescriptor> for DeviceDescriptor {
    fn as_rust(&self) -> wgpu::DeviceDescriptor {
        wgpu::DeviceDescriptor {
            features: self.features.as_rust(),
            limits: self.limits.as_rust(),
            shader_validation: self.shader_validation,
        }
    }
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn get_instance(backend: BackendBit) -> Instance {
    Instance(wgpu::Instance::new(wgpu_backend.as_rust()))
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn create_surface(
    instance: ocaml::Pointer<Instance>,
    window: window
) -> wgpu::Surface {
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
            power_preference: power.as_rust(),
            compatible_surface: Some(&w_surf),
        })
        .await
        .expect("Failed to find an appropriate adapter")
    )
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn enumerate_adapters(
    instance: ocaml::Pointer<Instance>,
    backend: BackendBit
) -> ocaml::List<Adapter> {
    let w_inst = instance.as_ref().0;
    let list = ocaml::List::empty();
    let adapters = w_inst.enumerate_adapters(backend.as_rust());

    for x in adapters { list.add(Adapter(x)); };
    list
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn request_device(
    adapter: ocaml::Pointer<Adapter>,
    desc: DeviceDescriptor,
    trace_path: Option<str>,
) -> (Device, Queue) {
    let w_adapter = adapter.as_ref().0;
    let tpath = match trace_path {
        Some(s) => Some(&Path::new(s)),
        None => None,
    };

    w_adapter.request_device(&desc.as_rust(), tpath)
        .await
        .expect("Failed to create device")
}
