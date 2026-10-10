#!/usr/bin/env python3
"""LAD7: function-preserving widening of a 2-layer MLP .npz (H1 x H2 -> H1' x H2').
usage: widen_net.py SRC.npz DST H1 H2 [seed]  -> DST.npz + DST.w (same encoding; new first-layer units get the
scratch init and feed only the new second-layer units; new second-layer units have zero output rows, so the
widened net computes exactly the source net's logits until training moves it)."""
import sys, numpy as np
sys.argv, args = sys.argv[:1], sys.argv[1:]
import train as T
src, dst, H1, H2 = args[0], args[1], int(args[2]), int(args[3]); seed = int(args[4]) if len(args) > 4 else 11
z = np.load(src); w1, b1, wm, bm, w2, b2 = (z[k] for k in ('w1', 'b1', 'wm', 'bm', 'w2', 'b2'))
I, h1 = w1.shape; h2 = wm.shape[1]; assert H1 >= h1 and H2 >= h2
rng = np.random.default_rng(seed)
W1 = np.zeros((I, H1), np.float32); W1[:, :h1] = w1; W1[:, h1:] = rng.standard_normal((I, H1 - h1)) * np.sqrt(2 / 32)
B1 = np.zeros(H1, np.float32); B1[:h1] = b1
WM = np.zeros((H1, H2), np.float32); WM[:h1, :h2] = wm
WM[:, h2:] = rng.standard_normal((H1, H2 - h2)) * np.sqrt(2 / H1)  # old and new first-layer units feed the new second-layer units
BM = np.zeros(H2, np.float32); BM[:h2] = bm
W2 = np.zeros((H2, 28), np.float32); W2[:h2] = w2  # new second-layer units start silent
p = dict(w1=W1, b1=B1, wm=WM, bm=BM, w2=W2, b2=b2.astype(np.float32))
enc = {1372 + 19 + 8: 2, 1372 + 27 + 84: 4, 1372 + 27 + 84 + 13: 5}[I]
cfg = dict(enc=enc, layers=2, hidden=H1, hidden2=H2)
np.savez(dst + '.npz', **p); T.write_w(dst + '.w', 'mlp', cfg, p)
print(dict(src=src, dst=dst, enc=enc, shape=(I, H1, H2), params=sum(v.size for v in p.values())))
