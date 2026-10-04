#!/usr/bin/env python3
"""Pin reviewer sources and actual binaries; audit retained capped executions."""
import hashlib,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
CHECK=Path(__file__).resolve().parent;HERE=CHECK.parent;ROOT=HERE.parents[1]
def main():
    receipts=[]
    for path in sorted(HERE.rglob('run.json')):
        if 'target'in path.parts:continue
        r=json.loads(path.read_text());assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300
        assert r['status']in['completed','failed']and r['child_returncode']is not None and not r['cleanup_errors']
        receipts.append(dict(path=str(path.relative_to(HERE)),status=r['status'],elapsed_seconds=r['elapsed_seconds']))
    paths=[p for p in CHECK.rglob('*')if p.is_file()and not any(x in p.parts for x in['target','__pycache__'])and p.suffix in['.py','.rs','.lean','.mjs','.toml','.lock']]
    paths += [CHECK/'REVIEW.md',HERE/'adapter/src/lib.rs',HERE/'adapter/src/main.rs',HERE/'adapter/Cargo.toml',HERE/'adapter/Cargo.lock',HERE/'experiment.py',HERE/'snapshots/experiment-measured.py',HERE/'snapshots/experiment-parallel-measured.py',HERE/'extension.py',HERE/'snapshots/extension-measured.py',HERE/'plan.json',HERE/'earlier-plan.json',HERE/'fixtures.json',HERE/'adapter/target/release/higher-k-player',HERE/'adapter/target/wasm32-unknown-unknown/release/higher_k_player.wasm',CHECK/'target/release/higher-k-independent-check']
    hashes={str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest()for path in sorted(set(paths))}
    for p in [HERE/'ARTIFACTS.json',HERE/'snapshots/adapter-measured.rs',HERE/'adapter/target/measured/higher-k-player',HERE/'adapter/target/measured/higher_k_player.wasm']:
        hashes[str(p.relative_to(ROOT))]=hashlib.sha256(p.read_bytes()).hexdigest()
    for row in json.loads((HERE/'ARTIFACTS.json').read_text()).values():
        if 'archive_path'in row:
            path=ROOT/row['archive_path'];hashes[row['archive_path']]=hashlib.sha256(path.read_bytes()).hexdigest()
    for path in[HERE/'restore_artifacts.py',HERE/'budget_intervals.py']:
        hashes[str(path.relative_to(ROOT))]=hashlib.sha256(path.read_bytes()).hexdigest()
    (CHECK/'SOURCE_HASHES.json').write_text(json.dumps(hashes,indent=2)+'\n')
    print(json.dumps(dict(receipts=len(receipts),failed_receipts=[r for r in receipts if r['status']=='failed'],source_and_actual_binary_hashes=len(hashes),allowances_at_most295=True,elapsed_below300=True,all_children_reaped=True,receipts_checked=receipts),indent=2))
if __name__=='__main__':main()
