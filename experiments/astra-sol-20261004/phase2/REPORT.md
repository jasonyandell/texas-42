# Phase 2: safe substitution, actual cost, and recovered Fable work

2026-10-04. Exploratory research. The useful mathematical result is a sufficient
certificate for safely replacing one frozen modeled field with another. Its
current implementation is slower than solving the target on the retained
panels. The recovered Fable material supplies a useful exact-tail example, but
does not supply a verified production speed or strength improvement.

## Completed certificate and costs

For each root action and original scenario, traverse every legal focal branch
under the cheap field. At each nonfocal node compare completed cheap and target
field actions. Flag the entire scenario on the first difference. The absence
of a flag guarantees identical traces and payoffs for **every lawful focal
continuation**, because the certificate explores a superset of their paths.
World-specific branching is used only to bound an event; it does not evaluate
a world-specific optimal policy. Refusal/incomplete work never certifies.

If the flagged mass for action a is epsilon_a, then every lawful policy's
payoff changes by at most epsilon_a, and so does its attained maximum. Thus
old value minus epsilon is a lower bound and old value plus epsilon is an
upper bound. A candidate survives canonical least-tile selection only when
every lower-index competitor has strictly smaller upper bound and every
higher-index competitor has weakly smaller upper bound. These are sufficient,
potentially loose conditions on the **same weighted bundle and policy family**.

`math/sol/FirstDivergence.lean` proves the all-policy coupling premise missing
from phase 1. It formalizes arbitrary finite deterministic transitions,
observation-only focal policies, and a certificate expanding all legal focal
actions. Four theorems prove full trace/payoff equality, payoff equality,
contrapositive, and unflagged-world coupling. `math/sol/certificate_witness.py`
checks 256 field pairs, 16 lawful policies, weighted duplicates and refusal:
8,192 root/policy comparisons, 65 canonical certificates, zero false positives.

`probe/src/main.rs` is a separate Rust crate using the pinned Walt library;
the production code is unchanged. `run_cost.py` uses source deal 680000+i,
nine declarations, bidder i mod 4, random-legal complete-trick prefixes and a
40-world outer bundle. Eligibility (unsettled contract and a free choice) is
decided before probing. Input validity is established by the fixture audit;
the probe is not a hardened public request adapter.

| Frozen field comparison | Eligible | Certified | Cold certificate + fallback / fresh target |
|---|---:|---:|---:|
| L0 inner worlds 1 → 8 | 84 | 1 | 487000 / 288052 us = 1.691× |
| L0 inner worlds 4 → 8 | 84 | 1 | 609173 / 286855 us = 2.124× |
| Equal L0 field 8 → 8 control | 25 | 25 | 258832 / 84520 us = 3.062× |
| Modeled L1, n1=2, inner 1 → 8 | 25 | 3 | 1246378 / 667401 us = 1.868× |

