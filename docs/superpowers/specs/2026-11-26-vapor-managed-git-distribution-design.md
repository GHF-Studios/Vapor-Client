# Vapor-Managed Git Distribution

## Intent

Developer setup provisions a usable Git provider under canonical Vapor User
Data. Source operations run through Vapor's provider interface. Player setup
does not download or install Git.

## Why

Standalone release CI must acquire exact Client and Platform source revisions
without silently selecting runner Git. Developer setup also needs Git without
placing tool state beside the Steam app or requiring the user to install a
system package.

## Goals

- Keep the managed provider at `<Vapor User Data>/tools/git` on Linux and
  Windows, selected explicitly by Vapor after installation.
- Use a version-pinned Linux Git build from upstream source and the official
  portable MinGit distribution on Windows.
- Acquire Git only for Developer setup, through a Vapor command, and install it
  by verified staging and promotion.
- Publish each platform's Git tool payload and its provenance as a separate,
  signed immutable release asset. Keep Git out of the Player setup download.
- Preserve the exact upstream source, build changes/configuration, licenses,
  copyright notices, and third-party notices required by all distributed
  components. Make the complete corresponding source and notices available
  alongside the binary download for the same immutable release.
- Retain compatibility with existing signed app-release manifests while adding
  the optional managed-tool record for newer releases.
- Roll out any new manifest field in two releases: first publish a release whose
  updater understands the optional field while the manifest omits it; only
  after that release is live may a later manifest contain the field. Older
  installed setup binaries reject unknown manifest fields before they can
  inspect `minimum_updater_version`.
- Keep an explicit operator-linked provider as an override, but never fall back
  silently to PATH or common host locations.

## Non-goals

- Platform Server disaster-recovery installation.
- Steam depot or Workshop changes.
- Installing Git for Player mode.
- General-purpose package-manager support or arbitrary user-selected tool
  versions.
- Treating successful Git execution as evidence that a repository checkout is
  under the canonical Vapor Superworkspace; source-root policy remains owned by
  the source workspace layer.

## Approaches

1. **Vapor-published platform payloads, built from upstream inputs — selected.**
   Linux Git is built from a pinned upstream source archive; Windows uses
   upstream MinGit. Vapor's signed release contract publishes the separate
   tool archive, source, and notices. Developer setup downloads only the Git
   payload it needs. This keeps the install owner and trust path in Vapor while
   avoiding Git in Player downloads.
2. **Explicitly linked host Git on Linux.** Rejected as the normal path because
   it cannot bootstrap a blank machine consistently and makes the shipped
   Developer contract depend on host state. An explicit link remains a
   deliberate operator override.
3. **Distro package extraction.** Rejected as the normal path because package
   dependency closure and ABI vary by distribution; it does not provide one
   version-pinned Linux payload for the current Linux target.

## Ownership and contracts

- The Client owns the Vapor provider resolver, Developer setup trigger,
  signature/hash verification, safe installation under User Data, and the
  source/build commands that consume Git.
- The release workflow builds and stages Linux Git through Vapor-managed
  tooling, pins and verifies the official Windows MinGit input, and sends both
  platform tool payloads through the same signed release preparation path.
- Native Git build inputs and intermediates belong to a named Vapor User Data
  cache namespace beneath `cache/build/native-tools/git`, never in the app
  install, the authored Superworkspace checkout, or `tools/git`. Its cache key
  must include the exact source archive digest, build-recipe revision,
  compiler/toolchain identity, target, and enabled feature/configuration set.
  A cache entry is reusable only when its recorded key matches and its
  completion marker was written after the staged output passed its declared
  inventory and version checks. The promoted provider remains separate under
  `tools/git`; deleting the build cache must not uninstall that provider.
- This is a Git-specific cache consumer, not a new generic tool-build
  framework. Current Vapor-managed source workspace builds use
  `output/dev/<project>`, while other managed Cargo realizations use
  `cache/build/<target>`. Do not claim native Git build reuse works until the
  Git recipe itself has demonstrated a repeat build hit through Vapor.
- Artifact owns immutable storage and read routes for tool payload, exact
  corresponding source, and notices. The signed app manifest is the authority
  for the tool asset's platform, version, URL, size, digest, source reference,
  and notice reference. Existing schema-1 application releases remain
  installable.
