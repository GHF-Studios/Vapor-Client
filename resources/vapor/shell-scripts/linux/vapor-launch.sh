#!/bin/sh
set -eu

target="x86_64-unknown-linux-gnu"
script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd || pwd)
steam_root=$(CDPATH= cd -- "$script_dir/.." 2>/dev/null && pwd || printf '%s' "$script_dir/..")

if [ -n "${VAPOR_HOME:-}" ] && [ -f "$VAPOR_HOME/Vapor-Home.toml" ]; then
    app_root=$VAPOR_HOME
elif [ -f "$steam_root/Vapor-Home.toml" ]; then
    app_root=$steam_root
elif [ -n "${HOME:-}" ] && [ -f "$HOME/Vapor/Vapor-Home.toml" ]; then
    app_root=$HOME/Vapor
else
    app_root=
fi

steam_entrypoint="$steam_root/bin/$target/vapor-entrypoint"
if [ -x "$steam_entrypoint" ]; then
    entrypoint=$steam_entrypoint
else
    entrypoint="${app_root:-}/bin/$target/vapor-entrypoint"
fi

if [ -z "$app_root" ]; then
    message="Vapor is not installed for this user. Download Vapor from https://vapor.ghf-studios.site/ and then launch this app again."
    if [ -x "$entrypoint" ]; then
        "$entrypoint" chooser
        exit $?
    fi
    for program in zenity kdialog; do
        if command -v "$program" >/dev/null 2>&1; then
            if [ "$program" = zenity ]; then
                "$program" --error --title "Vapor is not installed" --text "$message" || true
            else
                "$program" --title "Vapor is not installed" --error "$message" || true
            fi
            exit 1
        fi
    done
    printf 'error: %s\n' "$message" >&2
    exit 1
fi

vapor="$app_root/bin/$target/vapor"
log_dir="$app_root/data/logs"
launch_log="$log_dir/launch-wrapper.log"
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

hold_terminal=0
if [ "${1:-}" = "--hold" ]; then
    hold_terminal=1
    shift
fi

launch_target=${1:-shell}
if [ "$#" -gt 0 ]; then
    shift
fi

export VAPOR_HOME="$app_root"
export PATH="$app_root/bin/$target:$app_root/bin:${PATH:-}"
cd "$app_root" 2>/dev/null || true
log "launch_target=$launch_target vapor_home=$app_root steam_root=$steam_root hold=$hold_terminal"

echo "Vapor launch wrapper"
echo "  launch target: $launch_target"
echo "  Vapor Home: $app_root"
echo "  log: $launch_log"
echo

case "$launch_target" in
    installer|vapor-installer|launcher|studio|ide|chooser|play)
        if [ ! -x "$entrypoint" ]; then
            echo "error: the Vapor launch helper is missing: $entrypoint" >&2
            exit 127
        fi
        run_command "$entrypoint" "$launch_target" "$@"
        ;;
    shell|vapor-shell)
        if [ ! -x "$vapor" ]; then
            echo "error: the Vapor command is missing from Vapor Home: $vapor" >&2
            exit 127
        fi
        run_command "$vapor" "$@"
        ;;
    *)
        if [ ! -x "$vapor" ]; then
            echo "error: the Vapor command is missing from Vapor Home: $vapor" >&2
            exit 127
        fi
        run_command "$vapor" "$launch_target" "$@"
        ;;
esac
