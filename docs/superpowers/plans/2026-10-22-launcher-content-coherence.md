# Launcher Content Coherence Implementation Plan

**Spec:** [`2026-10-22-launcher-content-coherence-design.md`](../specs/2026-10-22-launcher-content-coherence-design.md)  
**Related:** [`2026-10-22-shared-console-editor-core-design.md`](../specs/2026-10-22-shared-console-editor-core-design.md)

## Pass A — Make type search truthful

- Trace `ContentKind` from Launcher control through protocol, Client operation, Steam UGC query and result snapshot.
- Make type a typed request value, retain generic tags, and map the kind to its canonical required Workshop tag in one service/provider boundary.
- Prevent stale results from appearing current after any applied filter changes; preserve query/filter on pagination.
- Add focused tests for canonical tags, mismatched returned tags, filter change, and paging.

## Pass B — Make selection the one launch authority

- Preserve current durable selection and composition records.
- Migrate only a valid legacy empty placeholder into the manifest-provided Packagepack seed.
- Set a user-facing content name from catalog identity and retain the stable selected Composition ID.
- Route Steam's direct-play entry through `launch selected`; preserve explicit compatibility behavior for scripts that still call `launch default`.
- Verify multiple-composition selection survives service restart and is honored by direct launch.

## Pass C — Join Workshop, Library, and selection state

- Add one launcher view projection keyed by PublishedFileId and resolved Vapor identity/version.
- Join Workshop results, subscriptions/acquisition progress, catalog locations, and composition graph membership.
- Present one row with state-driven actions; do not show a separate subscription universe beside the same Library item.
- Keep Crate and other creator-only entries out of player Library; include all local player-eligible runtime artifacts.
- Verify Subscribe triggers service acquisition and that failure/retry/unsubscribe preserve coherent state.
- Add provider-honest paging size, row selection, grouped subscribe/unsubscribe, and explicit local-state inspection/removal.
- Keep Discover and Subscriptions as separate views over the same PublishedFileId row model. Discover pages Steam's 50-item result page locally at 10/20/50 rows; Subscriptions pages the full account list at those sizes. Group actions apply only to selected items in the current visible page.
- Surface local cached-copy removal alongside unsubscribe, but keep those actions independent. Disable local removal while any composition references the content; preserve the Steam subscription when local files are removed.

## Pass D — Give pages useful panel hierarchy

- Add small reusable panel/card/status primitives to `vapor_gui_core`.
- Apply them to Play selection, Library grouping/details, and Workshop search/results/local state.
- Keep actions adjacent to the state they affect; keep the global footer/status from competing with page content.
- Use compact repeated rows, section-level guidance, subtle shaded panel boundaries, and hover feedback; introduce small semantic SVG marks only where they improve scanning.
- Check narrow and normal viewport layouts in code and run the available GUI locally if a display is available.

## Pass E — Integrate the shared console/editor core

- Add a pure, host-neutral shared crate for structured console records, style roles, bounded view/filter state, and Rhai lexical rules.
- Adapt Loo-Cast console/workbench, Vapor devtools/IDE, and Vapor CLI while keeping dispatch/permissions/document lifecycles host-owned.
- Preserve the existing uncommitted Rhai editor change in Loo-Cast; inspect and integrate it without replacing or dropping it.
- Add pure behavior tests; run compiler and workspace verification through Vapor-managed commands only.
- Adapt CLI Client operation summaries through shared ConsoleRecord severity/style roles without changing command grammar or game permissions.

## Completion checks

- `git diff --check` and targeted reviews across touched repositories.
- Vapor-managed validation for Vapor, Vapor-GUI, Vapor-CLI, Vapor-Steam-Entrypoint, Vapor-Studio, and Loo-Cast as the changed surfaces require; never raw Cargo.
- Local deploy is allowed after the build gates pass. Steam publish, launch-option edits, and deletion of Workshop test content are outside this pass.
- Record exact verification, any platform/runtime gaps, commits, and the remaining next step in the foreground workstream.
