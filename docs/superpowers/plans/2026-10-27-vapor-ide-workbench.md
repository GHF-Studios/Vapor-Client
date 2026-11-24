# Vapor IDE Workbench Implementation Plan

> **For agentic workers:** Implement inline in this task. Keep this plan and the approved design in view; do not delegate.

**Goal:** Build Vapor IDE into a serious Rust-and-Atlas authoring workbench: create code through a small fixed set of configurable templates, declare and review knowledge links at item/module scope, navigate the code/knowledge graph in both directions, and make large-scale human migration tractable. Fix the current Explorer and tool-window layout as part of the product work, not as a substitute for it.

**Architecture:** Extend `vapor_ide_ui` and `vapor_gui_core`; keep workspace, Atlas indexing, validation, and write operations behind Vapor Client service contracts. The Atlas repository is the canonical authored knowledge product and Git source. Hosted Atlas query/publication belongs in a distinct `Vapor-Atlas-Server` service nested in `Vapor-Platform-Server`; the existing `Vapor-Docs-Server` is a deprecated legacy documentation path, not Atlas's data store. Do not fake persistence in the GUI: define and implement one end-to-end annotation/review contract with explicit checkpoints for unresolved details.

**Tech Stack:** Rust, eframe/egui, existing `vapor_client` protocol and service, current Rust Analyzer/LSP integration.

**Spec:** `/tmp/vapor-ide-big-rework-design.md` plus the owner corrections captured here. The prior UI-heavy sequence was insufficient and is superseded by this plan.

## Global Constraints

- Keep the existing native eframe product and shared `vapor_gui_core`.
- Keep all workspace and managed build/run operations on Vapor Client service contracts; do not call raw Cargo from UI code.
- Template vocabulary remains small, defined in Rust, and extended only as a coordinated product change.
- Authors provide semantic meaning; the IDE may derive structure and indexes but never invent relationships.
- Atlas record hierarchy is homogeneous by record type; cross-kind relations are separately typed. Relation state/validity may be explicit. `supersedes` means a distinct record replaces another, not that an existing record was edited.
- Multiple scoped reviews may coexist. Reviews bind to the source/relationship revision they checked; affected edits invalidate relevant reviews, not every review globally.
- Existing Rust is migrated deliberately by humans through IDE-supported workflows; no silent repository-wide conversion is implied.
- Atlas is read-only when published. Authored records and code annotations change in their versioned source repositories, then validate and publish as derived views.
- Opening a workspace must not trigger builds or tests.
- Preserve unrelated dirty submodule state (`Vapor` and `Vapor-Installer`).
- Use the Vapor-managed validation workflow; no raw Cargo or broad formatter invocation.

## Review Focus

- Small or narrow windows: tool windows must collapse or scroll without hiding editor actions.
- Problems height stays fixed by default and user-resizable only at its divider; contents scroll and filters never wrap into pane height.
- Wide-layout Explorer uses available width; unrelated folders are collapsed by default, with the selected path or active search expanded.
- Slow or unavailable Rust Analyzer: editing and file navigation remain usable and status explains degraded capability.
- Dirty buffers during navigation, template preview, and tool-window changes: no implicit discard or write.
- Stale LSP ranges after edits: navigation and code actions must reject or refresh stale locations.
- Missing or malformed project metadata: welcome and workspace states explain recovery without hanging or starting builds.

---

### Task 1: Establish a testable IDE presentation model

**Files:** `Vapor-GUI/crates/vapor_ide_ui/src/lib.rs`, new focused `workbench.rs`, `tests.rs`.

- [x] Add tests for workbench tool-window visibility/state defaults and toggling.
- [x] Run the focused tests and confirm the new cases fail before implementation.
- [x] Move presentation-only state out of the oversized app module into a focused workbench state module; preserve current behavior.
- [x] Run focused tests through Vapor-managed validation.

### Task 2: Add shared visual tokens and primitives

**Files:** new `Vapor-GUI/crates/vapor_gui_core/src/theme.rs`, `panel.rs`, `icons.rs`, `lib.rs`, focused tests.

- [x] Add tests for palette token invariants.
- [x] Add code-drawn IDE icons and extend existing console marks without relying on host glyphs.
- [x] Implement graphite surfaces, restrained blue accent, status palette, accessible contrast, and titled tool-window headers.
- [x] Establish compact spacing and use shared panel headers across Explorer, Inspector, and bottom tools; preserve code editor contrast and density.
- [x] Run focused tests and managed UI crate validation.

### Task 3: Compose the IDE workbench

**Files:** `vapor_ide_ui/src/workbench.rs`, `workspace_view.rs`, `editor_view.rs`, `console_view.rs`, `inspector_view.rs`, `lib.rs`.

- [x] Add tests for tool-window selection, close/restore, and panel visibility rules.
- [ ] Visually review the menu/action/status bars, coordinated project/editor/inspector/bottom windows, keyboard-accessible toggles, and responsive collapse behavior.
- [x] Keep Problems height fixed by default and user-resizable only at its divider; contents scroll and filters do not wrap into panel height.
- [x] Give the wide Explorer more room, keep search and Clear on one row, and collapse unrelated folders by default; expand the selected path or active search matches.
- [x] Keep problems, console, and managed operations as distinct contextual tool windows; avoid showing every subsystem at once.
- [x] Run focused tests and managed validation.

### Task 4: Make workspace and editor navigation coherent

