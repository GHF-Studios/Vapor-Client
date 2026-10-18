# Vapor Client

Vapor Client is the first-party application source root for Vapor. It assembles
the Steam-installed Vapor app, SDK, launcher, shell, entrypoint, tools, docs,
and app/depot payload. It is source, not the Steam installation itself.

Most day-to-day work happens through the installed Vapor Shell. Bootstrap
installs may start from `bin/vapor`; release app payloads use
`bin/<target>/vapor-entrypoint[.exe]`, which opens the platform terminal and
runs the matching `bin/vapor-launch.*` script. Source repositories stay outside
the Steam app root and are opened explicitly:

```text
source open /path/to/Vapor-Client
metadata
validate
root package
root publish --dry-run
```

Release app/depot workflows consume the target matrix declared in
`App-Source.vapor.toml` by default, such as Linux plus Windows GNU/LLVM. Use
`--host-only` for quick local smoke passes that should not build or stage every
runtime target. Real app publication rejects host-only/custom target subsets
and publishes the complete declared Linux+Windows matrix.

Real app/depot publication still requires manual interactive confirmation:

```text
root publish --account ACCOUNT --yes
```

## Product Topology

- **Steam installation/app root**: the installed app directory that owns
  `App.vapor.toml`, `bin/vapor-launch.*`,
  `bin/<target>/vapor-entrypoint[.exe]`, `bin/<target>/vapor[.exe]`,
  optional bootstrap `bin/vapor`, app-local Rust/Cargo, SteamCMD, caches,
  generated output, installed content, indexes, locks, and receipts.
- **Vapor Client**: this source-stage application root. It packages and publishes
  the complete Steam app/depot.
- **Vapor-Registry**: separate registry authority infrastructure. It is not a
  Cargo workspace and not normal content.
- **Loo-Cast**: first-party custom content workspace. It is normal Workshop
  content, not app/depot source.

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

App-root tools own installed setup:

```text
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/setup_player.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/teardown_player.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/setup_development.rs
rust-script --force <app-root>/resources/vapor/tools/production/app_setup/teardown_development.rs
```

The default player setup prepares SteamCMD and generated `.vapor/` state.
Developer setup is an explicit upgrade. `vapor-installer` is the compiled
Steam/app-root launcher for the same setup implementation.

## Planning

Planning files are review infrastructure, not a substitute for user docs:

- [Protocol](docs/planning/README.md) — canonical planning lifecycle.
- [Intake](docs/planning-intake/README.md) — retained original sources.
- [Atomized](docs/planning-atomized/README.md) — unvetted atomic candidates.
- [Active](docs/planning-active/README.md) — current vetted assertions.
- [Superseded](docs/planning-superseded/README.md) — historical assertions that
  remain mandatory review material.
- [Evidence](docs/planning-evidence/README.md) — extraction and review records.
