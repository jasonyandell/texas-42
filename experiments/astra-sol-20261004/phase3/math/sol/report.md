# Sol phase 3: lawful compilation equivalence and dependency review

2026-10-04. EXPLORATORY. No production source or player changed.

## Verified abstract equivalence

`CompiledDAG.lean` formalizes a finite indexed graph in topological order:
node i may refer only to children of type `Fin i`. Nodes fold a fixed terminal
mass, SUM over opponent buckets, MAX over shared focal actions, or MIN with an
explicit initial ceiling. This typing prevents cycles and references to values
that have not been computed.

`compiled_prefix_agrees` proves that the abstract indexed-store prefix agrees
with structurally recursive evaluation at every completed node. Its root
corollary is uniform in every such graph. It does not assume that nodes form a
tree; sharing earlier nodes is permitted. Two additional exact two-world
theorems show that moving MAX or MIN before world aggregation changes semantics:
shared MAX gives 1 while world-first MAX gives 2; shared MIN gives 1 while
world-first MIN gives 0. The two hidden worlds have the same focal observation.

The final six theorem reports passed Lean v4.33.0-rc1 in 0.270 seconds under a
30-second group watchdog, logs `lean-compiled-dag-final-run/`. Dependencies are
only standard `propext` and `Quot.sound` (the two numerical counterexamples have
no axioms). No `sorry`, `native_decide`, or declared axiom was used.

The cache is an abstract persistent store represented as a function. This is a
semantic equivalence proof, not a claim that evaluating that Lean definition
uses constant-time table reads or has linear runtime. An actual array-based
implementation still needs independently checked indexing, adjacency and graph
construction invariants. The theorem deliberately does not prove the game-to-
graph compiler lawful or bound its compile time or memory.

## Required graph invariants

- Pin one root own hand, contract, complete public history, original scenarios
  and tapes, with explicit original multiplicities. Keep sampled world/tape
  identity and weights available; physically equal worlds with different tapes
  are different scenarios.
- At focal nodes share one action across the entire set of worlds consistent
  with that same observation. Root own hand is fixed and later remaining own
  hands are determined by public plays. Complete chronological history is a
  conservative key; using a reduced state needs its own sufficient-information
  argument. Never merge different observations because a mechanical position
  happens to look equal in a hidden world.
- At nonfocal nodes group scenarios by the actual observed tile. Each group
  carries its original mass. For an enumerated stochastic opponent, multiply
  cumulative mass by each branch likelihood before making later focal choices.
  The sum of child masses must equal the parent mass.
- Terminal mass is total successful scenario weight. Early termination must
  be irreversible under the actual contract. All focal root/action edges need
  complete children, canonical tie rules and explicitly oriented success counts.
- Fixed random tapes give deterministic model actions conditional on a
  scenario; they remain hidden to the focal policy. They model Dice/randomness,
  not automatically the modeled Walt L0 decision function. A Walt field needs
  its own pinned lawful query answers and query-cost accounting.

## What can and cannot be amortized

The mechanics of separate complete scenario/path rollouts are parallel once
their actions are fixed. Lawful policy optimization still couples worlds at
each focal observation. Backward folds depend on their children. This yields
parallel work within topological layers, not independence of all policy values.

A fully materialized graph can be evaluated in work proportional to its nodes
and edges if adjacency and value lookup are constant-time and arithmetic cost
is bounded. Building that graph may enumerate an exponential number of focal
histories. Compilation does not itself remove that growth. Report total node
and edge counts, width/depth, original scenario mass, cache queries, peak memory,
compilation plus evaluation time, and canonical-value parity with a direct
grouped recursive reference.

The tape-pruned graph depends on its tapes: changed dice can select different
opponent buckets, future leaders, legal action sets and termination points. A
universal skeleton containing every opponent edge can separate later tape
routing from some mechanics, but may be much larger than the adaptive graph.
Likewise, changing sampled worlds changes legal masks and bucket membership.
Changing a Walt field can require new lower-rung queries. A graph compiled for
one bundle cannot silently be reused for fresh bundles by changing leaf scores.

