#!/usr/bin/env python3
"""Audit and summarize the bounded partnership-review experiment.

Keep selected gym diagnostics separate from fresh, paired complete games.
All scores remain exact fractions; timing and descriptive uncertainty are
reported separately. Raw inputs/results are hash-pinned in each receipt.
"""
import argparse
from collections import Counter, defaultdict
from fractions import Fraction
import json
import math
from pathlib import Path
import random
import statistics

import campaign as c
import gym
from match import report as match_report
from partner_review import investigate, offers
from rules import information_state, replay_record


def fraction_mean(values):
    values = list(values)
    return str(sum(values, Fraction()) / len(values)) if values else None


def gym_report(directory, gallery):
    cases = dict(gym.gallery(gallery))
    manifest = c.read(directory / 'manifest.json')
    assert manifest['catalog'] == gym.exam_identity(cases)
    rows, changes, routes = [], [], Counter()
    for name, case in cases.items():
        files = [directory / 'items' / (name + '--' + p + '.json')
                 for p in ('l1-default', 'l1-partner-count-review')]
        base, candidate = [c.read(f) for f in files]
        assert base is not None and candidate is not None, ('incomplete', name)
        for row in (base, candidate):
            assert row['grade'] == gym.grade(case['key'], row['response']['choice'])
            assert not row['response']['over_budget']
        b, p = base['response'], candidate['response']
        if b['evaluation'] is not None and p['evaluation'] is not None:
            assert b['evaluation']['choice'] == p['evaluation']['choice'], ('baseline changed',name)
        request = gym.pupil_request(case['request'])
        state = information_state(request)
        q = p.get('review_result') or {}
        status = q.get('status','not-run')
        routes[status] += 1
        if status in ('changed','retained'):
            expected = {a['tile']:a['success_mass'] for a in case['key']['actions']}
            assert q['field_id'] == case['key']['field_id']
            assert q['worlds'] == case['key']['worlds']
            assert q['offers'] == offers(request,state)
            assert all(expected[t] == mass for t,mass in q['values'])
            decision, audit = investigate(request,state,q['baseline'],1,lambda *_:(q,'completed'))
            assert audit['status'] == status and decision == p['choice']
        delta = Fraction(candidate['grade']['success']) - Fraction(base['grade']['success'])
        rows.append(dict(scenario=name,source=case['source'],request=request,
                         baseline=base['grade'],candidate=candidate['grade'],delta=str(delta),
                         baseline_us=b['elapsed_us'],candidate_us=p['elapsed_us'],
                         review=q,artifacts={str(f):gym.file_hash(f) for f in files}))
        if delta: changes.append(dict(scenario=name,delta=str(delta),status=status))
    return dict(schema='sunshine-gym-report-v1',manifest=manifest,positions=len(rows),
                warning='Selected and overlapping gym cases, not representative independent deals.',
                baseline_optimal=sum(r['baseline']['optimal'] for r in rows),
                candidate_optimal=sum(r['candidate']['optimal'] for r in rows),
                baseline_regret=fraction_mean(Fraction(r['baseline']['regret']) for r in rows),
                candidate_regret=fraction_mean(Fraction(r['candidate']['regret']) for r in rows),
                mean_delta=fraction_mean(Fraction(r['delta']) for r in rows),
                baseline_mean_us=statistics.mean(r['baseline_us'] for r in rows),
                candidate_mean_us=statistics.mean(r['candidate_us'] for r in rows),
                routes=dict(routes),changes=changes,rows=rows)


