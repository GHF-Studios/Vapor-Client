# Vapor Single-Home Layout Implementation Plan

> **For agentic workers:** Execute natively in the existing Vapor home checkout. Do not create another source worktree or discard user changes.

**Goal:** Make `VAPOR_HOME` the sole root for Vapor binaries, tools, source, caches, content, configuration, and state, then move Vapor-owned Steam-root data into it.

**Architecture:** Vapor Core owns the canonical root and derives all owned paths from its marker. Installer, CLI, GUI, entrypoint, and shared setup scripts consume the same path contract. Cargo/Rustup and cross tools are pinned below the root; every Vapor-managed build shares `cache/build/<target-triple>`.

**Tech Stack:** Rust workspaces, Cargo/Rustup, Vapor setup scripts, Steam Packagepack staging.

**Spec:** `docs/superpowers/specs/2026-11-10-vapor-single-home-layout.md`

## Global Constraints

- Windows default: `%USERPROFILE%\\Vapor`; Linux default: `$HOME/Vapor`.
- `VAPOR_HOME` names the root, not an application-version folder or Steam app.
- The only local roles remain Player and Developer; Developer owns the complete managed toolchain.
- One managed Rust toolchain lock; `CARGO_HOME` and `RUSTUP_HOME` stay beneath `VAPOR_HOME`.
- One Cargo target cache per target triple beneath `VAPOR_HOME/cache/build`.
- Preserve dirty Git state, user content, and build outputs still needed for this pass.
- Steam receives only the Packagepack manifest, target executable, and verified runtime sidecars.

## Review Focus

- An unset or invalid `VAPOR_HOME` resolves to the documented home without adopting a Steam root or authored workspace.
- A dirty/staged/untracked source checkout is never discarded during consolidation.
- A conflicting Steam-root destination causes a previewed recoverable stop, not overwrite or deletion.
- A build requested for a non-host target uses the matching cache and cross-toolchain, not the host cache.
- Cleanup distinguishes Cargo registry/download cache from Cargo config, installed tools, sources, and state.

---

### Task 1: Establish the canonical checkout and protect authored state

**Files:** `.agents/workstreams/README.md`, `.agents/workstreams/open/standalone-vapor-runtime.md`, Git submodule pins and source roots.

**Deliverable:** One Vapor Client checkout under `VAPOR_HOME/sources` owns the standalone work. All other checkouts' unique commits and dirty diffs are recorded and transferred before removal. Do not delete any checkout in this task.

### Task 2: Make one root contract in Vapor Core

**Files:** `Vapor/crates/vapor_core/src/installation.rs`, `Vapor/crates/vapor_core/src/toolchain.rs`, root marker/manifest, public SDK exports.

**Deliverable:** A typed home value resolves `bin`, `tools`, `sources`, `cache`, `content`, `config`, `data`, and `output` from one root. The root marker and `VAPOR_HOME` are validated; no AppData or Steam-root fallback remains for Vapor-owned paths.

### Task 3: Route every managed Cargo/Rust operation through the home

**Files:** `Vapor/crates/vapor_core/src/development.rs`, `cargo.rs`, `run.rs`, `toolchain.rs`, `cross_target_toolchain.rs`; CLI managed-cargo commands; IDE/setup scripts under `resources/vapor/tools/**`.

**Deliverable:** Managed Rustup, Cargo, Rustfmt, Clippy, linker, and cross-tool invocations receive the one lock and `VAPOR_HOME` paths. All build/check/run/release operations use the shared target cache with explicit target selection.

### Task 4: Install and launch Vapor from its home

**Files:** Vapor Installer install/update/uninstall code; `Vapor-Steam-Entrypoint`; CLI/GUI launch discovery; Windows/Linux setup integration.

**Deliverable:** Vapor binaries install under `VAPOR_HOME/bin/<host-triple>`. Steam product launch discovers the separately installed Vapor home for tools handoff without installing Vapor or storing Vapor files in the Steam folder.

### Task 5: Move Vapor-owned Steam-root folders transactionally

**Files:** Installer migration/preview/rollback code; exact Steam-root ownership inventory; storage report.

**Deliverable:** Preview exact Steam-root paths, sizes, conflicts, and destinations. Copy to staging under the home, verify hashes and expected manifests, record the operation, then remove original Vapor-owned paths only after verification. Leave the Packagepack payload untouched.

### Task 6: Reconcile duplicate homes and reclaim space

**Files:** source workspace manager, toolchain/storage reporting, migration ledger, cleanup operation categories.

**Deliverable:** Compare duplicate checkouts and their Git state; compare toolchain versions and Cargo caches; preserve one canonical set under the home. Delete only verified duplicates or regenerable target/download outputs no longer in use. Keep unrelated global tools.

### Task 7: Verify target builds and update the operating docs

**Files:** Windows/Linux build workflows, installer/CLI/setup docs, storage tool, standalone workstream.

**Deliverable:** Build both supported targets with the Vapor-managed toolchain, inspect Packagepack staging, and report exact paths and remaining runtime checks. Do not run tests unless separately requested.
