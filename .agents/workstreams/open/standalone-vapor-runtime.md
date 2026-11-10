# Workstream — Standalone Vapor and Packagepack-Only Steam Mount

- State: foreground
- Goal: finish Vapor's independent install/update/uninstall lifecycle, move all
  Vapor-owned files out of the Loo Cast Steam app, and leave only
  `Packagepack.vapor.toml`, one primary executable per platform, and required
  runtime sidecars in the Steam depots. Apply the user's `unslop` UI method to
  the public site,
  admin, and local Vapor interface without losing their product identity.
- Semantic owners: Vapor Client owns standalone home, local lifecycle, and
  product discovery; Loo Cast owns its Packagepack and Steam allowlist;
  Platform Server coordinates service deployment and admin shell; each service
  owns its own business logic. Mad Max adapter work remains in MM-Building-System.
- Current decisions: Steam is not the Vapor installer. Steam launches the Loo
  Cast Packagepack directly. The common depot contains the Packagepack
  manifest; Linux and Windows each receive one matching primary executable
  plus explicitly required runtime sidecars. “One executable” is the product
  entry-point count, not a one-file restriction.
  No App manifest, Vapor runtime, locator, loose Engine/Game folders, or
  writable Vapor state belongs in Steam. Developer setup is mandatory and has
  the supported cross-target tools; local roles are Player and Developer.
- Design references: the current coordination contract is in
  `docs/superpowers/specs/2026-11-10-vapor-standalone-and-operations-realignment.md`
  and its implementation plan. The 2026-11-03 Client design/plan are historical
  context and must not override the newer payload decision.
- Mutating paths: Vapor Client's existing dirty work under `Vapor/`,
  `Vapor-CLI/`, `Vapor-Installer/`, `Vapor-Steam-Entrypoint/`, `Vapor-Studio/`,
  and `resources/vapor/tools/**`; root Client docs/manifests; Loo Cast
  `README.md`, `steam-mount/App-Source.vapor.toml`, and product manifests on
  its existing local branch. Preserve pre-existing staged and unstaged work.
- Implemented this pass: the Steam source allowlist now stages only the
  Packagepack manifest and the correct target executable; the Client installer
  recognizes that Packagepack layout and migrates the former three-product layout
  as legacy data; the migration no longer leaves a Steam locator behind. Added
  positive/negative packagepack-mount tests. Updated the current CLI setup and
  discovery docs plus the retired entrypoint README. Added the `unslop` compact
  CSS pass to Platform admin while preserving the current palette. Full-nuke
  preview now includes the known legacy Vapor directories and `App.vapor.toml`
  inside the standard Loo Cast Steam library on Windows/Linux, while excluding
  the Packagepack manifest and executable. Preview/removal accepts exact files
  as well as directories, and the confirmation copy names the retained product.
  The nuke inventory now reads Steam's `libraryfolders.vdf` so custom Steam
  libraries are included in the same exact-path preview. A read-only machine
  inventory confirmed the current Loo Cast Steam root still contains the old
  Vapor directories and `App.vapor.toml`; it has not been modified.
- Current clarification: each OS gets one primary Steam executable entry
  point, with required DLLs or other declared runtime sidecars allowed. This is
  not a one-file restriction. The Packagepack manifest belongs in the shared
  Steam depot; `App.vapor.toml` and the Vapor runtime do not.
- This pass repaired Windows fixture path containment so long and 8.3 user
  profile paths compare through their nearest existing canonical ancestor. The
  check still rejects paths that resolve outside the temporary fixture root.
  The legacy import/restart process test now uses production mode while only
  exercising Player-safe selection, so it tests the real importer without
  requiring Developer installation state.
- Verification: full Vapor Client workspace `cargo test --workspace --locked`
  passes; the Installer workspace passes all 8 UI tests; the docs publishing
  suite passes all 10 tests. The service emits one existing Rust deprecation
  warning (`AtomicUsize::fetch_update`). No Steam staging, local file migration,
  install, publish, or deployment has run.
- Added a read-only `vapor storage` command for the canonical Vapor home only;
  it reports top-level category sizes and cleanup priority without scanning
  legacy paths or deleting files. It has not been built in this pass. A requested
  bulk deletion of generated Cargo targets, old output, and duplicate caches was
  rejected by automatic review before any deletion occurred.
- Open: inspect exact Steam staging output after building Packagepack for both
  targets; verify required runtime dependencies are explicitly staged
  (especially Windows GNullVM/libunwind); audit additional historical Vapor
  roots; complete migration fixtures
  including a legacy three-root install and transactional rollback; wire the
  standalone Vapor UI and source
  resolver to identify the Steam Packagepack mount without resurrecting an
  App-root or locator contract; complete the explicit full-nuke preview and
  known-path inventory; audit/delete only confirmed obsolete local artifacts;
  finish the admin and Studio `unslop` passes; build both supported targets.
- Never run the local migration or full-nuke operation automatically. Review
  its exact path list first. No repository cleanup may overwrite or discard
  existing user changes.
