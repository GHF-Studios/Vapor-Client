# Vapor Installer and Clean Rebuild Design

## Intent

Make Vapor setup, repair, developer-tool provisioning, removal, and clean reset
behave as one understandable lifecycle. Fix the Windows command registration
failure at its source and make the installer show outcomes that it has actually
verified. Support a deliberate clean rebuild that can erase Vapor-owned local
data after showing the exact scope.

## Goals

- Offer only the agreed local roles: **Player** and **Developer**. Developer
  always includes the managed development toolchain.
- Keep ordinary Player setup and runtime unprivileged. Developer setup may ask
  the operating system for elevation only when a specific, named host
  prerequisite requires it; show the exact packages and reason before asking.
  Installing Vapor's own Rust, Zig, LLVM-MinGW, and wrappers remains app-local
  and must not require elevation.
- Keep the managed build matrix to three profiles (`dev` for local development,
  `test` for tests, and `release` for CI distribution) crossed with the two
  supported target triplets. `dev` uses line-table debug info; `test` uses
  Cargo's standard test profile and is never distributable; `release` uses
  thin LTO and is restricted to the designated CI distribution job.
  The target rows select their managed linker/compiler toolchain: Zig for
  Linux cross-builds and LLVM-MinGW for Windows GNU builds. CI alone uploads
  SteamPipe or Workshop content.
- Vapor-managed Cargo processes remove ambient profile, rustflag, target, and
  compiler-wrapper overrides and default to the Vapor-owned cache directory
  for the host triplet. Cross-target commands may then select one of the two
  supported triplets explicitly.
- The supported runtime matrix is exactly `x86_64-unknown-linux-gnu` and
  `x86_64-pc-windows-gnullvm`; reject arbitrary triples before invoking Cargo.
  Both targets always select the same Vapor-managed linker toolchain regardless
  of the host: Zig for Linux and LLVM-MinGW for Windows.
  Target- and product-specific compiler/linker arguments are allowed when they
  are narrowly scoped and required; they must not leak into unrelated targets
  or products. Loo-Cast's Windows target additionally enables static CRT
  linkage. Mad Max's ASI is a separate Windows MSVC product ABI and is not a
  third Vapor runtime target.
- Cross-host coverage is intended to be a full 2×2: Windows or Linux build
  host can build both Linux and Windows runtime targets. The target, not the
  host, selects Zig or LLVM-MinGW. This does not claim every product dependency
  is already cross-compilable. Cross compilation does not erase native
  dependency requirements: crates that probe Linux system libraries through
  `pkg-config` need a matching sysroot and target-aware `pkg-config`, or a
  native Linux builder.
- Build matrix:

  | Profile | Linux target | Windows target | Who may produce it |
  | --- | --- | --- | --- |
  | `dev` | Zig, line-table debug info | LLVM-MinGW, line-table debug info; Loo-Cast keeps static CRT | Local Developer |
  | `test` | Same target toolchain; test harness, cross-target uses `--no-run` | Same target toolchain; test harness, cross-target uses `--no-run` | Local or CI validation; never distributable |
  | `release` | Zig, thin LTO | LLVM-MinGW, thin LTO; Loo-Cast keeps static CRT | Designated CI distribution job only |
- Compiler flags are scoped to the narrowest real ABI boundary. They do not
  create extra Cargo profiles or justify adding extra supported Vapor targets.
  A build must fail clearly when the chosen host/toolchain lacks a required
  target sysroot or native dependency metadata.
- Prefer an app-local, pinned target sysroot for cross-target native libraries
  so building Linux outputs from Windows does not mutate the Windows host. For
  native Developer prerequisites, use a visible, one-operation OS elevation
  prompt; never silently invoke `sudo`, alter package sources, or leave an
  elevated background service running. Player setup never offers this path.
- The Mad Max ASI lane remains `x86_64-pc-windows-msvc`, because it injects into
  the game's Windows process and follows that loader ABI. It has its own target
  flags and build output; it is not part of Vapor's two-target runtime matrix.
- Register the Windows Vapor executable directory in the current user's PATH
  idempotently, broadcast the environment change, and verify the stored value.
- Remove PATH entries for the active Windows ABI and any prior supported ABI
  when uninstalling.
- Replace redundant navigation tabs with one compact lifecycle page: detected
  install, chosen role, readiness, primary setup/repair action, and a separate
  destructive full-erasure action.
- Make full erasure remove the Vapor Home, Vapor Local/Roaming state, known
  Vapor-managed legacy tool roots, and generated local data after the UI exits.
  Show the full path list and an irreversible confirmation first.
- Allow source roots beneath `Vapor Home/sources` while keeping runtime, data,
  cache, tools, and binary directories invalid as authored source roots.
- Keep actual Steam game/Packagepack files, the USB recovery copy, and unrelated
  user tools outside the erase boundary.

## Non-goals

- Redesign the Vapor product updater or Steam content lifecycle.
- Delete unrelated Rust installations or arbitrary directories merely because
  their names resemble Vapor-managed locations.
- Run the erase helper in the foreground while deleting the executable that is
  currently running.

## Approaches considered

1. Patch only the PowerShell arguments. This fixes one visible bug but leaves
   stale roles, duplicate screens, and a non-functional clean-rebuild path.
2. Keep the Rust/egui installer and redesign it around setup, repair, role, and
   erase lifecycle actions. Use a short-lived external cleanup handoff for the
   final deletion so the running installer can remove its own home. **Chosen:**
   it preserves the existing cross-platform installer surface and directly
   addresses the user's stated clean-rebuild workflow.
3. Replace the Windows path with an MSI/WiX-only installer. This may provide a
   more native Windows shell experience but creates a separate deployment
   system and does not solve the shared Linux workflow; it is disproportionate
   for this pre-alpha pass.

## Design decisions

- PowerShell arguments are passed through a single script-block invocation with
  explicitly bound parameters and safely quoted values; no values are appended
  as free command text. After writing the user environment, the installer
  verifies `VAPOR_HOME` and PATH and broadcasts `WM_SETTINGCHANGE`.
- Role state is reduced to `player` and `developer`. Because this is pre-alpha,
  the role-state format changes without legacy-role aliases; clean rebuild
  removes old role state.
- Normal removal remains distinct from full erasure. Full erasure first builds
  a path inventory and then stages a hidden helper outside Vapor Home, waits for
  the UI process to exit, revalidates every path against the known ownership
  roots, and deletes only those roots.
- The full-erasure inventory includes the canonical Vapor home and the known
  Vapor Local/Roaming state/tool directories. External legacy tool roots are
  included only when their ownership can be established by the inventory; the
  UI must not silently broaden deletion based on fuzzy name matching.
- The Loo Cast Steam product remains installed and is not removed by a local
  Vapor reset. Its Steam-owned game payload is separate from Vapor Home.

## Open implementation details

- Reconcile the UI's current first-launch handoff with the Player/Developer
  setup and current Packagepack launch behavior.
- Decide, from live ownership evidence, whether `C:\VaporTools` and the old
  global Cargo/Rustup homes belong in the full-erasure list or need explicit
  user selection.
- Ensure the short-lived helper reports completion and removes its temporary
  script after it has deleted Vapor Home.
