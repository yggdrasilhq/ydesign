#!/usr/bin/env bash
# Public notebook check. Examples must be invented; private screenshots stay outside git.
set -euo pipefail
cd "$(dirname "$0")/.."
python3 - <<'PY'
import pathlib, re, subprocess, sys
paths = subprocess.check_output(['git','ls-files','--cached','--others','--exclude-standard','-z']).decode().split('\0')
bad = []
for name in paths:
    p = pathlib.Path(name)
    if not p.is_file() or p.suffix in {'.png','.jpg','.jpeg','.webp','.lock'}:
        continue
    try:
        text = p.read_text()
    except UnicodeError:
        continue
    for line_no, line in enumerate(text.splitlines(), 1):
        homes = re.findall(r'/home/([a-z][a-z0-9_-]*)', line)
        if any(h not in {'user','example','test','alice','bob','runner'} for h in homes):
            bad.append(f'{name}:{line_no}: personal home path')
        if re.search(r'\b(?:192\.168\.\d+\.\d+|10\.\d+\.\d+\.\d+|172\.(?:1[6-9]|2\d|3[01])\.\d+\.\d+)\b', line):
            bad.append(f'{name}:{line_no}: private network address')
print('\n'.join(bad) if bad else 'privacy: source examples clean; binary assets require visual review')
sys.exit(bool(bad))
PY