**Files:** `workspace_navigation.rs`, `workspace_view.rs`, `editor_view.rs`, `semantic_view.rs`, `diagnostics.rs`, `tests.rs`.

- [x] Add behavior tests for empty-query browse requests, exact symbol/range navigation, stale result rejection, and dirty-buffer retention.
- [x] Connect project tree, Cargo crate groups, Rust module labels, nested semantic outline, tabs, and workspace search through existing navigation contracts.
- [x] Make Structure a workspace browser: show Rust Analyzer declarations without requiring a search term, nest by declaration containers, and retain open-file outlines as a distinct secondary view.
- [ ] Add contextual source links from diagnostics and console records; keep details on demand.
- [ ] Run focused tests and managed validation.

### Task 5: Build a parameterized authoring flow over fixed templates

**Files:** `authoring.rs`, `workspace_view.rs`, `service_session.rs`, tests.

- [x] Test wizard progression and content parameter validation.
- [x] Replace the flat content dialog with a focused multi-step authoring surface using only existing supported Rust item/module/content operations.
- [x] Expose typed content identity/version/dependency choices and a no-write content request review; provide a generated Rust item preview. Do not expose arbitrary template authoring.
- [x] Run focused tests and managed validation.

### Task 6: Make Atlas-aware Rust authoring a primary IDE capability

**Files:** Rust item/module templates, shared annotation model/protocol, Atlas source/index adapters, Inspector/Structure/editor integrations, migration tooling, tests and docs.

- [ ] Establish one source contract: Atlas records live in the Atlas repository; Rust annotations live with Rust source and refer to stable Atlas record identities; generated indices are rebuildable and never authoritative.
- [ ] Define a compact macro/annotation surface and parameterized templates from the intended authoring experience backwards. Authors supply record identity, relationships, inherited context, and review intent; tooling manages stable anchors, inverse indexes, validation, references, and review impact.
- [ ] Support type-, function-, field-, and module-level annotations. Permit direct links plus inherited context and independent relations on the same item. Keep generated identity and relationship syntax unobtrusive in the normal editor, while fully inspectable in a raw view.
- [ ] Model homogeneous parent/child hierarchies separately from heterogeneous typed relations; represent stateful relation validity and `supersedes` only where authored meaning warrants it.
- [ ] Support multiple review records scoped to item/relation sets and source revisions; surface missing coverage, stale references, and pending human review as IDE diagnostics.
- [ ] Make the fixed Rust-defined templates parameterizable for metadata blocks. New template families remain a rare coordinated product change.
- [ ] Add human-guided migration workflows so existing Rust items can be selected, annotated, reviewed, and tracked in batches without invented meaning or silent source rewrites.
- [ ] Test identity stability across moves/renames, relation validation, inherited context, reverse links, selective review invalidation, missing coverage, and partial migration.

### Task 7: Align Atlas repository and hosted service ownership

**Files:** Atlas repository contract, `Vapor-Platform-Server` orchestration, new `Vapor-Atlas-Server`, deprecated Docs-server documentation, IDE Atlas client contract.

- [ ] Keep Atlas records, schema, generator, static export, and source history in the `GHF-Studios/Atlas` repository.
- [ ] Put server-side read/query/publication behavior in `Vapor-Platform-Server/Vapor-Atlas-Server`; keep service state rebuildable from the canonical repo and independently deployable.
- [ ] Mark `Vapor-Docs-Server` as the deprecated legacy documentation solution and plan a deliberate migration; do not conflate it with Atlas's repository or knowledge schema.
- [ ] Expose a read-only query API plus generated static views from the same validated Atlas revision. IDE reads typed records and revision identity; edits flow through source-repository authoring, never a public-site write endpoint.
- [ ] Verify backup/restore, publication, query revision, and degraded-service states before the IDE depends on remote Atlas availability.

### Task 8: Strengthen language-service and task feedback

**Files:** `language_server.rs`, `diagnostics.rs`, `console_view.rs`, `service_session.rs`, tests.

- [ ] Test unavailable/restarting service states, diagnostic source navigation, and task lifecycle display.
- [ ] Show Rust Analyzer readiness/provenance and recoverable error state without blocking editing.
- [ ] Present managed operations as named tasks with origin, phase, progress, result, and safe cancellation only when service supports it.
- [ ] Run focused tests and managed validation.

### Task 9: Align supported structured editors and documentation

**Files:** `toml_semantics.rs`, `xml_semantics.rs`, `rhai_semantics.rs`, `Vapor-GUI/docs/ide.md`.

- [ ] Add tests for contextual labels and diagnostics routing without claiming semantic edits that are unsupported.
- [ ] Make supported TOML/XML/Rhai files discoverable as structured source while preserving a reliable raw text path.
- [ ] Document actual IDE capabilities, fixed template policy, key workflows, and known limits.
- [ ] Run managed validation.

### Task 10: Integration review and cross-platform readiness

**Files:** all plan-owned files only.

- [ ] Re-run the relevant Vapor-managed checks and inspect output.
- [ ] Review narrow-window behavior, keyboard access, dirty buffers, unavailable LSP, and editor/tool-window focus.
- [ ] Review full diff for accidental service logic duplication, implicit builds, claims of persisted Atlas semantics, and unrelated submodule changes.
- [ ] Record genuinely unresolved Atlas schema and review questions separately; do not imply unimplemented annotation contracts are persisted.
