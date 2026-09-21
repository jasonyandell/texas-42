#!/usr/bin/env python3
"""Bounded outcome-first relational discovery and independent fresh replication.

Python 3.10+. Uses the frozen v1 corpus for fitting; fresh outcomes are opened
only by `evaluate`, which requires immutable fitted queries first.
"""
import argparse
from collections import defaultdict
from fractions import Fraction
from functools import lru_cache
import hashlib
from itertools import combinations
import json
from pathlib import Path
import subprocess
import zlib

import threat_probe as t
from rules import TILES, called, context, follows

VERSION = 'kiln-scheme-mining-v2.1'


def immutable_json(path, value):
    if path.exists():
        if t.p.canonical(json.loads(path.read_text())) != t.p.canonical(value):
            raise ValueError(f'Refusing to replace immutable artifact {path}')
    else:
        t.p.atomic_json(path,value)


def rank(tile, decl):
    hi,lo = TILES[tile]
    return hi if hi == lo and decl == 7 else 12 if hi == lo else hi+lo


def key(tile, led, decl):
    return (2 if called(tile,decl) else 1 if follows(tile,led,decl) else 0,rank(tile,decl))


@lru_cache(None)
def properties(tile, decl):
    hi,lo = TILES[tile]
    count = hi+lo if hi+lo in (5,10) else 0
    trump = called(tile,decl)
    boss = trump and not any(called(other,decl) and rank(other,decl)>rank(tile,decl) for other in range(28))
    return tuple(sorted(('double' if hi == lo else '!double',f'count={count}',
                         'trump' if trump else '!trump','boss' if boss else '!boss')))


@lru_cache(None)
def tile_mask(decl, props):
    return sum(1 << tile for tile in range(28) if set(props) <= set(properties(tile,decl)))


@lru_cache(None)
def beating_mask(decl, own, relation):
    if relation == 'none': return (1 << 28)-1
    eligible = [tile for tile in own if relation == 'beats' or
                (relation == 'beats-called' and called(tile,decl)) or
                (relation == 'beats-off' and not called(tile,decl))]
    return sum(1 << tile for tile in range(28) if any(
        key(tile,context(mine,decl),decl)>key(mine,context(mine,decl),decl) for mine in eligible))


def descriptor_query(desc):
    holder,relation,props = desc
    roles = [f'(chair {name})' for name in t.ROLES] + ['(domino hidden)']
    atoms = ['(viewer me)','(successor me after)','(partner me mate)','(successor mate before)',
             f'(holds {t.ROLES[holder]} hidden)']
    for prop in props:
        atom = '(double hidden)' if prop.lstrip('!') == 'double' else \
               '(in hidden q*)' if prop.lstrip('!') == 'trump' else \
               '(boss hidden q*)' if prop.lstrip('!') == 'boss' else f'(count hidden {prop.split("=")[1]})'
        atoms.append(f'(not {atom})' if prop.startswith('!') else atom)
    if relation != 'none':
        roles += ['(domino own)','(context led)']
        atoms += ['(own-legal own)','(leads-context own led)','(beats hidden own led)']
        if relation == 'beats-called': atoms.append('(in own q*)')
        if relation == 'beats-off': atoms.append('(not (in own q*))')
    return '(fix\n  (roles ' + ' '.join(roles) + ')\n  (out)\n  (case\n    ' + '\n    '.join(atoms) + '))\n'


def matches(desc, row):
    holder,relation,props = desc
    hand = row['hands'][(row['seat']+holder)%4]
    bits = sum(1 << tile for tile in hand)
    return bool(bits & tile_mask(row['decl'],tuple(props)) &
                beating_mask(row['decl'],tuple(row['hands'][row['seat']]),relation))


def library(seed_rows):
    """Lift witnessed ground facts to one hidden role; no outcome lookups here."""
    found = {}
    for row in seed_rows:
        decl,own = row['decl'],tuple(row['hands'][row['seat']])
        for holder in (1,2,3):
            for tile in row['hands'][(row['seat']+holder)%4]:
                for relation in ('none','beats','beats-called','beats-off'):
                    if not beating_mask(decl,own,relation) & (1 << tile): continue
                    for size in (0,1,2):
                        for props in combinations(properties(tile,decl),size):
                            desc = (holder,relation,props)
                            found.setdefault(desc,{'job_id':row['job_id'],'hidden_tile':tile,
                                'relative_holder':holder,'true_properties':list(properties(tile,decl))})
    return sorted(found),found


