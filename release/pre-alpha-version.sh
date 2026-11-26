#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -gt 1 ]; then
  echo "Usage: pre-alpha-version.sh [CURRENT_PRE_ALPHA_VERSION]" >&2
  exit 2
fi

# PRE-ALPHA keeps the compatibility target fixed. An unseeded local release
# starts at 0.1.0; hosted releases advance the published inner SemVer patch.
compatibility_version=0.5.0
if [ "$#" -eq 0 ]; then
  printf '%s-pre-alpha-0.1.0\n' "$compatibility_version"
  exit 0
fi

if [[ ! "$1" =~ ^0\.5\.0-pre-alpha-(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
  echo "error: current version must use the pinned 0.5.0-pre-alpha SemVer form" >&2
  exit 2
fi

inner_major="${BASH_REMATCH[1]}"
inner_minor="${BASH_REMATCH[2]}"
inner_patch="${BASH_REMATCH[3]}"
inner_patch=$((10#$inner_patch + 1))
printf '%s-pre-alpha-%s.%s.%s\n' \
  "$compatibility_version" "$inner_major" "$inner_minor" "$inner_patch"
