# Public-prefix state factoring and exact child interning

2026-10-04. Exploratory component evidence, with six separately identified
conditional Lean lemmas. Parent checkpoint:
`c2ab1eb138ed5d19afbd9e9ec9262e0594e6e6a4`. [Reproduction](README.md),
[fresh independent review](checks/REVIEW.md), [accepted ledger](ACCEPTED.md),
[preserved contradiction links](CONTRADICTED.md).

## Result

Factoring one occupied public State per full prefix, then replacing repeated
`(prefix,tile)` hash-map probes with exact child lists, produces a **measured
cold Dice component win against equal-worker native recursion**. On a fresh
49-root holdout, four workers on both sides give1.81x/1.65x service throughput
at128/512 worlds, and1.24x/1.20x including process/JSON/replay/harness costs.
Eight-world batches lose. Full modeled-policy frontiers remain substantially
slower; their unique lower-actor demand grows sharply with rung. The bounded
field2/40-world full service refuses its50k-query allowance; the exact hybrid
completes. This checkpoint contains no full phone adapter or playing-strength
result.

## Numbered claims and evidence

1. **Baseline and originals preserved.** This isolated local repository/worktree
   starts at the specified checkpoint. All tracked changes are confined to
   `experiments/prefix-state-20261004/`. The fresh
   [identity receipt](results/identity/stdout.log) matches production Plunge
   `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, native source
   `cb1ef3b23072e4c268f31f625f2b61d5facc1929`,202 source inputs, and phone WASM
   SHA256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   Production source, inherited receipts, the canonical
   [ACCEPTED](../adversarial-20261004/ACCEPTED.md) and
   [C1–C3 contradictions](../adversarial-20261004/CONTRADICTED.md) are unchanged.
   The predecessor's2,750 vector comparisons included repeats and its17 Lean
   declarations were conditional abstract results; neither is promoted here.
   The agent tool accepted the requested fresh `gpt-6.1-sol` collaborator.
   Separate runtime model attestation was unavailable; no stronger identity
   claim is made.

2. **Public factoring is narrower than policy factoring.** Each root coordinate
   belongs to exactly one complete Query; each child is identified by
   `(parent coordinate,tile)`. Induction gives one query identity and one public
   state per coordinate because the rule transition is deterministic. STOP
   preserves a settled state. Prefixes remain full history coordinates even
   when different histories reach the same reduced State. The
   [implementation](native/src/lib.rs) keeps world index/multiplicity in each
   row, private hands in the original ordered worlds, and native seeds or
   historical tapes per world. Legal hidden-hand masks and nature draws remain
   per world. At focal prefixes the hand is fixed across the query's worlds,
   so one derived legal mask can be reused. Backward integer sums across all
   reachable rows precede one common MAX/MIN with first-ascending ties.
   No world-wise policy optimization or hidden-world lower-belief input is
   introduced.

   Full Query/ActorSpec cache identities are inherited unchanged, including
   declaration, bid, all sample budgets, boundary, belief mode, complete public
   state, actor/hand, ordered worlds and nature inputs as applicable. Fixed
   native source/selection/seed schema remain process scope. Lower private
   sampling, shuffle consumption and budgets `[4,2,2,2]` are unchanged. Native
   Dice's state-reset rejection/modulo and historical multiply-high depth tape
   remain distinct modes. Outer worlds still pass complete historical legality;
   default Voidless inner worlds are not silently conditioned on those voids.

3. **Mathematics and independent tests support that narrow claim.**
   [PrefixState.lean](math/PrefixState.lean) proves six conditional lemmas:
   replay-child equality, preservation of row consistency, one state per query
   and history, unchanged world identity, preservation of an arbitrary supplied
   actor view, and unchanged integer sum for a fixed row list/payoff evaluator.
   [Owner kernel receipt](results/lean-prefix-fixed/stdout.log) and
   [independent kernel rerun](checks/lean-prefix-independent/stdout.log) pass;
   axioms are absent or standard `propext`/`Quot.sound`. The supplied view itself
   must already be lawful: this does not prove privacy/noninterference. These
   lemmas do not verify the Rust compiler/interner/overflow/refusals or MAX/MIN
   implementation, and do not prove k complexity.

   The independent collaborator passes38 separate scalar roots,152 transformed
   query comparisons (paired world/seed reversal, tripled multiplicity,
   mixed/reversed/duplicate queries),32 full/hybrid policy vectors across both
   belief modes and fields0/1,32 pinned references, and254 linked-versus-hashed
   comparisons. Cache and bounded refusal checks pass. Its separate Python
   occupancy computation exactly matches9,224 row edges and7,331 public
   coordinates on the38 small holdout roots. See
   [final compact parity receipt](checks/independent-parity-final/run.json)
   and [review](checks/REVIEW.md). Four native tests pass, including all supported
   declarations' exhaustive ordered four-tile outcomes and interner equality.
   [Historical-tape receipt](results/final-tape/summary.json) adds282 repeated
   full-vector checks at8/40/128 worlds. The new retained measured panels add
   repeated comparisons, not thousands of independent deals.

4. **The implementation removes specific duplicated work.** The preceding Row
   carried State/query/world/history. The new Row carries only world/history;
   Coordinate owns State/query. Derived compact frames reuse actor, settlement,
   focal legal mask and lazy native public-record hash. State transitions run
   only on a newly occupied child coordinate. The backward fold stores explicit
   parent-coordinate query IDs and never reconstructs per-world public states.
   Local policy spec discovery uses `(coordinate, remaining actor hand)` before
   cloning Context; the global cache/dedup still uses full ActorSpec equality.
   Distinct coordinates with equal actor frames may therefore remain separate
   local demands but are globally deduplicated. This changes lookup/cache-hit
   counts without changing unique sampled queries.

   The exact linked interner has a dense head per parent and a small linked
   child list. Each unseen pair gets the next ID in insertion order. Links
   point backward, tile equality is exact, and parent chains stay separate.
   Child scans are bounded by the finite action alphabet (28 tiles, or one
   terminal STOP); this is not a claim about total tree or rung complexity.
   The hash interner remains selectable as a controlled ablation.

5. **Reuse is counted, and it is modest rather than universal.** On the47-root
   four-worker512-world panel, all three frontiers charge the same2,723,221 row
   edges and2,015,190 public child coordinates. The new service performs
   1,194,501 nonterminal public steps and avoids400,655 repeated steps; it
   performs399,393 native public hashes and avoids228,878 repeats. STOP
   coordinates are included in public_coordinates but excluded from step
   counts. Row/coordinate occupancy is about1.35x; transition reuse is about
   1.34x. On the49-root holdout the corresponding step totals are1,219,218
   performed /499,516 avoided. Scenario rows, legal masks and private draws
   still grow with demand. Most of the final speed improvement additionally
   comes from cheaper interning and removal of fold row copies. Factoring alone
   with a hash interner remains slower than native recursion at128/512 worlds.
   [Primary records](results/final-four-v2/summary.json),
   [holdout records](results/holdout-four/summary.json).

6. **The native Dice crossover survives a fresh panel and equal workers.**
   Primary deals960000–960127 produce47 eligible pip-trump roots; prospective
   holdout deals965000–965127 produce49. Both use plies12–15, frozen lawful
   ordered scenarios and4 alternating cold-process repetitions per count.
   Service times include service/partition setup, preparation, grouping,
   expansion, fold and completed-cache insertion. Native-reference wall includes
   preparation and across-query parallel scheduling, not summed solve intervals.

   | Worlds | Primary linked4 | Primary recursion4 | Native/linked | Holdout linked4 | Holdout recursion4 | Native/linked |
   | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
   | 8 | 1.065 ms | 0.339 ms | 0.32x | 0.845 ms | 0.349 ms | 0.41x |
   | 40 | 3.198 ms | 4.470 ms | 1.40x | 2.945 ms | 4.561 ms | 1.55x |
   | 128 | 8.874 ms | 12.262 ms | 1.38x | 7.493 ms | 13.528 ms | 1.81x |
   | 512 | 32.349 ms | 43.709 ms | 1.35x | 27.444 ms | 45.202 ms | 1.65x |

   At128/512 primary worlds, the predecessor takes17.461/65.582 ms and the
   hash-factored ablation13.104/49.494 ms. Linked factoring takes8.874/32.349 ms.
   With one worker on both sides, linked factoring takes23.264/89.237 ms vs
   34.900/131.460 ms native (1.50x/1.47x). These are local median observations,
   not population confidence intervals, universal linearity or phone latency.
   The independent [arithmetic audit](checks/REVIEW.md) recomputes every
   retained median/ratio from records and checks vectors, order balance,
   input identities, structural counts and binary hashes.

7. **Full costs and memory qualify the win.** With four workers at512 primary
   worlds, linked/native standalone totals are78.414/86.808 ms (1.11x).
   Input JSON takes24.962 ms, temp-file harness overhead1.271 ms, output parsing
   1.454 ms; replay is measured separately in every result. Child CPU totals are
   0.1119/0.1772 s. Actual-n generation over all128 proposed roots costs2.453 s;
   adding it reduces total gain to1.0033x. Holdout128/512 standalone gains are
   1.236x/1.203x; with generation1.0061x/1.0044x. These frontend costs are real
   for this executable/fixture harness and are not a production adapter model.

   PID-specific wait4 measures RSS for each child separately. Primary512 median
   RSS is59,473,920 bytes linked,56,606,720 predecessor and34,250,752 native
   recursion: there is **no uniform total-memory advantage**. Counted worker
   array peaks sum to27,866,720 bytes linked vs32,350,496 predecessor. They are
   a subset excluding maps/cache/query clones/allocator overhead, and a sum of
   worker maxima rather than simultaneous residency. Linked interner arrays
   are counted; the hash control's map allocation is excluded. Occupied States
   are per coordinate, but next-coordinate capacity reserves the projected row
   count, so allocated coordinate memory still scales with rows. Raw resources,
   sample times, cache-key bytes and warm correctness are retained separately.
   No natural online cache-hit claim follows. Phase timing sums aggregate worker elapsed
   intervals, not batch wall time or measured CPU; nested call scopes can also
   overlap. Moving record hashing between phases prevents simple phase attribution. Use the complete cold interval for speed comparisons.

8. **Policy component correctness improves; policy speed does not cross over.**
   On the same47 roots, four-worker full-frontier responses to modeled L0
   take31.407/131.989 ms at8/40 worlds, vs54.562/233.565 ms predecessor and
   2.640/18.922 ms native recursion. Unique actor misses and actual full-frontier
   samples remain10,845/43,380 and51,826/207,304; prefix factoring does not
   reduce them. These eager counts are not native recursion's minimum demand.
   Four workers have separate50k-query allowances, allowing the40-world batch
   with51,826 total unique misses to complete.

   Choice-only hybrid totals are8.145/32.942 ms vs8.504/36.672 predecessor and
   2.530/18.762 native recursion. It retains native `modeled_choice`, unchanged
   budgets and a separate choice cache; full actors() vectors remain complete.
   Its lower core nodes, misses and reported inner worlds are separately
   recorded, not mislabeled as frontier samples. Added cache hits can include
   duplicate complete actor specs within the same layer, not natural online
   reuse. [Full policy](results/final-policy0-four/summary.json),
   [hybrid](results/final-hybrid0-four/summary.json).

9. **Rung demand is a precise remaining obstacle.** On six common roots from
   deals962000–962031, plies16–19, eight outer worlds, fixed budgets and one
   worker, full policy field0/1/2 produces the following completed counts.
   A root best response to modeled Lr corresponds to the next response rung;
   this finite panel does not establish larger-k behavior.

   | Modeled field | Unique L0/L1/L2 misses | Inner samples | Row edges | Linked | Native recursion |
   | ---: | --- | ---: | ---: | ---: | ---: |
   | 0 | 446 | 1,784 | 41,483 | 2.201 ms | 0.458 ms |
   | 1 | 3,607 /468 | 15,364 | 251,201 | 12.424 ms | 1.383 ms |
   | 2 | 15,452 /3,708 /470 | 70,164 | 949,728 | 45.656 ms | 3.832 ms |

   The [2/8/40-world demand matrix](results/demand-matrix/summary.json) also
   tests the hybrid. At40 worlds, full field2 refuses the50k-query allowance
   after49,534 generated and49,202 completed actor queries; no root vector is
   emitted. Completed lower caches remain reusable. Hybrid completes with1,782
   explicit L2 choices and native-reported28,940/9,298/1,782 L0/L1/L2 misses,
   taking14.023 ms vs13.294 ms native in that single observation. All102
   completed matrix vectors match; the refused cell supplies no equality or
   hypothetical completed demand count. These are fixed-budget finite demand
   measurements, not a recurrence theorem. The experimental API still supports
   only late straight frames and fields/actor levels0–2.

10. **Next concrete experiment: demand-limited shared choice.** Keep full root
    vectors but give lower actor consumers a separate pure-frontier choice API.
    Propagate integer lower/upper totals and expand only unresolved demanded
    actor/action branches. For maximizing candidate a, exact ascending ties
    require L[a]>U[b] for every earlier b and L[a]>=U[b] for every later b;
    minimization uses U[a]<L[b] earlier and U[a]<=L[b] later. Unit-mass unresolved
    rows supply conservative bounds, and shared groups must sum before choice.
    Unknown lower-policy choices must remain unresolved rather than selected
    per hidden world. No universal all-world tree should be materialized.

    This is motivated by the measured full-vector/hybrid gap and the native
    core's existing choice pruning, not an asserted new speed gain. Freeze
    identical actor samples, RNG/tapes, ties, budgets and complete cache keys;
    compare complete root vectors and every demanded choice with both current
    full frontier and pinned native choice, under equal workers/caps. Measure
    demands saved per rung, sampling/cache costs, CPU/RSS and refusal behavior
    on the same six-root matrix plus a fresh panel. Proceed toward larger k
    only after exact choice parity and useful demand reduction. A complete
    adapter and balanced independent-deal matches would then be needed for
    any stronger-play claim; neither is supplied by this component checkpoint.

## Retained limits and failed setup receipts

All runs use the inherited process-group watchdog with allowance<=295 seconds;
actual completed runs are far below300 seconds. No external fixture export,
Claude/browser interaction, credential change, paid resources, merge or deploy
occurred. The first timing harness failed on Darwin time -l's sandbox-denied
sysctl; PID wait4 replaced it. The first Lean file used a reserved keyword and
failed parsing; the corrected file and independent rerun pass. Earlier
standalone timing omitted tempfile overhead; final v2 records explicitly charge
it. Failed receipts are retained and excluded from validation. The source/hash,
vector, scope and watchdog audit is [AUDIT.json](AUDIT.json).
