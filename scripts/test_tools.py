import hashlib
from pathlib import Path
import struct
import tempfile
import unittest
from unittest.mock import patch
import zipfile

import package
import release
from verify_nro import verify_nro
from nro_fixture import fixture_nro


class ToolsTests(unittest.TestCase):
  def test_package_has_exactly_the_cobalt_path_and_verifiable_hashes(self):
    with tempfile.TemporaryDirectory() as directory:
      root = Path(directory)
      source = root / "built.nro"
      source.write_bytes(fixture_nro())
      files = package.package(source, "0.1.0", root / "dist")
      with zipfile.ZipFile(files[1]) as bundle:
        self.assertEqual(bundle.namelist(), [package.INSTALL_PATH])
        self.assertEqual(bundle.read(package.INSTALL_PATH), source.read_bytes())
      for line in files[2].read_text().splitlines():
        digest, name = line.split()
        self.assertEqual(digest, hashlib.sha256((root / "dist" / name).read_bytes()).hexdigest())

  def test_corrupt_nro_is_not_packaged(self):
    with tempfile.TemporaryDirectory() as directory:
      root = Path(directory)
      source = root / "bad.nro"
      source.write_bytes(b"bad")
      with self.assertRaises(ValueError):
        package.package(source, "0.1.0", root / "dist")
      self.assertFalse((root / "dist").exists())

  def test_nro_header_and_segments_are_checked(self):
    with tempfile.TemporaryDirectory() as directory:
      source = Path(directory) / "test.nro"
      source.write_bytes(fixture_nro())
      verify_nro(source)
      for position in [0x10, 0x18, 0x20, 0x80]:
        data = bytearray(fixture_nro())
        data[position] ^= 1
        source.write_bytes(data)
        with self.subTest(position=position), self.assertRaises(ValueError):
          verify_nro(source)

  def test_version_increments(self):
    self.assertEqual(release.bump_version("1.2.3", "patch"), "1.2.4")
    self.assertEqual(release.bump_version("1.2.3", "minor"), "1.3.0")
    self.assertEqual(release.bump_version("1.2.3", "major"), "2.0.0")

  def test_version_update_does_not_change_dependency_versions(self):
    current = release.version()
    with tempfile.TemporaryDirectory() as directory:
      root = Path(directory)
      for name in ("VERSION", "Cargo.toml", "Cargo.lock"):
        (root / name).write_bytes((release.ROOT / name).read_bytes())
      previous_lock = (root / "Cargo.lock").read_text()
      with patch.object(release, "ROOT", root):
        release.write_version("0.2.0")
        self.assertEqual(release.version(), "0.2.0")
      self.assertEqual((root / "Cargo.lock").read_text(), previous_lock.replace(
        f'name = "fee-vanilla-outfits"\nversion = "{current}"',
        'name = "fee-vanilla-outfits"\nversion = "0.2.0"'))


if __name__ == "__main__":
  unittest.main()
