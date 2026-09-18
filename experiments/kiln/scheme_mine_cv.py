#!/usr/bin/env python3
"""Frozen-coefficient measurement-efficiency audit for recognized singleton queries."""
import argparse
from collections import defaultdict
from fractions import Fraction as F
import json
from pathlib import Path
import random

import scheme_mine as m


def prevalence(query, row):
    if query['kind'] == 'literal' and len(query['clause']) == 1:
        holder,tile = divmod(query['clause'][0],28)
    elif query['kind'] == 'relational':
        holder,relation,props = query['descriptor']
        if relation != 'none' or list(props) != ['boss']:
            raise ValueError('No exact singleton prevalence implementation for this query')
        bosses = [tile for tile in range(28) if 'boss' in m.properties(tile,row['decl'])]
        if not bosses: return F(0)
        if len(bosses) != 1: raise ValueError('Not a singleton boss')
        tile = bosses[0]
    else: raise ValueError('Unsupported query; do not guess its probability')
    if holder not in (1,2,3): raise ValueError('Expected hidden holder')
    return F(0) if tile in row['hands'][row['seat']] else F(1,3)


def beta(rows, query):
    cells = defaultdict(list)
    for row in rows:
        cells[row['cell_id']].append((int(m.query_match(query,row)),int(row['score']<30)))
    cross = square = F(0)
    for values in cells.values():
        n = len(values); sx = sum(x for x,_ in values); sy = sum(y for _,y in values)
        cross += sum(x*y for x,y in values)-F(sx*sy,n)
        square += sx-F(sx*sx,n)
    return cross/square if square else F(0)


def fit(corpus, output):
    primary = json.loads((output/'fit.json').read_text())
    m.verify_identity(primary)
    rows = m.t.split(json.loads((corpus/'rows.json').read_text()))['discovery']
    if m.t.digest(rows) != primary['training_sha256']: raise ValueError('Training identity changed')
    records = []
    for query in primary['queries']:
        if query['name'] not in primary['primaries']: continue
        for row in rows: prevalence(query,row)  # Reject an unsupported identity.
        records.append({'name':query['name'],'beta':m.t.ratio(beta(rows,query))})
    value = {'schema':'kiln-mining-control-variate-v1','fit_id':primary['id'],
             'protocol_sha256':m.t.p.digest(Path(__file__).with_name('MINING-PROBE-V2-CV.md')),
             'training_sha256':m.t.digest(rows),'coefficients':records}
    value['id'] = m.t.digest(value)
    m.immutable_json(output/'cv-fit.json',value)
    print(m.t.p.canonical(value))
    return value


def variance(values):
    n = len(values)
    if n < 2: raise ValueError('Need at least two observations')
    return (sum(x*x for x in values)-sum(values)**2/n)/(n-1)


def measure(rows, query, coefficient):
    cells = defaultdict(list)
    for row in rows: cells[row['cell_id']].append(row)
    reports = []
    for identity,observations in sorted(cells.items()):
        probabilities = {prevalence(query,row) for row in observations}
        if len(probabilities) != 1: raise ValueError('Cell does not have one public event probability')
        probability = probabilities.pop()
        ys = [F(row['score']<30) for row in observations]
        xs = [F(m.query_match(query,row)) for row in observations]
        if not probability and any(xs): raise ValueError('Impossible event matched')
        zs = [y-coefficient*(x-probability) for x,y in zip(xs,ys)]
        reports.append({'cell_id':identity,'deal_seed':observations[0]['deal_seed'],
            'eligible':bool(probability),'raw_variance':variance(ys),'corrected_variance':variance(zs),
            'raw_mean':sum(ys)/len(ys),'corrected_mean':sum(zs)/len(zs)})
    summaries = {}
    for name,subset in [('all',reports),('eligible',[r for r in reports if r['eligible']])]:
        if not subset:
            summaries[name] = {'cells':0,'variance_ratio':None,'ratio_bootstrap_95':None}
            continue
        vy = sum(r['raw_variance'] for r in subset); vz = sum(r['corrected_variance'] for r in subset)
        by_deal = defaultdict(lambda:[F(0),F(0)])
        for r in subset:
            by_deal[r['deal_seed']][0] += r['raw_variance']
            by_deal[r['deal_seed']][1] += r['corrected_variance']
        groups = list(by_deal.values()); rng = random.Random('kiln-mining-cv-bootstrap-v1'); draws=[]
        for _ in range(1999):
            picked = [rng.choice(groups) for _ in groups]
            denominator = sum(g[0] for g in picked)
            if denominator: draws.append(sum(g[1] for g in picked)/denominator)
        draws.sort()
        summaries[name] = {'cells':len(subset),'average_raw_variance':m.t.ratio(vy/len(subset)),
            'average_corrected_variance':m.t.ratio(vz/len(subset)),
            'variance_ratio':m.t.ratio(vz/vy) if vy else None,
            'ratio_bootstrap_95': [m.t.ratio(draws[int((len(draws)-1)*q)]) for q in (.025,.975)] if draws else None,
            'source_deal_groups':len(groups),'bootstrap_draws':len(draws),
            'mean_raw_failure':m.t.ratio(sum(r['raw_mean'] for r in subset)/len(subset)),
            'mean_corrected_failure':m.t.ratio(sum(r['corrected_mean'] for r in subset)/len(subset))}
    return {'summaries':summaries,'cells':[{k:m.t.ratio(v) if isinstance(v,F) else v for k,v in r.items()} for r in reports]}


def evaluate(output):
    frozen = json.loads((output/'cv-fit.json').read_text())
    primary = json.loads((output/'fit.json').read_text())
    outcome = json.loads((output/'result.json').read_text())
    rows = json.loads((output/'fresh-rows.json').read_text())
    for value in (frozen,primary,outcome): m.verify_identity(value)
    if frozen['fit_id'] != primary['id'] or outcome['fit_id'] != primary['id'] or m.t.digest(rows) != outcome['fresh_rows_sha256']:
        raise ValueError('Fresh/fitted identities disagree')
    queries = {q['name']:q for q in primary['queries']}
    result = {'schema':'kiln-mining-control-variate-result-v1','coefficient_fit_id':frozen['id'],
        'fresh_result_id':outcome['id'],'queries':[
            {'name':record['name'],'beta':record['beta'],**measure(rows,queries[record['name']],F(*record['beta']))}
            for record in frozen['coefficients']],
        'interpretation':'Fixed-player estimator variance, not stronger/faster play. Ten-group bootstrap is descriptive.'}
    result['id'] = m.t.digest(result)
    m.immutable_json(output/'cv-result.json',result)
    print(m.t.p.canonical({k:v if k!='queries' else [{a:b for a,b in q.items() if a!='cells'} for q in v] for k,v in result.items()}))
    return result


if __name__ == '__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command',choices=['fit','evaluate']);parser.add_argument('--output',required=True,type=Path)
    parser.add_argument('--corpus',type=Path)
    args=parser.parse_args()
    if args.command=='fit':
        if args.corpus is None: parser.error('fit needs --corpus')
        fit(args.corpus,args.output)
    else: evaluate(args.output)
