# Vapor Client Agent Bootstrap

This file bootstraps AI work in Vapor Client. It does not replace the canonical
planning protocol.

## Product scope orientation

Vapor Client is the application/depot source root. It is not the installed Steam
application directory and not a normal custom-content workspace. Vapor-Registry
is separate registry authority infrastructure. Loo-Cast is normal first-party
custom content and should use the Workshop/content model rather than app/root
publication paths.

User-facing product instructions belong in `README.md` and the Shell docs under
`Vapor-Shell/crates/vapor_shell/docs/`. Planning docs and this file are agent
protocol and review infrastructure; do not let them substitute for beginner or
operator documentation.

Current shell grammar uses `source *` for source selection, `content *` for
Workshop-backed content lifecycle, `root *` for app/depot workflows, and
`setup self *` for installed app command-environment setup. Backend tools such
as Cargo, Git, SteamCMD, and SteamUGC remain controlled providers rather than
raw public tool domains.

## Session startup

Before presenting or performing planning work, read these files in order:

1. `docs/planning/README.md` — canonical operating protocol.
2. `docs/planning-intake/corpus-register.md` — current source work.
3. `docs/planning-atomized/README.md` — unvetted candidate index.
4. `docs/planning-active/README.md` — current vetted planning set.
5. `docs/planning-superseded/README.md` — historical comparison set.
6. `docs/planning/operations/README.md` — user-facing operations.

Also inspect `git status --short` without changing the index or working tree.
Do not read the entire source corpus or candidate corpus during startup.
Then read `.agents/workstreams/README.md` and the records for any foreground,
queued, suspended, or mutation-owning workstreams relevant to the request.

If these operational files disagree, report the disagreement and stop before
planning edits.

## Default startup interface

The user controls the target, pace, and duration of every phase. The agent does
not rank work or recommend a task during ordinary startup.

When the user asks what is available, asks to begin, or supplies no specific
task, respond as a thin interactive menu:

```text
VAPOR-CLIENT
intake: <short state> · atomized: <short state> · active: <short state>

[1] <valid action>
[2] <valid action>
[3] <valid action>
[S] Status
[X] Exit
[>] Tell me directly
>
```

Derive options from the current files, not memory. Show only currently valid
actions, keep the first screen to roughly five choices, do not mark any option
recommended, and do not explain choices unless the user asks. Use a submenu
when a category contains many source artifacts or candidates. Do not edit files
until the user selects or directly states a task.

The user may answer with a number, command, filename, candidate name, or natural
language. If the initial prompt already names a valid task, skip the menu and
move directly into that task after checking for material ambiguity or conflict.

## Adaptive interaction model

The TUI is the session's navigation protocol, not a fixed screen that must be
printed mechanically. Render only the smallest interface useful at the current
moment.

Use these interaction states:

- `BOOTSTRAP`: show compact live state and valid top-level actions.
- `WORK`: perform the selected task; use ordinary concise progress updates and
  do not keep redisplaying menus.
- `DECISION`: when user input is genuinely required, show the question,
  consequences, and only the valid choices.
- `CHECKPOINT`: after an atomic result or safe stopping point, show the result
  and compact continuation controls.
- `PIVOT`: surface a material conflict, ambiguity, or scope change at the moment
  it becomes actionable.
- `EXIT`: checkpoint every open workstream and close with resumable state.

When useful, show a short breadcrumb instead of repeating a full header:

```text
VAPOR-CLIENT / <workstream> / <state>
```

Depend on the interface for real choices: do not bury a blocking question,
approval, pivot, or scope decision inside prose. Conversely, do not manufacture
a menu when the user has already given a clear valid instruction or when no
choice exists.

Adapt depth and wording to the user:

- accept natural language at every screen;
- keep stable controls stable, but make numbered options specific to the live
  context;
- show submenus only when they reduce overload;
- allow `back`, `menu`, `status`, `park`, `pivot`, and `exit` from any state;
- preserve every open workstream and keep exactly one in the foreground for
  mutation until it completes, suspends, pivots, or exits;
- do not infer that silence, an unrelated question, or a status request changes
  the foreground workstream;
- do not force the user through ceremony they have already bypassed explicitly.

The interface should feel like a simple responsive application backed by deep
reasoning, not like decorative terminal output.

## Session controls

Treat these as optional conversational controls; equivalent natural language
has the same effect:

- `menu`: show the current top-level menu.
- `back`: return one menu level without changing work.
- `status`: show concise repository and planning state.
- `park`: checkpoint and suspend the foreground workstream, then return to the
  menu.
- `pivot <target>`: checkpoint the foreground workstream, then switch scope.
- `exit`: checkpoint every open workstream and end the work session.
- any other text: interpret it normally in the current context.

A workstream is conversational/task scope, not necessarily a Git branch.

## Persistent workstreams and mutation claims

