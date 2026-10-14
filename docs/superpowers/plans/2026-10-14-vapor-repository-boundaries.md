# Vapor Repository Boundaries Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox syntax.

**Goal:** Consolidate shared Vapor capabilities in Vapor and reshape the product repositories into Vapor-CLI, Vapor-GUI, Vapor-Studio, Vapor-Installer, and Vapor-Steam-Entrypoint without losing existing work or changing Steam launch options.

**Architecture:** Vapor owns shared typed models, operations, and providers. Vapor-CLI and Vapor-GUI are the two frontend processes; Studio and Installer contribute product UI modules hosted by Vapor-GUI. Vapor-Steam-Entrypoint remains the app-root-aware dispatcher and Vapor-Installer retains its early-install bootstrap.

**Tech Stack:** Rust 2024, Cargo workspaces managed through Vapor, Bevy/egui runtime integration, eframe GUI, existing Steam and Workshop providers.

**Spec:** `docs/superpowers/specs/2026-10-14-vapor-repository-boundaries-design.md`

## Global Constraints

- Preserve all current parent and submodule work; recovery snapshot: `/tmp/vapor-repo-boundary-preserve-20261014T123835Z`.
- Do not use raw `cargo`, `rustc`, or `rustup`; build and test commands, if later authorized, run through Vapor from each owning root.
- Builds and tests are deferred for this tranche; use static checks only and report that compiler/runtime behavior remains unverified.
- Keep Steam launch options and Steam depots unchanged; no deployment, launch, or publish operation.
- Rename repositories only after source and parent-reference changes are locally reviewable; retain old Git history.
- Rewrite current manifests, source, active docs, tooling, and release configuration; leave historical transcripts and immutable planning source artifacts untouched.
- Keep `Vapor-Platform-Server` and `Vapor-Examples` unchanged except where current build aggregation requires a reference update.

## Review Focus

- A partially migrated SDK command must not execute a duplicate private implementation; all frontends must reach one Vapor operation handler.
- First-install and recovery launches must still work when `vapor-gui` is absent; keep the `vapor-installer` bootstrap route.
- `vapor-entrypoint` must preserve argument forwarding and app-root discovery after the repository rename.
- GUI product crates must not create a Cargo package cycle with `vapor-gui`; product UI crates depend on `vapor_gui_core`, and only the GUI executable assembles them.
- Current authored and untracked changes must survive repo renames, path moves, submodule updates, and SDK archival.

## Task 1: Freeze the migration inventory

**Files:**
- Modify: `docs/superpowers/specs/2026-10-14-vapor-repository-boundaries-design.md`
- Modify: `.agents/workstreams/open/workshop-content-superpass.md`
- Read: all current Cargo/Vapor workspace manifests, `.gitmodules`, build/deploy scripts, active product docs, and frontend launch code

**Produces:** A source-to-destination ledger for every public module in `Vapor-SDK`, `Vapor-Shell`, `Vapor-Launcher`, `Vapor-Installer`, and `Vapor-Entrypoint`; current branch/HEAD/diff/untracked inventory is tied to the recovery snapshot.

- [ ] Map `vapor_sdk_core` modules (`commands`, `content`, `environment`, `options`, `repair`, `root`, `spec`, `steam`, `template`, `toolchain`, `workspace`) to Vapor shared operations/provider modules or Vapor-CLI presentation adapters.
- [ ] Map every `vapor_shell` module to shared service/provider code in Vapor or CLI session/presentation code in Vapor-CLI.
- [ ] Record source and destination for Launcher GUI/core/CLI, Installer bootstrap, Devtools UI, and Entrypoint routing; preserve modules with distinct current ownership until their shared replacement exists.
- [ ] Enumerate live references to old repository names and distinguish them from immutable planning/history material.
- [ ] Run `git diff --check` and verify the preservation snapshot contains every current dirty repository and untracked file.

## Task 2: Move reusable SDK and Shell behavior into Vapor

**Files:**
- Modify: `Vapor/crates/vapor_core/src/**` and `Vapor/crates/vapor_core/Cargo.toml`
- Modify: `Vapor/Cargo.toml` and `Vapor/Workspace.vapor.toml`
- Source: `Vapor-SDK/crates/vapor_sdk_core/src/**`
- Source: backend modules in `Vapor-Shell/crates/vapor_shell/src/**`

**Consumes:** Task 1 source-to-destination ledger.

**Produces:** One Vapor-owned implementation of content authoring, workspace operations, toolchain management, install/acquire, Steam/Workshop distribution, publishing, and runtime launch operations, exposed as typed requests/results. Do not add a dependency from Vapor to a frontend or product repository.

**Shared workspace contract:** `WorkspaceOperationRequest { workspace_root: PathBuf, operation: WorkspaceOperation }` produces structured `WorkspaceOperationEvent` values for progress, success, cancellation, and failure. Non-workspace install/publish/launch actions use the same request/event dispatch pattern with operation-specific payloads.

