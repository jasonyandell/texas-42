#!/usr/bin/env python3
"""Final source boundary and actual tested binary identity; lightweight audit."""
import hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/resumable-choice-20261004';OLD=ROOT/'experiments/choice-arena-20261004'
a=(OLD/'native/src/lib.rs').read_text();b=(HERE/'native/src/lib.rs').read_text();same={}
for start,end in [('pub fn prepare_actor','#[derive(Clone, Deserialize)]'),('fn solve_batch','fn grow('),('fn validate_frame','pub fn reference')]:same[start]=a[a.index(start):a.index(end)]==b[b.index(start):b.index(end)]
for p in ['native/src/arena.rs','native/src/bounds.rs']:same[p]=(OLD/p).read_bytes()==(HERE/p).read_bytes()
assert all(same.values()),same
paths=['native/src/lib.rs','native/src/resumable.rs','native/src/arena.rs','native/src/bounds.rs','native/src/main.rs','native/Cargo.toml','native/Cargo.lock','native/target/release/native-frontier','integration/src/main.rs','integration/Cargo.toml','integration/Cargo.lock','integration/target/release/late-choice-player','math/ResumableChoice.lean','player.py','arena.py','compare.py','cold_worker.py','cold_pipeline.py','adapter_compare.py','ablation.py','checks/runner/src/main.rs','checks/runner/src/bin/refusal.rs','checks/resumable-target/release/refusal','checks/runner/Cargo.toml','checks/resumable-target/release/resumable-independent-check','checks/integration-target/release/late-choice-player','checks/old-arena-target/release/native-frontier','checks/old-demand-target/release/native-frontier','checks/check_integration.py','checks/audit_final_timings.py','checks/check_final_pipelines.py','checks/check_games.py','checks/check_preserved.py','checks/hash_sources.py']
sha={p:hashlib.sha256((HERE/p).read_bytes()).hexdigest()for p in paths}
out=dict(unchanged_sampler_fold_validation=same,sha256=sha,scope='Final native/math/integration and independent checker source; actual binaries after relocation. Rebuilt binary bytes need not equal historical ignored binary. Source mutations require applicable parity/arithmetic reruns.')
(HERE/'checks/SOURCE_HASHES.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
