"""Synthetic-only staged confirmation contracts; no confirmation receipt reads."""
import copy
from fractions import Fraction
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import tempfile
import unittest
from unittest import mock

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('confirmation_report',HERE/'compiled_confirmation_report.py')
R=importlib.util.module_from_spec(spec);spec.loader.exec_module(R)
ROOT=Path(os.environ.get('CONFIRMATION_TEST_ROOT',
    '/Users/jason/.codex/worktrees/walt-response-ladder/texas-42/experiments/response-ladder'))
H=R.load_module('test_confirmation_harness',ROOT/'tools/h2h_compiled.py')
RULES=R.load_module('test_confirmation_rules',Path('/Users/jason/code/texas-42-partnership-launch/experiments/partnership/rules.py'))


def fake_strata(ds):
    return {(p,d,b):[{'index':di*64+b*16+i,'delta':x,
                      'candidate_declaring_make':x>=0,'candidate_defending_set':x>0}
                     for i,x in enumerate(ds)]
            for p in ['synthetic-a','synthetic-b'] for di,d in enumerate(R.DECLS) for b in range(4)}


def physical_game():
    # Global index575, declaration9: explicitly catch confusing declaration8
    # or per-cell repeat0..15 with the real harness's global fixture index.
    f=H.fixture_panel('report-unit-synthetic',16)[575]
    protocol={'candidate_config':{'outer':40,'plans':1,'horizon':7,'work':2000000,'compiled_tail':True},'candidate_ms':20}
    record=[];moves=[];arm='declaring';candidate_parity=f['bidder']%2
    for turn in range(28):
        points,leader,remaining,trick=RULES.replay_record(f['hands'],record,f['decl'],f['bidder'])
        actor=(leader+len(trick))%4
        legal=RULES.legal_tiles(remaining[actor],trick,f['decl']);tile=legal[0]
        candidate=actor%2==candidate_parity
        request={'decl':f['decl'],'bid':30,'bidder':f['bidder'],'seat':actor,'seed':f['policy_seed']}
        if candidate:
            request.update(hand=sorted(remaining[actor]),original_hand=f['hands'][actor],
                history=[record[i:i+2] for i in range(0,len(record),2)],config=protocol['candidate_config'],budget_ms=20)
            response={'tile':tile,'compute':{'backend':'compiled-cpu'},'fallback':False}
        else:
            request.update(hand=f['hands'][actor],plays=record[:]);response={'choice':tile,'route':'partner'}
        moves.append({'actor':actor,'tile':tile,'player':'candidate' if candidate else 'cpu',
            'trick':turn//4+1,'legal':legal,'elapsed_ms':1.25,'request':request,'response':response})
        record.extend([actor,tile])
    points,_,_,_=RULES.replay_record(f['hands'],record,f['decl'],f['bidder'])
    game={'schema':'compiled-h2h-game-v1','fixture':f,'pairgroup':f['pairgroup'],
        'balanced_order':f['balanced_order'],'arm':arm,'moves':moves,'record':record,
        'points':points,'declaring_made':points[f['bidder']%2]>=30,'independent_replay_passed':True}
    return game,f,protocol


class ConfirmationContracts(unittest.TestCase):
    def test_independent_fraction_variance_balanced_effect_and_sign(self):
        result=R.inference(fake_strata([1]*6+[-1]*2+[0]*8),['synthetic-a','synthetic-b'])
        self.assertEqual(result['wins'],432);self.assertEqual(result['losses'],144)
        self.assertEqual(Fraction(result['delta']['numerator'],result['delta']['denominator']),Fraction(1,4))
        self.assertEqual(Fraction(result['variance_estimate']['numerator'],result['variance_estimate']['denominator']),Fraction(7,17280))
        self.assertEqual(result['cells'][0]['unbiased_sample_variance']['decimal'],float(Fraction(7,15)))
        self.assertTrue(all(result['gates'].values()))
        self.assertEqual(R.sign_probability(3,1),Fraction(5,16))
        self.assertEqual(R.sign_probability(0,0),Fraction(1))
        self.assertAlmostEqual(result['hoeffding_half_width'],math.sqrt(2*math.log(40)/1152))

    def test_zero_variance_or_missing_unbalanced_cell_never_passes(self):
        strata=fake_strata([1]*16)
        result=R.inference(strata,['synthetic-a','synthetic-b'])
        self.assertEqual(result['mean_inference_status'],'unresolved_zero_variance')
        self.assertFalse(result['gates']['normal99_lower_positive'])
        self.assertIsNone(result['one_sided_normal_p'])
        missing=copy.deepcopy(strata);missing.pop(next(iter(missing)))
        with self.assertRaisesRegex(ValueError,'strata'):R.inference(missing,['synthetic-a','synthetic-b'])
        wrong=copy.deepcopy(strata);wrong[next(iter(wrong))].pop()
        with self.assertRaisesRegex(ValueError,'16 physical'):R.inference(wrong,['synthetic-a','synthetic-b'])

    def test_positive_each_panel_is_required_even_when_pooled_positive(self):
        strata=fake_strata([1]*8+[0]*8)
        for key in strata:
            if key[0]=='synthetic-b':
                strata[key]=[{'delta':-1,'candidate_declaring_make':False,'candidate_defending_set':False}]+[
                    {'delta':0,'candidate_declaring_make':False,'candidate_defending_set':True} for _ in range(15)]
        result=R.inference(strata,['synthetic-a','synthetic-b'])
        self.assertGreater(result['delta']['decimal'],0)
        self.assertFalse(result['gates']['positive_each_panel'])

    def test_nearest_rank_and_exact_latency_boundary(self):
        self.assertEqual(R.nearest_rank(list(range(1,21)),Fraction(19,20)),19)
        self.assertEqual(R.nearest_rank([1,2],Fraction(1,2)),1)
        self.assertEqual(R.nearest_rank(list(range(1,101)),Fraction(99,100)),99)
        games=[]
        for arm in R.ARMS:
            moves=[]
            for trick in range(1,8):
                for player,cost in [('candidate',1.1),('cpu',1.0)]:
                    for _ in range(2):moves.append({'player':player,'trick':trick,'elapsed_ms':cost,'response':{}})
            games.append({'arm':arm,'moves':moves})
        result=R.latency(games)
        self.assertTrue(all(result['gates'].values()))
        self.assertEqual(result['ratios']['mean']['numerator'],11)
        self.assertEqual(result['ratios']['mean']['denominator'],10)
        games[0]['moves'][0]['elapsed_ms']=1.10001
        self.assertFalse(R.latency(games)['gates']['mean_at_most_110pct'])

    def test_real_shaped_fixtures_use_decl9_and_global_indices(self):
        fixtures=H.fixture_panel('report-unit-synthetic',16)
        spec={'fixture_hashes':{'report-unit-synthetic':hashlib.sha256(R.canonical(fixtures).encode()).hexdigest()}}
        R.check_fixtures(fixtures,'report-unit-synthetic',spec,H)
        self.assertEqual(fixtures[-1]['index'],575);self.assertEqual(fixtures[-1]['decl'],9)
        wrong=copy.deepcopy(fixtures);wrong[-1]['index']=15
        with self.assertRaises(ValueError):R.check_fixtures(wrong,'report-unit-synthetic',spec,H)
        wrong=copy.deepcopy(fixtures);wrong[-1]['decl']=8
        with self.assertRaises(ValueError):R.check_fixtures(wrong,'report-unit-synthetic',spec,H)

    def test_all_physical_moves_and_requests_replay_and_mutations_reject(self):
        game,f,protocol=physical_game()
        R.validate_game(game,f,'declaring',protocol,H,RULES)
        for mutation in ('outcome','actor','player','elapsed','request','response','missing_move'):
            bad=copy.deepcopy(game)
            if mutation=='outcome':bad['declaring_made']=not bad['declaring_made']
            if mutation=='actor':bad['moves'][0]['actor']=(bad['moves'][0]['actor']+1)%4
            if mutation=='player':bad['moves'][0]['player']='cpu'
            if mutation=='elapsed':bad['moves'][0]['elapsed_ms']=-1
            if mutation=='request':bad['moves'][0]['request']['seed']+=1
            if mutation=='response':bad['moves'][0]['response']['tile']=(bad['moves'][0]['tile']+1)%28
            if mutation=='missing_move':bad['moves'].pop()
            with self.assertRaises(ValueError,msg=mutation):R.validate_game(bad,f,'declaring',protocol,H,RULES)

    def test_whole_game_cache_requires_unchanged_source_and_complete_commit(self):
        game,f,protocol=physical_game()
        with tempfile.TemporaryDirectory() as td:
            p=Path(td);source=p/'575-declaring.json';cache=p/'cache/575-declaring.json'
            source.write_text(json.dumps(game))
            first=R.validate_cached_game(source,cache,f,'declaring',protocol,H,RULES)
            with mock.patch.object(H,'verify_game',side_effect=AssertionError('cached source should not replay')):
                second=R.validate_cached_game(source,cache,f,'declaring',protocol,H,RULES)
            self.assertEqual(first[0],second[0]);self.assertEqual(first[1],second[1])
            saved=json.loads(cache.read_text())
            broken=copy.deepcopy(saved);broken['compact']['moves'][0]['elapsed_ms']+=1
            cache.write_text(json.dumps(broken))
            with self.assertRaisesRegex(ValueError,'checksum'):
                R.validate_cached_game(source,cache,f,'declaring',protocol,H,RULES)
            broken=copy.deepcopy(saved);broken['compact']['moves'].pop()
            broken['compact_sha256']=hashlib.sha256(R.canonical(broken['compact']).encode()).hexdigest()
            cache.write_text(json.dumps(broken))
            with self.assertRaisesRegex(ValueError,'28 moves'):
                R.validate_cached_game(source,cache,f,'declaring',protocol,H,RULES)
            cache.write_text(json.dumps(saved))
            game['extra_changed_field']=True;source.write_text(json.dumps(game))
            with self.assertRaisesRegex(ValueError,'previously validated game changed'):
                R.validate_cached_game(source,cache,f,'declaring',protocol,H,RULES)

    def test_missing_duplicate_inventory_and_unclosed_cycle_rejected(self):
        with tempfile.TemporaryDirectory() as td:
            p=Path(td);fixture={'index':575}
            for arm in R.ARMS:(p/f'575-{arm}.json').write_text('{"schema":"compiled-h2h-game-v1"}')
            self.assertEqual(len(R.game_inventory(p,[fixture])),2)
            (p/'copy.json').write_text('{"schema":"compiled-h2h-game-v1"}')
            with self.assertRaisesRegex(ValueError,'duplicate'):R.game_inventory(p,[fixture])
            (p/'copy.json').unlink();(p/'575-defending.json').unlink()
            with self.assertRaisesRegex(ValueError,'inventory'):R.game_inventory(p,[fixture])
        identities={'fixed':'same'}
        warm={'ready':{'backend':'compiled-cpu'},'started_at':1,'start_identities':identities,'threads':6,'candidate_warmup':{'tile':1},
            'cpu_warmup':{'choice':1},'startup_ms':1,'candidate_warmup_ms':2,'cpu_warmup_ms':3}
        start={'started_at':1,'warmup':warm}
        end={'schema':'compiled-h2h-cycle-v1','started_at':1,'finished_at':2,
            'start_identities':identities,'end_identities':identities,'identity_drift':False,
            'failed_arms':0,'planned_arms':1152,'completed_arms':1152,'stop_reason':'exhausted',
            'threads':6,'seconds':55,'summary_complete':True}
        self.assertEqual(R.check_cycles([start,end],identities,1152,6)['cycles'],1)
        with self.assertRaisesRegex(ValueError,'missing cycle'):R.check_cycles([start],identities,1152,6)
        bad=copy.deepcopy(end);bad['identity_drift']=True
        with self.assertRaisesRegex(ValueError,'identity drift'):R.check_cycles([start,bad],identities,1152,6)
        bad=copy.deepcopy(end);bad['failed_arms']=1
        with self.assertRaises(ValueError):R.check_cycles([start,bad],identities,1152,6)


if __name__=='__main__':
    unittest.main()
