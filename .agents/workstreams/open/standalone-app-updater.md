# Workstream — Standalone App Updater

- State: suspended as a component of `standalone-vapor-runtime`
- Goal: implement the client-side signed Vapor app release verifier and safe update transaction for the independent public release feed.
- Ordered successor: continue through the signed update transaction tasks in `standalone-vapor-runtime`.
- Depends on: standalone distribution design and the Platform/Artifact Server release contract.
- Semantic mutation claim: standalone Vapor app release authenticity and update lifecycle, separate from Steam/Workshop content.
- Path claims: none independent; `Vapor-Installer/crates/vapor_installer/**` and the owned uncommitted changes below have transferred to `standalone-vapor-runtime`.
- Current source check (2026-11-24): this live Linux checkout has no app-release verifier module, verified-release type, or network/update transaction in `Vapor-Installer/crates/vapor_installer/src`; only the Platform Artifact API and release preparation/storage contract are present. The older completion, build, and Windows-home notes below this workstream's parent record cannot be reproduced from the current checkout and are not treated as current implementation evidence.
- Completed: the Platform contract defines signed Ed25519 manifests, immutable versioned archives, and SHA-256/size verification. Homepage reads the stable Artifact feed but explicitly labels current downloads as archives and says the guided installer/update path is unfinished.
- Remaining: implement manifest/signature verification in the live Installer if not recoverable from another authorized checkout; verify bounded archive download; reject unsafe archive entries; stage, validate, activate, and roll back app versions; add a signed release-history path for version selection; connect CLI/GUI lifecycle and stable bootstrap behavior.
- Blockers: no production public trust key or complete stable release/bootstrap has been verified. No Artifact or website publication is authorized by this workstream checkpoint.
- Owned uncommitted changes: none specific to a release verifier in this live checkout. Installer path, role, and erase-all changes are owned by `standalone-vapor-runtime`.
- External actions: none for this updater component. No release was signed or published.
- Smallest resume action: continue the standalone runtime plan from its live Artifact API and inspect the current Installer crate before implementing the signed verifier.

