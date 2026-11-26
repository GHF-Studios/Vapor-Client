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
- Stable Vapor releases are intended to publish from the default-branch pipeline only after all required checks and release packaging succeed. The release signing key belongs in a protected CI secret; remote publishing requires a separate restricted publisher identity.

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
- [x] Add separate CLI and Installer UI erase-all actions, with exact affected roots shown before confirmation. The standalone bootstrap can invoke this after program removal; the reviewed path set is rechecked before deletion. Ordinary removal still preserves user data/source.
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

- [x] Identify the release signer format and pin the dedicated production app-release public key in the Installer verifier (`vapor-release-2026-11-24`, SHA-256 `6560933ce5adeb11362cb01b070d6a5142d430d51de867c79e1c4bb2b9a8dd63`). Its private Ed25519 key file remains in the local Vapor home with mode `0600`, outside repositories and the VPS.
- [x] Wire the bounded manifest and archive verifier into HTTPS acquisition using only the offline-pinned release key set (Linux and Windows bundles are now signed and the publisher accepted both in isolated temporary storage; the public Artifact feed remains unexercised).
- [x] Implement bounded archive download, digest verification, safe same-volume extraction, runtime-closure checks, immutable version placement, and atomic active-pointer replacement. The Installer and setup targets are included in the Vapor-managed Linux/Windows release builds; actual acquisition/activation still needs a live feed.
- [x] Add installed-version listing and explicit activation of an already installed release. Artifact also exposes a newest-first stable history endpoint; the GUI's available-history picker remains unfinished.
- [x] Expose local installed-version selection in the standalone Installer panel; switching updates the active pointer for the next launch without deleting versions.
- [x] Add a setup-only `vapor-setup` GUI mode that installs, updates, repairs, opens, and normally removes the standalone app without requiring Steam or an installed app instance. Add `vapor-setup install [--channel stable]` as a headless entry to the same signed install path for managed bootstrap automation. The full Vapor-GUI workspace compiled through Vapor-managed Cargo for Linux and the Windows release target; the staged Linux setup executable reported this subcommand in `--help`. No tests or clean-machine install were run.
- [x] Normal standalone removal clears its managed PATH/environment integration while preserving canonical User Data and source.
- [x] Extend the signed release manifest, Artifact immutable routes, release preparation/publication scripts, and Homepage download link for a separate setup executable. Linux and Windows bundles were assembled and signed locally, and isolated publication validated both bundles. Those initial `0.5.0` bundles predate the owner's rolling inner SemVer release identity and must not be used as the first publication.
- [ ] Publish the signed setup bootstrap and app bundles for Windows and Linux under the pre-alpha release identity `0.5.0-pre-alpha-0.1.0`, then confirm public latest/history links and a clean install.
- [ ] Add signed available-version history so the GUI can select an older published release; current selection covers versions already installed locally.
- [x] Build standalone release artifacts through Vapor for Linux and Windows targets; no raw Cargo invocation. Tests and clean-machine install were skipped by owner direction, and the live release path is still pending.

### Task 3: Standalone distribution artifacts and website download surface

**Files:**
- Modify: `App-Source.vapor.toml` or add a dedicated standalone distribution manifest
- Modify: Vapor Client root release packaging/provider files that own staged artifact assembly
- Modify: `Vapor/crates/vapor_client_provider/src/standalone_release.rs` and `release/promotion.rs`
- Modify: `Vapor-CLI/crates/vapor_cli/src/command/model.rs` and `command/root_commands.rs`
- Modify: `Vapor-CLI/crates/vapor_cli/docs/commands.md`
- Create: `release/package-vapor-standalone-archive.sh`
- Modify: `Vapor-Platform-Server/Vapor-Artifact-Server/src/lib.rs`
- Modify: `Vapor-Platform-Server/Vapor-Homepage-Server/src/lib.rs`
- Modify: `Vapor-Platform-Server/Vapor-Homepage-Server/README.md`
- Test: release package review and Homepage route checks

**Interfaces:**
- Produce stable platform artifact names for Windows x86_64 and Linux x86_64 using the existing signed Artifact manifest and latest-feed contract consumed by Task 2.
- Homepage already exposes a download page that reads Artifact's stable feed for Linux and Windows; finish the user flow only when the signed app archive is accompanied by a real guided installer/bootstrap artifact.
- Do not alter Homepage Server deployment orchestration or embed large binaries in its image.

