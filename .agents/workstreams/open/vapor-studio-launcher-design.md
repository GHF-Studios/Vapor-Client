# Vapor Studio: launcher, compositions, and Steam

**State:** owner-approved direction; implementation design for review  
**Scope:** Vapor GUI, composition lifecycle, Steam entrypoints and publishing, and the handoff to Loo-Cast's in-game workbench.

> **Quick review:** Steam opens Vapor Studio by default. Studio manages
> compositions, authoring, local deploy/run, Workshop, and safely confirmed
> Steam publishing; direct play and recovery remain stable Steam entrypoints.
> Loo-Cast, USF, and scripting integration are part of the goal. Build/test
> uses Vapor commands at the correct workspace roots; deploy/publish/Steam
> launch remains an explicit owner-controlled action.

## Product goal

Vapor's normal Steam launch opens a polished GUI where a player can select and
run an experience, and a creator can discover, compose, edit, deploy, and
publish content. The GUI is the normal path. The CLI remains a recovery and
automation surface, not the product's primary workflow.

The same product serves two related jobs:

- **Launcher:** manage a library of installed/available compositions, resolve
  their content, choose a version/source, and launch the selected experience.
- **Studio:** create and edit content, assemble Packagepacks, inspect runtime
  state, deploy locally, and publish with an explicit safety review.

The existing eframe `vapor-devtools` application is the GUI foundation. Its
runtime telemetry and Tracy views become a Diagnostics area in Studio instead
of defining the whole product. Steam remains the stable bootstrap and exposes a
small, deliberate set of entrypoints: Studio (default), direct play of the
selected composition, and recovery shell/installer.

## Current implementation facts

- The active repositories are `Vapor`, `Vapor-CLI`, `Vapor-GUI`,
  `Vapor-Studio`, `Vapor-Steam-Entrypoint`, and `Vapor-Installer`. The root
  Steam AppID is `2122620`; Vapor-GUI selects Studio, Play, or Installer modes
  behind stable Steam entrypoints.
- Vapor already owns managed workspace operations and application-local tool
  providers. `vapor run` and related commands use the selected workspace's
  pinned toolchain and Vapor-managed caches; this coding pass does not invoke
  raw Cargo.
- Vapor-CLI contains app-root content discovery, fingerprints, cache,
  installation, verification, repair, Workshop, and SteamPipe workflows.
  These authorities and storage domains are distinct.
- Vapor-Studio has a GUI-owned profile list, local content catalog, workspace
  file editor, managed development actions, Workshop search/subscription and
  acquisition paths, composition preview/install/update/play, and a staged
  Workshop review/publish path. These pieces work, but they are still
  coordinated by a large `StudioApp`. Vapor Core now owns a shared lifecycle
  runner used by all Studio background jobs and Vapor Shell commands. The Shell
  wraps root, source, workspace, docs, IDE, content, provider, SDK, diagnostics,
  launch, and direct Client deployment in the same lifecycle service; script
  steps are recorded individually. Studio-specific operation names and result
  reconciliation remain UI-owned. CLI phase is currently coarse and logged to
  diagnostics; cooperative cancellation is not connected to a host handle.
- Studio's local catalog now joins workspace, Vapor cache, and installed
  records by exact `(id, version)` and reports observed metadata/fingerprint
  disagreement. Resolved installed profile graphs retain each node's payload
  fingerprint and Vapor-recorded install source class; their graph fingerprint
  covers exact nodes and edges. These are foundations for a real lock/provenance
  view; source workspace payloads do not yet have comparable deployed
  fingerprints or precise descendant source roots.
- Workshop search and subscription state come from SteamUGC. Steam's
  subscription/download state is not yet a Vapor cache/install state, and
  acquisition, cache installation, profile update decisions, and Workshop
  compatibility display still need one coherent lifecycle.
- Studio now has a first Compose surface for editing local Packagepack graphs.
  The editor loads an exact manifest snapshot, adds/removes workspace edges,
  and saves through Vapor Shell. Shell checks role cardinality, version
  requirements, graph cycles, and local authored descendant closure before a
  guarded manifest replacement. A saved graph can become the active profile
  root and flow through the existing dependency preview. This source-only pass
  has not been built or run; semantic editing for other content roles,
  Workshop-backed graph composition, and review of the full descendant lock
  remain future work.
- Workspace editing is currently file-tree plus text editor and TOML syntax
  diagnostics. Studio can register an already valid Vapor Workspace with the
  installation without changing its active source; this is source-only and has
  not been compiled. Project-level adoption/repair, semantic editing, Rust/Rhai
  language services, documentation discovery, and the full IDE/workbench
  connection remain absent.
- Studio's Workshop staging/publishing and local root preview are separate
  from full SteamPipe publication. The GUI does not yet offer a complete
  root-app publish flow or a reviewed local-deploy-to-Steam launch chain.
  Workshop upload now revalidates the reviewed payload fingerprint in the Shell
  operation and uploads an operation-owned copy of those exact staged bytes.
