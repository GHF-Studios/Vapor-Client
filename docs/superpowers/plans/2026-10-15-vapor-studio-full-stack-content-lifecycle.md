# Vapor Studio Full-Stack Content Lifecycle Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Vapor Studio the coherent GUI for discovering, installing, composing, creating, editing, running, and publishing Vapor content.

**Architecture:** Add a focused Studio workflow/domain layer that coordinates existing Vapor Core and Vapor CLI/Shell authorities. Keep eframe UI responsible for presentation and flow, route long work through typed operations, and connect launcher, workspace editor, and Workshop against shared content identity and provenance.

**Tech Stack:** Rust, eframe/egui, Vapor Core, Vapor CLI/Shell, Steam Workshop providers, Vapor-GUI, Steam entrypoint.

**Spec:** `docs/superpowers/specs/2026-10-15-vapor-studio-full-stack-content-lifecycle-design.md`

## Global Constraints

- Keep content and pack kinds Rust-defined; templates instantiate supported kinds.
- Preserve the local-authorship closure rule and distinguish authored source, resolved graph, installation, and launch composition.
- Studio owns presentation and confirmation; Vapor-owned services own resolution, toolchains, acquisition, installation, and publishing.
- Run builds/tests only through Vapor-managed commands at the owning workspace.
- Keep live Steam behavior, deploy, and publish checks as explicit runtime review points; compilation and available tests are authorized.
- Do not deploy, launch Steam, publish externally, or commit/push without explicit authorization.

## Review Focus

- Steam client unavailable or Workshop query fails: local/first-party content remains usable and Studio shows a recoverable source error.
- Missing or malformed prior profile state: migrate or recover without losing a valid selected composition.
- Missing, incompatible, or conflicting dependency: explain the exact graph blocker before installation or launch.
- Interrupted acquisition/install: expose partial state and a safe resume/repair path.
- Invalid direct-Play profile: return to Studio with a clear action instead of an empty or stranded launch.

---

### Task 1: Establish Studio workflow ownership and durable profile model

**Files:**
- Create: `Vapor-Studio/crates/vapor_studio_core/Cargo.toml`
- Create: `Vapor-Studio/crates/vapor_studio_core/src/lib.rs`
- Create: `Vapor-Studio/crates/vapor_studio_core/src/profile.rs`
- Create: `Vapor-Studio/crates/vapor_studio_core/src/operation.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/Cargo.toml`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/studio.rs`

**Produces:** A Studio-owned persisted composition/profile model, migration from current eframe storage, and typed workflow requests/results that wrap Vapor authorities without duplicating business logic.

- [ ] Map the existing eframe profile fields to content identity, root Packagepack, workspace association, launch settings, and installation state.
- [ ] Add Studio Core with versioned profile persistence; migrate `name`, `selected_content`, and `workspace_root`, and preserve recoverable diagnostics for malformed data.
- [ ] Define typed workflow requests/results with operation identity and structured progress, completion, cancellation, and error states.
- [ ] Keep installation state and per-profile selection in the owning model; do not treat Shell's single current-selection file as the library's source of truth.
- [ ] Review that local source, resolved graph, installation, and profile remain separate linked records.

### Task 2: Replace subsystem tabs with the product shell and focused workspaces

**Files:**
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/studio.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/app.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/navigation.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/operation_center.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/pages/home.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/pages/discover.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/pages/library.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/pages/create.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/pages/workspace.rs`
- Create: `Vapor-Studio/crates/vapor_studio_ui/src/components/content_detail.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/lib.rs`

**Produces:** Home, Discover, Library, Create, and Workspace destinations backed by Studio Core; a compact context/search header; contextual editor panels; and a collapsible operation center.

- [ ] Move app lifecycle and persisted state out of the monolithic view module.
- [ ] Implement navigation and content-detail routing so catalog, library, and workspace actions retain the same content identity.
- [ ] Add responsive loading, empty, and recoverable-error states to each destination.
- [ ] Move progress and operation results into the operation center; keep long work off the frame loop.
- [ ] Remove the old Compose/Workshop/Publish placeholder tabs when their actions have contextual homes.

### Task 3: Connect Discover to real content sources and Workshop

**Files:**
- Create: `Vapor-Studio/crates/vapor_studio_core/src/catalog.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_core/src/operation.rs`
- Modify: `Vapor-CLI/crates/vapor_shell/src/content.rs`
- Modify: `Vapor-CLI/crates/vapor_shell/src/steam.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/pages/discover.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/components/content_detail.rs`

**Produces:** One searchable catalog over local, first-party, installed, and Steam Workshop sources, retaining provenance and provider state.

- [ ] Define the catalog record around stable content identity, kind, version, source, compatibility, dependencies, PublishedFileId, and installed/subscribed state.
- [x] Add Steamworks UGC query and subscription actions through Vapor's `vapor_steam` crate.
- [x] Show remote Workshop search alongside local discovery; local browsing and exact-ID acquisition remain available when Steam cannot initialize.
- [ ] Add search, kind/source/compatibility filters, content details, and actionable provider error/loading states.
- [x] Keep Workshop discovery and subscription actions in Discover; subscription-aware update/download state remains open.

