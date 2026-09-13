"""Publish compact evidence and readable witnesses from a fully audited run."""
from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parents[1]))
import campaign as c
import gym
from rules import TILES, replay_record, trick_points, winner


def tile(value):
    a, b = TILES[value]
    return f'{a}–{b}'


def main():
    output = Path(sys.argv[1]).resolve()
    report_path = output / 'report.json'
    report = c.read(report_path)
    assert report['schema'] == 'sunshine-gym-replay-report-v1'
    assert report['manifest']['panel_sha256'] == gym.file_hash(HERE / 'panel.json')
    assert report['manifest']['identities'] == c.identities()
    for name, digest in report['manifest']['sources'].items():
        assert gym.file_hash(HERE.parents[1] / name) == digest
    for path, digest in report['artifact_hashes'].items():
        assert gym.file_hash(path) == digest
    interrupted = c.read(output / 'interruption.json')
    for path, digest in interrupted['durable_files'].items():
        assert gym.file_hash(path) == digest
    compact = {k: v for k, v in report.items() if k not in ('manifest', 'artifact_hashes', 'witnesses')}
    compact['manifest'] = {k: v for k, v in report['manifest'].items() if k != 'panel'}
    compact['raw_report'] = dict(path=str(report_path), sha256=gym.file_hash(report_path))
    archive = gzip.compress(gym.canonical(report['artifact_hashes']).encode(), mtime=0)
    (HERE / 'artifact-hashes.json.gz').write_bytes(archive)
    compact['artifact_index'] = dict(path='artifact-hashes.json.gz',
                                     sha256=hashlib.sha256(archive).hexdigest(),
                                     entries=len(report['artifact_hashes']))
    root_statuses = Counter(r['review']['status'] for r in report['roots'])
    routes, review_statuses = Counter(), Counter()
    for path in (output / 'decisions').glob('*.json'):
        response = c.read(path)['response']
        assert not response['over_budget']
        routes[response['route']] += 1
        if response.get('review_result'):
            review_statuses[response['review_result']['status']] += 1
    compact['execution'] = dict(root_review_statuses=dict(root_statuses),
                                 decision_routes=dict(routes), review_statuses=dict(review_statuses))
    c.atomic(HERE / 'summary.json', compact)
    c.atomic(HERE / 'witnesses.json', report['witnesses'])
    c.atomic(HERE / 'resume-verification.json', dict(
        schema='sunshine-gym-replay-resume-verified-v1',
        interrupted_returncode=interrupted['returncode'],
        completed_before_interrupt=interrupted['completed_trajectories'],
        completed_after_resume=report['full_game_replays'],
        durable_files_unchanged=len(interrupted['durable_files']),
        interruption_sha256=gym.file_hash(output / 'interruption.json')))
    c.atomic(HERE / 'run-receipts.json', {
        name: c.read(HERE.parents[1] / 'runs' / name / 'run.json')
        for name in ('sunshine-gym-replay-prepare', 'sunshine-gym-replay-interrupt',
                     'sunshine-gym-replay-resume-01')})
    lines = ['# Concrete same-hand counterfactuals', '',
             'Selected from the full-support replay. These are examiner-visible',
             'hidden-hand witnesses, not information available to the actor.',
             'Each pair changes only the root move; all later seats use deployed L1.',
             'The same frozen own/public decision is reused wherever its input repeats.', '']
    selected = {('miss-advantage-27', 'gain'), ('miss-advantage-06', 'gain'),
                ('miss-advantage-06', 'loss'), ('miss-advantage-20', 'loss')}
    for witness in report['witnesses']:
        if (witness['root'], witness['kind']) not in selected:
            continue
        req = witness['request']
        points, _, remaining, trick = replay_record(witness['hands'], req['plays'], req['decl'], req['bidder'])
        declaring = req['bidder'] % 2
        lines += [f'## {witness["root"]}: {witness["kind"]}, world {witness["world"]}', '',
                  f'Actor S{req["seat"]}; partner S{req["seat"] ^ 2}; declaring team parity {declaring}; '
                  f'called suit {req["decl"]}; bid 30.',
                  f'Banked scores before the current trick: {points}. '
                  f'Current plays: ' + ', '.join(f'S{s} {tile(t)}' for s, t in trick) + '.', '',
                  '| Seat | Remaining dominoes (examiner only) |', '|---|---|']
        lines += [f'| S{s} | ' + ', '.join(tile(t) for t in sorted(hand)) + ' |'
                  for s, hand in enumerate(remaining)]
        for name in ('baseline', 'offer'):
            row = witness[name]
            lines += ['', f'**{name}: first play {tile(row["action_taken"])}; '
                      f'declaring score {row["points"][declaring]}, '
                      f'{"make" if row["made"] else "set"}.**', '',
                      '| Completed trick | Plays in order | Winner | Trick points |', '|---|---|---|---|']
            start = len(req['plays']) // 8 * 8
            for index in range(start, 56, 8):
                plays = list(zip(row['record'][index:index+8:2], row['record'][index+1:index+8:2]))
                lines.append(f'| {index // 8 + 1} | ' + ', '.join(f'S{s} {tile(t)}' for s, t in plays)
                             + f' | S{winner(plays, req["decl"])} | {trick_points(plays)} |')
        lines += ['']
    (HERE / 'WITNESSES.md').write_text('\n'.join(lines).rstrip() + '\n')
    print(json.dumps(dict(published=str(HERE), artifacts_verified=len(report['artifact_hashes']),
                          durable_files_unchanged=len(interrupted['durable_files']))))


if __name__ == '__main__':
    main()
