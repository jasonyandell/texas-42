#!/usr/bin/env python3
"""Read-only snapshot audit of all Kiln receipts, refinement chains and producers.
Partial audits diagnose running campaigns; only a complete audit is a release gate.
"""
import argparse,hashlib,json,math,re,sqlite3,tarfile,time
from collections import Counter
from pathlib import Path
from kiln import atomic_json,canonical,deal

PROFILE='kiln-l1-fixed-inner8-voidless-v1'
DECLS=[0,1,2,3,4,5,6,7,9]
STAGES=[8,40,160]

def require(test,message):
    if not test:raise ValueError(message)
def sha(data):return hashlib.sha256(data).hexdigest()
def number(x):return type(x) is int

def price_seed(hand,seat):
    return int.from_bytes(hashlib.sha256(f'{PROFILE}/{seat}/{canonical(hand)}'.encode()).digest()[:8],'little')
def audit_cell(cell):return int(sha(('kiln-audit-v1/'+canonical(list(cell))).encode())[:8],16)%50==0

def stop_reason(cell,stage,n,d):
    if stage==2:return 'target-depth'
    weak=n*8<=d if stage==0 else n*2<d
    if weak:return 'audit-advance' if audit_cell(cell) else 'screened-heuristic'
    return 'advance'

def wilson(wins,n):
    if not n:return [None,None]
    z=1.96;p=wins/n;den=1+z*z/n
    mid=(p+z*z/(2*n))/den;radius=z*math.sqrt(p*(1-p)/n+z*z/(4*n*n))/den
    return [mid-radius,mid+radius]

