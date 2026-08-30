#!/usr/bin/env bash
# Syncs the npm package face of ydesign: notebooks + version. Run from the
# repo root, then `cd npm && npm publish --access public`.
set -euo pipefail
cd "$(dirname "$0")/.."
version=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
rm -rf npm/notebooks
mkdir -p npm/notebooks
cp notebooks/*.md npm/notebooks/
python3 - "$version" << 'PY'
import json, sys
p = "npm/package.json"
d = json.load(open(p))
d["version"] = sys.argv[1]
json.dump(d, open(p, "w"), indent=2)
open(p, "a").write("\n")
print("npm/package.json version ->", sys.argv[1])
PY
echo "synced. publish with: cd npm && npm publish --access public"
