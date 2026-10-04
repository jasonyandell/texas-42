# Independent higher-k budget review

Exploratory executable evidence; conditional Lean kernel mathematics is labeled
separately. The reviewer owns only `checks/`, makes no commit, and modifies no
inherited source, production crate or original evidence. Accepted native identity
is `2154b88b380822759a138937444c653be1b7644d138069eebaae4cf04cc12bd5`;
WASM is `fe04e6628166abb0917401393a3629d86125037edebd6d9e5f12d3bf7d460bab`.
Timing evidence retains the separately pinned measured binaries and source in
[ARTIFACTS.json](../ARTIFACTS.json); final identities are not substituted for them.
Actual native/WASM and reviewer-source identities are in [SOURCE_HASHES.json](SOURCE_HASHES.json).

1. **Accepted inherited checkpoint arithmetic and preservation.** Independently
   written [saved audit](audit_inherited.py), with its
   [capped receipt](inherited-audit/run.json), checks all 2,443 predecessor
   manifest entries, including 561 native-policy entries, and unchanged baseline
   and production source against `56794dc3`. It rereads 144 saved games on 18
   source-deal clusters: 0 paired wins, 2 losses, 70 ties, mean −1/36, bounded
   95% interval `[-0.6679929719910582, 0.6124374164355026]`. All 432 saved cold
   vectors match; median 12-request batch totals recompute as native 52.6219805 ms
   and resumable 310.4389995 ms. These are saved-artifact checks, not new timing,
   strength or physical-device evidence. Original [report](../../native-policy-check-20261004/REPORT.md)
   and [review](../../native-policy-check-20261004/checks/REVIEW.md) remain intact.

