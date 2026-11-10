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
- Current checkout evidence: the Vapor home source checkout, Documents checkout,
  and Codex worktree each contain distinct staged/unstaged/untracked work. Do
  not delete any of them until each change is committed, transferred, or
  explicitly abandoned. Canonical source destination is
  `%USERPROFILE%\\Vapor\\sources\\Vapor-Client` on Windows and
  `$HOME/Vapor/sources/Vapor-Client` on Linux.
- Remaining: implement and build the home path contract; route all managed
  writers; preview and migrate Steam-root directories; compare and consolidate
  source/toolchain/cache duplicates; update docs; build supported targets and
  inspect the final Steam Packagepack staging inventory.
- Verification: initial filesystem inventory only. No migration or duplicate
  deletion has run. The requested old Platform Server `target` removal was
  blocked by automatic review; no files were deleted. The Cleanup install build
  target remains available under the temporary candidate root.
