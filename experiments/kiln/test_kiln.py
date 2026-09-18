import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time
import unittest

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('kiln',HERE/'kiln.py')
k=importlib.util.module_from_spec(spec);spec.loader.exec_module(k)

class KilnTests(unittest.TestCase):
    def test_abrupt_death_preserves_commits_and_reclaims_leases(self):
        with tempfile.TemporaryDirectory() as folder:
            k.init(folder,1,420600)
            fake=Path(folder)/'fake-worker'
            fake.write_text('''#!/usr/bin/env python3
import json,sys,time
for line in sys.stdin:
 r=json.loads(line);time.sleep(.03)
 print(json.dumps({'schema':'kiln-price-v1','auction':r['auction'],'worlds':r['worlds'],
 'inner_worlds':8,'price':[r['decl'],'0','1'],'work':{'elapsed_us':30000}}),flush=True)
''')
            fake.chmod(0o755)
            cmd=['python3',str(HERE/'kiln.py'),'run',folder,'--binary',str(fake),'--workers','2']
            first=subprocess.Popen(cmd+['--seconds','10'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
            db=k.connect(folder);deadline=time.monotonic()+8
            try:
                while db.execute('SELECT count(*) FROM results').fetchone()[0]<5:
                    self.assertLess(time.monotonic(),deadline);time.sleep(.05)
                saved={r['job_id']:r['payload'] for r in db.execute('SELECT job_id,payload FROM results')}
                first.kill();first.wait(timeout=3)
                subprocess.run(cmd+['--seconds','1'],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,check=True,timeout=8)
                self.assertEqual(db.execute('PRAGMA integrity_check').fetchone()[0],'ok')
                current={r['job_id']:r['payload'] for r in db.execute('SELECT job_id,payload FROM results')}
                self.assertGreater(len(current),len(saved))
                for ident,payload in saved.items():self.assertEqual(current[ident],payload)
                self.assertEqual(db.execute("SELECT count(*) FROM jobs WHERE state='running'").fetchone()[0],0)
                self.assertEqual(db.execute("SELECT status FROM runs WHERE id=1").fetchone()[0],'recovered')
            finally:
                if first.poll() is None:first.kill();first.wait()
    def test_generator_matches_plunge_rng(self):
        # Execute the actual shipped RNG implementation, not a second translation.
        rng=Path('/Users/jason/code/plunge-sunshine/src/engine/rng.ts')
        js=f'''import {{mulberry32,nextRngState,shuffled}} from {json.dumps(rng.as_uri())};
        const out=[];for(const seed of [0,1,420600,4294967295,...Array.from({{length:1000}},(_,i)=>420600+i)]){{
        const shaker=Math.floor(mulberry32(seed)()*4);
        const ids=shuffled(Array.from({{length:28}},(_,i)=>i),mulberry32(nextRngState(seed)));
        out.push([seed,shaker,Array.from({{length:4}},(_,i)=>ids.slice(i*7,i*7+7).sort((a,b)=>a-b))]);}}
        console.log(JSON.stringify(out));'''
        rows=json.loads(subprocess.check_output(['node','--input-type=module','-e',js],text=True))
        for seed,shaker,hands in rows:self.assertEqual(k.deal(seed),(shaker,hands))

    def test_transaction_identity_progression_and_export(self):
        with tempfile.TemporaryDirectory() as folder:
            k.init(folder,1,420600);db=k.connect(folder)
            jobs=[dict(r) for r in db.execute('SELECT * FROM jobs')]
            self.assertEqual(len(jobs),468)
            hands=json.loads(db.execute('SELECT hands FROM deals').fetchone()[0])
            for job in jobs:
                hand=hands[job['seat']]
                req={'auction':{'hand':hand,'seat':job['seat'],'bid':job['bid'],'seed':k.sample_seed(hand,job['seat'])},'worlds':8}
                result={'schema':'kiln-price-v1','auction':req['auction'],'worlds':8,'inner_worlds':8,
                        'price':[job['decl'],'0','1'],'work':{'elapsed_us':1}}
                if job['id']==1:
                    bad={**result,'auction':{**req['auction'],'bid':29}}
                    with self.assertRaises(ValueError):k.commit_result(db,job,req,bad,'test')
                    self.assertEqual(db.execute('SELECT count(*) FROM results').fetchone()[0],0)
                k.commit_result(db,job,req,result,'test')
            self.assertEqual(k.status(db)['covered_deals'],1)
            expected=sum(k.audited(j) for j in jobs)
            self.assertGreater(expected,0)
            self.assertEqual(db.execute("SELECT count(*) FROM jobs WHERE stage=1").fetchone()[0],expected)
            out=Path(folder)/'book.json';k.export(folder,out)
            self.assertEqual(len(json.loads(out.read_text())['deals'][0]['prices']),468)
            self.assertEqual(db.execute('PRAGMA integrity_check').fetchone()[0],'ok')

    def test_kiln_matches_existing_auction_endpoint(self):
        old=k.ROOT/'walt/target/release/walt-table'
        new=k.DEFAULT_BINARY
        _,hands=k.deal(420600)
        env={**os.environ,'RAYON_NUM_THREADS':'1'}
        for seat,decl,bid in [(0,6,36),(1,9,30),(2,7,42),(3,0,31)]:
            req={'hand':hands[seat],'seat':seat,'bid':bid,'seed':k.sample_seed(hands[seat],seat)}
            job={'auction':req,'decl':decl,'worlds':4,'budget_ms':20000}
            raw=subprocess.check_output([str(new)],input=json.dumps(job)+'\n',text=True,env=env)
            value=json.loads(raw)
            call={'auction_price':req,'decl':decl,'worlds':4,'budget_ms':20000}
            before=subprocess.check_output([str(old)],input=json.dumps(call)+'\n',text=True,env=env)
            expected=json.loads(before.splitlines()[-1])['result']
            self.assertNotIn('error',value);self.assertNotIn('error',expected)
            self.assertEqual(value['price'],expected['price'])

if __name__=='__main__':unittest.main()
