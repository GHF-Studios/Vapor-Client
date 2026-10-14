# Vapor Repository Boundaries and Frontends

**Status:** Owner-approved; local migration in progress
**Goal:** Reshape the Vapor repositories around shared capabilities, two frontends, and separate Studio and Installer products without losing current authored work or Steam launch stability.

## Owner decisions captured

- Dissolve the `Vapor-SDK` repository by moving its useful capabilities into the correct remaining repositories.
- Rename the existing `Vapor-Launcher` repository to `Vapor-Studio`, preserving its history and current work.
- Rename `Vapor-Shell` to `Vapor-CLI`.
- Add a separate `Vapor-GUI` repository that supplies the GUI frontend.
- Keep `Vapor-Installer` as a separate product repository alongside `Vapor-Studio`.
- Rename `Vapor-Entrypoint` to `Vapor-Steam-Entrypoint`; retain its bootstrap role and `vapor-entrypoint` binary name.
- Leave `Vapor-Examples` and `Vapor-Platform-Server` in place for this migration.
- Expose terminal and GUI product modes through `vapor-cli` and `vapor-gui`; bind Steam and other entrypoints to those invocations.

## Target ownership

### Vapor

Own shared, frontend-independent capabilities: content and workspace identities, authoring schemas and templates, toolchain management, typed operation requests/dispatch, composition resolution, build/run, installation/setup/repair and acquisition services, and Steam/Workshop/root packaging providers. Keep these capabilities internally modular. No UI, REPL grammar, or duplicate per-frontend business logic belongs here.

Move SDK modules by capability rather than copying the repository wholesale:

- `vapor_sdk_core::content`, `spec`, `template`, and workspace generation become focused Vapor shared crates/modules.
- `toolchain` becomes part of Vapor's managed-toolchain services.
- `root::package`, `root::publish`, and `steam` become shared typed operations and provider implementations, with authorization/review behavior enforced by the operation contract.
- SDK CLI parsing/rendering becomes Vapor-CLI command adapters.
- SDK GUI scaffolding is retired in favor of Vapor-GUI and the product modules.

### Vapor-CLI

Rename the current Vapor-Shell repository. Own the interactive terminal, REPL, command grammar, and CLI-specific rendering. Commands call Vapor's shared operations. Preserve the existing `vapor` command name as the compatibility default unless packaging proves a rename necessary; repository naming does not force a binary rename.

### Vapor-GUI

A new shared GUI frontend repository containing a reusable `vapor_gui_core` crate and the `vapor-gui` executable. The core crate owns common window/application lifecycle, GUI mode dispatch, and shared GUI components. The executable assembles the Studio and Installer product UI crates. Studio and Installer UI crates may depend on `vapor_gui_core`; that core must not depend on either product crate, avoiding a Cargo package cycle. The GUI presents operation progress and results, while Vapor owns operation behavior.

### Vapor-Studio

Rename Vapor-Launcher in place to preserve history. Own the player/creator product module: library, compositions, workspace IDE, content authoring, publishing review, diagnostics, and direct-play selection/handoff. Its UI module is hosted by Vapor-GUI; it does not own shared operation implementations.

### Vapor-Installer

Remain a separate product repository alongside Vapor-Studio. Own the Installer product flow/UI module consumed by Vapor-GUI and its small early-install bootstrap adapter. Install/setup/repair behavior and state-changing operation contracts live in Vapor so CLI and GUI callers share them. Keep the existing `vapor-installer` bootstrap binary available for recovery and fresh-install cases until the GUI path can reliably operate without it.

### Vapor-Steam-Entrypoint

Remain the stable app-root-aware bootstrap and Steam mode dispatcher. It resolves the installed app root, logs startup, forwards arguments, and starts the requested frontend. Intended routing:

- `studio` -> `vapor-gui studio`
- `installer` -> `vapor-gui installer`, with the Installer bootstrap available for recovery
- `shell` -> `vapor` interactive CLI
- `play` -> the selected composition through the shared launch operation, then the game runtime

Steam launch options remain unchanged during repository migration.

## Dependency direction

```text
Vapor-Steam-Entrypoint ──dispatches──> Vapor-GUI / Vapor-CLI / Vapor-Installer
Vapor-GUI executable ──assembles──> Vapor-Studio UI / Vapor-Installer UI
Vapor-Studio UI / Vapor-Installer UI ──depend on──> Vapor-GUI core
Vapor-CLI ─────────> Vapor shared operation services
Vapor-GUI ─────────> Vapor shared operation services
Vapor-Studio ──────> Vapor shared contracts and operations
Vapor-Installer ───> Vapor shared contracts and operations
```

Shared behavior has one implementation in Vapor. Frontends submit equivalent typed requests and receive equivalent structured results. Product modules may depend on Vapor contracts and the shared GUI core, but Vapor and `vapor_gui_core` must not depend on product repositories.

## Migration sequence

1. Preserve the current parent and submodule HEADs, tracked diffs, and untracked files; inventory every path dependency, workspace binary declaration, CI/release reference, app-root staged path, and Steam route.
2. Establish the shared operation and provider module boundaries inside Vapor; map every SDK module and consumer before moving files.
3. Move SDK shared capabilities into Vapor, CLI adapters into Vapor-CLI, and retire the unused SDK GUI. Keep behavior callable through one shared implementation.
4. Rename Vapor-Shell to Vapor-CLI, Vapor-Launcher to Vapor-Studio, and Vapor-Entrypoint to Vapor-Steam-Entrypoint, updating parent submodules, local workspace manifests, repository metadata, dependency paths, and developer tooling while retaining the `vapor-entrypoint` binary.
5. Create Vapor-GUI with a `vapor_gui_core` library and a `vapor-gui` binary; expose Studio and Installer as product UI libraries that depend on the core, then assemble both behind GUI modes. Keep `vapor-installer` as a bootstrap/recovery binary during the transition.
6. Update Vapor-Steam-Entrypoint dispatch and Steam depot staging to use the new frontend binaries while retaining the existing launch-option contract.
7. Verify through Vapor-managed commands at each owning workspace; run packaging and local runtime checks before pushing repository renames or publishing updated Steam depots.
8. Remove the Vapor-SDK submodule only after all code, history references, documentation, workflows, and consumers have been reconciled. Preserve its Git history in the migration record/archive.

## Preservation and release constraints

The parent and affected submodules were committed locally with scoped messages before migration. A recovery snapshot remains at `/tmp/vapor-repo-boundary-preserve-20261014T123835Z`, including HEAD identifiers, binary tracked diffs from HEAD, working-tree status, and archives of untracked files. The local commits have not been pushed.

Local checkout and submodule-path renames have begun. GitHub repository renames, creation, archive, and pushes are pending because the saved GitHub CLI token is invalid. Steam launch-option values remain unchanged. No build, test, deploy, Steam launch, or publish has been performed for this migration.

## Review points

- The current `vapor` command remains the user-facing CLI binary name while its repository becomes Vapor-CLI; the new GUI frontend binary is `vapor-gui`, with `studio` and `installer` modes.
- The GUI Installer product module lives in Vapor-Installer and is assembled by Vapor-GUI; the existing `vapor-installer` executable remains the early-install/recovery adapter during transition.
- `Vapor-Steam-Entrypoint` retains its app-root-aware dispatch role; its `play` route invokes the shared launch operation for the selected composition without opening a persistent Studio window.