- Loo-Cast's in-game script workbench remains inside the game process and is
  not yet connected to Studio's authoring or operation lifecycle.
- Steamworks launch options were updated to the four stable entrypoint modes;
  BuildID `25970591` was accepted for `vapor-dev`. Linux/Windows runtime
  behavior remains owner-tested rather than established by this source review.

## Ownership and data flow

```text
Steam entrypoint
      │
      ▼
Vapor-GUI mode router
      │
      ├── Studio ─── profile/composition ───► Vapor operations
      │     │                                  │
      │     ├── authoring/editor                ├── identity, discovery,
      │     ├── publish review                  │   resolution, build/run
      │     └── diagnostics                     └── Vapor-CLI services:
      │                                              acquire/install/publish
      ├── direct Play ─────────────────────────► selected profile
      └── Installer

Loo-Cast in-game workbench ◄── shared bounded authoring/operation contracts
```

- **Vapor** owns stable content identity, local source discovery, composition
  resolution, build/run operations, and managed tool providers.
- **Vapor-CLI** owns app-root state, remote acquisition and installation,
  Steam/Workshop providers, root distribution staging, and platform entrypoint
  packaging.
- **Vapor-Studio** owns player/creator workflows: library, composition
  selection and authoring, workspace editing, launch intent, and GUI-facing
  operation coordination. It consumes Vapor and Vapor-CLI capabilities;
  lower layers do not depend on Studio.
- **Vapor-GUI** owns desktop mode selection and platform integration for Studio,
  direct Play, and Installer. It does not become a second content authority.
- **Vapor-Studio** owns presentation, navigation, progress, and confirmation.
  The GUI must not duplicate content resolution or directly mutate Steam state.
- **Loo-Cast's in-game workbench** remains inside the game process and closes
  when that process restarts. It edits the selected source workspace and invokes
  bounded host operations. It shares the same operation/data contracts as
  Studio, not Bevy references or editor-only copies of Vapor logic.
- Long operations run outside the frame loop, return structured progress and
  results, and can be cancelled where the underlying operation supports safe
  cancellation. The operation host is transport, not a second business-logic
  implementation.

### One managed operation path

The CLI and GUI are adapters over one typed operation lifecycle service. An
operation has a process-local identity, domain/action request, phase, optional
progress, and terminal outcome. The owning Vapor or Vapor-CLI handler still
performs the domain work; the shared service coordinates lifecycle reporting,
not a second business implementation. Studio supplies the selected workspace
or app root explicitly. The CLI resolves its source context from Shell state.
Both use the same lifecycle contract; neither relies on system Cargo for
managed development operations. Cancellation remains unsupported until a host
can retain and signal a safe operation handle.

Routine authoring and run workflows never invoke system `cargo`, `rustc`, or
`rustup` directly. `vapor run`, `vapor build`, and `vapor test` remain useful
human-facing commands, while Studio submits equivalent typed requests; neither
frontend gets a private implementation. Vapor's managed toolchain utility is
not the normal way to build a project. This gives users more centralized
operations, not fewer operations: format, check, test, build, clean, run,
deploy, acquire, and publish all go through Vapor's owned toolchain, caches,
workspace rules, and operation reporting.

## Composition model

A composition is a selected Packagepack plus its resolved, versioned content
graph and launch profile. The library can contain local authored content,
Workshop content, and first-party Steam-root content. The GUI presents origin,
version, update state, dependency roles, and whether an item is editable.

Creating a composition means authoring the Packagepack's declared content
relationships, resolving them, and reviewing the resulting graph. It does not
copy or silently rewrite child content. Content authored locally follows the
owner-approved locality rule: once a graph node is locally authored, its
constituent/descendant nodes are locally authored as well. Remote compositions
remain selectable and installable without being locally authored.

Each named composition/profile keeps its own selected Packagepack, resolved
versions/lock, local-vs-published source choices, installed state, and launch
arguments. Switching profiles must not silently overwrite another profile's
selection. The lock records resolved identities and provenance so the GUI can
explain exactly what will run and what will be published.

## GUI surfaces

- **Library:** installed compositions, available content, update/repair state,
  source and provenance, select and launch.
- **Compose:** create/edit Packagepacks, browse compatible content, add/remove
  relationships, resolve, and inspect dependency changes.
- **Workspace:** discover/open source projects, edit manifests and scripts,
  inspect diagnostics, and connect to the in-game workbench. This grows toward
  the user's full IDE goal; RustRover remains useful during the transition.
- **Workshop:** browse/acquire/publish Workshop content with clear ownership,
  PublishedFileId, visibility, version, and dependency information.
- **Publish:** present separate SDK-owned Workshop content and SteamPipe
  root-app publication workflows. Show the exact target, account,
  branch/visibility, app/depot IDs,
  staged payload and diff, selected source/versions, and consequences before
  the final authority-changing action. Preview is non-publishing. Final publish
  requires an explicit confirmation in the GUI; create/delete authority stays
  more strongly gated.
