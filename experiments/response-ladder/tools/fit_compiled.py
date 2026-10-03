#!/usr/bin/env python3
"""Exact finite v1 grammar audit. Local lesson costs are not game regret."""
import argparse
from collections import Counter, defaultdict
from fractions import Fraction
import hashlib
from itertools import permutations
import json
import math
from pathlib import Path
import time


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def atomic(path, value):
    tmp = path.with_suffix(path.suffix + '.tmp')
    tmp.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')
    tmp.replace(path)


# Explicit finite teaching contracts. Changing a target or its prior is not a
# larger sample of the same teacher. Old compatible receipts remain readable.
TEACHER_CONTRACTS = {
    ('compatible-root-reset-v1', 't0-dice-exact-v1'):
        ('compatible-t0', 'public-void-compatible-shuffle-reject', 'request-seed-v1'),
    ('compatible-root-reset-v1', 't1-br-compiled-c0-v1'):
        ('compatible-t1', 'public-void-compatible-shuffle-reject', 'request-seed-v1'),
    ('historical-voidless-h0-v1', 'h0-native-dice-exact-v1'):
        ('historical-h0', 'historical-voidless', 'native-h0-seat-current-hand-record-v1'),
    ('compatible-root-h0-seed-v1', 't0-h0-seed-dice-exact-v1'):
        ('compatible-h0-seed', 'public-void-compatible-shuffle-reject', 'native-h0-seat-current-hand-record-v1'),
}


def teacher_contract(response):
    pair = response.get('target'), response.get('revision')
    expected = TEACHER_CONTRACTS.get(pair)
    if expected is None:
        raise ValueError('unrecognized teacher contract')
    metadata = dict(zip(('mode', 'prior', 'seed_rule'), expected))
    historical_seed = expected[2] == 'native-h0-seat-current-hand-record-v1'
    for key, value in metadata.items():
        if response.get(key, None if historical_seed else value) != value:
            raise ValueError('teacher mode/prior/seed identity disagrees with target')
    field = response.get('field_revision')
    if pair[1] == 't1-br-compiled-c0-v1':
        if not isinstance(field, str) or not field.startswith('compiled-field-v1/'):
            raise ValueError('compiled T1 requires its frozen C0 field identity')
    elif field != 'historical-dice-v1':
        raise ValueError('Dice teacher has a different field')
    if historical_seed and response.get('request', {}).get('n', 8) != 8:
        raise ValueError('native-H0 diagnostic requires fixed n=8')
    return dict(target=pair[0], revision=pair[1], field_revision=field, **metadata)


