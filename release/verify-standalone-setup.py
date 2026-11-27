#!/usr/bin/env python3
"""Reject a Windows standalone setup executable that needs libunwind.dll."""

from __future__ import annotations

import struct
import sys
from pathlib import Path


class InvalidExecutable(ValueError):
    pass


def pe_imports(path: Path) -> set[str]:
    data = path.read_bytes()

    def unpack(fmt: str, offset: int) -> tuple[int, ...]:
        size = struct.calcsize(fmt)
        if offset < 0 or offset + size > len(data):
            raise InvalidExecutable("truncated PE structure")
        return struct.unpack_from(fmt, data, offset)

    if data[:2] != b"MZ":
        raise InvalidExecutable("missing DOS executable header")
    pe_offset = unpack("<I", 0x3C)[0]
    if data[pe_offset : pe_offset + 4] != b"PE\0\0":
        raise InvalidExecutable("missing PE signature")

    coff = pe_offset + 4
    _, section_count, _, _, _, optional_size, _ = unpack("<HHIIIHH", coff)
    optional = coff + 20
    magic = unpack("<H", optional)[0]
    if magic == 0x20B:
        data_directory_offset = optional + 112
    elif magic == 0x10B:
        data_directory_offset = optional + 96
    else:
        raise InvalidExecutable("unsupported PE optional-header format")
    if optional_size < data_directory_offset - optional + 16:
        raise InvalidExecutable("PE optional header has no import directory")
    import_rva, import_size = unpack("<II", data_directory_offset + 8)
    if import_rva == 0 or import_size == 0:
        return set()

    sections_offset = optional + optional_size
    sections = []
    for index in range(section_count):
        section = sections_offset + index * 40
        virtual_size, virtual_address, raw_size, raw_pointer = unpack("<IIII", section + 8)
        sections.append((virtual_address, max(virtual_size, raw_size), raw_size, raw_pointer))

    def file_offset(rva: int) -> int:
        for address, span, raw_size, raw_pointer in sections:
            if address <= rva < address + span:
                delta = rva - address
                if delta >= raw_size:
                    break
                return raw_pointer + delta
        raise InvalidExecutable(f"RVA 0x{rva:x} is outside file-backed PE sections")

    def c_string(rva: int) -> str:
        offset = file_offset(rva)
        end = data.find(b"\0", offset, min(len(data), offset + 512))
        if end < 0:
            raise InvalidExecutable("unterminated PE import name")
        try:
            return data[offset:end].decode("ascii").lower()
        except UnicodeDecodeError as error:
            raise InvalidExecutable("non-ASCII PE import name") from error

    imports: set[str] = set()
    descriptor = file_offset(import_rva)
    limit = descriptor + min(import_size, 1024 * 1024)
    while descriptor + 20 <= limit:
        original_thunk, timestamp, forwarder, name_rva, first_thunk = unpack("<IIIII", descriptor)
        if not any((original_thunk, timestamp, forwarder, name_rva, first_thunk)):
            return imports
        imports.add(c_string(name_rva))
        descriptor += 20
    raise InvalidExecutable("PE import table has no terminator")


def main() -> int:
    if len(sys.argv) != 2:
        print(f"usage: {Path(sys.argv[0]).name} WINDOWS_SETUP.exe", file=sys.stderr)
        return 2
    path = Path(sys.argv[1])
    try:
        imports = pe_imports(path)
    except (OSError, InvalidExecutable) as error:
        print(f"error: cannot verify standalone setup executable: {error}", file=sys.stderr)
        return 1
    if "libunwind.dll" in imports:
        print(
            "error: standalone vapor-setup.exe imports libunwind.dll and cannot be "
            "downloaded as a single executable",
            file=sys.stderr,
        )
        return 1
    print(f"Verified standalone setup imports: {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
