//! Host implementation: files, terminals, Runtimes, and the Daemon-side
//! interface. The Daemon binary is a thin entrypoint over this crate.
//!
//! Only the Daemon may depend on this crate. A Client that linked against it
//! would collapse the Client/Daemon split, so keep it out of Desktop and
//! Mobile dependency trees.
//!
//! The vocabulary lives in CONTEXT.md: the Host is the machine whose files,
//! terminals, and agent processes are being controlled.

/// Placeholder for filesystem surface: reads, writes, watches scoped to a
/// Session's roots.
pub mod files {
    #![allow(unused)]
}

/// Placeholder for terminal surface: spawning and driving PTYs on the Host.
pub mod terminals {
    #![allow(unused)]
}

/// Placeholder for the Host interface the Daemon exposes to Clients.
pub mod interface {
    #![allow(unused)]
}