def read_rows(directory):
    rows, statuses, sources = [], Counter(), {}
    identities, seen, splits = set(), set(), defaultdict(set)
    for path in sorted((directory / 'lessons').glob('*.json')):
        item = json.loads(path.read_text())
        response = item.get('response', item)
        status = response.get('status', 'missing-status')
        if item.get('status', status) != status:
            raise ValueError('outer and nested lesson status disagree')
        if item['id'] in seen or item['split'] not in ('train', 'development'):
            raise ValueError('duplicate lesson or unknown split')
        seen.add(item['id'])
        splits[str(item['group'])].add(item['split'])
        statuses[status] += 1
        sources[str(path.relative_to(directory))] = digest(path)
        # Even censored identified labels cannot silently come from another
        # prior/field. Unidentified JSON parse errors remain censored receipts.
        if status != 'complete':
            if 'target' in response:
                contract = teacher_contract(response)
                identities.add((json.dumps(contract, sort_keys=True),
                                response.get('feature_schema', 'scheme-relational-actor-v1'),
                                response.get('request', {}).get('n', 8)))
            continue
        feature_schema = response.get('feature_schema', 'scheme-relational-actor-v1')
        clause_count = {'scheme-relational-actor-v1': 14, 'scheme-relational-actor-v2': 16}.get(feature_schema)
        expected_schema = 'compiled-teacher-v2' if clause_count == 16 else 'compiled-teacher-v1'
        if not clause_count or response.get('schema') != expected_schema:
            raise ValueError('unrecognized teacher contract')
        contract = teacher_contract(response)
        identities.add((json.dumps(contract, sort_keys=True), feature_schema, response['request'].get('n', 8)))
        if contract['seed_rule'] == 'native-h0-seat-current-hand-record-v1':
            bundle = response.get('bundle', {})
            if any(bundle.get(k) != v for k, v in contract.items()):
                raise ValueError('bundle teacher identity disagrees with lesson')
            if bundle.get('n') != 8 or bundle.get('mass') != 8 or response.get('mass') != 8:
                raise ValueError('native-H0 diagnostic bundle must have eight unit columns')
            if type(bundle.get('seed')) is not int or not 0 <= bundle['seed'] < 2**64:
                raise ValueError('missing effective native-H0 seed')
            scenarios = bundle.get('scenarios', [])
            if len(scenarios) != 8 or any(s.get('weight') != 1 for s in scenarios):
                raise ValueError('native-H0 diagnostic must retain all original columns')
        if any(response['request'].get(k) != v for k, v in item['request'].items()):
            raise ValueError('teacher response belongs to a different request')
        values = response['action_counts']
        if isinstance(values, dict):
            values = [(int(a), n) for a, n in values.items()]
        if any(type(a) is not int or type(n) is not int or not 0 <= a < 28 for a, n in values):
            raise ValueError('action/count must be exact bounded integers')
        counts = dict(values)
        if len(counts) != len(values):
            raise ValueError('duplicate root action')
        mass = response['mass']
        actions = response['clause_actions']
        if type(mass) is not int or mass <= 0 or not counts or len(actions) != clause_count:
            raise ValueError('invalid complete lesson ' + str(path))
        if any(n < 0 or n > mass for n in counts.values()):
            raise ValueError('invalid sampled count')
        if any(a is not None and a not in counts for a in actions):
            raise ValueError('illegal clause answer')
        best = max(counts.values())
        if response['denominator'] != mass or response['bundle']['mass'] != mass or response['maxcount'] != best:
            raise ValueError('inconsistent exact lesson mass or maximum')
        if response['canonical_action'] != min(a for a, n in counts.items() if n == best):
            raise ValueError('noncanonical teacher best action')
        expected_costs = [{'action': a, 'numerator': best - counts[a], 'denominator': mass} for a in sorted(counts)]
        if sorted(response['costs'], key=lambda cost: cost['action']) != expected_costs:
            raise ValueError('teacher costs disagree with action values')
        rows.append(dict(id=item['id'], group=str(item['group']), split=item['split'],
                         mass=mass, costs={a: best - n for a, n in counts.items()},
                         actions=tuple(actions), fallback=min(counts),
                         request=item['request'], feature_schema=feature_schema, teacher_contract=contract))
    if not rows:
        raise ValueError('no complete lessons')
    if len(identities) != 1:
        raise ValueError('mixed teacher targets or compiled fields')
    if any(len(s) != 1 for s in splits.values()):
        raise ValueError('source group crossed splits')
    return rows, statuses, sources


