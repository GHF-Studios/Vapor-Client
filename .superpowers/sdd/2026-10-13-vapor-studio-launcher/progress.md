# SDD ledger — plan: docs/superpowers/plans/2026-10-13-vapor-studio-launcher.md

Pre-flight: execution is on the existing owner-directed `main` checkout, not a fresh worktree; the user explicitly asked to build on the current dirty workspace and defer commits. Existing authored changes are preserved. No independent subagent work requested or used.
Pre-flight shared interface: Task 1 Core operation request/events -> Tasks 2/4 Studio and workbench adapters; keep dispatcher in Core and transport thin.
Pre-flight shared interface: Task 3 Shell entrypoint and Launcher launch request -> Task 4 local composition profile; routing validated through explicit profile/composition identity.
Pre-flight shared interface: Task 5 Shell content providers -> Task 4 library/composition; provider owns acquisition and provenance.
Pre-flight shared interface: Task 6 SDK publish preview/execute -> Studio publish review; preview is read-only, execution requires explicit GUI confirmation.
Ruling: Use existing Vapor-Core operation APIs as the immediate shared operation implementation instead of introducing the plan's assumed `operations/**` framework wholesale — live code already centralizes the real managed operations and the CLI; add only missing typed envelopes and adapters — cost if wrong: APIs may need a later focused extraction.

## Implemented in this pass

- Core: restored the shared content source/kind/child vocabulary; made explicit-root workspace operations callable by Studio; centralized cargo realization outputs and pinned toolchain selection; fixed `vapor fmt` argument selection (`cargo fmt --all`, not unsupported `--workspace`); added `vapor studio`; added `vapor-studio` to distribution binary inputs.
- Launcher: added a standalone eframe `vapor-studio` application with Library, Compose, Workspace, Workshop, Publish, and Diagnostics sections; retained the Devtools diagnostics implementation as an embeddable library; added local discovery, graph resolve, managed build/run, workspace operations, persisted profiles, and typed local Packagepack creation with exact-version dependencies, manifest validation, safe output naming, and no-overwrite behavior. Publish now calls the SDK's read-only real Steam root package planner and displays the reviewed input counts and output location.
- Follow-up integration: replaced the hardcoded Loo-Cast workspace label with a picker backed by Vapor's registered source roots; profiles now retain their own workspace root as well as selected content. The Workshop view now describes the actual legacy provider boundary instead of implying it is already wired into Studio.
- Steam launch follow-up: the entrypoint locates `vapor-studio` beside itself (so the GNU Windows target works), Steam depot smoke checks now require Studio for both supported platforms, and release docs specify Studio as the default `studio` launch option. The `play` option now starts Studio's selected-profile build/run path; Shell and Installer remain recovery routes. Steamworks itself has not been changed.
- Entrypoint: no-argument Steam entry now starts Studio directly; explicit `studio`, `play`, and `shell` routes preserve argument behavior and legacy terminal paths. Three focused route tests cover the dispatch.
- SDK/Shell workspace metadata: completed missing pinned toolchain/project declarations so Vapor can resolve each as a real managed workspace. Reconciled SDK toolchain status with Core's `ManagedToolchain`; added its missing `Library` content-manifest case.
- No deploy, Steam launch, game launch, publication, or commit was performed.

## Verification

- `vapor fmt` passed for Core, Launcher, Entrypoint, and SDK.
- `vapor test` passed for Core (26 tests), Launcher (compiles all packages; no tests currently defined), Entrypoint (3 route tests), Shell (compiles; no tests currently defined), and SDK (compiles; no tests currently defined). These were run through Vapor's pinned workspaces/cache. Launcher was rechecked after adding the SDK planner dependency.
- `vapor client build` passed for the Core Vapor Client workspace. It does not prove the distribution bundle contains the new Studio executable; deployment was intentionally not invoked.
- Follow-up verification: after the registered-workspace picker/profile changes, `vapor fmt` and `vapor test` passed again in `Vapor-Launcher`; all launcher packages compiled and the workspace currently defines zero automated tests. `git diff --check` passed.
- Steam launch verification: `vapor fmt` and `vapor test` passed for Entrypoint (5 tests) and Shell (compiles; no automated tests defined); Launcher compiled after Play-mode startup changes. `vapor client build` passed for Vapor's own workspace. This did not stage or deploy the Steam depot.
- Existing warnings remain in Core Tracy/cargo, Devtools snapshot/deprecation, and Studio panel deprecation.
- Runtime UI behavior, Steam launch under Pressure Vessel/Windows, content creation through clicks, and deployment packaging remain unverified.
