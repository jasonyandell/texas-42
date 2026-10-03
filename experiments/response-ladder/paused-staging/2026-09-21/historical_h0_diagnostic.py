#!/usr/bin/env python3
"""Finite H0 teacher/predictor diagnostic. Never fits actors or measures game utility."""
import argparse
from collections import Counter, defaultdict
from fractions import Fraction
import hashlib
import importlib.util
import json
from pathlib import Path

ROLES = ('declaring', 'defending')
EXPECTED = {
    'h0': ('historical-voidless-h0-v1', 'h0-native-dice-exact-v1'),
    'h0_seed': ('compatible-root-h0-seed-v1', 't0-h0-seed-dice-exact-v1'),
    't0': ('compatible-root-reset-v1', 't0-dice-exact-v1'),
}


def require(ok, message):
    if not ok:
        raise ValueError(message)


def read(path):
    return json.loads(Path(path).read_text())


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def tree_digest(hashes):
    return hashlib.sha256(json.dumps(hashes, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def load_fitter(path):
    spec = importlib.util.spec_from_file_location('diagnostic_fitter', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def rational(value):
    if value is None:
        return None
    value = Fraction(value)
    require(0 <= value <= 1, 'normalized metric outside [0,1]')
    return {'numerator': value.numerator, 'denominator': value.denominator, 'decimal': float(value)}


def unrational(value):
    return Fraction(value['numerator'], value['denominator'])


def role(request):
    return 'declaring' if (request['seat'] + int(request['bidder'] % 2 == 0)) % 2 else 'defending'


def load_campaign(directory, fit, expected=None, expected_n=None):
    directory = Path(directory)
    manifest = read(directory / 'manifest.json')
    require(digest(directory / 'requests.jsonl') == manifest['requests_sha256'], 'request manifest digest mismatch')
    requests, groups = {}, defaultdict(set)
    for line in (directory / 'requests.jsonl').read_text().splitlines():
        item = json.loads(line)
        require(item['id'] not in requests, 'duplicate manifest request ID')
        require(item['split'] in ('train', 'development'), 'unknown manifest split')
        requests[item['id']] = item
        groups[str(item['group'])].add(item['split'])
    require(len(requests) == manifest['requests_total'], 'request count mismatch')
    require(all(len(s) == 1 for s in groups.values()), 'physical group crosses split')
    lessons, hashes = {}, {}
    for path in sorted((directory / 'lessons').glob('*.json')):
        item = read(path)
        identifier = item['id']
        require(identifier in requests and identifier not in lessons and path.stem == identifier, 'unknown/duplicate lesson ID')
        original = requests[identifier]
        require(all(item[k] == original[k] for k in ('request', 'group', 'split')), 'lesson request/group/split mismatch')
        response = item['response']
        status = response.get('status', 'missing-status')
        require(item.get('status', status) == status, 'outer/nested status mismatch')
        if 'request' in response:
            require(all(response['request'].get(k) == v for k, v in original['request'].items()), 'response request mismatch')
        if 'target' in response and expected:
            require((response['target'], response.get('revision')) == expected, 'unexpected teacher target/revision')
        if expected_n is not None and status == 'complete':
            require(response['request'].get('n', 8) == expected_n and response['mass'] == expected_n, 'unexpected teacher mass')
        lessons[identifier] = item
        hashes[str(path.relative_to(directory))] = digest(path)
    # Reuse authoritative exact cost/schema validation and homogeneous identities.
    if any(x['response'].get('status') == 'complete' for x in lessons.values()):
        rows, statuses, fitter_hashes = fit.read_rows(directory)
        require(fitter_hashes == hashes, 'fitter source set mismatch')
    else:
        rows = []
        statuses = Counter(x['response'].get('status', 'missing-status') for x in lessons.values())
    complete = {row['id']: row for row in rows}
    require(len(complete) == len(rows), 'duplicate complete row ID')
    return dict(directory=str(directory), requests=requests, lessons=lessons, rows=complete,
                hashes=hashes, statuses=dict(statuses), authority={name: digest(directory / name)
                for name in ('manifest.json', 'requests.jsonl', 'teacher-config.json') if (directory / name).exists()})


def match_campaigns(campaigns):
    first = next(iter(campaigns.values()))['requests']
    for campaign in campaigns.values():
        require(set(campaign['requests']) == set(first), 'campaign ID sets differ')
        for identifier, item in first.items():
            other = campaign['requests'][identifier]
            require(all(item[k] == other[k] for k in ('request', 'group', 'split')), 'matched campaign request/group/split differs')
    metadata = {}
    for identifier, item in first.items():
        states, features = [], []
        for campaign in campaigns.values():
            response = campaign['lessons'].get(identifier, {}).get('response', {})
            if response.get('normalized_state') is not None:
                states.append(response['normalized_state'])
            if identifier in campaign['rows']:
                require(response.get('normalized_state') is not None, 'complete lesson lacks public state')
                features.append(campaign['rows'][identifier]['actions'])
        require(all(state == states[0] for state in states), 'normalized public states differ')
        for actions in features:
            require(all(actions[:min(len(actions), len(other))] == other[:min(len(actions), len(other))]
                        for other in features), 'observation-only clause features differ')
        public = states[0]['public'] if states else None
        if public is not None:
            require(isinstance(public.get('voids'), list) and len(public['voids']) == 4, 'invalid public void masks')
        metadata[identifier] = dict(role=role(item['request']), depth=len(item['request']['hand']),
            public_void=('yes' if any(public['voids']) else 'no') if public is not None else 'unknown',
            split=item['split'], group=str(item['group']))
    return metadata


def status(campaign, identifier):
    return campaign['lessons'].get(identifier, {}).get('response', {}).get('status', 'missing')


def coverage(campaigns, identifiers):
    ids = sorted(identifiers)
    return {'requests': len(ids), 'campaigns': {name: dict(Counter(status(c, i) for i in ids))
            for name, c in campaigns.items()}, 'joint_statuses': dict(Counter(
            '|'.join(f'{name}:{status(c, i)}' for name, c in campaigns.items()) for i in ids))}


def weighted_cost(rows, actions, fit):
    if not rows:
        return None
    weighted, denominator = fit.weights(rows)
    total = 0
    for row, weight in weighted:
        action = actions[row['id']]
        require(action in row['costs'], 'chosen action absent from target legal vector')
        require(0 <= row['costs'][action] <= row['mass'], 'invalid raw cost')
        total += weight * row['costs'][action]
    answer = Fraction(total, denominator)
    require(0 <= answer <= 1, 'incorrect normalized group weighting')
    return answer


def canonical(row):
    optimal = {a for a, cost in row['costs'].items() if cost == 0}
    require(bool(optimal), 'no target optimum')
    return min(optimal)


def pair_metrics(left, right, ids, metadata, fit):
    ids = sorted(i for i in ids if i in left['rows'] and i in right['rows'])
    for i in ids:
        require(set(left['rows'][i]['costs']) == set(right['rows'][i]['costs']), 'matched legal action sets differ')
    la = {i: canonical(left['rows'][i]) for i in ids}
    ra = {i: canonical(right['rows'][i]) for i in ids}
    exact = sum(left['lessons'][i]['response']['action_counts'] == right['lessons'][i]['response']['action_counts']
                and left['rows'][i]['mass'] == right['rows'][i]['mass'] for i in ids)
    normalized = sum(all(Fraction(left['lessons'][i]['response']['maxcount'] - cost, left['rows'][i]['mass'])
        == Fraction(right['lessons'][i]['response']['maxcount'] - right['rows'][i]['costs'][a], right['rows'][i]['mass'])
        for a, cost in left['rows'][i]['costs'].items()) for i in ids)
    agreement = sum(la[i] == ra[i] for i in ids)
    overlap = sum(bool({a for a,c in left['rows'][i]['costs'].items() if c == 0}
                      & {a for a,c in right['rows'][i]['costs'].items() if c == 0}) for i in ids)
    roles = {}
    for name in ROLES:
        selected = [i for i in ids if metadata[i]['role'] == name]
        roles[name] = {'rows': len(selected), 'groups': len({metadata[i]['group'] for i in selected}),
            'left_choice_cost_on_right': rational(weighted_cost([right['rows'][i] for i in selected], la, fit)),
            'right_choice_cost_on_left': rational(weighted_cost([left['rows'][i] for i in selected], ra, fit))}
    means = {}
    for metric in ('left_choice_cost_on_right', 'right_choice_cost_on_left'):
        values = [roles[name][metric] for name in ROLES]
        means[metric] = rational(sum(map(unrational, values), Fraction()) / 2) if all(v is not None for v in values) else None
    return {'paired_complete': len(ids), 'exact_vector_matches': exact, 'normalized_vector_matches': normalized,
            'raw_canonical_matches': agreement, 'raw_canonical_fraction': rational(Fraction(agreement, len(ids))) if ids else None,
            'raw_optimal_set_intersections': overlap,
            'raw_optimal_intersection_fraction': rational(Fraction(overlap, len(ids))) if ids else None,
            'roles': roles, 'equal_role_mean_cost': means}


def no_void_invariant(h0, compatible, metadata):
    checked = []
    for i in sorted(h0['rows'].keys() & compatible['rows'].keys()):
        if metadata[i]['public_void'] != 'no':
            continue
        a, b = h0['lessons'][i]['response'], compatible['lessons'][i]['response']
        require(a['bundle']['seed'] == b['bundle']['seed'], f'no-void native seed mismatch: {i}')
        require(a['bundle']['scenarios'] == b['bundle']['scenarios'], f'no-void ordered worlds/tapes mismatch: {i}')
        require(a['action_counts'] == b['action_counts'] and a['mass'] == b['mass'], f'no-void exact vector mismatch: {i}')
        checked.append(i)
    return {'status': 'passed' if checked else 'no_complete_no_void_pairs', 'checked': len(checked),
            'checked_ids_sha256': tree_digest(checked)}


def actor_action(actor, row, fit):
    if actor.get('schema') == 'scheme-role-actors-v1':
        actor = actor[role(row['request'])]
    limit = {'scheme-relational-actor-v1': 14, 'scheme-relational-actor-v2': 16}.get(actor.get('schema'))
    require(limit is not None, 'diagnostic requires frozen role-only v1/v2 actors')
    require(actor.get('lead_clauses') is None, 'phase actors are a different control')
    clauses = actor['clauses']
    require(len(clauses) <= 3 and len(set(clauses)) == len(clauses)
            and all(type(c) is int and 0 <= c < limit for c in clauses), 'invalid frozen actor clauses')
    require(len(row['actions']) in (14, 16) and all(c < len(row['actions']) for c in clauses), 'actor needs missing clause features')
    # v1 uses the preserved first14 v2 features; v2 on v1 is allowed only when
    # all selected clauses exist. Never censor a complete row by actor outcome.
    return fit.choose(row, clauses)


def score_actor(actor, rows, fit):
    actions = {r['id']: actor_action(actor, r, fit) for r in rows}
    roles = {}
    for name in ROLES:
        selected = [r for r in rows if role(r['request']) == name]
        n = len(selected)
        ch = sum(actions[r['id']] == canonical(r) for r in selected)
        oh = sum(r['costs'][actions[r['id']]] == 0 for r in selected)
        roles[name] = {'rows': n, 'groups': len({r['group'] for r in selected}),
            'local_cost': rational(weighted_cost(selected, actions, fit)),
            'raw_canonical_hits': ch, 'raw_canonical_fraction': rational(Fraction(ch, n)) if n else None,
            'raw_optimal_hits': oh, 'raw_optimal_fraction': rational(Fraction(oh, n)) if n else None}
    costs = [roles[name]['local_cost'] for name in ROLES]
    return {'rows': len(rows), 'roles': roles,
            'equal_role_mean_cost': rational(sum(map(unrational, costs), Fraction()) / 2) if all(c is not None for c in costs) else None,
            'raw_canonical_hits': sum(v['raw_canonical_hits'] for v in roles.values()),
            'raw_optimal_hits': sum(v['raw_optimal_hits'] for v in roles.values())}


def verify_fit(actor, audit, campaign, fit):
    require(actor.get('schema') == 'scheme-role-actors-v1' and 'roles' in audit, 'authoritative audit must be role-only')
    require(audit['source_lesson_hashes'] == campaign['hashes'], 'actor audit references different lessons')
    require(all(campaign['authority'].get(k) == v for k,v in audit['campaign_authority_hashes'].items()), 'actor audit authority differs')
    for split in ('train', 'development'):
        scored = score_actor(actor, [r for r in campaign['rows'].values() if r['split'] == split], fit)
        for name in ROLES:
            expected = audit['roles'][name]['selected'][split + '_local_cost']
            require(scored['roles'][name]['local_cost'] is not None and
                    unrational(scored['roles'][name]['local_cost']) == unrational(expected), 'authoritative per-role fit cost mismatch')
            require(actor[name]['clauses'] == audit['roles'][name]['selected']['clauses'], 'actor/audit clauses differ')
        require(scored['equal_role_mean_cost'] is not None and
                unrational(scored['equal_role_mean_cost']) == unrational(audit['role_mean_cost'][split]), 'authoritative role-mean fit cost mismatch')
    return {'status': 'passed', 'checks': 'exact train/development costs by role and equal role mean on original fit population'}


def witnesses(left, right, metadata, maximum=8):
    found = []
    for i in sorted(left['rows'].keys() & right['rows'].keys()):
        a, b = left['rows'][i], right['rows'][i]
        ca, cb = canonical(a), canonical(b)
        ab, ba = Fraction(b['costs'][ca], b['mass']), Fraction(a['costs'][cb], a['mass'])
        if ca != cb or ab or ba:
            found.append((max(ab, ba), ab + ba, i, {'id': i, **metadata[i], 'left_canonical': ca,
                'right_canonical': cb, 'left_choice_cost_on_right': rational(ab), 'right_choice_cost_on_left': rational(ba)}))
    return [x[-1] for x in sorted(found, key=lambda x: (-x[0], -x[1], x[2]))[:maximum]]


def summarize(campaigns, actors, fit):
    metadata = match_campaigns(campaigns)
    all_ids = set(metadata)
    result = {'schema': 'historical-h0-diagnostic-v1', 'scope': 'Finite sampled teaching-target and imitation diagnostic; not game utility or stronger-policy evidence.',
        'weighting': 'Within each selected role/split/stratum and complete comparison population, fitter.weights assigns equal physical-group weight and equal row weight within group; cost divides by target row original mass. Role mean is half each role, never pooled row mean. Agreement counts are unweighted.',
        'coverage': coverage(campaigns, all_ids), 'coverage_by_joint_stratum': {}, 'comparisons': {}, 'actor_cross_scores': {}}
    strata = defaultdict(set)
    for i,m in metadata.items():
        strata[f"{m['split']}|{m['role']}|depth{m['depth']}|voids-{m['public_void']}"].add(i)
    result['coverage_by_joint_stratum'] = {k:coverage(campaigns,ids) for k,ids in sorted(strata.items())}
    result['no_void_invariant'] = no_void_invariant(campaigns['h0'],campaigns['h0_seed'],metadata)
    for a,b in [('h0','h0_seed'),('h0_seed','t0'),('h0','t0')]:
        left,right = campaigns[a],campaigns[b]
        metrics = pair_metrics(left,right,all_ids,metadata,fit)
        metrics['coverage'] = coverage({a:left,b:right},all_ids)
        metrics['by_joint_stratum'] = {k:pair_metrics(left,right,ids,metadata,fit) for k,ids in sorted(strata.items())}
        metrics['worst_witnesses'] = witnesses(left,right,metadata)
        result['comparisons'][f'{a}_versus_{b}'] = metrics
    common = campaigns['h0']['rows'].keys() & campaigns['t0']['rows'].keys()
    for target in ('h0','t0'):
        for label,allowed in [('all_complete',campaigns[target]['rows'].keys()),('common_complete',common)]:
            rows = [campaigns[target]['rows'][i] for i in sorted(allowed) if metadata[i]['split']=='development']
            result['actor_cross_scores'][target+'_'+label+'_development'] = {
                'matched_ids_sha256':tree_digest(sorted(r['id'] for r in rows)),
                'actors':{name:score_actor(actor,rows,fit) for name,actor in actors.items()}}
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,required=True,help='response-ladder crate root')
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--fitter',type=Path)
    args = parser.parse_args()
    root = args.root.resolve()
    fitter_path = args.fitter or root/'tools/fit_compiled.py'
    fit = load_fitter(fitter_path)
    names = {'h0':'compiled-h0-calibration-v1','h0_seed':'compiled-h0-seed-calibration-v1','t0':'compiled-calibration-v3'}
    campaigns = {k:load_campaign(root/'results'/name,fit,EXPECTED[k],8) for k,name in names.items()}
    actor_dirs = {'h0':root/'results/compiled-h0-c0-v1','v3':root/'results/compiled-c0-v3','v4':root/'results/compiled-c0-v4'}
    actors = {k:read(p/'actor.json') for k,p in actor_dirs.items()}
    audits = {k:read(p/'audit.json') for k,p in actor_dirs.items()}
    # v4 authority must be reproduced on its own n32 labels, not the n8 target.
    v4_labels = load_campaign(root/'results/compiled-calibration-v4',fit,EXPECTED['t0'],32)
    verified = {k:verify_fit(actors[k],audits[k],c,fit) for k,c in [('h0',campaigns['h0']),('v3',campaigns['t0']),('v4',v4_labels)]}
    report = summarize(campaigns,actors,fit)
    report['authoritative_fit_checks'] = verified
    all_campaigns = {**campaigns,'v4_fit_only':v4_labels}
    hashes = {'script':digest(__file__),'fitter':digest(fitter_path),
        'protocol':digest(root/'docs/HISTORICAL-H0-DIAGNOSTIC-PROTOCOL.md'),
        'campaigns':{k:{'path':c['directory'],'authority':c['authority'],'lessons':c['hashes']} for k,c in all_campaigns.items()},
        'actors':{k:{'path':str(p),'actor_sha256':digest(p/'actor.json'),'audit_sha256':digest(p/'audit.json')} for k,p in actor_dirs.items()}}
    report['input_identities'] = {'script':hashes['script'],'fitter':hashes['fitter'],'protocol':hashes['protocol'],
        'campaigns':{k:{'authority':c['authority'],'lesson_count':len(c['hashes']),'lesson_tree_sha256':tree_digest(c['hashes'])} for k,c in all_campaigns.items()},
        'actors':hashes['actors']}
    args.output.mkdir(parents=True,exist_ok=False)
    fit.atomic(args.output/'input-hashes.json',hashes)
    fit.atomic(args.output/'summary.json',report)
    text = ['# Historical H0 diagnostic','',report['scope'],'',
        f"Matched {report['coverage']['requests']} requests. Censored outcomes remain in coverage by role/depth/public-void stratum.",
        f"No-void exact check: {report['no_void_invariant']['status']} ({report['no_void_invariant']['checked']} complete pairs).",'',
        '| Targets | Complete pairs | Canonical matches | Optimal overlap | Left cost on right | Right cost on left |',
        '|---|---:|---:|---:|---:|---:|']
    def show(v): return 'unavailable' if v is None else f"{v['decimal']:.6f}"
    for name,m in report['comparisons'].items():
        text.append(f"| {name} | {m['paired_complete']} | {m['raw_canonical_matches']} | {m['raw_optimal_set_intersections']} | {show(m['equal_role_mean_cost']['left_choice_cost_on_right'])} | {show(m['equal_role_mean_cost']['right_choice_cost_on_left'])} |")
    text += ['', 'Costs use equal role means of physical-group weights; agreements are raw counts.','',
        '| Development labels/population | Actor | Rows | Canonical hits | Optimal hits | Teacher cost |', '|---|---|---:|---:|---:|---:|']
    for population,item in report['actor_cross_scores'].items():
        for name,m in item['actors'].items():
            text.append(f"| {population} | {name} | {m['rows']} | {m['raw_canonical_hits']} | {m['raw_optimal_hits']} | {show(m['equal_role_mean_cost'])} |")
    text += ['', 'All actors use identical completed rows within each population. Common-complete panels also fix IDs across targets. Authoritative role fit costs were reproduced exactly on original fit populations. Local teacher quantities do not establish game strength.', '', 'Full strata, small worst-case witness lists, coverage and input hashes are retained beside this report.']
    (args.output/'REPORT.md').write_text('\n'.join(text)+'\n')
    print(json.dumps({'output':str(args.output),'requests':len(campaigns['h0']['requests']),
                      'no_void_invariant':report['no_void_invariant'],'authoritative_fit_checks':verified},indent=2))


if __name__ == '__main__':
    main()
