# codex-protocol

This crate defines the "types" for the protocol used by Codex CLI, including "internal types" for communication between `codex-core` and `codex-tui` and "external types" consumed by clients.

This crate should have minimal dependencies.

Ideally, we should avoid "material business logic" in this crate, as we can always introduce `Ext`-style traits to add functionality to types in other crates.
