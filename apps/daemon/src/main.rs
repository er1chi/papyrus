//! Daemon: the process on a Host that owns files, terminals, Runtimes, and
//! the Host interface. A Client always talks to a Daemon, including when both
//! run on the same machine.
//!
//! This binary is a thin entrypoint. Host behavior lives in `papyrus-host`;
//! it must never depend on GPUI or any Client toolkit.

fn main() {
    println!("papyrus-daemon: listening for Clients (stub; no transport yet).");
}
