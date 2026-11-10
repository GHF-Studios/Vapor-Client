# Workstream — Mad Max on Vapor

- State: foreground
- Goal: deliver a standalone GUI-first Vapor distribution that manages a
  player-owned Mad Max installation and verified mod packages without Loo-Cast.
- Ordered successor: none until the client/core slice and first package reach a
  reviewable checkpoint.
- Depends on: owner-approved design in
  `MM-Building-System/docs/superpowers/specs/2026-11-01-vapor-mad-max-standalone-ecosystem-design.md`;
  canonical source identity and prerequisite/bootstrap contracts in the active
  Vapor issues must be consumed rather than duplicated.
- Semantic mutation claim: external installed-game targets, provider-neutral
  release acquisition, standalone Launcher distribution, one-process catalog
  hosting, and the first reversible Mad Max runtime package.
- Path claims: `Vapor/**`, `Vapor-GUI/**`, `Vapor-Installer/**`,
  `C:\Users\David Jackson\Documents\GitHub\Vapor-Platform-Server-madmax\**`,
  `.agents/workstreams/README.md`,
  `.agents/workstreams/open/madmax-vapor-ecosystem.md`, and the three
  Mad Max/Vapor execution plans in `../MM-Building-System/docs/superpowers/plans/`
  plus `../MM-Building-System/docs/superpowers/specs/2026-11-01-vapor-mad-max-standalone-ecosystem-design.md`.
- Mad Max's standalone Vapor install, update, launch, and removal path must not
  depend on Loo-Cast or its Steam App ID. The Loo-Cast Steam app is a
  content-only mount for the Spacetime Engine, Loo-Cast game, Packagepack, and
  composition manifest. Shared product data goes in the common depot; binary
  payloads are split into ABI-specific depots and selected by target triple.
  They are not Workshop items or Artifact Server releases; server-owned policy,
  not client manifests, marks them local-only and non-publishable.
- Vapor itself is independently installed under the user's Vapor home. No
  Vapor executable, entrypoint, tool, cache, source, or writable state belongs
  in the Steam product root. Steam uninstall does not remove Vapor. A direct
  Steam launch option into Vapor's tools selector still needs an external
  integration mechanism. The old Steam app-root layout is replaced in
  pre-alpha rather than kept as a compatibility path.
- The owner selected one monolithic server process on one VPS. Preserve separate
  service-domain modules, authorization boundaries, and state ownership inside
  that process. This explicitly supersedes the current multi-process deployment
  target for this workstream.
- Build and test through Vapor-managed commands at the owning workspace root;
  do not invoke raw Cargo. Deployment, publication, and live VPS migration
  remain explicit owner-controlled operations.

## Completed

- Approved the standalone Vapor/Mad Max design and produced an umbrella plan
  plus separate client/core, server, and Mad Max package plans.
- Initialized the Vapor-Client superworkspace submodules in the isolated
  worktree; the user's `main` checkout remains unchanged with its submodules
  uninitialized.
- Created independent feature clones with pinned submodules:
  `C:\Users\David Jackson\Documents\GitHub\Vapor-Client-madmax` has a root
  feature branch plus feature branches for `Vapor`, `Vapor-GUI`, and
  `Vapor-Installer`; `C:\Users\David Jackson\Documents\GitHub\Vapor-Platform-Server-madmax`
  has a root feature branch plus feature branches for the five Axum service
  submodules. The original checkouts remain clean on `main`.
- Confirmed the provider-neutral `VaporId`/`ContentReference` contract is under
  open `Vapor#2` (ACTIVE), and blank-machine prerequisite policy is under open
  `Vapor#4` (QUEUED). Client issues `#1`–`#3` remain BLOCKED by those contracts.
  No temporary identity or bootstrap format has been introduced.
- Confirmed all five server HTTP services are currently binary-only Axum/Tokio
  crates with route setup in `src/main.rs`; the host integration needs explicit
  router/library exports. Caddy currently serves Atlas files directly, so the
  one-process host plan now routes Atlas static files through the host too.
- Created an isolated Vapor-Client worktree on
  `codex/madmax-standalone-ecosystem`.
- Suspended the prior Loo-Cast-centered full-stack authoring workstream and
  transferred only the overlapping Vapor/core/GUI/installer/workstream-registry
  claims to this workstream. Its remaining claims are preserved.
- Reduced local installation roles to Player and Developer. Developer setup is
  mandatory and owns Rust/Rust Analyzer, Linux and Windows Rust targets,
  Vapor-home cross-linkers, and Tracy. Old local role identifiers are rejected;
  no role alias or migration path is retained. Workflow tiers remain a
  separate action-classification concept.
- Implemented the role gate, Developer readiness snapshot, two-target Rust
  provisioning, Vapor-home Zig and LLVM-MinGW installers, and target-specific
  build environment setup across the Vapor and Vapor-Installer submodules.
- Reworked Loo-Cast Steam publication as an ABI-aware product-content mount:
  `root.runtime.targets` is mandatory, each selected target must be declared,
  and Steam staging accepts only manifest-allowlisted product payloads plus
  the composition manifest. Removed the old Steam release-build switch so this
  path cannot build or package Vapor binaries.
