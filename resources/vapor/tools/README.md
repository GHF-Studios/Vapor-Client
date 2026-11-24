# Vapor app-root tools

This tree is source payload for tools shipped into:

```text
<app-root>/resources/vapor/tools/
```

At runtime, the app-root copy is canonical. A SuperWorkspace does not own tools
and should not contain a `tools/` implementation tree. Source repositories are
cloned into `sources/` by the app-root source clone tool.

## Reproducible SuperWorkspace flow

Run these from the installed app-root tool tree:

```text
rust-script --force <app-root>/resources/vapor/tools/development/superworkspace/create.rs --path /path/to/SuperWorkspace
rust-script --force <app-root>/resources/vapor/tools/development/source_setup/clone.rs --super-workspace /path/to/SuperWorkspace --all
rust-script --force <app-root>/resources/vapor/tools/development/ide_setup/patch_rustrover.rs --super-workspace /path/to/SuperWorkspace
```

Then open `/path/to/SuperWorkspace` in RustRover.

The order matters: patch RustRover before opening it. The patcher writes
repairable `.idea` metadata and run configurations; it does not rely on
RustRover already having a valid project model.

## Shape

```text
SuperWorkspace/
  SuperWorkspace.vapor.toml
  sources/
    Vapor-Client/
    Loo-Cast/
    Vapor-Registry/
    Vapor-Platform-Server/
  .idea/
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

`patch_rustrover.rs` writes:

- `.idea/cargoProjects.xml`;
- `.idea/workspace.xml` Cargo/Rust project components;
- `.idea/rust.xml`;
- `.idea/vapor.xml`;
- `.idea/vapor-toolchain/bin/*` wrappers;
- `.idea/runConfigurations/*.xml`.

It does not generate workflow scripts under `.idea`. Run configurations are
thin XML launchers into:

```text
<app-root>/resources/vapor/tools/development/ide_run/run.rs
```

The IDE runner then calls app-root `vapor`, app-root tools, and the app-root
toolchain.

Publishing workflows prompt inside the terminal and require typed confirmation
before passing `--yes` to Vapor Shell.
