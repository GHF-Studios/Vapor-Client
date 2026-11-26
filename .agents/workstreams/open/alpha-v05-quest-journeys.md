# Workstream — Alpha v0.5 Quest Journeys

- State: foreground
- Goal: deliver shared, evidence-backed Player and Developer journeys that
  guide real Vapor, Spacetime Engine, and Loo Cast workflows through the
  Launcher and CLI.
- Ordered successor: resume the visible clean-reinstall GUI walkthrough, then
  expand integrated demos from the first working journey slice.
- Depends on: owner-approved Alpha v0.5 charter and current Client service
  operation protocol.
- Semantic mutation claim: shared journey/quest identities, prerequisites,
  rewards, evidence, local progress persistence, and GUI/CLI presentation.
- Path claims: `Vapor/crates/vapor_client_application/**`,
  `Vapor/crates/vapor_client_provider/src/workspace_build.rs`,
  `Vapor/crates/vapor_core/src/installation.rs`,
  `Vapor-GUI/crates/vapor_launcher_ui/**`, `Vapor-GUI/.gitignore`,
  `Vapor-GUI/Cargo.lock`, `Vapor-CLI/crates/vapor_cli/**`,
  `Vapor-CLI/.gitignore`, `Vapor-CLI/Cargo.lock`,
  `Vapor-Examples/Workspace.vapor.toml`,
  `docs/superpowers/specs/2026-11-10-alpha-v0-5-phase-charter-design.md`,
  `docs/superpowers/plans/2026-11-11-alpha-v0-5-quest-vertical-slice.md`,
  `.agents/workstreams/README.md`, and this record.
- Decisions: stable IDs; prerequisite graph supports concurrent quests; points
  reward progress but grant no authority; persist beneath Vapor Home state;
  complete observable quests only from successful operation or state evidence;
  use explicit confirmation only for inherently unobservable learning steps.
- Completed: shared journey catalog/progress API; Player install/launch and
  Developer readiness/demo quests; Launcher page; CLI status/confirm commands;
  Wheel Salvage run configuration; tracked GUI/CLI lockfiles for deterministic
  cold builds; docs.
- Verification: Vapor, Vapor GUI, and Vapor CLI Windows development builds
  passed through Vapor Shell. No tests were run. Linux-native build remains a
  CI/Ubuntu-worker validation item; this Windows machine only built its host
  target. The deployed CLI `journey status` command ran and showed the shared
  Player/Developer catalog and progress.
- Deployment: all quest sources and lockfiles were committed/pushed to their
  submodule branches and pinned by the Client branch. Vapor's local deploy
  completed after invoking its staged Vapor-home deployment helper with no
  live `vapor.exe` parent. The normal self-deploy path still fails on Windows
  because its running parent holds `bin/<target>/vapor.exe` open; this needs a
  proper exit-and-resume update handoff. The detailed child error is now
  surfaced by source, but the executable lock is the remaining defect.
- Owned uncommitted changes: none intended; the current checkpoint should be
  committed after this status update.
- Remaining: design/implement the Windows self-update handoff; run the full
  empty-install recovery drill; validate the Linux-native build on the Ubuntu
  worker; have the owner visually review the installed Launcher page.
- Smallest resume action: make `client deploy local` on Windows release the
  current executable before promotion, then repeat the install flow.
- Pending external actions: owner must visually inspect the installed
  Launcher flow; no other authorization is pending.
