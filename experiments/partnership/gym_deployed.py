"""Selectable, frozen deployed-player continuation for the ordinary gym.

All actions share the same full support or sampled subset. Action values do not
depend on which Scheme expression proposed them. Player inputs remain own/public.
"""
from collections import Counter
from dataclasses import asdict
from fractions import Fraction
import json
import os
from pathlib import Path
import random
import threading
import time

import campaign as c
import gym
from gym_replay import full_hands
from matchup import Player
from rules import TILES, information_state, replay_record, trick_points, winner
from runtime import DecisionSession
from sunshine_worlds import Experiment as DecisionCache

CONTRACT = 'deployed-gym-v1'
SEMANTICS = dict(belief='Uniform mechanical root support; no pre-root behavioral weighting.',
    field='Named deployed players, pinned configurations and binaries; first exact-input decision frozen.',
    continuation='Forced root action followed by the named deployed focal, partner and opponent policies.',
    objective='Viewer team makes or sets bid 30; equal success fractions tie.',
    grading='Best assessed first action under the frozen continuation; coverage is explicit in each key.',
    count='Diagnostic only; never a score tie-break.')


def validate_evaluation(e):
    from gym_spec import fields, integer
    fields(e, 'contract max_worlds players sample_worlds sample_seed', 'evaluation')
    fields(e['players'], 'focal partner opponents', 'evaluation.players')
    catalog = c.read(gym.HERE / 'players.json')
    for name in e['players'].values():
        if not isinstance(name, str) or name not in catalog:
            raise ValueError('evaluation players must name existing presets')
        if Player(**catalog[name]).mode == 'phone':
            raise ValueError('deployed gym currently supports native presets')
    if e['sample_worlds'] is not None:
        integer(e['sample_worlds'], 1, 10000, 'evaluation.sample_worlds')
    integer(e['sample_seed'], 0, 2**64 - 1, 'evaluation.sample_seed')


def configurations(e):
    catalog = c.read(gym.HERE / 'players.json')
    return {role: asdict(Player(**catalog[name])) for role, name in e['players'].items()}


def role_config(players, viewer, seat):
    return Player(**players['focal' if seat == viewer else 'partner' if seat == viewer ^ 2 else 'opponents'])


def selected_worlds(req, worlds, settings):
    n = settings['sample_worlds']
    if n is None or n >= len(worlds):
        return worlds
    seed = settings['sample_seed'] ^ int(gym.digest(req)[:16], 16)
    indices = sorted(random.Random(seed).sample(range(len(worlds)), n))
    return [worlds[i] for i in indices]


