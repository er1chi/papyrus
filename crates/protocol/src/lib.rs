//! Wire types shared by every Client and Daemon.
//!
//! A Client talks to a Daemon on a Host, including over localhost, so both
//! sides must agree on one vocabulary. This crate holds that vocabulary:
//! request/response envelopes, event streams, and capability declarations.
//!
//! Nothing here knows about transport (TCP, stdio, websockets) or about any
//! particular product feature. Keep it dependency-light so a Mobile Client
//! can adopt it later.

/// Placeholder module for the Client-Daemon message envelope.
///
/// Replaced once the first wire operations exist. The shape to aim for is a
/// tagged enum so a single connection carries every message kind.
pub mod envelope {
    #![allow(unused)]
}

/// Placeholder module for connection setup: attach, reattach, Session identity.
///
/// A Session is one Client attached to one Daemon; reattach stays on that
/// Daemon, and another Host is another Session.
pub mod session {
    #![allow(unused)]
}
