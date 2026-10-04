# Phase 2 Sol review: first-disagreement certificates

2026-10-04. **EXPLORATORY RESEARCH.** No production player changed.

## The implemented mathematical guarantee

`FirstDivergence.lean` formalizes an arbitrary-action finite-horizon game model
with deterministic mechanical transitions, terminal payoff, public observation,
and observation-only focal policies. At focal nodes its certificate quantifies
over **every legal action**. At nonfocal nodes it requires completed cheap/target
action equality and recursively follows only the cheap edge.

The proved theorem `certified_complete_replay_equal` states that a completed
certificate forces identical **full action trace and terminal payoff** under
cheap and target fields for every lawful focal policy. The remaining three
theorems derive payoff equality, its contrapositive, and the coupling law for
every unflagged scenario. Thus the phase 1 weighted disagreement sandwich and
maximum transport apply with bad-scenario mass as an action-specific error.

This proof explains why the certificate may enumerate world-specific focal
actions as a superset while the response itself must share focal choices over
equal observations. The world-specific enumeration provides an event upper
bound; it never becomes a policy witness or a sampled payoff maximum.

The core-only Lean v4.33.0-rc1 run completed in 0.209 seconds under a 20-second
process-group watchdog. All four theorem axiom reports contain only standard
`propext`, with no `sorry`, `native_decide`, or custom axiom. Logs are in
`lean-first-divergence-run/`.

## Independent executable adversarial checks

`certificate_witness.py` checks 256 small finite-game field pairs, with 16 lawful
focal policies using only public histories, two roots, and weighted duplicate
scenarios (four original scenarios, three physical worlds, total mass seven).
Across 8192 policy/root comparisons, every unflagged trace and payoff matched,
the weighted discrepancy bounds held for policies and their maxima, and all
65 accepted canonical transports selected the exact target root. Zero false
positives occurred. Artificial refused target queries count the world bad.

Two strict negative witnesses also passed: exploring only the incumbent focal
action can wrongly certify a world whose other focal branch changes payoff;
and equality cannot exclude a smaller-index action under least-tile ties.
The bounded executable completed in 0.090 seconds; logs are in
`certificate-witness-run/`. These checks validate abstract finite cases and
must not be described as Walt timing or playing-strength measurements.

## Review of the Rust operationalization

The reviewed `phase2/probe/src/main.rs` is sound for audited valid straight-play
fixtures. It branches over all focal legal actions and stops early only after
witnessing a disagreement, which safely marks that scenario bad. Forced field
actions coincide. The target and cheap query caches are separate; completed
answers are checked for legality. The event memo contains world identity plus
played tiles, trick leader and partial plays, and banked scores. It is scoped
to one root context and fixed fields. Ignoring full chronology in this event
memo is safe under this implementation's reduced-key field purity and the
all-legal-focal-path superset; it does not merge lawful response choices by
hidden information.

Query refusal or incomplete traversal aborts the complete probe with
`certified:false`; no partial unflagged event mass is installed. Early objective
termination is safe because the straight bid is irrevocably decided at that
mechanical state. Duplicate worlds remain separate original scenarios when
the error counts are reduced. T0 values are complemented before maximizing
focal success, and canonical transport uses strict exclusion for lower tiles
and weak exclusion for higher tiles.

The certificate is a sufficient test, potentially a costly one. Its cheap
query solver has already been warmed by the cheap best-response solve; report
the deployable cost as `cheap_us + certificate_us`, compared with a separately
fresh target solve. A smaller certificate-only time does not establish a
faster replacement. Cheap/target field construction, outer sampling, key counts,
and all query/solver work must travel with timing. A timeout or a loose bound
rules out this method for that fixture, not all possible safe substitution.

The probe parser is less strict than the production request adapter's history
validation. Its input boundary is therefore the independently audited extracted
fixture corpus; it should not be exposed as a trusted live/public player API.

## Initial local distillation provenance search

The strongest local Walt predecessor found is the paused compiled lower-rung
campaign in
`/Users/jason/.codex/worktrees/walt-response-ladder/texas-42/experiments/response-ladder`:
`PAUSED-HANDOFF.md`, `COMPILED-PLAYER-STATUS.md`,
`docs/COMPILED-LOWER-RUNGS-DRAFT.md`, `src/bin/compiled-teacher.rs`,
`tools/compiled_cycles.py`, `tools/fit_compiled.py`, and fitted C0/C1 actor and
calibration artifacts. This fitted Scheme programs against finite cost-vector
teachers, rather than a tiny L1 neural net. Its handoff explicitly records a
user usage-budget pause and a failed fresh quality gate despite faster play;
that is not evidence that the Mac could not train a small model.

The `mk5-main/gus/model/student.py` and `wiki/topics/student-distillation.md`
contain older MLP/transformer distillation against an E[Q] perfect-information
oracle. They are a different teacher/architecture lineage and cannot silently
be treated as a distilled lawful Walt L1. The Walt idea-tier page
`wiki/field/directions.md` describes a tiny quantized NNUE-like evaluator as an
idea, not a completed implementation or a measured feasibility result.

