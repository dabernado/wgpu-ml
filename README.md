# wgpu-ml

`wgpu-ml` is a set of bindings to the WebGPU API for OCaml/ReasonML, with support for both web and native platforms via the OCaml and Bucklescript toolchains. You can think of `wgpu-ml` as being two separate sets of bindings, one for the native OCaml toolchain and one for the JS-targeting Bucklescript toolchain, which expose an API identical to one another. The native bindings compile and link WebGPU via OCaml's C FFI, while the web bindings compile the WebGPU library to WebAssembly, which is then linked to OCaml through Bucklescript/ReasonML's Javascript FFI.

This project is currently in its very early stages.

## Why?

Both the OCaml and ReasonML ecosystems desperately need bindings to a modern graphics API. If you want to work with a low-level graphics API in OCaml, you are limited to the ageing but still very usable OpenGL. As for ReasonML, WebGL bindings are missing entirely. This library provides a consistent, cross-platform interface to a modern, high performance graphics API with all of the safety and speed you would expect of an OCaml/ReasonML library.
