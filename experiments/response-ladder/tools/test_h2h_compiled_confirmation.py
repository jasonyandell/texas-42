import copy
import importlib
import json
import sys
import tempfile
from argparse import Namespace
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import h2h_compiled as mod

ROOT = Path.cwd()
mod.HERE = ROOT
PROTOCOL_PATH = ROOT / 'results/compiled-v5-confirmation/protocol.json'
PROTOCOL = json.loads(PROTOCOL_PATH.read_text())
CPU_ROOT = Path(mod.CPU_ROOT_DEFAULT)
sys.path.insert(0, str(CPU_ROOT / 'experiments/partnership'))
import player as cpu

COMMAND = [str(ROOT / 'target/release/compiled-player'), 'worker',
           '--c0', str(ROOT / 'results/compiled-c0-v5/actor.json'),
           '--c1', str(ROOT / 'results/compiled-c1-v5/actor.json')]
C0 = ROOT / 'results/compiled-c0-v5/actor.json'
C1 = ROOT / 'results/compiled-c1-v5/actor.json'
IDENTITIES = mod.source_identities(CPU_ROOT, cpu, COMMAND, C0, C1)
ARGS = Namespace(panel=PROTOCOL['panels'][0], deals_per_cell=PROTOCOL['deals_per_cell'],
                 candidate_ms=PROTOCOL['candidate_ms'], threads=PROTOCOL['threads'])
META = {'path': str(PROTOCOL_PATH.resolve()), 'sha256': mod.digest(PROTOCOL_PATH),
        'schema': PROTOCOL['schema'], 'id': PROTOCOL['id']}


def rejects(call, text):
    try:
        call()
    except SystemExit as exc:
        assert text in str(exc), (text, exc)
    else:
        raise AssertionError(f'accepted invalid confirmation protocol: {text}')


def main():
    path, loaded = mod.load_confirmation_protocol(str(PROTOCOL_PATH))
    assert path == PROTOCOL_PATH.resolve()
    mod.validate_confirmation_protocol(loaded, ARGS, loaded['candidate_config'], IDENTITIES)
    rejects(lambda: mod.validate_confirmation_protocol({**loaded, 'cpu_ms': mod.CPU_MS + 1}, ARGS, loaded['candidate_config'], IDENTITIES), 'cpu_ms drift')
    bad = copy.deepcopy(loaded); bad['fixture_hashes'][ARGS.panel] = '0' * 64
    rejects(lambda: mod.validate_confirmation_protocol(bad, ARGS, loaded['candidate_config'], IDENTITIES), 'fixture hash drift')
    bad = copy.deepcopy(loaded); bad['authority_document']['sha256'] = '0' * 64
    rejects(lambda: mod.validate_confirmation_protocol(bad, ARGS, loaded['candidate_config'], IDENTITIES), 'authority document drift')
    bad = copy.deepcopy(loaded); bad['schema'] = 'wrong'
    with tempfile.TemporaryDirectory() as td:
        bad_path = Path(td) / 'bad.json'; bad_path.write_text(json.dumps(bad))
        rejects(lambda: mod.load_confirmation_protocol(str(bad_path)), 'schema')
    protocol = {'identities': IDENTITIES, 'config_source': {'path': None, 'sha256': None},
                'panel': ARGS.panel, 'deals_per_cell': ARGS.deals_per_cell,
                'candidate_ms': ARGS.candidate_ms, 'threads': ARGS.threads,
                'confirmation_protocol': META}
    rejects(lambda: mod.verify_frozen(ROOT, protocol, ARGS, IDENTITIES,
                                      {'path': None, 'sha256': None}, None), 'mode drift')
    development_protocol = dict(protocol); development_protocol.pop('confirmation_protocol')
    rejects(lambda: mod.verify_frozen(ROOT, development_protocol, ARGS, IDENTITIES,
                                      {'path': None, 'sha256': None}, META), 'mode drift')
    with tempfile.TemporaryDirectory() as td:
        out = Path(td) / 'analysis'; out.mkdir()
        stored = {'fixtures': mod.fixture_panel(ARGS.panel, 1), 'confirmation_protocol': META,
                  'data_role': 'CONFIRMATION', 'final_holdout': PROTOCOL['id'],
                  'scope': 'frozen confirmation only', 'promotion': 'none'}
        summary = mod.analyze(out, stored, importlib.import_module('rules'))
        assert summary['development_only'] is False
        assert summary['data_role'] == 'CONFIRMATION'
        assert summary['final_holdout'] == PROTOCOL['id']
        dev_summary = mod.analyze(out, {'fixtures': mod.fixture_panel(ARGS.panel, 1)}, importlib.import_module('rules'))
        assert dev_summary['development_only'] is True
        assert 'data_role' not in dev_summary
        rejects(lambda: mod.main(['--output', str(out), '--analyze-only',
                                  '--confirmation-protocol', str(PROTOCOL_PATH)]), 'analyze-only')
    print('confirmation protocol validation, fixture/document/cap pins, resume mode guards, and analyze-only guard passed')


if __name__ == '__main__':
    main()