### Task 4: Complete profile-targeted install, composition, and Play

**Files:**
- Create: `Vapor-Studio/crates/vapor_studio_core/src/composition.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_core/src/operation.rs`
- Modify: `Vapor-CLI/crates/vapor_shell/src/content.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/pages/discover.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/pages/library.rs`
- Modify: `Vapor-GUI/crates/vapor_gui/src/main.rs`
- Modify: `Vapor-Steam-Entrypoint/crates/vapor_entrypoint/src/**` only if stable profile identity must cross the entrypoint.

**Produces:** Discover → install into a new/existing composition → manage graph and update state → Play, with safe recovery.

- [x] Connect Install to new-profile and existing-profile targets.
- [x] Resolve dependencies into a versioned/provenance-aware graph and explain missing, incompatible, duplicate, and conflicting nodes before installation.
- [ ] Show acquire, install, verify, update, and repair state with safe handling of interrupted work.
- [ ] Preserve independent selections across profiles and expose selected root, resolved versions, source, and launch target in Library.
- [ ] Route direct Play by stable profile/composition identity; return invalid selections to Studio with an actionable explanation.

### Task 5: Complete template-to-local-run authoring

**Files:**
- Create: `Vapor-Studio/crates/vapor_studio_core/src/workspace.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_core/src/operation.rs`
- Modify: `Vapor/crates/vapor_core/src/content.rs`
- Modify: `Vapor/crates/vapor_core/src/workspace.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/pages/create.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/pages/workspace.rs`
- Modify: `Loo-Cast/spacetime-engine/src/devtools/script_workbench/**` only for shared operation/content-contract integration.

**Produces:** Minimal-setup templates for supported kinds and a workspace capable of editing, composing, validating, and running content through Vapor-managed operations.

- [x] Connect kind-specific content templates and workspace creation to existing Vapor Core/Shell facilities; preserve exclusive-create and source-root safety.
- [ ] Make the project/content tree, editor or preview, and inspector act on real workspace files and manifest/script types.
- [x] Compose Packagepacks from compatible content identities and show the resolved graph and local-authoring closure.
- [x] Route workspace actions through explicit roots and shared progress/results.
- [ ] Connect the in-game workbench to shared content/operation contracts without moving it out of the game process or exposing Rust/Bevy borrows.

### Task 6: Complete Workshop creation and publishing

**Files:**
- Create: `Vapor-Studio/crates/vapor_studio_core/src/publishing.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_core/src/operation.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/components/content_detail.rs`
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/pages/workspace.rs`
- Modify: `Vapor-CLI/crates/vapor_shell/src/content.rs`
- Modify: relevant Workshop docs under `Vapor-CLI/crates/vapor_shell/docs/`

**Produces:** Context-aware Workshop create/update with actual staged-artifact review, explicit confirmation, receipt persistence, and refresh into discovery/library state.

- [ ] Expose a stage/preview operation returning actual staged files, content graph, version, visibility, target, and publisher identity.
- [x] Derive create-versus-update from Workshop identity; show the exact authority-changing action and confirmation target.
- [x] Ensure preview, navigation, or dismissal cannot publish; persist PublishedFileId/version/receipt after confirmed upload.
- [x] Refresh the source catalog after publication; profile graph refresh remains part of install/update recovery.
- [ ] Keep SteamPipe root publishing visibly separate from Workshop content publishing.

### Task 7: Close recovery, diagnostics, and cross-platform lifecycle gaps

**Files:**
- Modify: `Vapor-Studio/crates/vapor_studio_ui/src/**`
- Modify: `Vapor-GUI/crates/vapor_gui/src/main.rs`
- Modify: `Vapor-Steam-Entrypoint/crates/vapor_entrypoint/src/**` where needed.
- Modify: `README.md` and relevant Studio/Shell docs.

**Produces:** A recoverable full lifecycle with discoverable diagnostics and documented Linux/Windows review paths.

- [ ] Walk through all five Review Focus cases and remove dead ends.
- [ ] Preserve telemetry, logs, sessions, and Tracy behind the contextual diagnostics surface.
- [ ] Confirm GUI actions use Vapor-managed providers/toolchains and show their source, target, progress, and result.
- [ ] Document manual smoke paths for Discover/install/play and template/edit/run/publish.
- [ ] Update the active workstream with completed passes, remaining verification, and the smallest resume action.

## Execution notes

- Execute tasks sequentially as coherent megapasses; preserve the foreground full-stack workstream's path claims and unrelated local changes.
- Ask Leslie when provider capabilities or composition/package semantics require a product compromise.
- This pass used Vapor-managed checks/tests and committed the related repositories after explicit owner authorization. It did not deploy, launch Steam, or publish externally.
- Managed checks passed for Vapor, Vapor-CLI, and Vapor-Studio; workspace tests passed (26 Vapor tests; CLI and Studio currently define no tests). Vapor-GUI built and its Linux output resolves `libsteam_api.so` through `$ORIGIN`. Final depot inclusion, Windows runtime packaging/build verification, live Workshop behavior, and Steam client launch remain unverified.
