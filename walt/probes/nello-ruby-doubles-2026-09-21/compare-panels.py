"""Compare complete root-action panels using exact rational arithmetic.

These are optimized in-sample estimates, not fixed-plan confidence intervals.
"""
import json
from collections import Counter
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
TILES = [f'{hi}{lo}' for hi in range(7) for lo in range(hi + 1)]
SOURCES = [(40, 'panel.json'), (160, 'panel.json'), (640, 'panel640.json'),
           (2000, '2000/panel2000.json'), (10000, '10000/panel10000.json'),
           (100000, '100000/cases/ply*.json')]
rows = []
for n, filename in SOURCES:
    if '*' in filename:
        paths = sorted(HERE.glob(filename))
        if not paths:
            continue
        raw = [json.loads(path.read_text()) for path in paths]
    else:
        path = HERE / filename
        if not path.exists():
            continue
        raw = json.loads(path.read_text())['rows']
    if len(raw) != 16:
        print(f'{n}: incomplete ({len(raw)}/16), excluded')
        continue
    for ply in (6, 9):
        cases = sorted((r for r in raw if r['ply'] == ply),
                       key=lambda r: r['call']['request']['seed'])
        assert [r['call']['request']['seed'] for r in cases] == list(range(1, 9))
        values, choices = [], []
        for r in cases:
            if n == 40:
                result = next(c for c in r['checkpoints']
                              if c['phases'][-1].get('worlds') == 40
                              and c['phases'][-1]['status'] == 'completed')
            else:
                result = r['result']
            assert result['evaluation']['outer_worlds'] == n
            assert result['route'] == 'baseline'
            v = {TILES[t]: 1 - Fraction(int(a), int(b))
                 for t, a, b in result['evaluation']['options']}
            choice = TILES[result['choice']]
            assert v[choice] == max(v.values())
            values.append(v)
            choices.append(choice)
        means = {tile: sum(v[tile] for v in values) / 8 for tile in values[0]}
        played = '66' if ply == 6 else '44'
        gaps = [v[played] - max(v[t] for t in ('21', '31', '51')) for v in values]
        row = {'worlds': n, 'ply': ply, 'source': filename,
               'mean_set': {t: str(v) for t, v in means.items()},
               'choices': dict(sorted(Counter(choices).items())),
               'min_set': {t: str(min(v[t] for v in values)) for t in means},
               'max_set': {t: str(max(v[t] for v in values)) for t in means},
               'played_minus_best_mixed_each_seed': list(map(str, gaps)),
               'played_beats_best_mixed_seeds': sum(g > 0 for g in gaps),
               'played_ties_best_mixed_seeds': sum(g == 0 for g in gaps)}
        rows.append(row)
        print(n, ply, row['choices'],
              {t: round(float(v * 100), 3) for t, v in means.items()})
(HERE / 'ladder-summary.json').write_text(json.dumps(rows, indent=2) + '\n')
