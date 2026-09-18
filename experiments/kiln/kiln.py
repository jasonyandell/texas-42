#!/usr/bin/env python3
"""Kiln: durable, bounded native production of own-information auction prices.
SQLite is the authority; status.json is an expendable view. No network service.
"""
import argparse
import asyncio
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import sqlite3
import shutil
import subprocess
import tarfile
import time

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_BINARY = ROOT / 'walt/target/release/kiln-worker'
DECLS = [0, 1, 2, 3, 4, 5, 6, 7, 9]
STAGES = [8, 40, 160]
PROFILE = 'kiln-l1-fixed-inner8-voidless-v1'
MASK = 0xffffffff
STEP = 0x6d2b79f5

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'))

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def preserve_producer(directory, binary):
    producer = digest(binary)
    folder = Path(directory)/'producers'/producer
    folder.mkdir(parents=True,exist_ok=True)
    target = folder/'kiln-worker'
    if not target.exists():
        shutil.copy2(binary,target)
        assert digest(target)==producer
        paths = sorted([*ROOT.glob('walt/walt/src/**/*.rs'), *ROOT.glob('walt/walt-player/src/**/*.rs'),
                        ROOT/'walt/Cargo.toml', ROOT/'walt/Cargo.lock',ROOT/'walt/walt/Cargo.toml',
                        ROOT/'walt/walt-player/Cargo.toml',Path(__file__)])
        with tarfile.open(folder/'source.tar.gz','w:gz') as archive:
            for path in paths: archive.add(path,arcname=str(path.relative_to(ROOT)),recursive=False)
        atomic_json(folder/'producer.json',{'schema':'kiln-producer-v1','binary_sha256':producer,
          'profile':PROFILE,'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
          'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
          'sources':{str(p.relative_to(ROOT)):digest(p) for p in paths}})
    if digest(target)!=producer: raise ValueError('Stored producer binary is corrupt')
    return target,producer

def mulberry(seed):
    a = seed
    while True:
        a = (a + STEP) & MASK
        t = ((a ^ (a >> 15)) * (1 | a)) & MASK
        t ^= (t + (((t ^ (t >> 7)) * (61 | t)) & MASK)) & MASK
        yield ((t ^ (t >> 14)) & MASK) / 4294967296

def deal(seed):
    """Exact Plunge newGame numeric-seed first hand, generator v1."""
    shaker = int(next(mulberry(seed)) * 4)
    rand = mulberry((seed + STEP) & MASK)
    # ALL_DOMINO_IDS is canonical triangular order; checked against Plunge.
    tiles = list(range(28))
    for i in range(27, 0, -1):
        j = int(next(rand) * (i + 1))
        tiles[i], tiles[j] = tiles[j], tiles[i]
    return shaker, [sorted(tiles[i:i+7]) for i in range(0, 28, 7)]

def sample_seed(hand, seat):
    # Never seeded by the real hidden opponents. Same own hand/profile repeats.
    return int.from_bytes(hashlib.sha256(f'{PROFILE}/{seat}/{canonical(hand)}'.encode()).digest()[:8], 'little')

def atomic_json(path, value):
    path = Path(path)
    tmp = path.with_suffix(path.suffix + '.tmp')
    with tmp.open('w') as f:
        f.write(canonical(value) + '\n'); f.flush(); os.fsync(f.fileno())
    os.replace(tmp, path)
    fd = os.open(path.parent, os.O_RDONLY)
    try: os.fsync(fd)
    finally: os.close(fd)

def connect(directory):
    db = sqlite3.connect(Path(directory) / 'kiln.sqlite', timeout=30)
    db.row_factory = sqlite3.Row
    db.executescript('PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; PRAGMA fullfsync=ON; PRAGMA checkpoint_fullfsync=ON; PRAGMA foreign_keys=ON;')
    return db

