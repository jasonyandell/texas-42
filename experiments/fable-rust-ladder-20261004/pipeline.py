#!/usr/bin/env python3
"""Continuous rung-2 distillation pipeline: labeling, training and h2h overlap instead of running in sequence.

Stages (all heavy work under the 295 s cap wrapper; this driver is a light supervisor that polls every 2 s):
  labeling  one capped round of `ladder log2` at a time on --label-workers cores (last worker = validation),
            student-driven by the newest net that was ready when the round was launched;
  training  as soon as a round lands, `train.py` on the GPU over everything labeled so far (base data + all rounds);
            then the Rust `agree` check (must match the trainer's val agreement within .01; --trainer stu also regret within .001);
  h2h       every net that passes goes to `h2h_fast.py` (production memo, --h2h-workers cores) while labeling continues.
Default (`--lag`): the next round starts the moment the previous one lands, so the student it follows lags one net
behind (the runbook's strict order label(k) -> train(k+1) -> label with k+1 is `--strict`, which idles the label
cores while training). Everything is resumable: rerun the same command and it continues from the files on disk.

Files: data/labels2-<name>-r<r>-w<w>.bin (parts), data/labels2-<name>-{train,val}.bin (growing, append-only),
models/<name>-k<k>.{w,json}, results/h2h-np-<name>-k<k>-s<seed>-<deals>/, pipeline/<name>/log.jsonl (events).
"""
import argparse, json, os, shutil, subprocess, sys, time
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; PY = sys.executable
B = Path(os.environ.get('LADDER_BIN', HERE / 'ladder/target-s/release/ladder')); D = HERE / 'data'