- [x] Add a separate Vapor-owned `root stage-standalone` operation that assembles selected release outputs under the inferred canonical Vapor User Data/output roots, writes the canonical Vapor workspace toolchain pin and Vapor-only app manifest, and leaves the active app root untouched. It can run from the source-built CLI using the source root without an App Instance deploy; the release provider resolves User Data and Output itself. Vapor-managed Linux/Windows release build and staging both succeeded.
- [x] Update the archive assembler to consume this standalone stage instead of an app instance. Linux and Windows archives assembled successfully under Vapor User Data (113 MB and 98 MB); structural inspection confirmed target-specific runtime closure, launcher and toolchain metadata, with no Loo-Cast/source payload.
- [x] Define a separate app-instance manifest template in `Standalone-App.vapor.toml`; the release archive must install it as `App.vapor.toml`, not copy the Steam-owned Loo-Cast manifest.
- [x] Extend the Artifact release preparation guard to accept both current target IDs and require the documented standalone runtime closure, reject unsafe/duplicate/Steam/game payload paths, and require the Vapor-only app manifest (source changed; bundle preparation remains unverified).
- [x] Homepage validates release platform/version, setup/archive size and digest metadata, and canonical URLs; the setup binary is the primary download.
- [x] Artifact exposes a newest-first stable version index from complete immutable bundles; Homepage renders older per-platform setup links under the latest primary download. New releases append automatically from their immutable directory without a separate Homepage edit. Artifact and Homepage compiled through the Vapor shell; no tests or runtime requests were run.
- [x] Permit the pinned `0.5.0-pre-alpha-<inner SemVer>` release identity on the stable distribution channel. Artifact history orders inner SemVer numerically and puts the final pinned version after pre-alpha; Homepage accepts the same form. The implementation and API documentation are now present in local Platform submodules, but this continuation has not compiled or deployed those edits.
- [x] Verify signed manifest platform IDs and bootstrap package contents agree for both platforms; isolated publisher validation passed. Live Homepage links remain unverified because nothing is published.
- [x] Add the default-branch build/package/publish workflow using Vapor-managed builds, the protected release-signing secret, and a restricted publisher SSH key. The workflow exists on `dev`; its main-branch execution still requires the initial live feed seed. GitHub visibly lists the expected protected environment secrets and variables, but no hosted run has validated their values.
- [ ] Publish the signed first rolling pre-alpha release and confirm its public latest/history links; no release is live.
- [ ] Remove the remaining app-instance startup dependency from Vapor commands where the operation belongs to source/User Data or the standalone installation. The release-stage command is a narrow working example; this is broader than Steam depot publishing and needs its own ownership/dependency pass.

#### Bootstrap/rebuild blocker observed 2026-11-26

The default-branch workflow installs the latest signed stable setup before it
can build, so it cannot seed the first release. Existing signed `0.5.0`
bundles do not match the owner's chosen pre-alpha product version. The local
Steam-launched Vapor command is not a usable source-build home: its managed
service rejects the intentionally retired `ecosystem-developer` role state,
and source-local Client deployment resolves the Steam app root where a
standalone `Vapor-Home.toml` is required. No user state or Steam files were
changed. A direct Vapor-managed debug build compiled, but did not deploy. A
bootstrap path that builds through Vapor without borrowing the Steam App
Instance, or a valid signed first-release seed installed through the standalone
Installer, is required before regenerating the first rolling release.

#### Release-channel and live-state reconciliation (2026-11-26)

GitHub PR #10 is closed without a merge to `main`; its feature history was
integrated into the persistent `dev` branch. `main` remains the stable release
channel. The live Homepage `/downloads` view confirms that neither Linux nor
Windows currently has a published stable release. The Platform receiver is
deployed, but the Artifact feed is still empty. The first release must use
`0.5.0-pre-alpha-0.1.0`; the locally signed `0.5.0` bundles are not valid
seeds. The latest User Data path reconciliation is local-only in the dirty
`dev` worktree and nested Vapor/Vapor-CLI checkouts, and is not included in
PR #10 or on `main`.

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

