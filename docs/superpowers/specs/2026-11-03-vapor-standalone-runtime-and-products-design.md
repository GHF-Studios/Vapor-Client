# Standalone Vapor Runtime and Product Boundaries

**Status:** Superseded on 2026-11-10. Historical context only; use the current
Vapor-Platform-Server realignment spec for the Steam payload and lifecycle.
**Date:** 2026-11-03

> **Artifact correction:** The old “self-contained executable” wording is not
> the active Steam contract. Each target has one primary executable entry
> point; required DLLs or other declared sidecars are allowed. The Steam
> payload is the Loo Cast Packagepack, with `Packagepack.vapor.toml`; Vapor is
> installed independently and `App.vapor.toml` does not belong in that depot.

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
- The Loo Cast depot provides one self-contained executable per supported
  target. Each embeds `App.vapor.toml` and the Spacetime Engine, Loo Cast Game,
  and Loo Cast Packagepack, but not the Vapor runtime. Steam's Play action asks
  separately installed Vapor to run the default composition; the dedicated
  tools action opens Vapor's selector. Either action gives install guidance if
  Vapor is absent. The executable never bundles, installs, or updates Vapor.
- The only local setup roles are `Player` and `Developer`. Player setup excludes
  developer tooling. Developer setup installs all required development tools,
  including both supported targets and managed cross-compilers, with no skip
  path. Ecosystem developer authorization is server-side Identity state, not a
  local role.
- This is pre-alpha: remove the former Composer and EcosystemDeveloper local
  role model directly. Do not retain compatibility parsing, role migration, or
  alternate old-layout discovery. Preserve user-authored content and move
  Vapor-owned files out of the Steam app root as part of the standalone setup.
- After conversion, each Loo Cast Steam target contains one application
  artifact: the self-contained executable with its embedded manifest and
  product payloads. There are no loose manifest, product directories, or
  separate locator files. Before moving Vapor-owned Steam-root data to Vapor
  home, validate the replacement executable and preserve all source data until
  the package passes its target checks. Symlinks, special files, and conflicting
  content block conversion instead of being silently discarded.
- Known data inside the former `.vapor` folder is split into the standalone
  layout: diagnostics to `data/diagnostics`, downloads to `cache/downloads`,
  extraction staging to `cache/extract`, logs to `data/logs`, and runtime state
  to `data/state`. Unknown entries are retained under a recovery namespace;
  this is a one-time data move, not support for the old `.vapor` layout.
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
- Vapor Client is not the Loo Cast Steam app source. Its old Steam App/Depot
  declarations and Vapor-to-Steam self-publishing implementation are retired;
  Vapor releases use the independent Vapor release service. The Loo Cast
  composition manifest belongs to Loo Cast, and its app/depot source uses an
  explicit input allowlist that is packed into one executable rather than
  emitted as loose depot files.

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
| Loo Cast depot artifact and tools handoff | Loo Cast + Steam entrypoint + Client adapter | Build one self-contained per-target executable with all three products and the embedded manifest; optionally invoke separately installed Vapor |
| Mad Max discovery and Packagepack | MM-Building-System adapter | Detect user files, prepare/repair loader and bridge, and manage game-specific content |
| Product authority | Vapor server services | Enforce local-only and community-publishable source policy |

## Resulting layout and flow

```text
Steam / Loo Cast app root
  └─ one target-specific executable
       ├─ embedded App.vapor.toml
       ├─ embedded Spacetime Engine
       ├─ embedded Loo Cast Game
       ├─ embedded Loo Cast Packagepack
       ├─ Play -> separately installed Vapor runs the default composition
       └─ Tools -> separately installed Vapor selector

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
Both Steam launch actions require the separately installed Vapor runtime and
give install guidance when it is absent. Client never uploads or serves game
source bytes.

## Exit conditions

- Steam no longer owns Vapor's binary, tool, source, cache, config, or state
  paths.
- A clean standalone Vapor install works without Steam. Loo Cast's Play action
  starts its default composition through Vapor, and its Tools action opens the
  Vapor selector; either action gives install guidance if Vapor is absent.
- Setup offers exactly Player and Developer; Developer cannot finish without
  required tool readiness, and no old local role is recognized.
- Windows GNullVM packaging uses the correct MSVC Rustup host toolchain and
  LLVM-MinGW target linker while Linux packaging remains independently
  target-selected.
- Local Loo Cast and Mad Max sources are never mistaken for public Vapor
  release artifacts. Mad Max files remain user-owned; its Packagepack only
  manages integration prerequisites.

## Open implementation details

- The one-time move of Vapor-owned Steam-root files must be transactional and
  preserve authored content. It must validate the replacement executable
  artifact before moving other Vapor-owned top-level entries out of Steam.
  Old roles and old runtime discovery are not supported after the move.
- The existing AppID/depot IDs and Steam launch-action metadata need a final
  owner check. The embedded manifest must describe depot-local first-party
  products and must not retain the retired Workshop IDs.
- Mad Max adapter details are delegated to the separate MM-Building-System
  plan.