Cold cost charges cheap solve and certificate on every input, plus a fresh
target solve on each rejected input. It does not reuse target-query cache on
fallback. Common sampling is omitted on both sides. Even the single accepted
1→8 input costs 115 us versus target 63 us. These are native kernel timings on
random-legal conditional positions, not phone-device latency or game strength.
Sol independently regenerated fixtures, checked support and every interval,
recomputed accounting and replayed accepted/control cases. Full details and
logs are in [Sol's report](math/sol/report.md).

## Recovered sources and scope

All 66 user/assistant message bodies in the authorized Claude conversation
[Building an AI in Excel](https://claude.ai/chat/58a73a5e-c5d2-4355-84a6-9f8234207aad)
were read. Collapsed tool transcripts were not exhaustively expanded; the
relevant original neural source was recovered from message 22. No message was
sent. This is a read/download provenance statement, not replication of all
historical experiments.

* `incoming/walt-sense.zip`: 39,874 bytes, SHA256
  `df5fa7294cf41f334233aa4a0e864384ad18d555bf08c2819b6a8ed2eb49f326`.
  Twelve files extracted under `incoming/walt-sense/walt-sense/` after path,
  symlink and size checks. This packet itself has no neural training source.
* `incoming/rollout_net.py`: original creation version, 9,019 bytes, SHA256
  `8b1d2bc68e9d2299031ce8393741bd7bdff9932dcd04dcb73e03217887710508`.
  It has top-level training code and was **read, not imported or executed**.
* `incoming/drive/`: all nine current files fetched from the supplied
  [Drive folder](https://drive.google.com/drive/folders/1-0rjkAYyIWZbb-Pg6cpqEn80T2wwNLMb),
  including HANDOFF.md and TINY-MODEL.md. Its provenance records original file
  IDs, URLs, timestamps and byte hashes. Seven `.diff` files preserve changes
  from the ZIP. These versions must not be silently interchanged.
* `reference/simple_walt/` is issue-100 branch commit
  `d45b3998120999552e324a8b60d58f1fefba7f8f`: a Python memo replay experiment,
  **not** the user's tiny neural experiment. The older compiled Scheme student
  campaign is a third, separate lineage.

## Accepted results, errors and unresolved claims in Fable material

The standalone rational `endgame.py` was read and reproduced under a 30-second
watchdog in 1.210 seconds. It enumerates 560 remaining deals and all weighted
future Dice continuations, keeping the defender's choice shared by public
history. Bid-made probability is 515/2016 after 6-3 and 449/2016 after 6-0;
the defending seat therefore prefers 6-0. Information-tree node counts are
48,795 and 48,709. This is exact for the supplied uniform root fiber and uniform
future other-seat policies; it does not infer action-likelihood posterior
weights for the prior observed history or model the production Walt field.
Receipt: `results/fable-endgame-reproduction/`.

The C `br_tape` and Python `exact_tape.py`/`engine42.walt_decide` optimize future
own actions separately per world before averaging. They are fused relaxations,
even when their executable root choice receives only legal own/public inputs.
The claimed 12–160× acceleration compares against a factorial-ordering
prototype. Pinned Rust Walt already branches recursively; this is not evidence
of speeding it up. C `exact_value` does share decisions and carry accumulated
chance weights, a useful model-specific building block (floating arithmetic).

The Python no-fusion stochastic fold groups histories but omits prefix reach
likelihoods at later focal maxima. Equal-prior worlds A/B/C reaching one
observation with likelihoods 1,1/3,1/3 and actions X winning B,C versus Y
winning A give an exact abstract reversal: unweighted X=2 beats Y=1, while
weighted X=2/3 loses to Y=1. A concrete Texas42 fixture for this bug remains
to be constructed. Frozen unit-weight tapes do not have this particular bug.

`engine42.py` replay initializes bidder=0 and calls it irrelevant, although
its game stops at bidder-dependent thresholds; that can truncate odd-bidder
histories. This is a static control-flow finding, not yet a replayed Texas42
counterexample. The fixed-width base-29 history representation may overflow
after 13 plies; no legal-history collision is established. The workbook builder
requires absent `scenes.json`. These scope distinctions are deliberate.

Historical match summaries report 376/720 native-L1 and 58/120 partner results.
They lack a pinned baseline hash and move-level receipts. Their harness defaults
partner off, changes opening budget/profile, lacks all four rotations, uses
naive hand-level uncertainty for paired data, and executes subprocesses without
hard timeouts. We did not run it or adopt its strength/speed inference. Current
Drive edits add batch seeds but do not repair those evidentiary gaps.

## Tiny-network diagnosis

The recovered uniform-rollout teacher is lawful: a simulator seeing the real
hidden world does not create fusion when the continuation policy is fixed and
information-admissible. Regression learns a conditional value under the
training occupancy distribution; transferring it to another posterior/policy
needs assumptions. `TeacherFusion.lean` separately proves an abstract example
where oracle-optimized labels produce the wrong lawful action, a distribution
shift reversal, and the usual 2-epsilon action-regret guarantee under explicit
action-value error bounds. It does **not** assert the uniform teacher is fused.

The creation script generates 14,000 uniform games for one own hand/trump,
attaches each game's final result to all its rows, and randomly splits rows.
Its validation is not a held-out-game/hand test. Its comparison invokes the
archive's fused `walt_decide`, not the exact production phone player. Later
reported paired-label/iteration variants, checkpoints, and complete raw logs
were not recovered, so their claims remain attributed, not reproduced.

The network has 56,577 parameters (226,308 bytes float32; 678,924 bytes for
parameters plus two Adam copies; these omit gradients/data). A 512×28×64 gather
alone is 3,670,016 bytes. This source is plain NumPy, not a GPU implementation.
Small weights do not establish cheap label generation or useful action ranking.
No substantial neural training, dependency installation, or paid work occurred.

Next viable tests should use a pinned lawful teacher, paired all-action vectors
and advantages, complete-game/hand splits, tie-aware regret, teacher uncertainty,
and explicit training/label amortization. Common random numbers reduce variance
only when their induced covariance helps. Fixed-budget approximate rungs can
cost linearly in their count without preserving exactness or fixed error;
policy iteration may cycle. Cycles do not imply absence of a mixed equilibrium.

## Reproduction

From the research worktree, use fresh output directories and the watchdog:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/walt-build-2 -- cargo +1.95.0 build --offline --release --manifest-path experiments/astra-sol-20261004/phase2/probe/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-cost-log-2 -- python3 experiments/astra-sol-20261004/phase2/run_cost.py --out /tmp/walt-cost-2 --start 100 --count 144
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-certificate-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/phase2/math/sol/FirstDivergence.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-teacher-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/phase2/TeacherFusion.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-certificate-cases-2 -- python3 experiments/astra-sol-20261004/phase2/math/sol/certificate_witness.py
```

Use `--cheap 4`, `--cheap 8`, or `--level 1` for the follow-up cost variants;
the retained plans record exact ranges. Do not execute incoming training scripts
or the historical harness as a reproduction shortcut.
