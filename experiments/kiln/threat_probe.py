#!/usr/bin/env python3
"""Bounded outcome-first deletion generalization; Python >=3.10 for bit_count.

Frozen actual-play receipts -> train-only ownership clauses -> reserved audits.
No production writes, new Walt calls, or tactical predicate library.
"""
import argparse
from collections import defaultdict
from fractions import Fraction
import hashlib
from itertools import combinations
import json
from pathlib import Path
import random
import sqlite3
import subprocess
import time
import zlib

import history
import played as p
from rules import TILES

VERSION = 'kiln-outcome-scheme-deletion-v1'
ROLES = ('me', 'after', 'mate', 'before')


def digest(value):
    return hashlib.sha256(p.canonical(value).encode()).hexdigest()


def ratio(value):
    value = Fraction(value)
    return [value.numerator, value.denominator]


def hidden_facts(row):
    return tuple(sorted(28*relative+tile for relative in (1, 2, 3)
                        for tile in row['hands'][(row['seat']+relative) % 4]))


def source(clause):
    atoms = ['(viewer me)', '(successor me after)', '(partner me mate)',
             '(successor mate before)']
    for fact in clause:
        relative, tile = divmod(fact, 28)
        hi, lo = TILES[tile]
        atoms.append(f'(holds {ROLES[relative]} {hi}-{lo})')
    return '(fix\n  (roles ' + ' '.join(f'(chair {r})' for r in ROLES) + \
           ')\n  (out)\n  (case\n    ' + '\n    '.join(atoms) + '))\n'


def split(rows):
    seeds = sorted({r['deal_seed'] for r in rows})
    if len(seeds) != 25:
        raise ValueError('v1 requires the frozen 25-deal campaign')
    discovery = set(seeds[:15])
    result = {'discovery': [], 'same_hand': [], 'new_hand': []}
    for row in rows:
        name = ('discovery' if row['trial'] < 4 else 'same_hand') if row['deal_seed'] in discovery else 'new_hand'
        result[name].append(row)
    return result


def freeze(directory, output):
    path = output/'rows.json'
    if path.exists():
        rows = json.loads(path.read_text())
        meta = json.loads((output/'input.json').read_text())
        if digest(rows) != meta['rows_sha256']:
            raise ValueError('Frozen row identity changed')
        return rows, meta
    db = history.open_snapshot(directory)
    try:
        cfg = p.manifest(db)
        records = db.execute('''SELECT j.id job_id,j.cell_id,j.trial,h.id hand_id,
            h.seed deal_seed,h.seat,h.tiles,c.decl,g.score,g.sha256,g.producer,g.payload
            FROM jobs j JOIN games g ON g.job_id=j.id JOIN cells c ON c.id=j.cell_id
            JOIN hands h ON h.id=c.hand_id WHERE j.trial<8 ORDER BY j.cell_id,j.trial''').fetchall()
    finally:
        db.close()
    if len(records) != 7200 or any(sorted(r['trial'] for r in records if r['cell_id'] == cell) != list(range(8))
                                  for cell in {r['cell_id'] for r in records}):
        raise ValueError('Need all eight unscreened trials in every cell')
    output.mkdir(parents=True, exist_ok=True)
    archive = sqlite3.connect(output/'source-receipts.sqlite')
    archive.execute('CREATE TABLE IF NOT EXISTS receipts(job_id INTEGER PRIMARY KEY,sha256 TEXT,producer TEXT,payload BLOB)')
    rows = []
    with archive:
        for i, record in enumerate(records):
            raw = zlib.decompress(record['payload'])
            if hashlib.sha256(raw).hexdigest() != record['sha256']:
                raise ValueError('Original receipt hash mismatch')
            receipt = json.loads(raw)
            p.validate_game(p.game_request(record), receipt)
            if receipt['score'] != record['score']:
                raise ValueError('Receipt score differs from indexed score')
            rows.append({k: record[k] for k in ('job_id','cell_id','hand_id','deal_seed','seat','decl','trial','score','sha256','producer')})
            rows[-1].update(hands=receipt['hands'], policy_seed=receipt['seed'])
            archive.execute('INSERT OR REPLACE INTO receipts VALUES (?,?,?,?)',
                (record['job_id'], record['sha256'], record['producer'], record['payload']))
            if (i+1) % 1000 == 0:
                print(f'Validated and preserved {i+1}/7200 original games', flush=True)
    archive.close()
    producers = {}
    for producer in sorted({r['producer'] for r in rows}):
        producers[producer] = json.loads((Path(directory)/'producers'/producer/'producer.json').read_text())
    meta = {'schema': VERSION, 'campaign': cfg, 'rows_sha256': digest(rows), 'games': len(rows),
            'hidden_completions': len({(r['hand_id'],r['trial']) for r in rows}),
            'producers': producers, 'campaign_path': str(Path(directory).resolve())}
    p.atomic_json(output/'input.json', meta)
    p.atomic_json(path, rows)
    return rows, meta


