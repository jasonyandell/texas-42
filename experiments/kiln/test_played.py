import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

HERE = Path(__file__).resolve().parent
sys.path.insert(0,str(HERE))
import played as p


def fixture(request):
    """Legal first-choice data for storage fault tests, never player evidence."""
    record,decisions = [],[]
    for ply in range(28):
        points,leader,remaining,trick = p.replay_record(request['hands'],record,request['decl'],request['bidder'])
        actor = (leader+len(trick))%4
        legal = p.legal_tiles(remaining[actor],trick,request['decl'])
        call = {'request':{'decl':request['decl'],'bid':30,'bidder':request['bidder'],
            'seat':actor,'hand':request['hands'][actor],'plays':record.copy(),'seed':request['seed']},
            'worlds':160 if ply==0 else 40,'partner':ply!=0,'budget_ms':20000 if ply==0 else 14000}
        response = {'choice':legal[0],'legal':legal,'leader':leader,'points':points,'trick':ply//4+1,
            'elapsed_us':1,'player_version':'walt-table-v2','route':'baseline'}
        decisions.append({'call':call,'response':response})
        record.extend((actor,legal[0]))
    points,_,_,_ = p.replay_record(request['hands'],record,request['decl'],request['bidder'])
    return {**request,'schema':'kiln-played-game-v1','play_bid':30,'decisions':decisions,
        'record':record,'points':points,'score':points[request['bidder']%2],'elapsed_us':28}


