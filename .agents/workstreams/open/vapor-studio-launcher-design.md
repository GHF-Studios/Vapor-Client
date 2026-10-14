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

- `Vapor-Entrypoint` and `Vapor-Shell` already provide Steam-facing Play, Shell,
  and Installer entrypoints; the root Steam AppID is `2122620`.
- `vapor client deploy local` performs a source-authoritative local deploy of
  the Vapor application. `vapor client deploy steam` is a different operation
  that stages and publishes the root through SteamPipe.
- Vapor core already discovers local content, resolves a Packagepack into a
  composition, generates a Cargo realization, builds it, and runs it.
- Loo-Cast has a pinned `Workspace.vapor.toml` at
  `/home/leslieghf/Vapor/Loo-Cast`. Vapor already exposes `fmt`, `check`,
  `test`, `build`, `clean`, and `run` against the workspace discovered from the
  current directory. These commands use the managed toolchain and
  Vapor-managed caches.
- Vapor-Shell has app-root content acquisition/install, Workshop, and root
  distribution workflows. These are distinct authorities and storage domains.
- `Vapor-Launcher` already owns install/select/validate/compose/launch in its
  product boundary, but its current Rust crates are command-model stubs. It
  depends on Vapor core; Vapor core must not depend on Launcher.
- `vapor-devtools` is an eframe diagnostics UI. Loo-Cast's workbench is an
  in-game Bevy/egui editor. A JSON-lines `vapor-ide-host` prototype currently
  provides local content discovery and resolution, but is not yet included in
  the deployed Vapor runtime or connected to the full operation lifecycle.
- No single GUI currently owns the full player/creator flow. Existing CLI,
  Steam, Workshop, content, diagnostics, and in-game editing surfaces remain
  fragmented.
- The current Studio shell now has a local composition loop, managed workspace
  operations, per-profile workspace selection, and read-only Steam root package
  preview. Its Workshop view is intentionally not wired to the legacy Shell
  catalog: Shell uses role-specific manifests and app-root cache state, while
  Studio's local resolver uses generic `Vapor.toml` identities. Connecting them
  requires an explicit shared identity/provenance and install adapter; matching
  names alone is not enough.
- Steam's default launch option must pass `studio` to `vapor-entrypoint`; the
  `play` argument opens sibling Studio with `--play` to run the selected
  composition. Shell and Installer remain terminal recovery modes. Leslie
  reports that the Steamworks launch options now match this matrix; the depot
  has not been deployed or verified here.

## Ownership and data flow

```text
Steam entrypoint
      │
      ▼
Vapor Studio GUI ─── selected profile/composition ───► Launcher operations
      │                                                   │
      ├── authoring/editor                                  ├── Vapor core:
      │       │                                             │   identity, discovery,
      │       └── in-game workbench ◄── operation host ─────┤   resolution, build/run
      │                                                     └── Vapor-Shell:
      ├── publish review ───► Steam/Workshop providers          acquire/install/publish
      └── diagnostics
```

- **Vapor core** owns stable content identity, local source discovery,
  composition resolution, build/run operations, and typed operation requests.
- **Vapor-Shell** owns app-root state, remote acquisition and installation,
  Steam/Workshop providers, root distribution staging, and platform entrypoint
  packaging.
- **Vapor-Launcher** owns player/creator workflows: installed library,
  composition selection, composition authoring, launch intent, and GUI-facing
  operation coordination. It consumes core and Shell APIs; lower layers do not
  depend on it.
- **Vapor SDK** owns authoring, validation, publishing, and distribution
  workflows. Studio may present and confirm publishing, but it delegates the
  operation to SDK-owned workflows/providers; Launcher does not become the
  publisher.
- **Vapor Studio** owns presentation, navigation, progress, and confirmation.
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

The CLI and GUI are adapters over one typed operation service. An operation
receives an explicit workspace/root and request (for example `BuildWorkspace`,
`TestWorkspace`, or `RunConfiguration("default")`), resolves the workspace's
toolchain pin, and uses Vapor-owned Cargo/Rustup/cache locations. The CLI
discovers the root from its working directory; Studio passes the selected root
explicitly. Both paths call the same handler and report the same result.

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

1. **Operation contract:** finish the structured host, ship it with the runtime,
   and expose discovery, resolve, format/check/test/build/clean/run, local
   deploy, acquire, publish-preview, and launch requests through existing Vapor
   implementations. The CLI and GUI must share handlers and managed state.
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