- **Diagnostics:** preserve current telemetry, logs, sessions, and Tracy capture
  workflows.

## Steam launch and local deploy

Steam stays the stable product entrypoint, not the content manager. Normal
launch opens Studio. Direct Play starts the currently selected composition; if
no valid selection exists, it returns to Studio with a useful explanation.
Shell and Installer remain recovery/bootstrap paths. Entry points are stable
product modes, not one Steam launch option per composition.

`Deploy Vapor Locally` means build and promote the Vapor application from the
Vapor source workspace into its selected local Vapor app instance. It is not a
Loo-Cast deployment, a SteamPipe publish, or a Workshop publish. `Launch in
Steam` then opens the Steam-installed root entrypoint, which hands off to the
selected local composition/profile through a validated launch request. It must
not replace Steam-managed files or change Steam launch options as a hidden side
effect. The launch request is bounded, explicit, and visible in the operation
report.

Running a Loo-Cast Packagepack is a first proving workflow, not an excluded
project. Vapor must resolve and run it from the actual
`/home/leslieghf/Vapor/Loo-Cast` workspace so manifest discovery and run
configuration resolution are rooted correctly.

## Publishing safety

Publishing has two distinct destinations:

1. **Workshop content:** publish one or more content artifacts through the
   Workshop provider, preserving artifact identity and visibility settings.
2. **Steam root:** stage the Vapor application/depot set and publish through
   SteamPipe to a named non-default branch.

Both flows show a review screen derived from the actual staged operation, not
from form fields alone. It includes credentials/account identity without
exposing secrets, exact destination IDs, branch/visibility, version, content
graph, files to upload, and the resulting public/user-visible change. The final
button names the real action (for example, “Publish 3 Workshop items” or
“Publish Vapor root to `vapor-dev`”). No publish occurs on preview, navigation,
or closing the review. SteamPipe root publishing and Workshop publishing never
share a generic ambiguous “Publish” handler.

## Delivery order

1. **Identity and operation contract:** define exact cross-source content
   identity, provenance, conflict handling, and a versioned resolved graph lock;
   then finish the structured operation service and expose discovery, resolve,
   format/check/test/build/clean/run, local deploy, acquire, publish-preview,
   and launch through existing Vapor implementations. The CLI and GUI must
   share handlers and managed state. Studio currently has the first catalog
   conflict and installed-payload fingerprint slice, but not a source-complete
   lock or shared operation service.
2. **Studio shell and Steam handoff:** turn the current eframe diagnostics app
   into a navigable Studio shell, keep Diagnostics intact, route Steam's default
   entrypoint to Studio, and implement local deploy → Steam launch as a visible
   operation chain.
3. **Local composition loop:** discover and edit Packagepacks, resolve and lock
   graphs, install local realizations, maintain named profiles, and direct-play
   the selected composition.
4. **Remote content loop:** connect Workshop browse/acquire/update and package
   provenance to the same library and composition model.
5. **Safe publishing:** add GUI review and confirmation for Workshop items and
   SteamPipe root publishing, with previews and exact targets.
6. **Full authoring IDE:** deepen workspace editing, scripting, manifest/content
   creation, documentation discovery, and game-side workbench handoff until the
   GUI can replace the routine RustRover content-authoring path.

Keep the first playable milestone inside the full architecture: one source
workspace, one local Packagepack, one resolved/locked composition, one local
install/run profile, and Steam launching Studio/direct Play. Do not substitute a
mock catalog or a separate hardcoded Loo-Cast path for the real content graph.

## Decisions and open details

**Agreed:** Studio is the default Steam launch; direct Play is an optional
entrypoint for the selected composition. The GUI owns composition and publish
workflows; the CLI remains recovery tooling. Steam root publishing and Workshop
publishing are separate workflows with explicit safety review.

**To resolve during implementation discovery:** exact process/IPC form of the
launch request, the local app-instance handoff mechanism, which existing
Launcher/Shell APIs become the operation service, and the final user-facing
names. These are implementation choices; they must preserve the invariants
above and not block the initial local composition loop.

## Scope and operational safeguards

The product goal includes real Steam root and Workshop publishing, authoring
and deploying Loo-Cast, launching compositions through Steam, and integrating
the content workflow with the USF and scripting work. These are not future
non-goals or forbidden capabilities.

Execution remains deliberate: publish screens preview the actual staged
operation and require an explicit final confirmation; local deploy and launch
show which workspace, profile, and composition they affect. During the current
coding pass, builds and tests may run only through Vapor's managed commands in
the owning workspace. Raw Cargo commands are not used. Deployments, Steam
launches, external publishes, and commits remain deferred until explicitly
requested. These are work-session constraints, not product-design limits.

USF and scripting work is in scope where needed to make content authoring and
composition work coherently. It does not silently authorize unrelated rewrites
of those systems; such changes should be justified by an actual integration
boundary or root cause.
