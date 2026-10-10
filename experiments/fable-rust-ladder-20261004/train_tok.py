#!/usr/bin/env python3
"""DEPRECATED shim: train_tok.py was folded into train.py (--arch tok --opt mlx-adamw). This translates the old
command line (old defaults: --d 32 --layers 1 --heads 2 --dff 64 --lr 2e-3 --l2 0 --warm 300 --seed 1
--select agree --prefix STU-tokens-, --train as space-separated files, val default data/labels2-LAD4-val.bin).
Differences: the loss is train.py's (same math, different float association), regret is v_best - v_pick, --seconds
counts training time only, and --dump (logits in the .json) is gone (use train.py --eval-only --dump-logits)."""
import argparse, sys
from pathlib import Path
import train
if __name__ == '__main__':
    ap = argparse.ArgumentParser(); ap.add_argument('--train', nargs='+', default=[]); ap.add_argument('--val', default=str(Path(train.HERE) / 'data/labels2-LAD4-val.bin')); ap.add_argument('--name', required=True)
    ap.add_argument('--d', default='32'); ap.add_argument('--layers', default='1'); ap.add_argument('--heads', default='2'); ap.add_argument('--dff', default='64')
    ap.add_argument('--updates', default='20000'); ap.add_argument('--batch', default='1024'); ap.add_argument('--lr', default='2e-3'); ap.add_argument('--l2', default='0.0')
    ap.add_argument('--eval-every', default='1000'); ap.add_argument('--loss', default='mix'); ap.add_argument('--init', default=''); ap.add_argument('--seconds', default='262'); ap.add_argument('--warm', default='300')
    ap.add_argument('--dump', default='100'); ap.add_argument('--wd', default='0.0'); ap.add_argument('--seed', default='1'); ap.add_argument('--select', default='agree'); ap.add_argument('--prefix', default='STU-tokens-'); ap.add_argument('--relabel', default='')
    a, passthrough = ap.parse_known_args()  # unknown flags (e.g. --out-dir, --device) go to train.py as given
    argv = ['--arch', 'tok', '--opt', 'mlx-adamw', '--train', ','.join(a.train), '--val', a.val, '--name', a.prefix + a.name, '--d', a.d, '--layers', a.layers, '--heads', a.heads, '--dff', a.dff,
            '--updates', a.updates, '--batch', a.batch, '--lr', a.lr, '--l2', a.l2, '--eval-every', a.eval_every, '--loss', a.loss, '--seconds', a.seconds, '--warmup', a.warm, '--wd', a.wd, '--seed', a.seed, '--select', a.select]
    argv += (['--init', a.init] if a.init else []) + (['--relabel', a.relabel] if a.relabel else [])
    argv += passthrough
    print('train_tok.py is deprecated: forwarding to train.py ' + ' '.join(argv), file=sys.stderr)
    train.main(argv)
