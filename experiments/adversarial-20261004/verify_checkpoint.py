#!/usr/bin/env python3
"""Read-only checkpoint checks; run through the capped runner from repo root."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
FIRST = HERE.parent / 'astra-sol-20261004'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

snapshot = json.loads((HERE / 'SNAPSHOT.json').read_text())
changed = [p for p, sha in snapshot['copied_files'].items()
           if not (REPO / p).is_file() or digest(REPO / p) != sha]
assert not changed, changed

# Reuse the already audited independent move replay, stopping before its
# process/protocol and censoring tests. Redirect only immutable input paths.
source = FIRST / 'math/sol/panel_audit.py'
code = source.read_text().split('\ndef protocol_case(kind):')[0]
code = code.replace('panel = ROOT / "results/production-panel"',
                    'panel = REVIEW / "results/phone-holdout"')
code = code.replace('ROOT / "results/production-panel-summary/stdout.log"',
                    'REVIEW / "results/phone-holdout-summary/stdout.log"')
scope = {'__file__': str(source), 'REVIEW': HERE}
exec(compile(code, str(source), 'exec'), scope)
assert scope['checked'] == 896

receipts = []
for path in sorted(HERE.rglob('run.json')):
    if 'target' in path.parts:
        continue
    record = json.loads(path.read_text())
    assert record['allowance_seconds'] <= 295, path
    assert record['elapsed_seconds'] <= 300, path
    assert record['status'] in ('completed', 'failed', 'timeout', 'interrupted'), path
    assert record['runner_returncode'] is not None, path
    receipts.append({'path': str(path.relative_to(HERE)),
                     'status': record['status'],
                     'allowance_seconds': record['allowance_seconds'],
                     'elapsed_seconds': record['elapsed_seconds']})

for folder in ('drive-0815', 'drive-final'):
    directory = HERE / 'incoming' / folder
    provenance = json.loads((directory / 'PROVENANCE.json').read_text())
    entries = provenance if isinstance(provenance, list) else provenance['files']
    for row in entries:
        assert digest(directory / row['title']) == row['sha256'], row

frontier = json.loads((HERE / 'results/frontier-final/summary.json').read_text())
assert frontier['source_sha256'] == digest(HERE / 'frontier.py')
assert frontier['all_full_action_vectors_equal']
holdout = json.loads((HERE / 'results/frontier-holdout/summary.json').read_text())
assert all(p['all_vectors_equal'] for p in holdout['panels'])
print(json.dumps({
    'frozen_files_unchanged': len(snapshot['copied_files']),
    'phone_holdout_replayed_moves': scope['checked'],
    'phone_source_deals': 4,
    'phone_budget_ms_per_side': dict(scope['budgets']),
    'phone_replay_source_sha256': digest(source),
    'frontier_final_source_sha256': frontier['source_sha256'],
    'all_run_receipts_within_cap': True,
    'receipts': receipts,
}, indent=2))
