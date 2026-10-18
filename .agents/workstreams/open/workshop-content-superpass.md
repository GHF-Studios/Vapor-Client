# Workstream — Full-Stack Authoring Superpass

- State: foreground
- Goal: unify the Vapor Studio launcher, composition management, Workshop,
  Steam root publishing, in-game IDE, workspace operations, and scripting into
  one full-stack player/creator pipeline, using Loo-Cast as the proving workspace.
- Ordered successor: none until the full-stack authoring pipeline reaches a
  coherent implementation checkpoint; keep this workstream foreground.
- Depends on: owner direction in the current session.
- Semantic mutation claim: the full authoring path across workspace and content
  discovery, shared Vapor operations, Workshop lifecycle, in-game IDE and script
  workbench, Rust-defined content/pack vocabulary, Rhai composition, and app-root
  tool ownership. Preserve completed Workshop work and integrate it into this goal.
- Path claims: `Vapor-CLI/**`, `Vapor/crates/**`, `Vapor-GUI/**`,
  `Vapor-Studio/**`, `Loo-Cast/**` (including
  `spacetime-engine/src/devtools/script_workbench/**`), `Vapor-Installer/**`,
  `Vapor-Steam-Entrypoint/**`,
  `.vapor/launch/**`,
  `resources/vapor/shell-scripts/**`,
  `App-Source.vapor.toml`, `README.md`, `AGENTS.md`,
  `.agents/workstreams/README.md`, `../SuperWorkspace.vapor.toml`,
  `resources/vapor/tools/**`,
  `.agents/workstreams/open/vapor-studio-launcher-design.md`,
  `docs/superpowers/specs/2026-10-14-vapor-repository-boundaries-design.md`,
  and this record. Do not claim unrelated local flight changes as part of this
  workstream. Build/test may run only through Vapor's managed commands at the
  owning workspace root; no raw Cargo. Deploy, publish, and Steam launch remain
  owner-controlled actions.

## Completed

## Current checkpoint

Pass 1's ownership review is complete. `vapor_client_domain` owns shared
application policy and install planning; `vapor_client_provider` in Vapor owns
filesystem/content/Workshop provider operations; Vapor CLI re-exports those
operations; the Vapor-owned application crate consumes the provider directly.
This removes the Studio Core -> CLI dependency inversion. Pass 2 is underway:
the application and service-host crates now live in Vapor, the managed service
binary is `vapor-client-service`, and its distribution validation uses the same
name. Current service startup still launches one private stdio child per
frontend; singleton bootstrap and multi-client IPC remain. The CLI also still
calls provider operations directly. Auto-Install's agreed behavior is latest
version compatible with all dependent compositions plus automatic local
collection at zero dependents; only the domain policy and durable preference
exist so far, not lifecycle execution. No build, test, deploy, commit, or push
was performed in this continuation.


- Added Vapor-owned `vapor_client_domain` and exposed its application-domain
  contract through `vapor_client`. The first shared policy covers the user's
  Library Auto-Install toggle: select the newest version satisfying every
  dependent composition, retain a manually managed version, and collect only
  auto-managed content with zero composition dependents. This policy is not
  authored content or a Workshop subscription. The preference and typed action
  are durable in shared client state; automatic update/collection execution
  remains future lifecycle work.
- Moved provider-neutral exact candidates, recursive graph resolution, and
  resolved node/composition types into `vapor_content`. `vapor_core` now adapts
  its generic local catalog into that shared candidate model and delegates
  graph traversal. The shared domain owns availability-aware install planning;
  the Vapor-owned local provider supplies workspace/cache/installed/Workshop
  candidates and provenance.
- Replaced the protocol's duplicate `ContentIdentity` with shared validated
  `ContentReference`, and moved composition/install persistence records plus
  composition naming/duplication/removal and installed-graph comparison into
  `vapor_client_domain`; composition and Library actions are domain-owned
  intents, and all former protocol action families now live in the same
  domain crate; protocol/client crates re-export those shared types. Added
  checked projection conversion between durable graph snapshots
  and the canonical `vapor_content::ResolvedComposition` without changing the
  JSON field layout.
- Tightened the shared authored role-manifest parser: content references now
  deserialize through `VaporId`, dependency constraints through `VersionReq`,
  and exact authored versions through `semver::Version`; workspace-inherited
  versions are parsed as semantic versions before source discovery consumes
  them. Source discovery/editing and deployed cache/installed artifact reads
  now use that parser. Existing editor/provider projections keep their string
  wire fields; the generic `LocalCatalog` and Workspace catalog adapt to
  shared exact candidates, with one shared install planner and separate local
  provider observations.
- Updated the implementation plan and service contract with the corrected
  Auto-Install meaning and a canonical ownership map. The protocol graph is a
  projection of the shared graph; the Library preference is durable and has a
  typed service action. No build or tests were run; user handles compilation
  and testing. Pass 1's ownership review is now complete: the dependency
  traversal is shared, and Studio Core no longer depends on CLI.

- Added a shared `vapor_core::authored_manifest` parser for role-specific
  source declarations and composition edges. Vapor Shell discovery and
  source-edit validation now use it; Workshop policy also has one shared
  representation. Both generic and role-specific catalogs feed shared exact
  candidates; the local provider and planner are separate. No build, tests, or
  deployment were run in this continuation.

- Preserved the current parent and submodule HEADs, tracked diffs, status, and
  untracked files in `/tmp/vapor-repo-boundary-preserve-20261014T123835Z`.
- Mapped the owner-approved repository direction: dissolve Vapor-SDK, rename
  Vapor-Shell to Vapor-CLI, Vapor-Launcher to Vapor-Studio, and
  Vapor-Entrypoint to Vapor-Steam-Entrypoint; create Vapor-GUI; and keep
  Vapor-Installer as a separate product. Drafted the repository-boundary design.
- Committed the pre-existing changes locally in Vapor, Vapor-CLI, Vapor-Studio,
  Vapor-SDK, Vapor-Steam-Entrypoint, and the parent workspace before migrating.
- Renamed local submodule checkouts to Vapor-CLI, Vapor-Studio, and
  Vapor-Steam-Entrypoint; created Vapor-GUI with its own local Git history.
- Moved the SDK core into Vapor and its CLI adapter into Vapor-CLI, exposing
  that adapter as `vapor sdk` in the unified terminal process. Removed the
  duplicate Launcher CLI/core packages from Vapor-Studio.
- Added Vapor-GUI mode dispatch for Studio, Play, and Installer. Installer UI
  calls Vapor's managed-state, install, repair, and uninstall operations; the
  existing `vapor-installer` remains the bootstrap fallback.
- Routed Steam Entrypoint GUI modes to `vapor-gui`, preserving its binary name
  and Steam launch-option arguments. Removed Vapor-SDK from the active parent
  submodule list while retaining its Git history in `.git/modules`.
- GitHub operations are still pending: the configured CLI token is invalid.
  No build, test, deployment, Steam launch, or publish was run.

- Read the requested Shell design and command docs.
- Read Vapor-Root README, AGENTS, planning startup files, and open workstream
  records.
- Confirmed target worktrees were clean before mutation.
- Added the Vapor-Shell content lifecycle module for discovery, validation,
  packaging, acquire/cache, install/update, verify, disable/enable, uninstall,
  repair/quarantine, fingerprints, locks, receipts, and Workshop create/publish/
  delete authority boundaries.
- Replaced public source selection/index commands with `source *` grammar in
  parser, host facade, diagnostics, docs, scripts, and tests.
- Added Loo-Cast Workshop intent metadata and two scripts:
  `content-roundtrip` and `content-publish-preview`.
- Added explicit `content subscribe` and `content download` verbs that share the
  controlled acquire/cache provider path until live SteamUGC is available.
- Added app-owned packagepack selection state with `content select`,
  `content selected`, and `content deselect`.
- Updated Vapor-Shell command, manifest, content, discovery, architecture,
  scripts, Steam development, and design docs.
- Updated Vapor-Root README as the human/product entry point and AGENTS with
  product-scope orientation.
- Verified `cargo test -p vapor_shell`.
- Verified real local proof through `/tmp/vapor-proof-app.AtkxhP/bin/vapor`:
  opened the real Loo-Cast checkout, ran `script run content-roundtrip`, and
  ran `script run content-publish-preview`.
- Re-ran `script run content-roundtrip` after adding subscribe/download verbs;
  it exercised package, subscribe/acquire, download/cache, install, verify,
  update, disable, enable, repair, and uninstall.
- Re-ran `script run content-roundtrip` after adding packagepack selection; it
  exercised selection write/read/clear through the installed binary.
- Resolved Workshop package layout to a single deployed artifact-root model:
  `content package` stages directly under `output/content/packages/<slug>`,
  writes a resolved deployed `Vapor.toml` at the artifact root, and no longer
  emits a separate `Vapor-package.toml` or `payload/` wrapper.
- Preserved the source/runtime distinction: custom content authoring remains
  workspace-bound, while packaged, installed, cached, and Workshop-downloaded
  runtime artifact roots carry deployed `Vapor.toml` metadata.
- Updated package, cache, install, and publish paths so Workshop VDF
  `contentfolder` points at the artifact root, cache installs read deployed
  `Vapor.toml`, dry-run package fingerprints match deployed artifacts, and
  Workshop tags are de-duplicated.
- Updated Vapor-Shell content, command, manifest-schema, and Workshop design
  docs plus content lifecycle tests for the deployed `Vapor.toml` model.
- Rebuilt and installed the Loo-Cast app-local `vapor` binary, then verified
  `script run content-roundtrip` and `script run content-publish-preview`
  against that final binary.
- Exposed the full `content *` and `root *` command surfaces through the direct
  app-local host facade so live authority operations can run without relying on
  the interactive shell PTY.
- Implemented controlled live SteamCMD-backed Workshop create, publish, and
  numeric PublishedFileId download/import paths for the current Steam provider
  bridge.
- Created real Loo-Cast Workshop items with build account `ghf_vapor_build`:
  `spacetime-engine` -> `3762162251`, `loo-cast-game` -> `3762162407`, and
  `loo-cast-packagepack` -> `3762162548`.
- Recorded those PublishedFileIds in the three Loo-Cast content `Vapor.toml`
  files.
- Published all three items through Steam once with the manifests that were
  then authored as `visibility = "private"`.
- Found the Workshop visibility contradiction: Steam had accepted private VDFs,
  so the items existed under the publisher account but were not visible in the
  public Workshop surface.
- Updated the three Loo-Cast manifests to `visibility = "public"` and generated
  public Workshop VDFs for the same PublishedFileIds.
- Added `content publish ARTIFACT...` so multiple Workshop VDFs can be uploaded
  through one SteamCMD provider session instead of one login per artifact.
- Rebuilt and installed that batch-publish binary into the live Loo-Cast Steam
  app folder.
- Retried the public visibility republish after the Steam auth cooldown. One
  SteamCMD provider session logged in as `ghf_vapor_build` and committed all
  three `workshop_build_item` updates successfully with `visibility = "public"`
  and change note `Set initial Vapor Workshop items public.`.
- Recorded the successful public republish receipts at timestamp `1783766095`
  for all three artifacts.
- Downloaded/imported the real `spacetime-engine` Workshop item
  (`3762162251`) into the app-owned cache after fixing SteamCMD download-folder
  resolution from the shared Steam workshop directory.
- Updated cached install recursion so a Workshop-downloaded packagepack can
  install required dependencies from app-owned cache entries after source close.
- Verified the three public Workshop items are visible on Steam after the
  app-level/item-level visibility gates were corrected.
- Added batch `content download TARGET...` and used one SteamCMD provider
  session to download/import `3762162251`, `3762162407`, and `3762162548` into
  the app-owned cache.
- Closed source, installed the Workshop-downloaded packagepack from cache with
  dependency recursion, selected it, verified, updated, disabled/enabled,
  repaired, and explicitly uninstalled packagepack, game, and engine. Installed
  state ended empty; cache/index state remained available.

## Vapor Studio service boundary

- Added a child service with typed profile state, explicit host context,
  one-time migration from egui storage, and atomic durable profile/draft/history
  persistence.
- Routed profile CRUD, root assignment, launch arguments, content invalidation,
  default-root recovery, and workspace activation through service actions.
- Added a separate profile revision so catalog snapshots do not overwrite
  profile operation results; runtime installation proof is available in live
  protocol snapshots and cleared when saved or reloaded.
- Moved composition preview, install, verify, repair, update, source run, and
  Play into the service's asynchronous operation lifecycle. The service now
  owns resolved profile graphs, installation proof, and post-operation saves.
- Added typed composition preview data and profile-scoped operation events to
  the protocol. Fixture-mode preview and install exercise the same service
  route without Steam or game-process effects.
- Removed the client action that could submit a fabricated installed graph;
  only service execution can establish that state. Default Packagepack
  selection is now resolved and persisted by the service as well.
- Terminal operation success is emitted only after the service applies and
  saves the profile result; tests cover one terminal event and verification
  failure reconciliation.
- Vapor-managed Studio validation passes: 49 tests, Clippy, and Rustdoc.
- Moved workspace file discovery, bounded reads, conflict-checked writes, and
  safe new-file creation behind service-owned workspace actions and typed
  results. The GUI editor no longer reads or writes workspace files itself.
- Moved source composition, Workshop metadata, runtime output declarations,
  content validation, registered-source discovery, content scaffolding, and
  starter workspace creation behind async service authoring actions. The UI
  submits typed intent and applies service results; the service owns the Vapor
  Shell calls and workspace file effects.
- Moved workspace validation, registration, content adoption, and
  unregistration behind the service operation lifecycle. Unregister results
  carry the removed content identity so affected profile state is invalidated.
- Moved workspace path validation and initial file discovery into service
  actions. The UI now asks the service to open a folder and then requests its
  file list through the same operation/result path as later editor requests.
- Preserved full source-link semantics across the authoring protocol
  (relationship, optionality, version, Workshop ID, and reason), added complete
  Workshop policy fields, workspace-bound stale-result rejection, and raised
  the protocol version for the wire-shape change.
- Removed the GUI's duplicate local-worker result variants for those authoring
  operations. Vapor-managed formatting passed after the refactor. Compilation,
  tests, runtime exercise, deployment, and Steam actions were intentionally
  left to the owner for this tranche.
- Remaining service-boundary gaps found in the audit: Steam root publishing
  still has GUI-owned operation flow, and root publishing, app bootstrap, and
  local toolchain actions need a final ownership review. Address those before
  the end-to-end operation-flow review. The current authoring changes are not
  yet compile- or runtime-verified.
- Cancelled the first real `root publish` attempt after SteamPipe scanned about
  2.6 GiB. The staged root depot was runtime plus `packages/setup`; the setup
  payload alone contained the Rust toolchain, Cargo home, Git, and SteamCMD.
- Split root depot staging into runtime default and explicit stacked setup mode:
  default `root package`/`root publish` stages runtime-only app payload;
  `--include-setup-payload` is now required to stage `packages/setup`.
- Added `[workspace].binaries` so Vapor-Shell declares `binaries = ["vapor"]`
  in its `Vapor.toml`; `root build` now promotes the installed shell through
  the manifest-driven Vapor workflow.
- Used the existing source-controlled bootstrap script once as a bridge because
  the previously installed binary could not yet parse `[workspace].binaries`.
  After that, `vapor root build` promoted one `vapor` binary through Vapor.
- Re-ran `root publish --dry-run`; staging reported `payload: runtime only`,
  staged 160 MiB under `output/root/content`, and contained only the app
  runtime surface.
- Ran the real runtime-only SteamPipe root upload interactively as
  `ghf_vapor_build`. SteamPipe successfully finished AppID `2122620` build
  `24160810`; depot `2122621` uploaded 1070 chunks, created manifest
  `8985564292651668226`, and staged 1427 files / 157.04 MiB. `packages/setup`
  was absent from the real publish staging tree.
- Verified the freshly reinstalled Steam app root from the runtime-only depot:
  it contains the shipped runtime surface, and `bin/vapor setup self status`
  correctly reports an unregistered app root plus missing app-local Rust, Git,
  SteamCMD, and self-setup payload until `setup self install` is run.
- Added explicit local-only development deploy verbs:
  `root deploy [--skip-docs]` builds/promotes root binaries and docs into the
  Steam app root without SteamPipe, and `content deploy ARTIFACT [--select]`
  builds/installs source content into app-owned installed content without
  Workshop publication.
- Added content runtime output declarations in `Vapor.toml`: authored and
  deployed content manifests can carry `binaries` and `libraries`; package and
  deploy copy declared Cargo outputs into deployed artifact `bin/` and `lib/`
  directories instead of inventing a proprietary payload metadata format.
- Declared `spacetime-engine` as a content-shipped executable through
  `spacetime-engine/Vapor.toml` with `binaries = ["spacetime-engine"]`.
