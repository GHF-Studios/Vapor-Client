# Standalone Vapor Runtime and Products Implementation Plan

> **Status:** Superseded on 2026-11-10. Historical implementation record only;
> do not execute its old Steam artifact tasks. Use the current
> Vapor-Platform-Server realignment plan.
>
> **Artifact correction:** “One executable” means one primary executable
> entry point for each supported OS. Required DLLs or other declared runtime
> sidecars may ship beside it. This Steam payload is the Loo Cast Packagepack,
> not the Vapor runtime; use `Packagepack.vapor.toml`, not `App.vapor.toml`.

**Goal:** Install and manage Vapor independently from Steam while mounting Loo Cast and Mad Max through product-appropriate local sources.

**Historical architecture:** Vapor Client owns one standalone home, update lifecycle, and two-role setup. The former proposal for one self-contained executable embedding `App.vapor.toml` and all products was superseded. The current contract is a Loo Cast Packagepack manifest, one primary executable per target, and only its declared runtime sidecars. Vapor itself is installed and updated independently.

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
- Vapor Client itself has no Steam app/depot publisher; its runtime is released independently through Vapor.

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
- `Vapor-Steam-Entrypoint/**`: Steam launch entrypoint and embedded-payload reader for the self-contained Loo Cast executable; hands off to standalone Vapor and never bundles or provisions that runtime.
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

**Interfaces:** `VaporInstallation::discover()` resolves one Vapor home. Typed path accessors resolve `bin`, `tools/cargo/home`, `tools/rustup/home`, `sources`, `cache`, `content`, `config`, and `state` under that root. The Client service stores its default state under `$VAPOR_HOME/data/state/studio`; authored workspaces and product roots are distinct from Vapor home.

- [ ] Replace OS-local-data and Steam-root fallback discovery with the single Vapor-home root.
- [ ] Route every managed Cargo, Rustup, LLVM-MinGW/Zig, source, cache, configuration, and state path through that root.
- [ ] Implement and verify an idempotent, transactional one-time move of every Vapor-owned Steam-root entry after the replacement single-executable depot payload is built and validated. The implementation stages/conflict-checks data, previews every moved top-level entry, normalizes recognized roots, preserves unknown root names, and requires migration when an old Steam install is detected; target-build, fixture, and failure-recovery verification remain open.
- [ ] Verify normalization of the old `.vapor` subtree during that move: `diagnostics` → `data/diagnostics`, `downloads` → `cache/downloads`, `extract` → `cache/extract`, `logs` → `data/logs`, and `state` → `data/state`. Unknown children are preserved under `data/legacy-vapor/`; conflicting non-identical files fail rather than silently choosing a copy. The mapping is implemented but not yet validated with migration fixtures.
- [ ] Remove long-term dual-layout lookup and old Steam-root state behavior.
- [ ] Make uninstall remove only Vapor-owned home data according to explicit user selection; never remove mounted Loo Cast/Mad Max source content.

## Task 3: Collapse local setup roles and make Developer complete

**Files:** `Vapor/crates/vapor_core/src/role.rs`, domain action requirements, protocol snapshots, Client service handlers, Installer UI, setup scripts, CLI messages.

**Interfaces:** The persisted role enum is exactly `Player | Developer`. Developer is a superset of Player setup and requires all managed supported targets/cross-compilers. Server-side ecosystem developer authorization remains an independent Identity decision.

- [ ] Remove Composer, ContentDeveloper, and EcosystemDeveloper from local persisted and protocol role types; do not decode old values.
- [x] Remove the residual `WorkflowTier::Composer` wire value and classify composition authoring/build actions as Developer; the workflow profile now exposes only Player and Developer.
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

