#!/usr/bin/env python3
"""Rust/trainer parity check: for one net per weight-file kind, score the first --rows rows of a val file with the
Rust loader (`ladder agree --dump`) and with train.py's MLX forward (`--eval-only --dump-logits`, CPU device), and
assert the pipeline's tolerances: |agreement difference| <= .01 and |regret difference| <= .001. Also reports how
many picks match and the largest legal-logit difference (informational). Exits 1 on any failure.

Run under the cap:  run_capped.py --seconds 295 --output-dir results/parity-log -- $PY check_parity.py
Binary: $LADDER_BIN, default ladder/target-c/release/ladder (build: CARGO_TARGET_DIR=ladder/target-c cargo build --release).
Synthetic random-weight nets cover every header kind (MLP encodings 1-4 with one and two layers, residual with and
without LayerNorm, tokens), so the check is complete from a release checkout; listed model files that are absent are
reported as skipped. Default rows: fixtures/labels2-LAD4-val-first100.rec (the first 100 rows of labels2-LAD4-val).
Belief heads (header [10, 1, ...], ladder/src/belief.rs): Rust `belief-eval --dump` logits vs train_belief.py's CPU forward
on fixtures/belief-prodopp-d-val-first100.rec (128-byte belief records) for the kept BEL files and one synthetic
random-weight belief net; tolerance |log-loss difference| <= 1e-4, max |logit difference| reported.
"""
import argparse, json, os, subprocess, sys, tempfile
from pathlib import Path
import numpy as np
import train as T
import train_belief as TB

HERE = Path(__file__).resolve().parent
NETS = [  # (label, file) -- one per header kind and encoding, plus the release models
    ('mlp enc1 1-layer', 'models/full-h32x1.w'), ('mlp enc1 2-layer', 'models/l0-h128x2.w'), ('mlp enc2', 'models/v2-h128x2-128.w'),
    ('mlp enc3', 'models/v3-h128x2-128.w'), ('mlp enc4 LAD4-k17', 'models/LAD4-k17.w'), ('mlp enc4 LAD5-k73', 'models/LAD5-k73.w'),
    ('res LN 3 blocks', 'models/STU-s-res512-r1.w'), ('res LN W4', 'models/STU-scale-W4-res512ln-40k.w'),
    ('tok 1 layer', 'models/STU-tokens-a-d32L1.w'), ('tok b3', 'models/STU-tokens-b3-d48L2.w'), ('tok r3-b8', 'models/STU-r3-tok-b8.w'),
]


def synthetic(tmp):
    """Random-weight nets for header kinds no kept file has."""
    rng = np.random.default_rng(0); out = []
    for label, arch, cfg in [('synthetic res no-LN', 'res', dict(enc=4, hidden=64, inner=96, blocks=2, ln=0)),
                             ('synthetic res LN', 'res', dict(enc=4, hidden=64, inner=32, blocks=1, ln=1)),
                             ('synthetic mlp enc4 1-layer', 'mlp', dict(enc=4, layers=1, hidden=64, hidden2=64)),
                             ('synthetic mlp enc4 2-layer', 'mlp', dict(enc=4, layers=2, hidden=64, hidden2=96)),
                             ('synthetic mlp enc3', 'mlp', dict(enc=3, layers=2, hidden=32, hidden2=32)),
                             ('synthetic mlp enc2', 'mlp', dict(enc=2, layers=1, hidden=32, hidden2=32)),
                             ('synthetic mlp enc1', 'mlp', dict(enc=1, layers=2, hidden=32, hidden2=32)),
                             ('synthetic tok', 'tok', dict(d=32, layers=1, heads=2, dff=64))]:
        p = {k: (rng.standard_normal(s) * (0.3 if len(s) == 2 else 0.1)).astype(np.float32) for k, s in T.layout(arch, cfg)}
        path = Path(tmp) / (label.replace(' ', '_') + '.w'); T.write_w(path, arch, cfg, p); out.append((label, str(path)))
    return out


BELIEF_NETS = [('belief BEL-k73-a', 'models/BEL-k73-a.w'), ('belief BEL-po-c', 'models/BEL-po-c.w'), ('belief BEL-po-d', 'models/BEL-po-d.w')]