- Removed the leftover public-source-navigation implementation surface after
  the `cd`/`up`/`pwd`/`ls` command removal: the shell now keeps source
  selection/root context without retaining dead path-changing helpers or docs
  that describe them as user-facing commands.
- Bumped the canonical Vapor Rust toolchain intent to stable Rust `1.97.0`
  dated `2026-07-09`, retaining the channel/date model for dated channels but
  using the release version as the stable install identifier.
- Verified focused checks after the local deploy/runtime-output/toolchain
  changes: `cargo fmt --check` in Vapor, Vapor-Shell, and Vapor-SDK;
  `cargo clippy -p vapor_core --all-targets -- -D warnings`;
  `cargo test -p vapor_core`;
  `cargo test -p vapor_shell --test command --test content`;
  focused Vapor-Shell state/command/installation/content integration tests;
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`; and
  `cargo check -p vapor_sdk_cli`.
- Corrected the immediate Steam Shell launch slice: bare `bin/vapor` now
  detects a GUI/no-terminal start, spawns a real terminal emulator, runs Vapor
  there, then starts an interactive shell so the window remains usable until the
  user closes it manually.
- Tightened the Steam Shell terminal path after live testing: KDE sessions now
  prefer Konsole, the spawned terminal gets the app `bin/` directory in `PATH`,
  terminal launch attempts are logged under `.vapor/logs/terminal-launch.log`,
  and the original Steam-launched Vapor process waits for the terminal session
  so Steam does not tear the window down after startup.
- Simplified the immediate Steam Shell launch path further to Linux/Konsole
  only: `/usr/bin/konsole` is now the sole terminal target, Steam runtime linker
  variables are scrubbed before launching it, and the runner skips the fallback
  interactive shell when a terminal close signal is observed.
- Replaced the noisy REPL startup diagnostics with a compact first-run overview:
  `Status`, one `Next` command, an optional `Then` command, and a short
  `help`/`exit` reminder. Detailed setup and package-payload diagnostics remain
  in explicit setup commands instead of appearing on every Shell launch.
- Began applying the same UX shape beyond startup: `setup self status`,
  `source status`, `source list`, `source add`, `source open`, `source close`,
  empty `content verify`, and top-level host help now use clearer `Status` /
  `Next` sections and avoid default diagnostic dumps where normal runtime users
  do not need them.
- Found and repaired the installed docs breakage: the aggregate docs index
  linked to per-project `index.html` files that the docs builder did not
  generate. Vapor Shell now writes a section index after each Rustdoc copy, and
  the installed app docs were repaired and verified locally over HTTP.
- Rebuilt `vapor` from source and copied it into the Steam app root because the
  fresh runtime-only app root still lacks self-setup tools and cannot run the
  normal `root deploy` path yet.
- Corrected the workspace/content project model boundary: `[workspace]`
  manifests now register Vapor-managed child projects through
  `[[workspace.projects]]`, while each child `Vapor.toml` owns its own
  `[project]` or content identity. Content discovery now uses those
  registrations instead of recursively guessing publishable artifacts from
  arbitrary nested manifests.
- Registered the three live Loo-Cast first-party content projects in
  `Loo-Cast/Vapor.toml`: `spacetime-engine`, `loo-cast-game`, and
  `loo-cast-packagepack`. The real public PublishedFileIds remain in the child
  content manifests.
- Updated `metadata`, `source status`, and `source repair` output to separate
  Cargo workspaces from registered Vapor projects, so the shell no longer
  presents Cargo package membership, source roots, and content artifacts as one
  implied concept.
- Verified the registered-project slice with `cargo fmt --check`,
  `cargo test -p vapor_shell --test workspace --test content --test metadata --test command`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`, and
  `cargo build -p vapor_shell --bin vapor`; then copied the rebuilt binary into
  the Steam app root and smoked installed `source open`, `metadata`,
  `source status`, `source repair`, and `content list` against the real
  Loo-Cast source.
- Reworked Vapor Studio into a core/UI boundary with profile, catalog, install
  planning, operation-center, authoring, and workspace views; added managed
  content validation, safer content/workspace creation, stale-file conflict
  detection, explicit editor reload, and a navigable workspace file tree.
- Source-reading/build/install/stage actions now refuse to run while the editor
  has unsaved changes, and the UI disables those actions with save/reload
  guidance. Install planning now recognizes an already-cached root artifact.
- Workshop discovery remains numeric-ID download only. The Rust `steamworks`
  crate provides the needed client UGC query and subscription APIs, so a manual
  SDK-header download is not a prerequisite. The remaining integration work is
  adding the crate and packaging its required Steam API runtime redistributables
  for Linux and Windows; SteamCMD remains available for existing acquire and
  publishing flows.
- Owner requested no compile or test runs for this Studio tranche; only static
  diff/whitespace checks are permitted until the owner verifies it locally.
- Added the first product Play entrypoint as a Vapor Shell one-shot command:
  `launch loo-cast`. Steam launch options should target the platform wrapper
  under `.vapor/launch/<platform>/vapor.*`; Play uses wrapper argument `play`
  and Shell uses wrapper argument `shell`.
- Added app-root first-party content seeds in `Vapor-Root/Vapor.toml` under
  `[[root.content]]` for the public Loo-Cast engine, game, and packagepack
  Workshop items. This lets a runtime-only install resolve default content
  before any source checkout or registry client is open.
- Changed Workshop acquisition to use SteamCMD `anonymous` login by default
  when no account is passed, so public Workshop downloads are attempted instead
  of rejected before provider execution.
- Changed missing-cache install behavior so `content install` can resolve a
  root content seed, download/cache the Workshop item, then continue the normal
  install path. Cached packagepack dependencies now install by dependency
  `workshop-id` when present, falling back to content ID/seed lookup.
- Added `workshop-id` hints to the Loo-Cast packagepack's engine and game
  references so the next packagepack Workshop update can carry dependency
  download hints directly in its deployed `Vapor.toml`.
- Verified the launch/Workshop seed slice with `cargo fmt --check`,
  `cargo test -p vapor_shell --test command --test content --test metadata`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`,
  `cargo build -p vapor_shell --bin vapor`, and `git diff --check`; then copied
  the rebuilt binary and updated root `Vapor.toml` into the Steam app root and
  smoked installed `vapor launch --help`.
- Added `--account ACCOUNT` to `launch loo-cast` and `content install` so
  unreleased/private Workshop testing can use `ghf_vapor_build` or another
  account with app/content access instead of relying on anonymous SteamCMD.
  The selected account is threaded through root-seed acquisition and dependency
  install recursion.
- Corrected the first playable terminal runtime proof after rejecting the
  invalid `game.runtime` model: `loo-cast-game` now declares its engine through
  `[game.engine]`, depends on `spacetime-engine` through Cargo, ships
  `libloo_cast_game.so`, and owns the WIP game definition in Rust code. The
  installed `spacetime-engine` binary resolves the packagepack's game, verifies
  the game's engine dependency, loads the installed game library, and calls a
  Rust-native entrypoint exported from that library.
- Locally deployed the updated Loo-Cast packagepack into the Steam app and
  verified `vapor launch loo-cast` hands off to the new terminal runtime.
- Fixed `content deploy ARTIFACT --select` so selection falls back from the
  authored/source selector to the installed packagepack artifact ID emitted by
  the deploy reports.
- Published the refreshed Loo-Cast Workshop items again after the terminal
  runtime and wording fixes. Verified from an uninstalled/cacheless app state
  that `launch loo-cast --account ghf_vapor_build` downloaded all three live
  Workshop IDs, installed dependencies, selected the packagepack, and reached
  the game handoff.
- Published the refreshed runtime-only Steam app/depot to `vapor-dev` as build
  `24258893`, depot manifest `7291224128649500980`.
- Added a first friend-grade source bootstrap path: `source init basic-content
  PATH --organization ORG --name NAME [--app-id APPID]` creates a normal
  Cargo/Vapor workspace with engine, game, and packagepack projects; it writes
  a lockfile so app-local `--locked` Cargo workflows work immediately.
- Added `source repair --write` source metadata repair for dependency
  `workshop-id` hints derived from sibling artifacts' own
  `published-file-id` values. This supports the first Workshop publication
  loop where engine/game PublishedFileIds must be copied into dependent
  game/packagepack metadata before dependent items are created or updated.
- Registered `Vapor-Examples` projects as real content examples and staged
  `examples/vapor-examples/` in the runtime app payload while excluding `.git`
  and `target`.
- Published the template/examples app/depot slice to `vapor-dev` as Steam build
  `24259447`, depot manifest `3853568432132708180`, baseline
  `7291224128649500980`.
- Verified the template slice with focused Vapor-Shell tests, strict clippy,
  Vapor-Examples checks/tests, installed-app `source init`, `content validate`,
  `content deploy ... --select`, `content verify`, and `root package` staging
  that includes examples without `.git` or `target`.
- Extracted the terminal runtime proof out of the first-party Loo-Cast content
  workspace and into `Vapor-Examples` as three explicit example projects:
  `terminal-engine`, `hello-world-on-steroids-game`, and
  `hello-world-on-steroids-packagepack`.
- Gutted the current first-party Loo-Cast Rust payload back to product
  placeholders while preserving the engine binary and game library outputs so
  the packagepack still has real installable runtime artifacts.
- Updated Loo-Cast, Vapor-Examples, and Shell docs so the dynamic terminal
  proof is no longer framed as the Loo-Cast product runtime.
- Verified the extraction with `cargo fmt --check`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff
  --check` in the touched content workspaces/docs.
- Fixed source selection for explicitly opened nested source workspaces:
  ambient invocation from root submodules still resolves to the containing
  Vapor-Root, but `source open /path/to/Vapor-Examples` and saved source state
  now honor the exact nested `[workspace]` marker.
- Verified installed-app local deploy smoke after the topology correction:
  opened `Vapor-Examples`, validated 6 registered content projects, locally
  deployed `hello-world-on-steroids-packagepack`, verified installed content,
  opened `Loo-Cast`, validated the 3 product placeholder artifacts, locally
  deployed/selects `loo-cast-packagepack`, verified content, and ran
  `launch loo-cast` with terminal relaunch suppressed. The handoff reached the
  placeholder Spacetime Engine message.
- Added a Windows `LoadLibraryW`/`GetProcAddress` dynamic-loader path to the
  terminal engine example so the example runtime is no longer Linux-only in
  source. The Linux host only has Linux Rust targets installed, so the Windows
  cfg still needs to be compiled on the owner's Windows/MSVC machine.
- Added minimal OS-facing Steam launch wrappers under
  `.vapor/launch/linux/vapor.sh` and `.vapor/launch/windows/vapor.cmd`. The
  wrappers only select Play vs Shell mode and hand off to the installed
  target-specific `bin/<target>/vapor[.exe]`; Vapor Shell remains the central
  implementation surface.
- Changed content runtime output staging from flat `bin/`/`lib/` paths to
  target-specific `bin/<target>/` and `lib/<target>/` directories. Deployed
  `Vapor.toml` files now record actual staged files through target-specific
  `runtime` entries.
- Added `content package/create/publish --target TARGET` support, including
  repeated `--target` values for one package/Workshop item containing multiple
  platform payloads. Focused tests cover explicit Windows/MSVC package staging
  and a two-target Workshop dry-run payload.
- Extended the same target-specific layout to root/app binaries:
  release-mode Vapor Shell binaries are promoted and staged under
  `bin/<target>/vapor[.exe]`, while `.vapor/launch/...` wrappers are the stable
  Steam-facing entrypoints. Installation discovery now accepts both bootstrap
  `bin/vapor[.exe]` and release `bin/<target>/vapor[.exe]`.
- Added repeatable `root build --target TARGET`, `root deploy --target TARGET`,
  and `root publish --target TARGET` for root/app binary builds. Depot smoke
  now rejects shipped launch wrappers when the matching target-specific Vapor
  binary is missing.
- Added `[root.runtime].targets` and `[workspace.runtime].targets` as the
  canonical release target matrices. `root *` and content build/deploy/package/
  create/publish now accept `--release-targets` to consume those matrices,
  while no target flag remains host-only for quick local Linux iteration.
- Root/depot staging now copies selected `bin/<target>/` directories instead
  of the whole `bin/` tree and copies only launch-wrapper platform directories
  implied by the selected targets.
- Captured the Steam installscript decision: because Valve installscript
  support does not provide the same Windows/Linux/SteamOS bootstrap behavior,
  Vapor does not use `installscript.vdf` for runtime bootstrap.
- Added `Vapor-Installer` as a root-local Cargo/Vapor project with a
  visual-first no-argument installer surface plus narrow headless
  `install`, `uninstall`, and `dev-env` install/uninstall/status commands.
- Implemented installer-owned player-mode install for app-local Git, SteamCMD,
  `.vapor/registry` Vapor-Registry checkout/update, installer logs,
  diagnostics directories, content cache/download directories, and receipts.
- Implemented explicit installer-owned development environment install/uninstall
  for Rust/Cargo and cross-build tooling; normal player-mode install does not
  install Rust/Cargo. Top-level `vapor-installer uninstall` removes both
  developer-mode tooling and player-mode mutable app-root state, while
  `dev-env uninstall` is only a downgrade back to player mode.
- Updated Steam launch wrappers so Play/Shell run headless
  `vapor-installer install --app-root <app-root>` before handing off to
  target-specific `vapor`, reporting installer failure through the first
  visible Shell with the installer log path.
- Removed the user-facing Shell `setup` command group. Metadata/workflow
  diagnostics point to `vapor-installer`, and normal workflow validation no
  longer blocks on legacy app-root acceptance state.
- Removed root setup-payload staging from Shell and SteamPipe generation:
  `root package`/`root publish` are runtime-only and no longer accept
  `--include-setup-payload`.
- Updated distribution staging/tests so root-local Vapor projects such as
  `Vapor-Installer` are discovered even before they become Git submodules, and
  staged target directories include `vapor-installer` alongside `vapor` when
  promoted.
- Refactored `Vapor-Installer` from monolithic source files into documented
  modules for CLI/TUI, model, app-root discovery, acquisition, filesystem
  confinement, Git bootstrap, runtime bootstrap, development environment, and
  path policy.
- Initialized and pushed the standalone `GHF-Studios/Vapor-Installer` Git
  repository at `b0e9e78`, then registered `Vapor-Installer` as a
  `Vapor-Root` submodule.
- Verified installer and Shell slices with `cargo fmt --check`, focused
  Vapor-Shell command/workspace/distribution tests, strict
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`,
  `cargo test` and strict Clippy in `Vapor-Installer`, plus a local
  `vapor-installer uninstall --dry-run` preview against the local Steam app
  root.
- Built/promoted Linux and Windows runtime-matrix binaries for `vapor` and
  `vapor-installer` into the local Steam app root, staged a runtime-only depot
  containing both platform launch wrappers and no `packages/setup`, and
  published AppID `2122620` to `vapor-dev` as build `24274710`; depot
  `2122621` manifest `6914390332829415382`.
- Ran the newly shipped local `vapor-installer install` against the
  local Steam app root. Local player-mode status is ready: app-local Git,
  SteamCMD, Vapor-Registry, and generated state directories are present.
- Republished the simplified installer command model to Steam AppID `2122620`
  branch `vapor-dev` as build `24275153`; depot `2122621` manifest
  `7777478068318923421`, baseline manifest `6914390332829415382`.
- Added a first-class Steam Installer wrapper mode. Linux/Windows wrapper
  argument `installer` opens `vapor-installer` directly and skips the quiet
  player-mode install used by Play/Shell. Published to Steam AppID `2122620`
  branch `vapor-dev` as build `24275599`; depot `2122621` manifest
  `8367995713019312839`, baseline manifest `7777478068318923421`.
- Began the hard pivot away from the mono-depot runtime shape: `Vapor.toml`
  now requires `[root.steam.depots.<name>]` with common, Linux, and Windows
  depot IDs before root publication can proceed.
- Implemented split root staging under `output/root/content/{common,linux,windows}`:
  common receives OS-neutral app files, Linux receives Linux launch/runtime
  files, and Windows receives Windows launch/runtime files plus required runtime
  DLLs.
- Updated SteamPipe generation so `root publish --dry-run` writes one app-build
  VDF plus one depot-build VDF per staged depot, and command output prints the
  staged depot roots and Steamworks OS rule expectation.
- Updated distribution, command, manifest, and Steam development docs for the
  package/depot pivot and current launch-option model.
- Verified the split-depot Shell slice with `cargo fmt --all --check`,
  `cargo test -p vapor_shell --test distribution`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`, and
  `cargo test -p vapor_shell --all-targets`.
- Recorded the real Steamworks DepotIDs in `App-Source.vapor.toml`: common
  `2122621`, Linux `2122622`, and Windows `2122623`.