2. **Accepted production provenance and the distinct phone control.** A fresh
   [identity receipt](identity/run.json) checks all 202 source inputs against
   Rust `cb1ef3b23072e4c268f31f625f2b61d5facc1929`, source SHA256
   `5bedf14af0e5a5ef354acca4765f67843469d074d163e8e4786631aa23436e32`, and
   saved Plunge `a0d9fa806166b0e63fe016bb49d93f91f47b1af8` blobs. The exact
   pinned phone WASM is SHA256
   `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   [Final phone conformance](phone-conformance-final/run.json) checks nine independent
   public roots across all nine declarations at nominal 200 ms: 18 complete
   non-clock result comparisons between new native routing, new WASM routing and
   the exact pinned blob; complete WASM checkpoint sequences agree too. Only
   `elapsed_us`, `solver_us`, and `over_budget` are excluded. This is the actual
   L1/partner/base-eight-sample phone policy, distinct from the ladder's base four
   samples and deliberate forced/settled ascending fallback. It is local pin
   equality, not current external deployment, ordinary 14 s or physical phone.

3. **Accepted named rung and finite refinement; qualified information scope.**
   Root chooser k responds to `Field::Level(k-1)`; modeled L0 responds to Dice.
   Every rung freezes outer 40, inner `[4,2,2,2,2,2]`, Voidless lower beliefs,
   Fixed selection and ascending ties. [Independent Rust runner](runner/src/main.rs)
   separately reconstructs SplitMix64, modulo/rejection consumption, record hash,
   ordered outer worlds, duplicate mass and final RNG, then calls unmodified
   native recursion directly. Its [fresh oracle](independent-oracle/run.json)
   produces 45 complete vectors on nine fresh two-trick roots across all nine
   declarations. [Native](native-adapter/run.json) and [WASM](wasm-adapter/run.json)
   and fresh [final native](native-adapter-final/run.json) /
   [final WASM](wasm-adapter-final/run.json) each match all 45 vectors, samples,
   RNG, attempts and per-level counters;
   each independently full-replays 1,800 sampled deals. Thirty-one vectors have
   ties; strict ascending selection agrees. [Deeper conformance](deeper-conformance/run.json)
   and [final deeper receipt](deeper-conformance-final/run.json) add 20
   independently prepared rung vectors on all four eligible first-nine
   twelve-ply roots (five planned roots are ineligible), yielding 40 native/WASM
   vector/sample/RNG/counter comparisons. These reuse frozen deals and add no
   independent strength samples. [Exact inherited k3 comparison](inherited-k3-conformance/run.json)
   executes the unchanged task8 binary SHA256 `51259e4088de71f813ab1c8ad0b81345129ea01c26dbefec60cba462c1878de7`
   read-only and confirms nine fresh k3 vectors/worlds/RNG/attempts against its
   original four-entry budget, including prefix counters and unused zero tails.
   Outer support obeys public voids. Inner Voidless deliberately forgets void
   deductions; it is not exact perfect-recall belief or a claim that every inner
   sampled world can replay the whole observed history. Complete modeled actor
   state and immutable context remain authoritative, without referee deal seeds
   or opposing hands in chooser requests/cache identities.

4. **Accepted lawful refusals; contradicted strict total-wall budget hypothesis.**
   Native/WASM each pass 45 zero-budget refusals with ascending legal fallback,
   no partial vector, and 11 malformed/private/invalid-profile refusals. WASM
   emits the lawful initial public checkpoint before solving. The deeper audit
   checks nine controlled monotonic-clock exhaustions retaining that checkpoint.
   These are synchronous local ABI checks, not real browser interruption tests.
   The nominal 20/200 ms allowance charges status/replay, sampling and solver;
   transport/process scheduling/serialization remain recorded separately. The
   retained first-deal k4/20 call takes 267.065666 ms host wall, versus 13.944 ms
   wrapper time: a 247.065666 ms nominal host overrun, with unexplained transport/
   startup/wait gap. It is retained, not discarded. Thus a strict total 20 ms
   latency interpretation fails; nominal solver-budget behavior and host latency
   must remain separate. No production clock/deadline guarantee is established.
   A later adversarial scalar check found a genuine malformed-input discrepancy:
   native rejected k=`4294967297`, but WASM32 cast it to usize before validation,
   wrapped it to k1, and accepted it. The [failed check](scalar-boundaries-import-fixed/run.json)
   and original raw record are retained. The final source checks raw u64 range
   1–5 before casting; [final scalar receipt](scalar-boundaries-final/run.json)
   confirms seven invalid integer/boolean/fractional scalars are rejected by both
   backends. An exact source comparison verifies this guard is the only adapter
   change. All planned k1–5 policies remain the same; accepted timing stays tied
   to measured source/bytes rather than being relabeled final timing.

5. **Accepted frozen panel and independent full suffix arithmetic; qualified
   strength.** [Independent data checker](audit_data.py) and
   [final artifact receipt](data-final-artifacts-audit/run.json) freshly
   reconstruct all 54 source PRNG deals,
   lawful random twelve/sixteen-ply prefixes, all rotated hands/public seats and
   complete own/public requests. Reviewer-local Straight mechanics check 3,024
   complete twelve-move suffix games / 36,288 moves, all policies, legal actions,
   winners, points, outcomes, complete vectors/ties, refused fallbacks, source/
   binary identities and summary arithmetic. Each comparison has 54 source-deal
   clusters; four rotations and partnership swaps are correlated. Against same
   k1, k3 gives 2 wins / 4 losses / 210 ties at both budgets, mean −1/108;
   k5 gives 0/6/210 at 20 ms and 0/4/212 at 200 ms. The pointwise bounded interval
   radius is 0.3696284147. This assumes independent PRNG source deals and is the
   specified random-prefix suffix distribution, not full production games.
   These seven intervals are not simultaneous family coverage. No strength gain
   or equivalence follows; no completed-only outcome subset is substituted.

6. **Accepted budget cliff and adaptive phase disclosure; qualified growth.**
   The original [frozen plan](../plan.json) precedes outcomes; its 2,160 fixed
   root calls cover 216 positions, with 96 eligible positions from 24 source
   deals and 120 ineligible. All k1–4 eligible roots complete at 20/200 ms;
   k5 completes 80/refuses 16 at 20 ms and all 96 at 200 ms. The separate
   [earlier plan](../earlier-plan.json) explicitly follows this result; it covers
   every same 54 source deal / four rotations at twelve plies, with no new
   independent deals and no twelve-ply strength panel. Its 2,160 calls have 140
   eligible positions from 35 source deals: at 20 ms k3 completes 23, k4/k5 zero;
   at 200 ms k3 completes 140, k4 133, k5 16. All refusal denominators and partial
   JSONL records are retained and checked against complete files. The
   [refusal-interval audit](final-extras-audit/run.json) additionally reconstructs
   all 20 budget/horizon/rung cells using **all 54** source clusters, where each
   cluster averages four refusal indicators in [0,1], including zero for
   ineligible requests. Its pointwise radius is 0.1848142074. For twelve-ply
   k5/200 ms, planned-root refusal is 124/216 = 0.574074 with interval
   `[0.3892598667, 0.7588882814]`; this is distinct from 124/140 eligible refusals.
   This audits conditional arithmetic, not PRNG/runtime independence,
   simultaneous-family or adaptive post-selection coverage. The
   [final unconstrained/scaling audit](scaling-final-artifacts-audit/run.json) checks 225
   planned cells on first-nine deals: only four twelve-ply and eight sixteen-ply
   roots are eligible. All these 30 s cells complete; twelve-ply median modeled
   computations rise 1,199 → 14,981 → 97,748.5 → 441,409.5 → 1,421,657,
   with completed host medians about 1.55 → 8.27 → 39.85 → 147.83 → 438.91 ms.
   These are finite empirical costs on a small selected prefix panel, not a
   complexity lower bound or population cost interval. Changing k changes the
   modeled opponent; comparing model-relative action values is not a fixed
   reference strength test. `pi_calls_by_level` counts modeled computations;
   [L0 bypass](../../../walt/walt/src/solver/uncached_l0.rs:41) recomputes and is
   not a universally unique actor-query count. Sample totals count fully sampled
   bundles; RSS/cache-entry counts do not give exact logical allocation bytes.

7. **Accepted finite useful whole-query parallelism; qualified affordability.**
   Six balanced repetitions of one/four solver processes preserve 36 planned
   k5 requests on the same nine deals, including 16 eligible roots from four
   source deals and 20 ineligible roots per batch. Independent checks match all
   432 non-clock responses/counters, comprising 192 complete vectors and 240
   lawful ineligible responses; there are no refusals. Complete capped median
   walls are 7,118.9585 vs 2,066.365 ms, **3.4451602×** speedup. Internal batch
   medians 7,026.2045 vs 1,995.3503 ms (**3.5212887×**) are secondary because
   they exclude Python startup, hashing and receipt work. Child CPU rises from
   median 7,015.095 to 7,902.5675 ms (about 12.65%). Sum of PID maximum RSS rises
   67,698,688 to 254,713,856 B (about 3.76×): an upper envelope, not measured
   simultaneous aggregate RSS. Fresh query caches and the same 30 s allowance
   per query preserve equal planned aggregate budget. This supports throughput
   of frozen complete queries on this Mac, not lower monetary/CPU cost,
   recursive lower-actor parallel speedup, single phone-turn or device scaling.

8. **Accepted conditional mathematics; qualified implementation refinement.**
   Fresh [ChangingOpponent.lean](ChangingOpponent.lean) passes the
   [kernel receipt](lean-counterexample/run.json) without any axioms: both old
   and new actions can be exact responses to their own modeled opponents while
   the new response earns 0 against a fixed reference where the old earns 100.
   This is a generic two-action example, not a Texas42 or measured-k witness.
   Inherited [Bellman/mass/cache laws](../../native-policy-check-20261004/checks/PolicyEquivalence.lean)
   remain conditional on full equal ordered successors, folds and lossless keys.
   Inherited [nested attained-value monotonicity](../../adversarial-20261004/sol/NestedPlans.lean)
   assumes policy-class inclusion under one unchanged value map; a changing
   opponent ladder does not automatically meet those premises. No generic
   higher-k monotonicity, linearity, runtime lower bound, privacy or Rust
   refinement theorem is supplied.

9. **Accepted preserved repairs; no new sourced historical contradiction.**
   The first reviewer runner build used a Domino where a Context was needed;
   [failed receipt](build-runner/run.json) remains, and
   [fixed build](build-runner-fixed/run.json) passes. The builder's initial
   [development summary](../results/summary-development.json) overwrote candidate/
   reference cell identifiers with cost dictionaries. The final summary repairs
   those keys; [original measured source snapshot](../snapshots/experiment-measured.py)
   retains the first schema. The summary key assignment is the sole change
   between the original measured source and
   [parallel/earlier measured source](../snapshots/experiment-parallel-measured.py).
   The final source subsequently adds only configurable fresh output routing
   through ephemeral `WALT_K_RESULTS`; the measured extension source is also
   [preserved](../snapshots/extension-measured.py). Reviewer diff inspection finds
   output routing changes, with no sampler, policy, fixture or budget change.
   Every old record must match its exact appropriate retained source snapshot
   digest; the checker explicitly recognizes the two measured experiment sources
   and measured extension source. Earlier development analysis
   with a missing resource display is not final authority. These are concrete
   artifact/checker repairs, not refutations of the inherited scoped native or
   historical Dice-component claims. The tested idea that more k must yield
   stronger/affordable play is unsupported here; constructive next work is a
   budget-aware horizon policy and complete-query throughput with explicit
   resource limits, validated on fresh actual-game deals.
   The scalar test's first [attempt](scalar-boundaries/run.json) had a reviewer
   import typo, then the corrected test exposed the real cast bug above; both
   failures remain separate from accepted final checks. The final data checker
   verifies every ARTIFACTS entry against its actual measured/final executable
   and source bytes. The [portable archive audit](final-extras-audit/run.json)
   independently decompresses all four committed deterministic gzip files,
   checking compressed length/hash, raw length/hash and actual executable bytes.
   [Restore helper](../restore_artifacts.py) recreates only ignored target paths
   and refuses different existing bytes. This resolves the clean-clone byte
   blocker; rebuilding final policies alone still does not reproduce the old
   measured execution identity automatically.

10. **Accepted execution boundaries and remaining obligations.** Every reviewer
    build, kernel/check run uses `run_capped.py` allowance ≤295 s and elapsed
    below 300 s; final [receipt audit](receipts-delivery/run.json) pins source/binary hashes
    and checks all retained successes/failures and reaped child processes.
    No job remains pending. No merge/deploy, browser, paid resource, external
    messaging/fixture transmission or credential/persistent-access change occurs.
    The specifically blocked Claude fixture disclosure stays blocked. Actual
    browser/device allocation, interruption, privacy refinement, broader
    held-out strength, hard host latency and intentional product policy remain
    unmeasured. This review accepts finite reproducible local evidence only.
