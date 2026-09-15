#!/bin/sh
set -eu

target="x86_64-unknown-linux-gnu"

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd || pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." 2>/dev/null && pwd || pwd)

vapor="$app_root/bin/$target/vapor"
installer="$app_root/bin/$target/vapor-installer"
log_dir="$app_root/.vapor/logs"
launch_log="$log_dir/launch-wrapper.log"
bootstrap_state_dir="$app_root/.vapor/state/installer"
bootstrap_failure="$bootstrap_state_dir/bootstrap-failure.txt"

mkdir -p "$log_dir" 2>/dev/null || true

log() {
    timestamp=$(date '+%Y-%m-%dT%H:%M:%S%z' 2>/dev/null || date 2>/dev/null || echo "unknown-time")
    printf '%s vapor-launch[%s]: %s\n' "$timestamp" "$$" "$*" >>"$launch_log" 2>/dev/null || true
}

run_command() {
    command_label=${1:-"<command>"}
    set +e
    "$@"
    status=$?
    set -e
    log "command exited with status $status: $command_label"
    if [ "$hold_terminal" = "1" ]; then
        echo
        echo "Vapor exited with status $status."
        echo "Log: $launch_log"
        echo "Starting an interactive shell. Close this window when you are done."
        log "keeping terminal open after command status $status"
        exec "${SHELL:-/bin/sh}" -i
    fi
    exit "$status"
}

clear_bootstrap_failure() {
    rm -f "$bootstrap_failure" 2>/dev/null || true
}

write_bootstrap_failure() {
    mkdir -p "$bootstrap_state_dir" 2>/dev/null || true
    {
        printf '%s\n' "$1"
        printf '%s\n' "$installer_log"
    } >"$bootstrap_failure" 2>/dev/null || true
}

hold_terminal=0
if [ "${1:-}" = "--hold" ]; then
    hold_terminal=1
    shift
fi

launch_target="${1:-shell}"
if [ "$#" -gt 0 ]; then
    shift
fi

launch_path="$app_root/bin/$target:$app_root/bin"
export PATH="$launch_path:${PATH:-}"

cd "$app_root" 2>/dev/null || true
log "launch_target=$launch_target app_root=$app_root hold=$hold_terminal"

echo "Vapor launch wrapper"
echo "  launch target: $launch_target"
echo "  app root: $app_root"
echo "  log: $launch_log"
echo

case "$launch_target" in
    installer|vapor-installer)
        if [ ! -x "$installer" ]; then
            echo "error: expected Vapor Installer executable is missing"
            echo "  target:  $target"
            echo "  checked: $installer"
            echo
            echo "This Steam install is missing the platform installer for this launch option."
            echo "Update or reinstall the app on the selected Steam branch, then try again."
            exit 127
        fi
        if [ "$#" -eq 0 ]; then
            run_command "$installer" install
        fi
        run_command "$installer" "$@"
        ;;
esac

if [ ! -x "$vapor" ]; then
    echo "error: expected Vapor executable is missing"
    echo "  target:  $target"
    echo "  checked: $vapor"
    echo
    echo "This Steam install is missing the platform binary for this launch option."
    echo "Update or reinstall the app on the selected Steam branch, then try again."
    exit 127
fi

installer_log="$app_root/.vapor/logs/installer.log"
clear_bootstrap_failure
if [ -x "$installer" ]; then
    if "$installer" install; then
        :
    else
        installer_status=$?
        write_bootstrap_failure "vapor-installer exited with status $installer_status"
    fi
else
    write_bootstrap_failure "vapor-installer is missing for $target"
fi

case "$launch_target" in
    play)
        run_command "$vapor" --startup-script default "$@"
        ;;
    shell|vapor-shell)
        run_command "$vapor" "$@"
        ;;
    *)
        run_command "$vapor" "$launch_target" "$@"
        ;;
esac