- Replaced hardcoded depot file membership in Shell staging with manifest-owned
  depot include lists. Each depot now declares its own `include` entries with
  source/installation roots, from/to mappings, exclusions, and optional runtime
  target filters.
- Re-verified the manifest-driven depot staging slice with
  `cargo clippy -p vapor_shell --all-targets -- -D warnings` and
  `cargo test -p vapor_shell --all-targets`; root `Vapor.toml` also parses as
  valid TOML.
- Moved generated SteamPipe VDF file bodies out of Rust format strings and into
  checked-in templates under `resources/steam/steampipe-templates/`.
- Removed the bulky distribution staging tests and kept validation on real
  staging/dry-run command paths instead of historical negative-path assertions.
- Clarified the Windows runtime DLL contract: `libunwind.dll` is still required
  for the Windows GNU runtime, but build promotion/import must place it under
  `bin/x86_64-pc-windows-gnullvm/`; depot staging includes that runtime bin
  directory instead of copying directly from development toolchain paths.
- Removed the obsolete Shell self-setup package copy/status module. Setup
  packages are no longer reported in metadata and are no longer an alternate
  install path; install/uninstall lifecycle belongs to Vapor-Installer.
- Re-verified after the template/sample cleanup with `cargo fmt --all --check`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`, and
  `cargo test -p vapor_shell --all-targets`.
- Split app-source and installed-app root manifests: Vapor-Root now uses
  `App-Source.vapor.toml`; the shipped app root uses `App.vapor.toml`; ordinary
  workspaces/content/registry still use `Vapor.toml`.
- Removed source-root `.vapor` payloads. Source-controlled launch/scripts now
  live under `resources/vapor/shell-scripts/` and
  `resources/vapor/vapor-scripts/`; installed app-root `.vapor/` is reserved
  for generated disposable state.
- Removed Shell-owned public setup command surfaces, setup package staging,
  PATH/location registration, and the remaining setup install/repair/uninstall
  implementation. Shell now only inspects app-local tool readiness and points
  missing prerequisites at Vapor-Installer.
- Re-verified the renamed manifests, resource layout, and setup trim with
  `cargo fmt --all --check`, `cargo clippy -p vapor_shell --all-targets -- -D
  warnings`, and `cargo test -p vapor_shell --all-targets`.
- Re-verified Vapor-Installer with `cargo fmt --all --check`, `cargo clippy
  --all-targets -- -D warnings`, and `cargo test --all-targets`.
- Ran a temp app-root staging smoke with fake app-local Rust/Git readiness:
  `root package --host-only` staged common and Linux depots, then
  `root publish --dry-run --skip-build` generated app/common/linux/windows VDFs
  and confirmed the Windows depot stages `vapor.exe`, `vapor-installer.exe`,
  and `libunwind.dll`.
- Simplified Steam launch ownership: `vapor-entrypoint[.exe]` now owns only
  terminal launch and argument forwarding, `bin/vapor-launch.*` owns
  launch-target dispatch and terminal hold behavior, and Vapor Shell no longer
  performs its own no-terminal relaunch.
- Replaced launch wrapper environment IPC with explicit wrapper arguments and
  app-local bootstrap failure state under
  `.vapor/state/installer/bootstrap-failure.txt`.
- Routed installer-launched helper stdout/stderr through
  `<app-root>/.vapor/logs/installer.log` so launch-time player bootstrap no
  longer leaks curl/tar/PowerShell/rustup output into the Steam-visible shell.
- Drafted an unapproved private-test diagnostics rewrite away from
  Git/Vapor-Registry transport:
  `--send-diagnostics` now captures a local structured run under
  `.vapor/diagnostics/runs/YYYY-MM-DD/<unix_timestamp>-<platform>-<short-random>/`
  with `metadata.toml`, `vapor.log`, and `.vapor/diagnostics/latest.toml`.
- Drafted diagnostics metadata for run identity, coarse platform, app root,
  executable, startup/direct/script context, installation identity/readiness,
  selected source, launch target, selected packagepack, engine handoff,
  command/script steps, redacted errors, upload request state, and exit state;
  diagnostics deliberately avoids hostnames and persistent machine identifiers.
- Drafted removal of normal UX for arbitrary `--diagnostics-registry PATH` and Git-backed
  diagnostics push. `diagnostics upload` now uses a future HTTP-server transport
  seam that currently reports not configured; `diagnostics upload --dry-run`
  previews the local run and transport without sending.
- Drafted redaction for obvious password/token/secret/key/auth/ticket/credential/
  cookie arguments and log text, including entrypoint forwarded-argument logs;
  stopped capturing broad diagnostics registry environment values, avoided
  full-command launch-wrapper logging, and disallowed real diagnostics uploads
  from scripts.
- Drafted launch PATH boundary cleanup: Steam launch scripts and the Linux
  entrypoint now expose only app-facing `bin/<target>` and `bin` paths, while
  workflow child commands filter inherited app-local raw tool paths and invoke
  low-level tools by exact app-local paths instead of PATH exposure.
- Verified the diagnostics/PATH slice with app-local Cargo by absolute path:
  `cargo fmt --all --check` in Vapor-Shell and Vapor-Entrypoint,
  `cargo test -p vapor_shell --lib`,
  `cargo test -p vapor_shell --test command`,
  `cargo clippy -p vapor_shell --all-targets -- -D warnings`,
  `cargo test --all-targets` in Vapor-Entrypoint,
  `cargo clippy --all-targets -- -D warnings` in Vapor-Entrypoint, and
  `git diff --check` in Vapor-Root, Vapor-Shell, and Vapor-Entrypoint.
- Captured the diagnostics, registry, server, privacy, GitHub issue strategy,
  launch UX, installer lifecycle, and app-local tooling planning surface as
  unprocessed planning intake under
  `docs/planning-intake/source-corpus/raw-input/vapor-diagnostics/`.
- Captured the intended Vapor server-root topology as unprocessed planning
  intake: `Vapor-Server-Root` beside `Vapor-Root`, with service submodules for
  homepage, docs, identity, and diagnostics, single-domain path routing,
  composed export/import, and future MCP/ACP capability-surface notes.
- Initialized the new Vapor server repository set locally and on GitHub:
  `Vapor-Homepage-Server`, `Vapor-Docs-Server`,
  `Vapor-Identity-Server`, and `Vapor-Diagnostics-Server` each have initial
  README/AGENTS scaffolds, while `Vapor-Server-Root` has the orchestration
  scaffold, route map, example Caddy config, systemd notes, `server-root.toml`,
  and service submodules pointing at the pushed service commits.
- Added and pushed minimal no-dependency Rust service binaries for the four
  server services: homepage serves health/home/license/Impressum placeholders,
  docs serves health/current docs plus token-protected current-doc upload and
  export, identity serves health/status plus token-protected init/export
  scaffolds, and diagnostics serves health plus opt-in upload and
  token-protected list/download/export scaffolds. `Vapor-Server-Root` now
  points its submodules at those service commits.
- Migrated the four server service binaries from hand-rolled standard-library
  HTTP parsing to Axum/Tokio scaffolds, added service `.gitignore` files for
  `target/` and local `state/`, and advanced `Vapor-Server-Root` gitlinks to
  those commits. Updated local RustRover project metadata so the new server
  root and service submodules are visible as Git/Cargo project roots.
- Added `Vapor-Server-Root/docs/decisions-and-backlog.md` to persist the server
  boundaries, current scaffold behavior, near-term backlog, non-goals, and open
  questions. Corrected the local workspace shape so service repositories are
  checked out as root-level `Vapor-Server-Root/Vapor-*-Server` submodules,
  matching `Vapor-Root`, not as sibling top-level checkouts.
- Recorded the current VPS baseline direction without secrets: direct
  Caddy/systemd hosting can use Ubuntu 26.04 LTS, while choosing Plesk would
  require a currently supported Plesk OS such as Ubuntu 24.04 LTS.
- Confirmed key-based root SSH reachability to the new Ubuntu 26.04 VPS without
  persisting secrets, moved `Vapor-Identity-Server` from an ad-hoc registry-file
  scaffold to SQLite/SQLx schema bootstrap, and added first-pass
  `Vapor-Server-Root` deployment automation for no-Plesk Caddy/systemd hosting,
  root-level submodule deployment, server-local env files, SQLite state, systemd
  units, Caddy routing for `vapor.ghf-studios.site`, and local health checks.
- Ran the first real VPS bootstrap/deploy on Ubuntu 26.04: installed Caddy,
  Rust/Cargo/build dependencies, created the `vapor` system user, generated
  server-local env/token files, cloned/built the root-level service submodules,
  installed/restarted systemd units, initialized the SQLite identity database,
  tightened state permissions, and verified Caddy plus all four Vapor services
  running with local health checks. DNS for `vapor.ghf-studios.site` remains
  the external/public-HTTPS blocker.
- Added and deployed domain-independent operations scripts: pre-DNS public HTTP
  fallback routing, SSH/UFW hardening, public HTTP health checks, token-protected
  docs upload smoke testing, and diagnostics upload/redaction smoke testing.
  Verified key-only SSH still works, UFW allows only SSH/HTTP/HTTPS inbound,
  Caddy serves the IP fallback, docs placeholder upload succeeds, and diagnostics
  redaction is applied on disk.
- Added and deployed `Vapor-Server-Root` automatic branch polling through
  `vapor-deploy.timer`. The VPS now stores non-secret deployment settings in
  `/etc/vapor-server/root.env`, the timer is enabled/active, the first
  timer-triggered deploy completed successfully against `main`, and public/local
  health checks still pass.
- Added and deployed root-level state export/restore scripts in
  `Vapor-Server-Root`. Verified a real root-only VPS export bundle under
  `/var/backups/vapor-server` containing `/var/lib/vapor-server` state,
  manifest metadata, docs state, identity SQLite state, and diagnostics state,
  explicitly excluding `/etc/vapor-server` env/token files. Restore exists as an
  explicit `--yes` operation and has not been run against the live VPS.
- Added and deployed the real pre-DNS docs deploy path: `Vapor-Docs-Server`
  accepts safe `tar.gz` docs-tree uploads and serves static docs assets, while
  `Vapor-Server-Root` has docs bundle build/deploy helpers. Deployed the curated
  Vapor docs bundle from `Vapor-Root` through the public HTTP route
  `http://82.165.77.104/docs/v1/current.tar.gz`; `/docs/`, mdBook pages, Shell
  markdown, and docs metadata are reachable through the temporary IP fallback.
- Added and deployed `Vapor-Identity-Server` real verification seams:
  Steam ticket verification via server-side Steam Web API config, GitHub token
  verification via GitHub API config, fail-closed readiness when credentials are
  missing, admin profile listing, and a read-only Basic-auth dashboard. Closed
  pre-alpha public HTTP access to the dashboard was verified through
  `http://82.165.77.104/api/identity/admin`; HTTPS remains the DNS-ready pivot.
  Server-local identity env keys were ensured on the VPS without printing
  secrets; real Steam/GitHub credentials are still unset.
- Added GitHub Actions deploy-trigger plumbing in `Vapor-Server-Root` and a VPS
  script for a restricted `vapor-gh-actions` deploy user. Generated a dedicated
  local deploy key outside the repo, installed only its public key on the VPS,
  and verified that the restricted key can run `sudo -n systemctl start
  vapor-deploy.service` successfully. The GitHub Actions workflow now no-ops
  successfully until deployment secrets are configured. GitHub CLI auth is
  currently invalid, so repository secrets and branch protection remain a
  dashboard/re-auth step.

## Owned uncommitted changes

- The current Vapor, Vapor-CLI, Vapor-GUI, Vapor-Installer,
  Vapor-Steam-Entrypoint, and Vapor-Studio edits plus `App-Source.vapor.toml`
  are owned by this foreground workstream and are being checkpointed in the
  owner-authorized 2026-10-15 commit/push. Existing tracked lockfile changes
  are included and preserved.

## Current Studio pass — 2026-10-15

- Owner approved the full-stack Studio redesign plan and asked to stack work in
  the existing checkout; compile/test remains deferred to Leslie.
- Added a `vapor_studio_core` crate for versioned composition profiles,
  migration/recovery, operation identities, and Shell-backed installation
  coordination. Reorganized the Studio UI into Home, Discover, Library, Create,
  Workspace, and secondary Diagnostics destinations with a shared activity area.
- Added local catalog browsing with exact Packagepack version selection, profile
  resolution, install/verify/repair actions, a basic workspace file editor, and
  typed content-manifest creation for Rust-defined content kinds.
- Added exact-root local resolvers in Vapor Core and exact `id@version` selection
  to Shell content discovery and installed verification.
- Moved profile-root resolution, resolved-graph snapshots, and authored content
  workspace creation into Studio Core. Shell dependency installation now chooses
  local artifacts that satisfy declared semver requirements and returns each
  installed version to Studio; it reports when a graph requires two versions of
  one ID because the current app installation index cannot store both.
- Added a Studio content-detail action that stages a real Shell package for
  Workshop review and displays its metadata, dependency declarations, payload
  file list, target, size, and fingerprint. It does not upload.
- Editor transitions now preserve unsaved text by blocking profile/workspace
  switches. This is an initial vertical slice, not the completed Studio product.
- Leslie delegated the provider decision: use Steamworks UGC in the GUI process
  for catalog queries and subscription state/actions, while keeping local content
  available without Steam. The repository currently has no Steamworks client
  binding or packaged redistributable integration; this provider remains a major
  implementation task. Official `ISteamUGC` exposes query APIs and subscribed-item
  state/actions: https://partner.steamgames.com/doc/api/isteamugc?language=english
- Remaining material gaps: per-profile side-by-side installed versions, remote
  Steamworks UGC catalog and subscriptions, update flow, resolution previews
  from cache before install, templates and workspace creation, real operation
  progress/cancellation, Workshop stage/review/publish UI, and Linux/Windows
  smoke tests.
- Cached Packagepack installation now enforces declared semver requirements
  for enabled installed dependencies and compatible source/cache fallbacks; it
  reports when no compatible version exists. Side-by-side versions remain
  blocked by Shell's ID-keyed installed index.
- Studio content creation now opens the generated `Vapor.toml` directly in the
  workspace editor, keeping the source workspace selected and moving the user
  from creation into editing without a second operation.
- Studio installation records now query Shell's app-owned index for every
  exact node in the resolved local composition. Verification checks each exact
  graph node, and the Library exposes the engine/game roles, resolved content
  versions, install count, and repair state without Rust debug formatting.
  Disabled graph nodes are rejected as an incomplete install result.
- Shell now owns a verified installed-Packagepack graph and launch plan. Studio's
  installed Play path uses that service with profile arguments, and never
  changes Shell's global Packagepack selection. Engine launch paths are checked
  as regular in-root runtime binaries before execution.
- Studio install, verify, and repair now persist the graph Shell actually
  installed, including optional Game absence. Profiles retain their workspace
  association; source profiles install from that workspace, while Workshop and
  app-seeded profiles can install from Vapor's cache without depending on the
  currently open folder.
- Workshop cache records retain exact versions and accept exact `id@version`
  selectors. Studio Discover now exposes direct public PublishedFileId download
  through Shell's app-owned cache and only offers Packagepacks as profile roots.
  Steamworks UGC search/subscriptions remain unavailable because no client SDK
  binding or redistributable exists in the shipped stack.
- Steam Play mode now starts the saved profile, or resolves the app's declared
  `default` Packagepack seed when the profile has no root, then installs from
  cache/seed and launches through Shell. The shipped `App.vapor.toml` declares
  that default seed; the editable `App-Source.vapor.toml` does not.
- A focused repository search confirmed there is still no Steamworks client
  binding or `ISteamUGC` implementation in the shipped Vapor repositories.
  Keep the Studio Workshop screen recoverable/local-capable until the required
  client SDK binding and redistributable packaging are implemented.
- Replaced Studio's local-content discovery path with Vapor Shell's registered
  source catalog, and routed source validation and exact-profile Play through
  Shell. Removed Studio Core's unused competing local-composition resolver and
  its now-unneeded semver/TOML dependencies.
- Added Shell-owned role-specific source project creation. Studio Create now
  writes the matching role manifest, registers it in `Workspace.vapor.toml`,
  opens that manifest in the editor, and asks Shell for the compatible child
  roles. Game creation can select its required Engine dependency.
- Workspace changes now remain drafts until explicitly opened. Shell validates
  the candidate before Studio changes the active profile, so an invalid path
  does not discard the current composition. Home routes Play through the source
  or installed profile path according to that profile's workspace association.
- Exposed Shell's existing maintained basic-content initializer through Studio
  Core and added a guided Create flow that scaffolds a complete starter
  workspace, then discovers its content and files before switching the active
  profile. This gives a first-time user an Engine, Game, Packagepack, Cargo
  workspace, and editable project tree from the Studio UI.