class QueryIndex(t.Index):
    def __init__(self, rows, queries):
        super().__init__(rows)
        self.query_masks = {}
        for query in queries:
            self.query_masks[query['name']] = sum(1 << i for i,row in enumerate(rows) if query_match(query,row))

    def membership(self, name):
        return self.query_masks[name]


def query_match(query, row):
    if query['kind'] == 'literal': return set(query['clause']) <= set(t.hidden_facts(row))
    return matches(query['descriptor'],row)


def query_record(desc, name):
    return {'name':name,'kind':'relational','descriptor':desc,'source':descriptor_query(desc)}


def atom_count(desc):
    _,relation,props = desc
    return 5+len(props)+(3+int(relation != 'beats') if relation != 'none' else 0)


def verify_identity(value):
    content = dict(value)
    identity = content.pop('id')
    if t.digest(content) != identity: raise ValueError('Content identity changed')


def audit(rows, queries, output, binary, tag):
    """Audit label-free masks using the actual parser/compiler/query evaluator."""
    folder = output/tag; folder.mkdir(parents=True,exist_ok=True)
    inputs = []
    for i,row in enumerate(rows):
        bits = [sum(1 << tile for tile in hand) for hand in row['hands']]
        inputs.append(' '.join(map(str,[i,row['decl'],row['seat'],*bits])))
    input_text = '\n'.join(inputs)+'\n'
    parts = []
    for offset in range(0,len(queries),64):
        batch = queries[offset:offset+64]
        paths = []
        for query in batch:
            path = folder/(query['name']+'.scheme')
            path.write_text(query['source']); paths.append(str(path))
        native = subprocess.run([str(binary),*paths],input=input_text,text=True,
                                capture_output=True,check=True,timeout=120)
        lines = native.stdout.splitlines()
        if len(lines) != len(rows): raise ValueError('Native Scheme audit omitted rows')
        for i,(row,line) in enumerate(zip(rows,lines)):
            identity,mask = map(int,line.split())
            expected = sum(1 << j for j,query in enumerate(batch) if query_match(query,row))
            if identity != i or mask != expected:
                bad = [query['name'] for j,query in enumerate(batch) if (mask ^ expected) & (1 << j)]
                raise ValueError(f'Scheme mismatch row {i}: {bad}')
        parts.append(hashlib.sha256(native.stdout.encode()).hexdigest())
    return {'queries':len(queries),'rows':len(rows),'checks':len(queries)*len(rows),'mismatches':0,
            'output_hashes':parts,'binary_sha256':t.p.digest(binary)}


def mechanics_rows(seed_rows):
    rows = []
    for row in seed_rows:
        for decl in t.p.DECLS:
            for turn in range(4):
                hands = [None]*4
                for seat,hand in enumerate(row['hands']): hands[(seat+turn)%4] = hand
                rows.append({**row,'seat':(row['seat']+turn)%4,'hands':hands,'decl':decl})
    return rows


