#!/usr/bin/env python3
"""Durable empirical bid data: actual deployed players, bid30, seven tricks.

No model price is an observation. SQLite owns the queue and completed outcomes;
the native host owns a game; each player owns only its hand and public history.
"""
import argparse
import asyncio
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import random
import shutil
import signal
import sqlite3
import subprocess
import sys
import tarfile
import tempfile
import time
import zlib

from kiln import ROOT, DECLS, atomic_json, canonical, deal, digest
sys.path.insert(0, str(ROOT / 'experiments/partnership'))
from rules import legal_tiles, replay_record

PROFILE = 'walt-table-v2-opening160-ordinary40-partner-bid30-v1'
BINARY = ROOT / 'walt/target/release/kiln-play-worker'


def seeded(label):
    return int.from_bytes(hashlib.sha256(label.encode()).digest()[:8], 'little')


def connect(directory):
    db = sqlite3.connect(Path(directory) / 'played.sqlite', timeout=30)
    db.row_factory = sqlite3.Row
    db.executescript('''PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;
        PRAGMA fullfsync=ON; PRAGMA checkpoint_fullfsync=ON; PRAGMA foreign_keys=ON;''')
    return db


def manifest(db):
    return json.loads(db.execute("SELECT value FROM meta WHERE key='manifest'").fetchone()[0])


@contextlib.contextmanager
def writer_lock(directory):
    with (Path(directory) / 'writer.lock').open('a') as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            raise ValueError('Another coordinator owns this campaign') from error
        yield


def init(directory, hands=100, seed_start=420600, screen=True):
    directory = Path(directory)
    directory.mkdir(parents=True, exist_ok=True)
    if hands < 1 or seed_start < 0 or seed_start + (hands + 3)//4 > 2**32:
        raise ValueError('Invalid hands or seed range')
    with writer_lock(directory):
        if (directory / 'played.sqlite').exists():
            raise ValueError('Campaign already exists; run resumes it')
        db = connect(directory)
        db.executescript('''
        CREATE TABLE meta(key TEXT PRIMARY KEY,value TEXT NOT NULL);
        CREATE TABLE hands(id INTEGER PRIMARY KEY,seed INTEGER NOT NULL,seat INTEGER NOT NULL,tiles TEXT NOT NULL);
        CREATE TABLE cells(id INTEGER PRIMARY KEY,hand_id INTEGER REFERENCES hands(id),decl INTEGER NOT NULL,audited INTEGER NOT NULL,
            UNIQUE(hand_id,decl));
        CREATE TABLE jobs(id INTEGER PRIMARY KEY,cell_id INTEGER REFERENCES cells(id),trial INTEGER NOT NULL,
            state TEXT NOT NULL DEFAULT 'pending',attempts INTEGER NOT NULL DEFAULT 0,error TEXT,
            UNIQUE(cell_id,trial));
        CREATE INDEX ready ON jobs(state,trial,id);
        CREATE TABLE games(job_id INTEGER PRIMARY KEY REFERENCES jobs(id),score INTEGER NOT NULL CHECK(score BETWEEN 0 AND 42),
            producer TEXT NOT NULL,elapsed_us INTEGER NOT NULL,policy_us INTEGER NOT NULL,opening_us INTEGER NOT NULL,
            changed INTEGER NOT NULL,over_budget INTEGER NOT NULL,fallbacks INTEGER NOT NULL,
            sha256 TEXT NOT NULL,payload BLOB NOT NULL,created REAL NOT NULL);
        CREATE TABLE runs(id INTEGER PRIMARY KEY,started REAL NOT NULL,ended REAL,workers INTEGER NOT NULL,
            target INTEGER NOT NULL,producer TEXT NOT NULL,completed INTEGER NOT NULL DEFAULT 0,
            errors INTEGER NOT NULL DEFAULT 0,status TEXT NOT NULL);
        ''')
        config = {'schema':'kiln-played-campaign-v1','profile':PROFILE,'hands':hands,
            'seed_start':seed_start,'declarations':DECLS,'play_bid':30,'recommendation':[4,5],
            'generator':'plunge-mulberry32-newGame-numeric-v1',
            'completions':'python-random-shuffle-sha256-own-hand-v1','screen':screen,
            'screen_rule':'n=8 makes30<=1; n=40 makes30<20; preselected 2-percent bypass',
            'source_survey':'kiln-v1 remains separate; no survey scores count as games'}
        with db:
            db.execute('INSERT INTO meta VALUES (?,?)', ('manifest',canonical(config)))
            for hand_id in range(hands):
                seed, seat = seed_start + hand_id//4, hand_id % 4
                _, original = deal(seed)
                hand = original[seat]
                db.execute('INSERT INTO hands VALUES (?,?,?,?)',(hand_id,seed,seat,canonical(hand)))
                for decl in DECLS:
                    audited = seeded(f'kiln-played-audit-v1/{seat}/{canonical(hand)}/{decl}') % 50 == 0
                    cell = db.execute('INSERT INTO cells(hand_id,decl,audited) VALUES (?,?,?)',
                                      (hand_id,decl,int(audited))).lastrowid
                    db.execute('INSERT INTO jobs(cell_id,trial) VALUES (?,0)',(cell,))
        atomic_json(directory/'manifest.json', config)
        db.close()


