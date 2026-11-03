# Standalone Vapor Runtime and Products Implementation Plan

> **For agentic workers:** Use this plan task-by-task. Keep each task inside its named owner repository and preserve unrelated dirty work.

**Goal:** Install and manage Vapor independently from Steam while mounting Loo Cast and Mad Max through product-appropriate local sources.

**Architecture:** Vapor Client owns one standalone home, update lifecycle, and two-role setup. Loo Cast is trusted depot-provided local content plus a locator-only handoff; Mad Max is a locally acquired external game whose integration Packagepack prepares the game without containing licensed files. Server services retain authority over publication rights.

**Tech Stack:** Rust workspaces and Cargo, Vapor Client service/protocol, Vapor Installer, Vapor CLI/setup scripts, Steam depot metadata, signed Artifact releases.

**Spec:** `docs/superpowers/specs/2026-11-03-vapor-standalone-runtime-and-products-design.md`

## Global Constraints

- Vapor installs, updates, repairs, and uninstalls independently from Steam.
- `%USERPROFILE%\\Vapor` or `$HOME/Vapor` owns Vapor binaries, tools, sources, caches, configuration, and mutable state.
- The only local setup roles are `Player` and `Developer`; Developer installs all required development tooling without a skip path.
- Ecosystem developer status is server-side authorization, not a local installation role.
- On Windows, Rust compiler host MSVC and runtime target GNullVM are distinct; use target-specific output and LLVM-MinGW for GNullVM.
- Loo Cast and Mad Max game files are never distributed by Vapor services.
- Do not implement compatibility with old role records or long-term Steam-root layouts; preserve and move authored/Vapor-owned data during the one-time standalone conversion.
- Mad Max integration Packagepack manages loader/bridge setup and contains no game files.
- Use Vapor-managed build/validation commands at the owning workspace; do not invoke raw Cargo.

## Review Focus

- A missing, malformed, or conflicting Vapor-home marker must fail clearly without accidentally adopting an authored workspace or Steam root.
- An interrupted move from the Steam app root must preserve both source and destination data and leave a recoverable operation result.
- A Developer install missing one target compiler must remain visibly incomplete and must not expose a skip/Player downgrade path that mislabels it ready.
- A Windows runtime target must never be mistaken for the Rust compiler host when selecting toolchains, linkers, or build output.
- A user-owned Mad Max executable or mod source must never enter a signed Vapor application release or public game-content upload.

---

## File Structure

- `Vapor/crates/vapor_core/src/installation.rs`: single Vapor-home discovery and owned path derivation.
- `Vapor/crates/vapor_core/src/role.rs` and Client domain/protocol/service role files: exactly two local roles and consistent service contract.
- `Vapor-Installer/crates/vapor_installer/**`: signed release verification, staging, app-root swap/rollback, and tool setup.
- `Vapor-Installer/crates/vapor_installer_ui/src/lib.rs`: Player/Developer setup UI and readiness.
- `Vapor-CLI/crates/vapor_cli/src/**`: standalone paths, product handoff, and setup commands.
- `resources/vapor/tools/production/app_setup/**` and `_shared/**`: shared Player/Developer provisioning.
- `Vapor-Steam-Entrypoint/**`: optional locator only; never provisions Vapor.
- Loo Cast depot product manifests and MM-Building-System adapter remain separate owners and are only referenced through their own implementation plans.
- `AGENTS.md`, `README.md`, active setup docs, and the standalone workstream records: current user-facing/product orientation.

## Task 1: Establish the new workstream and ownership transfer

**Files:** `AGENTS.md`, `.agents/workstreams/README.md`, updater and Workshop workstream records.

**Interfaces:** The standalone runtime workstream owns Vapor home, the role model, and setup/tool lifecycle; the suspended Workshop workstream retains UI/content-authoring work only. No code changes occur before this transfer is recorded.

- [x] Register the standalone runtime/product workstream as foreground and record exact path claims.
- [x] Transfer the role/setup/home paths from the suspended Workshop claim; leave its unrelated content-authoring changes intact.
- [x] Update the Client agent bootstrap orientation to the owner-approved Loo Cast and Mad Max product-source model.
- [x] Mark the prior three-role workflow plan as historical/superseded by this decision; do not add runtime compatibility behavior.

## Task 2: Make Vapor home the sole installation and state root

**Files:** `Vapor/crates/vapor_core/src/installation.rs`, tool/source/cache path providers, Installer setup and uninstall flows.

**Interfaces:** `VaporInstallation::discover()` resolves one Vapor home. Typed path accessors resolve `bin`, `tools/cargo/home`, `tools/rustup/home`, `sources`, `cache`, `content`, `config`, and `state` under that root. Authored workspaces and product roots are distinct from Vapor home.

- [ ] Replace OS-local-data and Steam-root fallback discovery with the single Vapor-home root.
- [ ] Route every managed Cargo, Rustup, LLVM-MinGW/Zig, source, cache, configuration, and state path through that root.
- [ ] Implement an idempotent, transactional one-time move of Vapor-owned directories out of the Loo Cast Steam app root, preserving authored repositories and app content.
- [ ] Remove long-term dual-layout lookup and old Steam-root state behavior.
- [ ] Make uninstall remove only Vapor-owned home data according to explicit user selection; never remove mounted Loo Cast/Mad Max source content.

