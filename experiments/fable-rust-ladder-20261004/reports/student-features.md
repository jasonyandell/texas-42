# STU-features (knowledge features) report -- INCOMPLETE: Rust side blocked

Tier: exploratory (trainer-side numbers only). Worktree: /Users/jason/Documents/Codex/2026-10-04/task-11/research-worktree/.claude/worktrees/agent-aa99f20640cac7e1b/experiments/fable-rust-ladder-20261004

## Design
- feats.py (numpy) / ladder/src/feats.rs (Rust twin, written, NOT compiled): 23 integer per-tile features (cur_win, sure_win, n_threat, top_of_suit, stronger_unseen, trump, points, trick value, follows/ruff, overtakes partner, singleton, suit counts, void-in-contested-suit per other seat, lowest/top own in suit, points donated/added) + 66 globals. u8 values scaled by f32 1/den so Python and Rust inputs are identical.
- Net header tag 5: [5, layers, H, H2, hybrid, NF]; net.rs `scores_feat` path; existing files unchanged. `ladder featdump` writes u8 features for parity vs feats.py.
- train_feat.py: modes hybrid (enc-4 embedding sum + F@wf) and pure (F@wf only), cosine lr, selection by val agreement, permutation ablation (--ablate).

## Results (trainer side only; val = labels2-LAD4-val.bin, 45,850 rows)
Train = LAD4+LAD3+T160 = 1,524,616 rows, loss=mix, batch 1024, ~250 s unless noted.

| variant | params | trainer agree | regret | Rust agree | h2h | us/dec |
|---|---|---|---|---|---|---|
| control enc-4 h128x2 (train.py, mix, 36k upd) | ~100k | .633 | n/a | not run | not run | not run |
| hybrid enc4+features h128x2 (models/STU-features-hybrid128) | 300,956 | .6576 | .0149 | not run | not run | not run |
| pure features h128x2 (models/STU-features-pure128) | 111,132 | .6664 | .0144 | not run | not run | not run |
| enc-4 h128x2, LAD4-train only, 24k upd, bce | ~100k | .453 | n/a | - | - | - |
| hybrid, LAD4-train only, 24k upd, bce | 300,956 | .487 | .0201 | - | - | - |

Baseline LAD4-k17: Rust .6469, alone -.060. From-scratch feature nets exceed it on trainer agreement by 1-2 pts, but the matched control (.633) shows loss/data matter as much; pure beats hybrid. 64 -> 67%: not a plateau-breaker; curve still rising.

Ablation (hybrid, permutation on val, agreement drop): team flag .101, cur_win .037, n_threat .037, singleton .023, void flags .023, sure_win .021, win_partner .020, is_trump .018. Retrain without top 3 not run.

## Not done (blocked)
`cargo build --release --offline` into target-x was DENIED by the auto-mode permission classifier ("Interfere With Workloads"); a copy of MAIN's gym/ dir was also denied (used walt/gym from my own worktree). So: no Rust compile, no feature parity check, no `ladder agree`, no h2h, no us/dec. Rust code untested.

## Commands
run_capped.py --seconds 295 --output-dir <dir> -- python3 train_feat.py --train D/labels2-LAD4-train.bin,D/labels2-LAD3-train.bin,D/labels2-T160-train.bin --val D/labels2-LAD4-val.bin --name p3pure --mode pure --updates 40000 --eval-every 5000 --loss mix
(hybrid: --name p2 --updates 60000 --ablate; time-limited at 36k)

## Recommendation
Promising, unproven. Unblock the build; run parity + agree + h2h; then train longer / 256 wide.
