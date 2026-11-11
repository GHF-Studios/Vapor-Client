# Vapor Installer Clean Rebuild Implementation Plan

> **For agentic workers:** This plan is being executed inline by the current agent.

**Goal:** Make Windows setup register Vapor correctly, simplify installer setup to Player/Developer, and make a fully scoped Vapor erasure available for a clean reinstall.

**Architecture:** Keep the Rust/egui Installer UI and Vapor Core ownership model. Correct OS integration in Vapor Core, simplify the role model, and run full erasure through a temporary external helper so the running installer can close before its home is removed.

**Tech Stack:** Rust 2024, Vapor Core, `eframe`/`egui`, PowerShell only for Windows registry integration and the post-exit cleanup handoff.

**Spec:** `docs/superpowers/specs/2026-11-10-installer-clean-rebuild-design.md`

## Global Constraints

- User explicitly authorized erasing all Vapor data, including content and settings.
- Keep Steam-owned Packagepack/game files, the USB recovery copy, and unrelated user tools outside the cleanup boundary.
- The managed matrix is `dev`, test-only `test`, and CI-only `release`, crossed with exactly `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-gnullvm`. Release uses thin LTO; ambient profile/flag overrides are scrubbed; target-specific linkers are selected by Vapor's cross-toolchain provider. Only the designated CI distribution job may upload Steam/Workshop artifacts.
- Player setup stays unprivileged. Developer setup may request one explicit OS elevation for a named host prerequisite; Vapor-owned Rust and cross tools remain app-local. Prefer a pinned, app-local sysroot for cross-target native dependencies.
- Preserve unique checkout work until committed/pushed or copied to recovery storage.
- Vapor-owned authored checkouts live under `Vapor Home/sources`. Source selection may open those repositories, but must reject source roots anywhere else inside Vapor Home.
- Do not add or run tests in this pass; build and inspect concrete behavior instead.

## Review Focus

- Windows paths containing spaces, apostrophes, and trailing separators must bind as data and survive the user PATH update.
- PATH update must be idempotent and preserve unrelated entries.
- Unsupported or malformed install state must not display a successful setup result.
- Erase targets that are missing, redirected, or outside the known ownership roots must be rejected before any deletion.
- The cleanup helper must not race the running GUI or leave an executable locked inside the removed home.

---

### Task 1: Repair Windows command registration

**Files:**
- Modify `Vapor/crates/vapor_core/src/install.rs`
- Modify `Vapor/crates/vapor_core/src/uninstall.rs`

**Interfaces:**
- Keep the existing public install/uninstall functions.
- Add private PowerShell literal/verification helpers as needed.

- [x] Pass `root` and `bin` as explicitly bound script-block parameters, not loose values after a `-Command` string.
- [x] Verify the user PATH and `VAPOR_HOME` values after writing; fail the operation on mismatch.
- [x] Broadcast the Windows environment change.
- [x] Remove Vapor PATH entries for all supported Windows target directories under this installation root.
- [x] Build Vapor Core through the managed Vapor command and verify the real user PATH after invoking the new installer.

### Task 2: Reduce local roles and installer navigation

**Files:**
- Modify `Vapor/crates/vapor_core/src/role.rs`
- Modify `Vapor-Installer/crates/vapor_installer_ui/src/lib.rs`
- Update installer documentation where it lists role choices.

**Interfaces:**
- `VaporRole` exposes only `Player` and `Developer`.
- Developer promotion reconciles the full developer environment; Player setup does not.

- [x] Replace the four-option role enum, labels, parser, and policy with Player/Developer.
- [x] Replace the Overview/Role/Installation tab flow with one lifecycle view and one destructive section.
- [x] Show the current role, developer-tool readiness, one contextual primary action, and actual operation result.
- [x] Treat existing old role-state versions as unsupported; do not add compatibility aliases.

### Task 3: Add preflighted full erasure

**Files:**
- Modify `Vapor/crates/vapor_core/src/uninstall.rs`
- Modify `Vapor-Installer/crates/vapor_installer/src/cli.rs`
- Modify `Vapor-Installer/crates/vapor_installer_ui/src/lib.rs`

**Interfaces:**
- Installer exposes an inventory operation returning named paths and ownership status.
- Erasure accepts only inventoried, canonical, Vapor-owned paths.
- Windows post-exit helper accepts the installer process ID and the preflighted path set.

- [x] Inventory canonical Home, Vapor Local/Roaming state, and confirmed external legacy tool roots.
- [x] Show exact paths and irreversibility in the UI before execution.
- [x] Stage a hidden helper outside the home, wait for installer exit, revalidate marker/path containment, erase owned roots, and remove the helper script.
- [x] Preserve Steam-owned game/Packagepack files and unrelated user tools.
- [x] Expose the same inventory and erase operation through `vapor-installer erase`, with explicit app root and `ERASE VAPOR` confirmation.

### Task 4: Prepare the clean rebuild

**Files:**
- Update the workstream record and operator-facing installer documentation.

- [x] Commit/push the core and installer branches, then update the Client submodule pins.
- [x] Preserve or publish unique dirty work from every checkout before wiping any checkout; all retained checkouts are then created under the canonical `Vapor Home/sources` root.
- [x] Run the installer inventory and inspect its exact path list.
- [x] Execute the authorized full erase and reinstall from the built standalone installer/package source. The clean wipe was followed by the verified normal install/uninstall/install cycle, which preserves Home and source while removing and restoring the Windows user PATH integration.
- [ ] Verify a fresh Player install and a Developer install through the installer UI; report any handoff that still requires manual action.

The UI verification remains open: the Developer toolchain is ready and the installed CLI has been exercised, but a full first-launch Player/Developer walkthrough in the GUI has not been confirmed.
