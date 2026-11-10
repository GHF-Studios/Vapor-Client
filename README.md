# Vapor Client

Vapor Client is the source tree for the standalone Vapor runtime and its tools.
It is not the Loo Cast Steam product. The intended product boundary gives
Vapor its own installation and update lifecycle; the Steam Packagepack is a
separate product assembled from the Loo-Cast source tree. Moving the current
Vapor install and state out of the Steam directory is still in progress.

## Product boundaries

- **Vapor** owns the runtime, launcher, tools, documentation, development
  environment, local caches, and mutable Vapor state.
- **Loo-Cast Steam app** owns one runnable Packagepack product. Its shared
  depot contains `Packagepack.vapor.toml`; each platform depot contains one
  matching Packagepack executable and may include explicitly allowlisted
  runtime DLL/shared-library sidecars required by that executable.
- **Spacetime Engine** and **Loo Cast Game** are composed into the Packagepack
  executable. They are not separate executables in that Steam payload.
- **Vapor Registry** remains the authority for Workshop-backed community
  content. The first-party Loo-Cast Packagepack and its constituent content are
  depot-provided, not Workshop publications.

`App-Source.vapor.toml` in this repository identifies the Vapor Client source
root. It deliberately declares no Steam app. The Loo-Cast repository owns its
own root Steam manifest and staging allowlist.

## Developer workflow

Open this source tree from Vapor to inspect and build the client workspaces:

```text
source open /path/to/Vapor-Client
metadata
validate
```

For a local Loo-Cast Packagepack staging preview, open the Loo-Cast source root
instead and run:

```text
source open /path/to/Loo-Cast
root package --release-targets
root publish --dry-run --release-targets
```

The preview uses the same allowlist, product-specific validation, and SteamPipe
staging path as a release. It does not upload anything. A Packagepack depot
must contain only the shared Packagepack manifest or the one platform
executable plus explicitly included runtime sidecars.

## Repository checkouts

Initialize the pinned project checkouts with:

```bash
git submodule update --init --recursive
```

Key checkouts:

- [`Vapor-CLI`](Vapor-CLI/) — terminal facade.
- [`Vapor-GUI`](Vapor-GUI/) — graphical app host.
- [`Vapor-Installer`](Vapor-Installer/) — bootstrap and recovery installer.
- [`Vapor-Steam-Entrypoint`](Vapor-Steam-Entrypoint/) — Steam-facing locator.
- [`Vapor-Studio`](Vapor-Studio/) — Studio product UI.
- [`Vapor`](Vapor/) — core application workspace.
- [`Vapor-Examples`](Vapor-Examples/) — examples when present.

The CLI command vocabulary is documented in
[`Vapor-CLI/crates/vapor_cli/docs/commands.md`](Vapor-CLI/crates/vapor_cli/docs/commands.md).
Content lifecycle details are in
[`Vapor-CLI/crates/vapor_cli/docs/content.md`](Vapor-CLI/crates/vapor_cli/docs/content.md).
