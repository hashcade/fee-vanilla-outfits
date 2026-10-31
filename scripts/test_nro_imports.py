from pathlib import Path
import struct
import tempfile
import unittest
from nro_fixture import fixture_nro
from nro_imports import allowed_imports, required_imports, verify_imports
import package


class ImportTests(unittest.TestCase):
  def test_valid_cobalt_import(self):
    self.assertEqual(required_imports(fixture_nro()), {"getRegionAddress"})
    self.assertEqual(verify_imports(fixture_nro()), 1)

  def test_all_four_unavailable_skyline_interfaces_are_rejected(self):
    for symbol in ("get_program_id", "get_plugin_addresses", "add_nro_load_hook", "add_nro_unload_hook"):
      with self.subTest(symbol=symbol), self.assertRaisesRegex(ValueError, symbol):
        verify_imports(fixture_nro(symbol))

  def test_unrecognized_import_is_also_rejected(self):
    with self.assertRaisesRegex(ValueError, "unknown_loader_api"):
      verify_imports(fixture_nro("unknown_loader_api"))

  def test_non_plt_import_is_checked(self):
    with self.assertRaisesRegex(ValueError, "get_program_id"):
      verify_imports(fixture_nro("get_program_id", plt=False))

  def test_unused_header_declaration_is_not_a_runtime_import(self):
    self.assertEqual(required_imports(fixture_nro("__nro_header_start", referenced=False)), set())

  def test_locally_defined_symbols_are_not_imports(self):
    self.assertEqual(required_imports(fixture_nro("local_function", defined=True)), set())

  def test_unknown_import_cannot_be_packaged(self):
    with tempfile.TemporaryDirectory() as directory:
      root = Path(directory)
      source = root / "bad.nro"
      source.write_bytes(fixture_nro("get_program_id"))
      with self.assertRaisesRegex(ValueError, "get_program_id"):
        package.package(source, "0.1.1", root / "dist")
      self.assertFalse((root / "dist").exists())

  def test_out_of_range_dynamic_table(self):
    data = bytearray(fixture_nro())
    struct.pack_into("<i", data, 0x84, 0x700)
    with self.assertRaises(ValueError):
      required_imports(data)

  def test_out_of_range_symbol_index(self):
    data = bytearray(fixture_nro())
    struct.pack_into("<Q", data, 0x308, (3 << 32) | 1026)
    with self.assertRaisesRegex(ValueError, "symbol index"):
      required_imports(data)

  def test_bad_relocation_size(self):
    data = bytearray(fixture_nro())
    struct.pack_into("<Q", data, 0x468, 23)
    with self.assertRaisesRegex(ValueError, "table size"):
      required_imports(data)

  def test_contract_contains_no_skyline_only_interfaces(self):
    names = allowed_imports()
    for missing in ("get_program_id", "get_plugin_addresses", "add_nro_load_hook", "add_nro_unload_hook"):
      self.assertNotIn(missing, names)


if __name__ == "__main__":
  unittest.main()
