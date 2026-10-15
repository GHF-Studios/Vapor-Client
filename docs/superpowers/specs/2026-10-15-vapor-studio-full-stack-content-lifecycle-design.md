# Vapor Studio Full-Stack Content Lifecycle

**Status:** owner-approved design
**Date:** 2026-10-15

## Goal

Make Vapor Studio the normal GUI for the complete player and creator lifecycle: discover content, install and compose it, create from templates, edit and validate it, run it locally, publish to Steam Workshop, and keep installed content current. Studio is the product surface; Vapor supplies the operation and safety machinery beneath it.

## Product objects

- **Content release:** a versioned artifact with stable identity, kind, source/provenance, compatibility, dependencies, and publication state.
- **Composition/profile:** a user's launchable selection for an experience: a root Packagepack, profile settings, and a resolved versioned graph.
- **Installation:** the managed local realization of a composition's resolved graph, with download, verification, update, and repair state.
- **Workspace:** editable source for one or more authored content releases. A workspace can produce an artifact; an installation is not itself source.

These are distinct concepts and must be visibly linked. A locally authored graph node requires its constituent/descendant content to be locally authored too. Content and pack kinds remain Rust-defined; templates create instances of supported kinds rather than inventing new kinds.

## Application structure

A persistent, restrained shell provides location/context, global search, and operation status. Primary destinations:

1. **Home:** resume a composition or workspace; surface recent activity and actionable update/download/failure state.
2. **Discover:** one searchable catalog across enabled sources, including Steam Workshop, first-party sources, and local content. Browse filters by kind and compatibility. A detail view shows provenance, versions, dependencies, compatibility, and install targets.
3. **Library:** compositions and their installation state. A selected composition exposes its resolved graph, updates, settings, Play, add-content, repair, duplicate, and related source/workspace actions.
4. **Create:** templates for engine-supported content and pack kinds. Creation asks only for required identity/name/location choices, then opens the resulting workspace.
5. **Workspace:** contextual authoring shell with a project/content tree, central editor or preview, inspector, and collapsible operations/problems/logs panel. Validation, local run, and publishing act on the selected content and show their exact targets.
6. **Workshop integration:** browse and subscription/update state lives with discovery; a content item's owned upload, version, and publication state is available from its detail/workspace context. Publishing is an explicit review of staged content followed by a clearly named confirmation.

Diagnostics remain available as a tool surface without defining the top-level product navigation. Panels are contextual and hideable; launcher tasks do not inherit the full editor layout.

## End-to-end flows

**Consume:** Discover item → inspect detail → install into a new or existing composition → show resolve/download/install progress and recoverable errors → manage in Library → Play.

**Create and share:** choose template → open workspace → edit/compose → validate → run locally → review the actual staged Workshop artifact and dependencies → explicitly publish → expose the published version in Studio.

Both flows use the same stable content identity, provenance, compatibility, dependency, and operation-result contracts. The UI does not implement resolution, installation, or publishing itself.

## Ownership boundaries

- **Studio UI:** navigation, task flow, presentation, progress, and explicit user confirmation.
- **Vapor core:** identity, local discovery, compatibility/resolution, workspace/build/run operations, and typed operation requests/results.
- **Vapor CLI / Steam providers:** managed acquisition, subscriptions, install/update/repair and Workshop operations through controlled providers.
- **Vapor publishing services:** validate and stage the actual artifact; Studio presents review and confirmation, never a form-only preview.
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