class PlayedTests(unittest.TestCase):
    def test_information_boundary_scoring_and_all_declarations(self):
        for seat in range(4):
            for decl in p.DECLS:
                job = {'seat':seat,'decl':decl,'tiles':json.dumps(p.deal(420600)[1][seat]),'trial':0}
                request = p.game_request(job)
                value = fixture(request)
                p.validate_game(request,value)
                bad = copy.deepcopy(value)
                bad['decisions'][2]['call']['request']['hands'] = request['hands']
                with self.assertRaisesRegex(ValueError,'information set'):
                    p.validate_game(request,bad)
                bad = copy.deepcopy(value);bad['score'] = 42-value['score']
                if bad['score'] != value['score']:
                    with self.assertRaisesRegex(ValueError,'final score'):
                        p.validate_game(request,bad)
                bad = copy.deepcopy(value);bad['decisions'].pop()
                with self.assertRaisesRegex(ValueError,'seven tricks'):
                    p.validate_game(request,bad)

    def test_paired_worlds_and_prefix_stability(self):
        job = {'seat':1,'decl':0,'tiles':json.dumps(p.deal(420600)[1][1]),'trial':7}
        a = p.game_request(job);b = p.game_request({**job,'decl':9})
        self.assertEqual(a['hands'],b['hands']);self.assertEqual(a['seed'],b['seed'])
        self.assertEqual(sorted(sum(a['hands'],[])),list(range(28)))
        self.assertEqual(a['hands'][1],json.loads(job['tiles']))
        self.assertNotEqual(a['hands'],p.game_request({**job,'trial':8})['hands'])
        self.assertEqual(a,p.game_request(job))

    def test_abrupt_death_resume_progression_and_audit(self):
        with tempfile.TemporaryDirectory() as folder:
            p.init(folder,hands=1,screen=False)
            worker = Path(folder)/'test-worker'
            worker.write_text(f'''#!/usr/bin/env python3
import sys,json,time
sys.path.insert(0,{str(HERE)!r})
from test_played import fixture
for line in sys.stdin:
 time.sleep(.035)
 print(json.dumps(fixture(json.loads(line))),flush=True)
''')
            worker.chmod(0o755)
            command = [sys.executable,str(HERE/'played.py'),'run',folder,'--binary',str(worker),
                       '--workers','2','--games','2']
            first = subprocess.Popen(command+['--seconds','30'],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE)
            db = p.connect(folder)
            try:
                end = time.monotonic()+8
                while db.execute('SELECT COUNT(*) FROM games').fetchone()[0] < 4:
                    self.assertLess(time.monotonic(),end)
                    self.assertIsNone(first.poll(),first.stderr.read() if first.poll() is not None else '')
                    time.sleep(.02)
                saved = {r[0]:r[1] for r in db.execute('SELECT job_id,payload FROM games')}
                with self.assertRaisesRegex(ValueError,'Another coordinator'):
                    with p.writer_lock(folder):pass
                first.kill();first.wait(timeout=3)
                subprocess.run(command+['--seconds','0'],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,check=True,timeout=10)
                self.assertEqual(db.execute('SELECT COUNT(*) FROM games').fetchone()[0],18)
                self.assertEqual(db.execute("SELECT COUNT(*) FROM jobs WHERE state='running'").fetchone()[0],0)
                self.assertEqual(db.execute('SELECT status FROM runs WHERE id=1').fetchone()[0],'interrupted')
                for key,value in saved.items():
                    self.assertEqual(db.execute('SELECT payload FROM games WHERE job_id=?',(key,)).fetchone()[0],value)
                # Increasing N adds observations, never recomputes the first two.
                command[command.index('--games')+1] = '3'
                subprocess.run(command+['--seconds','0'],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,check=True,timeout=10)
                self.assertEqual(db.execute('SELECT COUNT(*) FROM games').fetchone()[0],27)
                self.assertEqual(p.audit(db,folder)['moves'],27*28)
                book_path = Path(folder)/'book.json';p.export(db,book_path)
                book = json.loads(book_path.read_text())
                for panel in book['panels']:
                    self.assertEqual(sum(panel['histogram']),3)
                    self.assertEqual(panel['tails30_42'],sorted(panel['tails30_42'],reverse=True))
                    bid = max((b for b in range(30,43) if sum(panel['histogram'][b:])*5 >= 3*4),default=None)
                    self.assertEqual(panel['recommended_bid'],bid)
                self.assertEqual(db.execute('PRAGMA integrity_check').fetchone()[0],'ok')
            finally:
                if first.poll() is None:first.kill();first.wait(timeout=3)
                first.stderr.close();db.close()

    def test_screening_never_invents_games_and_audit_bypasses(self):
        self.assertTrue(p.blocked(8,1,False,True))
        self.assertFalse(p.blocked(8,2,False,True))
        self.assertTrue(p.blocked(40,19,False,True))
        self.assertFalse(p.blocked(40,20,False,True))
        self.assertFalse(p.blocked(8,0,True,True))
        self.assertFalse(p.blocked(8,0,False,False))

    def test_extension_preserves_catalogue_trial_identity_and_is_idempotent(self):
        with tempfile.TemporaryDirectory() as folder:
            p.init(folder,hands=4)
            db=p.connect(folder)
            hands=[tuple(r) for r in db.execute('SELECT * FROM hands ORDER BY id')]
            jobs=[tuple(r) for r in db.execute('SELECT * FROM jobs ORDER BY id')]
            request=p.game_request(db.execute(p.JOB_SQL+' WHERE j.id=1').fetchone())
            with p.writer_lock(folder):
                with self.assertRaisesRegex(ValueError,'Another coordinator'):p.extend(folder,8)
            record=p.extend(folder,8)
            self.assertEqual(record['before']['hands'],4)
            self.assertEqual(p.manifest(db)['hands'],8)
            self.assertEqual([tuple(r) for r in db.execute('SELECT * FROM hands WHERE id<4 ORDER BY id')],hands)
            self.assertEqual([tuple(r) for r in db.execute('SELECT * FROM jobs WHERE id<=36 ORDER BY id')],jobs)
            self.assertEqual(p.game_request(db.execute(p.JOB_SQL+' WHERE j.id=1').fetchone()),request)
            self.assertEqual(db.execute('SELECT COUNT(*) FROM cells').fetchone()[0],72)
            self.assertEqual(p.extend(folder,8),record)
            with self.assertRaisesRegex(ValueError,'preserve'):p.extend(folder,4)
            db.close()

    def test_uncertainty_targets_all_score_tails_and_checks_only_at_stages(self):
        plan={'base_games':160,'cap_games':640,'checkpoints':[160,320,640]}
        # Certain make30 can still leave a higher bid unresolved.
        tails=[160]*6+[128]*7
        self.assertEqual(p.uncertain_tails(160,tails),list(range(36,43)))
        self.assertEqual(p.allocation_state(160,tails,False,True,plan),'refining')
        self.assertEqual(p.allocation_state(160,[160]*13,False,True,plan),'resolved')
        self.assertEqual(p.allocation_state(161,[161]*13,False,True,plan),'refining')
        self.assertEqual(p.allocation_state(320,[320]*13,False,True,plan),'resolved')
        self.assertEqual(p.allocation_state(640,[512]*13,False,True,plan),'capped-unsettled')
        self.assertEqual(p.allocation_state(160,[0]*13,True,True,plan),'refining-audit')
        self.assertEqual(p.allocation_state(640,[0]*13,True,True,plan),'audit-complete')
        self.assertEqual(p.allocation_state(8,[1]*13,False,True,plan),'screened')

    def test_adaptive_queue_and_export_resolve_without_inventing_max_depth(self):
        with tempfile.TemporaryDirectory() as folder:
            p.init(folder,hands=1,screen=False);p.extend(folder,1)
            db=p.connect(folder)
            cell=db.execute('SELECT id FROM cells WHERE audited=0 LIMIT 1').fetchone()[0]
            def add(trial,score):
                with db:
                    db.execute('INSERT OR IGNORE INTO jobs(cell_id,trial) VALUES (?,?)',(cell,trial))
                    job=db.execute('SELECT id FROM jobs WHERE cell_id=? AND trial=?',(cell,trial)).fetchone()[0]
                    # Synthetic storage fixtures, never evidence from a player.
                    db.execute('INSERT INTO games VALUES (?,?,?,?,?,?,?,?,?,?,?,?)',
                        (job,score,'fixture',0,0,0,0,0,0,'fixture',b'fixture',0))
                    db.execute("UPDATE jobs SET state='done' WHERE id=?",(job,))
                    p.schedule(db,160,False,cell)
            for i in range(160):add(i,30 if i<128 else 0)
            self.assertEqual(db.execute("SELECT trial FROM jobs WHERE cell_id=? AND state='pending'",(cell,)).fetchone()[0],160)
            for i in range(160,320):add(i,0)
            self.assertEqual(db.execute("SELECT COUNT(*) FROM jobs WHERE cell_id=? AND state='pending'",(cell,)).fetchone()[0],0)
            out=Path(folder)/'book.json';p.export(db,out)
            book=json.loads(out.read_text());row=next(r for r in book['panels'] if r['decl']==db.execute('SELECT decl FROM cells WHERE id=?',(cell,)).fetchone()[0])
            self.assertEqual(row['games'],320);self.assertEqual(row['allocation_state'],'resolved')
            self.assertFalse(book['complete'])
            self.assertEqual(p.status(db)['allocation_states']['resolved'],1)
            db.close()


if __name__ == '__main__':unittest.main()
