# Vapor-Root Workstream Registry

This registry is the durable navigation surface for concurrent conversational
work. The governing behavior lives in [`AGENTS.md`](../../AGENTS.md).

## Ordered work

1. `standalone-vapor-runtime` — foreground.
2. `standalone-app-updater` — suspended as an implementation component of the runtime workstream.
3. `workshop-content-superpass` — suspended; its content-authoring scope remains intact.
4. `source-atomization-raw-info-dump` — suspended.
5. corpus atomization under the unchanged global gate.
6. `vapor-shell-exploration` — suspended.

## Open workstreams

- [`standalone-vapor-runtime`](open/standalone-vapor-runtime.md) — foreground.
- [`standalone-app-updater`](open/standalone-app-updater.md) — suspended component; its existing work is preserved and owned by the runtime workstream.
- [`workshop-content-superpass`](open/workshop-content-superpass.md) — suspended.
- [`source-atomization-raw-info-dump`](open/source-atomization-raw-info-dump.md) — suspended.
- [`planning-assertion-representation`](open/planning-assertion-representation.md) — complete, awaiting later commit/closure.
- [`source-atomization-portable-multi-repo-development`](open/source-atomization-portable-multi-repo-development.md) — complete, awaiting later commit/closure.
- [`vapor-shell-exploration`](open/vapor-shell-exploration.md) — suspended.

## Closed workstreams

- [`planning-taxonomy`](closed/planning-taxonomy.md) — included in the shared Agent-upgrade commit.
- [`agent-workstreams`](closed/agent-workstreams.md) — included in the shared Agent-upgrade commit.
