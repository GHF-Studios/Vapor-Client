# Workstream — Standalone Vapor Single Home

- State: suspended
- Goal: make `C:\Users\David Jackson\Vapor` the single Windows Vapor root and
  `$HOME/Vapor` the single Linux Vapor root; move Vapor-owned Steam app files
  into it, route all Vapor operations to it, consolidate verified duplicates,
  and keep the Loo Cast Steam mount limited to its Packagepack product.
- Ordered successor: resume `workshop-content-superpass` after a coherent
  standalone-home migration checkpoint.
- Depends on: owner-approved single-root layout in the current conversation.
- Semantic mutation claim: none active; completed code claims are released.
- Remaining path claim: none. The only remaining item is a visible manual
  Player/Developer GUI walkthrough by the owner; source follow-up may reopen
  this workstream if that walkthrough reveals a defect.
- Decisions: one `VAPOR_HOME` root; `bin`, `tools`, `sources`, `cache`,
  `content`, `config`, `data`, and `output` are children of that root. Cargo and
  Rustup use homes under `tools`; one shared Cargo target cache lives under
  `cache/build/<target-triple>`. Preserve the current build target until the
  managed home cache is usable. Steam is a product mount, never Vapor's home.
- Completed: storage inventory across the known Vapor roots; measured duplicate
  Cargo/Rustup homes and target outputs; inspected checkout states; documented
  the single-home layout and migration plan.
- Current checkout evidence (2026-11-11): authored checkouts are now under
  `C:\\Users\\David Jackson\\Vapor\\sources`; the Client, Loo-Cast,
  Registry, and Platform checkouts report clean working trees. The two old
  `Documents\\GitHub` folders and two Codex worktree folders have no source
  files (the Codex folders contain only worktree marker files). There is no
  remaining 40 GiB Codex checkout/build output to delete. The repository's
  `git worktree list` reports only the canonical Client checkout. Keep new
  authored checkouts under `Vapor Home/sources` on both platforms.
- Remaining: implement and build the home path contract; route all managed
  writers; preview and migrate Steam-root directories; compare and consolidate
  source/toolchain/cache duplicates; update docs; build supported targets and
  inspect the final Steam Packagepack staging inventory.
- Current focus (2026-11-10): repair the Windows installer PATH registration,
  replace its four-role/tab-heavy flow with the agreed Player/Developer
  lifecycle, and provide a truthful full-erasure path for a clean rebuild.
  Full erasure is explicitly authorized to include Vapor content and settings;
  preserve the separate Steam Packagepack/game payload, unrelated user tools,
  and the USB recovery copy. Do not erase checkouts until their unique changes
  are committed/pushed or backed up.
- Evidence: the Windows installer passes root/bin values after PowerShell's
  `-Command` script text while the script expects `$args`; a local PowerShell
  reproduction parsed those values as command arguments, and the Windows user
  PATH currently lacks the Vapor binary directory. The UI still exposes the
  retired Composer/Content Developer/Ecosystem Developer role set.
- Steam entrypoint finding (2026-11-10): the Steam locator currently treats its
  Steam app root as Vapor Home, looks for the CLI/service/GUI/receipt beside
  itself, forces child `VAPOR_HOME` to that root, and logs under that root.
  This is why it fails after the standalone move and recreates Vapor folders
  inside Steam. The agreed repair is a tiny Steam locator that resolves the
  marked standalone Vapor Home, routes Play and the tools selector there, shows
  the official download site if Vapor is absent, and writes no Vapor state to
  Steam. See `docs/superpowers/specs/2026-11-10-steam-entrypoint-home-boundary.md`.
- Steam locator implementation (2026-11-10): the Steam Entrypoint source now
  resolves Vapor Home independently, launches Vapor surfaces from
  `Vapor Home/bin/<target>`, shows an actionable missing-install message, and
  logs only to `Vapor Home/data/logs`. The Loo Cast source allowlist and Vapor
  Packagepack smoke contract now include the matching locator. Windows and
  Linux locators were built with Vapor Home's managed toolchain, promoted into
  `Vapor Home/bin`, and the Windows locator was copied from that installed
  location to the Steam launch path. No Steam launch option targets Cargo
  output. Source commits pushed: Entrypoint `54e8f2d`, Vapor core/provider
  `fec1480` and `d6c7af4`, Loo Cast `165d3c8`, Client docs/gitlinks `0c89ff1`.
- Local deployment blocker: `vapor client deploy local` now discovers the
  sibling first-party workspaces within the selected Vapor-Client checkout,
  but full promotion still fails while Cargo compiles `ring` for the Windows
  MSVC target (`curve25519.c` is rejected by `cl.exe`). The Steam locator itself
  is already promoted; do not point Steam at a target directory. Repair the
  general deployment/toolchain environment separately before claiming the
  entire Vapor runtime has been freshly redeployed.
- Verification: both locator targets compiled, were copied into the canonical
  Vapor Home target directories, and the Windows copy matches the Steam launch
  path byte-for-byte. Full `vapor client deploy local` remains blocked by the
  MSVC `ring` build error above. No test suite or Steam depot upload was run.

