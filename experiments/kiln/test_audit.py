import contextlib,hashlib,io,json,tarfile,tempfile,unittest
from pathlib import Path
import kiln as k
from audit import audit

class AuditTests(unittest.TestCase):
    def populate(self,directory):
        k.init(directory,1,420600);db=k.connect(directory)
        # Explicit synthetic provenance used only in this temporary fixture.
        binary=b'synthetic-test-producer';producer=hashlib.sha256(binary).hexdigest()
        folder=Path(directory)/'producers'/producer;folder.mkdir(parents=True)
        (folder/'kiln-worker').write_bytes(binary)
        source=b'fixture source';entry=tarfile.TarInfo('fixture.txt');entry.size=len(source)
        with tarfile.open(folder/'source.tar.gz','w:gz') as tar:tar.addfile(entry,io.BytesIO(source))
        k.atomic_json(folder/'producer.json',{'schema':'kiln-producer-v1','profile':k.PROFILE,'binary_sha256':producer,
            'source_commit':'synthetic-fixture','sources':{'fixture.txt':hashlib.sha256(source).hexdigest()}})
        hands=json.loads(db.execute('SELECT hands FROM deals').fetchone()[0])
        while True:
            row=db.execute("SELECT * FROM jobs WHERE state='pending' ORDER BY stage,id LIMIT 1").fetchone()
            if row is None:break
            job=dict(row);hand=hands[job['seat']]
            req={'auction':{'hand':hand,'seat':job['seat'],'bid':job['bid'],'seed':k.sample_seed(hand,job['seat'])},'worlds':k.STAGES[job['stage']]}
            result={'schema':'kiln-price-v1','auction':req['auction'],'worlds':req['worlds'],'inner_worlds':8,
                'price':[job['decl'],'0','1'],'work':{'elapsed_us':1,'nodes':1,'policy_cache_entries':0,
                    'search_cache_entries':0,'pi_calls':[0,0],'inner_worlds':[0,0]}}
            k.commit_result(db,job,req,result,producer)
        book=Path(directory)/'book.json'
        with contextlib.redirect_stdout(io.StringIO()):k.export(directory,book)
        return db,book,folder

    def test_full_audit_and_scope(self):
        with tempfile.TemporaryDirectory() as d:
            db,book,_=self.populate(d)
            report=audit(d,book,required_deals=1)
            self.assertTrue(report['complete']);self.assertEqual(report['settled_deals'],1)
            self.assertEqual(report['audit_cells']['preselected'],report['audit_cells']['at_target_depth'])
            self.assertGreater(report['audit_cells']['at_target_depth'],0)
            with self.assertRaisesRegex(ValueError,'1000'):audit(d,book)
            db.close()

    def test_missing_refinement_cannot_be_declared_complete(self):
        with tempfile.TemporaryDirectory() as d:
            db,book,_=self.populate(d)
            ident=db.execute('SELECT id FROM jobs WHERE stage=2 LIMIT 1').fetchone()[0]
            with db:
                db.execute('DELETE FROM results WHERE job_id=?',(ident,));db.execute('DELETE FROM jobs WHERE id=?',(ident,))
            with self.assertRaisesRegex(ValueError,'refinement job'):audit(d,partial=True,required_deals=1)
            db.close()

    def test_wrong_receipt_and_forged_book_are_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            db,book,_=self.populate(d)
            with db:db.execute('UPDATE results SET num=1 WHERE job_id=1')
            with self.assertRaisesRegex(ValueError,'receipt identity'):audit(d,partial=True,required_deals=1)
            with db:db.execute('UPDATE results SET num=0 WHERE job_id=1')
            b=json.loads(book.read_text());b['deals'][0]['prices'][0][3]=1;b.pop('id')
            b['id']=hashlib.sha256(k.canonical(b).encode()).hexdigest();k.atomic_json(book,b)
            with self.assertRaisesRegex(ValueError,'matching original receipt'):audit(d,book,required_deals=1)
            db.close()

    def test_corrupt_producer_is_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            db,book,producer=self.populate(d)
            (producer/'kiln-worker').write_bytes(b'changed')
            with self.assertRaisesRegex(ValueError,'Producer binary'):audit(d,book,required_deals=1)
            db.close()

    def test_partial_audit_is_never_complete(self):
        with tempfile.TemporaryDirectory() as d:
            k.init(d,1,420600)
            report=audit(d,partial=True,required_deals=1)
            self.assertFalse(report['complete']);self.assertEqual(report['states'],{'pending':468})
            with self.assertRaisesRegex(ValueError,'incomplete'):audit(d,required_deals=1)

    def test_carried_policy_answers_are_distinguished_from_new_work(self):
        with tempfile.TemporaryDirectory() as d:
            db,book,_=self.populate(d)
            value=json.loads(db.execute('SELECT payload FROM results WHERE job_id=1').fetchone()[0])
            value['work'].update(carried_policy_entries=3,policy_cache_entries=3)
            with db:db.execute('UPDATE results SET payload=? WHERE job_id=1',(k.canonical(value),))
            self.assertTrue(audit(d,book,required_deals=1)['complete'])
            for carried,entries in [(4,3),(-1,3),(0.5,3),(3,4)]:
                value['work'].update(carried_policy_entries=carried,policy_cache_entries=entries)
                with db:db.execute('UPDATE results SET payload=? WHERE job_id=1',(k.canonical(value),))
                with self.assertRaisesRegex(ValueError,'carried-policy'):audit(d,book,required_deals=1)
            db.close()
if __name__=='__main__':unittest.main()