Amortization requires actual repeated evaluations of the same retained graph,
for example repricing terminal utilities on one frozen experiment bundle. If
the recursive reference costs R, graph evaluation costs E and compilation costs
C, a reuse count m saves time only if m(R−E)>C. Real play normally changes root
history and modeled posterior; charging C to only the first request requires a
demonstrated valid reusable skeleton. Full build plus first evaluation is the
primary comparison for a new root. GPU/vectorized throughput and Mac memory
feasibility remain measurements to make, not theorem consequences.

## Independent review of recovered tiny-network source

`phase2/incoming/rollout_net.py` was read as source only; its top-level generator,
training and comparison were not imported or run. It is the 174-line creation
version recovered by the lead from the authorized Claude conversation.

`gen_uniform:29–46` generates 14000 complete uniform games for one fixed own
hand and trump. Every live state from a game receives that game's eventual
outcome label. This teacher is a fixed lawful uniform rollout; it does not
optimize future actions per hidden world. The student inputs are categorical
own/unseen/played-by-seat-and-trick tiles plus public leader, points and current
trick length. `net_choice:123–142` uses a real deal to replay the mechanics, but
the computed network inputs and acting legal set use own/public information.
This source inspection found no direct opponent-hand feature passed to the net.

`rollout_net.py:81–82` randomly splits **rows**, not source games; related
histories and the same final label can occur in both train and validation.
The validation accuracy is therefore not held-out-game or held-out-hand
generalization. All training and test games fix one initial own hand and one
trump (`6–15,30,100`); the separate test stream at seed 7 tests new simulated
games for that hand, not cross-hand transfer or production matches.

The architecture has 56577 parameters: 840×64 embedding entries, 10×64 dense
weights, 64 first-layer biases, 64×32 second-layer weights, 32 biases, 32 output
weights and one output bias. Raw float32 parameters occupy 226308 bytes;
parameter plus two Adam-state copies occupy 678924 bytes, excluding gradients,
batches and data. A 512-row embedding gather holds 512×28×64 floats, 3670016
bytes before reduction. These are exact size calculations, not measured memory
or training throughput. The source is plain NumPy with `np.add.at`, not a GPU
training backend. The outer data/teacher work may dominate despite tiny weights.

The creation-version comparison at `rollout_net.py:148` calls the archive's
`engine42.walt_decide`, whose `230–234` takes per-world ordering optima before
aggregation. Thus its printed “Walt” regret is relative to a fused response to
uniform Dice, not the audited production lawful Walt L1. Its consistency replay
also inherits the dummy-bidder early-stop issue described in the phase 2 report.
This does not prove that every later reported tiny experiment used that same
benchmark; later source/logs must establish their own target provenance.

The source strengthens the target/validation diagnosis and makes a small
artifact-level feasibility investigation possible. It does not justify
repeating single-outcome training or treating its reported 79% as a useful
chooser. First fix target provenance, game-level splits and paired complete
action vectors with teacher uncertainty. No training has been executed here.

## Independent compiler and panel audits

`policy_audit.py` enumerated all 192 deterministic focal policies on a 12-ply
late root (source case 126), three legal root actions, two original scenarios,
and each of two frozen tapes. Policies receive only the complete public-history
coordinate, never world/tape identity. Explicit complete-play payoff maxima
equaled both the compiled vector and grouped recursive reference: tiles 5/12/17
had counts 1/0/1. Each tape-pruned graph had 108 coordinates; the universal
graph had 4059. Changing the tape on a tape-specific graph was rejected.
The bounded test passed in 0.090 seconds (`policy-audit-run/`).

