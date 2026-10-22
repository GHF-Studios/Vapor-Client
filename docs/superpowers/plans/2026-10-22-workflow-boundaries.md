# Vapor Workflow Boundaries Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax.

**Goal:** Separate Player, Composer, and Developer workflows across Vapor while keeping their shared service semantics coherent, separating authored source from usable local content, and preventing long operations from freezing or rejecting unrelated work.

**Architecture:** Vapor Client remains the authority for canonical content state, workflow classification, readiness, publishing authorization, and operations. Frontends consume a single projection; workflow tiers are progressive product boundaries, while tool installation and account permission remain independent. Local Library discovery must not scan workspaces; slow operations become scheduled service work with independent operation records and explicit conflict domains.

**Tech Stack:** Rust, serde protocol types, Vapor-managed toolchain, existing Vapor Client service and egui frontends.

**Spec:** `/home/leslieghf/.codex/.chatgpt-projects/g-p-6ab69f33d17481919fb92a0c5592210c/artifacts/vapor-workflow-boundaries-design.md`

## Global Constraints

- Keep Player, Composer, and Developer workflows nested; each higher level includes the lower levels.
- Composer uses Vapor-managed Rust/Cargo composition but does not require authoring Rust source; Developer owns source authoring.
- Workflow visibility is not authority; local tool readiness and external authorization stay separate.
- Source workspaces do not enter ordinary Library inventory.
- Use Vapor-managed commands only for build, format, tests, docs, and deployment; never raw Cargo.
- Preserve existing user data unless the user explicitly authorizes migration/reset behavior.

## Review Focus

- A large or permission-denied source tree cannot block Library or Workshop requests.
- Two independent long operations complete without overwriting each other's snapshot or progress.
- Conflicting operations serialize and cancellation targets the requested operation only.
- A stale or direct client request cannot bypass readiness or publishing authorization checks.
- Old persisted role state and protocol clients fail or migrate with a clear, recoverable outcome.

## File Structure

- `Vapor/crates/vapor_client_domain/src/`: workflow tiers and action classification.
- `Vapor/crates/vapor_client_protocol/src/`: shared snapshot, operation collection, and capability/readiness/authority projections.
- `Vapor/crates/vapor_client_service/src/session/`: authoritative policy, inventory discovery, and operation coordinator.
- `Vapor/crates/vapor_client_application/src/`: separate local artifact and workspace discovery APIs.
- `Vapor-GUI/crates/vapor_launcher_ui/src/`: Player-facing workflows.
- `Vapor-Studio/crates/vapor_studio_ui/src/`: Composer and Developer navigation/workflows.
- `Vapor-GUI/crates/vapor_ide_ui/src/`: Developer source workspace editor.
- `Vapor-Installer/crates/vapor_installer_ui/src/`: local capability setup and readiness.
- `Vapor-CLI/crates/vapor_cli/src/`: same workflow contracts for advanced commands.
- `Vapor-CLI/crates/vapor_cli/docs/design/`: update canonical product and role/tool setup descriptions.

## Tasks

### Task 1: Canonical workflow and action requirements

**Files:** domain action/types, protocol snapshots, service action policy, unit tests.

- [x] Add `WorkflowTier::{Player, Composer, Developer}` with stable serde names and `includes(required)` semantics.
- [x] Classify each public `ClientAction` once; Composer actions include Player, Developer includes both lower tiers.
- [ ] Finish protocol projections for current readiness and account/target authority result/reason. Action snapshots now carry tier and declared prerequisites; keep presentation tier separate from `InstallationRoleSnapshot` during migration.
- [x] Enforce declared local readiness at the service boundary for direct requests; concrete publishing handlers remain responsible for account- and target-specific authority.
- [ ] Add table-driven tests for tier inclusion, missing readiness, denied authority, and allowed direct action dispatch.

### Task 2: Split workspace sources from local Library artifacts

**Files:** `Vapor/crates/vapor_client_application/src/catalog.rs`, workspace discovery API, session catalog handlers, catalog protocol/UI mappings, CLI content output.

