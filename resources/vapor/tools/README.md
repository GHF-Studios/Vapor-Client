# Vapor home tools

This source payload is installed under:

```text
<vapor-home>/resources/vapor/tools/
```

The Vapor home owns the runtime, managed tools, source checkouts, caches, and
mutable state. A SuperWorkspace is authored separately and does not own Vapor
tools or the runtime.

## SuperWorkspace setup

Run these from the installed Vapor home:

```text
<vapor-home>/bin/<target>/vapor-installer superworkspace create --path /path/to/SuperWorkspace
<vapor-home>/bin/<target>/vapor-installer source clone Vapor-Client --super-workspace /path/to/SuperWorkspace
<vapor-home>/bin/<target>/vapor-installer source clone Vapor-Platform-Server --revision <full-commit-sha>
<vapor-home>/bin/<target>/vapor-installer ide patch-rustrover --super-workspace /path/to/SuperWorkspace
```

Clone only the sources needed for the current task. The SuperWorkspace stores
checkouts under `sources/`; its manifest records the independently installed
Vapor home that owns the toolchain and generated state.
`--revision` fetches and checks out the requested full commit ID before
initializing that revision's submodules, so detached CI checkouts stay under
the same Vapor-owned source root.

## Player and Developer setup

```text
<vapor-home>/bin/<target>/vapor-installer status
<vapor-home>/bin/<target>/vapor-installer install
<vapor-home>/bin/<target>/vapor-installer dev-env install
```

`clone.rs` performs the source clone operation internally. Git is the transport,
not the public workflow.

## App root and Vapor toolchain

App setup commands run from the shipped app-root tool tree:

```text
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/status.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/setup_player.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/setup_development.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/teardown_development.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/teardown_player.rs
```

`setup_player.rs` creates Player state in Vapor User Data; it needs no developer toolchain.
`setup_development.rs` installs/reconciles Rust/Cargo/rustup, Rust
targets/components, SteamCMD, Zig, llvm-mingw, and cross-linker wrappers in
Vapor User Data. The Steam app root contains the launch and setup entry points.

## RustRover

`patch_rustrover.rs` writes repairable `.idea` metadata and run configurations
inside the selected SuperWorkspace. Its runner, binaries, Rust toolchain, and
cross-linkers are resolved from the Vapor home.