`panel_audit.py` independently replayed all 15 retained source cases, 60 tapes
and 780 root action vectors from `phase3/results/panel-a`. It verified the
original source-plan hash, requests, rotated worlds, full 64-bit saved tapes,
recursive call counts, exact vectors at every bid 30–42, full-completion control,
universal graph object census and tape-specific active-node counts. Every check
passed in 14.911 seconds under a 30-second group watchdog (`panel-audit-run/`).
The compiler SHA256 is
`77e30a0df02852afd4730001e3996b3d68834966f73bbf109de373c4416e2fbd`.
The original retained panel completed in 6.785 seconds; no experiment remains
pending. This audit is evidence replay, not an additional timing sample.

Universal graphs totaled 592717 coordinates and 674101 scenario-support
incidences once per case. Across four tape routes, their physical coordinate
capacity was 2370868, while the adaptive routes visited 73104 coordinates and
93703 support incidences: a 32.43 ratio of physical capacity to visited
coordinates. The largest universal Python object census was 64894815 bytes,
not peak process RSS, and excludes a compact native/GPU representation claim.
The current prototype is a tree of grouped histories: `compile` appends each
unique full-history node without transposition merging. Reversing its indices
gives the child-before-parent order used in the abstract DAG theorem. Introducing
actual DAG reuse later requires correct support/flow merging; the present
single-coordinate dictionary reduction cannot silently overwrite multiple
incoming masks.

The original measured work totals for four tapes and thirteen bid payoffs per
root were universal compile+route+reduce 2726655 µs, adaptive
compile+route+reduce 570631 µs, and thirteen independent recursive responses
894150 µs. Universal compilation costs 3.049 times the recursive workload;
adaptive compilation costs 0.638 times it. The payoff repricing is real reuse
on one graph and is useful for exploring many objectives. Ordinary Walt asks
one fixed-bid query, so this workload does not establish a production gain.
All fifteen cases completed; refused-attempt time is zero. The updated parent
panel also retains refused-attempt duration for future cases.

## Coverage of opposing recursion

Bulk mechanics for outer sampled worlds do not by themselves cover nested
Walt best responses. The outer bundle conditions on the focal player's own
hand. A modeled opponent knows its own hand and public history but not that
focal hand; its inner posterior may include different focal hands absent from
every outer world. A mechanical universal graph on the outer bundle covers
legal histories for those worlds, not those additional belief fibers.

`InnerFiber.lean` gives an exact finite example: the outer actor knows x=false,
while the modeled opponent's constant observation supports one false and three
true worlds. Restricting its response to the outer support chooses false;
the lawful inner belief uniquely chooses true. Both the strict ranking reversal
and omitted positive inner-world mass are proved with no axioms. The bounded
run passed in 0.267 seconds (`lean-inner-fiber-run/`). This is an abstract
coverage counterexample, not a claimed Texas42 fixture or a proof that expanded
universal compilation is impossible. Precomputing separate belief supports and
pure-key field answers may address coverage but must charge their cost.

## Numbered falsifiable checkpoint claims

1. **Accepted, conditional theorem:** topologically indexed SUM/MAX/MIN graph
   evaluation equals its recursive semantics. Reproduce with Lean on
   `CompiledDAG.lean`; its graph-construction and performance limits above apply.
2. **Accepted, executable finite test:** on source case 126 and the two saved
   tapes, all 192 public-history policies per tape yield the same optimal root
   vector as the compiler. Reproduce `policy_audit.py` under `run_capped.py`.
3. **Accepted, retained-panel reproduction:** all 780 root vectors, tapes,
   worlds, recursive counts and graph censuses in the fifteen-case panel match
   the current hashed compiler. Reproduce `panel_audit.py` under the watchdog.
4. **Contradicted as a general claim:** optimizing each hidden world first
   preserves the lawful shared response. The strict 1-versus-2 MAX and
   0-versus-1 MIN Lean counterexamples refute it.
5. **Contradicted as a general claim:** outer-world bulk coordinates necessarily
   cover the modeled opponent's inner belief. `InnerFiber.lean` provides a
   strict opposite-response example because the outer support omits inner worlds.
