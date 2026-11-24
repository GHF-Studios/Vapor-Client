# Vapor Standalone Lifecycle

## Intent

Vapor is independently installable and manageable from `vapor.ghf-studios.site`.
The Loo-Cast Steam app remains a directly playable game distribution. Vapor is
optional for players and required for creating content and using creator
tooling.

## Goals

- Install, update, select a previous release, repair, and uninstall Vapor
  without requiring the Loo-Cast Steam app.
- Use the Platform Artifact Server's signed immutable release manifests and
  one installer-owned lifecycle on Windows and Linux.
- Keep Vapor-owned logs, caches, toolchains, game-specific user state, and
  saves in canonical Vapor-managed locations, never beside the Steam game.
- Offer an explicit full wipe of every registered Vapor-managed location,
  including user state, caches, logs, game saves, and the canonical source
  workspace.
- Keep direct Steam Play free of Vapor setup and first-run gates. Provide a
  separate creator entry that opens Vapor when installed and guides the user to
  the website installer when it is absent.
- Preserve a normal uninstall path that removes Vapor installation and OS
  integration while keeping authored source and user data.

## Non-goals

- Do not require Vapor to launch the depot-owned default Loo-Cast composition.
- Do not make Steam the installation, update, downgrade, or uninstall authority
  for Vapor.
- Do not delete the Steam-managed Loo-Cast depot during Vapor uninstall.
- Do not recursively delete a user's home directory or any unregistered path.

## Ownership and path contract

- The Steam App Instance owns only the Loo-Cast runtime payload, Steam metadata,
  and the minimum bootstrap/creator entrypoints needed to start the game or
  hand off to standalone Vapor.
- Vapor's installed versions, app state, tools, caches, logs, and per-game
  state live under OS-appropriate canonical Vapor data locations. Game-specific
  state is namespaced below that location (for example
  `apps/loo-cast/...`).
- Authored repositories remain under the canonical user-facing Vapor home,
  with source checkouts in `~/Vapor/sources` (and the Windows equivalent).
- A full wipe enumerates registered canonical roots before deleting. It removes
  all Vapor-owned content, but refuses to erase unrelated entries found in a
  shared parent such as `~/Vapor`; the UI shows the exact paths and skipped
  entries before confirmation.
- Legacy app-local mutable Vapor data is migrated into canonical locations
  before the Steam footprint is cleaned. Steam-owned binaries and game payload
  remain managed by Steam.

## Install and version behavior

- The public website is the discovery and download surface. A small standalone
  setup executable bootstraps the product from Artifact's signed immutable
  release artifacts; the signed manifest identifies platform, version,
  channel, URL, size, and SHA-256 digest. The public release bytes are served
  by Artifact, not GitHub Releases or the Homepage process.
- The installed bootstrap/installer owns version acquisition, validation,
  staging, activation, rollback, repair, and uninstall. It stages a version
  completely before activation and retains the previous known-good version for
  rollback.
- Stable is the default channel. Users can explicitly select an available
  published version to downgrade. User data is stored outside versioned program
  directories and remains intact across version switches.
- The homepage reads Artifact's stable feed and only exposes supported Windows
  and Linux releases after complete signed bundles are published. A signed
  version-history surface is still needed for a full version picker; exact
  immutable version manifests exist in the current API.
- Existing Steam-installed Vapor binaries are adopted or migrated through a
  one-time path that preserves Vapor User Data and authored source.

## Uninstall behavior

- **Remove Vapor:** removes the standalone installed product and OS integration;
  keeps canonical source, settings, content, caches, logs, and saves.
- **Erase all Vapor data:** displays the exact registered roots and permanently
  removes the installed product, OS integration, Vapor User Data (including
  game-specific state, caches, logs, and saves), and the canonical source
  workspace. It does not remove the Steam-managed Loo-Cast game.
- Both operations preflight every deletion and machine-integration change
  before mutating anything. An unsafe or unrecognized path blocks the wipe
  rather than expanding its deletion boundary.

## Creator entry UX

- Steam's primary Play entry launches Loo-Cast directly.
- A separate creator entry resolves the standalone Vapor installation. If it
  exists, it opens Vapor's creator surface; otherwise it opens the Vapor
  download page and explains that Vapor is needed to create content, not to
  play the bundled game.
- Vapor's Installer presents update, version selection, repair, normal remove,
  and explicit erase-all actions. No game launch is gated on this UI.

## Approaches

1. **Website-distributed standalone setup backed by signed Artifact releases** —
   selected. It gives Vapor independent lifecycle control without the Steam app
   fee, while preserving Steam as Loo-Cast's player distribution channel.
2. Portable archives with manual install/version management — rejected because
   they leave upgrades, rollback, and complete removal to users.
3. A separate paid Steam app — rejected by the owner because of its cost; Steam
   itself remains a trusted and acceptable dependency where useful.

## Open implementation questions

- The Artifact API and Homepage feed are present, but no complete stable
  release/bootstrap was confirmed in this checkout. Confirm the production
  signing key and first release artifacts before claiming a download is live.
- Preserve the existing Steam release route while adding a distinct standalone
  artifact/package route; do not make the Steam release format ambiguous.
- Any source or user-data roots outside the canonical defaults must be
  inventoried and explicitly registered before full wipe can claim to remove
  them.

## Completion evidence

- Clean Windows and Linux installs can be bootstrapped from the website surface,
  launch Vapor independently of Steam, update, select an older published
  version, repair, and uninstall.
- Default Steam Play launches Loo-Cast without Vapor setup or user-data writes
  in the Steam app root.
- Creator entry opens installed Vapor or gives a working website installation
  path.
- Full wipe removes every registered Vapor-managed root, including the canonical
  Superworkspace and game-specific state, and preserves unregistered neighboring
  user files and the Steam-managed Loo-Cast payload.
