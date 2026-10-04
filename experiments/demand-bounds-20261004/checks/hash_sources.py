#!/usr/bin/env python3
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
paths=['experiments/demand-bounds-20261004/native/src/lib.rs','experiments/demand-bounds-20261004/native/src/bounds.rs','experiments/demand-bounds-20261004/native/src/main.rs','experiments/demand-bounds-20261004/checks/runner/src/main.rs','experiments/prefix-state-20261004/native/src/lib.rs','experiments/astra-sol-20261004/tools/run_capped.py','experiments/demand-bounds-20261004/checks/candidate-target/release/demand-bounds-independent-check','experiments/demand-bounds-20261004/checks/prefix-target/release/native-frontier']
hashes={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths}
for relative in ['experiments/demand-bounds-20261004/math/DemandBounds.lean','experiments/demand-bounds-20261004/compare.py','experiments/demand-bounds-20261004/checks/check_checkpoint.py','experiments/demand-bounds-20261004/checks/audit_new_timings.py','experiments/demand-bounds-20261004/checks/check_preserved.py','experiments/demand-bounds-20261004/checks/trace-input.json','experiments/demand-bounds-20261004/native/target/release/native-frontier']:
 hashes[relative]=hashlib.sha256((ROOT/relative).read_bytes()).hexdigest()
for relative in ['reuse.py','reuse/src/main.rs','reuse/target/release/demand-bounds-reuse','results/reuse-final/inputs.json','results/reuse-final/result.json','results/reuse-final/generation.json','checks/audit_reuse.py']:
 p='experiments/demand-bounds-20261004/'+relative
 hashes[p]=hashlib.sha256((ROOT/p).read_bytes()).hexdigest()
before=(ROOT/paths[4]).read_text();after=(ROOT/paths[0]).read_text()
def block(text,start,end):return text[text.index(start):text.index(end)]
for start,end in [('pub fn prepare_actor','#[derive(Clone, Deserialize)]'),('fn solve_batch','fn grow('),('fn validate_frame','pub fn reference')]:assert block(before,start,end)==block(after,start,end),(start,end)
snapshot=Path(__file__).resolve().parent/'SOURCE_HASHES.json';previous=json.loads(snapshot.read_text()) if snapshot.exists() else {}
stable={p:previous.get('sha256',{}).get(p)==hashes[p] for p in hashes if p.startswith('experiments/demand-bounds-20261004/native/')}
out={'sha256':hashes,'previous_native_source_and_binary_still_match':stable,'unchanged_old_sampler_eager_fold_validation_blocks':True,'scope':'Hashes at independent parity test time. Source edits require relevant rerun.'}
(Path(__file__).resolve().parent/'SOURCE_HASHES.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
