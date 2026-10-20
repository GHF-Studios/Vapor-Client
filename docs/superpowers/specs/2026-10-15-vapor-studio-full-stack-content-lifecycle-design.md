# Vapor Studio Full-Stack Content Lifecycle

**Status:** owner-approved design
**Date:** 2026-10-15

## Product boundary revision (owner-approved 2026-10-20)

This revision supersedes the single-GUI assumption below. Vapor has three
separate graphical products:

- **Vapor Launcher** is the deliberately simple player experience: discover,
  subscribe, install into eligible Compositions, update, repair, and Play.
- **Vapor Studio** is the creator experience: create content, compose it,
  inspect and validate artifacts, run locally, and publish to Workshop.
- **Vapor IDE** is the code-authoring experience: open source workspaces,
  navigate and edit code, and invoke managed development operations.

They are separate frontends over one Vapor Client service and one canonical
content/workspace model. CLI is another facade over that service. UI separation
must not create separate catalogs, identities, dependency resolution, install
state, provider sessions, or publishing implementations. The service owns
operations and state; each frontend owns only the workflows and presentation
appropriate to its audience.

This is a product-boundary decision, not a claim that three applications are
already implemented. Current binaries still bundle some of these workflows.
Steam entrypoint aliases, packaging, and the exact IDE tool surface remain
implementation decisions and must be checked against the shipped manifests.

## Goal

Make Vapor's player, creator, and code-authoring experiences coherent frontends over one Vapor-owned content lifecycle: Launcher handles play, Studio handles creation and release, and IDE handles source editing. Vapor supplies the shared operation and safety machinery beneath them.

## Product objects

- **Content release:** a versioned artifact with stable identity, kind, source/provenance, compatibility, dependencies, and publication state.
- **Composition/profile:** a user's launchable selection for an experience: a root Packagepack, profile settings, and a resolved versioned graph.
- **Installation:** the managed local realization of a composition's resolved graph, with download, verification, update, and repair state.
- **Workspace:** editable source for one or more authored content releases. A workspace can produce an artifact; an installation is not itself source.

These are distinct concepts and must be visibly linked. A locally authored graph node requires its constituent/descendant content to be locally authored too. Content and pack kinds remain Rust-defined; templates create instances of supported kinds rather than inventing new kinds.

## Application structure

Each frontend keeps a persistent, restrained shell appropriate to its task; no frontend inherits another audience's navigation by default. Launcher destinations:

1. **Launcher:** Home, Discover, and Library focus on compositions, subscriptions, install/update/repair state, and Play. It has no source editor, Cargo workflow, or creator publishing controls.
2. **Studio:** Create and Compose workflows produce registered source content and playable Packagepacks. Workshop review and publishing live in the context of the authored artifact.
3. **IDE:** source navigation, code editing, diagnostics, and managed developer operations act on explicit workspaces without owning content identities or lifecycle state.
4. **Shared operation center:** each frontend presents progress and errors relevant to its task while consuming typed results from the same local service. Studio and IDE can expose developer details; Launcher translates failures into player-safe explanations.

A single searchable catalog, Composition model, local Library, Workshop provider, and operation service back all three surfaces. Diagnostics remain available as a contextual tool surface. Steam entrypoint mapping and packaging must be reviewed separately against the actual root manifest.

## End-to-end flows

**Consume:** Discover item → inspect detail → install into a new or existing composition → show resolve/download/install progress and recoverable errors → manage in Library → Play.

**Create and share:** choose template → open workspace → edit/compose → validate → run locally → review the actual staged Workshop artifact and dependencies → explicitly publish → expose the published version in Studio.

Both flows use the same stable content identity, provenance, compatibility, dependency, and operation-result contracts. The UI does not implement resolution, installation, or publishing itself.

## Ownership boundaries

- **Launcher UI:** player navigation, composition selection, install/update/repair choices, progress, and concise recovery actions.
- **Studio UI:** creator navigation, templates, composition authoring, validation, local run, artifact review, and explicit publish confirmation.
- **IDE UI:** source navigation/editing and developer-facing diagnostics; it does not implement build or content lifecycle logic.
- **Vapor Client service/core:** canonical identities, discovery, compatibility/resolution, workspace/build/run operations, install state, typed progress/results, and operation authorization.
- **CLI and Steam providers:** additional service facades/providers; no independent content model or parallel lifecycle implementation.
- **Publishing service:** validates and stages the actual artifact; Studio presents review and confirmation, never a form-only preview.
- **In-game workbench:** remains in-process, but shares workspace/content and operation contracts rather than duplicating business logic.

Long operations run off the UI frame loop and report structured progress, completion, cancellation, and actionable failure. Normal GUI workflows use Vapor-managed toolchains, caches, and providers.

## Delivery sequence

1. **Foundation:** reconcile existing identities and provider manifests; define composition/profile, installation, workspace, and release relationships; unify operation requests, progress, and errors.
2. **Player loop:** connect real local and Workshop discovery to profile-targeted acquire/resolve/install, Library state, updates, and Steam launch.
3. **Creator loop:** create supported content from templates, open/edit workspaces, compose dependencies, validate, and run locally.
4. **Sharing loop:** stage and review real Workshop artifacts, confirm publishing, then display ownership/version/update state and feed releases back into discovery/install.
5. **Product refinement:** integrate global search, cross-links, empty/error/recovery states, keyboard/accessibility, responsive panels, and diagnostics.

Each phase must leave a coherent working slice and preserve a clear path through the lifecycle. Verify against the Loo-Cast workspace and the existing Linux/Windows Steam entrypoints; do not claim platform behavior until exercised.

## Settled provider decision

Use the Steamworks client `ISteamUGC` interface for Workshop catalog queries,
current-user subscription state, and subscribe/unsubscribe. Keep local discovery
and authoring available without Steam. Vapor's `vapor_steam` owns the provider
calls and returns owned data; Vapor Shell retains the verified acquisition and
install boundary; Studio owns presentation and user actions. The binding and
Steamworks redistributable are now present. Vapor-GUI stages the redistributable
beside the development build and adds a relative Linux runpath. Final app-depot
staging, Windows runtime verification, subscription-aware updates, and live Steam
behavior remain open. The official interface exposes query requests and
subscribed-item APIs:
[Steamworks ISteamUGC documentation](https://partner.steamgames.com/doc/api/isteamugc?language=english).

## Open design question

Keep **composition/profile** as one launchable user object distinct from its root Packagepack and workspace. Exact labels and whether a profile's settings/overrides require a separate persisted object remain implementation-discovery questions; they must not collapse source, resolved graph, and installed state into one record.