def game_request(job):
    """The completion never depends on original hidden hands or declaration."""
    hand = json.loads(job['tiles'])
    seat, trial = job['seat'], job['trial']
    identity = f'{seat}/{canonical(hand)}/{trial}'
    unseen = sorted(set(range(28)) - set(hand))
    random.Random(seeded('kiln-played-hidden-v1/' + identity)).shuffle(unseen)
    hands = [None] * 4
    hands[seat] = hand
    for i, other in enumerate(s for s in range(4) if s != seat):
        hands[other] = sorted(unseen[7*i:7*i+7])
    return {'profile':PROFILE,'hands':hands,'bidder':seat,'decl':job['decl'],
            'seed':seeded('kiln-played-policy-v1/' + identity) & 0xffffffff}


def validate_game(request, value):
    """Independent rules, complete history, and exact actor-only call boundary."""
    def require(test, message):
        if not test:
            raise ValueError(message)
    require(value.get('schema') == 'kiln-played-game-v1', 'Wrong game schema')
    require(all(value.get(k) == v for k,v in request.items()), 'Game identity changed')
    require(value.get('play_bid') == 30, 'Wrong play target')
    decisions = value.get('decisions', [])
    require(len(decisions) == 28, 'Game must finish all seven tricks')
    hands, bidder, decl = request['hands'], request['bidder'], request['decl']
    record = []
    policy_us = opening_us = changed = over_budget = fallbacks = 0
    for ply, decision in enumerate(decisions):
        points, leader, remaining, trick = replay_record(hands,record,decl,bidder)
        actor = (leader + len(trick)) % 4
        expected = {'request':{'decl':decl,'bid':30,'bidder':bidder,'seat':actor,
            'hand':hands[actor],'plays':record.copy(),'seed':request['seed']},
            'worlds':160 if ply == 0 else 40,'partner':ply != 0,
            'budget_ms':20000 if ply == 0 else 14000}
        require(decision.get('call') == expected, 'Player received a different information set or profile')
        response = decision['response']
        legal = legal_tiles(remaining[actor],trick,decl)
        require(response.get('player_version') == 'walt-table-v2', 'Wrong player')
        require(response.get('choice') in legal, 'Illegal play')
        require(response.get('legal') == legal and response.get('points') == points
                and response.get('leader') == leader, 'Player and independent rules disagree')
        require(response.get('trick') == ply//4 + 1, 'Wrong trick')
        elapsed = response.get('elapsed_us')
        require(type(elapsed) is int and elapsed >= 0, 'Invalid decision time')
        policy_us += elapsed
        if ply == 0:
            opening_us = elapsed
        changed += (response.get('review_result') or {}).get('status') == 'changed'
        over_budget += bool(response.get('over_budget'))
        fallbacks += response.get('route') not in ('forced','baseline','baseline-reviewed')
        record.extend((actor,response['choice']))
    points, _, remaining, trick = replay_record(hands,record,decl,bidder)
    require(not any(remaining) and not trick and sum(points) == 42, 'Unfinished or mis-scored game')
    require(value.get('record') == record and value.get('points') == points
            and value.get('score') == points[bidder%2], 'Wrong final score/history')
    require(type(value.get('elapsed_us')) is int and value['elapsed_us'] >= policy_us,
            'Invalid game timing')
    return policy_us,opening_us,changed,over_budget,fallbacks


def blocked(n, makes, audited, screen):
    return screen and not audited and ((n == 8 and makes <= 1) or (n == 40 and makes < 20))


def schedule(db, target, screen, cell_id=None):
    query = '''SELECT c.id,c.audited,COUNT(g.job_id) AS n,
        COALESCE(SUM(g.score>=30),0) AS makes FROM cells c
        LEFT JOIN jobs j ON j.cell_id=c.id LEFT JOIN games g ON g.job_id=j.id'''
    params = ()
    if cell_id is not None:
        query += ' WHERE c.id=?'
        params = (cell_id,)
    for cell in db.execute(query+' GROUP BY c.id',params).fetchall():
        if cell['n'] < target and not blocked(cell['n'],cell['makes'],cell['audited'],screen):
            db.execute('INSERT OR IGNORE INTO jobs(cell_id,trial) VALUES (?,?)',(cell['id'],cell['n']))


JOB_SQL = '''SELECT j.*,c.hand_id,c.decl,c.audited,h.seat,h.tiles,h.seed FROM jobs j
    JOIN cells c ON c.id=j.cell_id JOIN hands h ON h.id=c.hand_id'''


def preserve(directory, binary):
    paths = sorted([*ROOT.glob('walt/walt/src/**/*.rs'),*ROOT.glob('walt/walt-player/src/**/*.rs'),
        ROOT/'walt/Cargo.toml',ROOT/'walt/Cargo.lock',ROOT/'walt/walt/Cargo.toml',
        ROOT/'walt/walt-player/Cargo.toml',Path(__file__),ROOT/'experiments/kiln/kiln.py',
        ROOT/'experiments/partnership/rules.py'])
    identity = {'binary_sha256':digest(binary),'sources':{str(p.relative_to(ROOT)):digest(p) for p in paths}}
    producer = hashlib.sha256(canonical(identity).encode()).hexdigest()
    parent = Path(directory)/'producers'
    parent.mkdir(exist_ok=True)
    folder = parent/producer
    if not folder.exists():
        temporary = Path(tempfile.mkdtemp(prefix=producer+'.pending-',dir=parent))
        try:
            target = temporary/'kiln-play-worker'
            shutil.copy2(binary,target)
            with tarfile.open(temporary/'source.tar.gz','w:gz') as archive:
                for path in paths:
                    archive.add(path,arcname=str(path.relative_to(ROOT)),recursive=False)
            atomic_json(temporary/'producer.json',{'schema':'kiln-played-producer-v2','profile':PROFILE,
                **identity,'bundle_id':producer,
                'archive_sha256':digest(temporary/'source.tar.gz'),
                'source_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                'python':sys.version,
                'rustc':subprocess.check_output(['rustc','--version'],text=True).strip()})
            for path in (target,temporary/'source.tar.gz'):
                with path.open('rb') as handle:
                    os.fsync(handle.fileno())
            os.rename(temporary,folder)
            fd = os.open(parent,os.O_RDONLY)
            try:
                os.fsync(fd)
            finally:
                os.close(fd)
        except BaseException:
            shutil.rmtree(temporary,ignore_errors=True)
            raise
    target = folder/'kiln-play-worker'
    info = json.loads((folder/'producer.json').read_text())
    if digest(target) != info['binary_sha256'] or digest(folder/'source.tar.gz') != info['archive_sha256']:
        raise ValueError('Incomplete or corrupt producer bundle')
    return target,producer


def status(db):
    cfg = manifest(db)
    row = dict(db.execute('''SELECT COUNT(*) AS games,COALESCE(SUM(score>=30),0) AS made30,
        COALESCE(SUM(policy_us),0) AS policy_us,COALESCE(SUM(opening_us),0) AS opening_us,
        COALESCE(SUM(changed),0) AS partner_changes,COALESCE(SUM(over_budget),0) AS over_budget_moves,
        COALESCE(SUM(fallbacks),0) AS fallback_moves FROM games''').fetchone())
    counts = [dict(r) for r in db.execute('''SELECT c.id,c.audited,COUNT(g.job_id) AS n,
        COALESCE(SUM(g.score>=30),0) AS makes FROM cells c LEFT JOIN jobs j ON j.cell_id=c.id
        LEFT JOIN games g ON g.job_id=j.id GROUP BY c.id''')]
    last = db.execute('SELECT * FROM runs ORDER BY id DESC LIMIT 1').fetchone()
    row.update(schema='kiln-played-status-v1',hands=cfg['hands'],cells=len(counts),
        covered_cells=sum(c['n']>0 for c in counts),
        depth={str(n):sum(c['n']>=n for c in counts) for n in (8,40,160)},
        screened_cells=sum(blocked(c['n'],c['makes'],c['audited'],cfg['screen']) for c in counts),
        states={r[0]:r[1] for r in db.execute('SELECT state,COUNT(*) FROM jobs GROUP BY state')},
        last_run=dict(last) if last else None)
    if last:
        seconds = (last['ended'] or time.time()) - last['started']
        row['last_run_seconds'] = round(seconds,3)
        row['last_run_games_per_second'] = round(last['completed']/max(seconds,.001),4)
    return row


async def run(args):
    directory = Path(args.directory)
    with writer_lock(directory):
        db = connect(directory)
        cfg = manifest(db)
        if cfg['profile'] != PROFILE:
            raise ValueError('Campaign profile mismatch')
        binary,producer = preserve(directory,Path(args.binary))
        with db:
            db.execute("UPDATE jobs SET state='pending' WHERE state='running'")
            if args.retry_failed:
                db.execute("UPDATE jobs SET state='pending',attempts=0 WHERE state='failed'")
            db.execute("UPDATE runs SET status='interrupted',ended=? WHERE status='running'",(time.time(),))
            schedule(db,args.games,cfg['screen'])
            run_id = db.execute('INSERT INTO runs(started,workers,target,producer,status) VALUES (?,?,?,?,?)',
                (time.time(),args.workers,args.games,producer,'running')).lastrowid
        stop = asyncio.Event()
        loop = asyncio.get_running_loop()
        for sig in (signal.SIGINT,signal.SIGTERM):
            loop.add_signal_handler(sig,stop.set)
        end = time.monotonic()+args.seconds if args.seconds else float('inf')

        async def worker(index):
            proc = None
            job = None
            async def kill():
                nonlocal proc
                if proc is not None and proc.returncode is None:
                    proc.kill()
                    await proc.wait()
                proc = None
            with (directory/f'worker-{index}.stderr.log').open('ab') as errors:
                try:
                    while not stop.is_set() and time.monotonic() < end:
                        job = db.execute(JOB_SQL+" WHERE j.state='pending' AND j.trial<? ORDER BY j.trial,j.id LIMIT 1",
                                         (args.games,)).fetchone()
                        if job is None:
                            # Another worker may publish the next sample shortly.
                            if not db.execute("SELECT 1 FROM jobs WHERE state='running' AND trial<? LIMIT 1",(args.games,)).fetchone():
                                return
                            await asyncio.sleep(.05)
                            continue
                        with db:
                            db.execute("UPDATE jobs SET state='running',attempts=attempts+1 WHERE id=?",(job['id'],))
                        try:
                            if proc is None:
                                proc = await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,
                                    stdout=asyncio.subprocess.PIPE,stderr=errors,limit=4*1024*1024,
                                    env={**os.environ,'RAYON_NUM_THREADS':'1'})
                            request = game_request(job)
                            proc.stdin.write((canonical(request)+'\n').encode())
                            await proc.stdin.drain()
                            line = await asyncio.wait_for(proc.stdout.readline(),args.job_seconds)
                            if not line:
                                raise RuntimeError('Game worker exited; inspect worker stderr')
                            value = json.loads(line)
                            if 'error' in value:
                                raise ValueError(value['error'])
                            metrics = validate_game(request,value)
                            raw = canonical(value).encode()
                            with db:
                                db.execute('INSERT INTO games VALUES (?,?,?,?,?,?,?,?,?,?,?,?)',
                                    (job['id'],value['score'],producer,value['elapsed_us'],*metrics,
                                     hashlib.sha256(raw).hexdigest(),zlib.compress(raw),time.time()))
                                db.execute("UPDATE jobs SET state='done',error=NULL WHERE id=?",(job['id'],))
                                db.execute('UPDATE runs SET completed=completed+1 WHERE id=?',(run_id,))
                                schedule(db,args.games,cfg['screen'],job['cell_id'])
                        except asyncio.CancelledError:
                            raise
                        except Exception as error:
                            await kill()
                            with db:
                                db.execute("UPDATE jobs SET state=CASE WHEN attempts>=3 THEN 'failed' ELSE 'pending' END,error=? WHERE id=?",
                                           (str(error),job['id']))
                                db.execute('UPDATE runs SET errors=errors+1 WHERE id=?',(run_id,))
                            print(canonical({'job':job['id'],'error':str(error)}),file=sys.stderr,flush=True)
                        job = None
                finally:
                    await kill()
                    if job is not None:
                        with db:
                            db.execute("UPDATE jobs SET state='pending' WHERE id=? AND state='running'",(job['id'],))

        tasks = [asyncio.create_task(worker(i)) for i in range(args.workers)]
        failure = None
        try:
            last = 0
            while not all(t.done() for t in tasks) and not stop.is_set() and time.monotonic() < end:
                for task in tasks:
                    if task.done() and task.exception() is not None:
                        raise task.exception()
                if time.monotonic()-last > 10:
                    view = status(db)
                    atomic_json(directory/'status.json',view)
                    print(canonical(view),flush=True)
                    last = time.monotonic()
                try:
                    await asyncio.wait_for(stop.wait(),.2)
                except asyncio.TimeoutError:
                    pass
        except BaseException as error:
            failure = error
        finally:
            for task in tasks:
                task.cancel()
            outcomes = await asyncio.gather(*tasks,return_exceptions=True)
            failure = failure or next((e for e in outcomes if isinstance(e,Exception)),None)
            with db:
                pending = db.execute("SELECT COUNT(*) FROM jobs WHERE state!='done' AND trial<?",(args.games,)).fetchone()[0]
                failed = db.execute("SELECT COUNT(*) FROM jobs WHERE state='failed'").fetchone()[0]
                result = 'failed' if failure or failed else 'stopped' if pending else 'complete'
                db.execute('UPDATE runs SET ended=?,status=? WHERE id=?',(time.time(),result,run_id))
            view = status(db)
            atomic_json(directory/'status.json',view)
            print(canonical(view),flush=True)
            db.execute('PRAGMA wal_checkpoint(FULL)')
            db.close()
            for sig in (signal.SIGINT,signal.SIGTERM):
                loop.remove_signal_handler(sig)
        if failure:
            raise failure