class Index:
    """Bitsets cache only explicit literal ownership; runtime independently audits."""
    def __init__(self, rows):
        self.rows = rows
        self.all = (1 << len(rows))-1
        self.facts = defaultdict(int)
        self.losses = 0
        self.cells = defaultdict(list)
        self.deals = defaultdict(int)
        for i, row in enumerate(rows):
            bit = 1 << i
            for fact in hidden_facts(row): self.facts[fact] |= bit
            if row['score'] < 30: self.losses |= bit
            self.cells[row['cell_id']].append(i)
            self.deals[row['deal_seed']] |= bit
        sizes = {len(ids) for ids in self.cells.values()}
        if len(sizes) != 1: raise ValueError('Comparison requires equal trial counts per cell')
        self.n = next(iter(sizes))
        self.loss_count_masks = defaultdict(int)
        for ids in self.cells.values():
            mask = sum(1 << i for i in ids)
            self.loss_count_masks[(mask & self.losses).bit_count()] |= mask

    def membership(self, clause):
        mask = self.all
        for fact in clause: mask &= self.facts[fact]
        return mask

    def excess(self, mask, losses=None):
        losses = self.losses if losses is None else losses
        return self.n*(mask & losses).bit_count() - sum(k*(mask & bits).bit_count()
                                               for k,bits in self.loss_count_masks.items())

    def merit(self, mask, losses=None):
        return Fraction(max(0,self.excess(mask,losses))**2, max(1,mask.bit_count()))

    def report(self, clause):
        mask = self.membership(clause)
        n = mask.bit_count()
        fail = (mask & self.losses).bit_count()
        expected = Fraction(self.n*fail - self.excess(mask), self.n)
        other = self.all ^ mask
        return {'games':len(self.rows), 'matches':n, 'matching_completions':len({
                    (r['hand_id'],r['trial']) for i,r in enumerate(self.rows) if mask & (1 << i)}),
                'matched_failures':fail, 'matched_failure_rate':ratio(Fraction(fail,n)) if n else None,
                'matched_cell_baseline':ratio(expected/n) if n else None,
                'excess_failure_rate':ratio(Fraction(self.excess(mask),self.n*n)) if n else None,
                'nonmatching_games':other.bit_count(), 'nonmatching_failures':(other & self.losses).bit_count(),
                'source_deals_matched':sum(bool(mask & b) for b in self.deals.values()),
                'cell_excess_numerator':self.excess(mask), 'cell_excess_denominator':self.n,
                'matches_sha256':digest([r['job_id'] for i,r in enumerate(self.rows) if mask & (1 << i)])}


