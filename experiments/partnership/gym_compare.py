"""Compare scenario recipes by stable coordinates, including unselected controls."""
from fractions import Fraction
import json
from pathlib import Path

import gym


def snapshot(directory):
    latest = directory/'latest.json'
    if latest.exists():
        receipt = json.loads(latest.read_text())
        if not receipt['complete']:
            raise ValueError('cannot compare an incomplete generation')
        directory = Path(receipt['gallery'])
    cases = {case['id']: (name, case) for name, case in gym.gallery(directory)}
    discovery = directory/'discovery.json'
    if discovery.exists():
        data = json.loads(discovery.read_text())
        rows = {row['id']: row for row in data['rows']}
    else:
        rows = {}
    for rid, (name, case) in cases.items():
        key = case['key']
        # Older collections did not retain contrast in discovery summaries.
        rows[rid] = {**rows.get(rid, {}), 'id': rid, 'outcome': gym.outcome_profile(key),
                     'worlds': key['worlds'], 'coverage': key.get('coverage', 'census'),
                     'support_worlds': key.get('support_worlds', key['worlds']),
                     'target_actions': case.get('target_actions', key['offers']),
                     'query_contrast': gym.query_contrast(key, case.get('target_actions', key['offers']))}
    for row in rows.values(): row['selected'] = row['id'] in cases
    resolved = directory/'specification.json'
    return dict(directory=str(directory.resolve()),
                specification=json.loads(resolved.read_text()) if resolved.exists() else None,
                rows=rows, cases=cases)


def transition(row):
    if row is None: return 'outside source/domain'
    if 'skipped' in row: return row['skipped']
    if 'query_contrast' not in row: return 'not recorded'
    contrast = row.get('query_contrast')
    if contrast is None: return 'no target/non-target contrast'
    gap = Fraction(contrast['gap'])
    return 'helpful' if gap > 0 else 'harmful' if gap < 0 else 'tied'


def witness(case):
    pair, key = case['pair'], case['key']
    actions = {a['tile']: a for a in key['actions']}
    other = {gym.world_key(t['hands']): t for t in actions[pair['comparison']]['traces']}
    examples = {}
    for good in actions[pair['preferred']]['traces']:
        bad = other[gym.world_key(good['hands'])]
        label = 'saved' if good['success'] and not bad['success'] else 'lost' if bad['success'] and not good['success'] else None
        if label and label not in examples:
            examples[label] = dict(hands=good['hands'], preferred={k: good[k] for k in ('plays','banked','success')},
                                   comparison={k: bad[k] for k in ('plays','banked','success')})
    return dict(request=case['request'], pair=pair, paired=gym.paired_outcomes(key, pair), examples=examples)


def comparison(before, after):
    old, new = before['rows'], after['rows']
    rows = []
    for rid in sorted(old.keys() | new.keys()):
        a, b = old.get(rid), new.get(rid)
        best_changed = (a is not None and b is not None and 'outcome' in a and 'outcome' in b
                        and a['outcome']['optimal'] != b['outcome']['optimal'])
        values_changed = (a is not None and b is not None and 'outcome' in a and 'outcome' in b
                          and a['outcome']['actions'] != b['outcome']['actions'])
        rows.append(dict(id=rid, before=a, after=b, before_relation=transition(a), after_relation=transition(b),
                         best_changed=bool(best_changed), values_changed=bool(values_changed)))
    old_selected, new_selected = set(before['cases']), set(after['cases'])
    measured_both = [r for r in rows if r['before'] and r['after']
                     and 'outcome' in r['before'] and 'outcome' in r['after']]
    examples = []
    for row in rows:
        if (row['before_relation'] != row['after_relation'] or row['best_changed']) and row['id'] in after['cases']:
            name, case = after['cases'][row['id']]
            examples.append(dict(id=row['id'], name=name, before_relation=row['before_relation'],
                                 after_relation=row['after_relation'], **witness(case)))
            if len(examples) == 6: break
    return dict(schema='gym-comparison-v1', before={k: before[k] for k in ('directory','specification')},
                after={k: after[k] for k in ('directory','specification')},
                summary=dict(before_selected=len(old_selected), after_selected=len(new_selected),
                             added=sorted(new_selected-old_selected), removed=sorted(old_selected-new_selected),
                             retained=len(old_selected & new_selected), measured_both=len(measured_both),
                             best_changed=sum(r['best_changed'] for r in rows),
                             values_changed=sum(r['values_changed'] for r in rows),
                             relation_changed=sum(r['before_relation'] != r['after_relation'] for r in measured_both
                                                  if 'not recorded' not in (r['before_relation'],r['after_relation']))),
                rows=rows, examples=examples)


def compare(args):
    result = comparison(snapshot(args.before), snapshot(args.after))
    gym.atomic(args.output, result)
    s = result['summary']
    lines = ['# Scenario comparison', '',
             f"Selected {s['before_selected']} → {s['after_selected']}; "
             f"{len(s['added'])} added, {len(s['removed'])} removed, {s['retained']} retained.", '',
             f"Among {s['measured_both']} coordinates valued in both runs, "
             f"{s['values_changed']} changed action values and {s['best_changed']} changed the set of best actions. "
             f"{s['relation_changed']} changed the helpful/harmful/tied relationship of the query.", '',
             'These are conditional continuation values. Full support is exact for the frozen players; '
             'sampled support is an estimate. Neither is a general game-strength measurement.', '',
             '| Coordinate | Before | After | Best actions before → after | Selected before → after |',
             '|---|---|---|---|---|']
    for row in result['rows']:
        a, b = row['before'] or {}, row['after'] or {}
        if row['best_changed'] or row['before_relation'] != row['after_relation'] or a.get('selected') != b.get('selected'):
            render = lambda r: ', '.join(gym.tile(t) for t in r.get('outcome', {}).get('optimal', [])) or '—'
            lines.append(f"| {row['id']} | {row['before_relation']} | {row['after_relation']} | "
                         f"{render(a)} → {render(b)} | {bool(a.get('selected'))} → {bool(b.get('selected'))} |")
    for item in result['examples']:
        lines += ['', f"## Paired witness: {item['name']} ({item['id']})", '',
                  f"{item['before_relation']} → {item['after_relation']}. Preferred "
                  f"{gym.tile(item['pair']['preferred'])}, comparison {gym.tile(item['pair']['comparison'])}; "
                  f"{item['paired']['gained']} saved worlds, {item['paired']['lost']} lost worlds."]
        for label, pair in item['examples'].items():
            lines += ['', f"{label.capitalize()} world, remaining hands: " +
                      ' / '.join(' '.join(gym.tile(t) for t in hand) for hand in pair['hands']) + '.']
            for arm in ('preferred', 'comparison'):
                trace = pair[arm]
                moves = ' '.join(f'{seat}:{gym.tile(tile)}' for seat,tile in zip(trace['plays'][::2],trace['plays'][1::2]))
                lines.append(f"- {arm}: {moves}; final team points {trace['banked']}; success {trace['success']}.")
    args.output.with_suffix('.md').write_text('\n'.join(lines)+'\n')
    print(gym.canonical(s), flush=True)
    return result