def panels(db):
    cfg = manifest(db)
    result = []
    for cell in db.execute('SELECT c.*,h.seed,h.seat,h.tiles FROM cells c JOIN hands h ON h.id=c.hand_id ORDER BY c.id'):
        hist = [0]*43
        for row in db.execute('SELECT score,COUNT(*) FROM games g JOIN jobs j ON j.id=g.job_id WHERE j.cell_id=? GROUP BY score',(cell['id'],)):
            hist[row[0]] = row[1]
        n = sum(hist)
        tails = [sum(hist[b:]) for b in range(30,43)]
        bid = max((b for b,num in zip(range(30,43),tails) if n and num*5 >= n*4),default=None)
        result.append({'hand_id':cell['hand_id'],'deal_seed':cell['seed'],'seat':cell['seat'],
            'hand':json.loads(cell['tiles']),'decl':cell['decl'],'games':n,'histogram':hist,
            'tails30_42':tails,'recommended_bid':bid,'audited':bool(cell['audited']),
            'screened':bool(blocked(n,tails[0],cell['audited'],cfg['screen']))})
    return result


def export(db,path):
    rows = panels(db)
    last = db.execute('SELECT target FROM runs ORDER BY id DESC LIMIT 1').fetchone()
    target = last['target'] if last else None
    book = {'schema':'kiln-played-book-v1','manifest':manifest(db),'panels':rows,
            'target_games':target,'complete':bool(target and all(r['screened'] or r['games']>=target for r in rows)),
            'producers':[r[0] for r in db.execute('SELECT DISTINCT producer FROM games ORDER BY producer')],
            'interpretation':'Observed score tails under bid30 play; 4/5 empirical recommendation, not a confidence bound.'}
    book['id'] = hashlib.sha256(canonical(book).encode()).hexdigest()
    atomic_json(path,book)
    return {'book_id':book['id'],'panels':len(book['panels']),'games':sum(p['games'] for p in book['panels'])}