- [x] Add a local artifact discovery projection that reads only cached/installed artifact indexes and receipts.
- [x] Keep source projects and unregistered candidates exclusively in workspace snapshots.
- [x] Make Library refresh update only the local artifact projection; it must not call `refresh_workspace_content` or scan the workspace.
- [ ] Keep explicit workspace discovery as its own request and expose it as a cancellable, progress-reporting operation. It now runs asynchronously with progress and cancellation checkpoints between scan stages; underlying scanners still need finer cancellation checkpoints.
- [ ] Add regression coverage proving a workspace project is absent from Library and cached/installed output remains present when workspace discovery fails or is slow.

### Task 3: Operation registry and conflict-aware scheduling

**Files:** `Vapor/crates/vapor_client_protocol/src/state.rs`, `session.rs`, `session/snapshot.rs`, `server.rs`, operation tests.

- [x] Replace singular internal operation and cancellation fields with an operation registry keyed by operation ID; expose ordered active/history snapshots while keeping `active_operation` as a compatibility projection.
- [x] Define explicit resource keys for catalog refresh, workspace scan/edit, workshop account, composition identity, workspace build, publishing, tool installation, and profiling.
- [x] Accept independent work concurrently and reject actual resource conflicts with a specific reason; safe typed actions are revalidated against current state after unrelated operation progress changes the snapshot revision.
- [x] Make cancellation target one operation; retain one terminal result in operation history for completion, cancellation, and panic paths. Worker spawn failure is rejected before acceptance and releases its reservation.
- [ ] Update protocol event handling and test concurrent independent requests, conflict serialization, targeted cancellation, and unrelated request responsiveness during a slow scan.

### Task 4: Migrate Launcher, Studio, IDE, and Installer

**Files:** Launcher UI, Studio navigation/page actions, IDE entry/open-workspace flow, Installer role/setup pages.

- [ ] Keep Launcher focused on Play, Workshop discovery/subscriptions, and Library; remove source projects from player inventory and technical cache language.
- [ ] Split Studio navigation into distinct Composer and Developer areas; Composer owns composition/packagepack graph and Cargo-backed build setup, Developer owns source projects, Rust/Rhai editing, build/validation, and publishing.
- [ ] Keep IDE workspace-bound and source-only; do not duplicate Library or install responsibilities.
- [ ] Change Installer from persona selection to explicit local capability setup/readiness; Composer readiness includes managed Rust/Cargo composition support, Developer adds source-authoring tooling such as rust-analyzer and applicable diagnostics.
- [ ] Consume service-projected action state; remove duplicated frontend eligibility rules where the service already owns the rule.
  - [x] Selected-composition Install, Verify, Repair, Update, Build-and-launch, and Launch controls now consume service availability and reasons in Launcher/Studio.
  - [ ] Migrate remaining front-end actions from the legacy single-operation busy flag to service-projected readiness.
- [ ] Add UI/model tests for nested visibility, readiness display, and no raw source in Player surfaces.

### Task 5: Migrate CLI, documentation, and persisted state

**Files:** CLI command dispatch/output, persistent-state schema/migration, product-topology and setup docs, protocol compatibility tests.

- [ ] Map CLI commands to the same workflow tier and local/authority requirements; return service operation IDs and final states consistently.
- [ ] Replace role-as-persona wording with local capability/readiness language; preserve or migrate existing installation-role records without deleting installed tools or authored data.
- [ ] Update docs to distinguish composition metadata/Cargo realization from Rust source authorship, and source projects from Library artifacts.
- [ ] Add migration coverage for missing, legacy, malformed, and future-schema role/workflow records.

### Task 6: Integrated verification and deployment

**Files:** all changed crates and the workstream checkpoint.

- [ ] Run Vapor-managed formatting and focused/full validation for each affected workspace, then local deployment.
- [ ] Exercise Player launch/install, Workshop subscription/library update, Composer composition creation/build, Developer workspace scan/edit/build/publish review, and CLI equivalents through the service.
- [ ] Verify Linux and Windows protocol/service startup, clean operation completion, and that no authored source appears as a Library item.
- [ ] Record proven behavior, gaps, and any deferred conflicts in the active workstream; preserve unrelated changes.
  - [x] Focused Vapor and Studio formatting, checking, tests, and Linux local deployment passed for this checkpoint.
