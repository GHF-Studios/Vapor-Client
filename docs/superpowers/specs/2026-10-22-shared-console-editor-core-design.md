# Shared Console and Editor Core

**Status:** Shared semantic core implemented; frontend integration ongoing  
**Goal:** Make the in-game console, Vapor IDE/devtools console, and Vapor terminal presentation consistent through shared semantics and rendering rules, while preserving each host's command permissions and lifecycle.

## Why this boundary

The in-game console already has structured records, severity, timestamps, source/target fields, bounded scrollback, command history, completion, and a dark presentation. Vapor devtools currently flattens telemetry into strings and displays a short plain-text log area. The game and Vapor editors both use `egui_code_editor`, but each defines language syntax locally and they use different egui versions (`0.34` in-game, `0.36` in Vapor). Directly sharing egui widgets would couple the products to a GUI upgrade and make the shared crate depend on a particular frontend.

The shared layer should own the meaning and presentation vocabulary of console records and editor languages. Each host should adapt that vocabulary to its own UI toolkit and retain its own command dispatch, runtime access, and authoring lifecycle.

## Ownership

Add a focused `vapor_console_core` crate to the Vapor workspace. It must not depend on Bevy, egui/eframe, the Vapor Client service, a terminal library, or either product frontend. It owns:

- A canonical structured console record: timestamp, level/kind, origin, source, target, message, and ordered structured fields. Clear is a view action, not a log message.
- Shared severity/style roles and formatting spans that hosts map to colors, typography, and terminal ANSI styles.
- Reusable bounded history, filtering, search, and prompt-history state where the behavior is identical across hosts. Hosts choose limits and persistence policy.
- Language identifiers and shared lexical definitions for supported editor languages, beginning with Rhai. A host-neutral tokenizer or syntax descriptor produces semantic token categories; it does not produce egui widgets or own document/file state.

The crate must remain deterministic and side-effect-free. It does not execute commands, inspect a process, read files, persist history, or perform telemetry I/O.

## Host adapters

- **Loo-Cast:** adapt tracing, game console output, and telemetry into shared records. Bevy/ECS command registration, runtime-variable completion, dispatch, game permissions, and the existing workbench's tabs/save/commit/revert lifecycle remain in Loo-Cast.
- **Vapor devtools and IDE:** retain telemetry and service-operation ingestion, but store structured records instead of formatted strings. Vapor service operations remain service-owned. IDE documents, Rust Analyzer/LSP, file access, and workspace actions remain Vapor-owned.
- **Vapor CLI:** retain Reedline, command grammar, and operation calls. Render shared records and severity styles in the terminal; do not route CLI commands through the game command registry.
- **GUI adapters:** each egui version maps shared records, style roles, and syntax categories into its own widgets. Do not align or upgrade egui merely to share concrete widget code.

No host may gain another host's capabilities through the shared core. In particular, a game command cannot invoke Vapor operations, and a Vapor command cannot invoke game ECS systems.

## Record and telemetry flow

Keep telemetry transport distinct from the console model. Convert typed telemetry events, tracing events, service output, and command responses at their host boundary into canonical records; retain source, stream, severity when available, timestamp, target, and fields instead of concatenating them into a display string. Where the existing telemetry payload lacks severity or fields, preserve compatibility and add typed optional data through an explicit protocol evolution rather than inferring semantics from message text. The UI formats records only at display time.

Scrollback remains bounded. Filtering, search, clear, and selection affect the view/model as documented and must not delete telemetry archives or execute host operations. Host-specific record sources remain visible and filterable.

## Editor sharing

Use one shared language registry and Rhai lexical definition so the in-game workbench and Vapor IDE classify the same source identically. Host adapters translate shared token categories to their own editor widgets. Keep the existing editor widgets during the first integration; do not force an egui-version convergence. Document buffers, tabs, save/commit/revert, LSP, and live script execution remain owned by their respective host workflows.

## Staged integration

1. Add the pure shared crate and move the canonical record, bounded view state, style roles, and Rhai language definition into it. Keep adapters minimal and verify the shared semantics without starting either GUI.
2. Adapt Loo-Cast console and workbench. Preserve command behavior and current interaction semantics while mapping records and Rhai tokens to the shared model.
3. Adapt Vapor devtools and IDE, removing string-flattening at ingestion and mapping the same model into the current egui UI. Add terminal rendering through Vapor CLI's existing prompt/output path.
4. Compare the same representative records and Rhai snippets across game overlay, Vapor GUI, and terminal; fix semantic or visual-role drift at the shared boundary.

Each stage must compile through its owning Vapor-managed workspace. Add focused tests for pure record formatting, bounds, filters/history, and language classification; do not duplicate widget snapshots as a substitute for behavioral verification.

## Implemented slice (2026-10-22)

- Added `vapor_console_core` with structured records, severity/style roles, bounded scrollback and prompt history, filters, and shared Rhai tokenization.
- Vapor telemetry preserves structured level, target, and fields compatibly; devtools consumes bounded shared history rather than flattening incoming records immediately.
- Vapor GUI and the in-game workbench use the same Rhai language definition while keeping their editor widgets and egui versions separate.
- Loo-Cast's game console adapts its local records to the shared semantic record and prompt history; command registration and dispatch remain game-owned.
- Vapor CLI renders Client operation messages through shared levels and style roles, using ANSI only for a terminal when `NO_COLOR` is not set.
- Remaining: exercise the shared structured record through live game/Vapor streams and improve richer terminal progress/output rendering. This slice does not claim one shared visual widget or full console parity.

## Acceptance

- A record with the same severity, source, target, fields, and message keeps those semantics in every frontend; rendering differs only where the host medium requires it.
- Game, Vapor GUI, and terminal show a consistent severity hierarchy, readable dark console palette, timestamps, source/target metadata, and field treatment.
- Scrollback is bounded and filter/search/history behavior is predictable and does not mutate host state.
- The same Rhai snippet receives the same semantic token categories in the in-game editor and Vapor IDE.
- Command dispatch, permission boundaries, telemetry ownership, service operations, and document lifecycles remain host-specific.
- No egui upgrade is required to adopt the shared core.

## Not in this tranche

This does not build a terminal emulator, merge game and Vapor command systems, redesign unrelated Studio screens, replace the in-game scripting workbench, implement Rust Analyzer/LSP, or make a single cross-product editor widget. Those systems can consume the shared contracts later without moving their ownership into the core.

## Constraints and risks

- Loo-Cast already consumes `vapor_telemetry` from the Vapor repository by pinned Git dependency. The new crate must follow that existing dependency and lockfile workflow; keep its package version at the current alpha version (`0.5.0`) and pin source revisions through the workspace dependency mechanism.
- Telemetry currently carries log stream and message but not full severity/fields. Enrichment must be typed and backward-compatible for existing producers/consumers.
- Shared syntax definitions must not accidentally become a shared editor document model or force dependency upgrades across the two GUI stacks.
- Existing console quality and behaviors are the reference to preserve; integration should replace duplicated semantics, not remove working controls or game-specific functionality.