def audit(db,directory):
    directory = Path(directory)
    cfg = manifest(db)
    if cfg != json.loads((directory/'manifest.json').read_text()) or cfg['profile'] != PROFILE:
        raise ValueError('Campaign manifest mismatch')
    if db.execute('SELECT COUNT(*) FROM hands').fetchone()[0] != cfg['hands']:
        raise ValueError('Incomplete hand catalogue')
    for hand in db.execute('SELECT * FROM hands'):
        if hand['seed'] != cfg['seed_start']+hand['id']//4 or hand['seat'] != hand['id']%4:
            raise ValueError('Catalogue identity mismatch')
        _, original = deal(hand['seed'])
        if json.loads(hand['tiles']) != original[hand['seat']]:
            raise ValueError('Catalogue shuffle mismatch')
    producers = []
    for (producer,) in db.execute('SELECT DISTINCT producer FROM games'):
        folder = directory/'producers'/producer
        info = json.loads((folder/'producer.json').read_text())
        if digest(folder/'kiln-play-worker') != info['binary_sha256'] or digest(folder/'source.tar.gz') != info['archive_sha256']:
            raise ValueError('Producer hash mismatch')
        if info['schema'] == 'kiln-played-producer-v2':
            identity = {k:info[k] for k in ('binary_sha256','sources')}
            if hashlib.sha256(canonical(identity).encode()).hexdigest() != producer or info['bundle_id'] != producer:
                raise ValueError('Producer identity mismatch')
        elif info['schema'] != 'kiln-played-producer-v1' or info['binary_sha256'] != producer:
            raise ValueError('Unknown producer identity')
        with tarfile.open(folder/'source.tar.gz','r:gz') as archive:
            for name,sha in info['sources'].items():
                if hashlib.sha256(archive.extractfile(name).read()).hexdigest() != sha:
                    raise ValueError('Source archive mismatch')
        producers.append(producer)
    checked = 0
    for row in db.execute(JOB_SQL+' JOIN games g ON g.job_id=j.id'):
        saved = db.execute('SELECT * FROM games WHERE job_id=?',(row['id'],)).fetchone()
        raw = zlib.decompress(saved['payload'])
        if hashlib.sha256(raw).hexdigest() != saved['sha256'] or row['state'] != 'done':
            raise ValueError('Receipt hash or queue state mismatch')
        value = json.loads(raw)
        metrics = validate_game(game_request(row),value)
        if saved['score'] != value['score'] or saved['elapsed_us'] != value['elapsed_us'] or metrics != tuple(saved[k] for k in ('policy_us','opening_us','changed','over_budget','fallbacks')):
            raise ValueError('Stored metrics mismatch')
        checked += 1
    for cell in db.execute('SELECT id FROM cells'):
        indices = [r[0] for r in db.execute('SELECT j.trial FROM jobs j JOIN games g ON j.id=g.job_id WHERE j.cell_id=? ORDER BY j.trial',(cell['id'],))]
        if indices != list(range(len(indices))):
            raise ValueError('Sample prefix has gaps')
    for hand in db.execute('SELECT * FROM hands'):
        cells = db.execute('SELECT * FROM cells WHERE hand_id=? ORDER BY decl',(hand['id'],)).fetchall()
        if [c['decl'] for c in cells] != DECLS:
            raise ValueError('Missing declaration panel')
        for cell in cells:
            expected = seeded(f'kiln-played-audit-v1/{hand["seat"]}/{hand["tiles"]}/{cell["decl"]}') % 50 == 0
            if bool(cell['audited']) != expected:
                raise ValueError('Audit sample selection changed')
    if db.execute("SELECT COUNT(*) FROM jobs j LEFT JOIN games g ON g.job_id=j.id WHERE j.state='done' AND g.job_id IS NULL").fetchone()[0]:
        raise ValueError('Completed job without receipt')
    return {'schema':'kiln-played-audit-v1','games':checked,'moves':checked*28,'producers':producers,
            'hands':cfg['hands'],'verified':True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command',required=True)
    p = sub.add_parser('init');p.add_argument('directory');p.add_argument('--hands',type=int,default=100)
    p.add_argument('--seed-start',type=int,default=420600);p.add_argument('--no-screen',action='store_true')
    p = sub.add_parser('run');p.add_argument('directory');p.add_argument('--binary',default=str(BINARY))
    p.add_argument('--workers',type=int,default=18);p.add_argument('--seconds',type=float,default=60)
    p.add_argument('--games',type=int,default=8);p.add_argument('--job-seconds',type=float,default=180)
    p.add_argument('--retry-failed',action='store_true',help='Retry failed trials, retaining their identities')
    for name in ('status','audit','export'):
        p = sub.add_parser(name);p.add_argument('directory')
        if name == 'export':p.add_argument('output')
    args = parser.parse_args()
    if args.command == 'init':
        init(args.directory,args.hands,args.seed_start,not args.no_screen)
    elif args.command == 'run':
        if args.workers < 1 or args.games < 1 or args.seconds < 0 or args.job_seconds <= 0:
            parser.error('Invalid run bounds')
        asyncio.run(run(args))
    else:
        db = connect(args.directory)
        db.execute('BEGIN')  # One consistent snapshot while production runs.
        if args.command == 'status':result = status(db)
        elif args.command == 'audit':result = audit(db,args.directory)
        else:result = export(db,args.output)
        db.close()
        print(canonical(result))


if __name__ == '__main__':
    main()