- Confirmed the current machine has three distinct locations: the Steam
  Loo-Cast root still contains the live Vapor runtime and mutable directories;
  `%USERPROFILE%\\Vapor` currently contains the source Superworkspace but has
  no `Vapor-Home.toml`; `%LOCALAPPDATA%\\Vapor` contains pre-alpha Rust homes,
  platform state, and SSH access material. The intended install root remains
  `%USERPROFILE%\\Vapor`; the current machine has not yet been migrated. The
  existing `Superworkspace.vapor.toml` there still declares Loo-Cast's Steam
  app root as its `app-root` and Rust tooling location, so the one-time rehome
  must preserve its source checkouts while reconciling that manifest to the
  standalone Vapor home and explicit external product mount.
- Routed managed Workshop downloads, installed add-on content, and generated
  content staging under Vapor home rather than the executable's product root.
  The explicit Loo-Cast product root remains the read-only source for mounted
  depot content.
- Made each standalone release bundle record its selected target triple and
  require a host-ABI match during installation. The supported target set is
  declared in `Vapor-Standalone.toml`; Steam product payloads remain separated
  into their matching ABI depots.
- Corrected the managed source-workspace ownership rule: the canonical
  `Vapor-Home/sources` directory is now an explicitly permitted source area,
  while arbitrary source roots that overlap runtime-owned paths remain
  rejected. Explicit Superworkspace purge recognizes that one managed child;
  ordinary uninstall still preserves it.
- Removed the generated Windows development activation script's old
  `%LOCALAPPDATA%\\Vapor` assumption; it now resolves the managed Rust toolchain
  relative to the canonical standalone Vapor home.

## Owned uncommitted changes

- In the initial Codex-managed Vapor-Client worktree: this record, the
  workstream registry, and the prior workstream's state/path-claim checkpoint.
- In `C:\Users\David Jackson\Documents\GitHub\MM-Building-System`: the
  approved design/execution plans, refined with the pinned client and server
  source findings.
- In MM-Building-System: approved design and execution plans. Existing unrelated
  user changes in that repository are not owned by this workstream.
- In this worktree's `Vapor` and `Vapor-Installer` submodules: two-role local
  setup, mandatory complete Developer environment reconciliation, managed
  cross-tool installation under Vapor user data, and readiness/UI updates.
- In this worktree's source manifests and `Vapor`, `Vapor-CLI`, and
  `Vapor-Studio` submodules: standalone Vapor distribution, content-only
  Loo-Cast Steam payload, ABI-specific depots, and the application-release UI
  and workflow.

## Remaining

- Resume `docs/superpowers/plans/2026-11-01-vapor-mad-max-standalone-ecosystem.md`.
- Earlier, a managed Windows host build compiled the `vapor` project, including
  core, provider, protocol, application, and service crates. The full workspace
  build stopped at `Vapor-GUI`: its `Cargo.lock` is absent and locked mode
  refuses to generate it. The newer ABI-aware Steam packaging changes have not
  been built yet; this checkout currently has no initialized standalone Vapor
  home/managed command available to run them. No tests have been run. Resolve
  the GUI workspace lock and use Vapor-managed validation before claiming the
  complete client stack builds.
- The approved product-boundary design also requires a one-time transfer of
  Vapor-owned tools, sources, caches, docs, and mutable state into the canonical
  Vapor home, followed by removal of only those transferred Vapor-owned paths
  from the Loo-Cast Steam app root. The Steam depot retains only its three
  first-party products and composition manifest, with native payloads split by
  ABI. Each platform-specific payload must match its host ABI; common depots
  are only for data verified to be ABI-neutral. The current `%LOCALAPPDATA%\\Vapor`
  toolchain/state folders also need to be folded into the same home layout,
  preserving platform SSH state without exposing key contents. `%USERPROFILE%\\Vapor`
  is already the active source Superworkspace, so preserve its `sources` tree
  and reconcile the existing workspace manifest rather than replacing that
  directory. Implement this as a one-time, conflict-aware transfer before
  touching the live folders. This checkout has not yet built or
  installed a complete new Vapor distribution, so the live machine layout is
  unchanged. Existing Workshop item removal remains an owner-operated
  migration; this workstream must not delete remote Workshop items.
- Then finish the client implementation boundary around the active identity
  and prerequisite gates. Use Vapor-managed validation only.
- Next: design the one-process server migration around the actual service
  submodules; do not deploy to the VPS without explicit authorization.
- Continue Mad Max Ghidra mapping in parallel, promoting findings only after
  build-scoped validation.

## Smallest resume action

Implement the one-time local rehome flow for the existing Steam-root and
`%LOCALAPPDATA%\\Vapor` state into `%USERPROFILE%\\Vapor`, with file-level
conflict handling and deletion only after the destination is verified. Keep
Steam-owned `content/installed/ghf-studios/loo-cast/*` product trees and
`App.vapor.toml` in place. Do not add an ongoing old-layout fallback.