- [ ] Integrate SDK content/spec/template/workspace behavior into focused Vapor domain modules, reusing existing Vapor manifest, toolchain, content, and operation models where they already own the same concept.
- [ ] Integrate SDK root package/publish and Steam behavior into Vapor distribution operations and provider modules; preserve destination-specific preview/execute boundaries.
- [ ] Integrate Shell backend capabilities (source/app-root state, discovery, content lifecycle, metadata, install, distribution, Steam, and workspace operations) into Vapor services/providers, removing duplicate behavior only after its consumer calls the shared service.
- [ ] Keep CLI command descriptions and GUI presentation out of Vapor; put shared typed intent/result contracts in Vapor only where both frontends or EntryPoint consume them.
- [ ] Resolve crate/module dependencies so the Vapor dependency graph is acyclic and no new crate is introduced solely to preserve an old repository boundary.
- [ ] Run static symbol/reference scans and `git diff --check`; do not compile or run tests in this tranche.

## Task 3: Make Vapor-CLI the single terminal facade

**Files:**
- Source: `Vapor-Shell/crates/vapor_shell/src/{command,prompt,launch_session,main,lib}.rs`
- Source: `Vapor-SDK/crates/vapor_sdk_cli/src/**`
- Source: `Vapor-Launcher/crates/vapor_launcher_cli/src/**`
- Destination: CLI workspace and manifest files in the renamed `Vapor-CLI` checkout

**Consumes:** Task 2 shared Vapor operations.

**Produces:** One `vapor` binary with the interactive REPL and noninteractive command adapters for shared Vapor operations; no standalone SDK or Launcher CLI implementation remains.

- [ ] Merge command vocabularies into the existing Vapor command grammar, preserving source/content/root domains and routing all behavior through Vapor operations.
- [ ] Move only argument parsing, REPL context, prompts, and CLI output into the CLI repository; remove duplicate command handlers after parity is mapped.
- [ ] Preserve the `vapor` executable and existing Vapor-managed workspace commands.
- [ ] Update CLI Cargo and Vapor workspace manifests to depend on the shared Vapor crates by local workspace path.
- [ ] Run static command/reference scans and TOML validation; do not compile or run tests.

## Task 4: Create Vapor-GUI host and mode dispatcher

**Files:**
- Create: `Vapor-GUI/Cargo.toml`, `Vapor-GUI/Workspace.vapor.toml`
- Create: `Vapor-GUI/crates/vapor_gui_core/**`
- Create: `Vapor-GUI/crates/vapor_gui/src/main.rs`
- Modify: `Vapor/Workspace.vapor.toml` and root packaging/binary declarations

**Consumes:** Task 2 shared operation contracts.

**Produces:** `vapor_gui_core` shared GUI lifecycle/components and a `vapor-gui` executable that selects GUI product mode without owning domain behavior. Its public mode is `GuiMode::{Studio, Installer}`; the executable registers product factories and delegates app/window startup to the core crate.

- [ ] Define the GUI mode enum for `studio` and `installer`, argument parsing, operation progress/results, window lifecycle, and product-app construction.
- [ ] Define a product-app interface that Studio and Installer UI crates can implement without depending on the `vapor-gui` binary package.
- [ ] Keep the dependency direction acyclic: product UI crates may depend on `vapor_gui_core`; the GUI executable depends on product UI crates and Vapor operations.
- [ ] Declare the `vapor-gui` binary in the Vapor-managed workspace packaging contract.
- [ ] Run static dependency-graph, manifest, and whitespace checks; no build/test.

## Task 5: Turn Vapor-Studio into the Studio product module

**Files:**
- Modify: `Vapor-Launcher/crates/vapor_launcher_core/**`
- Modify: `Vapor-Launcher/crates/vapor_launcher_gui/**`
- Modify: `Vapor-Launcher/Cargo.toml` and `Workspace.vapor.toml`
- Modify: `Vapor/crates/vapor_devtools/**`

**Consumes:** Tasks 2 and 4 shared operations and GUI product-app interface.

**Produces:** A `vapor_studio_ui` product module for library, compositions, workspace authoring, publishing review, diagnostics, and play selection; its app is assembled by `vapor-gui`.

- [ ] Replace direct GUI calls into Vapor internals and SDK Core with shared typed Vapor operation requests.
- [ ] Split the existing `vapor-studio` binary shell from Studio-specific UI state/surfaces; export the Studio app through `vapor_studio_ui`.
- [ ] Move reusable runtime diagnostics views into the Studio product module while keeping telemetry and runtime instrumentation in Vapor.
- [ ] Remove Launcher CLI/core stubs that duplicate Vapor operation types; retain only Studio-specific profile/composition behavior.
- [ ] Preserve user-authored and untracked Launcher GUI files from the recovery snapshot throughout the rename.
- [ ] Run static import/manifest/reference scans and whitespace checks; no build/test.

## Task 6: Give Vapor-Installer its GUI product module

**Files:**
- Modify: `Vapor-Installer/crates/vapor_installer/**`
- Modify: `Vapor-Installer/Cargo.toml` and `Workspace.vapor.toml`
- Modify: `Vapor-GUI/Cargo.toml` and mode assembly

