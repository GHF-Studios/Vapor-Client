# Vapor-Managed Git Distribution

**Design:** `docs/superpowers/specs/2026-11-26-vapor-managed-git-distribution-design.md`

**Goal:** Provision Git only for Developer mode under canonical Vapor User Data,
route every Git operation through Vapor, and make the blank-machine release
bootstrap work without ambient host Git. Preserve the Player setup contract and
the current `0.5.0-pre-alpha-<inner SemVer>` identity.

**Constraints:** Use Vapor-managed source, compiler, Cargo, and packaging
operations. Do not add Git to Player setup or Steam. Keep source checkouts under
`Vapor/sources`. The Windows payload is official pinned MinGit; Linux Git is
built from pinned upstream source. Do not publish until each payload's runtime
closure, exact corresponding source, notices, and redistribution terms are
verified.

## Task 1: Resolve exact payload inputs and licensing inventory

- Select supported upstream revisions and verify their release artifacts and
  digests.
- Inspect the actual MinGit archive and its package metadata to enumerate every
  executable, library, script, notice, and license it contains.
- Resolve Linux Git's required runtime closure using Vapor-managed build
  inputs; distinguish bundled libraries from normal host system libraries.
- Record component versions, origins, source hashes, license identifiers,
  required notices, patches, and build/packaging scripts in a generated
  per-platform inventory.
- Obtain complete source for each distributed component and preserve required
  license/copyright/third-party notices. Unknown license or missing source is a
  release blocker, not a warning.

**Proof:** Rebuild/source archives and inventory identify the precise binary
contents; every component has a license review and an available corresponding
source package. Do not make a broad legal-compliance claim from an upstream
homepage link alone.

## Task 2: Build and stage separate Git tool payloads through Vapor

- Add version-pinned acquisition/build recipes to Vapor-owned release tooling.
- Keep Linux source archives and native build intermediates under a
  Git-specific Vapor User Data cache namespace beneath
  `cache/build/native-tools/git`. Key reuse on the verified source digest,
  recipe revision, managed compiler/toolchain identity, target, and enabled
  build features; publish a completion marker only after version and staged
  inventory checks pass. Keep the activated Git provider under
  `tools/git`, outside the disposable build cache.
- Make cleanup categorize this cache as rebuildable generated state without
  conflating it with authored `sources/` checkouts or the installed `tools/`
  provider.
- Build Linux Git from upstream through Vapor's managed compiler and staged
  dependency closure; verify it runs in the supported Linux baseline.
- Verify and stage the official Windows MinGit archive without stripping its
  required notices or package structure.
- Produce separate platform archives for the Developer tool home and a
  source/legal bundle for each platform.
- Keep the payload outside the Player application archive and verify staged
  paths are contained, predictable, and version-addressed.

**Proof:** Both staged archives match their declared file inventories and
version reports; their runtime dependencies are explicit. Repeating the same
Linux build through Vapor reuses the keyed build tree; changing a key input
selects a different build tree. Clearing the native build cache leaves the
installed provider intact and allows Vapor to rebuild it.

## Task 3: Extend signed release metadata and Artifact delivery

- Extend the signed manifest with an optional managed-Git record containing
  platform, version, binary asset URL/size/hash, source bundle URL/size/hash,
  and notice/inventory URL/size/hash.
- Keep existing schema-1 app releases installable when this record is absent.
- Extend immutable Artifact storage/routes and preparation/publication checks
  so all three assets are promoted atomically and cannot be overwritten.
- Retain source and notice assets for at least as long as their binary is
  distributed; the Homepage exposes these links without adding source to the
  Developer install download.

**Proof:** Detached manifest signature covers all asset references; publisher
rejects missing, mismatched, or incomplete binary/source/legal assets.

## Task 4: Provision and own Git through Vapor

- Add a Vapor-owned Developer setup operation that downloads only the signed
  platform Git asset, verifies size and digest, safely stages it, then promotes
  it beneath `<Vapor User Data>/tools/git`.
- Preserve Player setup without Git; make Developer setup repair replace only
  Vapor-owned Git files and retain unrelated User Data.
- Resolve the installed provider explicitly through the existing Vapor Git
  provider boundary; fail clearly when unavailable, with no PATH/common-location
  fallback.
- Keep the explicit operator-linked provider only as an intentional override
  executed through Vapor.

**Proof:** Player setup skips Git; Developer setup provisions/repairs it; source
operations report and use the managed provider; a corrupt payload is rejected
before activation.

## Task 5: Close blank-machine bootstrap and release flow

- Resolve the first-transition bootstrap from the currently installed signed
  Vapor release, which predates managed Git. Any explicit provider used for
  seeding must stay a documented, narrow transition and must not become a
  hidden CI PATH fallback.
- Route Client and Platform source acquisition, version pinning, and release
  staging through Vapor under the canonical Superworkspace.
- Publish through the existing signed, immutable release pipeline only after
  build, licensing/source inventory, archive validation, and manifest checks
  succeed.
- Keep Client#3 blocked until clean-machine install, Developer Git setup,
  provider-mediated cloning, and the first public-download flow are proven.

**Proof:** A clean runner installs signed Vapor, bootstraps its own Git through
Vapor, checks out the exact pinned sources under `Vapor/sources`, and completes
the Vapor-owned build/stage/sign/publish flow. Confirm version format
`0.5.0-pre-alpha-<inner SemVer>` and visible source/license links on the
download page.

## Release compliance references

- [Git upstream license overview](https://git-scm.com/about)
- [Git for Windows MinGit scope](https://github.com/git-for-windows/git-for-windows.github.io/blob/main/content/mingit.md)
- [Git for Windows component licensing](https://github.com/git-for-windows/build-extra/blob/main/ReleaseNotes.md)
- [GNU GPLv2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)

This is a release-engineering checklist, not legal advice. A qualified review is
appropriate if the exact component inventory or intended redistribution terms
leave a material question unresolved.
