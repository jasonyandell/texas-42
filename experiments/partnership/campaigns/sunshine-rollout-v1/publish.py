"""Publish compact evidence; full traces and per-decision caches remain external."""
import argparse
from collections import Counter
from fractions import Fraction
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))
import campaign as c
import gym


def publish(root):
    read = lambda path: json.loads(path.read_text())
    freeze = read(HERE / 'freeze.json')
    arena = read(root / 'fresh-match/MATCH.json')
    assert arena['completed_pairs'] == 64
    assert freeze['match'] == read(root / 'fresh-match/manifest.json')
    summaries, roots = {}, {}
    for name in ('development', 'fresh-measurement'):
        report = read(root / name / 'report.json')
        assert report['summary']['complete']
        if name == 'fresh-measurement':
            assert report['manifest']['identities'] == freeze['identities']
        summaries[name] = report['summary']
        review_times = [phase['elapsed_us'] / 1e6 for row in report['rows']
                        for phase in row['after']['phases'] if phase['name'] == 'partner-rollout-review']
        summaries[name]['mean_review_seconds'] = sum(review_times) / len(review_times)
        summaries[name]['max_review_seconds'] = max(review_times)
        rows = []
        for row in report['rows']:
            reference = Path(report['manifest']['source']) / 'items' / (row['id']+'.json')
            case = read(reference)
            assert gym.digest(case) == report['manifest']['cases'][row['id']]
            values = {a['tile']: a['success_mass'] for a in case['key']['actions']}
            before, after = row['before']['choice'], row['after']['choice']
            assert Fraction(row['delta']) == Fraction(values[after]-values[before], row['worlds'])
            review = {k: v for k, v in row['after']['review_result'].items() if k not in ('decisions', 'traces')}
            rows.append(dict(id=row['id'], seed=row['source']['seed'], request=row['request'],
                             reference_sha256=gym.file_hash(reference), worlds=row['worlds'],
                             values=sorted(values.items()), offers=case['target_actions'],
                             baseline=before, candidate=after, delta=row['delta'], paired=row['paired'],
                             grade_before=row['grade_before'], grade_after=row['grade_after'], review=review))
        roots[name] = rows
    stats = Counter(); active = []; changed = []
    for path in sorted((root / 'fresh-match/seeds').glob('*/*/checkpoint.json')):
        for ply, decision in enumerate(read(path)['decisions']):
            review = decision['response'].get('review_result')
            if review is None: continue
            stats[review['status']] += 1
            if review['status'] != 'inactive':
                active.append(dict(seed=int(path.parent.parent.name), arm=path.parent.name,
                                   ply=ply, seat=decision['seat'], review=review))
            if review['status'] == 'changed': changed.append(active[-1])
            if path.parent.name == 'defending': assert review['status'] == 'inactive'
    # Checked before and after resume; never treat hidden completions as seeds.
    interrupted = read(root / 'interrupt-before.json')
    for path, sha in interrupted['committed'].items():
        assert gym.file_hash(root / 'fresh-match' / path) == sha
    audits = {name: read(root / name / 'summary.json')['results']
              for name in ('audit', 'fresh-audit', 'release-audit', 'release-fresh-audit')}
    prefixes = {}
    for name in ('fresh-helpful-offer', 'fresh-harmful-withholding'):
        audit = read(root / 'fresh-audit' / (name+'.json'))
        prefixes[name] = {}
        for n in sorted({8, 32, 52, 64, audit['support']}):
            if n > audit['support']: continue
            values = {a: 0 for a in audit['legal']}
            for trace in audit['traces'][:n*len(values)]: values[trace['action']] += trace['made']
            prefixes[name][n] = sorted(values.items())
    evidence = dict(schema='sunshine-rollout-results-v1', summaries=summaries,
                    arena=arena, arena_reviews=dict(statuses=dict(stats), active=active, changed=changed),
                    gym=read(root / 'fresh-gym/latest.json'), audits=audits, prefixes=prefixes,
                    interruption=read(root / 'interrupt-verification.json'),
                    measured_identities=freeze['identities'], release_identities=c.identities(),
                    release_note='After the frozen measurements, an infeasible continuation frame was made an explicit worker error instead of being labeled a deadline. Default baseline retention applies to that error. No policy settings changed; both complete audit panels were rerun against the release binary.')
    raw = [root/'development/report.json', root/'fresh-measurement/report.json',
           root/'fresh-match/MATCH.json', root/'fresh-gym/latest.json']
    raw.extend(sorted((root/'runs').glob('*/run.json')))
    for name in audits:
        raw.extend(sorted((root/name).glob('*.json')))
    evidence['raw_files'] = {str(path): gym.file_hash(path) for path in raw}
    evidence['workloads'] = [read(path) for path in sorted((root/'runs').glob('*/run.json'))]
    gym.atomic(HERE/'summary.json', evidence)
    gym.atomic(HERE/'roots.json', roots)
    print(gym.canonical(dict(development=summaries['development']['roots'],
                             fresh=summaries['fresh-measurement']['roots'],
                             mirrored_deals=arena['completed_pairs'],
                             raw_files=len(raw), summaries=str(HERE/'summary.json'))))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root', type=Path)
    publish(parser.parse_args().root.resolve())
