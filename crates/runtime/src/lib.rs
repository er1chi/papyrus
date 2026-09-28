//! Adapters for agent Runtimes. The first Runtimes are Pi, OpenCode, and
//! Scribe.
//!
//! A Runtime is an agent backend Papyrus can drive. This crate defines the
//! trait the Daemon uses to control one, plus one adapter per Runtime.
//! Runtimes run on the Host, spawned by the Daemon; they never appear in a
//! Client.

/// The trait every Runtime adapter implements. Placeholder until the first
/// operation (spawn, prompt, stream output) is pinned down.
pub trait Runtime {
    /// Name of the Runtime, e.g. "pi", "opencode", or "scribe". Used in logs
    /// and in the wire protocol once capabilities are declared.
    fn name(&self) -> &'static str;
}

/// Placeholder adapter for the Pi Runtime.
pub mod pi {
    #![allow(unused)]
}

/// Placeholder adapter for the OpenCode Runtime.
pub mod opencode {
    #![allow(unused)]
}

/// Placeholder adapter for Scribe, Papyrus's own coding agent (apps/scribe).
pub mod scribe {
    #![allow(unused)]
}