ap = argparse.ArgumentParser()
ap.add_argument('--name', required=True); ap.add_argument('--net', default='', help='initial student net (.w) for the first rounds; empty = teacher-driven rounds until the first net is ready')
ap.add_argument('--train-base', default=''); ap.add_argument('--val-base', default='')
ap.add_argument('--label-workers', type=int, default=12); ap.add_argument('--h2h-workers', type=int, default=4)
ap.add_argument('--outer', type=int, default=40); ap.add_argument('--inner', type=int, default=8); ap.add_argument('--eps', default='0.1'); ap.add_argument('--seed-base', type=int, default=9_000_000); ap.add_argument('--stop-at', type=int, default=288)
ap.add_argument('--hidden', type=int, default=128); ap.add_argument('--hidden2', type=int, default=128); ap.add_argument('--layers', type=int, default=2); ap.add_argument('--updates', type=int, default=30000); ap.add_argument('--batch', type=int, default=1024); ap.add_argument('--enc', type=int, default=4); ap.add_argument('--loss', default='mix'); ap.add_argument('--eval-every', type=int, default=500, help='trainer validation interval (the growing val file makes each pass costlier)'); ap.add_argument('--init-prev', action='store_true', help='initialize each net from the newest ready net (k0 from --net) instead of from scratch: continual fine-tuning'); ap.add_argument('--lr', default='0.002')
ap.add_argument('--rounds', type=int, default=1000, help='labeling rounds to run'); ap.add_argument('--nets', type=int, default=1000, help='stop after this many nets')
ap.add_argument('--h2h-deals', type=int, default=4096); ap.add_argument('--h2h-seed', type=int, default=5000); ap.add_argument('--no-h2h', action='store_true')
ap.add_argument('--strict', action='store_true', help='wait for the newly trained net before launching the next labeling round')
ap.add_argument('--mix', type=float, default=0.5, help='LAD6: fraction of games with a random bidder seat and rollout-chosen trump (log2 --mix; needs --bid-worlds)'); ap.add_argument('--bid-worlds', type=int, default=0, help='LAD6: rollout worlds for the bid level (0 = every game at bid 30, the LAD1-5 protocol)')
ap.add_argument('--bid-uniform', type=float, default=0.1, help='LAD6: probability a game\'s bid level is uniform over 30..42 (log2 --bid-uniform)')
ap.add_argument('--outer-val', type=int, default=0, help='LAD6: outer worlds for the validation worker (0 = --outer); train rows may use fewer worlds than the gate')
ap.add_argument('--h2h-floor', type=float, default=None, help='LAD6 hard gate: a net that passes the regret gate is promoted only after its np h2h vs production (seed --h2h-seed) scores >= this; until then it is pending_h2h and not the teacher')
ap.add_argument('--enumerate', type=int, default=0, help='log2 --enumerate CAP: exact outer ring when the support has <= CAP hidden deals (0 = off)')
ap.add_argument('--ladder', action='store_true', help="rung-3 ladder: each round's teacher is the --outer-world search with the newest ready net as the modeled rung-1 others (log2 --model NET, as log2_launch.py does) and the same net drives the states (--play np:NET); the first rounds use --net")
ap.add_argument('--gate', action='store_true', help='promotion gate: a new net becomes teacher/student (status ready) only if its Rust selector agreement on the current val file is >= the current teacher net\'s agreement on the same file; otherwise status rejected (no h2h, teacher unchanged)')
ap.add_argument('--h2h-search', action='store_true', help='after each passing net\'s np: h2h, also h2h the search player l2n:40:<net> at --h2h-deals on the same workers')
ap.add_argument('--h2h-search-outer', type=int, default=40, help='outer worlds of the l2n search arm (default 40 = the old l2n:40 arm)')
ap.add_argument('--trainer', default='train', choices=['train', 'stu'], help="both run train.py. train = the original rung-2 recipe (mix/bce loss, constant lr, selection by val agreement, or val BCE for --loss bce, from scratch unless --init-prev); stu = the LAD5 student recipe (mlp, --stu-args: regret loss, cosine lr, selection by val regret), initialized from the newest promoted net (.npz checkpoint when present)")
ap.add_argument('--extra-train', default='', help='stu only: comma-separated fixed label files appended to the train set every call (e.g. distillation piles data/STU-distill-LAD2-*.bin, strong-teacher piles)')
ap.add_argument('--stu-args', default='--loss regret --rw 20 --tau 0.5 --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --seconds 200', help='stu only: train.py optimisation args (the --seconds wall stop keeps each call inside one 295 s capped round; continuity across rounds comes from --init = newest promoted net)')
ap.add_argument('--gate-metric', default='agree', choices=['agree', 'regret', 'bce'], help='with --gate: agree = promote iff Rust agreement >= teacher (old); regret = promote iff Rust-side regret on the current val file <= the teacher net\'s regret (lower is better; rust_regret.py over `agree --dump`); bce = promote iff Rust-side masked BCE of the pmake vector <= the teacher\'s (LAD7: faithfulness to the teacher\'s vector, not its argmax)')
ap.add_argument('--select', default='regret', choices=['regret', 'agree', 'bce'], help='stu only: checkpoint selection metric passed to train.py --select (LAD7: bce)')
a = ap.parse_args()
P = HERE / 'pipeline' / a.name; P.mkdir(parents=True, exist_ok=True); LOG = P / 'log.jsonl'; STATE = P / 'state.json'
TR = D / f'labels2-{a.name}-train.bin'; VA = D / f'labels2-{a.name}-val.bin'
T0 = time.time()

def log(**kv):
    kv = dict(t=round(time.time(), 1), elapsed=round(time.time() - T0, 1), **kv); print(json.dumps(kv), flush=True)
    with open(LOG, 'a') as f: f.write(json.dumps(kv) + '\n')

def load_state():
    if STATE.exists(): return json.loads(STATE.read_text())
    return dict(rounds_started=0, rounds_landed=[], nets=[], initial_net=a.net, pending_rounds=[], h2h_queue=[], h2h_done=[])
