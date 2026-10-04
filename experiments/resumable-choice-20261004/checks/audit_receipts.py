#!/usr/bin/env python3
import json
from pathlib import Path
HERE=Path(__file__).resolve().parents[1];rows=[]
for p in sorted(HERE.rglob('run.json')):
 r=json.loads(p.read_text());assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300 and not r['cleanup_errors'];assert r['child_returncode']is not None;assert r['status']in ['completed','failed'];rows.append(dict(path=str(p.relative_to(HERE)),status=r['status'],elapsed_s=r['elapsed_seconds']))
print(json.dumps(dict(individual_receipts=len(rows),all_capped_and_reaped=True,failed_development_receipts=[r for r in rows if r['status']=='failed'],scope='Every retained run observed at audit time has≤295sallowance,<300selapsed and reaped child. Failures excluded from validation. This audits recorded process-group runner cleanup, not external jobs.'),indent=2))