class Evaluator(DecisionCache):
    """Inherit only the already-tested exact-input persistent decision cache."""
    def __init__(self, output, settings):
        self.output = output
        self.settings = settings
        self.players = configurations(settings)
        self.lock, self.locks = threading.Lock(), {}
        self.plans = {}
        os.environ['WALT_RAYON_THREADS'] = '2'

    def run(self, items, workers, seconds, case_seconds):
        jobs = []
        for item in items:
            path = self.output / 'items' / (item['id'] + '.json')
            if path.exists(): continue
            req = gym.pupil_request(item['request'])
            worlds = sorted(gym.compatible_worlds(req))
            assert 0 < len(worlds) <= self.settings['max_worlds']
            selected = selected_worlds(req, worlds, self.settings)
            self.plans[item['id']] = dict(request=req, worlds=selected, support=len(worlds))
            for i in range(len(selected)):
                for action in information_state(req)['legal']:
                    jobs.append(dict(id=f'{item["id"]}-{i:04d}-{action:02d}', root=item['id'], world=i, action=action))
        jobs.sort(key=lambda j: (j['world'], j['root'], j['action']))
        state = gym.bounded(jobs, lambda j: self.play(j, case_seconds), self.output / 'trajectories',
                            workers, seconds, case_seconds)
        for item in items:
            if item['id'] not in self.plans: continue
            mine = [j for j in jobs if j['root'] == item['id']]
            if all((self.output / 'trajectories/items' / (j['id'] + '.json')).exists() for j in mine):
                key = self.assemble(item['id'], mine)
                value = dict(id=item['id'], request=item['request'], key=key, semantics=SEMANTICS)
                verify(value, query=False)
                gym.atomic(self.output / 'items' / (item['id'] + '.json'), value)
        return state

    def play(self, job, seconds):
        deadline = time.monotonic() + seconds
        plan = self.plans[job['root']]
        req, world = plan['request'], plan['worlds'][job['world']]
        hands = full_hands(req, world)
        record, decisions = req['plays'] + [req['seat'], job['action']], []
        with DecisionSession() as session:
            while len(record) < 56:
                if deadline - time.monotonic() < 15:
                    raise RuntimeError('trajectory allowance exhausted; saved decisions survive resume')
                _, lead, _, trick = replay_record(hands, record, req['decl'], req['bidder'])
                seat = (lead + len(trick)) % 4
                request = dict(decl=req['decl'], bid=30, bidder=req['bidder'], seat=seat,
                               hand=hands[seat], plays=record.copy(), seed=req['seed'])
                response, key = self.decision(request, role_config(self.players, req['seat'], seat), session)
                record.extend([seat, response['choice']])
                decisions.append(key)
        points, _, remaining, trick = replay_record(hands, record, req['decl'], req['bidder'])
        assert not trick and not any(remaining) and sum(points) == 42
        payload = dict(**job, hands=world, plays=record[len(req['plays']):], decisions=decisions,
                       banked=points, success=(points[req['bidder'] % 2] >= 30) == (req['seat'] % 2 == req['bidder'] % 2))
        return dict(payload=payload, sha256=gym.digest(payload))

    def assemble(self, rid, jobs):
        plan = self.plans[rid]
        req = plan['request']
        state = information_state(req)
        actions, table = [], {}
        n = len(req['plays']) // 2 % 4
        prefix = list(zip(req['plays'][-2*n::2], req['plays'][-2*n+1::2])) if n else []
        mechanical_offers = [a for a in state['legal'] if sum(TILES[a]) in (5,10) and n >= 2
                             and winner(prefix,req['decl']) == req['seat'] ^ 2
                             and winner(prefix+[(req['seat'],a)],req['decl']) == req['seat'] ^ 2]
        for action in state['legal']:
            traces = []
            for job in sorted((j for j in jobs if j['action'] == action), key=lambda j:j['world']):
                saved = c.read(self.output / 'trajectories/items' / (job['id'] + '.json'))
                row = saved['payload']
                assert gym.digest(row) == saved['sha256'] and all(row[k] == v for k,v in job.items())
                assert gym.world_key(row['hands']) == gym.world_key(plan['worlds'][job['world']])
                for key in row['decisions']:
                    value = c.read(self.output / 'decisions' / (key + '.json'))
                    assert gym.digest(value['identity']) == key
                    assert gym.digest(value['response']) == value['response_sha256']
                    response = value['response']
                    assert not response['over_budget']
                    table[key] = dict(identity=value['identity'], choice=response['choice'], route=response['route'])
                trick, count = prefix.copy(), 0
                for seat,tile in zip(row['plays'][::2],row['plays'][1::2]):
                    trick.append((seat,tile))
                    if len(trick)==4:
                        if winner(trick,req['decl']) == req['seat'] ^ 2: count += trick_points(trick)-1
                        trick=[]
                traces.append({k:row[k] for k in ('hands','plays','decisions','banked','success')} | {'partner_count':count})
            bins = Counter(t['banked'][req['bidder'] % 2] for t in traces)
            actions.append(dict(tile=action, traces=traces, success_mass=sum(t['success'] for t in traces),
                                score_bins=[bins[i] for i in range(43)]))
        best = max(a['success_mass'] for a in actions)
        return dict(schema=CONTRACT, root_id=rid, legal=state['legal'], offers=mechanical_offers,
            leader=state['leader'], banked=state['points'], trick=state['trick'], prefix=[t for _,t in prefix],
            remaining=sorted(set(req['hand'])-set(req['plays'][1::2])),
            worlds=len(plan['worlds']), support_worlds=plan['support'],
            coverage='census' if len(plan['worlds'])==plan['support'] else 'sample-without-replacement',
            settings=self.settings, players=self.players, field_id=CONTRACT+'/'+gym.digest(self.players),
            best=[a['tile'] for a in actions if a['success_mass']==best], actions=actions, decisions=table)


