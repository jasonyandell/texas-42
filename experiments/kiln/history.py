#!/usr/bin/env python3
"""Read-only reconstruction of empirical bid changes and their original evidence.

Trial order, never asynchronous completion order, defines each sample prefix.
These are changes in an estimate, not counterfactual changes to an earlier game.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path
import sqlite3
import zlib

import played as p
from rules import TILES, trick_points, winner


def identity(value):
    return hashlib.sha256(p.canonical(value).encode()).hexdigest()


def open_snapshot(directory):
    path = (Path(directory)/'played.sqlite').resolve()
    db = sqlite3.connect(path.as_uri()+'?mode=ro',uri=True,timeout=30)
    db.row_factory = sqlite3.Row
    db.execute('BEGIN')
    return db


def estimate(histogram, threshold):
    n = sum(histogram)
    tails = [sum(histogram[b:]) for b in range(30,43)]
    numerator,denominator = threshold
    bid = max((b for b,m in zip(range(30,43),tails) if n and m*denominator >= n*numerator),default=None)
    return {'games':n,'histogram':histogram.copy(),'tails30_42':tails,'recommended_bid':bid}


def cell_history(cell,samples,threshold):
    samples = sorted(samples,key=lambda s:s['trial'])
    if [s['trial'] for s in samples] != list(range(len(samples))):
        raise ValueError('Sample prefix has gaps or duplicate trials')
    hist = [0]*43
    previous = estimate(hist,threshold)
    chain = identity({'schema':'kiln-prefix-v1','cell':cell,'threshold':threshold})
    events,milestones,observations = [],{},[]
    for sample in samples:
        if type(sample['score']) is not int or not 0<=sample['score']<=42:
            raise ValueError('Invalid final score')
        old_chain = chain
        chain = identity({'previous':chain,'trial':sample['trial'],'receipt_sha256':sample['receipt_sha256']})
        observations.append(sample)
        hist[sample['score']] += 1
        current = estimate(hist,threshold)
        if previous['recommended_bid'] != current['recommended_bid']:
            pn,pd = threshold
            changed = [b for b,old,new in zip(range(30,43),previous['tails30_42'],current['tails30_42'])
                       if (bool(previous['games']) and old*pd >= previous['games']*pn)
                       != (new*pd >= current['games']*pn)]
            events.append({'kind':'initial-estimate' if not previous['games'] else 'recommendation-change',
                'cell_id':cell['cell_id'],'added_trial':sample['trial'],'added_game':sample,
                'before':previous,'after':current,'thresholds_crossed':changed,
                'before_prefix_sha256':old_chain,'after_prefix_sha256':chain})
        if current['games'] in (8,40,160,320,640,1280):
            milestones[str(current['games'])] = {**current,'prefix_sha256':chain}
        previous = current
    return {**cell,'observations':observations,'latest':previous,'prefix_sha256':chain,
            'milestones':milestones,'changes':events}


def sampling_contract():
    worlds = math.comb(21,7)*math.comb(14,7)
    return {'unit':'one complete bid30 game with the fixed deployed player profile',
        'intended_hidden_distribution':'uniform labeled completions of the three unseen seven-tile hands',
        'intended_hidden_world_probability':['1',str(worlds)],
        'proposal_matches_target':True,'importance_weight':['1','1'],'adaptive_sampling':False,
        'hidden_seed_recipe':'SHA256 kiln-played-hidden-v1/seat/canonical-own-hand/trial; first 8 bytes little endian',
        'policy_seed_recipe':'separate SHA256 kiln-played-policy-v1/seat/canonical-own-hand/trial; low 32 bits',
        'pairing':'Same completion and policy seed across declarations of one hand/trial.',
        'selection':'All completed trials retained, including non-changing controls; stage screening is recorded by the campaign manifest.',
        'interpretation':'The probability is the intended Monte Carlo target, not a theorem about finite PRNG seed frequencies.'}


def load_cell(db,cell_id):
    row = db.execute('''SELECT c.*,h.seed,h.seat,h.tiles FROM cells c JOIN hands h ON h.id=c.hand_id
                        WHERE c.id=?''',(cell_id,)).fetchone()
    if row is None:raise ValueError('Unknown cell')
    return {'cell_id':row['id'],'hand_id':row['hand_id'],'deal_seed':row['seed'],
            'seat':row['seat'],'hand':json.loads(row['tiles']),'decl':row['decl'],'audit_bypass':bool(row['audited'])}


def load_samples(db,cell_id):
    return [dict(row) for row in db.execute('''SELECT j.id AS job_id,j.trial,g.score,
        g.sha256 AS receipt_sha256,g.producer FROM jobs j JOIN games g ON j.id=g.job_id
        WHERE j.cell_id=? ORDER BY j.trial''',(cell_id,))]


def report(db):
    manifest = p.manifest(db)
    cells = [cell_history(load_cell(db,row['id']),load_samples(db,row['id']),manifest['recommendation'])
             for row in db.execute('SELECT id FROM cells ORDER BY id')]
    events = [event for cell in cells for event in cell['changes'] if event['kind']=='recommendation-change']
    value = {'schema':'kiln-history-v1','manifest':manifest,'sampling':sampling_contract(),'cells':cells,
        'summary':{'cells':len(cells),'games':sum(c['latest']['games'] for c in cells),
            'recommendation_changes':len(events),'changes_after_eight_games':sum(e['before']['games']>=8 for e in events),
            'changes_after_forty_games':sum(e['before']['games']>=40 for e in events)},
        'scope':'Per-declaration bid recommendation from growing empirical game prefixes. Does not infer a causal feature, compare declarations at unequal depths, or trace inner solver worlds.'}
    value['id'] = identity(value)
    return value


def game_evidence(db,directory,cell_id,trial):
    row = db.execute(p.JOB_SQL+' WHERE j.cell_id=? AND j.trial=?',(cell_id,trial)).fetchone()
    if row is None:raise ValueError('Unknown game')
    stored = db.execute('SELECT * FROM games WHERE job_id=?',(row['id'],)).fetchone()
    if stored is None:raise ValueError('Game has not completed')
    raw = zlib.decompress(stored['payload'])
    if hashlib.sha256(raw).hexdigest()!=stored['sha256']:raise ValueError('Receipt hash mismatch')
    receipt = json.loads(raw)
    p.validate_game(p.game_request(row),receipt)
    text = f'{row["seat"]}/{row["tiles"]}/{trial}'
    producer_path = Path(directory)/'producers'/stored['producer']/'producer.json'
    producer = json.loads(producer_path.read_text())
    tricks = []
    for trick in range(7):
        plays = list(zip(receipt['record'][8*trick:8*trick+8:2],receipt['record'][8*trick+1:8*trick+8:2]))
        tricks.append({'trick':trick+1,'plays':[{'seat':a,'tile':t,'label':label(t)} for a,t in plays],
            'winner':winner(plays,receipt['decl']),'points':trick_points(plays)})
    return {'job_id':row['id'],'trial':trial,'receipt_sha256':stored['sha256'],'producer_id':stored['producer'],
        'producer':producer,'hidden_completion_seed':str(p.seeded('kiln-played-hidden-v1/'+text)),
        'policy_seed':receipt['seed'],'receipt':receipt,'tricks':tricks}


def label(tile):
    hi,lo = TILES[tile]
    return f'{hi}-{lo}'


def explain(db,directory,cell_id,n,against_trial=None):
    cfg = p.manifest(db)
    cell = load_cell(db,cell_id)
    samples = load_samples(db,cell_id)
    if n<1 or n>len(samples):raise ValueError('Requested prefix is unavailable')
    history = cell_history(cell,samples[:n],cfg['recommendation'])
    prior = cell_history(cell,samples[:n-1],cfg['recommendation'])
    added = game_evidence(db,directory,cell_id,n-1)
    event = next((e for e in history['changes'] if e['after']['games']==n),None)
    value = {'schema':'kiln-change-evidence-v1','cell':cell,'threshold':cfg['recommendation'],
        'sampling':sampling_contract(),'before':prior['latest'],'after':history['latest'],
        'event':event,'before_prefix_sha256':prior['prefix_sha256'],'after_prefix_sha256':history['prefix_sha256'],
        'prefix_observations':history['observations'],'added_game':added,
        'interpretation':'The added observation changes the aggregate; it does not alter the earlier games. Trial worlds and policy seeds both vary. Contrasts are descriptive, not causal.'}
    if against_trial is not None:
        if not 0<=against_trial<n-1:raise ValueError('Contrast must refer to an earlier trial')
        old = game_evidence(db,directory,cell_id,against_trial)
        value['comparison_game'] = old
        value['contrast'] = {'sampling_changes':['hidden completion','policy seed'],
            'old_score':old['receipt']['score'],'new_score':added['receipt']['score'],
            'old_opening':old['receipt']['decisions'][0]['response']['choice'],
            'new_opening':added['receipt']['decisions'][0]['response']['choice'],
            'hands':[{'seat':seat,'removed':[label(t) for t in sorted(set(a)-set(b))],
                'added':[label(t) for t in sorted(set(b)-set(a))]}
                for seat,(a,b) in enumerate(zip(old['receipt']['hands'],added['receipt']['hands']))]}
    value['id'] = identity(value)
    return value


def save(directory,value,output=None):
    if output is None:
        parent = Path(directory)/'history';parent.mkdir(exist_ok=True)
        output = parent/(value['schema']+'-'+value['id']+'.json')
    p.atomic_json(output,value)
    return str(output)


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    commands=parser.add_subparsers(dest='command',required=True)
    for name in ('report','explain'):
        sub=commands.add_parser(name);sub.add_argument('directory');sub.add_argument('--output')
        if name=='explain':
            sub.add_argument('--cell',type=int,required=True);sub.add_argument('--n',type=int,required=True)
            sub.add_argument('--against-trial',type=int)
    args=parser.parse_args();db=open_snapshot(args.directory)
    try:
        value=report(db) if args.command=='report' else explain(db,args.directory,args.cell,args.n,args.against_trial)
    finally:db.close()
    path=save(args.directory,value,args.output)
    print(p.canonical({'output':path,'id':value['id'],'summary':value.get('summary'),
        'before_bid':value.get('before',{}).get('recommended_bid'),
        'after_bid':value.get('after',{}).get('recommended_bid')}))
