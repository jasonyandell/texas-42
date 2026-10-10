#!/usr/bin/env python3
"""DEPRECATED shim: train_scale.py was folded into train.py (--opt adamw, resumable rounds). This translates the old
command line (old defaults: --arch mlp2 --inner 512 --total 100000 --batch 2048 --lr 1e-3 --lr-end 1e-5 --warmup 1000
--wd .01 --eval-every 5000 --seconds 240, no l2, selection by val regret, state saved every round).
Differences: <name>.w is now the best-regret net (it was the last weights; the last weights are <name>-last.w and the
old <name>-best.w is no longer written), the state lives in checkpoints/ (it was a session scratchpad path), the
train-side metric subset is drawn from all train rows, a fresh run's data order continues the init rng, and
`path@K` means a fixed random K rows (the old meaning, K rows by stride, is now `path@K:stride`)."""
import argparse, sys
import train
if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('--train', required=True); ap.add_argument('--val', required=True); ap.add_argument('--val2', default=''); ap.add_argument('--name', required=True)
    ap.add_argument('--arch', default='mlp2', choices=['mlp1', 'mlp2', 'res']); ap.add_argument('--hidden', default='256'); ap.add_argument('--hidden2', default='256')
    ap.add_argument('--inner', default='512'); ap.add_argument('--blocks', default='2'); ap.add_argument('--ln', default='0')
    ap.add_argument('--loss', default='mix', choices=['mix', 'tce']); ap.add_argument('--bw', default='1.0'); ap.add_argument('--tau', default='1.0'); ap.add_argument('--eps', default='0.0')
    ap.add_argument('--total', default='100000'); ap.add_argument('--batch', default='2048'); ap.add_argument('--lr', default='1e-3'); ap.add_argument('--lr-end', default='1e-5'); ap.add_argument('--warmup', default='1000')
    ap.add_argument('--wd', default='0.01'); ap.add_argument('--b2', default='0.999'); ap.add_argument('--dropout', default='0.0'); ap.add_argument('--eval-every', default='5000'); ap.add_argument('--seconds', default='240.0')
    ap.add_argument('--resume', action='store_true'); ap.add_argument('--seed', default='12345'); ap.add_argument('--stop-at', default='0')
    a, passthrough = ap.parse_known_args()  # unknown flags (e.g. --out-dir, --device) go to train.py as given
    arch, layers = {'mlp1': ('mlp', '1'), 'mlp2': ('mlp', '2'), 'res': ('res', '2')}[a.arch]
    fix = lambda s: ','.join(p + ':stride' if '@' in p else p for p in s.split(',') if p)  # old @K = stride
    argv = ['--opt', 'adamw', '--arch', arch, '--layers', layers, '--train', fix(a.train), '--val', fix(a.val), '--val2', fix(a.val2), '--name', a.name, '--hidden', a.hidden, '--hidden2', a.hidden2,
            '--inner', a.inner, '--blocks', a.blocks, '--ln', a.ln, '--loss', a.loss, '--bw', a.bw, '--tau', a.tau, '--eps', a.eps, '--updates', a.total, '--batch', a.batch, '--lr', a.lr,
            '--lr-end', a.lr_end, '--warmup', a.warmup, '--wd', a.wd, '--b2', a.b2, '--dropout', a.dropout, '--eval-every', a.eval_every, '--seconds', a.seconds, '--seed', a.seed,
            '--stop-at', a.stop_at, '--l2', '0', '--select', 'regret', '--state', '--save-last'] + (['--resume'] if a.resume else [])
    argv += passthrough
    print('train_scale.py is deprecated: forwarding to train.py ' + ' '.join(argv), file=sys.stderr)
    train.main(argv)
