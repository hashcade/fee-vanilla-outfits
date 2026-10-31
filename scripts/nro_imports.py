"""Inspect relocation-backed imports using the NRO's loaded ELF dynamic metadata."""

import json
from pathlib import Path
import struct


CONTRACT = Path(__file__).resolve().parent.parent / "tools/cobalt-imports.json"


def required_imports(data: bytes) -> set[str]:
  def checked(offset: int, size: int) -> bytes:
    if offset < 0 or size < 0 or offset + size > len(data):
      raise ValueError("dynamic metadata points outside the NRO")
    return data[offset:offset + size]

  def unpack(fmt: str, offset: int) -> tuple:
    return struct.unpack(fmt, checked(offset, struct.calcsize(fmt)))

  mod, = unpack("<I", 4)
  if checked(mod, 4) != b"MOD0":
    raise ValueError("MOD0 header is missing")
  relative, = unpack("<i", mod + 4)
  cursor = mod + relative
  tags = {}
  while True:
    tag, value = unpack("<QQ", cursor)
    cursor += 16
    if tag == 0:
      break
    if tag in tags and tag != 1:  # DT_NEEDED may occur more than once.
      raise ValueError(f"duplicate dynamic tag {tag}")
    tags[tag] = value

  if not {4, 5, 6, 10, 11}.issubset(tags) or tags[11] != 24:
    raise ValueError("missing or unsupported ELF dynamic symbol tables")
  buckets, symbol_count = unpack("<II", tags[4])
  checked(tags[4], 8 + 4 * (buckets + symbol_count))
  checked(tags[6], symbol_count * 24)
  strings = checked(tags[5], tags[10])
  tables = []
  if 7 in tags or 8 in tags:
    if not {7, 8, 9}.issubset(tags) or tags[9] != 24:
      raise ValueError("invalid RELA table")
    tables.append((tags[7], tags[8]))
  if 23 in tags or 2 in tags:
    if not {23, 2, 20}.issubset(tags) or tags[20] != 7:
      raise ValueError("invalid PLT RELA table")
    tables.append((tags[23], tags[2]))
  if 17 in tags or 18 in tags:
    raise ValueError("REL relocations are not supported")

  required = set()
  for offset, size in tables:
    checked(offset, size)
    if size % 24:
      raise ValueError("invalid RELA table size")
    for entry in range(offset, offset + size, 24):
      target, info, _ = unpack("<QQq", entry)
      checked(target, 8)
      index = info >> 32
      if index == 0:  # Relative relocations do not import a symbol.
        continue
      if index >= symbol_count:
        raise ValueError("relocation symbol index is out of range")
      name, _, _, section, _, _ = unpack("<IBBHQQ", tags[6] + index * 24)
      if section != 0:
        continue
      if name >= len(strings):
        raise ValueError("symbol name is outside the dynamic string table")
      end = strings.find(b"\0", name)
      if end < 0:
        raise ValueError("unterminated dynamic symbol name")
      symbol = strings[name:end].decode("utf-8")
      if not symbol:
        raise ValueError("relocation refers to an unnamed undefined symbol")
      required.add(symbol)
  return required


def allowed_imports() -> set[str]:
  contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
  if contract["schema"] != 1:
    raise ValueError("unsupported Cobalt ABI contract")
  names = [name for exports in contract["providers"].values() for name in exports]
  if len(names) != len(set(names)):
    raise ValueError("duplicate symbol in the Cobalt ABI contract")
  return set(names)


def verify_imports(data: bytes) -> int:
  required = required_imports(data)
  missing = required - allowed_imports()
  if missing:
    raise ValueError("imports unavailable in Cobalt 1.31.0 / Engage 2.0.0: " + ", ".join(sorted(missing)))
  return len(required)