def arena_report(directory):
    spec = c.load(directory)
    official = match_report(directory)
    results = c.complete_results(directory,spec)
    assert len(results) == spec['count'], 'finish the predeclared panel before reporting'
    timings, phases, statuses = defaultdict(list), defaultdict(list), Counter()
    reviewed, artifacts = [], {}
    games = moves = 0
    for result in results:
        seed = result['seed']
        fixture = c.fixture(spec,seed)
        for arm in c.arms_for(spec):
            path = directory / 'seeds' / str(seed) / arm / 'checkpoint.json'
            snap = c.read(path)
            ds = c.validate_checkpoint(snap,spec,fixture,seed,arm)
            assert len(ds)==28
            players = c.players_for(spec,arm,fixture['bidder'])
            record=[]
            for d in ds:
                response, seat = d['response'],d['seat']
                request=dict(decl=fixture['decl'],bid=30,bidder=fixture['bidder'],seat=seat,
                             hand=fixture['hands'][seat],plays=record.copy(),seed=c.PUBLIC_SEED)
                state=information_state(request)
                name=players[seat].name
                timings[name].append(response['elapsed_us'])
                for phase in response['phases']:
                    phases[name+' / '+phase['name']].append(phase['elapsed_us'])
                q=response.get('review_result')
                if q is not None:
                    statuses[q['status']]+=1
                    assert q['baseline']==response['evaluation']['choice']
                    assert q['offers']==offers(request,state)
                    if q['status'] in ('changed','retained'):
                        decision,audit=investigate(request,state,q['baseline'],1,lambda *_:(q,'completed'))
                        assert audit['status']==q['status'] and decision==response['choice']
                    else:
                        assert response['choice']==q['baseline']
                    if q['status']!='inactive':
                        reviewed.append(dict(seed=seed,arm=arm,ply=len(record)//2,request=request,
                                             choice=response['choice'],review=q))
                record.extend([seat,response['choice']])
            points,_,remaining,trick=replay_record(fixture['hands'],record,fixture['decl'],fixture['bidder'])
            assert sum(points)==42 and not trick and all(not h for h in remaining)
            assert (points[fixture['bidder']%2]>=30)==result['arms'][arm]['made']
            artifacts[str(path)]=gym.file_hash(path)
            games+=1; moves+=len(ds)
    deltas=[r['paired']['seed_delta'] for r in results]
    # Descriptive paired source-deal bootstrap; neither worlds nor moves are units.
    rng=random.Random(20260913)
    boot=sorted(sum(rng.choice(deltas) for _ in deltas)/len(deltas) for _ in range(5000))
    wins=sum(x>0 for x in deltas); losses=sum(x<0 for x in deltas)
    discordant=wins+losses
    sign_p=min(Fraction(1),Fraction(2*sum(math.comb(discordant,k) for k in range(min(wins,losses)+1)),2**discordant))
    return dict(schema='sunshine-arena-report-v1',manifest=spec,match=official,
                audit=dict(games=games,moves=moves,checkpoint_hashes=artifacts),
                mean_make_delta=fraction_mean(Fraction(x) for x in deltas),
                descriptive_paired_bootstrap_95=[boot[125],boot[4874]] if len(set(deltas))>1 else None,
                bootstrap_note='A degenerate bootstrap is omitted; observed ties do not establish zero uncertainty.',
                paired_discordant_sign_p=str(sign_p),bootstrap_samples=5000,
                timing={k:dict(moves=len(v),mean_us=statistics.mean(v),max_us=max(v)) for k,v in timings.items()},
                phases={k:dict(calls=len(v),mean_us=statistics.mean(v),total_us=sum(v)) for k,v in phases.items()},
                review_statuses=dict(statuses),reviewed_positions=reviewed)


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('kind',choices=['gym','arena']);p.add_argument('directory',type=Path)
    p.add_argument('--gallery',type=Path);p.add_argument('--output',type=Path,required=True)
    args=p.parse_args()
    result=gym_report(args.directory,args.gallery) if args.kind=='gym' else arena_report(args.directory)
    result['reporter_sha256']=gym.file_hash(__file__)
    c.atomic(args.output,result)
    print(json.dumps({k:v for k,v in result.items() if k in ('schema','positions','baseline_optimal',
              'candidate_optimal','routes','mean_delta','mean_make_delta','review_statuses','timing')},indent=2))


if __name__=='__main__':main()