## Task 3: Collapse local setup roles and make Developer complete

**Files:** `Vapor/crates/vapor_core/src/role.rs`, domain action requirements, protocol snapshots, Client service handlers, Installer UI, setup scripts, CLI messages.

**Interfaces:** The persisted role enum is exactly `Player | Developer`. Developer is a superset of Player setup and requires all managed supported targets/cross-compilers. Server-side ecosystem developer authorization remains an independent Identity decision.

- [ ] Remove Composer, ContentDeveloper, and EcosystemDeveloper from local persisted and protocol role types; do not decode old values.
- [ ] Separate local tooling readiness from server permission and from user-facing workflow grouping.
- [ ] Ensure Player setup installs only runtime/player prerequisites.
- [ ] Ensure Developer setup always provisions both supported targets, pinned Rust toolchain, Cargo, LLVM-MinGW/Zig cross tools, and required authoring tools; provide no skip route.
- [ ] Align Installer first-launch, role-change, and readiness UI with exactly two choices and accurate incomplete/ready states.
- [ ] Ensure CLI and scripts invoke the same service-owned setup and do not offer obsolete role names.

## Task 4: Preserve correct host/runtime target behavior in the standalone layout

**Files:** target mapping, toolchain discovery, managed Cargo, linker setup, release/promotion and content packaging.

**Interfaces:** `host_rust_target()` selects the installed toolchain host; `host_runtime_target()` selects the default package ABI. `x86_64-pc-windows-msvc` is the Windows compiler host and `x86_64-pc-windows-gnullvm` is the packaged Windows runtime target.

- [x] Retain explicit target selection and target-specific build/promotion paths for all runtime targets.
- [ ] Relocate compiler and linker lookup to the sole Vapor home, including Rustup/Cargo homes and LLVM-MinGW.
- [ ] Copy required Windows runtime DLLs from managed tools into the correct staged package without sourcing them from a stale app-root path.
- [x] Ensure selected-workspace deployment cannot select sibling checkout binaries.

## Task 5: Complete the signed standalone update transaction

**Files:** `Vapor-Installer/crates/vapor_installer/**`, installer UI update state, Artifact release contract.

**Interfaces:** Only a `VerifiedRelease` from the existing verifier may reach extraction/staging. App-root replacement is staged and recoverable; user data and product sources stay outside the replaceable app instance.

- [ ] Complete bounded archive extraction into a fresh staged app root.
- [ ] Validate staged manifest, updater version, target matrix, and expected files before swap.
- [ ] Atomically replace the installed app root and restore the prior version if the swap or startup check fails.
- [ ] Expose update phases and errors in Installer; provision a reviewed production verification key through the signed release configuration.
- [ ] Keep update/uninstall independent of Steam.

## Task 6: Keep Steam and game products as local sources

**Files:** Vapor Client source/launch adapters and Loo Cast/Steam/Mad Max owner plans.

**Interfaces:** Steam Loo Cast artifact contains `App.vapor.toml`, Engine, Game, and Packagepack. The locator opens the separate Vapor selector only. Mad Max adapter consumes a local Engine source, Game layer, and integration Packagepack that contains no game files.

- [ ] Make the Steam locator discover Vapor and give clear guidance if absent; it never downloads or provisions Vapor.
- [ ] Keep Steam app-root files to the declared Loo Cast artifact and the explicitly allowed locator.
- [ ] Keep Loo Cast first-party products depot-provided and non-publishable under server authority.
- [ ] Define Client adapter inputs against Vapor's canonical product/source identity; do not introduce a parallel schema.
- [ ] Keep Mad Max adapter implementation and loader provenance in MM-Building-System's owned plan; expose only generic Client contracts here.

## Task 7: Update current setup documentation and verify the product journeys

**Files:** Client README, Installer/CLI setup docs, workstream record, both target build outputs.

- [ ] Replace active references to Composer and EcosystemDeveloper local roles; retain historic records only where they explain past behavior.
- [ ] Document the standalone install, Steam locator behavior, Player/Developer tool difference, and local acquisition of game content.
- [ ] Use Vapor-managed formatting/check/build commands for affected workspaces; do not run tests unless explicitly requested.
- [ ] Verify Windows MSVC-host/GNullVM-target and Linux target builds independently.
- [ ] Verify fresh install, update rollback, Steam-present/absent locator behavior, and uninstall boundaries without publishing or deploying.
- [ ] Record actual completion, verification commands/results, and remaining external owner actions.

## Exit Conditions

- Vapor's binaries/tools/sources/cache/content/config/state are owned by the Vapor home, not Steam.
- Player and Developer are the only local roles; Developer cannot be marked ready without every required tool.
- The signed updater can stage, swap, roll back, and update without Steam.
- Loo Cast depot and Mad Max local files remain separate, trusted sources, and no game bytes are shipped by Vapor services.
- The Windows build keeps MSVC compiler-host and GNullVM runtime-target semantics distinct; Linux and Windows outputs build independently.
