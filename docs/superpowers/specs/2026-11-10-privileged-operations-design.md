# Vapor Privileged Operations Design

## Intent

Make setup actions that need operating-system administrator rights predictable
and understandable without turning Vapor into a permanently privileged service.
Developer setup may need host prerequisites; Player setup must remain
unprivileged.

## Goals

- Present each privileged action as a named operation with a plain-language
  reason, exact executable/package/action scope, and expected system changes.
- Ask for OS authorization separately for each operation, then verify its
  result and report success or failure to the invoking Vapor screen.
- Reuse the same operation description and handoff flow across Developer setup
  and future maintenance tools.
- Keep Vapor's Rust toolchain, Zig, LLVM-MinGW, wrappers, Cargo data, and
  cross-target sysroots under Vapor Home; installing these must not require OS
  elevation.
- Keep Player setup and ordinary gameplay free of elevation prompts.

## Non-goals

- No persistent privileged daemon, service, scheduled task, or root process.
- No arbitrary shell, command string, script path, or package list supplied by
  content or the user and then executed as administrator.
- No implicit package-manager changes or elevation during ordinary startup.
- No assumption that administrator rights can make a Windows host provide a
  Linux sysroot; cross-target libraries still need a separately provisioned,
  target-correct sysroot.

## Approaches

1. **Native OS prompt per typed operation — chosen.** Vapor constructs an
   allowlisted operation from its own code, displays its scope, and invokes the
   platform's normal elevation mechanism for that exact executable and
   arguments (`runas`/UAC on Windows; `pkexec` or an explicit terminal `sudo`
   flow on Linux). This keeps trust with the OS and makes authorization
   operation-scoped.
2. **Vapor-owned elevated helper.** A small helper receives a typed request,
   validates it, performs the operation, and exits. This provides stronger
   request/result framing when a native prompt cannot express or return the
   required result, but adds privileged code that must be maintained. Use only
   as a fallback for a concrete limitation in approach 1.
3. **Persistent privileged service.** Rejected for this phase. It leaves a
   standing admin surface and is unnecessary for infrequent setup actions.

## Operation contract

Every privileged operation has a stable operation identifier, a typed payload,
a user-visible summary, a preflight, a fixed executable plus argument vector,
and a post-operation verification. The invoking Vapor process owns the UI and
records the result. The elevated process receives no generic command string and
does not remain alive after the operation.

Native Linux game releases should build against a selected runtime SDK and run
against that matching runtime where the distribution platform provides one.
For Steam products, that means a matched Steam Linux Runtime SDK/runtime; common
runtime libraries come from that runtime, while hardware-specific graphics
drivers remain host-provided. Development headers are build dependencies, not
player requirements. Cross-target sysroots remain Vapor-managed artifacts
rather than host package installs. The current Wayland `pkg-config` failure on
Windows is evidence of a build-environment mismatch, not evidence that players
or Windows developers need administrator access.

The first real privileged operation should only be added after preflight
identifies a concrete native-host prerequisite that cannot be provided as an
app-local tool or selected build runtime. Its package set and affected system
state must be established before implementation; do not add a generic
"install dependencies" command with an inferred package list.

## Security and UX rules

- Show why elevation is needed, the exact action, and the affected system scope
  before opening the OS prompt.
- One prompt authorizes one operation. Cancellation changes nothing and is a
  normal result.
- Resolve package names and executable paths from Vapor-owned, reviewed code;
  never interpolate script content or user-controlled command text.
- Check preconditions before prompting and verify the resulting state after the
  operation. A successful OS prompt alone does not mean setup succeeded.
- If the OS prompt mechanism cannot pass a typed operation and return a useful
  result safely, document that concrete limitation before adding the helper.

## Review question

Approve this direction before implementation: native, operation-scoped OS
elevation first; a short-lived typed helper only for a demonstrated native
handoff limitation; no standing admin service.