No source of the user's specifically remembered tiny-L1/small-VM stall has yet
been located in these local checkouts. The incoming Fable archive remains to be
read once the lead supplies its acquired local path. No imported script or
training job was run during this provenance search.

## Independent native cost audit

`cost_audit.py` regenerated all 144 source deals (100–243), contracts, random
legal prefixes, public-only requests and eligibility decisions from the panel
plan. It audited all 84 eligible completed receipts, residual-world disjointness,
own hands and public void constraints, all per-world Boolean flags and error
counts, focal-team value bounds and least-tile interval exclusion. It reproduced
the only certified decision, id 213, including the exact sampled worlds and
flags, and checked an equal-field control with zero disagreement.

The bounded independent audit passed in 0.090 seconds in `cost-audit-run-2/`.
The initial run failed on this machine's Python lacking `int.bit_count`; the
portable replacement passed. Neither run changed panel evidence.

This panel certifies 1/84 positions; 46/84 cheap choices agree with target.
The deployable **certificate plus fresh fallback** accounting is 487000 µs,
versus 288052 µs for a fresh target solve, exact ratio 121750/72013 (about
1.691). This charges cheap solve plus certificate for every input and target
solve for every rejected input. The accepted input alone costs 115 versus
63 µs. Common sampling (431 µs total) is omitted from both compared sides.
These are native kernel measurements on conditional random-legal positions,
not phone latency or playing strength. A variant that reuses the certificate's
target-query cache for fallback was not measured; this cost objection does not
rule out that variant or all safe-substitution methods.

Independent follow-up receipt audits also passed for `cost-near` (84 inputs,
cheap 4→8, one certificate), `cost-control` (25 inputs, equal 8→8, all certify)
and `cost-level1` (25 inputs, three certificates). Their cold-fallback totals
versus fresh target are respectively 609173/286855, 258832/84520, and
1246378/667401 µs (about 2.124, 3.062, 1.868 times). Every fixture/request,
world constraint, flag reduction, interval bound and canonical exclusion was
checked; one accepted input per panel was replayed and an equal-field control
checked. Logs are `cost-near-audit-run/` (0.138s), `cost-control-audit-run/`
(0.090s), and `cost-level1-audit-run/` (0.080s). These audits do not add new
performance samples; they recompute accounting from retained evidence.

## Distillation versus issue 100

Issue 100 branch commit `d45b3998120999552e324a8b60d58f1fefba7f8f` contains
`walt/probes/simple_walt/README.md`, `simple_walt.py`, `walt42.py` and a replay
memo ablation. Its report covers 179 decisions in 11 deals; it is not a neural
student experiment. The user clarified that it shares the idea, not the tiny
experiment's identity. The tiny student's supplied statistics remain attributed
user observations, not verified by this branch or the incoming archive.

`simple_walt.py` uses traversal-stream RNG and expressly redraws modeled
decisions with memo disabled. Memo removal therefore changes realization of
the stochastic field, not just evaluation cost for an identical frozen field.
Own/public inputs can remain lawful while stochastic answers depend on the
query traversal. This ablation does not establish that production's pure-key
completed query replay is unnecessary.

The lead's six-theorem `phase2/TeacherFusion.lean` was independently read and
rerun cleanly in 0.264 seconds (`teacher-fusion-independent-run/`). Its constant
future observation makes the fusion counterexample valid: safe sum 6 beats
every lawful risk sum 4, but clairvoyant risk sum 8 reverses the label. The
separate posterior-shift example needs no clairvoyance. The action-value
regret theorem correctly requires both selected and optimal action errors;
outcome classification accuracy supplies neither hypothesis. All axiom reports
contain standard `propext` only. The game is an abstract scaled-reward example,
not a claimed Texas42 position.

## Recovered Walt-sense archive: independent initial review

The lead recovered the archive from the explicitly authorized Claude chat
download. I read the actual extracted files at
`phase2/incoming/walt-sense/walt-sense/`; no incoming script was imported or
executed. `archive_witness.py` records all twelve file SHA256 values and two
independent scalar witnesses, passing in 0.048 seconds under the watchdog
(`archive-witness-run/`). This supersedes the earlier unavailable-archive note.

The bundle contains a vectorized Python engine, a C player, exact-tail and tape
experiments, an Excel generator and four aggregate match lines. It contains no
neural training implementation, checkpoints or the remembered tiny-student
dataset. `build.py:9` requires absent `scenes.json`, so the delivered workbook
generator is not self-contained. It also executes `flat.py` at line 407.

**The early tape player uses strategy fusion.** `exact_tape.py:48–56` takes a
best/minimum continuation separately inside each hidden world before averaging;
`engine42.py:230–234` does the same over orderings. `walt.c:81–90` explicitly
recurses on one deal and optimizes its future focal choices, then sums over
deals. The actual root policy can still be a lawful own/public mapping because
it samples rather than receiving opponents' real hands. Its continuation label
is nevertheless a determinization heuristic, not the sampled lawful focal best
response used by the audited Walt solver. Common random tapes improve pairing
but do not fix fusion or turn Dice into Walt's modeled L0 field.