**Interfaces (superseded):** Each Steam target receives one primary Loo Cast Packagepack executable, its `Packagepack.vapor.toml` manifest, and only the runtime sidecars that executable requires. The manifest references the Steam-provided product content. No Vapor runtime or Vapor `App.vapor.toml` is embedded in this depot. Steam Play asks separately installed Vapor to run the default composition; Tools opens its selector. Both actions provide install guidance if Vapor is absent. Mad Max adapter consumes a local Engine source, Game layer, and integration Packagepack that contains no game files.

- [x] Remove the old Vapor Client Steam app/depot declarations and the unused Vapor-to-Steam self-publishing core module. Vapor Client manifests no longer carry Loo Cast Workshop product seeds.
- [x] Remove published Workshop IDs and Workshop dependency IDs from the three Loo Cast product manifests; add `steam-mount/App.vapor.toml` with local `steam-depot` product paths.
- [ ] Make the executable's dedicated tools launch discover Vapor and give clear guidance if absent; it never downloads, provisions, or bundles Vapor.
- [ ] Keep Steam app-owned payload to one primary Packagepack executable per ABI target, its `Packagepack.vapor.toml`, and explicitly required runtime sidecars; migrate every other old Vapor-owned root entry to Vapor home.
- [x] Keep the Loo Cast workspace as product source and use `steam-mount/` only as build staging input, with an explicit allowlist for the manifest, three product roots, and ABI-specific build inputs. Reuse only existing AppID/depot IDs already declared in local manifests and installed Steam app metadata; no IDs were invented.
- [ ] Implement target-specific staging that emits the allowlisted manifest, product executable, and declared runtime sidecars for each depot target. Do not embed product trees into the executable or include unrelated Vapor files.
- [ ] Add the dedicated tools launch to the same primary executable. Required Windows runtime libraries may be shipped as sidecar DLLs; Steamworks launch-option wiring remains app metadata.
- [x] Make `root package` stage standalone Vapor bundles, Steam depot payloads, or both according to the selected source root. A depot-only mount with no Cargo projects skips app builds and never syncs Vapor-owned scripts/tools into the product source.
- [x] Keep SteamPipe templates and ecosystem bootstrap metadata in Vapor-owned release resources/standalone metadata; remove the retired `Vapor-Client.vapor.toml` source marker.
- [ ] Verify the app-source allowlist and manifest against actual product outputs; inspect each Windows/Linux depot staging result to confirm it contains exactly one primary executable plus only its manifest and declared runtime sidecars.
- [ ] Keep Loo Cast first-party products depot-provided and non-publishable under server authority.
- [ ] Define Client adapter inputs against Vapor's canonical product/source identity; do not introduce a parallel schema.
- [ ] Keep Mad Max adapter implementation and loader provenance in MM-Building-System's owned plan; expose only generic Client contracts here.

## Task 7: Update current setup documentation and verify the product journeys

**Files:** Client README, Installer/CLI setup docs, workstream record, both target build outputs.

- [ ] Replace active references to Composer and EcosystemDeveloper local roles; retain historic records only where they explain past behavior.
- [x] Document the standalone install, Steam tools-launch behavior, Player/Developer tool difference, and local acquisition of game content in the Client README and CLI setup guide.
- [ ] Use Vapor-managed formatting/check/build commands for affected workspaces; do not run tests unless explicitly requested.
- [ ] Verify Windows MSVC-host/GNullVM-target and Linux target builds independently.
- [ ] Verify fresh install, update rollback, Steam-present/absent tools-launch behavior, and uninstall boundaries without publishing or deploying.
- [ ] Record actual completion, verification commands/results, and remaining external owner actions.

## Exit Conditions

- Vapor's binaries/tools/sources/cache/content/config/state are owned by the Vapor home, not Steam.
- Player and Developer are the only local roles; Developer cannot be marked ready without every required tool.
- The signed updater can stage, swap, roll back, and update without Steam.
- Loo Cast depot and Mad Max local files remain separate, trusted sources, and no game bytes are shipped by Vapor services.
- The Windows build keeps MSVC compiler-host and GNullVM runtime-target semantics distinct; Linux and Windows outputs build independently.
