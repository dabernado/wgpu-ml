extern crate wgpu;

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
enum PowerPreference {
    Default,
    LowPower,
    HighPerformance,
}

#[derive(ocaml::ToValue, ocaml::FromValue)]
struct Instance(BackendBit);

/*#[ocaml::func]
pub fn request_adapter(instance: Instance, pow_pref: PowerPreference) -> Adapter {
    let wgpu_backend = match instance {
        Instance(Vulkan) => wgpu::BackendBit::VULKAN,
        Instance(GL) => wgpu::BackendBit::GL,
        Instance(Metal) => wgpu::BackendBit::METAL,
        Instance(DX12) => wgpu::BackendBit::DX12,
        Instance(DX11) => wgpu::BackendBit::DX11,
        Instance(Browser) => wgpu::BackendBit::BROWSER_WEBGPU,
        Instance(Primary) => wgpu::BackendBit::PRIMARY,
        Instance(Secondary) => wgpu::BackendBit::SECONDARY,
    };
    let wgpu_pref = match pow_pref {
        Default => wgpu::PowerPreference::Default,
        LowPower => wgpu::PowerPreference::LowPower,
        HighPerformance => wgpu::PowerPreference::HighPerformance,
    };

    let instance = wgpu::Instance::new(wgpu_backend);
    let surface = unsafe { instance.create_surface(&window) };
}*/