def audit(directory,book_path=None,partial=False,required_deals=1000):
    started=time.monotonic();directory=Path(directory)
    db=sqlite3.connect(f'file:{directory / "kiln.sqlite"}?mode=ro',uri=True)
    db.row_factory=sqlite3.Row
    db.execute('PRAGMA query_only=ON');db.execute('BEGIN')
    try:
        require(db.execute('PRAGMA integrity_check').fetchone()[0]=='ok','SQLite integrity failure')
        require(not db.execute('PRAGMA foreign_key_check').fetchall(),'Broken database references')
        manifest=json.loads(db.execute("SELECT value FROM meta WHERE key='manifest'").fetchone()[0])
        require(manifest['schema']=='kiln-campaign-v1' and manifest['profile']==PROFILE
            and manifest['generator']=='plunge-mulberry32-newGame-numeric-v1' and manifest['stages']==STAGES,'Unsupported campaign')
        require(manifest['count']==required_deals,f'Expected {required_deals} preselected deals')
        deals=db.execute('SELECT * FROM deals ORDER BY id').fetchall()
        require(len(deals)==required_deals,'Deal count differs from campaign')
        require(db.execute("SELECT count(*) FROM results r LEFT JOIN jobs j ON j.id=r.job_id WHERE j.id IS NULL").fetchone()[0]==0,'Orphan receipt')
        producers=set();states=Counter();depth=Counter();dispositions=Counter()
        covered=settled=receipts=audit_total=audit_finished=0
        screening={str(n):{'would_screen':0,'deep_eligible':0,'deep_at_least_half':0} for n in (8,40)}
        screening['either']={'would_screen':0,'deep_eligible':0,'deep_at_least_half':0}
        book=None;book_deals={}
        if book_path:
            book=json.loads(Path(book_path).read_text());identity=book.pop('id',None)
            require(identity==sha(canonical(book).encode()),'Book content identity mismatch');book['id']=identity
            require(book['schema']=='kiln-book-v1' and book['campaign']==manifest,'Book campaign mismatch')
            for d in book['deals']:
                require(d['seed'] not in book_deals,'Duplicate book seed');book_deals[d['seed']]=d
            require(set(book_deals)<=set(d['seed'] for d in deals),'Unknown book deal')
            if not partial:require(len(book_deals)==required_deals,'Shipping book is incomplete')
        for di,d in enumerate(deals):
            require(d['id']==di and d['seed']==manifest['seed_start']+di,f'Deal {di}: preselected seed changed')
            shaker,hands=deal(d['seed'])
            require(d['shaker']==shaker and json.loads(d['hands'])==hands,f'Deal {di}: shuffle mismatch')
            rows=db.execute('''SELECT j.*,r.job_id AS receipt_id,r.num,r.den,r.worlds,r.disposition,r.producer,
                r.elapsed_us,r.payload FROM jobs j LEFT JOIN results r ON r.job_id=j.id
                WHERE j.deal_id=? ORDER BY j.seat,j.decl,j.bid,j.stage''',(di,)).fetchall()
            cells={};base_done=0;all_done=True;history={};latest={}
            for row in rows:
                r=dict(row);cell=(di,r['seat'],r['decl'],r['bid']);stage=r['stage']
                require(r['seat'] in range(4) and r['decl'] in DECLS and r['bid'] in range(30,43)
                    and stage in range(3),f'Job {r["id"]}: invalid coordinates')
                chain=cells.setdefault(cell,{})
                require(stage not in chain,f'{cell}: duplicate stage');chain[stage]=r
                require(r['state'] in ('pending','running','failed','done'),f'{cell}: unknown queue state')
                states[r['state']]+=1
                require((r['state']=='done')==(r['receipt_id'] is not None),f'{cell}: state and receipt disagree')
                if r['state']!='done':all_done=False;continue
                receipts+=1;depth[r['worlds']]+=1;dispositions[r['disposition']]+=1
                if stage==0:base_done+=1
                n,q=r['num'],r['den'];worlds=STAGES[stage]
                require(number(n) and number(q) and 0<=n<=q and q>0 and worlds%q==0
                    and math.gcd(n,q)==1 and r['worlds']==worlds,f'{cell}: invalid rational/sample depth')
                hand=hands[r['seat']]
                expected={'hand':hand,'seat':r['seat'],'bid':r['bid'],'seed':price_seed(hand,r['seat'])}
                v=json.loads(r['payload'])
                require(v.get('schema')=='kiln-price-v1' and v.get('auction')==expected
                    and v.get('worlds')==worlds and v.get('inner_worlds')==8
                    and v.get('price')==[r['decl'],str(n),str(q)],f'{cell}: receipt identity or value mismatch')
                work=v.get('work',{})
                require(work.get('elapsed_us')==r['elapsed_us'] and all(number(work.get(k)) and work[k]>=0
                    for k in ('elapsed_us','nodes','policy_cache_entries','search_cache_entries')),
                    f'{cell}: missing/invalid work counters')
                require(all(isinstance(work.get(k),list) and len(work[k])==2
                    and all(number(x) and x>=0 for x in work[k]) for k in ('pi_calls','inner_worlds')),
                    f'{cell}: invalid modeled-level counters')
                require(work['inner_worlds'][0]==8*work['pi_calls'][0] and work['inner_worlds'][1]==0 and work['pi_calls'][1]==0,
                    f'{cell}: counters disagree with L1/inner8 profile')
                reason=stop_reason(cell,stage,n,q)
                require(r['disposition']==reason,f'{cell}: wrong refinement disposition')
                require(re.fullmatch('[a-f0-9]{64}',r['producer']) is not None,f'{cell}: invalid producer')
                producers.add(r['producer'])
                value=[r[k] for k in ('seat','decl','bid','num','den','worlds','disposition')]
                history[tuple(value)]=True;latest[cell[1:]]=value
            require(len(cells)==468 and all(0 in chain for chain in cells.values()),f'Deal {di}: base panel incomplete')
            for cell,chain in cells.items():
                for stage,r in chain.items():
                    if stage:
                        previous=chain.get(stage-1)
                        require(previous and previous['state']=='done' and previous['disposition'] in ('advance','audit-advance'),
                            f'{cell}: unearned refinement stage {stage}')
                    if r['state']=='done':
                        should_advance=r['disposition'] in ('advance','audit-advance')
                        require((stage+1 in chain)==should_advance,f'{cell}: missing or unexpected refinement job')
                if audit_cell(cell):
                    audit_total+=1
                    final=chain.get(2)
                    if final and final['state']=='done':
                        audit_finished+=1;weak=[]
                        for stage in (0,1):
                            r=chain[stage];screened=r['num']*8<=r['den'] if stage==0 else r['num']*2<r['den']
                            if screened:weak.append(str(STAGES[stage]))
                        if weak:weak.append('either')
                        for label in weak:
                            s=screening[label];s['would_screen']+=1
                            s['deep_eligible']+=final['num']*4>=final['den']*3
                            s['deep_at_least_half']+=final['num']*2>=final['den']
            covered+=base_done==468;settled+=all_done
            if d['seed'] in book_deals:
                b=book_deals[d['seed']]
                require(b['hands']==hands and b['shaker']==shaker
                    and b['sample_seeds']==[str(price_seed(h,s)) for s,h in enumerate(hands)],f'Deal {di}: book hand/seed mismatch')
                require(len(b['prices'])==468,f'Deal {di}: incomplete book panel')
                seen=set()
                for value in b['prices']:
                    cell=tuple(value[:3]);require(cell not in seen,f'Deal {di}: duplicate book cell');seen.add(cell)
                    require(tuple(value) in history,f'Deal {di}: book price has no matching original receipt')
                    if not partial:require(latest[cell]==value and value[-1] in ('screened-heuristic','target-depth'),f'Deal {di}: unfinished book refinement')
        provenance=[]
        for producer in sorted(producers):
            folder=directory/'producers'/producer
            require(sha((folder/'kiln-worker').read_bytes())==producer,'Producer binary identity mismatch')
            p=json.loads((folder/'producer.json').read_text())
            require(p['schema']=='kiln-producer-v1' and p['profile']==PROFILE and p['binary_sha256']==producer,'Producer manifest mismatch')
            with tarfile.open(folder/'source.tar.gz','r:gz') as tar:
                for path,identity in p['sources'].items():
                    stream=tar.extractfile(path)
                    require(stream is not None and sha(stream.read())==identity,f'Producer {producer}: source snapshot mismatch {path}')
            provenance.append({'binary':producer,'source_commit':p['source_commit'],'verified_source_files':len(p['sources'])})
        if book:
            require(set(book['producers'])<=producers and len(set(book['producers']))==len(book['producers']),'Book producer list mismatch')
            if not partial:require(set(book['producers'])==producers,'Book is missing producer provenance')
        complete=settled==required_deals and not any(states[s] for s in ('failed','pending','running')) and audit_finished==audit_total
        if not partial:require(complete,'Campaign still has incomplete/failed work')
        for s in screening.values():s['deep_eligible_wilson95']=wilson(s['deep_eligible'],s['would_screen'])
        return {'schema':'kiln-campaign-audit-v1','verified_structure':True,'complete':complete,
            'required_deals':required_deals,'covered_deals':covered,'settled_deals':settled,'receipts':receipts,
            'states':dict(states),'depth':dict(depth),'dispositions':dict(dispositions),'profile':PROFILE,
            'audit_cells':{'preselected':audit_total,'at_target_depth':audit_finished,'screening':screening},
            'producers':provenance,'book_id':book['id'] if book else None,'elapsed_seconds':time.monotonic()-started,
            'note':'Sampling depth and model scores are verified; calibration and actual player strength are separate evidence.'}
    finally:db.close()

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('directory');p.add_argument('output');p.add_argument('--book');p.add_argument('--partial',action='store_true');p.add_argument('--required-deals',type=int,default=1000)
    a=p.parse_args();result=audit(a.directory,a.book,a.partial,a.required_deals);atomic_json(a.output,result)
    print(canonical({k:v for k,v in result.items() if k not in ('producers','audit_cells')}))
