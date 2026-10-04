#!/usr/bin/env python3
import json,gzip
from pathlib import Path
HERE=Path(__file__).resolve().parent
with gzip.open(HERE.parent/'results/matrix-one/inputs-8.json.gz','rt') as f:rows=json.load(f)
assert len(rows)==6
(HERE/'trace-input.json').write_text(json.dumps(rows)+'\n')
print(json.dumps({'roots':len(rows),'seeds':[r['seed'] for r in rows],'worlds':[len(r['worlds']) for r in rows]}))