The user may keep several workstreams open and jump among them. Exactly one
workstream is foreground for active mutation in a given agent session; other
workstreams may remain queued, suspended, blocked, or complete-awaiting-close.

Repository persistence is the default. Create or update a record under
`.agents/workstreams/open/` when a workstream:

- has changed or is about to change repository state;
- claims a path or semantic mutation scope;
- remains incomplete across an attention switch or session boundary;
- participates in an explicit work sequence or dependency;
- has pending external state, authorization, commit, or handoff; or
- would lose material resume context if kept only in conversation.

A workstream may remain session-local only while it is read-only exploration,
owns no mutation claim, has no dependent or queued work, and can be abandoned
without losing material state. Persist it immediately when any of those
conditions stops being true.

Every persistent record contains:

- unique workstream name and goal;
- state and ordered successor/dependencies;
- exact path claims and any semantic mutation claim;
- completed work and verification;
- owned uncommitted changes;
- remaining work, blockers, and smallest resume action;
- pending external actions or authorizations.

Mutation claims are exclusive across open workstreams and agent sessions.
Claims restrict writes, not read-only inspection. Keep claims as narrow as
practical. Before writing, inspect the registry and current Git state. If the
requested write overlaps another claim or its owned uncommitted changes,
checkpoint both scopes and surface a `PIVOT` decision instead of silently
taking over. Never treat an old-looking claim as abandoned without evidence or
explicit direction.

When attention changes, checkpoint and suspend the foreground workstream at the
smallest honest boundary, retain its claims, then activate the requested or
queued workstream. Suspending is not closing. Returning resumes from the
recorded smallest action rather than reconstructing state from memory.

A workstream releases its claims only after its work and owned changes are
completed, explicitly transferred, committed as authorized, or abandoned by
explicit user direction. Move its record to `.agents/workstreams/closed/` when
the release is complete. A shared commit containing changes from multiple
workstreams requires explicit user authorization and must name those
workstreams.

On total session exit, checkpoint every open workstream, not only the
foreground one. Preserve the user's declared queue order; do not silently
choose a different successor.

## Properly timed pivots

Do not propose a different target merely because another task seems more useful
or efficient. Propose a pivot only when continuing as requested would create a
material problem, including:

- conflict with the canonical protocol or a current gate;
- ambiguity whose interpretations would produce materially different work;
- new evidence that invalidates the current target or assumptions;
- scope growth that would abandon partially completed work;
- a destructive, externally visible, or newly authorized action;
- a task unit too entangled to leave the repository in an honest state.

Raise the pivot at the moment the issue becomes actionable, not speculatively
far in advance and not after silently choosing a direction. Keep the prompt
small:

```text
PIVOT
<one-sentence reason>

[1] <valid path>
[2] <valid path>
[P] Park foreground workstream
[X] Exit
[>] Give another direction
>
```

Include `continue as requested` only when it remains safe and protocol-valid.
The user chooses; the agent does not select on the user's behalf.

## Packaging work before suspend, park, pivot, or exit

Before leaving a workstream, stop at the smallest safe boundary and make its
state honest and resumable:

1. Do not mark atomization, coverage, promotion, or another lifecycle step
   complete when it is partial.
2. Preserve completed atomic work and clearly identify incomplete work.
3. Leave partial coverage or state explicitly incomplete rather than implying
   success.
4. Report the current target, completed portion, changed files, remaining work,
   blockers, and the exact smallest resume action.
5. Preserve unrelated user changes and existing staging.
6. Never discard or revert work merely to make packaging tidy without explicit
   user authorization.

Packaging runs before every foreground switch and implicitly before a pivot
that drops current scope. It runs explicitly for `park` or `exit`. Update the
persistent record whenever the workstream meets the persistence conditions. If
no work has begun, packaging is a one-line clean-exit acknowledgement.

## Task execution

Use the matching operation contract under
`docs/planning/operations/` and follow the canonical protocol
exactly. Operations are visible user-requested functionality; selecting one is
ordinary navigation unless another foreground workstream must first be
checkpointed.

- Process one source document or one promotion candidate at a time.
- Treat intake sources and atomized candidates as entirely unvetted.
- Never silently resolve, modernize, or reconcile claims during atomization.
- Do not promote anything while registered sources remain unatomized.
- Preserve unrelated user changes and existing staging.
- Do not commit or push unless the user explicitly requests it.
- Do not edit submodule repositories unless the user explicitly places them in
  scope. If a submodule is in scope, read its local agent instructions first.

After completing a task, report the concrete state change and show only compact
continuation controls appropriate to the live workstream. For example:

```text
VAPOR-CLIENT / <workstream> / checkpoint
<one-line result>

[C] Continue this workstream
[M] Menu
[P] Pivot
[X] Exit
[>] Tell me directly
>
```

Omit controls that are not valid. Do not recommend the next task unless a
properly timed pivot is required.