6. **Accepted only for this repeated-objective workload:** thirteen-bid repricing
   amortizes the adaptive graph, whose retained measured total is 0.638 times
   thirteen recursive responses. Universal preprocessing is slower, at 3.049
   times. This is neither a phone benchmark nor a strength claim.
7. **Contradicted on this panel:** the universal mechanical skeleton avoids
   substantial unused preprocessing. Its four-tape coordinate capacity is 32.43
   times the active coordinates, and build costs reverse the speed comparison.
8. **Conjecture, not contradicted:** a compact native/GPU implementation or
   valid cross-request skeleton could make lawful batches cheaper. No throughput,
   memory-transfer, fixed-bid latency or production quality result establishes it.
9. **Accepted scope-limited certificate evidence:** the first-disagreement
   certificate is mathematically safe for frozen fields and audited inputs, but
   certificate plus fresh fallback is slower on all four retained native cost
   panels. Cache-reusing fallback and other safe replacements remain untested.
10. **Contradicted for the recovered creation source:** its random-row validation
    is held-out-game/hand validation, or its named comparison is production Walt.
    `rollout_net.py:81–82` mixes game rows, while its line-148 comparator invokes
    the archive's fused Dice ordering routine. The original uniform teacher is
    lawful; these defects do not prove a fusion teacher or invalidate every later
    tiny variant. Training weights and later source/logs remain absent.

Fresh-team handoff should preserve these exact hashes, graph/tape semantics,
conditional-proof scopes, observed cost accounting and source-version distinctions.
No training, deployment or original-checkout edits occurred.

## Final coverage and fixed-bid accounting audit

`coverage_audit.py` independently recompiled each universal graph from the new
`phase3/results/panel-coverage` saved requests/worlds, rerouted all four saved
tapes, ORed scenario masks per coordinate, and recomputed the union node/support
counts and every per-depth physical/used count. It also checked the bid-30
vectors and recursive counts, semantic parity with the earlier panel, and every
single-bid timing sum against saved receipts. All checks passed in 3.019 seconds
under a 30-second group watchdog (`coverage-audit-run/`). Compiler hash is
unchanged. Fifteen cases and sixty tape queries completed; no refusal or pending
job remains.

The **true union** across these four tapes uses 58835 of 592717 coordinates,
leaving 533882 coordinates, or **90.074%**, unused by any of those tapes.
Union support incidences are 71816; repeated visits across individual tapes are
73104 coordinates and 93703 support incidences. The earlier 32.43 figure was
four times physical capacity divided by repeated active visits, not a union
unused fraction. Later tapes could use currently unused coordinates; this is
measured four-tape coverage, not a proof of permanent irrelevance.

Width grows sharply in the universal skeleton: at depth 8 it has 93546 physical
coordinates versus 8130 union-used; at depth 12 it has 99678 versus 7227.
The audit stdout retains all depths 0–12. Thus the proposal covers many more
legal histories than these frozen tapes actually need, rather than having
eliminated history growth.

For **bid 30 only** across the same sixty saved queries, the retained recursive
total is 120290 µs. Universal compilation (charged once per root across all
four tapes), routing and bid-30 reduction total 2472145 µs, **20.552×** the
recursive work. Adaptive compile+route+bid-30 reduction totals 359957 µs,
**2.992×**. With an already available universal graph, route+reduce costs
78225 µs, **0.650×** recursive work; that warm result excludes the 2393920 µs
build cost. This is actual fixed-payoff accounting on the frozen research
bundle, still a Python Dice/tape prototype rather than production Walt/phone.

The earlier adaptive thirteen-payoff advantage is therefore an amortization
result, not evidence that compiling each fixed-bid decision is cheaper. Claim 6
remains accepted in its thirteen-objective scope; claim 7 is strengthened by
the true four-tape union-unused measurement. The native/GPU/cross-request reuse
conjecture remains open.
