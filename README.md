# Vapor Client

Vapor Client is the source workspace for Vapor's independent runtime and its
Loo-Cast Steam content mount. The two distributions have separate owners and
payloads.

## Vapor distribution

Vapor installs independently under `%USERPROFILE%\\Vapor` on Windows or
`$HOME/Vapor` on Linux. That home owns the runtime, tools, docs, sources,
content, caches, generated output, and writable state. The standalone package
staging path is `output/standalone/<target>`; it is intended for the Vapor
download service and contains only the target runtime, shared read-only assets,
and `Vapor-Standalone.toml`. Each package is built for one target ABI; shared
assets do not make the executable or native product payload portable across
targets.

## Loo-Cast Steam app

The Loo-Cast Steam app (`2122620`) is a content mount containing the
Spacetime Engine, Loo-Cast game, and Loo-Cast Packagepack as one composition.
Its common depot contains the composition manifest and shared product data;
Linux and Windows depots carry ABI-specific product files. It contains no
Vapor executables, launchers, tools, caches, or writable runtime state. Vapor
keeps its runtime and state in its own home and mounts the Steam product as
explicit external context.

The three Loo-Cast products are staged from their locally installed Vapor
content trees into the Steam payload. Steam then owns their installation,
updates, and removal. They are not Workshop downloads or Vapor Artifact Server
releases. The Loo-Cast source checkout is the authority for these products; this
client checkout only declares the final depot staging paths.

Mad Max is different: Steam installs the publisher-owned base game, while Vapor
only manages supported user-installed add-ons around that local game.

## Build and package

Open this source root in the installed Vapor Shell and use Vapor-managed
commands. Build both supported targets before producing release packages. The
package workflow stages two distinct outputs:

- `output/root/content/` for the Loo-Cast Steam depots;
- `output/standalone/` for independent Vapor downloads, one folder per target.

The Steam payload is allowlisted in `App-Source.vapor.toml`; the standalone
payload is described by `Vapor-Standalone.toml`. Review both staged outputs
before publication. Steam publication and website deployment are separate
explicit release actions.

## Repository checkouts

Initialize the pinned workspaces with:

```text
git submodule update --init --recursive
```

- [`Vapor/`](Vapor/) — client core, providers, and managed workflows.
- [`Vapor-CLI/`](Vapor-CLI/) — terminal facade and user documentation.
- [`Vapor-GUI/`](Vapor-GUI/) — shared GUI host and launcher.
- [`Vapor-Installer/`](Vapor-Installer/) — standalone install and maintenance UI.
- [`Vapor-Steam-Entrypoint/`](Vapor-Steam-Entrypoint/) — Steam-to-Vapor handoff
  prototype; it is not included in the Loo-Cast depots.
- [`Vapor-Studio/`](Vapor-Studio/) — Studio UI.
- [`Vapor-Examples/`](Vapor-Examples/) — example content.

## Product boundaries

- `Vapor-Home.toml` identifies an independent Vapor installation.
- `App.vapor.toml` describes the Loo-Cast Steam product root.
- `App-Source.vapor.toml` is the allowlist for Loo-Cast Steam depot payloads.
- `Vapor-Standalone.toml` identifies the independently downloadable Vapor
  runtime payload.
- Server-side policy is authoritative for whether first-party content may be
  published; a client manifest cannot grant that permission.
