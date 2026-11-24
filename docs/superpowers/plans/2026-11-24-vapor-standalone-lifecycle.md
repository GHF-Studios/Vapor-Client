# Vapor Standalone Lifecycle Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Vapor independently installable, version-managed, and completely removable while keeping all Vapor state outside the Steam game and preserving direct Loo-Cast Play.

**Architecture:** Extend Vapor Installer from an app-root setup shim into the owner of standalone product installation and version transitions. Reuse Vapor Core's canonical data-root and Superworkspace machinery, consume Platform Artifact Server's signed immutable release manifests and SHA-256 checked archives, keep Homepage as the public discovery/download surface, and route Steam creator launch separately from direct game Play.

**Tech Stack:** Rust 2024 workspaces and Vapor-managed build/release commands; Axum Homepage and Artifact Servers; Ed25519-signed immutable manifests; existing Steam entrypoint.

**Spec:** [Vapor Standalone Lifecycle](../specs/2026-11-24-vapor-standalone-lifecycle-design.md)

## Global Constraints

- The default Loo-Cast Steam Play route never waits for Vapor installation or setup.
- No Vapor-managed mutable state, cache, log, save, or source data is written beside the Steam game.
- Use `~/Vapor/sources` for the canonical authored source tree and the OS app-data root for installed versions and machine-local Vapor data.
- A full wipe deletes only registered Vapor-managed canonical roots and never a parent home directory or the Steam-owned game depot.
- Keep ordinary uninstall separate from explicit erase-all; ordinary uninstall preserves user data and source.
- Preserve unrelated dirty and untracked work; use Vapor-managed commands for all Rust builds and validation.
- Do not publish Steam's public default branch; public Steam rollout remains owner-controlled.

## Review Focus

- Installer interruption during download, extraction, or activation leaves the previous version launchable — exercise staged activation and rollback.
- A malformed or mismatched release asset is rejected before activation — exercise the catalog and artifact integrity checks.
- Full wipe with unrelated files in a canonical parent preserves those files and refuses unsafe roots — exercise preflight with temp fixtures.
- Steam creator launch with Vapor absent opens the download surface; with Vapor present launches the creator surface — exercise both routing cases.
- Game-specific writes and logs from the default Steam launch resolve under Vapor User Data — inspect and run a bounded launch with a temporary canonical data root.

---

### Task 1: Canonical Vapor path ownership and full wipe

**Files:**
- Modify: `Vapor/crates/vapor_core/src/installation.rs`
- Modify: `Vapor/crates/vapor_core/src/uninstall.rs`
- Modify: `Vapor-Installer/crates/vapor_installer/src/cli.rs`
- Modify: `Vapor-Installer/crates/vapor_installer_ui/src/lib.rs`
- Modify: `Vapor/docs/roadmap/11-local-install-source-lifecycle/README.md`
- Test: focused `vapor_core` installation/uninstall tests

**Interfaces:**
- Produce a single `UninstallOptions` full-wipe intent that preflights canonical user-data and registered Superworkspace roots together before mutation.
- Produce a report listing removed managed roots and preserved unknown entries.
- Preserve the current ordinary uninstall and individual purge behavior for compatibility.

- [x] Add focused coverage showing registered source checkouts are erased while unrelated neighboring files are preserved; validate canonical containment during preflight.
- [x] Implement a combined erase-all operation that preflights user data, registered source state, and machine integration before mutation. It deletes registered checkout directories (including local work) but preserves unknown Superworkspace neighbors.
- [x] Add separate CLI and Installer UI erase-all actions, with exact affected roots shown before confirmation. Ordinary removal still preserves user data/source.
- [x] Run Vapor-managed verification for the Vapor workspace and Vapor Installer workspace. Both passed; one earlier client process test run transiently failed with `Text file busy` and passed on rerun.

### Task 2: Signed standalone release verification and app version lifecycle

**Files:**
- Modify: `Vapor-Installer/crates/vapor_installer/src/cli.rs`
- Modify: `Vapor-Installer/crates/vapor_installer/src/lib.rs`
- Inspect and reuse: the live Vapor-Installer release verification code, if present; do not create a parallel unsigned catalog beside the Artifact contract
- Create: focused download/activation modules under `Vapor-Installer/crates/vapor_installer/src/`
- Modify: `Vapor-Installer/Cargo.toml` and lockfile only if required
- Modify: `Vapor/crates/vapor_core/src/installation.rs`
- Modify: standalone release packaging metadata under `Vapor-Client`
- Test: focused release parsing, digest validation, staged activation, and rollback tests

