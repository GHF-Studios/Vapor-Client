# Standalone Vapor Runtime and Product Boundaries

**Status:** Approved by owner
**Date:** 2026-11-03

## Intent

Make Vapor the independently installed runtime and content manager for several
products while keeping each product's acquisition, trust, and runtime lifecycle
explicit. The Loo Cast and Mad Max integrations should share Vapor semantics,
not pretend the games acquire or execute content the same way.

## Owner-approved invariants

- Vapor installs, updates, repairs, and uninstalls independently from Steam.
- The Vapor home (`%USERPROFILE%\\Vapor` or `$HOME/Vapor`) owns the Vapor
  application, managed tools, source checkouts, caches, configuration, and
  mutable state. No Vapor runtime/tool/cache/state directory is owned by the
  Loo Cast Steam app root.
- The Loo Cast depot provides one local first-party content artifact containing
  `App.vapor.toml`, Spacetime Engine, Loo Cast Game, and Loo Cast Packagepack.
  A tiny optional Steam locator may find Vapor and open its tools selector; it
  never bundles, installs, or updates Vapor.
- The only local setup roles are `Player` and `Developer`. Player setup excludes
  developer tooling. Developer setup installs all required development tools,
  including both supported targets and managed cross-compilers, with no skip
  path. Ecosystem developer authorization is server-side Identity state, not a
  local role.
- This is pre-alpha: remove the former Composer and EcosystemDeveloper local
  role model directly. Do not retain compatibility parsing, role migration, or
  alternate old-layout discovery. Preserve user-authored content and move
  Vapor-owned files out of the Steam app root as part of the standalone setup.
- Keep the Rust compiler host distinct from a packaged runtime target. On the
  supported Windows development host, MSVC is the Rust compiler host while
  GNullVM is the Windows runtime target; Linux and Windows outputs remain
  target-specific.
- Mad Max uses the same content vocabulary with a different source adapter:
  user-acquired `MadMax.exe` and required original runtime files are the Engine
  source; mod scripts/content and installed composition are the Game layer;
  `MadMaxModding` is the integration Packagepack that manages ASI-loader and
  bridge setup. The Packagepack contains no Mad Max files. If the loader cannot
  be redistributed, acquire it from an authorized source.
- Mad Max mods may target the Engine, Game, or another mod/Packagepack using
  Vapor's canonical dependency, conflict, and override semantics. Do not add a
  parallel Mad Max manifest or identity schema.
- No service may distribute licensed Loo Cast or Mad Max game files. Product
  publishability is enforced by server authority, not client manifest claims.

## Goals

- Provide one coherent Vapor home with a clear owner for installed binaries,
  tools, sources, derived asset caches, user configuration, and state.
- Make the Steam app root a local content mount, not Vapor's installation.
- Let Player and Developer setup be understandable and deterministic.
- Make the package/runtime boundaries usable by future Loo Cast and Mad Max
  adapters without duplicating product identity, dependency, or authority rules.
- Preserve existing authored source while migrating Vapor-owned installation
  files to the standalone home.

## Non-goals

- Do not implement the Mad Max game adapter in generic Vapor Client code; that
  adapter is owned by MM-Building-System.
- Do not change server-owned Identity, Registry, Artifact, or Build Cache policy
  in this Client plan.
- Do not choose Steam AppID/depot IDs, Workshop retirement IDs, or release
  signing keys by inference.
- Do not keep the old Steam-root or multi-role layout as a supported runtime.

## Approaches considered

1. **Keep Vapor in Steam and gradually move selected state.** Rejected because
   it leaves Steam as the lifecycle owner and preserves split paths.
2. **Create a separate Vapor install per game.** Rejected because it duplicates
   tools, caches, update state, and recovery.
3. **One standalone Vapor home with product-specific local source adapters.**
   Selected. It gives Vapor one updater and toolchain owner while preserving
   Loo Cast's depot source and Mad Max's external licensed-game source as
   different adapters.

## Ownership

| Area | Owner | Responsibility |
|---|---|---|
| Vapor release install/update transaction | Vapor-Installer + Vapor Core | Verify signed releases, stage, swap, roll back, and uninstall outside Steam |
| Home discovery and managed tools | Vapor Core/Client | Resolve a single home and tools/cache/state layout |
| Player/Developer setup UX | Vapor-Installer | Offer only two roles; Developer is all-or-nothing for required tooling |
| CLI and setup scripts | Vapor-CLI + shared setup resources | Apply the same role/tool/home semantics outside the GUI |
| Loo Cast depot products and locator | Loo Cast + Steam entrypoint + Client adapter | Mount the depot artifact and optionally invoke separately installed Vapor |
| Mad Max discovery and Packagepack | MM-Building-System adapter | Detect user files, prepare/repair loader and bridge, and manage game-specific content |
| Product authority | Vapor server services | Enforce local-only and community-publishable source policy |

## Resulting layout and flow

```text
Steam / Loo Cast app root
  ├─ App.vapor.toml
  ├─ Spacetime Engine product
  ├─ Loo Cast Game product
  ├─ Loo Cast Packagepack product
  └─ tiny optional locator -> %USERPROFILE%\\Vapor\\bin\\vapor tools

Vapor home
  ├─ bin/                 installed Vapor entrypoints
  ├─ tools/cargo/home/    managed Cargo home
  ├─ tools/rustup/home/   managed Rustup home and toolchains
  ├─ sources/             managed source checkouts
  ├─ cache/               rebuildable indexes and derived product data
  ├─ content/             acquired Vapor content and compositions
  ├─ config/              local configuration
  └─ state/               mutable runtime and install state

Mad Max local install
  ├─ Engine source: user-owned game executable/runtime
  ├─ Game layer: scripts, content, and mod composition
  └─ MadMaxModding Packagepack: loader/bridge install and repair only
```

Player installs the runtime without development tools. Developer installs the
same player-facing runtime plus the complete managed development toolset.
Steam's locator only dispatches into Vapor when Vapor already exists; it cannot
install it. Client never uploads or serves game source bytes.

## Exit conditions

- Steam no longer owns Vapor's binary, tool, source, cache, config, or state
  paths.
- A clean standalone Vapor install works without Steam. Loo Cast's optional
  Steam locator works when Vapor is installed and gives install guidance when
  it is absent.
- Setup offers exactly Player and Developer; Developer cannot finish without
  required tool readiness, and no old local role is recognized.
- Windows GNullVM packaging uses the correct MSVC Rustup host toolchain and
  LLVM-MinGW target linker while Linux packaging remains independently
  target-selected.
- Local Loo Cast and Mad Max sources are never mistaken for public Vapor
  release artifacts. Mad Max files remain user-owned; its Packagepack only
  manages integration prerequisites.

## Open implementation details

- Exact directory names below Vapor home should converge with the existing
  Vapor-home path owner; the layout above is the intended ownership, not a
  second path-discovery API.
- The one-time move of Vapor-owned Steam-root files must be transactional and
  preserve authored content. Old roles and old runtime discovery are not
  supported after the move.
- Loo Cast depot IDs and locator packaging require owner metadata.
- Mad Max adapter details are delegated to the separate MM-Building-System
  plan.