**Consumes:** Tasks 2 and 4 shared setup operations and GUI product-app interface.

**Produces:** Installer UI assembled under `vapor-gui installer`, with the existing `vapor-installer` executable retained as the early-install/recovery adapter.

- [ ] Keep install/setup/repair operations in Vapor; make the Installer product module gather intent, display progress/recovery, and call shared operations.
- [ ] Preserve the current precompiled bootstrap behavior for app roots where Vapor-GUI or its managed dependencies are not yet available.
- [ ] Connect the Installer product module to Vapor-GUI without making Vapor-GUI depend on Installer implementation details outside the exported app interface.
- [ ] Run static dependency and manifest checks; no build/test.

## Task 7: Rename and route Vapor-Steam-Entrypoint

**Files:**
- Modify: `Vapor-Entrypoint/crates/vapor_entrypoint/src/**`
- Modify: `Vapor-Entrypoint/README.md`, Cargo/Vapor workspace manifests
- Modify: `Vapor-Client/.gitmodules`, launch scripts, staging and smoke guards

**Consumes:** Tasks 3–6 frontend binaries and product modes.

**Produces:** The renamed `Vapor-Steam-Entrypoint` repository retains `vapor-entrypoint`, app-root discovery, logging, argument forwarding, and routes `studio`, `installer`, `shell`, and `play` to the intended frontend/operation.

- [ ] Route `studio` to `vapor-gui studio`, `installer` to `vapor-gui installer` with `vapor-installer` recovery, `shell` to `vapor`, and `play` to selected-composition launch behavior.
- [ ] Keep Steam launch-option strings and depot configuration unchanged; only update the staged binary set and internal dispatch target paths.
- [ ] Update current repo references and local submodule folder/URL while retaining the `vapor-entrypoint` binary name.
- [ ] Run static argument-route, path, and manifest checks; no Steam launch or deployment.

## Task 8: Reconcile the aggregate workspace and current documentation

**Files:**
- Modify: `Vapor-Client/.gitmodules`, `Vapor-Client.vapor.toml`, root build/deploy/staging scripts, current README/docs
- Modify: all affected Vapor, CLI, GUI, Studio, Installer, and Steam-Entrypoint manifests/docs
- Modify: `resources/vapor/tools/_shared/vapor_tools/rustrover.rs`

**Consumes:** Tasks 2–7 final crate and repo names.

**Produces:** The parent Vapor Client workspace can discover and package the frontends, Studio, Installer, host, and entrypoint from their new locations; current tools/docs use the new names.

- [ ] Update submodule entries and local relative paths for Vapor-CLI, Vapor-GUI, Vapor-Studio, Vapor-Installer, and Vapor-Steam-Entrypoint.
- [ ] Remove Vapor-SDK from active workspace/submodule/build references only after all modules and consumers have moved.
- [ ] Update current app packaging declarations, developer tooling, scripts, release docs, and Studio design documentation.
- [ ] Leave historical transcripts, intake sources, and planning evidence unchanged; verify each remaining old-name match is historical or an explicit compatibility name.
- [ ] Run `git diff --check`, TOML parsing, and old-reference scans over live files; no build/test.

## Task 9: Apply GitHub repository lifecycle changes

**Files:** GitHub repository metadata and pushed repository refs; parent submodule pointers

**Consumes:** Locally reviewable source migration from Tasks 1–8 and the recovery snapshot.

**Produces:** `Vapor-Shell` renamed to `Vapor-CLI`; `Vapor-Launcher` renamed to `Vapor-Studio`; `Vapor-Entrypoint` renamed to `Vapor-Steam-Entrypoint`; new `Vapor-GUI` repository created; `Vapor-SDK` archived after migration; all active remotes and submodule references point to the new locations.

- [ ] Push the completed preserved repo histories/changes to their renamed destinations and create the `Vapor-GUI` remote only after local references and manifests are coherent.
- [ ] Archive `Vapor-SDK` only after verifying no active source, build, or release dependency remains; retain its Git history and a recovery reference.
- [ ] Update local origin URLs and parent gitlinks; verify each URL/HEAD resolves without changing Steamworks launch configuration.
- [ ] Do not delete remote history or force-push.

## Task 10: Static integration review

**Files:** all changed manifests, code, docs, and parent/submodule references

**Produces:** A reviewed source tree with no active Vapor-SDK ownership, no package cycles, and consistent terminal/GUI/Steam routes. Compiler, automated-test, and runtime behavior remain explicitly unverified in this tranche.

- [ ] Parse every affected TOML manifest and validate declared binary names against source targets.
- [ ] Check dependency direction: Vapor is below both frontends and product modules; `vapor_gui_core` is below Studio/Installer UI; no cross-product dependency cycle exists.
- [ ] Search live files for obsolete repository, crate, binary, and staging names; classify historical matches without rewriting evidence.
- [ ] Check every repo and parent working tree to ensure no original or newly authored changes disappeared.
- [ ] Report the exact static checks, preserved snapshot location, pushed/renamed repositories, and outstanding build/test/runtime verification.
