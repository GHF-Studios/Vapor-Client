# Vapor Standalone Runtime and Steam Product Boundaries

**Status:** Draft for owner review  
**Date:** 2026-11-02

## Goal

Define a clean boundary between Vapor as an independently installed modding/content-management runtime and products that are locally acquired through Steam. Keep Steam responsible for installing and updating its app depots; keep Vapor responsible for its own installation and for managing modding workflows around locally available games and engines. Ensure that server policy, rather than an editable client manifest, decides which products may be published or accessed.

## Decisions from owner direction

- Vapor is independently installed, updated, and uninstalled through Vapor's own installer.
- The Loo Cast Steam app retains its game launch entrypoint and its dedicated entrypoint for opening the Vapor tools selector.
- The Steam-side Vapor entrypoint discovers an existing Vapor installation. If absent, it displays install guidance; it does not silently install Vapor.
- Uninstalling the Loo Cast Steam app never removes an independently installed Vapor runtime.
- Mad Max remains an unmodified Steam app. Vapor detects the expected local install and reports an absent or incompatible installation; it does not replace Steam's launch integration.
- The three Loo Cast first-party products (Spacetime Engine, Loo Cast game, and PackagePack) are installed locally through the Loo Cast Steam app depot. They are not Workshop content and are not distributed by Vapor's content artifact service.
- There is one ecosystem-level Vapor.lock, one canonical toolchain lock, and one target-specific build-cache namespace per unique target. Windows and Linux build caches are isolated.
- Steam app manifests may describe products and their local source, but a client-controlled declaration cannot grant publication or access authority.

## Product and runtime ownership

### Steam-owned product installations

The Loo Cast Steam app installs and updates its three first-party products and retains the two existing launch entrypoints plus App.vapor.toml. The app manifest identifies the products by stable canonical IDs and describes their Steam-depot origin. These records are useful to local discovery and Vapor's UI, but are not authorization credentials.

Vapor references the files installed by Steam. It does not copy those files into a public content cache or upload them to Vapor services. Steam remains the authority for installation and update of its files. Any Vapor-managed overlays or modifications to a live game directory require a separate reversible deployment ledger and recovery design; they must not be conflated with the depot's ownership of the base product.

Mad Max is represented as an externally acquired local product. Discovery uses the expected Steam app identity and required file checks. The detector is read-only; it does not patch Steam, distribute game files, or pretend an unrecognized directory is valid.

### Independent Vapor home

The standalone installer places Vapor under the user's home directory (%USERPROFILE%\Vapor on Windows and $HOME/Vapor on Linux). Vapor's installation and user-managed state live there, not in any Steam app directory. The exact initial tree is:

~~~text
Vapor/
  bin/
  config/
  content/
  data/
  docs/
  examples/
  metadata/
  output/
  resources/
  sources/
  cache/
    build/
      <target-triple>/
  tools/
    cargo/
      bin/
      home/
    rustup/
      bin/
      home/
~~~

App.vapor.toml remains with the Steam-installed Loo Cast products because it describes that Steam app's local product set. The Vapor home uses named top-level directories rather than a redundant hidden .vapor folder. Installer receipts, locks, source registrations, and user preferences are placed in the relevant config, data, or metadata directory according to their ownership; they do not create a second hidden root.

Vapor's installer registers the install location so the Steam handoff can find it without searching arbitrary disks. The installer owns self-update, repair, and uninstall. Uninstalling Vapor leaves Steam-owned game files and user-authored source/content data intact unless the user separately selects their removal.

## Steam launch handoff

- The direct Loo Cast entrypoint launches the game as it does today.
- The Vapor entrypoint resolves the registered Vapor home and opens the Vapor tools selector, passing the Loo Cast app root as an explicit local product context.
- If the runtime is absent, the entrypoint displays a small explanation and the official Vapor download location.
- The handoff does not set VAPOR_HOME to the Steam app root and does not require Steam to own Vapor's binaries.
- Steam uninstall removes only files owned by its app/depot. It does not run Vapor's uninstaller.

## Server authority and publication policy

A local App.vapor.toml, content manifest, or package-provided publishable field is descriptive input only. It cannot establish who owns a product or grant permission to upload it.

- Vapor Identity remains the authority for verified identities, roles, and server-side entitlement decisions.
- Vapor Registry may hold canonical product IDs and versioned product/source policy records. Registry metadata remains protected by Identity authorization where required.
- An upload service must resolve the submitted product ID and publisher against server-owned policy, then ask Identity for the relevant author and entitlement decision. Unknown, reserved, or conflicting product IDs fail closed. A payload cannot claim a first-party namespace by editing its local manifest.
- The existing Vapor Artifact Server remains the Vapor application release service. User-generated content uploads are a separate service concern; routing may still compose every service into the one VPS process.
- The Identity-to-service authorization contract is internal and server-only. No Steam publisher credential, signing private key, or service token ships in Vapor clients or manifests.

First-party Steam-depot products are registered as local-only, non-publishable products. Vapor content packages may declare them as local requirements, but the actual product bytes remain sourced from the user's Steam installation.

**Payload limit:** Reserved IDs and server-side policy prevent publishing the canonical first-party products under their real identities. They cannot, by themselves, detect a user who copies or transforms game files and uploads them under a new identity. Stronger enforcement needs a protected-file fingerprint/scan policy and, for ambiguous cases, review. This protection must be stated honestly rather than represented as a property of a client manifest.

## Locks and build caches

