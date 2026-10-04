#!/usr/bin/env python3
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/choice-arena-20261004'
old=(ROOT/'experiments/demand-bounds-20261004/native/src/lib.rs').read_text();new=(HERE/'native/src/lib.rs').read_text();blocks={}
for start,end in [('pub fn prepare_actor','#[derive(Clone, Deserialize)]'),('fn solve_batch','fn grow('),('fn validate_frame','pub fn reference')]:
 blocks[start]=old[old.index(start):old.index(end)]==new[new.index(start):new.index(end)]
assert all(blocks.values()),blocks
paths=['native/src/lib.rs','native/src/arena.rs','native/src/bounds.rs','native/src/main.rs','native/Cargo.toml','native/target/release/native-frontier','math/ChoiceArena.lean','math/DemandBounds.lean','checks/runner/src/main.rs','checks/candidate-target/release/choice-arena-independent-check','checks/baseline-target/release/native-frontier','sampling.py','compare.py','pipeline.py','checks/check_sampling.py','checks/check_pipeline.py','checks/audit_arena_timings.py','cold_worker.py','cold_pipeline.py','checks/check_cold.py']
hashes={p:hashlib.sha256((HERE/p).read_bytes()).hexdigest()for p in paths}
out={'sha256':hashes,'unchanged_sampler_eager_fold_validation':blocks,'scope':'Final independent successful arena extended parity, Lean, sampling/pipeline and arithmetic check source identities. Any source edits need relevant rerun.'}
(HERE/'checks/SOURCE_HASHES.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