**Interfaces:**
- Consume the Artifact manifest contract: schema, key ID, SemVer version, channel, platform, minimum updater version, artifact URL/size/SHA-256, and release-notes URL; verify exact manifest bytes with the configured Ed25519 public key before trusting the archive.
- Add installer operations for install/latest, update, list/select version, repair, and uninstall that target an explicit standalone app root.
- Keep authored source and mutable user data outside versioned program directories.

- [ ] Confirm verifier presence against current source; where absent, implement bounded manifest/signature retrieval and archive verification using the existing Artifact API and trust contract.
- [ ] Implement staged download/extraction and activation that leaves the prior version intact on every failure path; retain the previous immutable release for rollback.
- [ ] Implement current-version and explicit rollback to a published version. A version history/list endpoint is not in the current Artifact API; resolve an Artifact-owned signed history surface before claiming a complete version picker.
- [ ] Keep the installed bootstrapper stable across app version swaps and route GUI/CLI lifecycle actions through it.
- [ ] Build/check with Vapor for Linux and Windows targets; no raw Cargo invocation.

### Task 3: Standalone distribution artifacts and website download surface

**Files:**
- Modify: `App-Source.vapor.toml` or add a dedicated standalone distribution manifest
- Modify: Vapor Client root release packaging/provider files that own staged artifact assembly
- Modify: `Vapor-Platform-Server/Vapor-Homepage-Server/src/main.rs`
- Modify: `Vapor-Platform-Server/Vapor-Homepage-Server/README.md`
- Test: release package review and Homepage route checks

**Interfaces:**
- Produce stable platform artifact names for Windows x86_64 and Linux x86_64 and a release catalog consumed by Task 2.
- Homepage already exposes a download page that reads Artifact's stable feed for Linux and Windows; finish the user flow only when the signed app archive is accompanied by a real guided installer/bootstrap artifact.
- Do not alter Homepage Server deployment orchestration or embed large binaries in its image.

- [ ] Assemble a standalone payload that excludes Loo-Cast game assets and Steam-only content.
- [x] Homepage reads the Artifact stable feed and validates returned platform, version, size, digest, and canonical artifact URL; it does not fabricate a download while no stable artifact exists.
- [ ] Verify signed manifest platform IDs, bootstrap package contents, and Homepage links agree.
- [ ] Confirm repository release write access before publishing public assets; do not claim downloads live without a successful fetch.

### Task 4: Steam creator handoff and game-state path contract

**Files:**
- Modify: `Vapor-Steam-Entrypoint/crates/vapor_entrypoint/src/main.rs`
- Modify: `Vapor-Steam-Entrypoint/crates/vapor_entrypoint/src/entrypoint/app_root.rs`
- Modify: `Vapor/crates/vapor_client_provider/src/content/launch.rs` only if needed
- Modify: `Loo-Cast/spacetime-engine/src/devtools/script_workbench/storage.rs` only if needed
- Modify: Steam launch-option metadata owned by current source/deployment pipeline
- Test: focused creator-present/creator-absent route checks and bounded app-root write audit

**Interfaces:**
- Preserve no-argument/`play` direct launch of the bundled default Loo-Cast Packagepack.
- Add a distinct creator handoff that starts the installed standalone Vapor GUI or opens the download page if absent.
- Pass the canonical Vapor user-data root to every game process; all app-specific writable data is namespaced below it.

- [ ] Add focused route coverage for installed and absent Vapor cases while keeping `play` as direct game launch.
- [ ] Confirm all currently writable Loo-Cast runtime paths use `VAPOR_USER_DATA_ROOT` and never fall back to the Steam directory in packaged mode.
- [ ] Build through Vapor and perform bounded runtime path verification without deleting the user's Steam installation.

### Task 5: Integrate documentation and issue/workstream state

**Files:**
- Modify: `Vapor-Installer/README.md`
- Modify: `Vapor/docs/roadmap/11-local-install-source-lifecycle/README.md`
- Modify: `.agents/workstreams/open/workshop-content-superpass.md`
- Modify: `.agents/workstreams/README.md` only if workstream ordering/state changes
- Update: owning GitHub issue after verifying the correct repository owner and workflow relations

- [ ] Document standalone install, update/downgrade, normal uninstall, full wipe, and Steam separation.
- [ ] Reconcile the stale Steam build/Windows evidence in Vapor-Client#3 without closing it prematurely.
- [ ] Keep the standalone lifecycle work ACTIVE and identify any still-blocked publication or clean-machine proof.
- [ ] Run `git diff --check` across the parent and every modified subrepository.
