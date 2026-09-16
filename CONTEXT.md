# Papyrus

Papyrus is an Agent Development Environment for Pi and OpenCode. A Client talks to a Daemon on a Host so local and remote work feel like the same thing.

## Language

**Host**:
The machine whose files, terminals, and agent processes are being controlled.
_Avoid_: server, remote, workspace, box

**Client**:
A user-facing surface that talks to a Daemon. Desktop now; Mobile later.
_Avoid_: app, frontend, UI

**Daemon**:
The process on a Host that owns files, terminals, Runtimes, and the Host interface. A Client always talks to a Daemon, including when both run on the same machine.
_Avoid_: server, backend, agent

**Broker**:
A future hosted introducer that lists a user's Hosts and pairs a Client to one Daemon across a network. It does not own files or Runtimes, and it does not sync Hosts.
_Avoid_: server, cloud, relay, sync, control plane

**Control Plane**:
A future product backend for accounts, billing, and Plugin discovery. Not on a Host and not a Broker.
_Avoid_: server, api, backend, cloud

**Runtime**:
An agent backend Papyrus can drive. The first Runtimes are Pi and OpenCode.
_Avoid_: agent, model, backend, harness

**Plugin**:
A Papyrus-owned WASM extension. Distinct from plugins that belong to a Runtime.
_Avoid_: extension, addon

**Host Plugin**:
A Plugin that extends a Daemon (tools, files, Runtime hooks).
_Avoid_: server plugin, backend plugin

**UI Plugin**:
A Plugin that extends a Client (views, commands, keymaps). Desktop and Mobile need not load the same UI Plugins.
_Avoid_: frontend plugin, theme

**Session**:
One Client attached to one Daemon for a stretch of work. Reattach stays on that Daemon; another Host is another Session.
_Avoid_: connection, workspace, project