def verify(case, query=True):
    req, key = gym.pupil_request(case['request']), case['key']
    assert case['semantics']==SEMANTICS and key['schema']==CONTRACT
    state=information_state(req)
    for name in ('legal','leader','trick'):
        assert key[name]==state[name]
    assert key['banked']==state['points']
    assert key['remaining']==sorted(set(req['hand'])-set(req['plays'][1::2]))
    n=len(req['plays'])//2%4
    prefix=list(zip(req['plays'][-2*n::2],req['plays'][-2*n+1::2])) if n else []
    assert key['prefix']==[tile for _,tile in prefix]
    assert key['root_id']==gym.digest(req)[:20]
    assert key['offers']==[a for a in state['legal'] if sum(TILES[a]) in (5,10) and n>=2
        and winner(prefix,req['decl'])==req['seat']^2
        and winner(prefix+[(req['seat'],a)],req['decl'])==req['seat']^2]
    assert key['field_id']==CONTRACT+'/'+gym.digest(key['players'])
    assert set(key['players'])=={'focal','partner','opponents'}
    for value in key['players'].values(): Player(**value)
    from gym_spec import fields, integer
    settings=key['settings']
    fields(settings,'contract max_worlds players sample_worlds sample_seed','frozen evaluation')
    assert settings['contract']==CONTRACT and set(settings['players'])==set(key['players'])
    integer(settings['max_worlds'],1,10000,'frozen cap')
    integer(settings['sample_seed'],0,2**64-1,'frozen sample seed')
    if settings['sample_worlds'] is not None:integer(settings['sample_worlds'],1,10000,'frozen sample size')
    worlds=sorted(gym.compatible_worlds(req))
    assert len(worlds)==key['support_worlds']
    assert len(worlds)<=settings['max_worlds']
    selected=selected_worlds(req,worlds,key['settings'])
    assert len(selected)==key['worlds']>0
    assert key['coverage']==('census' if len(selected)==len(worlds) else 'sample-without-replacement')
    assert sorted(a['tile'] for a in key['actions'])==state['legal']
    used=set()
    for action in key['actions']:
        assert len(action['traces'])==len(selected)
        assert [gym.world_key(t['hands']) for t in action['traces']]==selected
        bins, successes=Counter(),0
        for trace in action['traces']:
            hands=full_hands(req,trace['hands'])
            assert trace['plays'][:2]==[req['seat'],action['tile']]
            record=req['plays']+trace['plays']
            assert len(record)==56
            assert len(trace['decisions'])==len(trace['plays'])//2-1
            for n,identity in enumerate(trace['decisions']):
                ply=len(req['plays'])//2+1+n
                seat,tile=record[2*ply:2*ply+2]
                request=dict(decl=req['decl'],bid=30,bidder=req['bidder'],seat=seat,
                             hand=hands[seat],plays=record[:2*ply],seed=req['seed'])
                config=asdict(role_config(key['players'],req['seat'],seat))
                entry=key['decisions'][identity]
                assert entry['identity']==dict(request=request,player=config)
                assert gym.digest(entry['identity'])==identity and entry['choice']==tile
                used.add(identity)
            points,_,remaining,trick=replay_record(hands,record,req['decl'],req['bidder'])
            assert not any(remaining) and not trick and sum(points)==42 and points==trace['banked']
            success=(points[req['bidder']%2]>=30)==(req['seat']%2==req['bidder']%2)
            assert type(trace['success']) is bool and trace['success']==success
            successes+=success;bins[points[req['bidder']%2]]+=1
            count=0
            for start in range(len(req['plays'])//8*8,56,8):
                trick=list(zip(record[start:start+8:2],record[start+1:start+8:2]))
                if winner(trick,req['decl'])==req['seat']^2:count+=trick_points(trick)-1
            assert count==trace['partner_count']
        assert successes==action['success_mass'] and [bins[i] for i in range(43)]==action['score_bins']
    assert used==set(key['decisions'])
    best=max(a['success_mass'] for a in key['actions'])
    assert key['best']==[a['tile'] for a in key['actions'] if a['success_mass']==best]
    for name, expected in (('categories',lambda:gym.case_pairs(case)),('outcome',lambda:gym.outcome_profile(key)),
                           ('paired_outcomes',lambda:gym.paired_outcomes(key,case['pair'])),
                           ('query_contrast',lambda:gym.query_contrast(key,case['target_actions']))):
        if name in case: assert case[name]==expected()
    if 'pair' in case: assert case['pair'] in gym.case_pairs(case)
    if query and 'query_match' in case:
        import tempfile
        found=case['query_match']
        assert found['worlds']==len(worlds)
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'query.scheme';path.write_text(found['source'])
            assert gym.native(req,inspect=True,query=path,max_worlds=len(worlds),seconds=5)['query_match']==found
        assert case['target_actions']==[t for t,m in found['presence'] if Fraction(m,len(worlds))>=Fraction(case['min_presence'])]
    return dict(worlds=len(selected),support_worlds=len(worlds),coverage=key['coverage'],
                actions=len(key['actions']),information_sets=len(used))