def fit(corpus, output, binary):
    output.mkdir(parents=True,exist_ok=True)
    if (output/'fit.json').exists(): raise FileExistsError('Fit is already frozen; use a new output')
    rows = json.loads((corpus/'rows.json').read_text())
    original = json.loads((corpus/'result.json').read_text())
    if t.digest(rows) != original['input_sha256']: raise ValueError('v1 input changed')
    training = t.split(rows)['discovery']
    by_job = {row['job_id']:row for row in training}
    seeds = [by_job[job] for job in original['search']['seed_jobs']]
    if any(row['score'] >= 30 for row in seeds): raise ValueError('Seed world is not a discovery failure')
    descriptors,witnesses = library(seeds)
    queries = [query_record(desc,f'library-{i:03d}') for i,desc in enumerate(descriptors)]
    index = QueryIndex(training,queries)
    ranked = []
    for query in queries:
        mask = index.membership(query['name'])
        if mask.bit_count() < 36 or index.excess(mask) <= 0: continue
        if sum(bool(mask & d) for d in index.deals.values()) < 6: continue
        # Number of actual predicate atoms, not serialization length.
        atoms = atom_count(query['descriptor'])
        ranked.append((index.merit(mask),atoms,query,mask))
    ranked.sort(key=lambda x:(-x[0],x[1],x[2]['descriptor']))
    chosen,masks = [],[]
    for merit,atoms,query,mask in ranked:
        if any(5*(mask & old).bit_count() >= 4*(mask | old).bit_count() for old in masks): continue
        selected = {**query,'name':f'relational-{len(chosen)}','training_merit':t.ratio(merit),
                    'atom_count':atoms,'witness':witnesses[query['descriptor']],
                    'training':index.report(query['name'])}
        chosen.append(selected); masks.append(mask)
        if len(chosen) == 4: break
    if len(chosen) != 4: raise ValueError('Protocol could not select four relational candidates')
    literal = [{'name':f'literal-{i}','kind':'literal','clause':c['clause'],'source':c['source']}
               for i,c in enumerate(original['candidates'])]
    literal.append({'name':'literal-double-five','kind':'literal','clause':[104],'source':t.source([104])})
    final = literal+chosen
    # Persist the selection before audits; neither mechanics audit reads fresh labels.
    immutable_json(output/'selection.json',{'queries':final,'training_sha256':t.digest(training)})
    print(t.p.canonical({'generated':len(queries),'eligible':len(ranked),'chosen':[
        {'name':q['name'],'descriptor':q['descriptor'],'matches':q['training']['matches']} for q in chosen]}),flush=True)
    mechanical = audit(mechanics_rows(seeds),queries,output,binary,'library-audit')
    trained = audit(training,final,output,binary,'training-audit')
    value = {'schema':VERSION,'protocol_sha256':t.p.digest(Path(__file__).with_name('MINING-PROBE-V2.md')),
             'source_v1_id':original['id'],'training_sha256':t.digest(training),
             'seed_jobs':list(original['search']['seed_jobs']),
             'library':[dict(q,witness=witnesses[q['descriptor']]) for q in queries],
             'eligible':len(ranked),'queries':final,
             'primaries':['literal-double-five','relational-0'],
             'audit':{'library':mechanical,'training':trained}}
    value['id'] = t.digest(value)
    immutable_json(output/'fit.json',value)
    print(t.p.canonical({'fit_id':value['id'],'audit':value['audit']}),flush=True)
    return value


def fresh_rows(directory, output):
    db = t.history.open_snapshot(directory)
    try:
        cfg = t.p.manifest(db)
        if cfg['seed_start'] != 420625 or cfg['hands'] != 40 or cfg['screen']:
            raise ValueError('Fresh corpus differs from frozen protocol')
        records = db.execute('''SELECT j.id job_id,j.cell_id,j.trial,h.id hand_id,
            h.seed deal_seed,h.seat,h.tiles,c.decl,g.score,g.sha256,g.producer,g.payload
            FROM jobs j JOIN games g ON g.job_id=j.id JOIN cells c ON c.id=j.cell_id
            JOIN hands h ON h.id=c.hand_id WHERE j.trial<8 ORDER BY j.cell_id,j.trial''').fetchall()
    finally: db.close()
    if len(records) != 2880: raise ValueError('Fresh fixed-size campaign is incomplete')
    if {r['deal_seed'] for r in records} != set(range(420625,420635)):
        raise ValueError('Wrong source deals')
    groups = defaultdict(list)
    for r in records: groups[r['cell_id']].append(r['trial'])
    if len(groups) != 360 or any(sorted(v) != list(range(8)) for v in groups.values()):
        raise ValueError('Missing, duplicated, or screened cells')
    rows = []
    for record in records:
        raw = zlib.decompress(record['payload'])
        if hashlib.sha256(raw).hexdigest() != record['sha256']: raise ValueError('Receipt hash mismatch')
        receipt = json.loads(raw)
        t.p.validate_game(t.p.game_request(record),receipt)
        if record['score'] != receipt['score']: raise ValueError('Indexed score mismatch')
        rows.append({k:record[k] for k in ('job_id','cell_id','trial','hand_id','deal_seed','seat','decl','score','sha256','producer')})
        rows[-1].update(hands=receipt['hands'],policy_seed=receipt['seed'])
    immutable_json(output/'fresh-rows.json',rows)
    return rows


