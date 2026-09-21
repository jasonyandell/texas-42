#!/usr/bin/env python3
"""Publish compact decision-mining evidence and paired continuation witnesses.

Reads completed assessments only. This does not fit, select or change a policy.
The full responses remain in the external, producer-pinned SQLite assessments.
"""
import argparse
from collections import Counter
from fractions import Fraction as F
import json
from pathlib import Path
import sqlite3
import zlib

import decision_mine as m


def summarize(data):
    misses=[r for r in data if max(r['values'].values())>r['values'][r['baseline']]]
    audit=Counter()
    for row in data:audit.update(row['audit'])
    return dict(assessments=len(data),misses=len(misses),missed_roots=len({r['root'] for r in misses}),
        mean_regret=str(sum((max(r['values'].values())-r['values'][r['baseline']] for r in data),F(0))/len(data)),
        strict_assessments=sum(len(set(r['values'].values()))>1 for r in data),
        detectors=m.diagnostics(data),variance_diagnostic=m.stratification(data),aggregate_audit=dict(audit))


def witnesses(folder,data):
    db=sqlite3.connect(folder/'assessments.sqlite');db.row_factory=sqlite3.Row
    misses=[r for r in data if max(r['values'].values())>r['values'][r['baseline']]]
    answer=[]
    for row in sorted(misses,key=lambda r:-(max(r['values'].values())-r['values'][r['baseline']])):
        saved=db.execute('SELECT * FROM jobs WHERE id=?',(row['root']+f'-{row["rep"]}',)).fetchone()
        value=json.loads(zlib.decompress(saved['payload']))
        alternative=max(row['values'],key=row['values'].get);baseline=row['baseline'];n=len(value['worlds'])
        paths={(t['world'],t['action']):t for t in value['traces']}
        flips={}
        for sign in ('help','harm'):
            indices=[i for i in range(n) if (paths[i,alternative]['made']-paths[i,baseline]['made'])==(1 if sign=='help' else -1)]
            if indices:
                i=indices[0];pair=[paths[i,a] for a in (baseline,alternative)]
                used=sorted({d for t in pair for d in t['decisions']}|{value['baseline_decision']})
                flips[sign]=dict(world=i,hands=value['worlds'][i],traces=pair,
                    decisions={str(k):value['decisions'][k] for k in used})
        answer.append(dict(root=row['root'],rep=row['rep'],source_deal=row['deal_seed'],request=value['request'],
            baseline=baseline,alternative=alternative,values={str(a):str(v) for a,v in row['values'].items()},
            threat_presence=str(row['threat']),worlds=n,
            paired_counts=dict(help=sum(b<a for b,a in zip(row['vectors'][baseline],row['vectors'][alternative])),
                harm=sum(b>a for b,a in zip(row['vectors'][baseline],row['vectors'][alternative]))),
            original_evaluation=value['decisions'][value['baseline_decision']]['response'],
            examples=flips,receipt_sha256=row['sha256']))
    db.close()
    return dict(schema='kiln-decision-witnesses-v1',cases=answer)


def publish(source,destination):
    destination.mkdir(parents=True,exist_ok=True)
    summary={}
    for split in ('train','validation'):
        data=m.rows(source/split)
        summary[split]=summarize(data)
        m.immutable_json(destination/f'{split}-misses.json',witnesses(source/split,data))
    m.immutable_json(destination/'summary.json',summary)
    for name in ('fit.json','result.json','train-audit.json','validation-audit.json','resumption-audit.json','fresh-source-audit.json'):
        m.immutable_json(destination/name,m.read(source/name))
    for q in m.queries():
        path=destination/'queries'/(q['name']+'.scheme');path.parent.mkdir(exist_ok=True)
        if path.exists() and path.read_text()!=q['source']:raise ValueError('Published query changed')
        path.write_text(q['source'])
    return dict(destination=str(destination),missed_position_seed_cases=sum(s['misses'] for s in summary.values()),
                distinct_missed_positions=sum(s['missed_roots'] for s in summary.values()))


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source',type=Path);parser.add_argument('destination',type=Path)
    args=parser.parse_args()
    print(json.dumps(publish(args.source,args.destination)))