def fit(rows):
    """Only discovery rows enter candidate generation, ranking or selection."""
    index = Index(rows)
    losing = sorted((r for r in rows if r['score'] < 30),
                    key=lambda r: (digest(['seed-world-v1',r['job_id']]),r['job_id']))
    seeds, seen = [], set()
    for row in losing:
        facts = hidden_facts(row)
        if facts in seen: continue
        seen.add(facts); seeds.append(row)
        if len(seeds) == 64: break
    candidates = {}
    for row in seeds:
        for size in (1,2,3):
            for clause in combinations(hidden_facts(row),size):
                candidates.setdefault(clause,row['job_id'])
    ranked = []
    for clause,witness in candidates.items():
        mask = index.membership(clause)
        if mask.bit_count() < 36 or index.excess(mask) <= 0: continue
        if sum(bool(mask & b) for b in index.deals.values()) < 6: continue
        ranked.append((index.merit(mask),clause,witness,mask))
    ranked.sort(key=lambda c:(-c[0],len(c[1]),c[1]))
    selected = []
    masks = []
    for merit,clause,witness,mask in ranked:
        if any(5*(mask & old).bit_count() >= 4*(mask | old).bit_count() for old in masks): continue
        selected.append({'clause':list(clause),'seed_job_id':witness,'training_merit':ratio(merit)})
        masks.append(mask)
        if len(selected) == 8: break
    return {'schema':VERSION,'training_rows_sha256':digest(rows),'seed_jobs':[r['job_id'] for r in seeds],
            'candidates':len(candidates),'eligible':len(ranked),'selected':selected}


def permutations(index, clauses, count=999):
    """Permute trial outcomes jointly across declarations, within each hand."""
    masks = [index.membership(c) for c in clauses]
    observed = [index.merit(m) for m in masks]
    individual = [0]*len(masks)
    adjusted = [0]*len(masks)
    lookup = {(r['hand_id'],r['decl'],r['trial']):i for i,r in enumerate(index.rows)}
    hands = defaultdict(set)
    for r in index.rows: hands[r['hand_id']].add(r['trial'])
    rng = random.Random('kiln-threat-permutation-v1')
    for _ in range(count):
        maps = {}
        for hand in sorted(hands):
            old = sorted(hands[hand]); new = old.copy(); rng.shuffle(new)
            maps[hand] = dict(zip(old,new))
        losses = 0
        for i,row in enumerate(index.rows):
            other = lookup[(row['hand_id'],row['decl'],maps[row['hand_id']][row['trial']])]
            if index.rows[other]['score'] < 30: losses |= 1 << i
        stats = [index.merit(mask,losses) for mask in masks]
        maximum = max(stats,default=Fraction(0))
        for i,stat in enumerate(stats):
            individual[i] += stat >= observed[i]
            adjusted[i] += maximum >= observed[i]
    return [{'draws':count,'one_sided_p':ratio(Fraction(a+1,count+1)),
             'familywise_max_statistic_p':ratio(Fraction(b+1,count+1))}
            for a,b in zip(individual,adjusted)]


def native_audit(rows, clauses, output, binary):
    paths = []
    for i,clause in enumerate(clauses):
        path = output/f'audit-query-{i}.scheme'
        path.write_text(source(clause)); paths.append(str(path))
    lines = []
    for row in rows:
        bits = [sum(1 << t for t in h) for h in row['hands']]
        lines.append(' '.join(map(str,[row['job_id'],row['decl'],row['seat'],*bits])))
    native = subprocess.run([str(binary),*paths],input='\n'.join(lines)+'\n',text=True,
                            capture_output=True,check=True,timeout=120)
    answers = native.stdout.splitlines()
    if len(answers) != len(rows): raise ValueError('Native membership output omitted rows')
    for row,line in zip(rows,answers):
        identity,mask = map(int,line.split())
        facts = set(hidden_facts(row))
        expected = sum(1 << i for i,c in enumerate(clauses) if set(c) <= facts)
        if identity != row['job_id'] or mask != expected:
            raise ValueError(f'Actual Scheme runtime disagrees on job {row["job_id"]}')
    return {'world_rows':len(rows),'queries':len(clauses),'membership_checks':len(rows)*len(clauses),
            'mismatches':0,'native_output_sha256':hashlib.sha256(native.stdout.encode()).hexdigest(),
            'binary_sha256':hashlib.sha256(Path(binary).read_bytes()).hexdigest()}


