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
  deploy, launch, or Steam action was run. Vapor-Studio commit and matching
  parent checkpoint are recorded in Vapor-Studio `6a8770b`; the parent
  checkpoint is pending.
