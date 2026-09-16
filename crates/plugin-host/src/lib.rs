//! Plugin runtime for the Daemon: loads and runs Host Plugins as WASM.
//!
//! A Plugin is a Papyrus-owned WASM extension. Host Plugins extend a Daemon
//! with tools, files, or Runtime hooks; UI Plugins extend a Client and live
//! in the Client codebase, not here. This crate will embed wasmtime and load
//! components over a WIT interface. Until then it is a stub.
//!
//! First-party Plugins live in `plugins/` at the repo root.

/// Placeholder for the WIT world and bindings shared with plugin authors.
pub mod wit {
    #![allow(unused)]
}

/// Placeholder for the wasmtime store, linker, and plugin lifecycle.
pub mod engine {
    #![allow(unused)]
}
