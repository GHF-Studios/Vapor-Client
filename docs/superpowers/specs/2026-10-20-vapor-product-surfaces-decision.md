# Vapor Product Surfaces

**Status:** owner-approved direction
**Date:** 2026-10-20

## Decision

Vapor has separate graphical experiences for separate jobs:

| Product | Primary user | Owns the experience |
| --- | --- | --- |
| Vapor Launcher | Player | Discover and subscribe to content, install it into a Composition, update or repair that Composition, and Play. |
| Vapor Studio | Creator | Create and compose content, validate and run it locally, review artifacts, and publish to Workshop. |
| Vapor IDE | Code author | Navigate and edit source workspaces and invoke managed development operations. |

The CLI is a fourth facade for support, automation, and recovery. These are
frontends over one Vapor Client service and one semantic model. They do not own
independent catalogs, identities, dependency resolution, provider sessions,
installation state, or publishing logic.

## Service boundary

The Vapor Client service owns canonical content and workspace reads, typed
operations, progress, cancellation, results, errors, and access to Steam and
managed build providers. The entrypoint/bootstrap path is responsible for
making the local service available. Frontends request operations and render
them for their audience.

Player-facing wording should omit internal cache and ledger mechanics. Studio
and IDE can expose technical detail when it helps creators or developers act.
All surfaces still use the same operation result and recovery path.

## Product contracts

- A Launcher action is phrased as an outcome: Browse, Subscribe, Install,
  Update, Repair, Remove, Play.
- Studio operates on a selected authored artifact or Composition and shows the
  staged Workshop artifact before a real publish.
- IDE operates on an explicit source workspace. It does not silently change
  the player's installed Composition.
- Opening a creator workspace never changes the active player Composition.
- The same stable content identity and provenance follow an artifact from
  discovery through installation, authoring, and publication.
- A failed or unavailable Steam provider does not disable local content,
  workspace editing, or non-Steam operations.

## Migration rule

The current binaries still bundle product modes and some UI operations.
Implement the split as a sequence of working slices over the shared service:
first isolate operation ownership and data contracts, then extract the focused
Launcher surface, then Studio and IDE responsibilities. Avoid copying business
logic into new binaries. Do not change stable Steam entrypoint mappings or
repository boundaries until the root manifest, installed product, and existing
entrypoints are audited together.

## Verification

The product split is complete only when the three frontends can be started
independently, use the same service state, and demonstrate their audience
boundaries. Verify that a Launcher install is visible in Studio/IDE where
appropriate, that opening an IDE workspace leaves player selection unchanged,
and that publishing in Studio updates the same catalog consumed by Launcher.
