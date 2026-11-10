# Steam Entrypoint and Vapor Home Boundary

## Decision

The Loo Cast Steam app is a Packagepack product, not a Vapor installation.
The Steam depots contain the Packagepack manifest and executable, plus one
small `vapor-entrypoint[.exe]` locator per supported target. The locator is an
adapter between Steam launch options and an independently installed Vapor.
It is not a bundled Vapor runtime.

## Ownership rules

- Steam owns acquisition and updating of the Loo Cast Packagepack payload.
- Vapor owns Vapor binaries, installation state, logs, tools, caches, content,
  and settings under the canonical Vapor Home (`%USERPROFILE%\\Vapor` or
  `$HOME/Vapor`, unless `VAPOR_HOME` selects another marked home).
- The locator resolves Vapor Home from a valid explicit `VAPOR_HOME`, the
  standard user home, or the Vapor CLI found on `PATH`.
- The locator never sets `VAPOR_HOME` to the Steam app directory and never
  creates Vapor logs, state, or other writable data there.
- Steam's normal Play option invokes the installed Vapor CLI to launch the
  selected playable composition. The separate chooser option opens the
  installed Vapor tools selector. Both use the locator shipped in the Steam
  target depot.
- If Vapor is absent, the locator shows a visible message with the official
  Vapor site (`https://vapor.ghf-studios.site/`) and exits. It does not silently
  install Vapor or recreate a runtime in the Steam app.
- Entry logs, when Vapor Home can be resolved, live under
  `VAPOR_HOME/data/logs/entrypoint.log`.

## Current failure evidence

The prior locator discovered its Steam app root from its executable path, then
used that same path as Vapor Home. It looked for the CLI, Client service, GUI
applications, launch scripts, and Player setup receipt beside the Steam
locator; it also forced child `VAPOR_HOME` and its log path to the Steam root.
After Vapor binaries moved into the standalone home, those lookups fail and
attempted launches recreate Vapor-owned folders inside the Steam app.

## Supported Steam payload

- Shared depot: `Packagepack.vapor.toml` only.
- Linux target depot: the Packagepack executable, the Linux Steam locator at
  the existing launch-option path, and explicitly required runtime sidecars.
- Windows target depot: the Packagepack executable, the Windows Steam locator
  at the existing launch-option path, and explicitly required runtime sidecars.
- No Vapor CLI, GUI, service, installer, tools, docs, checkout, build cache,
  or writable runtime state is shipped in the Steam app.

## Acceptance

1. With Vapor installed in the default home, both Steam launch choices resolve
   binaries from that home and never write beneath the Steam app root.
2. With Vapor installed at a custom marked `VAPOR_HOME`, both choices use that
   home.
3. With Vapor absent, the user sees the official Vapor location and no
   Vapor-owned directories are created beneath Steam.
4. Packagepack staging rejects every extra file beyond the Packagepack
   manifest, target executable, target locator, and explicitly allowed
   runtime sidecars.
