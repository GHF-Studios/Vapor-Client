# Launcher Content Lifecycle Coherence

**Status:** Implementation in progress  
**Related design:** [Shared Console and Editor Core](2026-10-22-shared-console-editor-core-design.md)  
**Goal:** Make Workshop discovery, subscription/acquisition, local Library availability, and the persistent playable selection read as one player-facing content lifecycle.

## User-visible problems

- Workshop type filters appear ineffective or leave old results visible after a filter change.
- The same Loo-Cast Packagepack appears as content and as a separate bespoke “Default” concept.
- Screens use weak visual grouping, so content sections and actions blur together.
- Workshop search, subscriptions/downloads, and Library each present separate versions of the same content.

## Product invariants

1. **One persistent selection:** the Client service's durable selected-composition identity is the single player launch selection. Every Launcher/CLI/Steam play path resolves that identity. The app manifest's `default` launch seed is only used to initialize a first playable selection or recover legacy empty state; it is not a second visible composition or ongoing selection mode.
2. **One content identity across lifecycle:** Workshop search, account subscriptions, local acquisition, Library availability, and composition membership are joined by PublishedFileId and, once resolved, Vapor content ID/version. These are status facets of one item, not competing catalogs.
3. **Subscribe obtains usable content:** after a successful subscription, the existing service acquisition path refreshes the Workshop item into Vapor's Library. The UI reports subscription, download progress, Library availability, update, and composition use as clear states of one row. It does not expose cache/provider mechanics to players.
4. **Typed Workshop filters are enforced:** the selected Vapor ContentKind is a typed filter, mapped to the canonical Steam Workshop type tag for remote search and checked against returned item metadata. A filter change cannot leave unmarked stale results looking current. Published items always carry their canonical type tag; tags remain additional user-authored discovery labels.
5. **Clear surfaces:** Launcher pages use consistent reusable panels/cards and hierarchy. Each panel groups one purpose, its relevant state, and its actions. Lists stay compact: repeated generic descriptions belong at the section level, with subtle boundaries and hover feedback. Small SVG marks should identify real content/tool semantics, not decorate every row.
6. **Single authority:** Vapor Client remains the owner of persistent selection, subscriptions/acquisition operations, and catalog state. Frontends only present and request typed operations.
7. **Workshop browsing controls:** Discovery and account Subscriptions are separate views over one published-item identity. Discovery uses Steam's 50-item remote page contract, with local 10/20/50 row paging inside that page; subscriptions use the same configurable page sizes against the complete account list. Multi-select uses PublishedFileId and visibly scoped rows.
8. **Local-state clarity:** subscription is account-wide; download, cache, and installed availability are device-local. Unsubscribe never silently removes local content, and removing a local cached copy never silently changes the account subscription. Removal is blocked while any Composition uses the content. Auto-Install is explicit policy, not a synonym for Subscribe.
9. **Progressive disclosure:** show the next useful action and essential state by default. Put secondary or destructive actions (unsubscribe, remove from this device, policy toggles) behind a small contextual menu or disclosure. Use arrows/overflow affordances to reveal detail when needed; avoid making every operation visible in every row. Selection toolbars own bulk actions and make their scope explicit.

## Lifecycle presentation

- **Play:** feature the currently selected Composition once, with its Packagepack identity, readiness, and one primary action (Play, Install, Check setup, Update, or Repair). Selecting another Composition persists through the Client service. Do not display a manifest alias beside the same selected Packagepack.
- **Library:** show locally available player content and playable Compositions from the shared catalog. A content row can indicate subscribed, downloading, available in Library, included in a Composition, update available, or needs repair. “Library” is the user-facing local availability view, not a separate source of identity.
- **Workshop:** Discovery and Subscriptions are two views over the same item identity. Discovery merges a matching account subscription into each result; Subscriptions pages the full Steam-account list independently of search. Both surface per-device Library/download/update state. Provide 10/20/50 visible-row sizing, remote Steam page navigation, row selection, and scoped group subscribe/unsubscribe. The remote Steam page remains 50 results because that is the provider contract; local page-size controls never claim to change it. Unsubscribe and local removal are separate actions. Local removal is available from Library or Subscriptions only when the item is cached and no Composition uses it.
- **Panels:** use the shared GUI foundation for titled sections, content cards, status groups, and contextual details. Keep the current dark theme and page shell. Prefer compact, shaded groups with hover feedback. Keep primary action and essential state visible; move secondary row actions into a small contextual `⋯` menu. Do not redesign Studio/IDE/Installer wholesale in this tranche; give these primitives a reuse seam and use them on Launcher Play, Library, and Workshop.