- Closed a Shell launch gap for the existing pack vocabulary: installed
  Packagepacks now resolve Enginepack/Gamepack members to their effective
  Engine/Game before launch, while rejecting ambiguous or missing pack members.
- Added a focused Studio Core publishing module and bridge that re-stages the
  source and compares its package fingerprint against the reviewed artifact
  before delegating to Shell's create or update operation.
- Added a contextual Studio upload panel that derives create/update from the
  source PublishedFileId, names the Steam publisher account and target, and
  requires typing `PUBLISH <content-id>` before the upload action is enabled.
- Studio records the returned receipt and PublishedFileId in its operation
  outcome, then refreshes the source catalog. The UI upload path was not
  invoked during this work.
- Steamworks UGC is now connected through Vapor's `vapor_steam` crate. Studio
  Discover can search public Workshop items and list/modify the signed-in
  account's subscriptions. Results are owned data returned from a bounded
  callback pump; local catalog and exact-ID acquisition continue to work if
  Steam cannot initialize.
- Vapor-GUI stages the Steamworks redistributable beside its built executable
  and uses a relative Linux runpath. Managed `ldd` inspection now resolves
  `libsteam_api.so` from the GUI build output. The library still needs to be
  included in final Linux/Windows app depot staging; that shipping path and
  live Steam account behavior have not been exercised.
- Added target-specific `[workspace.runtime.libraries]` declarations and
  managed root promotion for runtime support files, then declared the GUI's
  Linux `libsteam_api.so` and Windows `steam_api64.dll`. Existing platform
  depot rules copy the full target binary directory, so promoted sidecars flow
  into the corresponding depot. Added safe portable filename validation and a
  unit test for traversal and cross-platform duplicate names.
- The managed metadata inspection resolved all seven app workspaces and the
  Steam depot graph, but reported no Cargo metadata because the installed Loo
  Cast app root lacks app-local Rust/Cargo and cross-build tools. `root build
  --host-only` correctly refused to fall back to another toolchain. No
  deployment, Steam launch, or publication was performed; the new build/test
  path remains unverified until a Vapor-managed development toolchain is
  available.
- Steamworks subscription records now include queried title, description,
  preview URL, Steam-installed/update/download flags, and byte progress. Studio
  shows those details, distinguishes Steam's download state from presence in
  Vapor's local catalog, and refreshes subscription state after subscribe or
  unsubscribe. The existing import action remains the path from Workshop ID to
  Vapor cache and composition flow. These changes are source-reviewed but not
  compiled or tested because the managed development toolchain is absent.
- Added `vapor sdk test`, dispatched through the same app-owned pinned Cargo
  toolchain and target cache. Vapor, Vapor-CLI, and Vapor-Studio managed checks
  pass; managed tests pass (Vapor: 26 tests; CLI and Studio currently define no
  tests). Vapor-GUI managed build also passes. No Studio launch, Steam call,
  deploy, publish, or account mutation was performed.
- Studio Discover now merges exact content identities from the active workspace,
  Vapor's app-owned cache, and its installed index. Entries retain each local
  location and installed enabled state; selecting a cached Packagepack can seed
  a profile without requiring an open source workspace. Catalog refresh runs at
  Studio startup and after Workshop acquisition or publication. A failed source
  scan no longer prevents cache/install entries from appearing, and the page
  copy now describes all three local sources. Remote Workshop search and
  account subscription actions are now wired through Steamworks; Workshop
  compatibility metadata and subscription-aware update/download state remain
  open.
- The Studio activity center now uses readable names for operation kinds and
  phases, retains completion/error summaries in recent history, and renders
  recent history after clearing the finished active operation. Progress bars
  appear only when an operation reports numeric totals. The current worker
  bridge still reports no incremental progress or cancellation; those remain
  future operation-contract work.
- Saving a `*.vapor.toml` manifest now refreshes authored-source validation and
  the merged local catalog, so edits to content identity or dependencies are
  reflected without leaving the workspace. Ordinary source-file saves do not
  trigger catalog work.
- Opening or creating a workspace now succeeds independently of cache/install
  catalog loading; if the latter fails, Studio opens the valid workspace and
  reports the local catalog error instead of stranding the authoring flow.
- Workspace now has a safe new-file action for authored scripts and other text
  content. It accepts only relative paths into existing in-workspace folders,
  uses exclusive creation so it cannot overwrite a file, selects the new file
  in the editor, and resets the draft path when switching workspace/profile.
- Recent activity now renders human-readable operation and phase labels, and
  completed success/error detail remains visible after the active operation is
  cleared. Progress is shown only when a worker supplies a total; the current
  worker bridge still has no incremental progress or cancellation.
- Shell now exposes a read-only dependency-first install preview over the same
  source, cache, installed, and first-party-seed records used by installation.
  It reports exact versions/provenance, missing or disabled dependencies,
  cycles, version collisions, installed conflicts, optional edges, and seed
  acquisitions that need a manifest before preview can complete. Workshop
  cache selectors are rejected if they resolve to the wrong content ID.
- Installation now enforces the local-authorship closure rule: once a selected
  node comes from workspace source, every required descendant must also resolve
  from that workspace; installed/cache fallbacks cannot silently cross that
  boundary. Studio Library shows the plan and blockers, routes seed acquisition
  into Discover, and gates install, source build/play, and repair on a ready
  preview. Content creation also refreshes the catalog after its manifest is
  generated.
- Tightened profile composition actions around plan provenance: an installed-only
  root is offered for verify/play but cannot masquerade as a fresh install or
  update source; source build/play is enabled only when the selected root comes
  from the associated workspace. Added root-kind validation, correct Workshop
  seed identity checks, and a real Update action through Shell's existing
  source/cache update path. Profile root assignment now resets graph/install
  state through one shared Studio method, catalog root changes are blocked while
  operations run, and profiles can be duplicated without manually rebuilding
  their root and launch settings. Discover now filters the merged local catalog
  by workspace/cache/installed/Workshop provenance and content type, with the
  shared search field and matching-result count feeding the same list. Leslie
  will compile and run tests.
- Reworked `content create` so each new role project is both a registered Vapor
  artifact and a real Cargo package. It generates role-aware Rust starters,
  declares Engine binaries and Game/mod libraries in the content manifest,
  adds the project to the root Cargo workspace while preserving TOML formatting,
  and validates Vapor and Cargo workspace discovery. Creation and registration
  roll back together on failure. Leslie will compile and run tests.
- Hardened Studio's workspace editor against stale writes: Save now compares the
  opened snapshot with the current file before writing, reports conflicts
  without overwriting external edits, and offers an explicit Reload from disk
  action. Replaced the flat file list with a collapsible directory tree. The
  inspector identifies Vapor/Cargo manifests, Rhai, Rust, and documentation,
  and shows TOML syntax diagnostics only when a manifest buffer changes. Added
  a workspace-wide content validation action through Vapor Shell. This is a
  first editor-awareness slice; semantic manifest forms, richer project nodes,
  Rhai tooling, and in-game workbench integration remain open.
- Corrected install-preview provenance for a cached Packagepack root that is
  already installed: the plan now marks the exact installed version as present,
  allowing Library to expose verify/play/update actions accurately.
- Routed Home's Play action from actual composition readiness: it now uses the
  installed launch path for installed profiles, source build/play only for a
  ready workspace-root plan, and otherwise opens Library to resolve the plan.
- Connected Workshop identities to profile composition actions across search
  results and subscriptions. Locally available Packagepacks can become the
  selected profile root; ambiguous local versions are surfaced for exact
  selection. Importing a missing dependency from a Library plan now starts the
  Vapor cache acquisition directly, and imported dependencies offer an explicit
  plan recheck. Successful root selection opens Compositions.
- Source-reviewed only. Leslie requested coding without another build or test
  pass after the 2026-10-15 Steam release cycle; no build, test, launch, deploy,
  or publication was run for this change.
- Strengthened the Studio catalog's exact-version identity merge: conflicting
  kind, Workshop ID, dependency metadata, or recorded cache/install fingerprint
  is retained as a visible conflict instead of silently accepting the first
  record. Conflicted identities are blocked from profile-root selection and
  Workshop staging/publication. Workspace, cache, and install paths identify
  the competing records; cache/install fingerprints are carried into catalog
  provenance.
- Catalog dependency comparisons now include relationship, version
  requirement, optionality, Workshop ID, and authored reason, so different
  constraints cannot be merged as if they were the same edge.
- Persisted per-node installed payload fingerprints in resolved profile graphs
  and derive a deterministic graph fingerprint from exact nodes and dependency
  edges. Legacy graph snapshots remain readable without fingerprints.
- Reused the profile's typed `ContentIdentity` as the catalog merge key and
  selector adapter, reducing the chance that catalog and profile identity
  rules drift apart.
- Exposed Vapor-CLI's recorded installation source on installed snapshots and
  retained it in each profile graph node. Graph fingerprints now cover node
  payload fingerprints, exact identities, dependency edges, and recorded source
  class; Library and Catalog show the recorded source class beside the installed
  path.
- Closed a Workshop review/upload time-of-check gap: the GUI now invokes a
  reviewed Shell operation that stages again, compares the exact payload
  fingerprint, copies that verified payload to operation-owned temporary
  storage, and uploads that same copy. The operation receipt records the
  uploaded payload fingerprint. Source edits or a concurrent overwrite between
  the displayed review and the final stage stop the upload. Documented the
  reviewed-upload guarantee in the Shell content guide.
- Source-reviewed only. Per the owner's current instruction, no build, test,
  launch, deploy, or publication was run; these changes remain uncompiled.

## Remaining

- Do not publish the extracted topology until the owner explicitly starts that
  authority boundary; the live Workshop items currently still reflect the last
  published terminal-proof-in-Loo-Cast state.
- Verify the published root app in Steam on a non-development Windows
  machine, including Studio, Play, Shell, and Installer entrypoints. BuildID
  `25970591` was accepted for `vapor-dev`; runtime behavior on Windows remains
  unverified.
- Implement and test the Windows/MSVC content build/runtime path before friend
  testing: Windows depot binary availability, app-local setup, target-specific
  runtime outputs, and ABI policy. The example dynamic loader now has a Windows
  implementation, but it has not yet been compiled or run under MSVC.
- Steamworks launch options match the four-option matrix. Linux uses
  `bin/x86_64-unknown-linux-gnu/vapor-entrypoint` with `studio`, `play`,
  `shell`, or `installer`; Windows uses
  `bin\x86_64-pc-windows-gnullvm\vapor-entrypoint.exe` with the same
  arguments. Studio and Play launch sibling Vapor Studio directly; Play adds
  `--play`. Shell and Installer use the terminal wrapper; Shell runs player
  setup before opening Vapor, while Installer opens directly. The target was
  uploaded in BuildID `25970591`; test runtime behavior on Windows.
- Run the owner-controlled installer install path from a fresh runtime-only depot
  install, open the Vapor and Loo-Cast sources only for development work,
  locally deploy root and selected first-party content, then iterate on the
  startup path.
- Continue the content model pass: packagepack/game/engine dependency semantics,
  legal modding declarations, content-shipped executable/library semantics,
  command-domain naming, contextual help, and an explicit review/removal flow
  for the `[[workspace.projects]]` and Cargo-member changes made by content
  creation.
- The new role starters are intentionally minimal. Their produced binary/library
  surfaces are declared, but actual Engine/Game/Mod runtime contracts and
  manifest-aware editing still need integration; confirm local build/run from
  Studio against the declared Shell runtime outputs.
- Continue the beginner development loop: turn the template into a playable
  reusable engine/game contract rather than only a buildable/publishable content
  skeleton, add import/adopt/repair operations for existing arbitrary source
  trees, and test the first Workshop create sequence with a non-owner account
  once an appropriate account is available.
- Continue the game/modding demo slice: packagepack-declared mod legality,
  installed optional mod content, and runtime discovery through Vapor-managed
  installed composition state.
- Keep final Steam create, publish, and delete authority-changing operations
  manual and interactive unless the owner explicitly scopes a scriptable
  authority path. Setup/teardown/toolchain DevOps is now script-owned.

## Smallest resume action

Continue source-only work on Workspace authoring and creator documentation:
close the remaining creation-to-local-run gaps, connect the game-side script
workbench to bounded shared operations, and improve discovery of supported
content APIs. Preserve the owner's request not to build or test; 2026-10-15
managed validation and Steam release remain the last verified state. Current
source-only edits are not compiler-verified.

## Release checkpoint

- On 2026-10-15, the managed seven-workspace `validate --project all` passed:
  fmt, check, tests, Clippy, and docs. Tests passed: Vapor 26, CLI 2, Examples
  7, Steam Entrypoint 5; the other workspaces currently define no tests.
- Fixed the empty workspace library map type, several Clippy failures in Studio
  and CLI, and app-local toolchain discovery/PATH ordering so Rust 1.97 is
  selected over older installed toolchains. Added coverage for selecting the
  newer toolchain. The compiler-selection correction was locally deployed.
- The `root publish --dry-run` and publish workflow both completed through the
  Vapor-managed CLI. The publish regenerated docs, promoted Linux and Windows
  installation binaries into the Steam app root, staged the split depots, and
  smoke-checked the payload. SteamCMD logged in as `ghf_vapor_build` using
  cached credentials and accepted app `2122620` BuildID `25970591` with
  `SetLive` set to
  `vapor-dev`; common, Linux, and Windows depots were uploaded.
- The Steam app is locally deployed at
  `/home/leslieghf/.local/share/Steam/steamapps/common/Loo Cast`. No game or
  Steam client was launched; Linux/Windows runtime behavior remains unverified
  until the owner launches it manually.
- The earlier Steam app build was BuildID `25939502`; replace that checkpoint
  with `25970591` for current testing.
- The unused `schema` key under Vapor-GUI's Cargo `[workspace.package]` was
  removed after Cargo warned that it was not recognized.

## Next implementation sequence (owner-approved)

1. **Shared operation boundary (implemented, source-only).** Vapor Core owns process-local identity, typed request metadata, phases, progress, and terminal status. Studio streams lifecycle events into its operation center; Vapor Shell wraps every top-level command family and direct Client deployment, with script steps recorded individually. CLI phases are coarse/diagnostic-only; cancellation is not connected to a host-controlled handle.
2. **Shell-owned Packagepack graph editing (implemented, source-only).** Typed load/save operations edit existing authored Packagepacks. `toml_edit` retains untouched document content; the replaced role/dependency tables are authored from the proposed graph. Shell validates source identity, versions, role cardinality, cycles, and local descendant closure before a recoverable same-directory replacement.
3. **Studio Compose (implemented, source-only).** A dedicated screen loads a local Packagepack, adds/removes workspace edges, saves through a managed operation, and hands a saved graph to the active profile for re-resolution. Library links into the editor; cache and installed copies stay read-only.
4. **Connect authoring to the full content lifecycle.** Bring graph provenance, source/cache/install status, compatibility, Workshop acquisition/update, and publish review into one legible composition view. Do not conflate Steam subscriptions with Vapor installation.
5. **Expand the workspace authoring path.** Valid Workspace registration and explicit adoption of an existing content project into the workspace registry are implemented without activating the workspace or moving project files. Add project repair/import and semantic manifest/script affordances atop the same Shell-owned contracts; keep arbitrary text editing as the escape hatch.
6. **Close operational integration.** Keep Steam and deployment mutations behind reviewed operations, address host-controlled cancellation, and update docs/issues after source review.

Implementation is source-only until Leslie requests managed verification again. Commit/push only files proven to belong to this workstream; preserve unrelated dirty files.

## 2026-10-16 CLI operation lifecycle integration

- Replaced the content-only lifecycle wrapper with one Shell command dispatcher wrapper. Root, source, workspace, docs, IDE, content, provider, SDK, diagnostics, runtime launch, and direct local Client deployment now emit the shared queued/running/phase/final lifecycle through Vapor Core. A Vapor script records its child commands as individual operations.
- Centralized the CLI adapter in `run_cli_operation`; the sink records lifecycle transitions in the existing diagnostics event stream. Existing command handlers retain domain authority and synchronous result handling.
- No build, test, formatting, launch, deployment, Steam action, or commit was run. This remains source-reviewed only. Cancellation is still an explicit operation-contract gap, not claimed as supported.

## 2026-10-16 Packagepack Compose implementation

- Added a Shell-owned `SourceComposition` snapshot and save API. Save uses exact-source stale-write detection, a per-manifest process lock, TOML edit preservation for unrelated fields, a recoverable same-directory replacement, live content-kind/version validation, the Packagepack Engine/Game cardinality rules, cycle rejection, and the owner-approved local-descendant closure rule.
- Exposed the API through Studio Core, added a dedicated Compose navigation screen, connected it to background operations and the operation center, and added Library-to-Compose plus Compose-to-profile-root handoffs. A changed graph cannot be assigned to a profile until it has been saved; a saved selected root triggers catalog refresh and dependency re-resolution.
- Added the user-facing flow summary to Vapor Studio's README.
- Source-reviewed only. Per Leslie's current instruction, no build, test, formatter, launch, deployment, or Steam action was run. This is not compiler-verified. No commit/push has been made for this tranche yet.