**The C exact tail has the appropriate information structure for its model.**
`walt.c:107–116` keeps all compatible deals together at focal choices and divides
each carried `Deal.w` by the opponent's legal count before descending. The
root prior is uniform over void-compatible remaining deals; opponents are
uniform legal Dice. This is a lawful best response for that specified prior and
opponent model, not an exact best response to production Walt L1 or a proof that
actual opponents induce the same posterior. It uses floating arithmetic, so
enumeration-exact semantics do not imply rationally exact arithmetic/ties.

**The Python no-fusion chance fold omits past likelihoods in its choice.**
`nofusion_sc.py:42` stores local opponent branch weights, but line 61 selects
future focal actions from unweighted sums of continuation `val`; line 65 applies
those weights only when folding earlier opponent nodes. At a reached observation
the maximizing choice needs the probability mass that reached each hidden row.
Equal-prior worlds A/B/C with observation likelihoods 1, 1/3, 1/3 demonstrate the
error: X wins B/C and Y wins A. Unweighted totals 2 versus 1 choose X, but reached
mass 2/3 versus 1 prefers Y. This exact Fraction witness is abstract, not a
reproduced Texas42 position. The issue concerns `tape=None`; fixed-tape scenarios
have unit scenario masses and need no such chance weights. The C tail already
handles these weights correctly. Grouping keys are base-29 `np.int64` encodings
(`nofusion_sc.py:35,49`), which can overflow beyond 13 continuation plies; no
concrete legal-history collision was established, and a ≤12-ply tail avoids it.

**The Python consistency replay can terminate early with the wrong bidder.**
`engine42.py:181` constructs `Game(...,30,0)` and calls bidder irrelevant for
legality, but `Game.play:94,112` freezes rows after bidder-dependent score
termination. In an odd-bidder history, an odd team reaching 13 points can stop
this dummy game even though the real bid remains undecided, corrupting later
history legality checks. The C sampler uses inferred void constraints and is
separate; this finding does not invalidate it automatically.

**The archived head-to-head is a promising heuristic lead, not verified phone
parity or a demonstrated gain.** `harness.mjs:22` loads unpinned local bytes,
with no supplied WASM/source/compiler hash. Lines 3,46 use optional partner
default false and the same 40-world/14-second request even on bidder opening;
the audited production default is partner true, with first-play 160/20 seconds
and review off, ordinary 40/14 seconds and review on. It ignores checkpoint
recovery (line 24), has no process timeout for the C subprocess (line 58), keeps
one WASM instance for the whole match, and records only aggregate results rather
than plans, transcripts and failed/incomplete attempts.

The C boundary receives only its own dealt hand and public plays (lines 55–58),
and its decision RNG seed comes from its own hand and public history
(`walt.c:147–149`). The WASM policy seed at line 45 derives from the same
SEED/deal index used at lines 69,75 for hidden deals. Given the advertised match
seed, that field recovers deal index and side exactly; the scalar witness checks
this algebra. This does not prove that the present WASM exploits the seed, but
it fails the intended independence boundary for a reusable eval harness.

Each source deal is played twice with team assignment swapped, which is useful
pairing. There are no seat rotations or fresh planned validation blocks, and the
aggregate log does not identify which source deal sets overlap. Repeated games
must be clustered by source deal, not treated as independent hands. The three
partner-off lines report 127/240, 129/240,120/240 wins; the partner-on line reports
58/120. Those figures alone do not establish improvement. The archive's early
per-world search may still be a useful fast heuristic to test in the audited
production harness after correcting boundary/profile/provenance accounting;
it should not teach a student labels presented as lawful Walt L1.

The lead's full authorized Claude conversation read subsequently establishes
that the tiny student's original labels were fixed uniform-rollout outcomes
(message 20), not this later per-world optimized tape kernel. The fusion finding
must not be applied retroactively to that teacher. Outcome-versus-advantage
targets, finite-label noise, train/deploy posterior or occupancy mismatch, and
the gap between a uniform rollout evaluator and Walt remain the relevant tiny
student hypotheses. The archive cannot reproduce that experiment because its
neural source, weights and training artifacts are absent.

The strongest reusable route from this archive is a corrected **grouped tape
best response**: sample a frozen bundle of compatible worlds and hidden random
tapes, share each focal action across equal observations, and group worlds by
observed nonfocal tile before recursing. On a fixed tape bundle the original
scenario masses stay attached to rows; on full Dice enumeration, carry the
cumulative 1/legal-count reach mass forward before choosing at focal nodes.
This must first agree with a small direct lawful-policy enumeration. It remains
a best response to the named Dice/tape model, not automatically Walt's lower
rung. Charge the full branching/query work and teacher label generation.

Only after those semantics and costs are checked would a small reproducible
paired-action-vector dataset clarify student capacity: freeze the original own
hand/deal splits before related histories, preserve all scenario/tape seeds and
legal-action targets, compare finite teacher noise and held-out regret/gaps, and
retain fresh paired production games for strength. No training has been run or
authorized through this review.
