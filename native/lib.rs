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
struct Device(wgpu::Device);
struct Queue(wgpu::Queue);
struct ShaderModule(wgpu::ShaderModule);
struct ShaderModuleSource<'a>(wgpu::ShaderModuleSource<'a>);
struct CommandEncoder(wgpu::CommandEncoder);
struct RenderPass<'a>(wgpu::RenderPass<'a>);
struct ComputePass<'a>(wgpu::ComputePass<'a>);
struct Buffer(wgpu::Buffer);
struct Texture(wgpu::Texture);
struct TextureView(wgpu::TextureView);
ocaml::custom! (Instance);
ocaml::custom! (Surface);
ocaml::custom! (Adapter);
ocaml::custom! (Device);
ocaml::custom! (Queue);
ocaml::custom! (ShaderModule);
ocaml::custom! (ShaderModuleSource<'a>);
ocaml::custom! (CommandEncoder);
ocaml::custom! (RenderPass<'a>);
ocaml::custom! (ComputePass<'a>);
ocaml::custom! (Buffer);
ocaml::custom! (Texture);
ocaml::custom! (TextureView);

trait ToRust<T> {
    fn as_rust(self) -> T;
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
    fn as_rust(self) -> wgpu::BackendBit {
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
    fn as_rust(self) -> wgpu::PowerPreference {
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
    fn as_rust(self) -> wgpu::Features {
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
enum LoadOp<T> {
    Clear(T),
    Load,
}

impl<T> ToRust<wgpu::LoadOp<T>> for LoadOp<T> {
    fn as_rust(self) -> wgpu::LoadOp<T> {
        match self {
            LoadOp::Clear(x) => wgpu::LoadOp::Clear(x),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct Limits {
  max_bind_groups: u32,
  max_dynamic_uniform_buffers_per_pipeline_layout: u32,
  max_dynamic_storage_buffers_per_pipeline_layout: u32,
  max_sampled_textures_per_shader_stage: u32,
  max_samplers_per_shader_stage: u32,
  max_storage_buffers_per_shader_stage: u32,
  max_storage_textures_per_shader_stage: u32,
  max_uniform_buffers_per_shader_stage: u32,
  max_uniform_buffer_binding_size: u32,
  max_push_constant_size: u32,
}

impl ToRust<wgpu::Limits> for Limits {
    fn as_rust(self) -> wgpu::Limits {
        wgpu::Limits {
            max_bind_groups: self.max_bind_groups,
            max_dynamic_uniform_buffers_per_pipeline_layout: self.max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: self.max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: self.max_sampled_textures_per_shader_stage,
            max_samplers_per_shader_stage: self.max_samplers_per_shader_stage,
            max_storage_buffers_per_shader_stage: self.max_storage_buffers_per_shader_stage,
            max_storage_textures_per_shader_stage: self.max_storage_textures_per_shader_stage,
            max_uniform_buffers_per_shader_stage: self.max_uniform_buffers_per_shader_stage,
            max_uniform_buffer_binding_size: self.max_uniform_buffer_binding_size,
            max_push_constant_size: self.max_push_constant_size,
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
    fn as_rust(self) -> wgpu::DeviceDescriptor {
        wgpu::DeviceDescriptor {
            features: self.features.as_rust(),
            limits: self.limits.as_rust(),
            shader_validation: self.shader_validation,
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct Operations<T> {
    load: LoadOp<T>,
    store: bool,
}

impl<T> ToRust<wgpu::Operations<T>> for Operations<T> {
    fn as_rust(self) -> wgpu::Operations<T> {
        wgpu::Operations {
            load: self.load.as_rust(),
            store: self.store,
        }
    }
}

impl ToRust<wgpu::Operations<wgpu::Color>> for Operations<(f64, f64, f64, f64)> {
    fn as_rust(self) -> wgpu::Operations<wgpu::Color> {
        wgpu::Operations {
            load: match self.load {
                LoadOp::Clear((r, g, b, a)) => wgpu::LoadOp::Clear(wgpu::Color {r, g, b, a}),
                LoadOp::Load => wgpu::LoadOp::Load,
            },
            store: self.store,
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct RenderPassColorAttachmentDescriptor {
    attachment: ocaml::Pointer<TextureView>,
    resolve_target: Option<ocaml::Pointer<TextureView>>,
    ops: Operations<(f64, f64, f64, f64)>,
}

/*impl<'a> ToRust<wgpu::RenderPassColorAttachmentDescriptor<'a>> for RenderPassColorAttachmentDescriptor {
    fn as_rust(self) -> wgpu::RenderPassColorAttachmentDescriptor<'a> {
        wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &self.attachment.as_ref().0,
            resolve_target: match self.resolve_target {
                Some(x) => Some(&x.as_ref().0),
                None => None,
            },
            ops: self.ops.as_rust(),
        }
    }
}*/

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct RenderPassDepthStencilAttachmentDescriptor {
    attachment: ocaml::Pointer<TextureView>,
    depth_ops: Option<Operations<f32>>,
    stencil_ops: Option<Operations<u32>>,
}

/*impl<'a> ToRust<wgpu::RenderPassDepthStencilAttachmentDescriptor<'a>> for RenderPassDepthStencilAttachmentDescriptor {
    fn as_rust(self) -> wgpu::RenderPassDepthStencilAttachmentDescriptor<'a> {
        wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &self.attachment.as_ref().0,
            depth_ops: match self.depth_ops {
                Some(x) => Some(x.as_rust()),
                None => None,
            },
            stencil_ops: match self.stencil_ops {
                Some(x) => Some(x.as_rust()),
                None => None,
            },
        }
    }
}*/

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct RenderPassDescriptor {
    color_attachments: ocaml::List<RenderPassColorAttachmentDescriptor>,
    depth_stencil_attachment: Option<RenderPassDepthStencilAttachmentDescriptor>,
}

/*impl<'a, 'b> ToRust<wgpu::RenderPassDescriptor<'a, 'b>> for RenderPassDescriptor {
    fn as_rust(self) -> wgpu::RenderPassDescriptor<'a, 'b> {
        wgpu::RenderPassDescriptor {
            color_attachments: self.color_attachments
                .to_vec()
                .into_iter()
                .map(|x| x.as_rust())
                .collect::<Vec<wgpu::RenderPassColorAttachmentDescriptor>>()
                .as_slice(),
            depth_stencil_attachment: match self.depth_stencil_attachment {
                Some(x) => Some(x.as_rust()),
                None => None,
            },
        }
    }
}*/

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct TextureDataLayout {
    offset: u64,
    bytes_per_row: u32,
    rows_per_image: u32,
}

impl ToRust<wgpu::TextureDataLayout> for TextureDataLayout {
    fn as_rust(self) -> wgpu::TextureDataLayout {
        wgpu::TextureDataLayout {
            offset: self.offset,
            bytes_per_row: self.bytes_per_row,
            rows_per_image: self.rows_per_image,
        }
    }
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct BufferCopyView {
    buffer: ocaml::Pointer<Buffer>,
    layout: TextureDataLayout,
}

/*impl<'a> ToRust<wgpu::BufferCopyViewBase<&'a wgpu::Buffer>> for BufferCopyView {
    fn as_rust(self) -> wgpu::BufferCopyViewBase<&'a wgpu::Buffer> {
        wgpu::BufferCopyViewBase {
            buffer: &self.buffer.as_ref().0,
            layout: self.layout.as_rust(),
        }
    }
}*/

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct TextureCopyView {
    texture: ocaml::Pointer<Texture>,
    mip_level: u32,
    origin: (u32, u32, u32),
}

/*impl<'a> ToRust<wgpu::TextureCopyViewBase<&'a wgpu::Texture>> for TextureCopyView {
    fn as_rust(self) -> wgpu::TextureCopyViewBase<&'a wgpu::Texture> {
        let (x, y, z) = self.origin;

        wgpu::TextureCopyViewBase {
            texture: &self.texture.as_ref().0,
            mip_level: self.mip_level,
            origin: wgpu::Origin3d {x, y, z},
        }
    }
}*/

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
    trace_path: Option<&str>,
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

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn include_spirv(s: str) -> ShaderModuleSource {
    ShaderModuleSource(wgpu::include_spirv!(s))
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn create_shader_module(
    device: ocaml::Pointer<Device>,
    source: ocaml::Pointer<ShaderModuleSource>,
) -> ShaderModule {
    let w_device = device.as_ref().0;
    let w_source = source.as_ref().0;

    ShaderModule(w_device.create_shader_module(w_source))
}

#[cfg(feature = "derive")]
#[ocaml::func]
pub unsafe fn create_command_encoder(
    device: ocaml::Pointer<Device>,
    desc: Option<&str>,
    ) -> CommandEncoder {
    let w_device = device.as_ref().0;

    CommandEncoder(w_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: desc,
    }))
}