def save_state(): STATE.write_text(json.dumps(S, indent=1))
S = load_state()
if not TR.exists():
    for src, dst in ((a.train_base, TR), (a.val_base, VA)):
        if src: shutil.copyfile(src, dst)
        else: dst.touch()
    log(event='init', train_rows=TR.stat().st_size // 100, val_rows=VA.stat().st_size // 100)

def rows(p): return p.stat().st_size // 100 if p.exists() else 0
def newest_net():
    ok = [n for n in S['nets'] if n.get('status') == 'ready']
    return ok[-1]['w'] if ok else S['initial_net']
def part(r, w): return D / f'labels2-{a.name}-r{r}-w{w:02d}.bin'
def logdir(r, w): return HERE / 'results' / f'log2-{a.name}-r{r}-w{w:02d}-log'

def launch_round(r):
    net = newest_net(); procs = []
    for w in range(a.label_workers):
        ld = logdir(r, w); k = 0
        while ld.exists(): k += 1; ld = HERE / 'results' / f'log2-{a.name}-r{r}-w{w:02d}-{k}-log'
        cmd = [PY, str(CAP), '--seconds', '295', '--output-dir', str(ld), '--', str(B), 'log2', '--seed', str(a.seed_base + 100_000 * r), '--inner', str(a.inner), '--outer', str(a.outer_val if (a.outer_val and w == a.label_workers - 1) else a.outer), '--eps', a.eps, '--worker', str(w), '--workers', str(a.label_workers), '--out', str(part(r, w)), '--stop-at', str(a.stop_at)]
        if net: cmd += ['--play', f'np:{net}']
        if a.bid_worlds: cmd += ['--mix', str(a.mix), '--bid-worlds', str(a.bid_worlds), '--bid-uniform', str(a.bid_uniform)]  # LAD6 deal mixture (log2 VARIANT)
        if a.enumerate: cmd += ['--enumerate', str(a.enumerate)]  # exact outer average when the support is small (late tricks)
        if a.ladder:
            assert net, '--ladder needs a net (--net for the first rounds)'
            cmd += ['--model', net]
        procs.append(subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    log(event='round_start', round=r, net=net, workers=a.label_workers, teacher=('search+net' if a.ladder else 'sampled'))
    return dict(r=r, net=net, procs=procs, t0=time.time())

def land_round(L):
    r = L['r']; before = (rows(TR), rows(VA)); n = 0
    for w in range(a.label_workers):
        p = part(r, w)
        if not p.exists(): continue
        data = p.read_bytes(); data = data[:len(data) - len(data) % 100]; n += len(data) // 100
        with open(VA if w == a.label_workers - 1 else TR, 'ab') as f: f.write(data)
    S['rounds_landed'].append(dict(r=r, net=L['net'], states=n, wall=round(time.time() - L['t0'], 1))); S['pending_rounds'].append(r); save_state()
    log(event='round_landed', round=r, states=n, wall=round(time.time() - L['t0'], 1), train_rows=rows(TR), val_rows=rows(VA), added=(rows(TR) - before[0], rows(VA) - before[1]))

def launch_train(k):
    name = f'{a.name}-k{k}'; ld = HERE / 'results' / f'train-{name}-log'; i = 0; init_net = newest_net() or a.net
    while ld.exists(): i += 1; ld = HERE / 'results' / f'train-{name}-{i}-log'
    if a.trainer == 'stu':
        init = ''
        if init_net:
            npz = (HERE / init_net).with_suffix('.npz'); init = str(npz) if npz.exists() else str(HERE / init_net)
        trs = ','.join([str(TR)] + [x for x in a.extra_train.split(',') if x])
        cmd = [PY, str(CAP), '--seconds', '295', '--output-dir', str(ld), '--', PY, str(HERE / 'train.py'), '--train', trs, '--val', str(VA), '--name', name, '--arch', 'mlp', '--hidden', str(a.hidden), '--hidden2', str(a.hidden2), '--layers', str(a.layers), '--enc', str(a.enc), '--batch', str(a.batch), '--select', a.select] + a.stu_args.split() + (['--init', init] if init else [])
        proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        log(event='train_start', k=k, name=name, trainer='stu', init=init, train_rows=rows(TR), extra=a.extra_train, val_rows=rows(VA), rounds=list(S['pending_rounds']))
        return dict(k=k, name=name, proc=proc, t0=time.time(), rounds=list(S['pending_rounds']), logdir=ld)
    cmd = [PY, str(CAP), '--seconds', '295', '--output-dir', str(ld), '--', PY, str(HERE / 'train.py'), '--train', str(TR), '--val', str(VA), '--hidden', str(a.hidden), '--hidden2', str(a.hidden2), '--layers', str(a.layers), '--updates', str(a.updates), '--batch', str(a.batch), '--eval-every', str(a.eval_every), '--name', name, '--enc', str(a.enc), '--loss', a.loss, '--lr', a.lr, '--select', 'bce' if a.loss == 'bce' else 'agree', '--seconds', '280'] + (['--init', init_net] if a.init_prev and init_net else [])
    proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    log(event='train_start', k=k, name=name, train_rows=rows(TR), val_rows=rows(VA), rounds=list(S['pending_rounds']))
    return dict(k=k, name=name, proc=proc, t0=time.time(), rounds=list(S['pending_rounds']), logdir=ld)

def finish_train(T):
    name = T['name']; meta = HERE / 'models' / f'{name}.json'; w = HERE / 'models' / f'{name}.w'; entry = dict(k=T['k'], name=name, w=str(w.relative_to(HERE)), rounds=T['rounds'], train_seconds=round(time.time() - T['t0'], 1))
    if not meta.exists():
        entry['status'] = 'train_failed'; S['nets'].append(entry); save_state(); log(event='train_failed', **entry); return
    if a.trainer == 'stu': return finish_train_stu(T, entry, json.loads(meta.read_text()), w)
    m = json.loads(meta.read_text()); sel = m['selected']; entry.update(val_rmse=sel['rmse'], val_agree=sel['agree'], selected_update=m['selected_update'], train_rows=m['train_rows'])
    ag = json.loads(subprocess.run([str(B), 'agree', '--net', str(w), '--labels', str(VA), '--limit', str(rows(VA))], capture_output=True, text=True).stdout)  # whole val file: the trainer's number is over all rows, and the default --limit 20000 would read only the oldest part
    entry['rust_agree'] = ag['agree']; entry['status'] = 'ready' if abs(ag['agree'] - sel['agree']) <= .01 else 'agree_mismatch'
    if a.gate and entry['status'] == 'ready':
        cur = newest_net(); agc = json.loads(subprocess.run([str(B), 'agree', '--net', str(HERE / cur), '--labels', str(VA), '--limit', str(rows(VA))], capture_output=True, text=True).stdout)['agree']
        entry['teacher'] = cur; entry['teacher_agree'] = agc
        if ag['agree'] < agc: entry['status'] = 'rejected'
    S['nets'].append(entry)
    if entry['status'] == 'ready' and not a.no_h2h:
        S['h2h_queue'].append([T['k'], 'np'])
        if a.h2h_search: S['h2h_queue'].append([T['k'], 'l2n'])
    save_state(); log(event='net', **entry)

def rust_eval(w):
    """Rust-side agreement and regret of net file w on the current val file (`agree --dump` + rust_regret.py)."""
    dump = P / 'dump.jsonl'
    subprocess.run([str(B), 'agree', '--net', str(w), '--labels', str(VA), '--limit', str(rows(VA)), '--dump', str(dump)], capture_output=True, text=True)
    r = json.loads(subprocess.run([PY, str(HERE / 'rust_regret.py'), str(VA), str(dump)], capture_output=True, text=True, cwd=str(HERE)).stdout); dump.unlink()
    return r

def finish_train_stu(T, entry, m, w):
    sel = m['selected']; entry.update(val_agree=sel['agree'], val_regret=sel['regret'], val_zero_regret=sel['zero_regret'], selected_update=m['selected_update'], train_rows=m['train_rows'], val_rows=m['val_rows'])
    r = rust_eval(w); entry.update(rust_agree=r['agree'], rust_regret=r['regret'], rust_bce=r.get('bce'), rust_rmse=r.get('rmse'), rust_rows=r['rows'])
    # the trainer read VA when it started; rows appended since are not in its number, so compare only when the row counts match
    same = r['rows'] == m['val_rows']
    entry['status'] = 'ready' if (not same or (abs(r['agree'] - sel['agree']) <= .01 and abs(r['regret'] - sel['regret']) <= .001)) else 'agree_mismatch'
    if a.gate and entry['status'] == 'ready':
        cur = newest_net(); rc = rust_eval(HERE / cur); entry.update(teacher=cur, teacher_agree=rc['agree'], teacher_regret=rc['regret'], teacher_bce=rc.get('bce'))
        better = (r['regret'] <= rc['regret']) if a.gate_metric == 'regret' else (r['bce'] <= rc['bce']) if a.gate_metric == 'bce' else (r['agree'] >= rc['agree'])
        if not better: entry['status'] = 'rejected'
    if a.h2h_floor is not None and entry['status'] == 'ready': entry['status'] = 'pending_h2h'  # promoted only by finish_h2h
    S['nets'].append(entry)
    if entry['status'] in ('ready', 'pending_h2h') and not a.no_h2h:
        S['h2h_queue'].append([T['k'], 'np'])
        if a.h2h_search: S['h2h_queue'].append([T['k'], 'l2n'])
    save_state(); log(event='net', **entry)

def launch_h2h(item):
    k, kind = (item if isinstance(item, list) else [item, 'np'])
    n = next(n for n in S['nets'] if n['k'] == k)
    arm, tag = (f"np:{n['w']}", f"np-{n['name']}-s{a.h2h_seed}-{a.h2h_deals}") if kind == 'np' else (f"l2n:{a.h2h_search_outer}:{n['w']}", f"l2n-{a.h2h_search_outer}-{n['name']}-s{a.h2h_seed}-{a.h2h_deals}")
    cmd = [PY, str(HERE / 'h2h_fast.py'), '--net', arm, '--seed', str(a.h2h_seed), '--deals', str(a.h2h_deals), '--workers', str(a.h2h_workers), '--tag', tag, '--voids']
    proc = subprocess.Popen(cmd, stdout=open(P / f'h2h-k{k}-{kind}.out', 'a'), stderr=subprocess.STDOUT, env=dict(os.environ, LADDER_BIN=str(B)))
    log(event='h2h_start', k=k, arm=arm, tag=tag, workers=a.h2h_workers)
    return dict(k=k, kind=kind, tag=tag, proc=proc, t0=time.time())

def finish_h2h(H):
    f = HERE / 'results' / f"h2h-{H['tag']}" / 'summary.json'; s = json.loads(f.read_text()) if f.exists() else {}
    S['h2h_done'].append([H['k'], H['kind']])
    if H['kind'] == 'np' and a.h2h_floor is not None:
        for n in S['nets']:
            if n['k'] == H['k'] and n.get('status') == 'pending_h2h':
                adv = s.get('mean_paired_advantage'); n['h2h_adv'] = adv; n['status'] = 'ready' if adv is not None and adv >= a.h2h_floor else 'rejected_h2h'
                log(event='h2h_gate', k=H['k'], adv=adv, floor=a.h2h_floor, status=n['status'])
    save_state()
    log(event='h2h', k=H['k'], arm=H['kind'], tag=H['tag'], wall=round(time.time() - H['t0'], 1), deals_paired=s.get('deals_paired'), adv=s.get('mean_paired_advantage'), ci=s.get('ci95'), win=s.get('hybrid_overall_win_rate'), cached=(s.get('prod_cache') or {}).get('native_cached'))

labeling = trainer = h2h = None
log(event='start', args=vars(a), binary=str(B), state=dict(rounds_started=S['rounds_started'], nets=len(S['nets'])))
while True:
    # labeling
    if labeling is None and S['rounds_started'] < a.rounds and len(S['nets']) < a.nets:
        waiting = a.strict and (trainer is not None or S['pending_rounds'])
        if not waiting:
            labeling = launch_round(S['rounds_started']); S['rounds_started'] += 1; save_state()
    elif labeling is not None and all(p.poll() is not None for p in labeling['procs']):
        if trainer is None or time.time() - trainer['t0'] > 60:  # never append while a trainer may still be loading the files
            land_round(labeling); labeling = None
    # training
    if trainer is None and S['pending_rounds'] and len(S['nets']) < a.nets:
        trainer = launch_train(len(S['nets'])); S['pending_rounds'] = []; save_state()
    elif trainer is not None and trainer['proc'].poll() is not None:
        finish_train(trainer); trainer = None
    # h2h
    if h2h is None and S['h2h_queue']:
        h2h = launch_h2h(S['h2h_queue'].pop(0)); save_state()
    elif h2h is not None and h2h['proc'].poll() is not None:
        finish_h2h(h2h); h2h = None
    done_rounds = S['rounds_started'] >= a.rounds or len(S['nets']) >= a.nets
    if done_rounds and labeling is None and trainer is None and h2h is None and not S['h2h_queue'] and (not S['pending_rounds'] or len(S['nets']) >= a.nets):
        log(event='done', rounds=S['rounds_started'], nets=len(S['nets'])); break
    time.sleep(2)
