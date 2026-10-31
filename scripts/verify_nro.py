#!/usr/bin/env python3
"""Verify NRO structure and imported interfaces against the Cobalt ABI contract."""

import argparse
from pathlib import Path
import struct
from nro_imports import verify_imports


def verify_nro(path: Path) -> None:
  data = path.read_bytes()
  if len(data) < 0x80 or data[0x10:0x14] != b"NRO0":
    raise ValueError("NRO0 header is missing")
  def u32(offset: int) -> int:
    return struct.unpack_from("<I", data, offset)[0]

  if u32(0x18) != len(data):
    raise ValueError("declared NRO size does not match the file")
  previous_end = 0
  for name, position in (("text", 0x20), ("rodata", 0x28), ("data", 0x30)):
    offset, size = struct.unpack_from("<II", data, position)
    if size == 0 or offset < previous_end or offset + size > len(data):
      raise ValueError(f"invalid {name} segment")
    previous_end = offset + size
  mod = u32(4)
  if mod + 4 > len(data) or data[mod:mod + 4] != b"MOD0":
    raise ValueError("MOD0 header is missing")
  if not any(data[0x40:0x60]):
    raise ValueError("NRO Build ID is empty")
  verify_imports(data)


def main() -> None:
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument("nro", type=Path)
  args = parser.parse_args()
  verify_nro(args.nro)
  print(f"Verified {args.nro} ({args.nro.stat().st_size:,} bytes), including Cobalt imports")


if __name__ == "__main__":
  main()
