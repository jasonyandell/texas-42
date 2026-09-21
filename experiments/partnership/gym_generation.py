"""One scenario pipeline: coordinates -> Scheme matches + values -> collection.

Matching and valuation have independent identities. Source membership, query
thresholds and selection filters never change an already applicable value key.
"""
from collections import Counter
from fractions import Fraction
import json
import os
from pathlib import Path
import subprocess
import time
from types import SimpleNamespace

import gym
import gym_spec as spec


def read(path):
    return json.loads(path.read_text())


def identities(arguments):
    evaluation = arguments['evaluation']
    engine = gym.file_hash(gym.ENGINE)
    common = dict(gym=gym.file_hash(gym.__file__), rules=gym.file_hash(gym.HERE/'rules.py'),
                  generator=gym.file_hash(__file__))
    value = dict(schema='gym-value-cache-v1', evaluation=evaluation, implementation=common)
    if evaluation['contract'] == 'deployed-gym-v1':
        import campaign
        import gym_deployed
        value.update(players=gym_deployed.configurations(evaluation), native=campaign.identities(),
                     deployed=gym.file_hash(gym_deployed.__file__),
                     decision_cache=gym.file_hash(gym.HERE/'sunshine_worlds.py'),
                     reconstruction=gym.file_hash(gym.HERE/'gym_replay.py'), rayon_threads=2)
    else:
        value['engine'] = engine
    match = dict(schema='gym-match-cache-v1', engine=engine, implementation=common,
                 source=arguments['query']['source'], max_worlds=evaluation['max_worlds'])
    return value, match


def targets(matched, threshold):
    return [tile for tile, mass in matched['presence']
            if Fraction(mass, matched['worlds']) >= Fraction(threshold)]