def init(directory, count, seed_start):
    directory = Path(directory); directory.mkdir(parents=True, exist_ok=True)
    if (directory / 'kiln.sqlite').exists(): raise ValueError('Database already exists; use run to resume.')
    if count < 1 or seed_start < 0 or seed_start + count > 2**32: raise ValueError('Invalid seed range.')
    db = connect(directory)
    db.executescript('''
    CREATE TABLE meta(key TEXT PRIMARY KEY,value TEXT NOT NULL);
    CREATE TABLE deals(id INTEGER PRIMARY KEY,seed INTEGER UNIQUE NOT NULL,shaker INTEGER NOT NULL,hands TEXT NOT NULL);
    CREATE TABLE jobs(id INTEGER PRIMARY KEY,deal_id INTEGER NOT NULL REFERENCES deals(id),seat INTEGER NOT NULL,decl INTEGER NOT NULL,bid INTEGER NOT NULL,stage INTEGER NOT NULL,
      state TEXT NOT NULL DEFAULT 'pending',attempts INTEGER NOT NULL DEFAULT 0,available REAL NOT NULL DEFAULT 0,error TEXT,
      UNIQUE(deal_id,seat,decl,bid,stage));
    CREATE INDEX ready ON jobs(state,stage,id);
    CREATE INDEX by_deal ON jobs(deal_id,state);
    CREATE TABLE results(job_id INTEGER PRIMARY KEY REFERENCES jobs(id),num INTEGER NOT NULL,den INTEGER NOT NULL,worlds INTEGER NOT NULL,
      disposition TEXT NOT NULL,producer TEXT NOT NULL,elapsed_us INTEGER NOT NULL,payload TEXT NOT NULL,created REAL NOT NULL);
    CREATE TABLE runs(id INTEGER PRIMARY KEY,started REAL NOT NULL,ended REAL,workers INTEGER NOT NULL,threads INTEGER NOT NULL,
      producer TEXT NOT NULL,completed INTEGER NOT NULL DEFAULT 0,errors INTEGER NOT NULL DEFAULT 0,status TEXT NOT NULL);
    ''')
    manifest = {'schema':'kiln-campaign-v1','profile':PROFILE,'count':count,'seed_start':seed_start,
                'generator':'plunge-mulberry32-newGame-numeric-v1','stages':STAGES,
                'screen':'8 worlds <= 1/8; 40 worlds < 1/2; deterministic 2-percent cell audit advances to 160',
                'sunshine':'A fairly quick, lawful, partner-aware player. Model scores are not calibrated win probabilities. Preserve original receipts, saved questions and the Scheme gym.'}
    with db:
        db.execute('INSERT INTO meta VALUES (?,?)',('manifest',canonical(manifest)))
        for i, seed in enumerate(range(seed_start, seed_start+count)):
            shaker,hands = deal(seed)
            db.execute('INSERT INTO deals VALUES (?,?,?,?)',(i,seed,shaker,canonical(hands)))
            db.executemany('INSERT INTO jobs(deal_id,seat,decl,bid,stage) VALUES (?,?,?,?,0)',
                           ((i,s,d,b) for s in range(4) for d in DECLS for b in range(30,43)))
    atomic_json(directory/'manifest.json',manifest)
    db.close()

def audited(job):
    cell = [job[k] for k in ('deal_id','seat','decl','bid')]
    return int(hashlib.sha256(('kiln-audit-v1/'+canonical(cell)).encode()).hexdigest()[:8],16) % 50 == 0

def disposition(job,num,den):
    stage = job['stage']
    if stage == 2: return 'target-depth'
    weak = num * 8 <= den if stage == 0 else num * 2 < den
    if weak and not audited(job): return 'screened-heuristic'
    return 'audit-advance' if weak else 'advance'

def validate(job,request,result):
    if result.get('schema') != 'kiln-price-v1' or result.get('auction') != request['auction']:
        raise ValueError('Worker identity mismatch')
    if result.get('worlds') != request['worlds'] or result.get('inner_worlds') != 8:
        raise ValueError('Worker profile mismatch')
    d,n,q = result['price']
    n,q = int(n),int(q)
    if d != job['decl'] or not 0 <= n <= q or q <= 0 or request['worlds'] % q:
        raise ValueError('Invalid finite-sample price')
    if not isinstance(result.get('work',{}).get('elapsed_us'),int): raise ValueError('Missing work record')
    return n,q

def commit_result(db,job,request,result,producer,run_id=None):
    n,d = validate(job,request,result)
    reason = disposition(job,n,d)
    with db:
        db.execute('INSERT INTO results VALUES (?,?,?,?,?,?,?,?,?)',
                   (job['id'],n,d,request['worlds'],reason,producer,result['work']['elapsed_us'],canonical(result),time.time()))
        db.execute("UPDATE jobs SET state='done',error=NULL WHERE id=?",(job['id'],))
        if reason in ('advance','audit-advance'):
            db.execute('INSERT INTO jobs(deal_id,seat,decl,bid,stage) VALUES (?,?,?,?,?)',
                       (job['deal_id'],job['seat'],job['decl'],job['bid'],job['stage']+1))
        if run_id is not None:
            db.execute('UPDATE runs SET completed=completed+1 WHERE id=?',(run_id,))