- [x] Implement a distinct `creator` handoff that resolves the active release under the canonical Vapor User Data root, removes inherited Steam `VAPOR_HOME`, and opens Downloads when no active release exists. The `play` route remains direct.
- [x] Narrow Linux and Windows Steam depot allowlists to the player entrypoint, Vapor CLI, Client service bootstrap, required runtime libraries, and packaged Loo-Cast closure; omit Vapor GUI and Installer executables.
- [x] Wire Installer acquisition/activation source to write this versioned product layout; the pointer writer and creator reader are present but uncompiled and unverified together.
- [ ] Confirm all currently writable Loo-Cast runtime paths use `VAPOR_USER_DATA_ROOT` and never fall back to the Steam directory in packaged mode.
- [ ] Compile and perform bounded runtime path verification without deleting the user's Steam installation. User explicitly asked to skip tests; no build or runtime result is claimed here.

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

## Owner version clarification (2026-11-26)

Keep the pre-alpha compatibility target fixed at `0.5.0`; identify standalone
product releases as `0.5.0-pre-alpha-<inner SemVer>`. The initial release is
`0.5.0-pre-alpha-0.1.0`. Successful default-branch workflow runs advance the inner patch
component on the `0.1` train. The Artifact `stable` value names the update
channel and may carry this prerelease-form version. Package/content/schema
versions remain independent. The locally signed `0.5.0` candidates are not
valid seeds for this policy.

#### Rust/Cargo path ownership follow-up (2026-11-26)

The setup helper provisions Rustup and Cargo under `<Vapor User Data>/tools/{rustup,cargo}`, but several consumers still resolved the app root or flat legacy directories. `VaporUserDataLayout` now defines the canonical tool, build-cache, and output paths; Core, SDK, Client provider, CLI, IDE setup, generated activation scripts, and shared setup helpers use that contract. Vapor-home initialization no longer creates these caches/tool homes inside the versioned app root. The XDG override is accepted only when absolute, matching generated Unix activation scripts.

The changed Core, SDK, Client provider, and CLI packages compiled through Vapor-managed Cargo. The local deployment command then stopped at its explicit guard because the Steam app root is not a marked Vapor Home. No tests ran; the managed build cache under Vapor User Data was updated, while Steam and release artifacts were untouched. This resolves the demonstrated tool-path mismatch; it does not resolve the missing first feed seed or `root build`'s installed-App Instance dependency.

#### First pre-alpha candidate prepared (2026-11-26)

The lockfiles needed by the active workspace were resolved through Vapor's
managed command. `root build --release --release-targets` then completed at
100% for Linux and Windows. The rebuilt source CLI staged 24 artifacts under
canonical Vapor User Data without deploying to an App Instance. That run
exposed a provider path assumption: Cargo's native host outputs are directly
under the profile directory, while foreign-target outputs use a target-triple
directory; the provider now models both layouts.

The staged outputs were packaged and signed as
`0.5.0-pre-alpha-0.1.0`. Local verification passed for both platform
signatures, manifest identities, artifact/setup sizes and SHA-256 digests,
and stable pointers. This does not publish or prove the live download/install
flow. The live Homepage still reports no stable release. The signed-in browser
recorded this evidence on Vapor-Client#3; that issue remains OPEN + BLOCKED.

The `vapor-stable-release` GitHub environment now contains the three expected
secrets and two publisher variables, restricted to `main`; only names and
timestamps were inspected, not secret values, and no hosted workflow has
validated them. Local GitHub CLI reauthentication reached an owner-only fresh
passkey/authenticator/email confirmation screen and was stopped. The local
publisher private/public keypair matches, but a no-payload SSH probe was
rejected with `Permission denied (publickey)`, so the restricted account is
not usable from this session and no bundle was sent. GitHub `dev` still has the
old CI-run-number version generator; the corrected pre-alpha workflow remains
in the dirty local tree. Next is to resolve publisher access, then synchronize
the built Client/submodule changes, seed the signed candidate, integrate the
workflow on `main`, and verify live download/install behavior. Steam remains
untouched. The Homepage copy was locally corrected to describe Player and
Developer setup as implemented; that nested source edit is uncompiled and
undeployed.
