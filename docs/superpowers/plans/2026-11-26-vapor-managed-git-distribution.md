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

## Implementation checkpoint — 2026-11-27

Vapor SDK now exposes `sdk git build` for an x86-64 Linux shadow build. It
downloads pinned Git 2.56.1 and GNU Make 4.4.1 source archives, checks their
SHA-256 digests, builds with Vapor-managed Zig, and stores source/build output
under User Data `cache/build/native-tools/git`. The executable remains outside
`tools/git` and the report explicitly marks it ineligible for distribution.

The first build completed as Git 2.56.1; an unchanged rerun reused the same
cache key. The marker now records the actual shared libraries resolved by all
executable ELF files in the install tree, including canonical paths and content
digests, and cache validation rechecks that runtime closure. The current host
build scans 163 executable images and records 627 dependency references to 26
unique shared-library identities, so its HTTPS helper is still tied to this
host's system library set and is not a portable payload. Running through Vapor's
explicit-provider boundary also confirmed the helper-path contract: the
provider-managed install path supplies `GIT_EXEC_PATH`, while an explicit
external executable override does not. The shadow binary's compiled helper
path points to its staging directory, so it must only be exercised with the
managed helper environment until it is replaced by a package built for
`tools/git`.

This checkpoint proves Linux source acquisition, managed compilation, cache
reuse, and provider-mediated HTTPS. A later runtime check found that Git had
embedded the temporary `install.staging` path for its remote helpers. The
builder now configures the final immutable cache prefix, stages with
`DESTDIR`, and verifies `git --exec-path` before promotion and during cache
validation. A rebuilt shadow cache resolves `git-remote-https` at the promoted
location and completes provider-mediated HTTPS `ls-remote`; the provider was
selected by a one-command `VAPOR_GIT` override and was not activated under
`tools/git`.

That runtime-path fix does not close Linux runtime distribution, Windows Git,
component source and notice inventories, Developer setup provisioning, signed
payload metadata, or clean-machine bootstrap. The host-linked libcurl/OpenSSL
closure remains non-portable and distribution-ineligible. No Git payload was
activated or published.

## Portability and installed-tool checkpoint — 2026-11-27

The host-linked libcurl is not a distribution recipe: its installed build pulls
26 shared-library identities, including an Arch-specific glibc, OpenSSL,
Kerberos, IDN, compression, and HTTP/2/3 components. Recording that closure
improves cache correctness but does not make the payload portable.