def status(db):
    manifest = json.loads(db.execute("SELECT value FROM meta WHERE key='manifest'").fetchone()[0])
    states = [dict(r) for r in db.execute('SELECT stage,state,count(*) AS n FROM jobs GROUP BY stage,state')]
    depth = [dict(r) for r in db.execute('SELECT worlds,disposition,count(*) AS n FROM results GROUP BY worlds,disposition')]
    complete = db.execute("SELECT count(*) FROM (SELECT deal_id FROM jobs WHERE stage=0 AND state='done' GROUP BY deal_id HAVING count(*)=468)").fetchone()[0]
    settled = db.execute("SELECT count(*) FROM deals d WHERE NOT EXISTS (SELECT 1 FROM jobs j WHERE j.deal_id=d.id AND j.state!='done')").fetchone()[0]
    runs = [dict(r) for r in db.execute('SELECT * FROM runs ORDER BY id DESC LIMIT 8')]
    for r in runs:
        elapsed = (r['ended'] or time.time())-r['started']
        r['completed_per_second'] = r['completed']/max(elapsed,.001)
    return {'schema':'kiln-status-v1','updated':time.time(),'deals':manifest['count'],
            'covered_deals':complete,'settled_deals':settled,'jobs':states,'depth':depth,'runs':runs}

async def run(args):
    directory = Path(args.directory)
    lock = (directory/'coordinator.lock').open('w')
    try: fcntl.flock(lock, fcntl.LOCK_EX|fcntl.LOCK_NB)
    except BlockingIOError: raise ValueError('Another Kiln coordinator owns this database.')
    lock.write(str(os.getpid()));lock.flush()
    binary,producer = preserve_producer(directory,Path(args.binary).resolve())
    db = connect(directory)
    if getattr(args,'order','coverage')=='deal':
        db.execute('CREATE INDEX IF NOT EXISTS ready_deal ON jobs(state,deal_id,stage DESC,id)')
    with db:
        # Exclusive OS lock proves any old leases have no active coordinator.
        db.execute("UPDATE jobs SET state='pending' WHERE state='running'")
        db.execute("UPDATE runs SET ended=?,status='recovered' WHERE status='running'",(time.time(),))
        run_id = db.execute('INSERT INTO runs(started,workers,threads,producer,status) VALUES (?,?,?,?,?)',
                            (time.time(),args.workers,args.threads,producer,'running')).lastrowid
    stopped = asyncio.Event(); loop = asyncio.get_running_loop()
    for sig in (signal.SIGINT,signal.SIGTERM): loop.add_signal_handler(sig,stopped.set)
    end = time.monotonic()+args.seconds if args.seconds else float('inf')
    completed = errors = 0
    order = {'coverage':'stage,id','depth':'stage DESC,id','deal':'deal_id,stage DESC,id'}[getattr(args,'order','coverage')]
    async def worker(slot):
        nonlocal completed,errors
        process = None
        try:
            while not stopped.is_set() and time.monotonic()<end:
                job = db.execute(f"SELECT * FROM jobs WHERE state='pending' AND available<=? ORDER BY {order} LIMIT 1",(time.time(),)).fetchone()
                if job is None:
                    if db.execute("SELECT 1 FROM jobs WHERE state='pending' LIMIT 1").fetchone():
                        await asyncio.sleep(.5);continue
                    return
                job = dict(job)
                # Leases need no standalone fsync: the single OS-locked writer
                # sees them immediately; crash recovery reclaims them anyway.
                # Commit them with the next durable result instead.
                db.execute("UPDATE jobs SET state='running',attempts=attempts+1 WHERE id=?",(job['id'],))
                hands = json.loads(db.execute('SELECT hands FROM deals WHERE id=?',(job['deal_id'],)).fetchone()[0])
                hand = hands[job['seat']]
                request = {'auction':{'hand':hand,'seat':job['seat'],'bid':job['bid'],'seed':sample_seed(hand,job['seat'])},
                           'decl':job['decl'],'worlds':STAGES[job['stage']],'budget_ms':args.job_ms}
                try:
                    if process is None or process.returncode is not None:
                        process = await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,
                            stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.DEVNULL,
                            env={**os.environ,'RAYON_NUM_THREADS':str(args.threads)})
                    process.stdin.write((canonical(request)+'\n').encode());await process.stdin.drain()
                    line = await asyncio.wait_for(process.stdout.readline(),args.job_ms/1000+2)
                    result = json.loads(line)
                    if 'error' in result: raise ValueError(result['error'])
                    commit_result(db,job,request,result,producer,run_id)
                    completed += 1
                except asyncio.CancelledError: raise
                except Exception as e:
                    errors += 1
                    attempts = job['attempts']+1
                    with db:
                        db.execute("UPDATE jobs SET state=?,error=?,available=? WHERE id=?",
                            ('failed' if attempts>=5 else 'pending',str(e)[:500],time.time()+min(60,2**attempts),job['id']))
                        db.execute('UPDATE runs SET errors=errors+1 WHERE id=?',(run_id,))
                    if process is not None and process.returncode is None:
                        process.kill();await process.wait()
                    process = None
        finally:
            if process is not None and process.returncode is None:
                process.kill();await process.wait()
    tasks = [asyncio.create_task(worker(i)) for i in range(args.workers)]
    try:
        last = 0
        while not all(t.done() for t in tasks) and not stopped.is_set() and time.monotonic()<end:
            if time.monotonic()-last>=10:
                view = status(db);atomic_json(directory/'status.json',view)
                print(canonical({'completed':completed,'errors':errors,'covered_deals':view['covered_deals'],
                                 'rate':view['runs'][0]['completed_per_second'],'pid':os.getpid()}),flush=True)
                last=time.monotonic()
            try: await asyncio.wait_for(stopped.wait(),timeout=.25)
            except asyncio.TimeoutError: pass
    finally:
        for task in tasks: task.cancel()
        await asyncio.gather(*tasks,return_exceptions=True)
        with db:
            db.execute("UPDATE jobs SET state='pending' WHERE state='running'")
            db.execute("UPDATE runs SET ended=?,completed=?,errors=?,status='stopped' WHERE id=?",(time.time(),completed,errors,run_id))
        view=status(db);atomic_json(directory/'status.json',view)
        print(canonical(view),flush=True)
        db.execute('PRAGMA wal_checkpoint(FULL)');db.close();lock.close()