## 2026-10-16 source-only continuation

- Added Vapor Core's shared operation request, process-local identity, lifecycle
  event, phase, progress, and stateless execution service. Hosts retain control
  over scheduling and presentation; operation bodies report semantic phase and
  progress through an owned context. Studio now reuses Core's identity/phase/
  status/progress types directly and layers only profile and typed action data
  onto its presentation event.
- Routed every Studio background task through that service and connected its
  lifecycle stream to the existing operation center/history. Workspace
  development operations now report resolve/build phases and per-project
  progress. Vapor CLI's workspace fmt/check/test/build/clean and content
  lifecycle commands use the same runner. Other CLI command families still
  need migration to the shared operation contract.
- No build, test, formatting, deployment, launch, Steam action, or commit was
  run for this continuation. Source is not compiler-verified; the last verified
  checkpoint remains 2026-10-15.


## 2026-10-16 Workspace adoption continuation

- Added `remember_source` in Vapor Core: it canonicalizes and validates a Vapor Workspace, rejects roots overlapping the installation, registers it in the known-source list, and leaves the active source and superworkspace untouched.
- Studio now validates `Workspace.vapor.toml` when opening a folder, so profile association cannot silently treat an arbitrary directory as a Vapor Workspace. Added a Studio Workspace action to register the currently open profile workspace with this Vapor installation. The operation runs in the background and reports through the shared operation center; the profile can use a workspace without globally registering or activating it.
- Source-reviewed only. `git diff --check` passed. No compiler, tests, formatter, deployment, launch, or Steam action was run. Workspace/Compose integration, source-root handoff, optional-descendant validation, explicit adoption, and candidate discovery have been committed and pushed. The per-project runtime-output and validation slice is committed/pushed in Vapor-Studio `f4b6799` and parent workspace `9915b29`. The worktree is source-only and clean apart from the unrelated pre-existing Vapor-GUI change.

### Current implementation checkpoint

The source-only content-authoring checkpoint is pushed across Vapor-CLI (`768b663`) and Vapor-Studio (`424121a`); the parent pointer checkpoint is pending. Vapor-GUI has a pre-existing user change in `build.rs`; preserve it. Current editor slices remain uncompiled and untested by owner request.


## 2026-10-16 explicit content-project adoption

- Added a Vapor Shell operation to adopt one caller-selected relative folder into a valid Vapor Workspace's explicit content registry. It rejects traversal, outside-root resolution, symlinked target/manifest files, unrecognized roles, duplicate paths/IDs, and invalid workspace graph results. It preserves unrelated TOML through `toml_edit`, uses a workspace edit lock, writes a recoverable same-directory replacement, and validates/reverts if discovery fails. Project files remain where they are; Cargo membership is unchanged.
- Exposed adoption through Vapor Studio Core and the Workspace page as one typed background operation. Success refreshes source content and the catalog; the selected project becomes immediately discoverable, and Packagepacks can continue into Compose/profile selection. This initial adoption path is committed and pushed.
- Static diff checks only. No build, tests, formatter, deploy, launch, or Steam action was run.

## 2026-10-16 candidate discovery continuation

- Replaced the required path knowledge with a non-mutating bounded scan for role-specific manifests in non-hidden workspace folders. The scan returns only valid unregistered content candidates, with diagnostics for invalid manifests, duplicate identities, inaccessible paths, and traversal limits. It does not register discovered folders. Studio presents candidates and requires an explicit Adopt action; Shell validates again before writing.
- This scan-and-select layer is source-only and committed/pushed in Vapor-CLI `b0af369`, Vapor-Studio `4ef0d3d`, and parent workspace `d594a1a`. No build, tests, formatter, deploy, launch, or Steam action was run.


## 2026-10-16 per-project authoring actions

- Studio's explicit project overview now surfaces declared runtime binary/library outputs and offers per-project Vapor validation, while preserving the whole-workspace validation action. This gives the creator a direct metadata-to-runtime check from the project row.
- Source-only; no compiler, tests, formatter, deploy, launch, or Steam action was run. Committed/pushed in Vapor-Studio `f4b6799` and parent workspace `9915b29`.

## 2026-10-16 structured runtime-output editing

- Added a typed Vapor Shell snapshot and save API for content `binaries` and
  `libraries`. Composition, Workshop metadata, and runtime-output edits now
  share one exact-source guard, per-manifest lock, atomic replacement, and
  validated read-back/recovery path. All three keep unrelated TOML fields.
- Studio exposes a contextual runtime-output editor in each discovered source
  project. It validates file names in Shell, protects dirty drafts across
  workspace/profile navigation, refreshes the source catalog, and invalidates
  every profile pinned to the exact edited content version. Affected profiles
  must be resolved and installed again before Play.
- Static review and `git diff --check` only. No build, test, formatter,
  deployment, launch, or Steam action was run; this pass is not
  compiler-verified.


## 2026-10-16 reversible content-project registration

- Added explicit unregistration from the Workspace registry through the same
  locked, stale-source-checked, recoverable Shell transaction used by adoption.
  It validates the saved Workspace and refreshed discovery before accepting the
  edit; a validation error restores the original manifest. It leaves content
  files and Cargo workspace membership untouched, so Find projects / Adopt can
  re-register it.
- Studio exposes Unregister on each registered project with a clear confirmation,
  performs the operation in the background, refreshes discovery, and invalidates
  profiles pinned to the exact removed content version.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `768b663` and Vapor-Studio
  `424121a` are pushed; the parent pointer checkpoint is being recorded.


## 2026-10-16 serialized content registration

- Closed a registry lost-update path: content creation now acquires the same
  Workspace project edit lock as adoption and unregistration, and verifies its
  Workspace and Cargo manifest snapshots before writing. This keeps overlapping
  Vapor-managed registry edits from silently overwriting one another.
- Source-reviewed and `git diff --check` passed. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `f86c70a` is pushed; the
  parent pointer update is recorded in this checkpoint.


## 2026-10-16 Workspace editor save reliability

- Replaced direct in-place Workspace file writes with a serialized same-directory
  save: preserve permissions, write and sync a temporary, recheck the opened
  snapshot, preserve the original during replacement, and restore it if the new
  file cannot be installed. This applies to authored Rhai, Rust, TOML, and other
  text files in the existing editor.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `08ff3e7` is pushed; the
  parent pointer update is recorded in this checkpoint.


## 2026-10-16 Compose lifecycle handoff

- Added a source-graph availability preview to Compose for saved Packagepacks.
  It resolves against the active Workspace, Vapor cache, and installed content,
  then shows source provenance, blockers, warnings, and direct Workshop download
  actions. Unsaved graph drafts cannot show a stale plan.
- Extracted the same plan presentation for both Compose and Library, so both
  entrypoints render identical source and acquisition semantics.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `6c68a5e` is pushed; the
  parent pointer update is recorded in this checkpoint.


## 2026-10-16 nested workspace file creation

- Workspace file creation can now build missing parent folders inside the
  selected Workspace, allowing a creator to start a Rhai or Rust source tree
  without pre-creating every directory. Path traversal and symlinked folders
  are rejected; existing files are never overwritten, and failed creation
  removes newly created empty folders where possible.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `876e1fe` is pushed; the
  parent pointer update is recorded in this checkpoint.


## 2026-10-16 Compose-to-publish handoff

- Added a staged Workshop-review handoff for any saved source content project
  directly from Compose. It enters the existing review surface and keeps the
  current explicit confirmation gate before publication.
- Static review and `git diff --check` only. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `6d8ffb0` is pushed; the parent
  pointer update is recorded in this checkpoint.


## 2026-10-16 Compose publishing handoff

- Compose now lets the creator stage any saved source project for Workshop
  review without leaving the authoring flow. It selects that project in the
  existing review surface; upload remains a distinct, explicitly confirmed
  operation.
- Static review and `git diff --check` only. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `6d8ffb0` is pushed; the parent
  pointer update is recorded in this checkpoint.


## 2026-10-16 Workspace surface separation

- Split the Workspace screen into explicit Overview and Files views. The
  Overview retains workspace selection, content registry actions, validation,
  and managed development operations. The Files view owns file creation, the
  project tree, source editor, and inspector. Opening or creating a file moves
  directly into Files; opening another Workspace returns to Overview.
- Static review and `git diff --check` only. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `14b4301` is pushed; the parent
  pointer update is recorded in this checkpoint.


## 2026-10-16 Workspace file-access boundary

- Moved bounded Workspace file discovery, containment checks, UTF-8 reads,
  nested-file creation, and guarded replacement saves out of the Studio app
  coordinator into a dedicated file-access module. Overview and Files now use
  the same explicit boundary. The 4 MiB editor limit reports that large files
  cannot be opened instead of promising a preview that does not exist yet.
- Static review and `git diff --check` only. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `87833a6` is pushed; the parent
  pointer update is recorded in this checkpoint.


## 2026-10-16 stable editor lock correction

- A source review caught that the extracted editor lock path included a unique
  timestamp, preventing concurrent writers from contending. Lock paths now use a
  stable per-file sibling name; only temporary and recovery files are unique.
- Static review and `git diff --check` only. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `04929d7` is pushed; the parent
  pointer update is recorded in this checkpoint.


## 2026-10-16 recoverable content creation

- Content creation now uses the guarded same-directory replacement path for
  Cargo membership and Workspace registration instead of truncating manifests
  in place. It validates after each step, rolls back the Workspace registration
  and Cargo membership on reported failure when snapshots still match, and
  leaves concurrently changed files untouched with explicit recovery details.
  If the process stops between the two manifest updates, the content folder
  remains discoverable for explicit adoption.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `fd9055a` is pushed; the
  parent pointer update is recorded in this checkpoint.


## 2026-10-16 Workshop review integrity

- Publishing now compares the complete staged review (identity, version,
  publication metadata, dependencies, target, payload fingerprint, file count,
  size, and file list) with the fresh source stage immediately before upload.
  A changed title or visibility can no longer be silently published under a
  preview that showed different values. The reviewed payload fingerprint is
  still passed to Vapor Shell's final upload guard.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `b305218` is pushed;
  this checkpoint records the matching submodule pointer.


## 2026-10-16 invalidate stale Workshop reviews

- Studio now drops the staged review and typed publish confirmation when source
  files, manifests, composition links, runtime outputs, or new files change. A
  failed re-stage also clears the previous preview, so the screen cannot imply
  that an older package is the current review.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `12ce703` is pushed;
  this checkpoint records the matching submodule pointer.


## 2026-10-16 role-valid content creation

- Vapor Shell now exposes its exclusive dependency-choice groups to Studio.
  The creator picker replaces a conflicting Engine/Enginepack or Game/Gamepack
  choice in the same role slot, while ordinary supported dependencies remain
  multi-selectable. Changing the content kind or workspace clears stale choices
  so invisible selections cannot leak into a later creation request.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `4042e80` and Vapor-Studio
  `cacdf17` are pushed; this checkpoint records both submodule pointers.


## 2026-10-16 required-role creation readiness

- The create form now gets required dependency slots from Vapor Shell and keeps
  Create disabled until the required role is selected and a Workspace is open.
  The role picker uses Shell's exclusive-slot API to replace conflicting choices;
  changing content kind or workspace clears stale selections. General
  dependencies remain multi-selectable.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `d59914a` and Vapor-Studio
  `81261af` are pushed; this checkpoint records both submodule pointers.


## 2026-10-16 clear confirmations with review invalidation

- Profile/workspace changes, catalog refreshes, source edits, staging failures,
  and completed publication now use one Workshop-review invalidation path. It
  clears both the preview and any typed publish confirmation, so a refresh or
  context switch cannot carry prior approval into a later staged review.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `74f8c06` is pushed;
  this checkpoint records the matching submodule pointer.


## 2026-10-16 shared role-cardinality contract

- Consolidated bounded relationship slots in Vapor Shell and reused those same
  slots in its source-graph cardinality validator. Studio's creation picker and
  readiness gate now consume that Shell contract, avoiding separate UI rules
  for the Engine/Enginepack and Game/Gamepack alternatives.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `9fa137c` is pushed; this
  checkpoint records the matching submodule pointer.


## 2026-10-16 metadata-only composition projects

- Packagepack, Enginepack, Gamepack, and Modpack creation now produces only the
  role manifest and registers it in the Vapor workspace. It no longer creates
  an empty Cargo crate, `src/` placeholder, README boilerplate, or Cargo member.
  Engine, Game, and mod roles retain their existing Rust scaffolds, runtime
  output declarations, and recoverable Cargo registration transaction.
- Updated the Studio creation form and Shell manifest/topology docs to show the
  role-specific source footprint. Static review and `git diff --check` only;
  no build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-CLI `59f8cfc` and Vapor-Studio `3e9d1e4` are pushed; this checkpoint
  records both submodule pointers.


## 2026-10-16 metadata-only starter composition

- The complete Engine/Game/Packagepack starter now follows the same role split:
  Engine and Game remain Rust Cargo projects, while Packagepack is a registered
  manifest-only project. Removed its empty crate, Cargo lock entry, and generated
  Rust composition stub; the starter README states this source structure.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `3f23d6b` is pushed; this
  checkpoint records the matching submodule pointer.


## 2026-10-16 complete Workshop review equality

- Replaced a hand-maintained field list with equality on the whole review
  snapshot. The stage output path, current files, metadata, target, and any
  future preview fields now all participate in the pre-publish comparison.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `b1c32fe` is pushed;
  this checkpoint records the matching submodule pointer.


## 2026-10-16 managed starter-workspace contract

- The generated starter now includes Vapor Core's top-level code-project entries
  and the active managed toolchain pin, so it satisfies the same workspace
  contract used by managed development operations. The nested Vapor content
  registry remains separate and still includes the metadata-only Packagepack.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-CLI `907f215` is pushed; this
  checkpoint records the matching submodule pointer.


## 2026-10-16 clarify project registries

- Documented the two Workspace lists explicitly: top-level `[[project]]` names
  Cargo roots for managed development/run configuration; nested
  `[workspace].projects` registers Vapor content. This makes clear that a code
  content member can live inside a Cargo root, while metadata-only packs exist
  only in the Vapor content registry.
- Source-only documentation update; `git diff --check` only. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-CLI `f03d3ae` is
  pushed; this checkpoint records the matching submodule pointer.


## 2026-10-16 starter schema documentation

- Updated the `source init basic-content` guide to describe the managed toolchain
  pin, Cargo-root project list, separate content registry, and metadata-only
  Packagepack produced by the starter.
- Source-only documentation update; `git diff --check` only. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-CLI `2551800` is
  pushed; this checkpoint records the matching submodule pointer.


## 2026-10-16 creation readiness guidance

- The disabled Create action now tells creators to open a Workspace before
  choosing a required Engine/Game role when both prerequisites are absent.
