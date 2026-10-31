#!/usr/bin/env python3
"""Check the catalog against locally supplied vanilla XML without writing files."""

import argparse
from pathlib import Path
import re
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parent.parent


def catalog():
  text = (ROOT / "src/outfits/catalog.rs").read_text(encoding="utf-8")
  pattern = r'outfit!\(\s*"([^"]+)",\s*"([^"]+)",\s*(\d+),\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"\s*,?\s*\)'
  rows = [dict(zip(("name", "pid", "gender", "initial_class", "dress", "rig"), match))
    for match in re.findall(pattern, text)]
  for row in rows:
    row["gender"] = int(row["gender"])
  if len(rows) != 42 or len({(r["pid"], r["gender"]) for r in rows}) != 42:
    raise ValueError("expected 42 unique character/dress-gender entries")
  return rows


def verify(person_path, asset_path):
  people = {row.get("Pid"): row for row in ET.parse(person_path).findall(".//Data/Param")}
  assets = ET.parse(asset_path).findall(".//Data/Param")
  for row in catalog():
    person = people[row["pid"]]
    if person.get("Jid") != row["initial_class"]:
      raise ValueError(f"starting job mismatch: {row['name']}")
    gender = int(person.get("Gender"))
    if int(person.get("Flag", "0")) & 32:
      gender = 3 - gender
    if row["pid"] != "PID_リュール" and gender != row["gender"]:
      raise ValueError(f"dress gender mismatch: {row['name']}")
    found = False
    for asset in assets:
      if asset.get("Mode") != "2" or asset.get("DressModel") != row["dress"]:
        continue
      groups = [[part.strip() for part in group.split("|")]
        for group in asset.get("Conditions", "").split(";")]
      if [person.get("Name")] not in groups or not any(row["initial_class"] in group for group in groups):
        continue
      if ["私服"] in groups:
        continue
      if (row["gender"] == 1 and ["女装"] in groups) or (row["gender"] == 2 and ["男装"] in groups):
        continue
      found = True
      break
    if not found:
      raise ValueError(f"no native battle dress rule: {row['name']}")
    # Base-game personal preview presets provide an independent rig cross-check.
    suffix = row["pid"][4:]
    if suffix == "リュール":
      suffix += "男" if row["gender"] == 1 else "女"
    presets = [a for a in assets if a.get("PresetName") == "個人/" + suffix]
    if presets and presets[0].get("BodyModel") != row["rig"]:
      raise ValueError(f"personal rig mismatch: {row['name']}")
    if not any(a.get("BodyModel") == row["rig"] for a in assets):
      raise ValueError(f"unknown native rig: {row['name']}")
  return 42


if __name__ == "__main__":
  parser = argparse.ArgumentParser(description=__doc__)
  parser.add_argument("person", type=Path)
  parser.add_argument("asset_table", type=Path)
  args = parser.parse_args()
  print(f"Verified {verify(args.person, args.asset_table)} original battle outfits against vanilla XML")
