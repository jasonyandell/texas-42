"""Compare override margins on existing complete keys; no new source-deal claim."""
from fractions import Fraction
from pathlib import Path
import argparse
import json
import random

import gym


def choice(baseline, values, samples, support, margin):
    if samples < min(8, support): return baseline
    best = baseline
    for tile in sorted(values):
        if values[tile] > values[best]: best = tile
    return best if samples == support or values[best]-values[baseline] >= margin else baseline


def summarize(rows):
    regrets = [Fraction(r['regret']) for r in rows]
    return dict(roots=len(rows), improved=sum(Fraction(r['delta'])>0 for r in rows),
                harmed=sum(Fraction(r['delta'])<0 for r in rows),
                changed=sum(r['choice']!=r['baseline'] for r in rows),
                mean_regret=str(sum(regrets)/len(rows)))


def main(output):
    source = gym.HERE/'campaigns/sunshine-rollout-v1/roots.json'
    report_paths = {stage:Path('/Users/jason/data/texas-42/sunshine-rollout-v1')/stage/'report.json'
                    for stage in ('development','fresh-measurement')}
    all_rows = json.loads(source.read_text()); result = {}; sensitivity = {}
    for stage, rows in all_rows.items():
        source_report = json.loads(report_paths[stage].read_text())
        source_dir = Path(source_report['manifest']['source'])
        by_margin = {m:[] for m in (1,2,3)}
        sampled = {m:[] for m in (1,2,3)}
        for row in rows:
            values = dict(row['values']); rv = row['review']; base = row['baseline']
            def grade(a):
                return dict(id=row['id'],seed=row['seed'],baseline=base,choice=a,
                            delta=str(Fraction(values[a]-values[base],row['worlds'])),
                            regret=str(Fraction(max(values.values())-values[a],row['worlds'])))
            for margin in by_margin:
                a = choice(base,dict(rv['values']),rv['samples'],rv['support'],margin) if rv['status'] in ('changed','retained') else base
                by_margin[margin].append(grade(a))
            case = json.loads((source_dir/'items'/(row['id']+'.json')).read_text())
            assert gym.digest(case)==source_report['manifest']['cases'][row['id']]
            vectors = {a['tile']:{gym.world_key(t['hands']):int(t['success']) for t in a['traces']} for a in case['key']['actions']}
            worlds = sorted(vectors[base]); n = min(64,len(worlds))
            for repeat in range(32):
                seed = int(gym.digest([42060001,row['id'],repeat])[:16],16)
                selected = random.Random(seed).sample(worlds,n)
                estimates = {a:sum(v[w] for w in selected) for a,v in vectors.items()}
                for margin in sampled:
                    a = choice(base,estimates,n,len(worlds),margin) if len(row['offers'])<len(values) else base
                    sampled[margin].append(grade(a))
        result[stage] = {m:dict(summary=summarize(rs),rows=rs) for m,rs in by_margin.items()}
        sensitivity[stage] = {m:summarize(rs) for m,rs in sampled.items()}
    selected = min((1,2,3),key=lambda m:(Fraction(result['development'][m]['summary']['mean_regret']),m))
    value=dict(schema='sunshine-override-study-v1',source_sha256=gym.file_hash(source),
               selected_margin=selected,observed=result,uniform_subset_sensitivity=sensitivity,
               replicas_per_root=32,subset_worlds=64,
               interpretation='Known source positions, exact reference values. Repeated subsets are not independent deals or a calibrated uncertainty interval. Selection uses the saved development prefixes only.')
    gym.atomic(output,value)
    print(gym.canonical(dict(selected_margin=selected,observed={s:{m:r['summary'] for m,r in v.items()} for s,v in result.items()},sensitivity=sensitivity)))


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True)
    main(p.parse_args().output)
