# Vapor Client

Vapor Client is the source tree for Vapor, the independently installed runtime
and content manager. Vapor owns its own install, update, repair, uninstall,
tools, source checkouts, caches, configuration, and state under the user's
Vapor home. Steam is a source of game products, not Vapor's installation
authority.

## Product Boundaries

- **Vapor** installs independently from Steam. Local setup has two choices:
  **Player** (no development toolchain) and **Developer** (the full managed
  development toolset, including both supported targets; no partial Developer
  setup). Ecosystem developer authorization is a server-side permission, not a
  local role.
- **Loo Cast** ships from its official Steam depot as a Packagepack mount. The
  common depot contains `Packagepack.vapor.toml`; Linux and Windows depots each
  contain one matching primary Packagepack executable plus only the runtime
  DLLs or shared libraries it declares as required. That executable composes
  the Engine and Game. No Vapor runtime or Vapor-managed files are part of the
  Steam app. The candidate still needs both target builds and exact depot
  inventory verification.
- **Mad Max** is acquired and installed by the user. Vapor treats the local
  executable and required original runtime files as its Engine source, scripts
  and mods as the Game layer, and `MadMaxModding` as the Packagepack that
  prepares the game to accept Vapor content. That Packagepack contains no Mad
  Max game files.
- Vapor's servers must not distribute Loo Cast or Mad Max game files. The
  owning services decide publishability; a local manifest cannot grant itself
  permission.

This is the approved pre-alpha target, not a claim that migration is complete.
The source now has separate standalone Vapor metadata and Loo Cast Steam-mount
metadata. The unified `root package` route selects standalone bundles and/or
Steam depot staging from the selected source root; `root deploy` is limited to
standalone Vapor releases. The code has compile-level verification on Windows,
but actual Loo Cast package generation and depot staging, Linux target builds,
and the first-run Steam-root migration have not been verified. The installed
Steam app is still untouched and remains in its old layout. The implementation
is tracked in
[`docs/superpowers/plans/2026-11-03-vapor-standalone-runtime-and-products.md`](docs/superpowers/plans/2026-11-03-vapor-standalone-runtime-and-products.md).

## Repository Ownership

- **Vapor Client** owns the standalone runtime, its independent installer and
  updater, local Player/Developer setup, and generic Engine/Game/Packagepack
  source handling.
- **Loo Cast** owns its Steam application manifest and the depot composition
  containing the Spacetime Engine, Loo Cast Game, and Loo Cast Packagepack.
  Those licensed files are acquired through the official Steam app and are not
  published as public community content.
- **MM-Building-System** owns the Mad Max-specific adapter and Packagepack. The
  user supplies the game files; Vapor's servers never include them.
- **Vapor's Platform services** own publication policy. A local manifest does
  not grant permission to publish content.

Use the source-controlled Vapor CLI for packaging work. `root package` stages
the products declared by the selected source; it does not itself publish them.
Do not use the old installed CLI or run the Steam-root migration until the
candidate is installed and the first-run migration path has been reviewed. A
real standalone release/deploy remains unverified, and Loo Cast depot upload
still requires its own reviewed app-source staging and publication step.

The current Shell command model is documented in
[`Vapor-CLI/crates/vapor_cli/docs/commands.md`](Vapor-CLI/crates/vapor_cli/docs/commands.md).
The product model behind it is in
[`product-topology.md`](Vapor-CLI/crates/vapor_cli/docs/design/product-topology.md).

## Repository Checkouts

Initialize pinned project checkouts with:

```bash
git submodule update --init --recursive
```

Key checkouts:

- [`Vapor-CLI`](Vapor-CLI/) — terminal facade.
- [`Vapor-GUI`](Vapor-GUI/) — graphical app host.
- [`Vapor-Installer`](Vapor-Installer/) — bootstrap and recovery installer.
- [`Vapor-Steam-Entrypoint`](Vapor-Steam-Entrypoint/) — Steam-facing entrypoint.
- [`Vapor-Studio`](Vapor-Studio/) — Studio product UI.
- [`Vapor`](Vapor/) — core application workspace.
- [`Vapor-Examples`](Vapor-Examples/) — examples when present.

## Custom Content

Workshop-backed content uses `content *` commands from an opened content
workspace. Loo-Cast currently proves the first-party path with:

```text
source open /path/to/Loo-Cast
content validate
script run content-roundtrip
script run content-publish-preview
```

The local roundtrip packages, caches, installs, verifies, updates, repairs, and
uninstalls content without changing Steam authority. Publication dry-runs write
provider scripts but perform no upload. Real Workshop create, publish, and
delete operations remain manual authority-changing actions.

See
[`Vapor-CLI/crates/vapor_cli/docs/content.md`](Vapor-CLI/crates/vapor_cli/docs/content.md)
for layout, receipts, fingerprints, and repair behavior.

## Setup And Recovery

Vapor-home tools own installed setup:

```text
rust-script --force <vapor-home>/resources/vapor/tools/production/app_setup/setup_player.rs
rust-script --force <vapor-home>/resources/vapor/tools/production/app_setup/teardown_player.rs
rust-script --force <vapor-home>/resources/vapor/tools/production/app_setup/setup_development.rs
rust-script --force <vapor-home>/resources/vapor/tools/production/app_setup/teardown_development.rs
```

The standalone Installer owns the Player or Developer choice and applies the
matching setup inside Vapor home. Developer always receives the complete
supported toolchain; there is no partial Developer option. Steam does not
install Vapor. Loo Cast's Play action runs its default composition through the
installed Vapor runtime; its Tools action opens Vapor's selector. Both give
install guidance if Vapor is absent.

## Planning

Planning files are review infrastructure, not a substitute for user docs:

- [Protocol](docs/planning/README.md) — canonical planning lifecycle.
- [Intake](docs/planning-intake/README.md) — retained original sources.
- [Atomized](docs/planning-atomized/README.md) — unvetted atomic candidates.
- [Active](docs/planning-active/README.md) — current vetted assertions.
- [Superseded](docs/planning-superseded/README.md) — historical assertions that
  remain mandatory review material.
- [Evidence](docs/planning-evidence/README.md) — extraction and review records.
