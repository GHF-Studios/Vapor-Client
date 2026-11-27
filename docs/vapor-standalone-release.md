# Standalone Vapor release pipeline

The `vapor-stable-release` workflow builds and stages the standalone Vapor
client through Vapor's signed bootstrap and managed toolchain. It does not use
Steam to build or publish the client.

Pushes to `dev` create an unsigned candidate archive for Windows and Linux.
The candidate is a short-lived GitHub Actions artifact, retained for one day;
this path has no access to the release-signing or publisher credentials. The
Windows package step checks that `vapor-setup.exe` has no dynamic
`libunwind.dll` import before the candidate can be uploaded.

Pushes to `main` run the same build and packaging path, then a separate job uses
the `vapor-stable-release` protected environment to sign the immutable release
bundles and publish them through the restricted SSH publisher. Publication
cannot start unless the candidate job succeeds. A workflow run on `dev` stops
after candidate upload.

Source revisions are acquired through Vapor's source command. The runner's
read-only `GITHUB_TOKEN` is exposed only to that acquisition step and is used
as the Git credential; it must have read access to the private repositories
and submodules needed by the pinned Client and Platform sources. Do not replace
it with a broad repository secret. If the token cannot read a required
repository, configure narrowly scoped Actions access for the Client
repository's token rather than making the release credentials available to the
build job.

Optimized distribution builds are restricted by Vapor to this designated CI
job. Do not reproduce them with a local Cargo invocation or a local override
of Vapor's distribution guard.
