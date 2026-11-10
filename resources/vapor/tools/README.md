# Vapor home tools

This source payload is installed under:

```text
<vapor-home>/resources/vapor/tools/
```

The Vapor home owns the runtime, managed tools, source checkouts, caches, and
mutable state. A SuperWorkspace is authored separately and does not own Vapor
tools or the runtime.

## SuperWorkspace setup

Run these from the installed Vapor home:

```text
<vapor-home>/bin/<target>/vapor-installer superworkspace create --path /path/to/SuperWorkspace
<vapor-home>/bin/<target>/vapor-installer source clone Vapor-Client --super-workspace /path/to/SuperWorkspace
<vapor-home>/bin/<target>/vapor-installer ide patch-rustrover --super-workspace /path/to/SuperWorkspace
```

Clone only the sources needed for the current task. The SuperWorkspace stores
checkouts under `sources/`; its manifest records the independently installed
Vapor home that owns the toolchain and generated state.

## Player and Developer setup

```text
<vapor-home>/bin/<target>/vapor-installer status
<vapor-home>/bin/<target>/vapor-installer install
<vapor-home>/bin/<target>/vapor-installer dev-env install
```

Player setup creates Vapor-owned runtime state and content directories.
Developer setup is mandatory for the Developer role and provisions the pinned
Rust toolchain, both supported Rust targets, the required cross-linkers, and
authoring tools. Neither role setup installs or modifies a Steam game.

## RustRover

`patch_rustrover.rs` writes repairable `.idea` metadata and run configurations
inside the selected SuperWorkspace. Its runner, binaries, Rust toolchain, and
cross-linkers are resolved from the Vapor home.
