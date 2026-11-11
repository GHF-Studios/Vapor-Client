# Alpha v0.5 Quest Journey Vertical Slice

## Goal

Make Alpha v0.5 progress visible through real Player and Developer workflows,
with one shared journey model used by the GUI and CLI.

## Design

`vapor_client_application` owns a static journey catalog and a small local
progress document under `Vapor Home/data/state/journeys/`. Quest records use
stable IDs, prerequisites, points, and completion evidence. The application
layer owns eligibility and staged-file persistence; frontends render the same
snapshot. Existing Client operations remain the sole implementation of
install, launch, workspace, and tool actions.

The first Player path covers inspecting the library, installing a Packagepack,
and launching it. The first Developer path covers observing Developer tool
readiness and completing the Wheel Salvage Demo through existing workspace
operations. Tasks with no reliable machine-observable outcome require an
explicit confirmation and are visibly identified as such.

## Sequence

1. Add the shared catalog, progress schema, eligibility calculation, observed
   operation completion, explicit confirmation, and atomic read/write API.
2. Add a Launcher Journeys page that renders catalog/progress, routes the user
   to the relevant existing Play/Library action, and updates from actual Client
   operation events and role state.
3. Add `vapor journey list|status|confirm` using the same application API;
   do not duplicate install/build/launch operations in CLI.
4. Repair cold-checkout build readiness: GUI/CLI lockfiles were ignored although
   managed Cargo operations require `--locked`. Track each independent Cargo
   workspace lockfile rather than silently regenerating dependency resolution.
5. Build the Client GUI and CLI with Vapor-managed tools for Windows and Linux.
6. Inspect the diff against the charter, verify progress persists under Vapor
   Home and is within the full-erasure boundary, then commit/push.

## Evidence rules

- Operation acceptance is not completion; only matching successful terminal
  events count.
- A stored completion is keyed by stable quest ID. Catalog changes must not
  silently award newly introduced quests.
- Repeated success is idempotent and does not duplicate points.
- Explicit confirmation stores that it was user-confirmed rather than
  disguising it as observed system state.
- No quest completion grants access, license, role, publishing rights, or
  administrative authority.

## Out of scope for this slice

Transparent desktop overlay, cloud progress sync, leaderboards, broad tutorial
authoring tools, new engine/game features, and Steam release publishing.

## Validation

Do not add or run tests in this phase. Use the Vapor-managed Windows and Linux
builds as compile validation, inspect the resulting operation wiring and
progress path, and leave visible end-to-end GUI review to the owner.
