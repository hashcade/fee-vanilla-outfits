#!/usr/bin/env python3
"""Push a versioned release tag; CI validates, builds, and publishes the artifacts."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parent.parent


def run(*args: str) -> str:
  return subprocess.run(args, cwd=ROOT, check=True, text=True, capture_output=True).stdout.strip()


def version() -> str:
  value = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
  if not re.fullmatch(r"\d+\.\d+\.\d+", value):
    raise ValueError(f"invalid VERSION: {value}")
  manifest = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
  match = re.search(r'(?m)^version = "(\d+\.\d+\.\d+)"$', manifest)
  lock = (ROOT / "Cargo.lock").read_text(encoding="utf-8")
  locked = re.search(r'name = "fee-vanilla-outfits"\nversion = "([^"]+)"', lock)
  if match is None or match.group(1) != value or locked is None or locked.group(1) != value:
    raise ValueError("VERSION, Cargo.toml, and Cargo.lock must agree")
  return value


def bump_version(current: str, part: str) -> str:
  major, minor, patch = map(int, current.split("."))
  if part == "major":
    return f"{major + 1}.0.0"
  if part == "minor":
    return f"{major}.{minor + 1}.0"
  if part == "patch":
    return f"{major}.{minor}.{patch + 1}"
  raise ValueError(f"unknown increment: {part}")


def write_version(selected: str) -> None:
  # Only change this package's version; dependencies are not resolved again.
  for name, pattern in (
    ("Cargo.toml", r'(?m)^(version = ")\d+\.\d+\.\d+("$)'),
    ("Cargo.lock", r'(name = "fee-vanilla-outfits"\nversion = ")[^"]+("\n)'),
  ):
    path = ROOT / name
    updated, count = re.subn(pattern, lambda m: m[1] + selected + m[2],
      path.read_text(encoding="utf-8"), count=1)
    if count != 1:
      raise ValueError(f"could not update {name}")
    path.write_text(updated, encoding="utf-8")
  (ROOT / "VERSION").write_text(selected + "\n", encoding="utf-8")
  version()


def clean_main() -> None:
  if run("git", "rev-parse", "--show-toplevel") != str(ROOT):
    raise ValueError("run from this repository")
  if run("git", "branch", "--show-current") != "main":
    raise ValueError("release from main")
  if run("git", "status", "--porcelain"):
    raise ValueError("commit changes before releasing")
  run("git", "fetch", "origin", "main", "--tags")
  if run("git", "rev-parse", "HEAD") != run("git", "rev-parse", "origin/main"):
    raise ValueError("main must match origin/main")


def main() -> None:
  parser = argparse.ArgumentParser(description=__doc__)
  group = parser.add_mutually_exclusive_group(required=True)
  group.add_argument("--current", action="store_true")
  group.add_argument("--bump", choices=("patch", "minor", "major"))
  group.add_argument("--check", action="store_true", help="check version files without publishing")
  parser.add_argument("--yes", action="store_true")
  args = parser.parse_args()
  current = version()
  if args.check:
    print(current)
    return
  clean_main()
  selected = current if args.current else bump_version(current, args.bump)
  tag = f"v{selected}"
  existing = run("git", "tag", "--list", tag)
  if existing or run("git", "ls-remote", "--tags", "origin", f"refs/tags/{tag}"):
    raise ValueError(f"tag already exists: {tag}")
  if not args.yes and input(f"Push {tag} for CI publication? [y/N] ").lower() not in ("y", "yes"):
    return
  if args.bump:
    write_version(selected)
    run("git", "add", "VERSION", "Cargo.toml", "Cargo.lock")
    run("git", "commit", "-m", f"chore: release {tag}")
  run("git", "tag", "-a", tag, "-m", tag)
  run("git", "push", "--atomic", "origin", "main", tag)
  print(f"Pushed {tag}. The release workflow owns validation, build, and publication.")


if __name__ == "__main__":
  main()
