#!/usr/bin/env python3
"""Exhaustive selected-gym action replay with frozen deployed continuations.

The examiner may force a root action. Every subsequent player receives only its
own original hand and public history. Root interventions and deployed choices
are recorded separately; no teacher label is an input to a player.
"""
import argparse
from collections import Counter
from dataclasses import asdict
from fractions import Fraction
import json
import os
from pathlib import Path
import threading

import campaign as c
import gym
from matchup import Player
from partner_review import offers
from rules import information_state, replay_record
from runtime import DecisionSession
from sunshine_worlds import Experiment as ConditionalExperiment

HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / 'campaigns/sunshine-partner-count-v1'


def normalized(value):
    return json.loads(gym.canonical(value))


def full_hands(request, world):
    hands = [set(h) for h in world]
    for seat, tile in zip(request['plays'][::2], request['plays'][1::2]):
        assert tile not in hands[seat]
        hands[seat].add(tile)
    full = [sorted(h) for h in hands]
    assert full[request['seat']] == sorted(request['hand'])
    _, lead, remaining, trick = replay_record(full, request['plays'], request['decl'], request['bidder'])
    assert gym.world_key(remaining) == gym.world_key(world)
    assert (lead + len(trick)) % 4 == request['seat']
    return full


def make_panel(composed, broad):
    """Selection depends only on the previously recorded diagnostic reports."""
    galleries = {'miss': dict(gym.gallery(composed)), 'withhold': dict(gym.gallery(broad))}
    narrow_report = c.read(EVIDENCE / 'gym-30.json')
    broad_report = c.read(EVIDENCE / 'gym-broad.json')
    selected = [('miss', r) for r in narrow_report['rows'] if not r['baseline']['optimal']]
    selected += [('certain', r) for r in narrow_report['rows'] if r['scenario'] == 'advantage-09']
    for row in broad_report['rows']:
        review = row['review']
        if review['status'] == 'retained':
            values = dict(review['values'])
            if values[review['baseline']] > max(values[a] for a in review['offers']):
                selected.append(('withhold', row))
    assert Counter(group for group, _ in selected) == {'miss': 6, 'certain': 1, 'withhold': 38}
    roots = []
    seen = set()
    for group, previous in selected:
        case = galleries['miss' if group == 'certain' else group][previous['scenario']]
        req = gym.pupil_request(case['request'])
        assert req == previous['request']
        identity = c.digest(req)
        assert identity not in seen, 'overlapping roots must not be counted twice'
        seen.add(identity)
        worlds = sorted(gym.compatible_worlds(req))
        assert 0 < len(worlds) == case['key']['worlds'] <= 400
        legal = information_state(req)['legal']
        targets = offers(req, information_state(req))
        assert targets == case['target_actions'] and targets
        assert req['seat'] % 2 == req['bidder'] % 2
        values, outcomes = {}, {}
        for action in case['key']['actions']:
            traces = {gym.world_key(t['hands']): t for t in action['traces']}
            assert len(traces) == len(action['traces']) == len(worlds)
            assert set(traces) == set(worlds)
            bits = []
            for world in worlds:
                trace = traces[world]
                hands = full_hands(req, world)
                record = req['plays'] + trace['plays']
                assert trace['plays'][:2] == [req['seat'], action['tile']]
                points, _, remaining, trick = replay_record(hands, record, req['decl'], req['bidder'])
                assert len(record) == 56 and not trick and not any(remaining)
                assert points == trace['banked'] and sum(points) == 42
                made = points[req['bidder'] % 2] >= 30
                assert made == trace['success']
                bits.append('1' if made else '0')
            values[action['tile']] = sum(b == '1' for b in bits)
            assert values[action['tile']] == action['success_mass']
            outcomes[action['tile']] = ''.join(bits)
        assert sorted(values) == legal
        offer = min(targets, key=lambda a: (-values[a], a))
        roots.append(dict(id=f'{group}-{previous["scenario"]}', group=group,
                          scenario=previous['scenario'], request=req, source=case['source'],
                          legal=legal, offers=targets, teacher_offer=offer,
                          teacher_field=case['key']['field_id'], teacher_values=values,
                          teacher_outcomes=outcomes, worlds=worlds,
                          previous=previous, case_sha256=c.digest(case)))
    return normalized(dict(schema='sunshine-gym-replay-panel-v1', roots=roots,
                           sources={str(EVIDENCE / name): gym.file_hash(EVIDENCE / name)
                                    for name in ('gym-30.json', 'gym-broad.json')}))