def export(directory,output):
    db=connect(directory); manifest=json.loads(db.execute("SELECT value FROM meta WHERE key='manifest'").fetchone()[0])
    deals=[]
    for deal_row in db.execute('SELECT * FROM deals ORDER BY id'):
        rows=db.execute('''SELECT j.seat,j.decl,j.bid,r.num,r.den,r.worlds,r.disposition FROM jobs j JOIN results r ON j.id=r.job_id
            WHERE j.deal_id=? AND j.stage=(SELECT max(k.stage) FROM jobs k JOIN results q ON k.id=q.job_id
              WHERE k.deal_id=j.deal_id AND k.seat=j.seat AND k.decl=j.decl AND k.bid=j.bid)
            ORDER BY j.seat,j.decl,j.bid''',(deal_row['id'],)).fetchall()
        if len(rows)!=468: continue
        deals.append({'seed':deal_row['seed'],'shaker':deal_row['shaker'],'hands':json.loads(deal_row['hands']),
                      'sample_seeds':[str(sample_seed(h,s)) for s,h in enumerate(json.loads(deal_row['hands']))],
                      'prices':[[r[k] for k in ('seat','decl','bid','num','den','worlds','disposition')] for r in rows]})
    producers=[r[0] for r in db.execute('SELECT DISTINCT producer FROM results ORDER BY producer')]
    artifact={'schema':'kiln-book-v1','campaign':manifest,'producers':producers,'deals':deals}
    artifact['id']=hashlib.sha256(canonical(artifact).encode()).hexdigest()
    atomic_json(output,artifact)
    print(canonical({'deals':len(deals),'id':artifact['id'],'bytes':Path(output).stat().st_size}))

def main():
    p=argparse.ArgumentParser(description=__doc__);sub=p.add_subparsers(dest='command',required=True)
    i=sub.add_parser('init');i.add_argument('directory');i.add_argument('--count',type=int,default=1000);i.add_argument('--seed-start',type=int,default=420600)
    r=sub.add_parser('run');r.add_argument('directory');r.add_argument('--seconds',type=float,default=60);r.add_argument('--workers',type=int,default=12);r.add_argument('--threads',type=int,default=1);r.add_argument('--job-ms',type=int,default=30000);r.add_argument('--binary',default=str(DEFAULT_BINARY));r.add_argument('--order',choices=['coverage','depth','deal'],default='coverage',help='Coverage first, deep refinement first, or finish each deal')
    s=sub.add_parser('status');s.add_argument('directory')
    e=sub.add_parser('export');e.add_argument('directory');e.add_argument('output')
    a=p.parse_args()
    if a.command=='init':init(a.directory,a.count,a.seed_start)
    elif a.command=='run':
        if a.workers<1 or a.threads<1 or a.seconds<0 or not 5<=a.job_ms<=600000:p.error('Invalid run bounds')
        asyncio.run(run(a))
    elif a.command=='status':print(json.dumps(status(connect(a.directory)),indent=2))
    elif a.command=='export':export(a.directory,a.output)
if __name__=='__main__':main()