def run(directory, output, binary):
    if (output/'result.json').exists():
        raise FileExistsError('Completed probe is immutable; use a new output directory')
    started = time.monotonic()
    rows,meta = freeze(directory,output)
    folds = split(rows)
    indexes = {name:Index(values) for name,values in folds.items()}
    learned = fit(folds['discovery'])
    # Freeze the selected hypotheses BEFORE accessing reserved outcome statistics.
    p.atomic_json(output/'fitted.json',learned)
    print(f'Froze {len(learned["selected"])} candidates from {learned["candidates"]} clauses',flush=True)
    if not learned['selected']: raise ValueError('No eligible candidate; frozen fit retained')
    clauses = [tuple(c['clause']) for c in learned['selected']]
    checks = {name:permutations(indexes[name],clauses) for name in ('same_hand','new_hand')}
    by_job = {r['job_id']:r for r in rows}
    results = []
    full = []
    for i,candidate in enumerate(learned['selected']):
        clause = tuple(candidate['clause'])
        original = hidden_facts(by_job[candidate['seed_job_id']]); full.append(original)
        (output/f'candidate-{i}.scheme').write_text(source(clause))
        (output/f'seed-{i}.scheme').write_text(source(original))
        results.append({**candidate,'source':source(clause),'retained_facts':len(clause),
            'removed_facts':sorted(set(original)-set(clause)),
            'seed_description':source(original), 'seed_receipt_sha256':by_job[candidate['seed_job_id']]['sha256'],
            'folds':{name:index.report(clause) for name,index in indexes.items()},
            'permutation':{name:checks[name][i] for name in checks},
            'immediate_ablations':[{'clause':list(c),'discovery':indexes['discovery'].report(c)}
                                   for c in combinations(clause,len(clause)-1)]})
    audit = native_audit(rows,clauses+full,output,binary)
    protocol = Path(__file__).with_name('THREAT-PROBE.md').read_bytes()
    result = {'schema':VERSION,'input_sha256':meta['rows_sha256'],
              'protocol_sha256':hashlib.sha256(protocol).hexdigest(),
              'search':{k:v for k,v in learned.items() if k != 'selected'},
              'split':{name:{'games':len(values),'deal_seeds':sorted({r['deal_seed'] for r in values}),
                    'trials':sorted({r['trial'] for r in values})} for name,values in folds.items()},
              'candidates':results,'native_membership_audit':audit,
              'interpretation':'Exploratory outcome associations; fixed literal-deletion grammar, not causal threats or player improvement.'}
    result['id'] = digest(result)
    p.atomic_json(output/'result.json',result)
    print(p.canonical({'output':str(output/'result.json'),'id':result['id'],
                      'elapsed_seconds':round(time.monotonic()-started,2),'audit':audit}),flush=True)
    return result


def ablate(output, binary):
    """Explicit post-hoc diagnosis; never refit or replace frozen candidates."""
    result = json.loads((output/'result.json').read_text())
    rows = json.loads((output/'rows.json').read_text())
    if digest(rows) != result['input_sha256']:
        raise ValueError('Frozen rows changed')
    parent = dict(result); saved_id = parent.pop('id')
    if digest(parent) != saved_id: raise ValueError('Frozen result changed')
    clauses = sorted({sub for c in result['candidates']
                      for sub in combinations(c['clause'],len(c['clause'])-1)})
    indexes = {name:Index(values) for name,values in split(rows).items()}
    report = {'schema':'kiln-threat-posthoc-ablations-v1','parent_id':result['id'],
        'interpretation':'Descriptive follow-up after seeing all reserved outcomes; no further selection or confirmation claim. Every one-literal deletion of every selected clause is included.',
        'ablations':[{'clause':list(c),'source':source(c),
                     'folds':{name:index.report(c) for name,index in indexes.items()}} for c in clauses]}
    target = output/'posthoc-ablations'; target.mkdir(exist_ok=True)
    report['native_audit'] = native_audit(rows,clauses,target,binary)
    report['id'] = digest(report)
    path = target/'result.json'
    if path.exists() and json.loads(path.read_text()) != report:
        raise ValueError('Refusing to replace a different completed ablation')
    p.atomic_json(path,report)
    print(p.canonical({'output':str(path),'id':report['id'],'audit':report['native_audit']}))
    return report


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory'); parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--binary',type=Path,default=p.ROOT/'walt/target/release/scheme_worlds')
    parser.add_argument('--ablate',action='store_true',help='Diagnose frozen candidates; no refitting')
    args = parser.parse_args()
    if args.ablate: ablate(args.output,args.binary)
    else: run(args.directory,args.output,args.binary)