def paired(before, after):
    assert len(before) == len(after) and before
    changes = [int(b) - int(a) for a, b in zip(before, after)]
    counts = Counter(changes)
    return dict(before=sum(before), after=sum(after), worlds=len(before),
                wins=counts[1], losses=counts[-1], ties=counts[0],
                delta=str(Fraction(sum(changes), len(changes))))


class ReplayExperiment(ConditionalExperiment):
    """Reuse the existing exact-input decision cache; supply a new fixed panel.

    Only `decision` is inherited. This class owns selection, job identity,
    root interventions, continuation, and independent result auditing.
    """
    def __init__(self, panel, output):
        self.output = output
        self.configs = [Player('l1-default'), Player('l1-partner-count-review', review='partner-count')]
        self.lock = threading.Lock()
        self.locks = {}
        self.panel = c.read(panel)
        assert self.panel['schema'] == 'sunshine-gym-replay-panel-v1'
        self.manifest = normalized(dict(
            schema='sunshine-gym-replay-v1', panel_sha256=gym.file_hash(panel), panel=self.panel,
            identities=c.identities(), players=[asdict(p) for p in self.configs],
            sources={name: gym.file_hash(HERE / name) for name in
                     ('gym_replay.py', 'sunshine_worlds.py', 'gym.py', 'runtime.py')},
            policy='first realized exact own/public request + full configuration; frozen on disk',
            prior='entire uniform mechanical support; no pre-root behavioral weighting',
            interventions='all legal root actions then ordinary L1; reviewed root then reviewed declaring team',
            native_threads=2))
        gym.pin(output, self.manifest)

    def jobs(self):
        jobs = []
        for i, root in enumerate(self.panel['roots']):
            for world in range(len(root['worlds'])):
                for action in root['legal']:
                    jobs.append(dict(id=f'{i:02d}-{world:03d}-a{action:02d}', root=i,
                                     world=world, kind='action', action=action))
                jobs.append(dict(id=f'{i:02d}-{world:03d}-review', root=i,
                                 world=world, kind='review', action=None))
        return sorted(jobs, key=lambda j: (j['world'], j['root'], j['id']))

    def root_decisions(self, root, session):
        return [self.decision(root['request'], config, session) for config in self.configs]

    def config(self, kind, seat, bidder):
        return self.configs[int(kind == 'review' and seat % 2 == bidder % 2)]

    def play(self, job):
        root = self.panel['roots'][job['root']]
        req = root['request']
        hands = full_hands(req, root['worlds'][job['world']])
        with DecisionSession() as session:
            decisions = self.root_decisions(root, session)
            action = job['action'] if job['kind'] == 'action' else decisions[1][0]['choice']
            assert action in root['legal']
            record = req['plays'] + [req['seat'], action]
            ids = []
            while len(record) < 56:
                _, lead, _, trick = replay_record(hands, record, req['decl'], req['bidder'])
                seat = (lead + len(trick)) % 4
                request = dict(decl=req['decl'], bid=30, bidder=req['bidder'], seat=seat,
                               hand=hands[seat], plays=record.copy(), seed=req['seed'])
                response, key = self.decision(request, self.config(job['kind'], seat, req['bidder']), session)
                record.extend([seat, response['choice']])
                ids.append(key)
        points, _, remaining, trick = replay_record(hands, record, req['decl'], req['bidder'])
        assert sum(points) == 42 and not trick and not any(remaining)
        payload = dict(**job, action_taken=action, root_decisions=[key for _, key in decisions],
                       hands=hands, record=record, decisions=ids, points=points,
                       made=points[req['bidder'] % 2] >= 30)
        return dict(payload=payload, payload_sha256=c.digest(payload))

    def checked_decision(self, key, request, config):
        path = self.output / 'decisions' / (key + '.json')
        decision = c.read(path)
        assert decision is not None
        assert decision['identity'] == dict(request=request, player=asdict(config))
        assert c.digest(decision['identity']) == key
        assert c.digest(decision['response']) == decision['response_sha256']
        response = decision['response']
        assert not response['over_budget']
        assert response['choice'] in information_state(request)['legal']
        return response

    def audit_job(self, job, saved):
        row = saved['payload']
        assert c.digest(row) == saved['payload_sha256']
        assert all(row[k] == value for k, value in job.items())
        root = self.panel['roots'][job['root']]
        req = root['request']
        assert row['hands'] == full_hands(req, root['worlds'][job['world']])
        assert len(row['root_decisions']) == 2
        roots = [self.checked_decision(key, req, config)
                 for key, config in zip(row['root_decisions'], self.configs)]
        action = job['action'] if job['kind'] == 'action' else roots[1]['choice']
        assert row['action_taken'] == action and action in root['legal']
        prefix = req['plays'] + [req['seat'], action]
        assert row['record'][:len(prefix)] == prefix
        assert len(row['record']) == 56
        assert len(row['decisions']) == (56 - len(prefix)) // 2
        for n, key in enumerate(row['decisions']):
            ply = len(prefix) // 2 + n
            seat, tile = row['record'][2 * ply:2 * ply + 2]
            request = dict(decl=req['decl'], bid=30, bidder=req['bidder'], seat=seat,
                           hand=row['hands'][seat], plays=row['record'][:2 * ply], seed=req['seed'])
            response = self.checked_decision(key, request, self.config(job['kind'], seat, req['bidder']))
            assert response['choice'] == tile
        points, _, remaining, trick = replay_record(row['hands'], row['record'], req['decl'], req['bidder'])
        assert not trick and not any(remaining) and sum(points) == 42
        assert points == row['points'] and type(row['made']) is bool
        assert row['made'] == (points[req['bidder'] % 2] >= 30)
        return row, roots

    def report(self):
        jobs = self.jobs()
        if any(not (self.output / 'items' / (j['id'] + '.json')).exists() for j in jobs):
            return None
        rows, roots_by_index, hashes, used = {}, {}, {}, set()
        for job in jobs:
            path = self.output / 'items' / (job['id'] + '.json')
            row, root_decisions = self.audit_job(job, c.read(path))
            key = (job['root'], job['world'], job['kind'], job['action'])
            assert key not in rows
            rows[key] = row
            assert roots_by_index.setdefault(job['root'], root_decisions) == root_decisions
            used.update(row['decisions'] + row['root_decisions'])
            hashes[str(path)] = gym.file_hash(path)
        summaries, witnesses = [], []
        for i, root in enumerate(self.panel['roots']):
            count = len(root['worlds'])
            outcomes = {a: [rows[i, w, 'action', a]['made'] for w in range(count)] for a in root['legal']}
            masses = {a: sum(bits) for a, bits in outcomes.items()}
            baseline, reviewed = roots_by_index[i]
            b, r, offer = baseline['choice'], reviewed['choice'], root['teacher_offer']
            assert b == root['previous']['baseline']['choice'], 'frozen baseline must reproduce original miss/control'
            full = [rows[i, w, 'review', None]['made'] for w in range(count)]
            comparisons = dict(teacher_offer=paired(outcomes[b], outcomes[offer]),
                               root_review=paired(outcomes[b], outcomes[r]),
                               whole_review=paired(outcomes[b], full))
            summaries.append(dict(id=root['id'], group=root['group'], source=root['source'],
                                  worlds=count, baseline_choice=b, reviewed_choice=r, teacher_offer=offer,
                                  teacher_values=root['teacher_values'], deployed_values=masses,
                                  deployed_best=[a for a in root['legal'] if masses[a] == max(masses.values())],
                                  teacher_offer_is_deployed_best=masses[offer] == max(masses.values()),
                                  baseline_regret=str(Fraction(max(masses.values()) - masses[b], count)),
                                  root_review_regret=str(Fraction(max(masses.values()) - masses[r], count)),
                                  review=reviewed.get('review_result'), baseline_us=baseline['elapsed_us'],
                                  reviewed_us=reviewed['elapsed_us'], comparisons=comparisons,
                                  teacher_disagreements={a:sum((bit == '1') != outcome for bit, outcome in
                                      zip(root['teacher_outcomes'][str(a)], outcomes[a])) for a in root['legal']}))
            for sign, label in ((1, 'gain'), (-1, 'loss')):
                found = next((w for w in range(count) if int(outcomes[offer][w]) - int(outcomes[b][w]) == sign), None)
                if found is not None:
                    before, after = rows[i, found, 'action', b], rows[i, found, 'action', offer]
                    witnesses.append(dict(root=root['id'], kind=label, world=found, request=root['request'],
                                          hands=before['hands'], baseline=before, offer=after))
        aggregates = []
        for group in ('miss', 'withhold', 'certain'):
            cases = [row for row in summaries if row['group'] == group]
            aggregates.append(dict(group=group, roots=len(cases), worlds=sum(r['worlds'] for r in cases),
                root_changes=sum(r['baseline_choice'] != r['reviewed_choice'] for r in cases),
                comparisons={name:dict(
                    wins=sum(r['comparisons'][name]['wins'] for r in cases),
                    losses=sum(r['comparisons'][name]['losses'] for r in cases),
                    ties=sum(r['comparisons'][name]['ties'] for r in cases),
                    mean_root_delta=str(sum((Fraction(r['comparisons'][name]['delta']) for r in cases), Fraction()) / len(cases)),
                    positive_roots=sum(Fraction(r['comparisons'][name]['delta']) > 0 for r in cases),
                    negative_roots=sum(Fraction(r['comparisons'][name]['delta']) < 0 for r in cases))
                    for name in ('teacher_offer', 'root_review', 'whole_review')}))
        for key in used:
            path = self.output / 'decisions' / (key + '.json')
            hashes[str(path)] = gym.file_hash(path)
        result = normalized(dict(schema='sunshine-gym-replay-report-v1', manifest=self.manifest,
            groups=aggregates, roots=summaries, witnesses=witnesses,
            full_game_replays=len(rows), unique_decisions=len(used),
            decision_uses=sum(len(row['decisions']) for row in rows.values()),
            artifact_hashes=hashes,
            scope='Full uniform mechanical support of selected roots; frozen timed deployed players; no population-strength or live-latency inference.'))
        c.atomic(self.output / 'report.json', result)
        return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    prepare = commands.add_parser('prepare')
    prepare.add_argument('--composed', type=Path, required=True)
    prepare.add_argument('--broad', type=Path, required=True)
    prepare.add_argument('--output', type=Path, required=True)
    for name in ('run', 'report'):
        command = commands.add_parser(name)
        command.add_argument('panel', type=Path)
        command.add_argument('output', type=Path)
        if name == 'run':
            command.add_argument('--workers', type=int, default=10)
            command.add_argument('--seconds', type=int, default=240)
    args = parser.parse_args()
    if args.command == 'prepare':
        if args.output.exists(): raise ValueError('prepare requires a new output file')
        panel = make_panel(args.composed, args.broad)
        c.atomic(args.output, panel)
        print(json.dumps(dict(roots=len(panel['roots']), worlds=sum(len(r['worlds']) for r in panel['roots']))))
        return
    os.environ['WALT_RAYON_THREADS'] = '2'
    with gym.run_lock(args.output):
        experiment = ReplayExperiment(args.panel, args.output)
        if args.command == 'run':
            if not 1 <= args.workers <= 10 or not 150 <= args.seconds <= 260:
                raise ValueError('workers must be 1..10 and seconds 150..260')
            gym.bounded(experiment.jobs(), experiment.play, args.output, args.workers, args.seconds, 140)
        report = experiment.report()
        if report is None:
            print('incomplete: resume unchanged')
            raise SystemExit(75)
        print(json.dumps({k: report[k] for k in ('groups', 'full_game_replays', 'unique_decisions', 'decision_uses')}, indent=2))


if __name__ == '__main__':
    main()