The Rustls route is a useful experiment, not a selected release backend. curl
still labels its Rustls backend experimental, and rustls-ffi recommends static
linking because it does not promise dynamic ABI stability. See the upstream
[curl build options](https://github.com/curl/curl/blob/master/docs/INSTALL-CMAKE.md)
and [rustls-ffi linking guidance](https://github.com/rustls/rustls-ffi/blob/main/README.md).
Do not mark a payload distributable merely because static linkage reduces the
shared-library count. A shipping candidate still needs a maintained TLS path,
target-ABI proof, complete source bundle, and reviewed component inventory.

GnuTLS remains a stable-backend candidate for a source-built, minimized curl;
its upstream download page identifies Nettle and GMP as required and p11-kit,
IDN, and other support as optional. The current GnuTLS security page recommends
3.8.14 or later for advisories published on 2026-11-18. This narrows the
dependency question, but no GnuTLS build, source pin, license inventory, or
redistribution decision has been made. See [GnuTLS downloads](https://gnutls.org/download.html)
and [security advisories](https://www.gnutls.org/security-new.html).

The currently installed Steam CLI also cannot drive this new path: its
`sdk git` command is unrecognized. Its existing SDK build command reports
`missing_rustup`, while the shipped Vapor Installer reports the pinned Rust
toolchain ready under User Data. This is a live installed-version/tool-state
mismatch, not evidence that the source shadow builder failed. The current
source binary refuses to run directly from its build directory, as designed;
do not bypass that guard or invoke raw Cargo. Restore the supported
Vapor-managed self-deploy path before claiming the current installed product
can reproduce the new SDK operation.

No TLS backend was activated, no runtime tool was installed or published, and
no Steam payload changed during this investigation.

## Static Linux candidate checkpoint — 2026-11-27

The Linux shadow recipe now builds Git 2.56.1 and its HTTPS stack from verified
source archives with Vapor-managed Zig for `x86_64-linux-musl`: GNU Make 4.4.1,
zlib 1.4, OpenSSL 3.5.9, curl 8.23.0, and Git 2.56.1. The build explicitly
selects the pinned `curl-config` ahead of ambient host tools, handles OpenSSL's
`lib64` install layout, and recognizes static-musl `ldd` output on stderr.

Vapor Core and Vapor-CLI compiled successfully through Vapor-managed Cargo.
The source-built `sdk git build` then completed and produced Git 2.56.1 in the
disposable native build cache under cache key
`01905e0f15a9e17155a930cab7afd98652f2b1e99c322147595a7f14e7564eb7`. Its full
tree contains 163 executable ELF images and the runtime inventory confirms
each is statically linked for musl with no dynamic-library closure. A repeat
build reused the same validated cache entry. Through Vapor's explicit Git
provider, the candidate reports its version, reads the Vapor-Client checkout,
and successfully resolves the Git 2.56.1 tag over HTTPS.

This closes the Linux compiler/runtime portability experiment for this
candidate, not its redistribution gate. It remains in the disposable cache,
`distribution_eligible=false`, and is not selected under `tools/git`. Its full
corresponding-source bundle, build-script/source archive packaging, component
license and notice inventory, signed release references, and Developer setup
activation are still open. Windows MinGit/source inventory and clean-machine
bootstrap are also outstanding. No Steam payload changed and no tests were
run.

## Linux corresponding-source bundle checkpoint — 2026-11-27

The Linux shadow build now creates a cache-owned source bundle next to the
candidate. It packages the five exact hash-pinned upstream archives used by
the recipe (Git, GNU Make, curl, OpenSSL, and zlib), the license/notice files
extracted from those archives, a combined `THIRD_PARTY_NOTICES.txt`, and a
machine-readable `components.json` containing each source URL, version, digest,
build role, observed license-text identification, toolchain identity, feature
set, and notice-file digest. The archive uses normalized ordering, timestamps,
and owner metadata; its SHA-256 and byte count are recorded in the validated
cache marker and surfaced by `sdk git build`.

The recipe changed to `2026-11-27.16`. Vapor-managed Core/CLI compilation
passed, and the updated managed Git build produced a 68,884,480-byte bundle
with SHA-256
`cdcc76fff04c2113e9103cf21226291347f54182a3e83857ceb79270b18a72d5`. An
independent archive inspection found 41 unique safe members, verified all five
embedded source archive digests and all twelve extracted notice digests, and
confirmed a repeat `sdk git build` reused the same cache key
`9e2aa0b0c43ed5bc0a35f8468f345c820bbc702e0cbccde409195eeb1cd630fc`.

This is source-and-notice packaging evidence, not a completed component map or
legal review. The manifest explicitly says review is incomplete; the
candidate remains `distribution_eligible=false`, under disposable build
cache only. No tests ran, no provider was installed or published, and Steam
was untouched.

## Linux recipe-source and cache-identity checkpoint — 2026-11-27

The source bundle now includes the exact Vapor Core Git builder and bundle
generator source files under `recipe/`, with each file's byte count and SHA-256
in schema-2 `components.json`. Recipe source digests also participate in the
native-build cache key and are recorded in schema-5 cache markers, so editing
the builder invalidates old candidate output without relying only on a manual
revision bump.

Vapor-managed Core/CLI compilation succeeded. The recipe-source-keyed build
produced cache key
`412d5b73951ed310fcaef16cad2d08a1b9a1d4e4dcad472463fb22ab6b10c711` and a
68,956,160-byte source bundle with SHA-256
`db4b69e8ebb1f27ce5178385fc80ec51611a705d88e314a785b47656c59caeed`. An
independent inspection verified 46 unique safe archive members, all five
source archives, twelve notice files, and both recipe-source digests. A repeat
`sdk git build` reused the same cache entry.

The bundled recipe code and upstream sources do not complete the binary-to-
component map or legal review. The candidate remains cache-only and
`distribution_eligible=false`; Windows MinGit review, signed release
references, Developer provisioning, and blank-machine bootstrap remain open.
No tests ran, no provider was installed or published, and Steam was untouched.