- Source-only UI polish; `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `5dd1d54` is pushed;
  this checkpoint records the matching submodule pointer.

## 2026-10-16 explicit workspace ownership

- The active profile association now decides whether a Workspace is open.
  Current-directory fallback is only a suggested path; explicit/remembered
  paths and a current directory containing `Workspace.vapor.toml` pass through
  the validated open flow.
- Workspace-only creation, editing, validation, staging, and development
  operations now guard against operating on an arbitrary launch directory.
  Switching profiles or detaching to cache-only content clears source-editor
  state; reopening the same workspace refreshes it without dropping its profile
  composition.
- Static review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `6a8770b` is pushed;
  Vapor-Client checkpoint `4bc39db` records the matching submodule pointer.

## 2026-10-16 Workshop search result context

- Workshop result state now carries the successful search query. Starting a
  new query/page clears the previous list and count; successful empty results
  are identified as empty for that exact query, while failures cannot leave an
  older result list looking current.
- Static review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `d06dde7` is pushed;
  Vapor-Client checkpoint `d696629` records the matching submodule pointer.

## 2026-10-16 installation readiness lifetime

- Installation readiness is now a live observation rather than saved profile
  configuration. Reopening Studio deserializes it as `Unknown`, forcing the UI
  to inspect the app-owned installation again before presenting it as ready.
- Static source review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `92d6f9f` is pushed;
  Vapor-Client checkpoint `7fee4a9` records the matching submodule pointer.

## 2026-10-16 Workshop operation identity

- Workshop search, subscription refresh, and subscribe/unsubscribe now have
  distinct operation identities and operation-center labels instead of being
  reported as generic catalog refreshes.
- Static review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `30c3dbb` is pushed;
  Vapor-Client checkpoint `b4fa482` records the matching submodule pointer.

## 2026-10-16 profile lifecycle

- Profiles can now be renamed with case-insensitive uniqueness checks or
  removed through an inline confirmation. The last profile cannot be removed;
  workspace files and installed content are explicitly preserved.
- Static review and `git diff --check` only. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `775d620` is pushed; Vapor-Client checkpoint `92598ba` records
  the matching submodule pointer.


## 2026-10-16 Studio operation orchestration boundary

- Moved the background operation result vocabulary, polling/result application,
  operation progress observation, and workspace/catalog discovery helpers out
  of the central Studio app module into `vapor_studio_ui::operations`. The UI
  app now retains screen/profile state and delegates operation lifecycle work.
- Static review and `git diff --check` only. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `d7290ca` is pushed; the parent
  checkpoint records the matching submodule pointer.

## 2026-10-16 Studio profile lifecycle boundary

- Moved profile switching, root assignment from catalog/Workshop, profile
  creation, duplication, rename/removal, and the associated editor/catalog state
  reset into `vapor_studio_ui::profiles`. The screen code retains its existing
  calls through `StudioApp`.
- Static source review, a source-only visibility/method-presence check, and
  `git diff --check` only. No build, tests, formatter, deploy, launch, or Steam
  action was run. Vapor-Studio `0dbcef2` is pushed; the parent checkpoint
  records the matching submodule pointer.

## 2026-10-16 Studio workspace lifecycle boundary

- Consolidated workspace discovery/open/activation, saved-state guards,
  source validation, workspace file editing, starter and content creation,
  project adoption/unregistration, and registered-source management in
  `vapor_studio_ui::workspace_ops`. Existing UI pages retain their calls through
  `StudioApp`; the main app module is now roughly half its previous size.
- Static source review, moved-method visibility checks, and `git diff --check`
  only. No build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `76a2b23` is pushed; the parent checkpoint records the matching
  submodule pointer.

## 2026-10-16 Studio source-authoring boundary

- Moved source composition load/save, Workshop metadata editing, runtime output
  declarations, source-change profile invalidation, and applying a source
  Packagepack to a profile into `vapor_studio_ui::source_authoring`. The central
  UI module is now under 900 lines.
- Static source review, moved-method visibility checks, and `git diff --check`
  only. No build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `be397a7` is pushed; the parent checkpoint records the matching
  submodule pointer.

## 2026-10-16 Studio composition and Workshop feature boundaries

- Separated composition preview/install/update/verify/repair/run behavior into
  `composition_lifecycle` and Workshop search/subscriptions/stage/publish/acquire
  plus review invalidation into `workshop`. The central `app.rs` now owns UI
  state, navigation, rendering, and persistence instead of content workflows.
- Static source review, moved-method visibility checks, and `git diff --check`
  only. No build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `460c938` is pushed; the parent checkpoint records the matching
  submodule pointer.

## 2026-10-16 Studio operation result reducer boundary

- Split UI-thread operation completion handling from background scheduling.
  `operations` now owns result production, lifecycle polling, progress events,
  and worker threads; `operation_results` owns the UI-state reducer. The reducer
  was compared to its pre-extraction Git body and is unchanged aside from
  indentation; all 31 result variants remain represented.
- Static source comparison, result-variant presence check, and `git diff --check`
  only. No build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `149be67` is pushed; the parent checkpoint records the matching
  submodule pointer.

## 2026-10-16 Studio shell/helper boundary correction

- Static cross-module review found the workspace extraction had left the
  `eframe::App` trait implementation empty and put shared catalog selectors in
  the workspace module. Moved the unchanged UI/save methods together into
  `app_shell` and restored shared selectors/filter helpers to the app boundary.
  The immediate source bodies compare exactly; one trait implementation now
  owns `ui` and `save`.
- Source-only exact-body comparison, helper-scope checks, and `git diff --check`
  only. No build or tests were run. Vapor-Studio `a92b481` is pushed; this
  parent checkpoint records the corrective pointer.

## 2026-10-16 Studio Workshop state ownership

- Grouped Workshop preview/publish confirmation, pending acquisition, search
  result/page state, and subscriptions into `WorkshopState`, replacing thirteen
  unrelated fields on the root app object. The existing first-page default and
  all current UI/action reads remain represented.
- Static field-reference/initial-default checks and `git diff --check` only. No
  build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `06856df` is pushed; the parent checkpoint records the pointer.

## 2026-10-16 Studio workspace state ownership

- Grouped workspace root/draft/fallback, editor buffers and file selection,
  registered sources, unregistered-project review, and starter-workspace inputs
  into `WorkspaceState`. All existing page/actions now access that state through
  the workspace owner; starter defaults and initial path selection are retained.
- Static nested-field/initializer checks and `git diff --check` only. No build,
  tests, formatter, deploy, launch, or Steam action was run. Vapor-Studio
  `15b3f07` is pushed; the parent checkpoint records the matching pointer.

## 2026-10-16 Studio profile state ownership

- Grouped the profile collection, active profile, recovery text/backup, and
  profile add/rename/removal form state into `ProfileState`. Every page,
  operation handler, and persistence access now addresses the profile owner.
- Static nested-field/initializer checks and `git diff --check` only. No build,
  tests, formatter, deploy, launch, or Steam action was run. Vapor-Studio
  `1db41fd` is pushed; the parent checkpoint records the matching pointer.

## 2026-10-16 Studio state ownership integration

- Grouped local source/catalog data, query/filter/selection, source-authoring
  drafts, and operation execution/progress state under explicit owners. Static
  cross-module checks found every nested field declared and used, and all app
  root fields initialized. Corrected stale profile-container lookups left in
  the operation, composition, and workspace modules after the earlier profile
  extraction.
- `git diff --check` and source-only structural checks passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `27f3afc` is
  pushed; this checkpoint records the matching submodule pointer.

## 2026-10-16 Studio lifecycle and panel boundaries

- Added state-owned context reset operations for catalog, authoring, profile
  composition, and workspace file state. Split the Workspace file-browser and
  editor panel into its own UI module, and moved content/starter creation out
  of workspace lifecycle operations into `content_creation`.
- Static cross-module field and stale-reference checks plus `git diff --check`
  passed. No build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `176ec57` is pushed; this checkpoint records the matching
  submodule pointer.

## 2026-10-16 Studio workspace editor boundary

- Moved guarded file listing, opening, saving, reloading, creation, and TOML
  diagnostics into `workspace_editor`. Corrected a stale lookup of
  `selected_workspace_file` to use the current `WorkspaceState.selected_file`.
- Static stale-reference review and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `23785c3` is
  pushed; this checkpoint records the matching submodule pointer.

## 2026-10-16 Studio catalog and composition ownership

- Moved catalog state/selectors/filtering into `catalog.rs`. Centralized the
  installed-composition success, verification, and failure transitions under
  the composition lifecycle owner instead of repeating profile mutations in
  the generic result reducer.
- Static source review and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `3c888e7`
  and `7506ee0` are pushed; this checkpoint records their current submodule
  pointer.

## 2026-10-16 Studio operation-result ownership

- Kept `OperationResult` exhaustive in a small router and moved successful
  result application beside the owning catalog, workspace, editor, content
  creation, Workshop, authoring, and composition workflows. Routed manifest
  synchronization through the workspace editor owner. Removed one duplicate
  workspace transition reset already performed by `activate_workspace`.
- Static route inventory confirmed all result variants remain represented,
  all declared module files and handlers resolve, and the router no longer
  mutates domain state directly. `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `a0d659e`
  is pushed; this checkpoint records the current submodule pointer.

## 2026-10-16 Studio workspace draft save gates

- Found that the generic “workspace must be clean” guard also rejected the
  editor Save/Reload actions and the structured Workshop/runtime-output Save
  actions while their own drafts were dirty. Added separate guards that allow
  each action to consume its own draft while blocking conflicting edits in the
  other editor. Unified the pending-edit predicate across source-dependent
  actions and their UI enablement.
- Static guard-routing checks and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `de52d03`
  is pushed; this checkpoint records the current submodule pointer.

## 2026-10-16 Studio workspace ownership boundaries

- Split the workspace overview into registry, discovered content, authoring,
  unregistered-project, and operation panels. Moved discovery, validation,
  adoption, registration, and their result transitions into a workspace content
  owner module; kept the page as a thin composition of panels.
- Static comparisons confirmed the panel split retained every original UI
  string and action call; `git diff --check` passed. No build, tests, formatter,
  deploy, launch, or Steam action was run. Vapor-Studio `5a2bd3e` is pushed;
  this checkpoint records the current submodule pointer.

## 2026-10-16 Studio async operation protocol

- Separated the typed operation result protocol and its failure/refresh policy
  from background execution. Moved catalog refresh/discovery under the catalog
  owner. Profile switching now requests a transition without temporarily
  mutating the active index and reports the running-operation guard.
- Static inventories confirmed all 31 operation results remain in the protocol
  and reducer, and content/subscription refresh triggers are preserved.
  `git diff --check` passed. No build, tests, formatter, deploy, launch, or
  Steam action was run. Vapor-Studio `3ef3191` is pushed; this checkpoint records
  the current submodule pointer.

## 2026-10-16 Studio composition screen boundaries

- Split the Compositions screen into profile selection, profile management,
  composition preview/graph, launch arguments, and installation actions/status.
  The screen now coordinates these named panels; identity formatting reuses
  the catalog's selector instead of a duplicate helper.
- Static inventories confirmed every prior UI string and action call was
  retained; all touched Rust delimiters balance and `git diff --check` passed.
  No build, tests, formatter, deploy, launch, or Steam action was run.
  Vapor-Studio `b34e5f1` is pushed; this checkpoint records the current pointer.

## 2026-10-16 Studio workspace-open contract

- Replaced the positional four-field workspace-open result tuple with a named
  `WorkspaceOpenSnapshot` carrying root, source content, local catalog, and
  workspace files. The workspace activation owner now consumes that typed
  snapshot directly.
- Static delimiter/reference checks and `git diff --check` passed. No build,
  tests, formatter, deploy, launch, or Steam action was run. Vapor-Studio
  `7d1ef98` is pushed; this checkpoint records the current pointer.

## 2026-10-16 Studio Publish surface

- Added a dedicated Publish screen. Workshop staging remains in Discover, while
  exact package review, dependencies, account, change note, and typed upload
  confirmation now live in one publication flow. Discover links a staged item
  to that review. Moved the root package preview out of Diagnostics and labeled
  it preview-only; the SDK cannot yet bind an upload to the exact reviewed
  staged bytes, so no unsafe root publish button was added.
- Static delimiter/routing checks and `git diff --check` passed. No build,
  tests, formatter, deploy, launch, or Steam action was run. Vapor-Studio
  `5ffb2a8` is pushed; this checkpoint records the current pointer.

## 2026-10-16 Studio Workshop batch import

- Connected Workshop multi-selection in Discover to Vapor's existing batch
  cache acquisition path. Selection survives page/search changes, duplicate
  IDs are collapsed at the domain boundary, and each imported artifact gets
  its own follow-up action for profile roots or composition preview.
- Added one-session Steamworks batch subscribe/unsubscribe with bounded batches,
  per-item callback aggregation, and subscription-state refresh after both
  success and failure. Importing into Vapor's cache remains an independent
  action from subscribing in Steam.
- Static API/route checks and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run.

## 2026-10-16 Composition acquisition loop

- Install-plan previews now collect all missing Workshop IDs and import them
  through one acquisition. Library profiles re-resolve after catalog refresh;
  source compositions retain their exact selector and workspace for their own
  re-preview. Failed/partial acquisitions also refresh the cache catalog.
- Static route checks and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `bab22c6`
  is pushed; this checkpoint records the current pointer.

## 2026-10-16 Subscription-to-cache workflow

- Subscription rows now join the same cross-page selection queue as search
  results, with batch cache import, local-copy refresh, and profile-root actions.
  Studio distinguishes subscriptions not yet loaded from a confirmed empty list.
- Static UI-route checks and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `a21201a`
  is pushed; this checkpoint records the current pointer.

## 2026-10-16 Discover and Workshop page ownership

- Split the Workshop browser out of the general local Discover page. Workshop
  search, subscriptions, cross-result selection, import, and profile handoff
  now share a named page owner; Discover composes it beside the local catalog.
- Static comparisons retained every prior UI string and self-action, and
  `git diff --check` passed. No build, tests, formatter, deploy, launch, or
  Steam action was run. Vapor-Studio `40e110e` is pushed; this checkpoint
  records the current pointer.

## 2026-10-16 Workshop browser panel boundaries

- Split the Workshop browser into subscription management, search/selection,
  and acquisition panels under a small page coordinator. This keeps cache
  import, Steam subscription, and search state in separate render boundaries.
- Static string/action comparisons and `git diff --check` passed. No build,
  tests, formatter, deploy, launch, or Steam action was run. Vapor-Studio
  `9ee075a` is pushed; this checkpoint records the current pointer.

## 2026-10-16 Local catalog panels

- Separated the local catalog list and selection actions from the selected
  content detail panel. Discover now composes both beside the Workshop browser.
- Static string/action comparisons and `git diff --check` passed. No build,
  tests, formatter, deploy, launch, or Steam action was run. Vapor-Studio
  `5329082` is pushed; this checkpoint records the current pointer.

## 2026-10-16 Live Workshop progress

- Active Steam downloads now refresh their UGC state in the background. Steam
  subscription reads back off from 2 seconds to a 30-second cap on failures;
  inactive subscriptions stop polling, and the frame loop only schedules wakeups.
- Static lifecycle checks and `git diff --check` passed. No build, tests,
  formatter, deploy, launch, or Steam action was run. Vapor-Studio `3f27cda`
  is pushed; this checkpoint records the current pointer.

## 2026-10-16 Multi-file Workspace editor

- Replaced the single editor buffer with a bounded set of 16 open file tabs.
  Each document keeps an independent baseline/current buffer, so switching tabs
  preserves drafts; builds, validation, and workspace changes remain blocked
  while any tab is dirty. Clean tabs close independently.
- Studio storage now checkpoints open buffers with their workspace root and
  restores them only when reopening that same canonical workspace. Draft writes
  continue using the existing on-disk conflict check and atomic replace path.
- `git diff --check`, delimiter-balance checks, and a stale-single-buffer-field
  scan passed. Per owner direction, no build, tests, formatter, deploy, launch,
  or Steam action was run. Vapor-Studio `8e8fa85` is pushed; parent pointer
  checkpoint is pending.

## 2026-10-16 Multi-tab save operation

- Added `Save all` for dirty Workspace tabs as one reported operation. It uses
  the existing conflict-detecting, atomic per-file writer and preserves dirty
  buffers for any files that could not be saved; successful files become clean
  independently, so partial failure is explicit and recoverable.
- Static diff and delimiter checks passed. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `f741beb` is pushed; parent
  pointer checkpoint is pending.

## 2026-10-16 Workspace quick file search

- Added case-insensitive path filtering to the Workspace file tree, with Enter
  opening a unique match. It searches only the discovered visible workspace
  files and uses the same bounded tab and guarded reader as tree selection.
- Static diff and delimiter checks passed. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `61959b8` is pushed; parent
  pointer checkpoint is pending.

## 2026-10-16 Per-document editor identity

- Keyed the editor widget to its selected workspace-relative file path. This
  isolates egui cursor, scroll, and undo state per tab instead of sharing one
  widget identity across unrelated files.
- Static diff and delimiter checks passed. No build, tests, formatter, deploy,
  launch, or Steam action was run. Vapor-Studio `c711925` is pushed; parent
  pointer checkpoint is pending.

## 2026-10-17 Studio product-flow and shell UX pass

- Reframed primary navigation around Play, Content, Authoring, and Share; moved
  profile switching into play-related screens and separated on-device discovery
  from Steam Workshop browsing. Local content search now appears only where it
  actually filters.
- Rebuilt Home as a resume dashboard for the active play profile and workspace,
  with install health, guarded next actions, and recent operation outcomes that
  persist across restarts. Operation progress/history now has its own fixed
  bottom panel; long non-editor pages scroll independently, while the file
  editor retains its split-pane workspace.
- Changed Create into a workspace-first template flow, then a focused identity,
  dependency, and optional Workshop metadata step. Split Workspace overview into
  Sources, Content, and Build destinations. Moved profile-root selection from
  every catalog row into the selected content detail, and exposed an explicit
  target profile in Discover where profile-root actions are available. Replaced
  the template dropdown with grouped, explanatory role cards.
- Applied a restrained dark-blue/cyan Studio palette and clarified the Play vs
  local content-cache terminology.
