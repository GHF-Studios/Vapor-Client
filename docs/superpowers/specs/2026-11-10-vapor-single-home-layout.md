# Vapor Single-Home Layout

**Status:** Owner-approved direction  
**Date:** 2026-11-10

## Intent

Make `VAPOR_HOME` the one root for every Vapor-owned file on a machine. The
Windows default is `%USERPROFILE%\\Vapor`; the Linux default is `$HOME/Vapor`.
Application binaries, toolchains, source checkouts, build outputs, caches,
content, settings, and state all live below that root. Steam installs product
content; it does not install Vapor or own Vapor state.

The work is a location and ownership correction: move existing Vapor-owned
trees out of the Loo Cast Steam folder, route all Vapor writers to the home,
consolidate duplicate checkouts and managed toolchains, and remove only verified
duplicates or regenerable outputs after their replacements are usable.

## Owner decisions

- Use one `VAPOR_HOME` root on Windows and Linux, rather than separate OS data,
  roaming, and cache roots.
- Keep platform and ABI differences inside that root. Build output is separated
  by target triple; tool binaries remain host-specific.
- Keep one managed Rust toolchain lock and Vapor-owned Cargo and Rustup homes.
- Keep one Cargo build cache per target triple under the Vapor home.
- Keep source checkouts under one Vapor-owned `sources` tree. Preserve unique
  staged, unstaged, and untracked work until it is committed or deliberately
  transferred.
- The Loo Cast Steam depot contains its Packagepack manifest, one primary
  Packagepack executable per supported target, and only verified runtime
  sidecars. No Vapor executable, toolchain, checkout, cache, or mutable state
  belongs there.
- Vapor cleanup may remove only exact, inventoried Vapor paths. Build outputs
  currently needed for this pass remain until replacement builds are verified.

## Canonical layout

```text
VAPOR_HOME/
  Vapor-Home.toml             root identity and schema
  bin/<host-triple>/          Vapor application entrypoints
  tools/
    cargo/home/               managed Cargo home and tool-installed binaries
    rustup/home/              pinned Rust toolchain and Rustup state
    cross/<host>-<target>/    host-specific cross-compilers and SDKs
    ...                       other Vapor-managed tools
  sources/                    the canonical multi-repository workspace
  cache/
    build/<target-triple>/    shared Cargo target directory per target
    downloads/
    extract/
  content/                    acquired and mounted Vapor content
  config/                     Vapor settings and install records
  data/
    state/
    logs/
    diagnostics/
  output/                     named release/package outputs, not Cargo cache
```

`CARGO_HOME` is `VAPOR_HOME/tools/cargo/home`; `RUSTUP_HOME` is
`VAPOR_HOME/tools/rustup/home`. Cargo registry and Git caches remain beneath
the managed Cargo home and receive explicit cleanup categories, so deleting
them never deletes Cargo configuration or installed Cargo tools.
Vapor-managed builds set `CARGO_TARGET_DIR` beneath
`VAPOR_HOME/cache/build/<target-triple>` and use that target consistently
across source checkouts.

The application root is the Vapor home itself. The running executable is
resolved from `VAPOR_HOME/bin/<host-triple>`; no Steam-root or AppData fallback
is a supported installation location. OS-level launcher registration may point
into the home but stores no Vapor-owned files outside it.

## Approaches considered

1. **OS-native split roots:** rejected by the owner. It scatters one product's
   files across Local, Roaming, XDG data, and XDG cache locations.
2. **One root with per-machine platform/target subfolders:** selected. It keeps
   install, backup, cleanup, and recovery simple while preserving ABI isolation
   inside the root.
3. **Leave Rustup/Cargo in their user-default homes:** rejected. The machine
   currently uses Rust 1.99 from `.cargo/.rustup`, while Vapor has a separate
   1.97 toolchain and multiple Cargo homes; this creates the observed cache and
   toolchain duplication.

## Goals

- Every Vapor-owned persistent file is under one `VAPOR_HOME`.
- Vapor-managed Rust commands use the pinned toolchain, Cargo home, Rustup
  home, and target cache regardless of which Vapor source checkout launched
  them.
- Each supported target triple has one shared build-cache location.
- The Loo Cast Steam root contains only the Packagepack product payload and
  explicitly required Steam metadata/runtime sidecars.
- Moving Steam-root data is previewable, conflict-aware, and recoverable; no
  source is removed before its destination is verified.
- Duplicate sources and caches are retired only after authored Git state and
  replacement data are verified.

## Non-goals

- Do not relocate unrelated user projects or globally installed Rust tools
  unless they are proven Vapor-owned.
- Do not delete active build output until the replacement target cache is
  verified and the build owner confirms it is no longer needed.
- Do not add compatibility discovery for multiple long-term Vapor homes.
- Do not move Loo Cast or Mad Max game files into Vapor-hosted distribution.

## Required implementation order

1. Reconcile the dirty Vapor Client checkouts and select one canonical
   `VAPOR_HOME/sources/Vapor-Client` source tree without losing staged,
   unstaged, or untracked work.
2. Make the root marker and `VAPOR_HOME` path contract authoritative in Core,
   Installer, CLI, GUI, entrypoint, and setup scripts.
3. Route Cargo, Rustup, cross-compilers, source clones, and all Cargo builds to
   their canonical children under the root. Remove implicit system Cargo and
   Steam-root fallbacks from Vapor-managed operations.
4. Migrate Vapor-owned Steam-root folders into the home with an exact path
   preview, collision checks, copy/verify/commit phases, and rollback record.
5. Verify the Packagepack-only Steam staging output and each supported target.
6. Compare duplicate Cargo/Rustup registries, toolchains, source trees, and
   targets. Retire exact duplicates and obsolete generated output to recover
   space; retain unrelated Rust installations and any user-authored content.
7. Update the standalone lifecycle docs and storage cleanup UI/CLI to report
   this single-root model.