- A Git payload is not installed as a substitute for the Vapor command
  boundary: every Git process launched by Vapor is resolved and executed by the
  provider.
- Linux build inputs and all included runtime libraries require a recorded
  version, source location, digest, and license. Their corresponding source and
  notices travel with the published source bundle. MinGit remains intact with
  its bundled license and package notices; the exact upstream source links and
  third-party inventory are published with it. The MinGit inventory must be
  derived from the selected archive's actual files and upstream package
  metadata, not inferred from the Git version alone: Git for Windows documents
  that it bundles additional projects under their own licenses.
- For each platform payload, publish a machine-readable component inventory
  (SPDX JSON where the available source metadata supports it) recording each
  component's name, exact version/revision, origin, license identifier and
  license text/notice path, source archive/repository and digest, plus any
  local patches and build/packaging scripts. Include the complete
  corresponding source required by each component's license, not only a link
  to its upstream homepage or a Git commit. Preserve all upstream notices and
  license files in the installed tool payload where required and in the
  separately downloadable legal/source bundle.
- The signed Vapor release manifest references the exact immutable Git binary,
  corresponding-source bundle, and notice/inventory bundle, each with size and
  SHA-256. These references are covered by the existing detached signature.
  The source and notice routes remain available for as long as the binary is
  distributed. The download page exposes those links without making a user
  fetch source during Developer setup.
- Manifest compatibility is staged: a compatibility release with the new
  optional-field parser ships without a Git record first. Once it is the
  current signed release, the next release may include the managed-Git record.
  Do not put the new field into the first release built by an updater that
  cannot parse it; the existing strict schema rejects unknown fields before
  checking the minimum updater version.
- No product binary is distributed until a release check confirms the exact
  payload inventory is complete, all declared licenses permit the intended
  redistribution, all required texts/notices and corresponding sources are
  included, and every referenced asset passes signed size/digest verification.
  A failed or unknown license classification blocks publication for that
  payload; this is a release-engineering gate, not a legal opinion.

## Open implementation detail

The Linux builder must produce a runtime bundle with a known dependency closure
using Vapor-managed compiler/toolchain inputs. The published 0.1.1 setup
predates both role management in `vapor-installer` and distributable managed
Git. Its `dev-env install` provisions the managed compiler but leaves the
installation in Player role, while root release builds are Developer-only.
For this first transition, the release workflow may use GitHub checkout only
when the installed bootstrap has no role command: copy that exact Client
revision into a temporary canonical `Vapor/sources` seed, build the current
Installer with `vapor sdk build`, and use that Installer's role promotion to
reconcile Developer setup. Then remove the seed and reacquire Client and
Platform through Vapor's Git provider. The signed bootstrap's role command
should make this source seed unnecessary on later releases. Until a managed
Git payload passes its distribution gates, CI may select its runner Git only
as an explicit `VAPOR_GIT` provider override; never use ambient PATH fallback
or invoke Git directly.

The native Git cache must coexist with Vapor's disposable-output and Cargo
cache ownership. A repeat build with unchanged inputs should reuse configured
and object outputs; a change in any cache-key input must select a fresh entry.
The cache must be safe to delete while the separately installed Git provider
remains usable.

The legal inventory is not yet complete because the exact Git and MinGit
revisions and Linux runtime closure have not been selected. In particular,
MinGit's complete corresponding-source set may include Git for Windows build
recipes, patches, and sources for bundled dependencies. Verify the exact
selected archive and its notices before making the first distributable payload;
do not assume a Git source tarball alone covers MinGit.

## Completion evidence

- Player setup installs Vapor without downloading Git.
- Developer setup installs the correct Linux or Windows Git payload beneath
  Vapor User Data and reports its pinned version; repair replaces only the
  owned Git component.
- `provider git status` reports the managed provider, and Vapor-mediated
  clone/fetch/status work succeeds without a running Client service or host
  PATH Git.
- The latest release signature authenticates each Git payload's exact size and
  digest; it also authenticates the source and notice/inventory asset
  references, whose complete contents are reachable from the immutable release
  path and retained while the binary is offered.
- The resulting standalone app release retains version format
  `0.5.0-pre-alpha-<inner SemVer>` and leaves Steam unchanged.
