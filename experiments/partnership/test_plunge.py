"""Cross-engine hand replay, live input boundary, durable evidence and resume."""
from copy import deepcopy
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import gym
import campaign as c
from override_study import choice
from plunge_io import decode_hand, flag_root
from plunge_bridge import Store
from plunge_analysis import analyze
from rules import information_state

FIXTURE=gym.HERE/'campaigns/sunshine-playable-v1/codec-audit.json'


class PlungeTests(unittest.TestCase):
    def fake(self,request,**kwargs):
        self.assertEqual(set(request),gym.INPUT_KEYS)
        state=information_state(request)
        return dict(choice=state['legal'][0],legal=state['legal'],leader=state['leader'],points=state['points'],
                    route='test',elapsed_us=0,over_budget=False)

    def body(self,index=22):
        game=c.read(FIXTURE)[3];req,played,_=flag_root(game['code'],index,game['positions'][index]['request']['seed'])
        return dict(request=req,player='l1-partner-rollout',game_id='unit',hand_number=1),dict(
            share_code=game['code'],ply=index,seed=req['seed'],note='Save this decision',alternative=played,receipt_id=None)

    def test_independent_decoder_agrees_across_nine_plunge_declarations(self):
        for game in c.read(FIXTURE):
            self.assertEqual(decode_hand(game['code'])['points'],game['points'])
            for ply,expected in enumerate(game['positions']):
                req,_,_=flag_root(game['code'],ply,expected['request']['seed'])
                self.assertEqual(req,expected['request'])
                state=information_state(req)
                for name in ('legal','leader','points'):self.assertEqual(state[name],expected[name])

    def test_malformed_and_illegal_hand_codes_are_rejected(self):
        code=c.read(FIXTURE)[0]['code']
        bads=['',code[:-2],code+'00',code.replace('30PPP','29PPP'),code.replace('D0','D8'),
              code[:4]+code[6:8]+code[6:],code[:-2]+code[-4:-2],code.replace('30PPP','PPPP')]
        for bad in bads:
            with self.subTest(code=bad),self.assertRaises(ValueError):decode_hand(bad)
        for ply in (-1,28,True):
            with self.assertRaises(ValueError):flag_root(code,ply,1)

    def test_higher_straight_contracts_preserve_targets_and_scoring(self):
        for row in c.read(gym.HERE/'campaigns/regular-bidding-v1/codec.json'):
            decoded=decode_hand(row['code'])
            self.assertEqual(decoded['points'],row['points'])
            self.assertEqual(decoded['bid'],row['bid'])
            req,_,_=flag_root(row['code'],0,42)
            self.assertEqual(req['bid'],row['bid'])
            with tempfile.TemporaryDirectory() as tmp:
                store=Store(tmp)
                try:
                    flag=store.flag(dict(share_code=row['code'],ply=0,seed=42,note='higher bid',alternative=None,receipt_id=None))
                    self.assertEqual(store.analysis(flag['id'],'l1',True)['status'],'outside-scope')
                finally:store.close()

    def test_retry_freezes_original_answer_and_player_identity(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_bridge.decide',side_effect=self.fake) as decide:
            store=Store(tmp)
            try:
                body,_=self.body();first=store.decision(body)
                self.assertEqual(store.decision(body),first);self.assertEqual(decide.call_count,1)
                other=store.decision({**body,'player':'l1-default'})
                self.assertNotEqual(first['id'],other['id']);self.assertEqual(decide.call_count,2)
                for extra in ('hands','worlds','teacher','dealt'):
                    bad=deepcopy(body);bad['request'][extra]=[]
                    with self.assertRaises(ValueError):store.decision(bad)
                self.assertEqual(decide.call_count,2)
                path=Path(tmp)/'decisions'/(first['id']+'.json');bad=deepcopy(first);bad['response']['choice']+=1;gym.atomic(path,bad)
                with self.assertRaisesRegex(ValueError,'contents changed'):store.decision(body)
            finally:store.close()

    def test_only_bidder_opening_uses_deeper_l1_without_partner_review(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_bridge.decide',side_effect=self.fake) as decide:
            store=Store(tmp)
            try:
                body,_=self.body()
                opening={**body,'request':{**body['request'],'seat':body['request']['bidder'],'plays':[]}}
                store.decision(opening)
                self.assertEqual(decide.call_args.kwargs['n'],160)
                self.assertEqual(decide.call_args.kwargs['budget_ms'],20000)
                self.assertEqual(decide.call_args.kwargs['review'],'off')
                self.assertEqual(c.read(next((Path(tmp)/'decisions').glob('*.json')))['identity']['player']['n'],160)
                decide.reset_mock()
                store.decision(body)
                self.assertEqual(decide.call_args.kwargs['n'],40)
                self.assertEqual(decide.call_args.kwargs['budget_ms'],14000)
                self.assertEqual(decide.call_args.kwargs['review'],'partner-rollout')
            finally:store.close()

    def test_flag_keeps_original_receipt_but_gym_input_excludes_examiner_hands(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_bridge.decide') as decide:
            store=Store(tmp)
            try:
                body,flag=self.body()
                decide.return_value={**self.fake(body['request']),'choice':flag['alternative']}
                receipt=store.decision(body)
                value=store.flag({**flag,'receipt_id':receipt['id']})
                self.assertEqual(value['original_receipt'],receipt)
                self.assertEqual(value['request'],body['request'])
                pupil=c.read(Path(tmp)/'gym-inputs'/(value['id']+'.json'))
                self.assertEqual(set(pupil['request']),gym.INPUT_KEYS);self.assertNotIn('examiner_game',pupil)
                with self.assertRaises(ValueError):store.flag({**flag,'receipt_id':receipt['id'],'ply':23})
                illegal=next(t for t in range(28) if t not in information_state(body['request'])['legal'])
                with self.assertRaises(ValueError):store.flag({**flag,'alternative':illegal})
            finally:store.close()

    def test_inspection_cache_is_separate_and_tracks_worlds_without_rewriting_play(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_bridge.decide',side_effect=lambda req,**kw:{**self.fake(req),'route':'baseline'}) as decide:
            store=Store(tmp)
            try:
                body,_=self.body();original=deepcopy(store.decision(body))
                query=dict(request=body['request'],worlds=40)
                first=store.estimate(query)
                self.assertEqual(decide.call_args.kwargs['n'],40)
                self.assertEqual(decide.call_args.kwargs['review'],'off')
                self.assertIsNot(decide.call_args.kwargs['session'],store.session)
                store.implementation['frontend']='presentation change'
                self.assertEqual(store.estimate(query),first);self.assertEqual(decide.call_count,2)
                closer=store.estimate({**query,'worlds':160})
                self.assertNotEqual(first['id'],closer['id']);self.assertEqual(decide.call_args.kwargs['n'],160)
                self.assertEqual(decide.call_args.kwargs['budget_ms'],20000)
                self.assertEqual(store.receipt(original['id']),original)
                path=Path(tmp)/'estimates'/(first['id']+'.json');bad=deepcopy(first);bad['response']['choice']+=1;gym.atomic(path,bad)
                with self.assertRaisesRegex(ValueError,'contents changed'):store.estimate(query)
            finally:store.close()

    def test_inspection_rejects_hidden_inputs_and_concurrency_but_does_not_lock_live_play(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_bridge.decide',side_effect=self.fake) as decide:
            store=Store(tmp)
            try:
                body,_=self.body();query=dict(request=body['request'],worlds=40)
                for worlds in (True,0,41,100000):
                    with self.assertRaises(ValueError):store.estimate({**query,'worlds':worlds})
                for extra in ('hands','worlds','teacher','dealt'):
                    with self.assertRaises(ValueError):store.estimate({**query,'request':{**query['request'],extra:[]}})
                with self.assertRaises(ValueError):store.estimate({**query,'request':{**query['request'],'bid':29}})
                decide.assert_not_called()
                with store.estimate_lock:
                    with self.assertRaisesRegex(ValueError,'another move'):store.estimate(query)
                    store.decision(body)
                self.assertEqual(decide.call_count,1)
            finally:store.close()

    def test_inspection_timeout_is_retryable_and_closes_its_worker(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_bridge.decide',side_effect=self.fake) as decide:
            store=Store(tmp)
            try:
                body,_=self.body();query=dict(request=body['request'],worlds=160)
                with patch('plunge_bridge.DecisionSession') as session:
                    store.estimate(query);store.estimate(query)
                    self.assertEqual(decide.call_count,2)
                    self.assertEqual(session.return_value.__exit__.call_count,2)
                self.assertFalse(list((Path(tmp)/'estimates').glob('*.json')))
            finally:store.close()

    def test_finished_comparison_is_a_census_and_resumes_without_redeciding(self):
        with tempfile.TemporaryDirectory() as tmp,patch('sunshine_worlds.decide',side_effect=self.fake) as decide,patch('builtins.print'):
            store=Store(tmp)
            try:
                _,flag=self.body();value=store.flag(flag);path=Path(tmp)/'flags'/(value['id']+'.json');out=Path(tmp)/'audit'
                analyze(path,out,'l1',50)
                result=c.read(out/'result.json');self.assertEqual(result['status'],'complete')
                self.assertEqual(result['coverage'],'census');self.assertGreater(result['support'],0)
                self.assertEqual({a['tile'] for a in result['actions']},set(information_state(value['request'])['legal']))
                calls=decide.call_count;analyze(path,out,'l1',50);self.assertEqual(decide.call_count,calls)
                with self.assertRaisesRegex(ValueError,'identity changed'):analyze(path,out,'partner-l2',50)
            finally:store.close()

    def test_large_support_is_saved_without_enumerating_the_gym(self):
        with tempfile.TemporaryDirectory() as tmp,patch('plunge_analysis.Evaluator') as evaluator,patch('builtins.print'):
            store=Store(tmp)
            try:
                _,flag=self.body(0);value=store.flag(flag);out=Path(tmp)/'audit'
                analyze(Path(tmp)/'flags'/(value['id']+'.json'),out,'l1',50)
                self.assertEqual(c.read(out/'result.json')['status'],'outside-scope');evaluator.assert_not_called()
            finally:store.close()

    def test_analysis_reuse_tracks_the_actual_players_but_not_presentation(self):
        from plunge_analysis import configurations
        with tempfile.TemporaryDirectory() as tmp:
            store=Store(tmp)
            try:
                _,body=self.body();flag=store.flag(body);before=store.analysis_dir(flag['id'],'l1')
                store.implementation['frontend']='new interface'
                self.assertEqual(store.analysis_dir(flag['id'],'l1'),before)
                def changed(settings):
                    players=configurations(settings);players['partner']['n']=41;return players
                with patch('plunge_analysis.configurations',side_effect=changed):
                    self.assertNotEqual(store.analysis_dir(flag['id'],'l1'),before)
            finally:store.close()

    def test_override_rule_preserves_ties_small_samples_and_census_evidence(self):
        self.assertEqual(choice(2,{1:8,2:8},16,200,1),2)
        self.assertEqual(choice(2,{1:7,2:1},7,200,1),2)
        self.assertEqual(choice(2,{1:3,2:2},3,3,3),1)
        self.assertEqual(choice(2,{1:9,2:8},16,200,2),2)


if __name__=='__main__':unittest.main()
