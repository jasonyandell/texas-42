#!/usr/bin/env python3
"""Reviewer-owned saved-record arithmetic and preservation, without rerunning timing."""
import collections, gzip, hashlib, json, math, statistics, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
OLD = ROOT / 'experiments/native-policy-check-20261004'
def read(path): return json.loads(path.read_text())
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    source = '56794dc3'
    history = {}
    for name in ['astra-sol-20261004','adversarial-20261004','native-frontier-20261004','prefix-state-20261004','demand-bounds-20261004','choice-arena-20261004','resumable-choice-20261004','native-policy-check-20261004']:
        folder = ROOT/'experiments'/name
        count = 0
        for line in (folder/'SHA256SUMS').read_text().splitlines():
            expected, relative = line.split(maxsplit=1)
            relative = relative.strip().lstrip('*')
            path = ROOT/relative if relative.startswith('experiments/') else folder/relative
            assert sha(path) == expected, str(path)
            count += 1
        history[name] = count
    tracked = subprocess.check_output(['git','diff',source,'--','experiments/native-policy-check-20261004','walt'],cwd=ROOT)
    assert not tracked, 'Inherited baseline/production differs from checkpoint'
    summary = read(OLD/'results/native-games/summary.json')
    rows = [read(p) for p in sorted((OLD/'results/native-games').glob('game-*.json'))]
    assert len(rows) == 144 and sum(len(r['moves']) for r in rows) == 4032
    index = {(r['seed'],r['rotation'],r['role']):r for r in rows}
    assert len(index) == len(rows)
    seeds = sorted({r['seed'] for r in rows})
    differences = []
    clusters = []
    for seed in seeds:
        d = [int(index[seed,r,'declaring']['made'])-int(index[seed,r,'defending']['made']) for r in range(4)]
        differences += d
        clusters.append(sum(d)/4)
    assert len(seeds) == 18 and collections.Counter(differences) == {0:70,-1:2}
    mean = sum(clusters)/len(clusters)
    radius = math.sqrt(2*math.log(40)/len(clusters))
    interval = [max(-1,mean-radius),min(1,mean+radius)]
    assert mean == summary['mean_paired_make_advantage'] and interval == summary['hoeffding95']
    cold = OLD/'results/cold-same-policy'
    records = json.loads(gzip.decompress((cold/'records.json.gz').read_bytes()))
    stated = read(cold/'summary.json')
    assert len(records) == 72 and len({r['source_deal_seed'] for r in records}) == 12
    variants = sorted(stated['variants'])
    medians = {}
    for v in variants:
        rr = [r['runs'][v] for r in records]
        assert all('error' not in r['result'] and r['result'].get('evaluation') for r in rr)
        for r in records:
            assert r['runs'][v]['result']['evaluation'] == r['runs']['native']['result']['evaluation']
        totals = [sum(r['runs'][v]['total_s'] for r in records if r['repeat']==rep)*1000 for rep in range(6)]
        medians[v] = statistics.median(totals)
        assert math.isclose(medians[v],stated['variants'][v]['panel_total_median_ms'],rel_tol=1e-12)
        positions = collections.Counter(r['order'].index(v) for r in records)
        assert positions == {pos:12 for pos in range(6)}
    print(json.dumps(dict(source_checkpoint=source,history_manifest_entries=history,unchanged_baseline_and_walt=True,
        game_rows=144,source_deals=18,paired_counts=dict(collections.Counter(differences)),mean=mean,hoeffding95=interval,
        cold_complete_vector_comparisons=len(records)*len(variants),cold_medians_ms=medians,
        scope='Saved artifact arithmetic/preservation only; no fresh timing, player strength, live deployment, or PRNG independence proof.'),indent=2))
if __name__ == '__main__': main()
