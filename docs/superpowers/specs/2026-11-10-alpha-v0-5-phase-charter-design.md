# Alpha v0.5 Phase Charter and Tutorial Game Design

**Status:** Proposed for owner approval. This records the current PRE-ALPHA
operating contract and the next milestone; it does not authorize implementation
of the tutorial runtime before the design is approved.

## Intent

Make PRE-ALPHA produce visible, integrated progress toward a Vapor-led
development and play experience on Windows and Linux. Establish a repeatable
clean reinstall/recovery path first. Then onboard players and developers
through interactive, quest-based tutorials that teach real Vapor, Spacetime
Engine, and Loo Cast workflows while making Alpha v0.5 progress legible.

The tutorials are a user-facing journey and progress view. They can serve the
organizing purpose of milestones without becoming a second engineering issue
tracker or replacing GitHub's ownership and dependency records.

## Current evidence

- The owner-provided Active Development Policy is recorded in
  `.agents/active-development-policy.md`.
- Vapor-GUI currently uses eframe/egui (`eframe = 0.36.2`). Its viewport API
  supports transparent and always-on-top windows. The underlying winit window
  level reports Wayland as unsupported, so the tutorial must remain usable in a
  regular Vapor window there.
- The current GitHub issue stack is not a reliable phase plan by itself: many
  unrelated Vapor and Loo Cast issues are labeled ACTIVE, Vapor-Client has no
  ACTIVE issues, and Loo Cast #57's body says its broader program is QUEUED
  despite the ACTIVE label. This charter narrows focus without rewriting issue
  states automatically.
- Existing installer and privileged-operation design documents already
  establish the clean-rebuild direction, Player/Developer roles, the three
  profiles (`dev`, `test`, `release`), the two Vapor runtime targets, and
  operation-scoped native elevation rather than a persistent privileged
  service. Those decisions remain inputs, subject to current source evidence.

## Phase objective

### PRE-ALPHA exit foundation

Deliver one clear, GUI-led lifecycle for a clean Vapor installation and
reconstruction on the supported Windows and Linux environments. It must make
the ownership boundary and destructive scope visible, let the user deliberately
erase Vapor-owned state, reinstall the selected role, provision the Developer
toolchain when selected, and show verified outcomes. The workflow must not
silently include Steam-owned game content or unrelated tools/data in Vapor's
erase boundary.

The normal Player path remains unprivileged. Developer prerequisites use the
previously approved one-operation OS authorization flow only when a concrete
host prerequisite needs it. Vapor-managed toolchains and target sysroots remain
app-local.

### Alpha v0.5 milestone

Deliver integrated demonstrations of Vapor, Spacetime Engine, and Loo Cast as
guided player and developer journeys. The exact demonstration set is an
implementation decision to finalize from the live capabilities and the first
working vertical slice; this charter does not pretend every open issue is part
of Alpha 0.5.

The agreed Vapor build contract remains three profiles (`dev`, `test`,
`release`) crossed with the two supported Vapor runtime targets. Release
artifacts and publishing remain CI-owned. Target-specific product ABI flags do
not create extra Vapor-wide profiles or target rows.

## Tutorial game

The tutorial is a lightweight meta-game built from real tasks, not a separate
simulation or a collection of static help pages.

- A **journey** groups quests for a purpose such as first-time Player setup or
  Developer setup and first demo.
- A **quest** has a stable identity, a human-readable objective, prerequisite
  quest IDs, an actionable operation or observable completion condition, and
  optional reward/progress presentation.
- Prerequisites form a directed graph. Independent quests may become active at
  the same time; dependent quests unlock only when their prerequisites are
  complete. Cycles or missing prerequisites are invalid content.
- Points, badges, and progress feedback reward exploration. They never grant
  product access, licensing, administrative privilege, or permission to publish.
- Quest actions call the same typed domain operations used by ordinary Vapor
  GUI and CLI surfaces. The tutorial does not implement a second copy of setup,
  install, build, launch, or recovery behavior.
- Completion is based on state the shared core can observe where possible. If a
  step is inherently explanatory or external, it may ask the user to confirm;
  it must not claim an operation succeeded without evidence.
