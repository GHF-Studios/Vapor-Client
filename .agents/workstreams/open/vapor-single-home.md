# Workstream — Standalone Vapor Single Home

- State: foreground
- Goal: make `C:\Users\David Jackson\Vapor` the single Windows Vapor root and
  `$HOME/Vapor` the single Linux Vapor root; move Vapor-owned Steam app files
  into it, route all Vapor operations to it, consolidate verified duplicates,
  and keep the Loo Cast Steam mount limited to its Packagepack product.
- Ordered successor: resume `workshop-content-superpass` after a coherent
  standalone-home migration checkpoint.
- Depends on: owner-approved single-root layout in the current conversation.
- Semantic mutation claim: the Vapor home marker/path contract, managed
  toolchain and build cache paths, standalone install/update/uninstall,
  Steam-root migration, and duplicate checkout/cache consolidation.
- Path claims: `Vapor/crates/vapor_core/src/{installation,toolchain,development,cargo,run}.rs`,
  `Vapor-CLI/**`, `Vapor-Installer/**`, `Vapor-GUI/**`,
  `Vapor-Steam-Entrypoint/**`, `Vapor-Studio/**`, `resources/vapor/tools/**`,
  root setup manifests/docs, Loo Cast Steam staging inputs, and this record.
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