- `git diff --check` passed. Vapor-managed `client deploy local` compiled and
  deployed the updated Linux client to the local Loo-Cast app root. No tests,
  UI launch, Windows run, Steam publication, or Steam launch were performed.
  The previously observed `reviewed_stage` warning was removed by keeping the
  reviewed temporary stage alive directly in the upload tuple.

## 2026-10-17 Launcher and content discovery integration

- Opening or refreshing a workspace now scans for valid, unregistered content
  manifests alongside registered source content and the local cache/install
  catalog. The scan is read-only. Loo-Cast's Engine, Game, and Packagepack now
  appear directly in Workspace and Discover with explicit adoption actions;
  adoption remains separate from moving files, changing Cargo membership, or
  publishing.
- Discover includes unregistered workspace projects in its local search,
  source/type filters, and result counts. Empty results now lead to a relevant
  next step or clear filters. Home can jump directly to workspace content.
- Workspace Sources now separates the profile's current authoring workspace
  from Vapor's registered source list. Startup workspace resolution now keeps
  the documented Studio Home landing; explicit workspace navigation still
  opens Workspace.
- Inspected Linux screenshots of Home, Workspace Sources/Content, Create,
  Discover, Play, and Workshop. The content scan found three Loo-Cast projects
  on first open without an extra scan click. No project was adopted and no
  Loo-Cast files were changed.
- `git diff --check` passed. Vapor-managed workspace builds passed for Vapor,
  Vapor-CLI, and Vapor-Studio; `vapor client deploy local` built and deployed
  the Linux client successfully. The deployed app was launched locally through
  `vapor client launch studio -- --workspace ...`; Home and Discover were
  visually confirmed. Tests, Windows, Steam launch, and Steam publication were
  not run.
- Vapor `d8774cb`, Vapor-CLI `4d6ab17`, and Vapor-Studio `9ee09b5` are
  committed and pushed. The parent checkpoint is pending. Preserve the
  unrelated dirty `Vapor-GUI/build.rs`.

## 2026-10-17 Studio reliability and authoring continuation

- Refresh failures now preserve last-known source, catalog, project-scan, file,
  and Workshop subscription data with explicit stale-state messaging. Actions
  that depend on current source/catalog snapshots are disabled until refresh
  succeeds; failed Workshop batches retain their selection for retry. Recent
  imports keep the latest 50 exact content versions for the current session.
- Workspace file discovery now includes dotfiles and configuration folders,
  while skipping repository history and generated/dependency directories.
- Opening a valid workspace no longer depends on every content manifest or the
  entire file listing succeeding. Workspace validation is advisory on open, so
  an invalid manifest or project reference can be repaired in the editor;
  workspace actions remain subject to Vapor's validation. Studio reports each
  subsystem failure in place and retains saved editor recovery data until a
  file list can be read. Content actions now require a successful current
  snapshot; adoption waits for fresh source discovery; Discover distinguishes
  failed discovery from a confirmed empty catalog; and Workshop staging,
  publishing, and profile assignment wait for fresh source/local-conflict data.
  Workspace validation re-runs during discovery, so a saved repair clears the
  warning without switching away and reopening the folder. Reopening the same
  workspace after a transient scan failure preserves its last successful lists.
- The Vapor source registry now marks failed refreshes as stale and prevents
  duplicate-registration actions until the current list is confirmed.
- `git diff --check` passed. No build, tests, formatter, deploy, launch, or
  Steam action was run, per owner direction. Studio changes remain uncommitted;
  unrelated dirty parent submodules `Vapor` and `Vapor-GUI` were preserved.

## 2026-10-17 First-run clarity and Steam launch failure trace

- Reworded Studio's primary destinations and page introductions around user
  goals (Start, Play, Find content, Set up a game, My projects, Share content,
  App status). Added short hover explanations, clarified profiles, and moved
  the profile's raw content identifier behind an optional details disclosure.
  Replaced exposed workflow terms such as "adopt", "workspace", "composition",
  and "Vapor cache" in first-run paths with plain-language project and download
  actions. Updated profile setup and operation labels, and added direct local
  and Steam Workshop routes from the Start screen.
- Traced the reported Workshop failure: the local entrypoint log records the
  latest request as `play`; Studio play mode loads the root `default`
  packagepack and installs it if missing. `App.vapor.toml` maps that packagepack
  to App ID `2122620` and PublishedFileId `3762162548`. The install path passes
  no account, and Vapor's downloader converts that to SteamCMD `anonymous`.
  This matches the failing item and confirms that the user's logged-in Steam
  client session is not used by this path. The local logs do not establish
  whether the item is public/available to anonymous downloads, so that part
  remains unverified. The exact item is also absent from the local Steam
  Workshop content directory for App ID `2122620`.
- Static diff whitespace check passed. No build, tests, formatter, deployment,
  launch, or Steam action was run. Studio edits remain uncommitted.

## 2026-10-17 Signed-in Workshop acquisition and approachability pass

- Missing root-seed content in a play profile now uses Steamworks UGC from the
  signed-in client, copies each item into Vapor's managed content cache before
  Steam closes the download session, then resolves the newly available
  manifests and dependencies. This does not subscribe the user. The existing
  SteamCMD path remains available when Steamworks cannot initialize outside a
  Steam-launched session. Generic Steam failures now explain the likely access
  checks instead of showing only `Failure`.
- Workshop search now distinguishes Download (keep a copy here) from Subscribe
  (let Steam keep it updated) and explains that neither action modifies the
  current game setup. Temporary Steam download paths are only passed to a
  synchronous import callback; the Steam provider no longer exposes them as
  long-lived paths.
- Navigation now groups game setup with Play and app status separately from
  Publish. The creation screen presents descriptive project choices while
  retaining the existing Rust-defined content types and validation rules.
  Recent-download and setup requirement panels use plain-language actions and
  keep technical details behind disclosures.
- Steam's official `ISteamUGC::DownloadItem` documentation confirms that an
  unsubscribed item's download is temporarily cached, and instructs callers
  to wait for `DownloadItemResult_t` before reading its install folder. This
  validates the chosen signed-in-client acquisition lifecycle, but does not
  establish the availability/permissions of PublishedFileId `3762162548`.
- `git diff --check` passed across the parent and modified submodules. No build,
  tests, formatter, deployment, launch, or Steam action was run. Changes remain
  uncommitted; user must verify with the requested Vapor-managed build and
  Steam launch before the workaround can be called confirmed.

## 2026-10-17 First-time user language pass

- Continued the newcomer-facing pass across Play, Create, and project-folder
  screens. Key actions now say “project folder,” “game setup,” “included
  content,” and “add to Studio,” while IDs and technical filenames stay in
  optional detail areas. Internal content kinds remain unchanged.
- Changed the Steam download path to use the signed-in Steam client for
  unsubscribed Workshop downloads. This is still a code-level proposal until
  the user's managed build and Steam launch confirm it against item
  `3762162548`.
- Whitespace checks passed for Vapor, Vapor-CLI, and Vapor-Studio. No build,
  tests, formatting, deployment, launch, or Steam action was run. All changes
  remain uncommitted; unrelated dirty files and submodules were preserved.

## 2026-10-17 Typed dependency enforcement

- Connected Vapor Core's pack-child vocabulary to semantic graph validation,
  so resolved packs reject child kinds outside their declared role set.
- Vapor Shell now checks dependency target kinds and reserved role labels in
  source validation, structured graph saves, install previews, actual installs,
  and installed launch-closure verification. Packagepacks require one Engine
  or Enginepack and one Game or Gamepack.
- Studio now obtains all required role slots from the shared Shell contract.
  Create and composition screens require a Game and describe that rule
  consistently. Launch graph/result types carry the resolved Game as required.
- `git diff --check` passed in Vapor, Vapor-CLI, and Vapor-Studio. No build,
  tests, formatter, deployment, launch, or Steam action was run. Existing dirty
  changes remain uncommitted and preserved.

## 2026-10-17 Play handoff and page-height regression

- After the owner tested the local deployment, the latest Steam entrypoint log
  confirmed `play` launched `vapor-gui`, and the content index showed the
  default Packagepack, Engine, and Game installed successfully. The installed
  Engine's `src/main.rs` is still the old placeholder that prints that its
  runtime is unavailable and exits successfully; the current Loo-Cast source
  Engine has a Bevy game entrypoint. Local client deployment does not update
  Workshop content, so the Steam Play path cannot open the game until the
  current Engine content is staged and published.
- Removed the new page-centering wrapper around Studio's full-height scroll
  area, restoring the previous direct CentralPanel scroll layout after the
  owner reported page content collapsed into a thin top strip. This needs the
  owner's visual recheck.
- `git diff --check` passed in Vapor-Studio. Vapor-managed `vapor client deploy
  local` compiled and deployed the layout change to the local Loo-Cast app root.
  No tests or app launch were performed. Steam content was not published; the
  owner should recheck the layout and decide when to stage/publish the updated
  Loo-Cast Engine item.

## 2026-10-17 Packagepack launch artifact

- Confirmed the installed Spacetime Engine binary was the success-exiting
  placeholder. Play ran it, so the launcher reported success without opening a
  game. Moved the runnable Loo-Cast composition to a Packagepack executable;
  Packagepack launch now resolves that executable from its target-specific
  runtime payload. The Game integration now installs the actual game and
  developer adapter plugins. Engine is library-only for this composition, and
  the unused Game shared library was removed from its Workshop payload.
- Updated the tracked Loo-Cast Cargo lock entry for the Packagepack's Engine
  and Game dependencies. Disabled third-party dependency debug symbols in the
  developer profile to reduce the host Packagepack from 2.5 GB to 589 MB.
- Vapor-managed `vapor client deploy local` succeeded, followed by
  `vapor content deploy ghf-studios/loo-cast/loo-cast-packagepack --select
  --host-only`. The default Packagepack is selected and contains an executable
  at `bin/x86_64-unknown-linux-gnu/loo-cast-packagepack`. Active source was
  restored to Vapor-Client. No tests or game launch were run; Steam Workshop
  was not uploaded or changed. The installed developer binary still carries
  debug info and is not the release-sized cross-platform Workshop publication.
  Next: owner tests Play through Steam; then address release-profile and full
  target-matrix Workshop packaging before any upload.

## 2026-10-17 Studio service-boundary design

- The approved architecture and implementation sequence are in
  `Vapor-Studio/docs/superpowers/specs/2026-10-17-studio-service-boundary-design.md`
  and `Vapor-Studio/docs/superpowers/plans/2026-10-17-vapor-studio-service.md`.
- Implemented the typed protocol, bounded stdio process client/server, fixture
  simulator and scenarios. The GUI now sends local catalog refresh, search,
  exact-version selection, workspace activation, and active-profile selection
  through the child service. Production Workshop publication is unreachable
  from fixtures.
- Vapor-managed Studio `validate` passes formatting, compilation, tests,
  Clippy, and Rustdoc. The compiled service simulator also passed the
  `fixture-profile-selection` scenario over a real child process. Tests exercise
  protocol round trips, stale revisions, frame caps, malformed input recovery,
  child-process lifecycle, fixture scenarios, and the publication boundary.
- `vapor root build --host-only` built Vapor core, then stopped at `vapor-gui`
  because its ignored local `Cargo.lock` needs regeneration while the managed
  workflow passes `--locked`. Consequently, promotion of the service beside
  `vapor-gui` is not yet verified. No deployment or Steam action was run.
- This is a foundation slice, not the finished service migration: other action
  families still report unavailable; profile persistence, provider ports,
  operation ownership, most workspace/editor flows, and the remaining GUI
  workflows are still in the plan.


## 2026-10-18 Studio service Part 1 complete

- Service-owned workspace discovery is implemented. `StudioSnapshot` now
  carries registered source projects, unregistered adoption candidates, scan
  diagnostics, independent partial errors, and workspace validation state.
  The session bounds project/diagnostic counts, preserves last good scan data on
  same-workspace failures, resets on workspace changes, and gates explicit scans
  on an open workspace.
- Catalog refresh, workspace activation, and the explicit project-scan control
  now consume the child service snapshot. GUI workers no longer call source or
  candidate discovery directly. Editor file enumeration remains in the UI and
  is part of the later editor migration.
- Added conversion and backwards-decoding coverage, a fixture-only child-process
  scenario, and interactive simulator `open`/`discover` commands. Large protocol
  snapshot/response payloads are boxed in memory; JSON shape is unchanged.
- Vapor-managed `fmt --project vapor-studio` and
  `validate --project vapor-studio` pass (33 tests, check, Clippy, Rustdoc). No
  deploy, publication, or game launch was run.
- Next active slice: host-owned durable state and one-time import of existing
  profile data. The service foundation has not yet taken over profile CRUD,
  drafts, or operation history.

## 2026-10-18 Studio coherence and profiling pass

- Root Steam publishing now runs in the Studio service. The service owns the
  review token, rechecks the exact confirmation phrase, and executes staging or
  upload as a background operation; fixture mode rejects this authority.
- The Studio protocol carries the first-party release review/result. The UI
  submits typed actions and renders service snapshots; it no longer invokes the
  SDK root publisher directly. Protocol version is 6.
- Registered project roots now include repositories discovered through the
  active Vapor Superworkspace, so first-party container and workspace repos can
  be opened from Studio without treating them as content.
- Browse and Profiles share a framed contextual panel; the details pane has
  more room. Create exposes one template family at a time and keeps sample
  workspace setup collapsed. Projects separates registered projects, adding
  existing folders, and project settings into focused views. The setup composer
  has a clearer name, and primary actions no longer stretch across the page.
- Restored the shipped shell routes `toolchain tracy` and
  `launch --current-workspace --profiling`. Devtools now explains that a Steam
  launch argument does not create a Vapor Development Session.
- `git diff --check` passed. The user will run compilation and tests. The
  Vapor-managed formatter could not write into the mounted submodule checkout,
  so formatting and compilation remain unverified; no build, test, deployment,
  or Steam publication was run.

## 2026-10-18 Shared Vapor Client Service direction

- The owner approved one local service process with one shared Rust API for
  Studio, CLI, Installer, Steam launch modes, and future language tooling.
  Frontends connect to that process; they do not own alternate implementations
  of workflow policy.
- Steam Entrypoint remains a narrow handoff adapter: app-root discovery,
  launch-mode selection, logging, and facade handoff. Shared frontend
  connection code owns service discovery/startup/lifetime.
- Added `Vapor/docs/vapor-client-service.md` as the concise architecture
  contract. The prior Studio-only stdio child design is marked historical.
- Existing content/Studio edits remain uncommitted and preserved. No build,
  tests, deployment, launch, or publishing was run during this design shift.
- Next: relocate the current protocol/client API into Vapor shared code, then
  evolve the per-GUI child process into the single local IPC service and route
  frontends through it. Preserve one service instance and eliminate duplicated
  workflow semantics as each action family migrates.

## 2026-10-18 Shared service architecture correction

- The owner clarified that the implementation must use the intended shared
  service and domain model from the outset; this is not a provisional API to
  clean up after migration.
- Steam Entrypoint invokes the shared bootstrap manager before frontend
  handoff. The manager owns process discovery, startup, connection, recovery,
  and shutdown policy; the entrypoint does not duplicate that machinery.
- Moved the client and protocol Rust sources into `Vapor/crates/` and replaced
  the public `Studio*` message names with client/service names. Removed the
  `Client* = Studio*` aliases. This is source/API cleanup only; it does not
  make the process model complete.
- Reworked the lifecycle plan to put semantic ownership and domain mapping
  before moving the service host. In particular, it records the current split
  between ProfileAction and CompositionAction and the reuse of profile storage
  structures in protocol snapshots as design work, not settled domain shape.
- Current evidence: the host is still a Studio-owned stdio child, still depends
  on Studio workflow code and CLI code, and still starts one process per
  frontend. Vapor-managed format and compile checks now pass for `vapor` and
  `vapor-studio`; the elevated managed shell was required to use the existing
  app-local build cache. Tests were not run per owner preference.
- Next: settle the canonical composition/application domain and untangle the
  `Studio service -> Studio core -> CLI` dependency before selecting or
  implementing the multi-client IPC host. Do not treat the current child
  process as the accepted service architecture.


## 2026-10-18 Canonical content-kind slice

- Removed `vapor_shell::manifest::ContentKind`; Vapor CLI now consumes
  `vapor_core::ContentKind` directly. Manifest filenames and kind spellings
  derive from that shared enum, and SDK/CLI manifest discovery iterate its
  canonical kind list.
- Routed CLI dependency-kind validation through the shared Vapor rule instead
  of a second pack/member policy table.
- Added `Library.vapor.toml` recognition and carried that existing core kind
  through CLI authoring and deployed-manifest parsing. Whether Studio exposes
  Library as a creator artifact remains a product/domain classification point.
- Vapor-managed format and compile checks pass for `vapor`, `vapor-cli`, and
  `vapor-studio`. No tests were run per owner preference. No runtime deploy,
  launch, publication, or commit was performed.