- Users may pause, skip, or hide optional tutorials. Essential concepts may be
  offered again when a later action depends on them or the workflow materially
  changes; avoid arbitrary nagging and respect an explicit “do not show again”
  choice until that relevant trigger occurs.
- Progress is local to the Vapor user installation for this phase. Cloud sync,
  social competition, leaderboards, and account-wide achievements are not
  prerequisites for Alpha 0.5.
- Ordinary repair and update preserve quest progress. The explicitly selected
  full-erasure operation clears it with the rest of Vapor-owned user data.

## Presentation and accessibility

Use the existing eframe/egui stack. Provide a normal, focused tutorial view as
the dependable presentation. Add a borderless transparent always-on-top coach
overlay where the host desktop can support it. On Linux Wayland, or whenever
the compositor/game display mode cannot provide the overlay reliably, use the
normal Vapor tutorial window without losing quest state or functionality.

The tutorial must not require exclusive fullscreen, steal input from the target
application, or leave an uncloseable topmost surface. Essential controls remain
reachable from the normal Vapor window.

## Shared GUI/CLI core and future automation

GUI and CLI are two interfaces to one operation/core layer. They share typed
requests, outcomes, progress/evidence, and error semantics; neither interface
reimplements the operation. Tutorial quests consume those same operations.

This gives a future CLI-driven journey checker or automated test harness a
stable place to operate. A broad automated test suite is not a gate for this
phase. Use manual end-to-end validation for the clean reinstall path and the
integrated demo journeys, plus narrow automated checks only where destructive
scope, quest graph validity, or operation safety justifies them.

## Operating contract

- Optimize for meaningful progress per unit time and usage.
- Keep one milestone journey deliberately ACTIVE; bring in direct blockers,
  not every related idea. Preserve other work as QUEUED or PARKED rather than
  letting labels imply simultaneous execution.
- Make substantial, cohesive changes when they remove a real bottleneck. Commit
  and push meaningful checkpoints frequently. Deploy or publish in explicitly
  authorized development environments.
- Do not add mandatory PRs, compatibility layers, exhaustive tests, or process
  ceremony without a concrete benefit.
- Preserve unrelated work, credentials, irreplaceable data, and external
  resources. Scale validation to the consequences of the operation.
- Advance to Alpha only by explicit owner decision; a version string alone does
  not change the phase.

## Approaches considered

1. **Quest-led phase — recommended and reflected here.** Complete the clean
   reinstall journey, then use quests to guide real Player and Developer demo
   paths. The tutorial provides a motivating, visible map of progress without
   replacing GitHub issue ownership. This best matches the owner's request for
   tutorials to serve both onboarding and organization.
2. **Demo-first, tutorial-later.** Build integrated demos, then add a guide.
   This delays tutorial infrastructure but risks treating onboarding and
   project progress as separate products.
3. **Gamify every surface immediately.** Add points and quest UI to setup,
   administration, development, and runtime in one broad pass. This has the
   widest early reach but recreates the sprawling scope PRE-ALPHA is intended
   to control.

The owner selected the quest/meta-game direction, required clean reinstall and
GUI paths first, requested concurrent as well as sequential quests, accepted a
Wayland windowed fallback, and endorsed a shared GUI/CLI core as the future
automation boundary.

## Non-goals

- Replacing GitHub issues, dependencies, or repository ownership with quest
  state.
- Requiring users to earn points to access normal product functionality.
- Requiring a transparent global overlay on Linux Wayland or inside exclusive
  fullscreen applications.
- Building a complete social rewards economy, leaderboard, or cloud-sync
  service before Alpha 0.5.
- Finishing every existing ACTIVE issue before the first integrated demo.
- Treating the existence of a tutorial step as proof that the underlying
  capability works.

## Open decisions for the implementation plan

1. Which small set of concrete engine/game capabilities makes the Alpha 0.5
   demo suite feel comprehensive without turning the milestone into the entire
   backlog?
2. What exact clean environments are available for the Windows and Linux
   reinstall demonstrations, and which paths/data must the full-erasure UI
   enumerate before deletion?
3. Which tutorial progress is observable from current operation results, and
   which explanatory steps need explicit user confirmation?

These are implementation-plan decisions unless live source inspection shows
that one changes the architecture above.
