#!/usr/bin/env python3
"""Read-only NSO fingerprint/signature verification. No extracted files are written."""

import argparse
import hashlib
from pathlib import Path
import re
import struct


ROOT = Path(__file__).resolve().parent.parent


def lz4_block(data: bytes, expected_size: int) -> bytes:
  output = bytearray()
  cursor = 0

  def length(base: int) -> int:
    nonlocal cursor
    if base == 15:
      while True:
        if cursor >= len(data):
          raise ValueError("truncated LZ4 length")
        value = data[cursor]
        cursor += 1
        base += value
        if value != 255:
          break
    return base

  while cursor < len(data):
    token = data[cursor]
    cursor += 1
    literals = length(token >> 4)
    if cursor + literals > len(data) or len(output) + literals > expected_size:
      raise ValueError("invalid LZ4 literal range")
    output.extend(data[cursor:cursor + literals])
    cursor += literals
    if cursor == len(data):
      break
    if cursor + 2 > len(data):
      raise ValueError("truncated LZ4 match")
    distance = struct.unpack_from("<H", data, cursor)[0]
    cursor += 2
    count = length(token & 15) + 4
    if distance == 0 or distance > len(output) or len(output) + count > expected_size:
      raise ValueError("invalid LZ4 back-reference")
    pattern = output[-distance:]
    output.extend((pattern * ((count + distance - 1) // distance))[:count])
  if len(output) != expected_size:
    raise ValueError("LZ4 output size mismatch")
  return bytes(output)


def segments(data: bytes) -> list[tuple[int, bytes]]:
  if len(data) < 0x100 or data[:4] != b"NSO0":
    raise ValueError("expected an NSO0 main module")
  flags = struct.unpack_from("<I", data, 0x0C)[0]
  result = []
  for index in range(3):
    offset, address, size = struct.unpack_from("<III", data, 0x10 + index * 0x10)
    compressed = bool(flags & (1 << index))
    stored = struct.unpack_from("<I", data, 0x60 + index * 4)[0] if compressed else size
    if size > 256 * 1024 * 1024 or offset < 0x100 or offset + stored > len(data):
      raise ValueError("invalid NSO segment range")
    block = data[offset:offset + stored]
    unpacked = lz4_block(block, size) if compressed else block
    if flags & (1 << (index + 3)):
      expected = data[0xA0 + index * 32:0xC0 + index * 32]
      if hashlib.sha256(unpacked).digest() != expected:
        raise ValueError("NSO segment checksum mismatch")
    if result and address < result[-1][0] + len(result[-1][1]):
      raise ValueError("overlapping NSO segments")
    result.append((address, unpacked))
  return result


def verify_game(path: Path) -> int:
  profile = (ROOT / "src/lib.rs").read_text(encoding="utf-8")
  catalog_profile = (ROOT / "src/outfits/profile.rs").read_text(encoding="utf-8")
  build_values = re.search(r"pub const BUILD_ID:.*?=\s*\[(.*?)\];", profile, re.S).group(1)
  build_id = bytes(int(v, 16) for v in re.findall(r"0x([0-9A-Fa-f]+)", build_values))
  data = path.read_bytes()
  if data[0x40:0x60] != build_id + bytes(16):
    raise ValueError("unsupported NSO Build ID")
  sections = segments(data)
  address, text = sections[0]
  if address != 0:
    raise ValueError("unsupported text layout")
  windows = re.findall(r"\((0x[0-9A-Fa-f]+),\s*\[([^\]]+)\]\)", catalog_profile)
  for address, words in windows:
    expected = b"".join(struct.pack("<I", int(w, 16)) for w in re.findall(r"0x[0-9A-Fa-f]+", words))
    offset = int(address, 16)
    if text[offset:offset + len(expected)] != expected:
      raise ValueError(f"instruction mismatch at {address}")
  return len(windows)


if __name__ == "__main__":
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument("main", type=Path)
  print(f"Verified {verify_game(parser.parse_args().main)} outfit instruction windows")