def belief_checks(binary, tmp, rows_path, tol=1e-4):
    """Rust belief-eval --dump logits vs train_belief's numpy-loaded MLX CPU forward on the belief fixture."""
    rng = np.random.default_rng(1); c = dict(layers=2, hidden=64, hidden2=96)
    p = {k: (rng.standard_normal(s) * (0.3 if len(s) == 2 else 0.1)).astype(np.float32) for k, s in TB.layout(c)}
    syn = Path(tmp) / 'synthetic_belief.w'; TB.write_w(syn, c, p); out = []
    for label, net in BELIEF_NETS + [('synthetic belief 64x96', str(syn))]:
        net = str(HERE / net) if not os.path.isabs(net) else net; res = dict(net=label, file=Path(net).name)
        if not os.path.exists(net): res.update(ok=True, skipped='file not present'); out.append(res); continue
        try:
            dump = Path(tmp) / 'bdump.jsonl'; r = subprocess.run([binary, 'belief-eval', '--net', net, '--data', rows_path, '--dump', str(dump)], capture_output=True, text=True)
            if r.returncode != 0: raise RuntimeError(r.stderr.strip()[-300:])
            zr = np.array([json.loads(l) for l in open(dump)]); rust_ll = json.loads(r.stdout)['all']['ll_belief']
            npy = str(Path(tmp) / 'bz.npy'); TB.main(['--eval-only', '--init', net, '--val', rows_path, '--dump-logits', npy, '--device', 'cpu'])
            zp = np.load(npy); d = TB.load(rows_path); py_ll = TB.metrics(zp, d)['ll']
            res.update(rust_ll=rust_ll, trainer_ll=py_ll, max_logit_diff=float(np.abs(zr - zp).max()), ok=abs(rust_ll - py_ll) <= tol and zr.shape == zp.shape)
        except Exception as e:
            res.update(ok=False, error=f'{type(e).__name__}: {e}')
        out.append(res)
    return out


def rust_dump(binary, net, labels, rows, dump):
    r = subprocess.run([binary, 'agree', '--net', net, '--labels', labels, '--limit', str(rows), '--dump', dump], capture_output=True, text=True)
    if r.returncode != 0: raise RuntimeError(' | '.join(l for l in r.stderr.strip().splitlines() if l.strip() and 'RUST_BACKTRACE' not in l) or 'agree failed')
    return [json.loads(l) for l in open(dump)]


def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--val', default=str(HERE / 'fixtures/labels2-LAD4-val-first100.rec')); ap.add_argument('--rows', type=int, default=100)
    ap.add_argument('--agree-tol', type=float, default=.01); ap.add_argument('--regret-tol', type=float, default=.001)
    a = ap.parse_args(); binary = os.environ.get('LADDER_BIN', str(HERE / 'ladder/target-c/release/ladder'))
    tmp = tempfile.mkdtemp(prefix='parity-'); labels = Path(tmp) / 'rows.bin'
    labels.write_bytes(Path(a.val).read_bytes()[:100 * a.rows])
    rec = np.fromfile(labels, dtype=T.REC); d = T.load_many(str(labels), 'mlp', T.E4); legal = d['legal'] > 0
    results = []; ok_all = True
    for label, net in NETS + synthetic(tmp):
        net = str(HERE / net) if not os.path.isabs(net) else net
        res = dict(net=label, file=os.path.relpath(net, HERE) if net.startswith(str(HERE)) else Path(net).name)
        if not os.path.exists(net):
            res.update(ok=True, skipped='file not present'); results.append(res); print(json.dumps(res), flush=True); continue
        try:
            dump = rust_dump(binary, net, str(labels), a.rows, str(Path(tmp) / 'dump.jsonl'))
            rc = np.array([x['choice'] for x in dump]); assert len(rc) == len(rec)
            npy = str(Path(tmp) / 'z.npy'); T.main(['--eval-only', '--init', net, '--val', str(labels), '--dump-logits', npy, '--device', 'cpu'])
            z = np.load(npy); pc = T.choose(z, d['legal'], d['maximize'])
            reg = lambda c: float((d['vbest'] - d['v'][np.arange(len(c)), c]).mean())
            ra, pa = float(np.mean(rc == d['choice'])), float(np.mean(pc == d['choice'])); rr, pr = reg(rc), reg(pc)
            dz = max(float(np.max(np.abs(np.array(x['scores']) - z[i][legal[i]]))) for i, x in enumerate(dump))
            ok = abs(ra - pa) <= a.agree_tol and abs(rr - pr) <= a.regret_tol
            res.update(rust_agree=ra, trainer_agree=pa, rust_regret=rr, trainer_regret=pr, picks_match=float(np.mean(rc == pc)), max_logit_diff=dz, ok=ok)
        except Exception as e:  # a net that fails to load or score is a failure, reported, not a crash
            res.update(ok=False, error=f'{type(e).__name__}: {e}')
        ok_all &= res['ok']; results.append(res); print(json.dumps(res), flush=True)
    for res in belief_checks(binary, tmp, str(HERE / 'fixtures/belief-prodopp-d-val-first100.rec')):
        ok_all &= res['ok']; results.append(res); print(json.dumps(res), flush=True)
    print(json.dumps(dict(rows=a.rows, val=a.val, binary=binary, nets=len(results), checked=sum('skipped' not in r for r in results), failed=[r['net'] for r in results if not r['ok']], ok=ok_all)))
    sys.exit(0 if ok_all else 1)


if __name__ == '__main__': main()