- Clean-rebuild follow-through (2026-11-11): committed Core/Installer fixes
  and updated Client submodule pins are present in the canonical checkout.
  Vapor Installer's normal install → uninstall → install cycle was exercised;
  uninstall removed the user PATH/VAPOR_HOME integration while preserving
  Vapor Home and source, and reinstall restored the integration. The user PATH
  contains one canonical app-local command directory. The installed `vapor`
  executable runs, and `vapor-installer dev-env status` reports the managed
  Developer toolchain ready. The recovery USB kit's Windows installer and one
  support script were refreshed from Vapor Home; their hashes match the
  regenerated 44-entry checksum manifest. A fresh Player/Developer walkthrough
  through the Installer GUI remains unverified.
- Storage snapshot (2026-11-11): Codex worktree markers contain zero file bytes;
  Documents GitHub stubs contain no source files; `C:\\VaporTools` is absent.
  Vapor Home is about 6.7 GiB, of which 1.85 GiB is generated `cache/build`
  output. A deletion attempt was rejected by the host policy and made no
  change. The Loo Cast Steam directory still has about 4 GiB under `content`
  plus `.vapor` and other generated files; relocating it requires the
  Steam-mount/content path migration, so it remains an open item.
- Clean-room reinstall (2026-11-11): used the USB recovery payload to erase all
  Vapor Home data, reinstall the Player app, install the Developer toolchain,
  and recreate the four locked first-party checkouts under `Vapor Home/sources`.
  Installer and Developer status both report ready; Rust, the two supported
  targets, Zig, LLVM-MinGW, and SteamCMD are managed under Vapor Home. The
  global `%USERPROFILE%/.cargo` and `.rustup` copies remain because the host
  rejected the requested deletion; no alternate deletion path was used.
- Steam/Home boundary fix (2026-11-11): Steam launch wrappers now resolve the
  independent Vapor Home, route GUI/game/shell launches there, and put logs in
  `Vapor Home/data/logs`; the installer and entrypoint now read the Player
  receipt from `data/state/installer/player.toml`. The source-built deployment
  initially hit a Windows executable lock when it tried to replace the CLI
  that launched it. Re-running from the separate installed `vapor-next.exe`
  succeeded and promoted all ten client binaries into Vapor Home. Next: sync
  the Steam locator/wrapper from those installed files and exercise the Steam
  wrapper without launching a game; the old Steam `.vapor`/runtime/content
  directories are not yet migrated or removed.
- Source-index correction (2026-11-11): the Superworkspace root had also been
  persisted as an authored source, producing an “unavailable source” row after
  reinstall. Core now keeps the Superworkspace identity separate from its
  authored workspaces. Repaired the Home state to retain the Superworkspace
  root and only the valid Vapor and Loo-Cast workspaces; `vapor source status`
  and `source list` now report that clean state.
- Steam wrapper smoke check (2026-11-11): copied the promoted Home wrapper and
  locator into the Steam launch path, then ran `client --help` through that
  wrapper. It invoked the Home CLI and wrote its log under Home `data/logs`;
  no Steam-local log was created. Their SHA-256 hashes match their Home copies.
- Developer setup UI progress (2026-11-11): the first-launch and role-promotion
  paths now forward the Client service's live operation stage into the
  Installer status line while the Developer toolset installs. The Windows
  source-authoritative deployment compiled and promoted all ten client
  binaries after this change. A visible first-launch GUI walkthrough remains
  unverified; do not equate the build with a completed UI interaction check.
- Developer provisioning convergence (2026-11-11): GUI first-launch and role
  promotion now call the same app-root Developer setup routine as the CLI,
  including both Rust targets, Zig, LLVM-MinGW, and cross-wrapper setup; its
  stage messages are forwarded into the Installer status line. The Client
  service then performs its role reconciliation and Tracy setup. The managed
  Windows deployment promoted all eleven binaries, including the previously
  omitted `vapor-linux-cc.exe`; a Linux-target Cargo check of `vapor_installer_ui`
  passed using the Vapor-managed Rust/Cargo toolchain and target cache. A visible
  GUI walkthrough remains unverified.

- Recovery payload refresh (2026-11-11): `E:\VaporRecovery\Bootstrap\20261111-clean-rebuild`
  now records Client commit `96e3ff9611e2140ea9f7f9c2ee27b82542d0c8ed` and its
  pinned submodules. Its payload contains the 14-file managed Windows runtime,
  Linux `vapor-installer`, `vapor-installer-gui`, and `vapor-linux-cc` artifacts,
  the current 22-file Vapor tools resource tree, and current Home/Packagepack/
  toolchain manifests. All 17 binary files match their Vapor Home/build-cache
  source hashes, the resource file set matches, and all 46 snapshot checksum
  entries verify. The Linux artifacts were cross-built from Windows and were
  not runtime-tested on Linux. Snapshot size is about 1.60 GiB.
- Checkout cleanup recheck (2026-11-11): the two Codex worktree attachment
  directories contain only zero-byte marker files (no source or build output).
  Codex's archive operation and direct cleanup were blocked by host policy, so
  those negligible metadata remnants remain. `Documents\GitHub` contains no
  source content in its old Client/MM folders and only 2.7 KB of `.idea` files
  in the old Platform folder. C: has about 152.5 GiB free; the claimed ~40 GiB
  of duplicate Codex checkout data is not present.

- Alpha v0.5 handoff (2026-11-11): single-home implementation is paused at a
  reviewable checkpoint. The clean reinstall has been executed through the
  CLI/recovery path; the visible GUI walkthrough remains unverified and is
  retained as a manual follow-up. Overlapping source claims are released to
  the Alpha v0.5 journey workstream.
