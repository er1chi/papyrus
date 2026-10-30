# @papyrus/plugin-sdk

Stub. Will hold the toolchain that compiles TypeScript Plugins to WASM.

A Plugin is a Papyrus-owned WASM extension. Host Plugins extend a Daemon
(tools, files, Runtime hooks); UI Plugins extend a Client (views, commands,
keymaps). The SDK will compile both from TypeScript, against the WIT world
that crates/plugin-host defines.

Not implemented. No WIT, no wit-bindgen, no wasm-pack yet.
