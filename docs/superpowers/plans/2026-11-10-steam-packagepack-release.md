# Steam Packagepack Release Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Let Vapor stage and validate the Loo Cast Steam app as one Packagepack product per platform, with only its shared Packagepack manifest and explicitly allowed runtime files.

**Architecture:** Add a required product kind to `root.steam`; retain the existing Vapor-runtime smoke contract and add a Packagepack contract. Packagepack app releases skip Vapor-runtime docs and app-root asset synchronization, while continuing to use the current review, fingerprint, depot, and SteamPipe pipeline.

**Tech Stack:** Rust, serde/TOML, Vapor Client provider and application publishing crates, Steam depot staging.

**Spec:** `docs/superpowers/specs/2026-11-10-steam-packagepack-product.md`

## Global Constraints

- Common depot contains `Packagepack.vapor.toml` only.
- Each platform depot contains one logical Packagepack executable and may contain only explicitly allowlisted required runtime sidecars.
- Vapor runtime files, `App.vapor.toml`, docs, and the source manifest are excluded from the Packagepack payload.
- Do not upload to Steam during implementation or local validation.

## Review Focus

- Missing or unknown product kind fails closed.
- Packagepack payload missing its common identity or target executable is rejected.
- A second executable or unrelated file in a Packagepack depot is rejected; required `.dll`/`.so` sidecars remain allowed.
- Vapor-runtime smoke and staging behavior is unchanged.
- Packagepack release staging does not build Vapor documentation or synchronize Vapor app assets.

---

### Task 1: Declare and parse the Steam product kind

**Files:**
- Modify: `Vapor/crates/vapor_client_provider/src/distribution.rs`
- Modify: `App-Source.vapor.toml`
- Test: `Vapor/crates/vapor_client_provider/src/distribution.rs`

**Interfaces:**
- Produces `SteamProductKind::{VaporRuntime, Packagepack}` and
  `Application::product() -> SteamProductKind`.

- [x] Add tests that parse `packagepack`, parse `vapor-runtime`, and reject a missing or unknown product kind.
- [x] Run those tests and confirm they fail before the implementation.
- [x] Add the enum and required field; mark the Loo Cast source as `packagepack`. The Vapor Client source no longer declares a Steam product because Vapor is not to ship in the Loo Cast depot.
- [x] Run the product-kind tests.

### Task 2: Enforce product-specific depot contents

**Files:**
- Modify: `Vapor/crates/vapor_client_provider/src/steam.rs`
- Modify: `Vapor/crates/vapor_client_provider/src/distribution.rs` if staged-file inspection needs a test constructor.
- Test: `Vapor/crates/vapor_client_provider/src/steam.rs`

**Interfaces:**
- `steam::publish` passes `Application::product()` into payload smoke validation.

- [x] Add staged-fixture tests for a valid Packagepack payload, missing manifest, missing executable, extra executable, unrelated file, and allowed runtime sidecar.
- [x] Keep the runtime smoke branch unchanged; add Packagepack validation requiring `Packagepack.vapor.toml`, one target executable, and only explicitly staged runtime sidecars.
- [x] Run the Packagepack fixture tests: 5 passed.
- [ ] Run the complete `vapor_client_provider` test suite. Attempted; 54 passed and 7 other Windows/environment-sensitive tests failed in the current working tree (path normalization, source fixture layout, linker setup, and target selection). The Packagepack tests all passed.

### Task 3: Keep Packagepack release staging independent of Vapor runtime assets

**Files:**
- Modify: `Vapor/crates/vapor_client_application/src/publishing.rs`
- Modify: `App-Source.vapor.toml`
- Modify: `README.md`

- [x] Skip Vapor docs generation and app-root runtime asset synchronization only for `Packagepack` releases.
- [x] Move the Loo Cast `App-Source.vapor.toml` to its source root so its allowlist can reference the Packagepack manifest without escaping the source boundary.
- [x] Point target includes at the Packagepack executable promoted under the Vapor installation's `bin/<target>` path; allow runtime sidecars only as explicit includes.
- [x] Embed SteamPipe templates in Vapor's publisher so staging needs neither files in the game checkout nor additional Steam-depot payload.
- [x] Run a Packagepack staging dry-run: exactly 3 files—shared `Packagepack.vapor.toml`, Linux executable, Windows executable; no Vapor runtime/doc payload.
- [x] Run all 17 `vapor_client_application` tests and the 5 Packagepack fixture tests; `git diff --check` passes in the Client, Vapor, and Loo-Cast checkouts.
