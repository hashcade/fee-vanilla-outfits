#!/usr/bin/env python3
"""Create the Cobalt mod ZIP and SHA256SUMS from a verified NRO."""

import argparse
import hashlib
from pathlib import Path
import re
import shutil
import zipfile
from verify_nro import verify_nro


NRO_NAME = "fee_vanilla_outfits.nro"
INSTALL_PATH = f"engage/mods/fee_vanilla_outfits/{NRO_NAME}"


def package(nro: Path, version: str, output: Path) -> list[Path]:
  if not re.fullmatch(r"\d+\.\d+\.\d+", version):
    raise ValueError("expected a stable semantic version")
  verify_nro(nro)
  output.mkdir(parents=True, exist_ok=True)
  artifact = output / NRO_NAME
  if artifact.resolve() != nro.resolve():
    shutil.copyfile(nro, artifact)
  archive = output / f"fee-vanilla-outfits-v{version}.zip"
  with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as bundle:
    bundle.write(artifact, INSTALL_PATH)
  hashes = output / "SHA256SUMS"
  hashes.write_text("".join(
    f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n"
    for path in (artifact, archive)
  ), encoding="utf-8")
  return [artifact, archive, hashes]


def main() -> None:
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument("nro", type=Path)
  parser.add_argument("--version", required=True)
  parser.add_argument("--output", type=Path, default=Path("dist"))
  args = parser.parse_args()
  for path in package(args.nro, args.version, args.output):
    print(path)


if __name__ == "__main__":
  main()