def evaluate(directory, output, binary):
    fitted = json.loads((output/'fit.json').read_text())
    identity = fitted['id']; content = dict(fitted); content.pop('id')
    if t.digest(content) != identity: raise ValueError('Frozen fit changed')
    if (output/'result.json').exists(): raise FileExistsError('Evaluation is already frozen')
    rows = fresh_rows(directory,output)
    queries = fitted['queries']; names = [q['name'] for q in queries]
    index = QueryIndex(rows,queries)
    check = audit(rows,queries,output,binary,'fresh-audit')
    permutation = t.permutations(index,names,count=1999)
    results = []
    for query,perm in zip(queries,permutation):
        report = index.report(query['name'])
        gate = query['name'] in fitted['primaries'] and report['excess_failure_rate'] is not None and \
               Fraction(*report['excess_failure_rate']) >= Fraction(3,100) and \
               report['matching_completions'] >= 20 and report['source_deals_matched'] >= 5 and \
               Fraction(*perm['one_sided_p']) <= Fraction(1,40)
        results.append({'name':query['name'],'primary':query['name'] in fitted['primaries'],
                        'report':report,'permutation':perm,'replication_gate_passed':bool(gate)})
    value = {'schema':VERSION,'fit_id':identity,'fresh_rows_sha256':t.digest(rows),
             'games':len(rows),'hidden_completions':len({(r['hand_id'],r['trial']) for r in rows}),
             'queries':results,'native_audit':check,
             'replication_gate_passed':any(r['replication_gate_passed'] for r in results),
             'interpretation':'Fresh fixed-size outcome association audit. No causal or player-strength inference.'}
    value['id'] = t.digest(value)
    immutable_json(output/'result.json',value)
    print(t.p.canonical(value),flush=True)
    return value


def publish(output, destination):
    """Keep every frozen query and its evidence, including failed replication."""
    fitted = json.loads((output/'fit.json').read_text())
    result = json.loads((output/'result.json').read_text())
    efficiency = json.loads((output/'cv-result.json').read_text())
    for value in (fitted,result,efficiency): verify_identity(value)
    if result['fit_id'] != fitted['id'] or efficiency['fresh_result_id'] != result['id']:
        raise ValueError('Publication inputs belong to different experiments')
    destination.mkdir(parents=True,exist_ok=True)
    outcomes = {q['name']:q for q in result['queries']}
    corrections = {q['name']:q for q in efficiency['queries']}
    entries = []
    for query in fitted['queries']:
        evidence = outcomes[query['name']]
        path = destination/(query['name']+'.scheme')
        if path.exists() and path.read_text() != query['source']:
            raise ValueError('Refusing to replace another query source')
        path.write_text(query['source'])
        entries.append({'name':query['name'],'source':path.name,
            'source_sha256':hashlib.sha256(query['source'].encode()).hexdigest(),
            'kind':query['kind'],'origin':query.get('witness'),
            'status':'replicated-outcome-association' if evidence['replication_gate_passed'] else 'unconfirmed-outcome-association',
            'scope':'opening bidder; fixed bid30 deployed player; fresh uniform hidden completions',
            'evidence':evidence,
            'measurement_efficiency':corrections.get(query['name'])})
    catalog = {'schema':'sunshine-scheme-mining-catalog-v1','fit_id':fitted['id'],
        'fresh_result_id':result['id'],'efficiency_result_id':efficiency['id'],
        'deployment':'analysis-only; hidden-world predicates must not become live policy guards',
        'replication_gate_passed':result['replication_gate_passed'],'queries':entries}
    catalog['id'] = t.digest(catalog)
    immutable_json(destination/'catalog.json',catalog)
    for filename,value in [('fit.json',fitted),('result.json',result),('cv-result.json',efficiency)]:
        immutable_json(destination/filename,value)
    print(t.p.canonical({'catalog':str(destination/'catalog.json'),'id':catalog['id'],
        'replicated':[q['name'] for q in entries if q['status']=='replicated-outcome-association']}))
    return catalog


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command',choices=['fit','evaluate','publish'])
    parser.add_argument('input',type=Path)
    parser.add_argument('--output',required=True,type=Path)
    parser.add_argument('--binary',type=Path,default=t.p.ROOT/'walt/target/release/scheme_worlds')
    args = parser.parse_args()
    if args.command == 'fit': fit(args.input,args.output,args.binary)
    elif args.command == 'evaluate': evaluate(args.input,args.output,args.binary)
    else: publish(args.input,args.output)
