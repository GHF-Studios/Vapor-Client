#!/usr/bin/env bash
set -euo pipefail

cloc Vapor/crates Vapor-CLI/crates Vapor-Examples/content Vapor-GUI/crates Vapor-Installer/crates Vapor-Steam-Entrypoint/crates Vapor-Studio/crates Vapor-Studio/docs resources, docs "Premium Docs" --timeout=0
