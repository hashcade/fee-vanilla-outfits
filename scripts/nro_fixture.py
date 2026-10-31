"""Synthetic NRO metadata for verifier tests; contains no game code."""

import struct


def fixture_nro(symbol: str = "getRegionAddress", referenced: bool = True,
                plt: bool = True, defined: bool = False) -> bytes:
  data = bytearray(0x600)
  struct.pack_into("<I", data, 4, 0x80)
  data[0x10:0x14] = b"NRO0"
  struct.pack_into("<I", data, 0x18, len(data))
  for position, offset, size in ((0x20, 0, 0x100), (0x28, 0x100, 0x300), (0x30, 0x400, 0x200)):
    struct.pack_into("<II", data, position, offset, size)
  data[0x40:0x60] = bytes(range(32))
  data[0x80:0x84] = b"MOD0"
  struct.pack_into("<i", data, 0x84, 0x400 - 0x80)
  strings = b"\0" + symbol.encode() + b"\0"
  data[0x280:0x280 + len(strings)] = strings
  struct.pack_into("<IIII", data, 0x180, 1, 2, 0, 0)
  struct.pack_into("<IBBHQQ", data, 0x218, 1, 0x12, 0, int(defined), 0, 0)
  struct.pack_into("<QQq", data, 0x300, 0x580, (1 << 32) | 1026, 0)
  tags = [(4, 0x180), (5, 0x280), (6, 0x200), (10, len(strings)), (11, 24)]
  if plt:
    tags += [(23, 0x300), (2, 24 if referenced else 0), (20, 7)]
  else:
    tags += [(7, 0x300), (8, 24 if referenced else 0), (9, 24)]
  for index, pair in enumerate(tags + [(0, 0)]):
    struct.pack_into("<QQ", data, 0x400 + index * 16, *pair)
  return bytes(data)
