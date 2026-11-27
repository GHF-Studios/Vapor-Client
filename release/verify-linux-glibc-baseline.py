#!/usr/bin/env python3
"""Reject Linux release payloads that require newer glibc than policy allows."""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path


VERSION_NAME = re.compile(r"\bName:\s+GLIBC_(\d+(?:\.\d+){1,2})\b")


def version_tuple(value: str) -> tuple[int, ...]:
    parts = tuple(int(part) for part in value.split("."))
    return parts + (0,) * (3 - len(parts))


def main() -> int:
    if len(sys.argv) != 3:
        print(
            f"usage: {Path(sys.argv[0]).name} STAGED_APP_ROOT TARGET_POLICY.toml",
            file=sys.stderr,
        )
        return 2

    root = Path(sys.argv[1])
    policy_path = Path(sys.argv[2])
    try:
        policy = tomllib.loads(policy_path.read_text(encoding="utf-8"))
        maximum = policy["linux-x86_64"]["glibc_maximum"]
        if policy.get("schema") != 1 or not re.fullmatch(r"\d+\.\d+", maximum):
            raise ValueError("unsupported or malformed Linux target policy")
    except (OSError, UnicodeError, tomllib.TOMLDecodeError, KeyError, TypeError, ValueError) as error:
        print(f"error: invalid Linux target policy: {error}", file=sys.stderr)
        return 1

    readelf = shutil.which("readelf")
    if readelf is None:
        print("error: readelf is required to verify the Linux glibc baseline", file=sys.stderr)
        return 1

    maximum_version = version_tuple(maximum)
    violations: list[tuple[str, str]] = []
    inspected = 0
    for path in sorted(root.rglob("*")):
        if not path.is_file() or path.is_symlink():
            continue
        try:
            with path.open("rb") as file:
                if file.read(4) != b"\x7fELF":
                    continue
        except OSError as error:
            print(f"error: cannot inspect Linux release payload {path}: {error}", file=sys.stderr)
            return 1

        inspected += 1
        result = subprocess.run(
            [readelf, "--version-info", "--wide", str(path)],
            check=False,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            detail = result.stderr.strip() or f"readelf exited {result.returncode}"
            print(f"error: cannot inspect ELF version requirements for {path}: {detail}", file=sys.stderr)
            return 1

        needs = result.stdout.split("Version needs section", maxsplit=1)
        if len(needs) < 2:
            continue
        needs_section = needs[1].split("Version definition section", maxsplit=1)[0]
        for match in VERSION_NAME.finditer(needs_section):
            required = match.group(1)
            if version_tuple(required) > maximum_version:
                violations.append((str(path.relative_to(root)), required))

    if violations:
        print(
            f"error: Linux payload exceeds the pinned glibc {maximum} baseline:",
            file=sys.stderr,
        )
        for relative, required in violations:
            print(f"  {relative}: requires GLIBC_{required}", file=sys.stderr)
        return 1

    print(
        f"Verified {inspected} ELF payload(s) against the pinned glibc {maximum} baseline"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
