#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'EOF'
Usage:
  package-vapor-standalone-archive.sh STAGED_APP_ROOT PLATFORM VERSION OUTPUT_ARCHIVE

Packages the Vapor-owned output of `vapor root stage-standalone` for the signed
Artifact release pipeline. It does not invoke Cargo or modify the active app.
PLATFORM is windows-x86_64 or linux-x86_64.
EOF
}

if [ "$#" -ne 4 ]; then
  usage
  exit 2
fi

staged_root_input="$1"
platform="$2"
version="$3"
output_input="$4"
if [[ "$platform" != windows-x86_64 && "$platform" != linux-x86_64 ]]; then
  echo "error: supported release platforms are windows-x86_64 and linux-x86_64" >&2
  exit 2
fi
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
  echo "error: VERSION must be a SemVer version" >&2
  exit 2
fi
if [ ! -d "$staged_root_input" ] || [ -L "$staged_root_input" ]; then
  echo "error: STAGED_APP_ROOT must be a real directory" >&2
  exit 1
fi

app_root="$(cd -- "$staged_root_input" && pwd -P)"
script_dir="$(cd -- "$(dirname -- "$0")" && pwd -P)"
repo_root="$(cd -- "$script_dir/.." && pwd -P)"
output_parent_input="$(dirname -- "$output_input")"
output_name="$(basename -- "$output_input")"
mkdir -p -- "$output_parent_input"
output_parent="$(cd -- "$output_parent_input" && pwd -P)"
output_archive="$output_parent/$output_name"
if [ -e "$output_archive" ]; then
  echo "error: output already exists: $output_archive" >&2
  exit 1
fi

case "$platform" in
  windows-x86_64)
    target=x86_64-pc-windows-gnullvm
    launcher=vapor-launch.cmd
    ;;
  linux-x86_64)
    target=x86_64-unknown-linux-gnu
    launcher=vapor-launch.sh
    ;;
esac

for path in \
  "$app_root/bin/$target" \
  "$app_root/bin/$launcher" \
  "$app_root/resources/vapor/tools" \
  "$app_root/metadata/toolchain.toml" \
  "$repo_root/Standalone-App.vapor.toml"; do
  if [ ! -e "$path" ]; then
    echo "error: required standalone release input is missing: $path" >&2
    exit 1
  fi
done

case "$platform" in
  windows-x86_64)
    required=(
      vapor-entrypoint.exe vapor.exe vapor-installer.exe vapor-gui.exe vapor-setup.exe
      vapor-client-service.exe vapor-ide-host.exe steam_api64.dll libunwind.dll
    )
    ;;
  linux-x86_64)
    required=(
      vapor-entrypoint vapor vapor-installer vapor-gui vapor-setup vapor-client-service
      vapor-ide-host libsteam_api.so
    )
    ;;
esac
for name in "${required[@]}"; do
  if [ ! -f "$app_root/bin/$target/$name" ]; then
    echo "error: built app instance is missing runtime file: bin/$target/$name" >&2
    exit 1
  fi
done

python3 - "$app_root" "$target" "$launcher" <<'PY'
import os
import stat
import sys
from pathlib import Path

root = Path(sys.argv[1])
for relative in (
    Path("bin") / sys.argv[2],
    Path("bin") / sys.argv[3],
    Path("resources/vapor/tools"),
    Path("metadata/toolchain.toml"),
):
    start = root / relative
    if start.is_symlink():
        sys.exit(f"error: release input contains a symbolic link: {start}")
    paths = [start]
    if start.is_dir():
        paths.extend(start.rglob("*"))
    for path in paths:
        mode = path.lstat().st_mode
        if stat.S_ISLNK(mode) or not (stat.S_ISDIR(mode) or stat.S_ISREG(mode)):
            sys.exit(f"error: unsupported filesystem entry in release inputs: {path}")
PY

temporary_root="$(mktemp -d "$output_parent/.vapor-standalone.XXXXXX")"
temporary_archive="$output_parent/.$output_name.$$.tmp"
cleanup() {
  rm -rf -- "$temporary_root"
  rm -f -- "$temporary_archive"
}
trap cleanup EXIT

mkdir -p -- "$temporary_root/bin/$target" "$temporary_root/resources/vapor" "$temporary_root/metadata"
cp -a -- "$app_root/bin/$target/." "$temporary_root/bin/$target/"
cp -a -- "$app_root/bin/$launcher" "$temporary_root/bin/$launcher"
cp -a -- "$app_root/resources/vapor/tools" "$temporary_root/resources/vapor/tools"
cp -a -- "$app_root/metadata/toolchain.toml" "$temporary_root/metadata/toolchain.toml"

python3 - "$repo_root/Standalone-App.vapor.toml" "$temporary_root/App.vapor.toml" "$version" <<'PY'
import re
import sys
import tomllib
from pathlib import Path

source = Path(sys.argv[1])
destination = Path(sys.argv[2])
version = sys.argv[3]
text = source.read_text(encoding="utf-8")
data = tomllib.loads(text)
root = data.get("root")
if not isinstance(root, dict) or root.get("name") != "vapor-client" or root.get("organization") != "ghf-studios":
    sys.exit("error: standalone app manifest template has the wrong product identity")
if "steam" in root or "content" in data:
    sys.exit("error: standalone app manifest template must not contain Steam or game metadata")

section = None
matches = 0
lines = []
for line in text.splitlines(keepends=True):
    stripped = line.strip()
    if stripped.startswith("[") and stripped.endswith("]"):
        section = stripped.strip("[]")
    if section == "root" and re.match(r"^\s*version\s*=", line):
        line = re.sub(r'^(\s*version\s*=\s*)"[^"]*"(\s*(?:#.*)?(?:\r?\n)?)$', rf'\1"{version}"\2', line)
        if f'version = "{version}"' not in line:
            sys.exit("error: could not safely set the standalone app manifest version")
        matches += 1
    lines.append(line)
if matches != 1:
    sys.exit("error: standalone app manifest must contain exactly one root.version field")
result = "".join(lines)
tomllib.loads(result)
destination.write_text(result, encoding="utf-8")
PY

tar --sort=name --mtime='@0' --owner=0 --group=0 --numeric-owner \
  --format=posix -czf "$temporary_archive" -C "$temporary_root" .
ln -- "$temporary_archive" "$output_archive"
rm -f -- "$temporary_archive"
echo "Prepared standalone Vapor archive: $output_archive"