- One Vapor.lock is the sole ecosystem lock. It pins the current first-party source revisions, product graph, build-plan identity, and inputs needed to reproduce the pre-alpha workspace. There is only one supported rolling Vapor state; older clients are updated or rejected rather than supported as parallel cache generations.
- One canonical toolchain lock pins Rust/Rustup/Cargo versions, supported targets, and the native linker/tool versions used by the canonical build plan.
- The installer configures one shared Cargo target root per unique target triple at Vapor/cache/build/<target-triple>. All managed Rust workspaces use that root. Cargo.lock files inside independently maintained repositories remain source inputs captured by the ecosystem lock; they are not additional Vapor-level profiles or user-facing locks.
- For each supported target, the VPS serves one current prebuilt target snapshot, generated by the canonical Vapor-managed build plan for the current Vapor.lock and toolchain lock. This can be a multi-gigabyte archive and includes the artifacts Vapor expects its developer/composer workflows to need. It is not a general cache lookup for arbitrary client versions.
- The VPS is the build orchestrator and sole publisher of official snapshots. Its Linux lane runs in a pinned Linux container on the Ubuntu VPS host. The container image and libc baseline are part of the canonical toolchain lock. Arch is not the default build image: a newer glibc baseline can produce binaries that fail on older Linux installations.
- The Windows lane runs on a native Windows worker controlled by the VPS, preferably a Windows VM on that VPS. It builds the supported Windows target on Windows so Cargo host tools and target artifacts are both Windows-compatible. A Linux cross-build may still be used for release binaries, but it is not accepted as a complete Windows developer snapshot.
- Developers may build locally for their own iteration, but local outputs do not publish or replace official snapshots. If the VPS cannot host a Windows VM, the fallback is a managed Windows CI worker that the VPS build orchestration controls; a human-run build on the owner's desktop is not a required release step.
- On composer/developer role setup, Vapor computes the selected target triple, fetches that target's current snapshot, verifies its server-authenticated manifest, lock identities, target, and archive digest, and installs it into the shared target root. The target triple selects the cache bucket; all other build dimensions are fixed by the one Vapor lock and canonical build plan.
- Cache publication is a controlled pre-alpha release operation. The server stages and validates a new archive, then atomically replaces the current target snapshot. Identity authorizes publication; consumers cannot upload or select arbitrary snapshots. Vapor APIs reject stale client versions, and developer setup refuses a snapshot whose lock or target does not match the installed runtime. A missing, stale, or corrupt snapshot is a setup failure with a repair/retry path, not a silent local cold-build fallback.
- The snapshot is produced and restored by Vapor-controlled tooling, not by relying on Cargo to manage distribution. Cargo still performs its normal freshness checks when a developer invokes a build. The acceptance test is a clean install on a second machine/home path followed by the canonical build commands with no unexpected recompilation.

The precise Cargo invocation, profile, feature set, environment, build-script inputs, host tools, and linker were what I meant by “how the build was produced.” Those are variables in an unmanaged build environment. Here, Vapor owns the recipe and lock, so they belong to the one canonical snapshot rather than a user-facing cache-key matrix. Cargo's target directory is internal and can change with Cargo, but the pinned toolchain controls that version dimension. Moving the snapshot between user home paths still needs a restore test; Cargo's fingerprinting documentation explicitly aims to preserve reuse when a project directory is renamed.

The remote build-cache capability is a separate service concern from Vapor release downloads and user content. It should have its own service module/repository, while the Platform Server composes it into the existing single VPS process and public domain. This keeps one deployment footprint without mixing cache policy into Artifact or Identity business logic.
## Migration sequence

1. Publish a new Loo Cast depot layout and manifest that contains only the three local products, the two entrypoints, and their required small launch resources.
2. Remove first-party product publication paths from the Workshop catalog and install/update them only through Steam depots. Existing Workshop items are not deleted until their owners explicitly perform or approve that external migration.
3. Ship the standalone Vapor installer and move Vapor binaries, developer tools, source checkouts, caches, docs, examples, and mutable state to the Vapor home layout.
4. Update the Steam handoff to discover Vapor externally and pass the explicit Steam product root to the tools selector.
5. Add server-owned first-party product policies and upload enforcement before enabling user-content uploads for any new content type.
6. Add the target-snapshot service and VPS build orchestration: isolated Linux and Windows VMs hosted by the one VPS and managed through the same orchestration path. Verify exact lock/target/environment matching, atomic replacement, stale-runtime refusal, archive integrity, and clean restore on a second machine/home path. If the VPS cannot host both VMs with the required resources and licensing, leave the affected target unavailable until the VPS is upgraded or replaced; do not use a separate worker or owner PC.
7. Add read-only Mad Max detection and local-source validation without modifying its Steam app.

## Non-goals for this design

- Removing or editing existing Steam Workshop items without explicit owner approval.
- Shipping Mad Max or Loo Cast game files through Vapor.
- Making client manifests authoritative or placing server secrets in client builds.
- Putting identity, registry, content-upload, or cache business logic into the Platform Server root repository.
- Treating a downloaded Cargo target tree as portable without validation.
- Implementing the full overlay/profile transaction model in the same change as the installer relocation.

## Review points

1. Is %USERPROFILE%\Vapor / $HOME/Vapor the desired canonical home, with the listed directories as the initial ownership map?
2. Does the server boundary read correctly: Registry stores canonical product policy, Identity authorizes, a separate content-upload service enforces, Artifact remains Vapor-release-only?
3. Does the builder topology match: Linux and Windows in pinned VMs hosted by the single VPS and managed through one orchestration path, with no external worker or owner-PC fallback?
4. I interpret Cargo.lock files inside source repositories as build inputs governed by the one Vapor.lock, not as extra Vapor-level locks. Is that your intended meaning of “one lock”?
5. Does the migration sequence preserve the intended Steam ownership and leave external Workshop deletion under explicit owner control?