- The remaining domain mismatch is concrete: protocol `ContentIdentity` and
  `ResolvedCompositionGraph` duplicate Vapor's `VaporId`, `ContentVersionId`,
  and `ResolvedComposition`; the API also still exposes separate profile and
  composition action families and profile storage structures.


## 2026-10-18 Crate content semantics

- The owner decided to rename the Rust-only content kind from `Library` to
  `Crate`. Its canonical enum, manifest section and filename, CLI command, and
  reconciliation API now use `Crate`, `crate`, and `Crate.vapor.toml`.
- A Crate is build-time Rust code, not runtime-loaded content. It emits Rust
  `rlib` and `staticlib` outputs, may depend on other Crates, and is excluded
  from pack membership. Game and add-on code roles may depend on Crates.
- Studio's separate acquired-content `Library` remains the collection of
  locally available non-Packagepack content; Packagepacks remain compositions.
  The shared Library availability aggregate is still a known gap.
- Fixed the stale Studio workspace error that claimed a supported Crate kind
  had no manifest role. Vapor-managed `sdk check` passes for Vapor, Vapor-CLI,
  and Vapor-Studio. No tests, deployment, publication, launch, or commit was
  performed.


## 2026-10-18 Shared-domain ownership audit

- Continued the approved plan past composition naming and checked the live
  ownership graph instead of moving the current Studio core wholesale.
- Found that `vapor_core` is a broad application/infrastructure crate as well
  as a semantic model: its public modules include CLI dispatch, Cargo,
  installation, Steam deployment, profiling, and developer-environment
  orchestration.
- Found the resolution blocker: `vapor_core::LocalCatalog` still consumes the
  provisional generic `Vapor.toml`, while current content uses role manifests
  such as `Engine.vapor.toml` and `Packagepack.vapor.toml`. The Shell
  `preview_install` path independently resolves across source, cache,
  installed content, and Workshop seeds, while also carrying source
  provenance. Those concerns need one shared resolver and a separate
  availability/install projection before either implementation is removed.
- Updated the Vapor Client architecture contract and Studio implementation
  plan with the verified ownership matrix and prerequisites for service
  extraction. The plan now separates the active authoring folder, canonical
  Superworkspace (default `~/Vapor`), and optional per-composition source
  binding. The existing regression test proves folder changes preserve
  composition selection, source association, and launch arguments; explicit
  Superworkspace state/actions remain unimplemented.
- No product code, deployment, userdata, or Steam state was changed during
  this audit. No tests were rerun because only architecture documentation
  changed; validation from the composition pass remains green. Run
  `git diff --check` before continuing.


## 2026-10-18 Separate player Library from Compositions

- Renamed the old `Section::Library` route to `Compositions`, including its
  page and composition-only helper modules. The navigation now has distinct
  Compositions, Library, and Browse destinations.
- Added a player Library view backed by downloaded and installed catalog entries,
  excluding Packagepacks and creator-only static Crates. Search and type filters
  are independent from Browse; cache records are presented as “Downloaded.”
- Successful Workshop acquisition now refreshes shared service catalog state
  and lands in Library. Partial acquisition stays in Workshop Imports for retry.
- Vapor-managed Studio compilation passes after these changes. No tests were
  run per owner preference. The Library is still a Studio projection over the
  existing catalog contract; canonical identity/availability/action semantics
  must still move into shared Vapor code. The standalone Add-to-compatible-
  composition workflow and actual static linking of Crates also remain gaps.


## 2026-10-18 Composition-facing player language

- Play, Compose, the composition selector, and player actions now use the
  approved Composition language instead of exposing “profile” as a second
  player concept. The Play screen no longer reveals a raw content selector.
- Legacy saved keys and protocol/profile identifiers remain intact for data
  compatibility. The actual DTO/action migration and shared Library availability
  model are still required; this wording pass does not resolve those boundaries.
- `vapor sdk check` passes for Vapor, Vapor-CLI, and Vapor-Studio after correcting
  the root UI module path to its still-existing `profiles.rs`. No tests, deployment,
  publication, launch, or commit were performed.


## 2026-10-18 Player-facing composition cleanup

- Replaced player-visible “profile” and “game setup” wording with the approved
  Composition concept across Play, Compose, Workshop, and composition actions.
  Legacy local-storage keys remain unchanged for migration compatibility.
- Removed raw Packagepack ID/version selectors, install paths, fingerprints,
  cache/record terminology, and payload fingerprints from ordinary player panels.
- Vapor-managed `sdk check` passes for Vapor, Vapor-CLI, and Vapor-Studio; the
  Studio check is warning-free. `git diff --check` passes. No tests, deployment,
  publication, launch, or commit were performed.
- The next protocol step is blocked by automatic review: unifying the `Profile`
  and `Composition` action families and renaming serialized composition state
  could alter IPC and persisted data. The intended migration needs explicit
  approval for that exact contract change before implementation.


## 2026-10-18 Studio navigation enclosure

- Reworked the desktop rail so every destination, search, and project-folder
  action sits inside one amber outlined Creator Tools group. Routes are grouped
  by Play, Discover & Library, Make & Manage, and Share & App; Compose now sits
  with authoring rather than player launch. Both desktop and compact navigation
  render from one canonical route-group definition. The duplicate project-folder
  action was removed from the desktop header.
- Reduced the fixed desktop rail from 220 px to 194 px and navigation rows from
  32 px to 28 px, tightened row spacing/padding, and removed a duplicate brand
  wordmark. The project action is compact and remains in the same group. The
  compact-navigation breakpoint now sits just below the 980 px minimum window
  width, so the vertical rail remains available at the supported minimum.
- `vapor client deploy local` built and deployed the updated Vapor GUI
  successfully after correcting an unsupported egui `Panel::width_range` call
  and the nested UI module import for shared route groups. A final deployment
  passed after removing the duplicate header control and centralizing the group
  list for both desktop and compact navigation.
  `git diff --check` passes. No tests, userdata deletion, publication, launch,
  or commit were done.
- Dedicated Studio state locations (`~/.local/share/vapor/studio` and the eframe
  `~/.local/share/vaporstudio` path) do not currently exist, so there was no
  Studio userdata to clear. The similarly named `com.ghf-studios.vapor.sdk`
  directory is unrelated and was left untouched.


## 2026-10-18 Catalog authority and stale completion pass

- Applied the prepared catalog contract in the live dirty checkout: complete
  catalog facts are retained separately from query identities, freshness/error
  status, and explicit truncation. Shared search now matches ID, kind, version,
  and Workshop ID. Browse and Library keep independent local filters.
- Catalog refresh preserves the query and last-good facts on failure; unrelated
  request results cannot clear catalog errors. Workspace opening remains
  applied when its follow-up catalog refresh fails. Workshop staging refuses
  to treat a truncated catalog as complete conflict evidence. The wire protocol
  is now version 7; persisted profile data was not changed.
- Composition operations capture the profile inputs they use. If the selected
  root, workspace, or resolved graph changes before completion, the result is
  not applied to the changed profile. A successful old install still refreshes
  the catalog and reports that it completed for the previous selection.
- Added service, protocol, and process regression coverage. Updated the
  protocol action round-trip inventory for the current action set. Strict lint
  findings in the active migration were fixed (boxed large operation results,
  borrowed path comparison, collapsed nested conditionals, boxed the large
  resolution error payload).
- Vapor-managed `validate --project vapor-studio` passes formatting, check,
  56 tests, Clippy, and Rustdoc. `validate --project vapor` passes formatting,
  check, 39 tests, Clippy, and Rustdoc. Total: 95 passing tests across the two
  affected workspaces. No deployment, Steam action, commit, or push was made.
- The authoring-folder versus playable-composition association remains an
  explicit owner decision; the question is pending. The full shared-service
  host, operation availability contract, async responsiveness, provider
  isolation, multi-session persistence policy, and rendered UX review remain
  open passes. This checkpoint is not a full-stack completion claim.


## 2026-10-18 Fixture isolation, scenario outcomes, and navigation correction

- Fixture mode now rejects real workspace paths before inspecting them. It
  accepts only isolated temporary workspace folders for file scenarios, and
  rechecks that boundary in the worker before file access. Remembering a
  workspace, running production validation, changing the installed source
  registry, and all authoring/toolchain actions are unavailable in fixtures.
- Scenario paths use the explicit `fixture://workspace` namespace. The runner
  maps it to a temporary directory and removes that directory when the scenario
  ends. It now waits for accepted asynchronous operations to emit a terminal
  event and updated snapshot, with expectations for action, status, and result
  text. A checked-in child-process scenario proves this path.
- Corrected the sidebar enclosure based on review: Play and Find Content remain
  player areas; only Create, Compose, Projects, and Share sit inside the amber
  Creator group; App Status has its own Application group. The brand and search
  sit above these areas without labeling the whole rail as Creator Tools. The
  rail width is 178 px and its rows/padding are tighter.
- Vapor-managed `validate --project vapor-studio` passes formatting, compile,
  58 tests, strict Clippy, and Rustdoc. Combined with the already validated
  Vapor and CLI workspaces, this checkpoint covers 102 tests across all three.
  No deployment, publication, commit, or push was performed.
- No rendered screenshot review has been done, so the new navigation taxonomy
  and sizing still need visual confirmation. The larger service ownership,
  operation readiness, and product journey passes remain active.


## 2026-10-18 Authoring workspace / composition boundary

- The owner confirmed that opening a creator folder must leave the selected
  playable composition and its launch settings unchanged; authoring workspace
  is tracked separately.
- `WorkspaceAction::Open` now changes only the service's global authoring
  workspace and its workspace/catalog projections. It no longer attaches that
  folder to the active composition, clears its Packagepack/resolved graph/
  installation, or increments the composition revision.
- Creating or selecting a composition no longer implicitly binds or switches
  the authoring workspace. A composition can still retain an explicit source
  workspace binding for source-run behavior; it is independent state. The core
  composition-creation helper no longer accepts a workspace argument, making
  that binding explicit rather than implicit at its API boundary.
- Added regression coverage proving an installed composition, graph, explicit
  source binding, and launch arguments survive opening another folder, and
  proving profile selection/creation do not move or inherit the editor context.
- Vapor-managed `validate --project vapor-studio` passes: formatting, compile,
  60 tests, strict Clippy, and Rustdoc. `git diff --check` passes across the
  parent, Studio, and shared Vapor workspaces. No local deployment, Steam
  action, commit, or push was performed.
- This resolves the previously pending workspace/composition decision. The
  one-service process model, shared domain contracts, player operation
  lifecycle, and visual review remain active work—not completion claims.


## 2026-10-18 Composition contract and saved-state migration

- Promoted composition as the actual shared contract: renamed the public
  protocol and Studio core types/modules, document fields, active selection,
  revision, action IDs, operation identifiers, and service bootstrap field.
  Bumped the client protocol to version 9; the service accepts the old
  `--active-profile` process argument only as a compatibility alias.
- Added a schema 1 to schema 2 durable-state migration. Old `profiles`,
  `active_profile_id`, and `profile_revision` keys are accepted only while
  decoding schema 1; migration rewrites canonical composition keys atomically.
  Existing composition identity, Packagepack, graph, source association,
  launch arguments, global workspace, and auxiliary state are retained.
  Runtime installation observations remain intentionally transient.
- The legacy egui seed is now explicitly the only profile-named import shape;
  current writes and protocol snapshots use compositions. GUI bootstrap reads
  prior storage keys and forwards the legacy payload to the service.
- Added regression tests for v1 state migration and launch-setting preservation,
  canonical protocol serialization, plus updated process scenario naming.
- Vapor-managed `validate --project vapor` passes: formatting, check, 41 tests,
  Clippy, and Rustdoc. `validate --project vapor-studio` passes: formatting,
  check, 61 tests, Clippy, and Rustdoc. Combined, 102 tests pass across the
  two affected workspaces.
- No deployment, userdata deletion, Steam action, commit, or push has been
  performed. Full domain ownership, shared service-host extraction, player
  lifecycle, and UI review remain open.


## 2026-10-18 Composition UI alignment and panel ownership

- Renamed the UI's selected composition state, lifecycle actions, creation,
  preview, launch, and selection flows to composition semantics. Old profile
  storage keys and fields now appear only in the explicit legacy-egui import
  adapter. The developer simulator uses `composition ID` as its documented
  command; `profile` remains an undocumented command alias for continuity.
- Split composition selection and management into separate page modules, and
  renamed the composition-content panel to describe its actual responsibility.
  The composition route now assembles selection, management, content, install,
  and launch panels by name rather than hiding selection under a profile file.
- The service and core operation/result names now agree with the composition
  action family, and composition CRUD errors and diagnostics use the product
  noun. Legacy decoder identifiers are retained only when upgrading old data.
- Vapor-managed `validate --project vapor-studio` passes: formatting, compile,
  61 tests, strict Clippy, and Rustdoc. The shared Vapor workspace also passed
  its full validation earlier in this pass: 41 tests, Clippy, and Rustdoc.
  `git diff --check` passes in the parent, shared Vapor, and Studio worktrees.
  No local deployment, Steam action, commit, push, or userdata migration was
  performed.


## 2026-10-18 Shared composition and Library policy ownership

- Moved composition create/rename/remove/duplicate rules, exact graph identity
  comparison, and stable graph fingerprinting from Studio core into
  `vapor_client_domain`; Studio core now re-exports the shared rules and keeps
  only legacy eframe storage decoding locally.
- Moved the durable cross-frontend state document to `vapor_client_domain`;
  the protocol snapshot and Studio file store now adapt the same document.
- Moved composition state transitions to `vapor_client_domain`; the service
  retains host-specific default-root lookup, persistence, revisioning, and rollback.
- Added durable Library Auto-Install preferences to service state and snapshots,
  plus a domain-owned `LibraryAction::SetAutoInstall` operation with ID validation and
  atomic save/rollback behavior. The preference defaults to retained/manual
  management. Incremented the client protocol version to 10.
- Kept automatic update selection and unreachable-item collection as domain
  policy only; lifecycle orchestration and provider effects remain open.
- `git diff --check` is the only verification run; no build, test, deploy, or
  launch was performed per owner preference.

- Extended `vapor_content::resolve_from` so a host can select an eligible exact
  artifact per dependency while the shared resolver owns edge validation, graph
  construction, convergence, and cycle detection. Shell preview has not yet
  adopted the shared traversal; source/cache/installed policy stays its adapter
  responsibility. No build, tests, or deployment were run.
- Persisted each resolved dependency's authored version requirement so later
  automatic updates can honor composition compatibility. Added a shared query
  that gathers constraints across all compositions and fails closed when a
  legacy or malformed edge lacks a usable range.
- Moved `InstallPlan` and its node/source/dependency result types from Shell into
  `vapor_client_domain`; Shell retains compatibility re-exports while provider
  selection remains in its adapter. Added the path dependency to the CLI
  lockfile without running Cargo.
- Added optional-edge semantics to shared `DependencySpec` and resolution:
  optional content remains in authored/provider data but is deliberately omitted
  from automatic required-graph resolution.
- Replaced Studio core and protocol copies of install-preview node/source/
  dependency records with aliases to the shared application plan. The service
  and GUI adapters now pass those records without field-by-field semantic
  remapping; the protocol wrapper still carries the selected composition ID.
- The protocol was already at version 10 for the expanded shared contracts;
  its source enum now uses the domain's `RootSeed` value consistently. Added
  direct shared-domain dependencies and lockfile entries to CLI and Studio
  core rather than relying on a transitive or Shell-owned alias.
- Moved pack-member, target-relationship, pack-shape, and playable composition
  validation from the broad `vapor_core` resolver into `vapor_content`.
  Core retains its catalog-facing compatibility functions and uses the shared
  error type. The shared validator now rejects malformed graphs with missing
  roots or dangling edges instead of assuming its public input is well formed.
- Replaced the Shell install preview's recursive resolver with
  `vapor_content::resolve_report_from`; provider selection still applies the
  existing workspace-first-for-authored-closures, installed, cache, then seed
  order. The shared graph supplies dependency-first plan order, optional edges
  remain omitted by shared policy, and complete Packagepack graphs pass the
  shared validator before the preview can be ready.
- Moved catalog merge identity, same-version metadata conflict detection, and
  location reconciliation into `vapor_client_domain`; Studio core now only
  gathers workspace/cache/installed observations and calls the shared merge.
- Moved install-plan orchestration into `vapor_client_domain` behind
  `InstallContentProvider`. Shell now implements root/dependency candidate
  selection and returns availability/provenance facts; the shared planner owns
  exact traversal, compatibility, cycle reporting, dependency ordering,
  conflicts, and readiness.
- `git diff --check` passed for the parent, Vapor, CLI, and Studio repositories.
  No build, tests, deployment, commit, or push was run.