## Compatibility and recovery

- Preserve existing saved compositions and selection. Upgrade only the legacy empty `Default` placeholder: resolve the app's default Packagepack into that one persistent Composition, give it the Packagepack's human-readable name when available, and retain its stable ID/selection. Do not create a second Composition or overwrite a custom root/name.
- If no default Packagepack exists, preserve a valid empty selection state and show a clear choose-a-game action; do not fabricate a playable item.
- Keep `launch default` as a compatibility alias if existing scripts or root configuration require it, but route Steam's normal player entry through the persistent selected Composition. The alias must not bypass the selection after initialization.
- Existing Workshop content missing its canonical kind tag is classified as unknown; do not guess from title/description. New Vapor publication always includes the kind tag.
- Keep package/content schema version at `0.5.0` during alpha; any service protocol evolution must remain compatible with the currently shipped entrypoint/frontends during a staged deploy.

## Implementation sequence

1. Trace and fix the typed search path end to end; test tag derivation, stale-result invalidation, and paging. Preserve custom Workshop tags.
2. Seed the durable selected Composition from the manifest's first-run Packagepack and route the Steam play path through selection. Preserve existing state.
3. Build a shared Launcher lifecycle row projection that joins Workshop results/subscriptions with catalog/cache/install/composition facts; make Subscribe/Install/Unsubscribe statuses consistent.
4. Add reusable GUI panel/card primitives and apply them across Launcher pages; verify layout at normal and narrow widths.
5. Continue the approved shared console/editor-core design: structured records survive ingestion, severity/source/target/fields remain structured, and Rhai language rules are shared without merging host commands or GUI framework versions.
6. Keep Steam's remote page size truthful; add local result-page sizing and account-subscription pagination, visible-page multi-select, and grouped actions. Make account-wide subscription distinct from machine-local cache/install state and expose safe local removal without changing subscription.
7. Run owning-workspace Vapor-managed verification and local deployment only after the user-visible slices compile cleanly. Do not publish to Steam or alter Steam launch options in this pass.

## Acceptance

- Selecting a Workshop type yields only that type's results after search; page navigation preserves the applied filter; changing controls cannot present old results as current.
- Fresh setup has one persistent selected playable Composition when the manifest supplies one, named for the actual content. Existing valid selections and custom composition names survive restart.
- Steam's normal play entry, Launcher Play, and CLI selected launch use the same persisted selection; compatibility seed resolution runs only when needed.
- A subscribed Workshop item, its acquisition result, and its Library row resolve to the same identity and present one coherent state with usable retry/remove actions.
- Discover shows only the current filtered provider page; Subscriptions shows the complete account list in local pages. Page size, visible selection, and group actions have explicit scope.
- A device can remove an unused cached copy while remaining subscribed, or unsubscribe while keeping its local copy. Content used by a Composition cannot be removed from the Library.
- Library includes locally available player-eligible content beyond only top-level games, while keeping Crate/developer-only content out of Launcher.
- Visual sections have consistent boundaries and readable status/action hierarchy; no duplicate item rows or duplicate “Default” presentation.
- Item rows expose one clear next action; secondary account/device cleanup and policy actions are discoverable in contextual menus, with bulk actions kept in the selected-items toolbar.
- Console/editor integration retains the separate host command and permission boundaries documented in the related design.

## Not in scope

No Steam publishing, deleting Workshop items, changing launch-option configuration, broad Studio/IDE redesign, or content/schema version bump. Do not expose cache paths or internal provider records in the normal player flow. Do not conflate Workshop subscription with composition membership: users still choose which Composition uses acquired add-ons.