def generate(args):
    if not (1 <= args.workers <= 10 and 20 <= args.seconds <= 270 and 14 <= args.case_seconds < args.seconds):
        raise ValueError('workers 1..10, seconds 20..270, case-seconds >=14 and <seconds; use the watchdog')
    started = time.monotonic()
    definition, a, expected = spec.load(args.spec, args.overrides)
    paths, source_hash = spec.inputs(a)
    if expected is not None and source_hash != expected['source_sha256']:
        raise ValueError('reference source changed; create a new specification/reference for the expanded corpus')
    items = list(spec.coordinates(a, paths))
    if not items:
        raise ValueError('no eligible recorded coordinates in source')
    for item in items:
        assert item['id'] == gym.digest(gym.pupil_request(item['request']))[:20]
    assert len({item['id'] for item in items}) == len(items)
    value_identity, match_identity = identities(a)
    evaluation_id, match_id = gym.digest(value_identity), gym.digest(match_identity)
    view_id = gym.digest(dict(evaluation=evaluation_id, matching=match_id, arguments=a,
                             source_sha256=source_hash, coordinates=items,
                             specification_sha256=gym.digest(definition)))
    values = args.output/'evaluations'/evaluation_id
    matches = args.output/'matches'/match_id
    view = args.output/'collections'/view_id
    raw = args.output/'views'/view_id
    resolved = dict(schema='gym-resolved-spec-v2', name=definition['name'], arguments=a,
                    specification_sha256=gym.digest(definition), reference=expected,
                    source_sha256=source_hash, evaluation_id=evaluation_id, match_id=match_id,
                    collection_id=view_id)
    receipt = dict(schema='gym-generation-receipt-v2', complete=False, resolved=resolved,
                   evaluation=str(values.resolve()), matching=str(matches.resolve()),
                   discovery=str(raw.resolve()), gallery=None, planned=len(items))

    def finish(**extra):
        receipt.update(extra, elapsed_seconds=round(time.monotonic()-started, 6))
        gym.atomic(args.output/'latest.json', receipt)
        print(gym.canonical(receipt), flush=True)

    with gym.run_lock(args.output):
        gym.atomic(args.output/'latest.json', receipt)
        gym.pin(values, value_identity)
        gym.pin(matches, match_identity)
        snapshot = matches/'query.scheme'
        if snapshot.exists() and snapshot.read_text() != a['query']['source']:
            raise ValueError('frozen query snapshot changed')
        if not snapshot.exists():
            # A stop during query publication must not strand a truncated
            # snapshot that would refuse the otherwise valid resumable cache.
            temporary = snapshot.with_suffix('.tmp')
            with temporary.open('w') as out:
                out.write(a['query']['source'])
                out.flush()
                os.fsync(out.fileno())
            temporary.replace(snapshot)
        checked = subprocess.run([str(gym.ENGINE), '--check-query', str(snapshot)],
                                 capture_output=True, text=True, timeout=3)
        if checked.returncode:
            raise ValueError(checked.stderr.strip())
        print(gym.canonical(dict(planned=len(items), evaluation_id=evaluation_id, match_id=match_id)), flush=True)

        def match(item):
            req = item['request']
            info = gym.native(req, inspect=True, seconds=3)
            base = dict(id=item['id'], request=req, worlds=info['worlds'])
            if info['worlds'] > a['evaluation']['max_worlds']:
                return {**base, 'skipped': 'world cap'}
            found = gym.native(req, inspect=True, query=snapshot,
                               max_worlds=a['evaluation']['max_worlds'], seconds=5)['query_match']
            assert found['worlds'] == info['worlds']
            return {**base, 'query_match': found}

        stage = gym.bounded(items, match, matches, args.workers,
                            args.seconds-(time.monotonic()-started), 8)
        saved = {item['id']: read(matches/'items'/(item['id']+'.json')) for item in items
                 if (matches/'items'/(item['id']+'.json')).exists()}
        needed = []
        for item in items:
            row = saved.get(item['id'])
            if row is None: continue
            assert row['request'] == item['request']
            if 'query_match' in row and targets(row['query_match'], a['query']['min_presence']):
                needed.append(item)
        before = sum((values/'items'/(i['id']+'.json')).exists() for i in needed)
        receipt.update(matched=len(needed), reused_values=before, matching= str(matches.resolve()))
        if len(saved) < len(items) or stage['stopped']:
            finish(pending=len(items)-len(saved), stopped=stage['stopped'], phase='matching')
            return

        remaining = args.seconds-(time.monotonic()-started)
        if a['evaluation']['contract'] == 'deployed-gym-v1':
            from gym_deployed import Evaluator
            stage = Evaluator(values, a['evaluation']).run(needed, args.workers, remaining, args.case_seconds)
        else:
            def evaluate(item):
                key = gym.native(item['request'], max_worlds=a['evaluation']['max_worlds'],
                                 partner_worlds=a['evaluation']['partner_worlds'], seconds=args.case_seconds)
                case = dict(id=item['id'], request=item['request'], key=key, semantics=gym.SEMANTICS)
                gym.verify(case)
                return case
            stage = gym.bounded(needed, evaluate, values, args.workers, remaining, args.case_seconds)
        pending = [i for i in needed if not (values/'items'/(i['id']+'.json')).exists()]
        receipt.update(new_values=len(needed)-len(pending)-before)
        if pending or stage['stopped']:
            finish(pending=len(pending), stopped=stage['stopped'], phase='evaluation')
            return
        if spec.inputs(a)[1] != source_hash:
            raise ValueError('source changed during evaluation; rerun against a stable corpus')

        manifest = dict(schema='gym-discovery-v2', engine=gym.file_hash(gym.ENGINE),
                        runner=gym.file_hash(gym.__file__), rules=gym.file_hash(gym.HERE/'rules.py'),
                        candidates=items, generator=resolved, value_identity=value_identity,
                        query_source=a['query']['source'], query_name=a['query']['name'])
        gym.pin(raw, manifest)
        skips = Counter()
        for item in items:
            row = saved[item['id']]
            case = {**row, **item, 'family': a['query']['name'], 'min_presence': a['query']['min_presence']}
            if 'query_match' in row:
                case['target_actions'] = targets(row['query_match'], a['query']['min_presence'])
                if not case['target_actions']: case['skipped'] = 'no query match'
                else:
                    value = read(values/'items'/(item['id']+'.json'))
                    assert value['request'] == item['request'] and value['id'] == item['id']
                    case.update(key=value['key'], semantics=value['semantics'])
                    case['categories'] = gym.classify(case['key'], case['target_actions'])
            if 'skipped' in case: skips[case['skipped']] += 1
            gym.atomic(raw/'items'/(item['id']+'.json'), case)
        gym.select(SimpleNamespace(source=raw, output=view, criterion=a['selection']['criterion'],
                    side=a['selection']['side'], all=True, each=1),
                   predicate=lambda case: spec.accepts(case, a['selection']), allow_empty=True)
        result = spec.result_identity([case for _, case in gym.gallery(view)])
        if expected is not None and any(result[k] != expected[k] for k in result):
            finish(result=result, error='reference result mismatch')
            raise ValueError('generated collection differs from the frozen reference; see latest.json')
        gym.atomic(view/'specification.json', resolved)
        finish(complete=True, gallery=str(view.resolve()), result=result, pending=0,
               reference_verified=expected is not None, skipped=dict(skips), phase='complete')
