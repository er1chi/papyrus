//! Scribe: Papyrus's own coding agent. A Runtime like Pi and OpenCode, so it
//! runs on the Host as its own process, spawned by the Daemon.
//!
//! This binary must stand alone: it never depends on `papyrus-host` or any
//! Client toolkit. The Daemon drives it through the adapter in
//! `papyrus-runtime`.

fn main() {
    println!("scribe: coding agent stub. Waits for a Daemon to drive it.");
}
