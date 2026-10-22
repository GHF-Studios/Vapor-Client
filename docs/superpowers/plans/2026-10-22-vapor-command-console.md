# Implementation plan: Vapor command console

## 1. Canonical command surface

- Add command descriptors, typed parse results, and capability profiles in a host-neutral/shared Vapor layer.
- Reuse the descriptors in terminal help/completion and GUI prompts; dispatch typed actions through the existing Client service.
- Start with safe, high-value commands (help/status, catalog refresh, workspace tools, composition selection/launch). Keep guarded/destructive flows on their existing review APIs.
- Add parser and capability tests before implementation.

## 2. Structured process output

- Extend the managed process event with a process label and preserve stream, timestamp, level, source/domain, and action in the Client protocol.
- Normalize terminal controls/progress updates at the producer boundary; preserve bounded output and omission markers.
- Route role/toolchain installation through the managed operation reporter so Rustup is visible while running.
- Add round-trip, mapping, and bounded-stream tests before implementation.

## 3. Shared presentation model

- Extend `vapor_gui_core` with the reusable console panel and explicit read-only/command-enabled modes.
- Remove Launcher text-based severity guessing and use typed records.
- Replace Studio’s independent console rendering with an integrated F2 docked pane; separate Problems from Console while preserving operation cancel/status controls.
- Replace IDE's raw build-output buffer with the same operation console, retaining Problems and its diagnostics actions.

## 4. CLI adapter and integration

- Render the same structured records and command help through terminal-native formatting.
- Ensure all three GUI apps reuse the same connected service; do not create a child CLI process.
- Update docs/workstream and remove obsolete parallel console state after the unified path is exercised.

## Verification

- Add focused tests for parsing, capability filtering, process event metadata, ANSI/progress normalization, bounded scrollback, and protocol serialization.
- Run Vapor-managed validation for Vapor, CLI, GUI, and Studio. Deploy locally only after the protocol and UI compile together. Inspect the diff and current workstream before reporting.
