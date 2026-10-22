# Vapor command console and operation stream

## Goal

Give Vapor CLI, Studio, IDE, and Launcher one understandable command and operation experience. Use the in-game F2 console as the interaction reference while keeping game commands and capabilities owned by Loo-Cast.

## Shared ownership

- `vapor_console_core` owns host-neutral console records, bounded history/filter state, command tokenization, command descriptors, and prompt history.
- Vapor Client owns the canonical command catalog, typed invocation, authorization/availability checks, dispatch, operation lifecycle, managed process identity, and structured output events.
- CLI, Studio, IDE, and Launcher are adapters. They do not spawn another Vapor CLI process or infer semantics from text.
- The game F2 console remains game-owned. It may consume shared record/editor semantics, but its command registry and capabilities do not cross into Vapor.

## Command flow

A line is parsed once into a typed Vapor command invocation, checked against the active frontend's capability profile and service-provided availability, then dispatched through the already-connected Client service as a typed action. Service validation remains authoritative. The CLI remains the full terminal adapter; Studio and IDE offer the service command surface appropriate to their current workspace/role. Launcher exposes no command execution.

Initial shared commands cover help/status, catalog refresh, composition selection/launch and workspace managed tools. The catalog is extensible by typed command registration; app-specific operations remain service actions rather than arbitrary shell strings. Commands requiring confirmation, credentials, external publication, or destructive effects use existing guarded review flows and cannot bypass them through the console.

## One operation stream

Lifecycle and child output preserve operation ID, timestamp, explicit severity, source/domain, action, process label, stream, and message. Severity is assigned at the producer from typed event kind/stream, never guessed from message words. ANSI/progress control sequences are normalized once while preserving useful progress updates. Existing bounded buffering and omission notices remain explicit. Full records are shared; each host chooses how to render them.

Managed process runners report the executable label and each output line through `OperationContext`. Rustup/toolchain installation uses the same managed runner and operation reporting, so the service can show named setup stages and live output rather than a delayed result.

## Presentation

- Studio: docked, resizable bottom operation panel, opened by F2 or the Console control. The first implementation puts the Console inside the existing operation panel; a distinct Problems tab is still a follow-up. Command prompt, history, filters, live operations, and full process output share the in-game console’s compact visual hierarchy. This stays in the authoring window because commands are part of the workspace workflow.
- IDE: the existing bottom tool window toggles with F2 and now renders build/process output through the shared console model. Problems remain in the same panel for this first integration; a separate Problems tab is still a follow-up.
- Launcher: on-demand separate read-only console window. It shows summarized active work by default, with expandable full records and no prompt.
- CLI: retains terminal-native interaction and ANSI/TUI behavior and now renders the same structured operation records. Reusing the GUI command descriptors for CLI help/completion remains follow-up work; its broader shell grammar stays intact.

## Safety and limits

The command catalog is an explicit capability surface, not a generic shell. Frontends may hide or disable commands, but the Client service still checks authorization and operation policy. Launcher is read-only. Studio/IDE cannot publish or mutate protected state without the same review/confirmation required elsewhere. Histories remain bounded; clearing a view does not cancel work or delete service state.

## Acceptance

- The same command descriptor, invocation, operation identity, phase, process output, severity, and source are visible consistently across CLI, Studio, IDE, and Launcher.
- Launcher has no command prompt and cannot issue mutations.
- Studio/IDE command dispatch uses their existing service connection and produces normal typed Client operations.
- No frontend classifies severity by searching message text.
- Rustup/toolchain work emits named stages and live process output.
- Game command registrations remain game-owned.
- Both GUI surfaces use a reusable console renderer/model and bounded history; IDE Problems remains distinct from operation output.