def weights(rows):
    sizes = Counter(r['group'] for r in rows)
    if not sizes:
        return [], 1
    denominator = math.lcm(*(r['mass'] * sizes[r['group']] for r in rows))
    weighted = [(r, denominator // (r['mass'] * sizes[r['group']])) for r in rows]
    return weighted, denominator * len(sizes)


def choose(row, clauses):
    for c in clauses:
        action = row['actions'][c]
        if action is not None:
            return action
    return row['fallback']


def score(weighted, clauses):
    return sum(w * row['costs'][choose(row, clauses)] for row, w in weighted)


def rational(n, d):
    f = Fraction(n, d)
    return {'numerator': f.numerator, 'denominator': f.denominator,
            'decimal': float(f)}


def signature_audit(rows):
    weighted, denominator = weights(rows)
    groups = defaultdict(list)
    for row, weight in weighted:
        groups[row['actions'] + (row['fallback'],)].append((row, weight))
    loss, positive, witnesses = 0, 0, []
    for signature, members in groups.items():
        choices = sorted({a for a in signature if a is not None})
        costs = {a: sum(w * row['costs'][a] for row, w in members) for a in choices}
        best = min(costs.values())
        loss += best
        positive += best > 0
        if best and len(witnesses) < 20:
            witnesses.append({'ids': [r['id'] for r, _ in members],
                              'actions': list(signature),
                              'lower': rational(best, denominator)})
    return {'signature_groups': len(groups), 'positive_groups': positive,
            'optimistic_empirical_loss_lower': rational(loss, denominator),
            'witnesses': witnesses,
            'scope': 'Finite local costs, independently selectable action per signature; not game regret.'}


def fit_programs(rows):
    train = [r for r in rows if r['split'] == 'train']
    dev = [r for r in rows if r['split'] == 'development']
    if not train or not dev:
        raise ValueError('both training and development complete labels are required')
    tw, td = weights(train)
    dw, dd = weights(dev)
    candidates = []
    clause_count = len(train[0]['actions'])
    if clause_count not in (14, 16) or any(len(r['actions']) != clause_count for r in rows):
        raise ValueError('mixed or unsupported clause vocabulary')
    for length in range(4):
        for clauses in permutations(range(clause_count), length):
            candidates.append((score(tw, clauses), length, clauses))
    assert len(candidates) == sum(math.perm(clause_count, length) for length in range(4))
    candidates.sort()
    # Model selection uses training only. Development is a diagnostic here;
    # eventual promotion must use saved-program complete-policy evaluation.
    best = candidates[0]
    by_length = [min(c for c in candidates if c[1] == length) for length in range(4)]
    selected = list(dict.fromkeys([c[2] for c in candidates[:8]] + [c[2] for c in by_length]))
    actor = {'schema': train[0]['feature_schema'], 'clauses': list(best[2])}
    def report(clauses):
        return {'clauses': list(clauses), 'train_local_cost': rational(score(tw, clauses), td),
                'development_local_cost': rational(score(dw, clauses), dd)}
    result = {'programs': len(candidates), 'feature_schema': actor['schema'],
              'rows': {'train': len(train), 'development': len(dev)},
              'groups': {name: len({r['group'] for r in rs}) for name, rs in [('train', train), ('development', dev)]},
              'consequential_rows': {name: sum(max(r['costs'].values()) > 0 for r in rs) for name, rs in [('train', train), ('development', dev)]},
              'selected': report(best[2]), 'empty_control': report(()),
              'candidates': [report(c) for c in selected],
              'train_signature_audit': signature_audit(train),
              'development_signature_audit': signature_audit(dev)}
    return actor, result


def select_phase_program(parent_train, phase_train):
    """Select a phase program with the parent role's global group weights.

    Filtering happens after ``weights(parent_train)``.  In particular, this
    does not re-normalize phase rows as if each phase had its own population.
    The returned denominator is the parent-role denominator so phase costs can
    be added into the full role audit exactly.
    """
    if not parent_train or not phase_train:
        raise ValueError('lead/follow phase requires complete training rows')
    clause_count = len(parent_train[0]['actions'])
    if clause_count != 16 or any(len(row['actions']) != clause_count for row in parent_train + phase_train):
        raise ValueError('lead/follow requires the 16-feature vocabulary')
    weighted, denominator = weights(parent_train)
    allowed = {row['id'] for row in phase_train}
    phase_weighted = [(row, weight) for row, weight in weighted if row['id'] in allowed]
    if len(phase_weighted) != len(phase_train):
        raise ValueError('phase rows must be a subset of parent training rows')
    candidates = []
    for length in range(4):
        for clauses in permutations(range(clause_count), length):
            candidates.append((score(phase_weighted, clauses), length, clauses))
    return min(candidates), denominator


def fit_lead_follow_role(rows):
    """Fit lead and follow programs for one role against global role weights."""
    train = [row for row in rows if row['split'] == 'train']
    development = [row for row in rows if row['split'] == 'development']
    if not train or not development:
        raise ValueError('both training and development complete labels are required')
    if any(row['feature_schema'] != 'scheme-relational-actor-v2' for row in rows):
        raise ValueError('lead/follow requires the 16-feature vocabulary')
    train_weighted, train_denominator = weights(train)
    dev_weighted, dev_denominator = weights(development)
    phases = {}
    programs = {}
    for phase, predicate in (
            ('lead', lambda row: len(row['request'].get('history', [])) % 4 == 0),
            ('follow', lambda row: len(row['request'].get('history', [])) % 4 != 0)):
        phase_train = [row for row in train if predicate(row)]
        phase_dev = [row for row in development if predicate(row)]
        if not phase_train or not phase_dev:
            raise ValueError(f'lead/follow phase {phase} lacks complete train or development rows')
        selected, _ = select_phase_program(train, phase_train)
        clauses = selected[2]
        programs[phase] = clauses
        phase_train_weighted = [(row, weight) for row, weight in train_weighted
                                if row['id'] in {item['id'] for item in phase_train}]
        phase_dev_weighted = [(row, weight) for row, weight in dev_weighted
                              if row['id'] in {item['id'] for item in phase_dev}]
        phases[phase] = {
            'programs': sum(math.perm(16, length) for length in range(4)),
            'rows': {'train': len(phase_train), 'development': len(phase_dev)},
            'groups': {'train': len({row['group'] for row in phase_train}),
                       'development': len({row['group'] for row in phase_dev})},
            'selected': {
                'clauses': list(clauses),
                'train_local_cost': rational(score(phase_train_weighted, clauses), train_denominator),
                'development_local_cost': rational(score(phase_dev_weighted, clauses), dev_denominator),
            },
        }
    train_cost = sum((Fraction(phases[phase]['selected']['train_local_cost']['numerator'],
                               phases[phase]['selected']['train_local_cost']['denominator'])
                      for phase in ('lead', 'follow')), Fraction())
    dev_cost = sum((Fraction(phases[phase]['selected']['development_local_cost']['numerator'],
                             phases[phase]['selected']['development_local_cost']['denominator'])
                    for phase in ('lead', 'follow')), Fraction())
    actor = {
        'schema': 'scheme-relational-lead-follow-v1',
        'clauses': list(programs['follow']),
        'lead_clauses': list(programs['lead']),
    }
    result = {
        'programs': sum(math.perm(16, length) for length in range(4)),
        'feature_schema': 'scheme-relational-actor-v2',
        'actor_schema': 'scheme-relational-lead-follow-v1',
        'rows': {'train': len(train), 'development': len(development)},
        'groups': {'train': len({row['group'] for row in train}),
                   'development': len({row['group'] for row in development})},
        'phases': phases,
        'selected': {
            'clauses': list(programs['follow']),
            'lead_clauses': list(programs['lead']),
            'train_local_cost': rational(train_cost.numerator, train_cost.denominator),
            'development_local_cost': rational(dev_cost.numerator, dev_cost.denominator),
        },
        'selection': 'Training only; parent-role global physical-group weights; development never selects.',
    }
    return actor, result


def fit_lead_follow(rows):
    actor = {'schema': 'scheme-role-actors-v1'}
    result = {'roles': {}}
    for role in ('declaring', 'defending'):
        actor[role], result['roles'][role] = fit_lead_follow_role(
            [row for row in rows if row_role(row) == role])
    result['role_mean_cost'] = {}
    for split in ('train', 'development'):
        values = [result['roles'][role]['selected'][split + '_local_cost']
                  for role in ('declaring', 'defending')]
        mean = sum((Fraction(value['numerator'], value['denominator']) for value in values), Fraction()) / 2
        result['role_mean_cost'][split] = rational(mean.numerator, mean.denominator)
    return actor, result


def row_role(row):
    request = row['request']
    viewer = (request['seat'] + int(request['bidder'] % 2 == 0)) % 4
    return 'declaring' if viewer % 2 else 'defending'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('campaign', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--roles', action='store_true', help='fit declaring and defending actors independently')
    parser.add_argument('--lead-follow', action='store_true',
                        help='fit role-specific lead and follow actors with parent-role weights')
    args = parser.parse_args()
    if args.lead_follow and not args.roles:
        parser.error('--lead-follow requires --roles')
    started = time.monotonic()
    rows, statuses, sources = read_rows(args.campaign)
    if args.lead_follow:
        actor, result = fit_lead_follow(rows)
    elif args.roles:
        actor = {'schema': 'scheme-role-actors-v1'}
        result = {'roles': {}}
        for role in ('declaring', 'defending'):
            actor[role], result['roles'][role] = fit_programs([r for r in rows if row_role(r) == role])
        result['role_mean_cost'] = {}
        for split in ('train', 'development'):
            values = [r['selected'][split + '_local_cost'] for r in result['roles'].values()]
            mean = sum((Fraction(v['numerator'], v['denominator']) for v in values), Fraction()) / 2
            result['role_mean_cost'][split] = rational(mean.numerator, mean.denominator)
    else:
        actor, result = fit_programs(rows)
    result.update({'schema': 'compiled-exhaustive-audit-v2',
              'teacher_contract': rows[0]['teacher_contract'],
              'source_statuses': dict(statuses), 'source_lesson_hashes': sources,
              'campaign_authority_hashes': {name: digest(args.campaign / name) for name in ['manifest.json', 'requests.jsonl', 'teacher-config.json']},
              'fitter_sha256': digest(__file__), 'elapsed_seconds': time.monotonic() - started,
              'selection': 'Exact minimum training local cost within each selected role partition, then fewer clauses, then canonical order; development never selects.',
              'scope': 'Calibration of finite local teacher costs; no full-policy or H2H claim. Incomplete labels are reported as censored.'}
    )
    args.output.mkdir(parents=True, exist_ok=False)
    atomic(args.output / 'actor.json', actor)
    atomic(args.output / 'audit.json', result)
    keys = ['programs', 'rows', 'groups', 'consequential_rows', 'selected', 'empty_control', 'elapsed_seconds', 'role_mean_cost']
    print(json.dumps({'actor': actor, **{k: result[k] for k in keys if k in result}}, indent=2))


if __name__ == '__main__':
    main()
