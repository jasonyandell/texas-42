#!/usr/bin/env python3
import hashlib,json
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
receipts=[]
for path in sorted(HERE.glob('*/run.json')):
    r=json.loads(path.read_text());assert 0<r['allowance_seconds']<=295
    assert r['elapsed_seconds']<=r['allowance_seconds']+3.5
    assert not r['cleanup_errors']
    receipts.append({'path':str(path.relative_to(HERE)),'status':r['status'],'allowance':r['allowance_seconds'],'elapsed':r['elapsed_seconds']})
files=[ROOT/'experiments/native-frontier-20261004/native/src/lib.rs',ROOT/'experiments/native-frontier-20261004/native/src/main.rs',HERE/'runner/src/main.rs',HERE/'scalar_oracle.py',HERE/'independent-panel.json',HERE/'parallel_protocol.py',HERE/'protocol_guards.py',HERE/'reference_protocol.py',HERE/'final_results_audit.py',HERE/'FINAL_RESULTS_AUDIT.json']
result={'receipts':receipts,'source_sha256':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},'all_allowances_le_295':True,'cleanup_errors':False}
(HERE/'AUDIT.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'receipts':len(receipts),'retained_failed_receipts':[r['path'] for r in receipts if r['status']!='completed'],'source_sha256':result['source_sha256']},indent=2))
