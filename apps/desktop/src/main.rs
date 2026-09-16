//! Desktop Client: a GPUI shell that attaches to a Daemon.
//!
//! A Client is a user-facing surface that talks to a Daemon; Desktop is the
//! first one. This binary stays thin: windows, input, rendering. Session and
//! Host logic belongs to crates (protocol, host, runtime), not here.
//!
//! Stub: opens a window with a placeholder label. No Daemon attachment yet.

fn main() {
    println!("papyrus-desktop: Client stub. Talks to a Daemon once protocol exists.");
}
