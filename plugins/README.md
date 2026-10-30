# Papyrus Plugins

First-party Plugins live here once they exist.

A Plugin is a Papyrus-owned WASM extension, distinct from plugins that belong
to a Runtime. Two kinds:

- Host Plugins run in the Daemon: tools, files, Runtime hooks.
- UI Plugins run in the Client: views, commands, keymaps.

Author them with packages/plugin-sdk against the WIT world in
crates/plugin-host. Nothing here yet.
