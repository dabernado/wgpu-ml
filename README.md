# wgpu-ml

`wgpu-ml` is a set of bindings to the WebGPU API for OCaml/ReasonML, with support for both web and native platforms via the OCaml and Bucklescript toolchains. You can think of `wgpu-ml` as being two separate sets of bindings, one for the native OCaml toolchain and one for the JS-targeting Bucklescript toolchain, which expose an API identical to one another. The native bindings compile and link WebGPU via OCaml's C FFI, while the web bindings compile the WebGPU library to WebAssembly, which is then linked to OCaml through Bucklescript/ReasonML's Javascript FFI.

This project is currently in its very early stages.
