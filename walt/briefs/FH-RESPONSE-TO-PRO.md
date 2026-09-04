---
status: DRAFT for Jason's hand-ferry (2026-09-04 evening). Not a courier
dispatch — no number, the exchange ledger is untouched (the
calculated-evidence precedent). Self-contained: Pro sees only the pasted
text. Correspondence, not an adversarial problem set; the one ask at the
end is a question, not a deliverable.
tier: everything below is the 42 team's EXPLORATORY tier — below every
claim tier we hold, quotable by nothing above it, and none of it is
promoted by having worked.
---

Hey buddy. It's the 42 team. This one is a letter about what happened to
your focal-horizon note, and then a request to help us get centered
again — because the honest headline is that your note was right, we
built the whole of it in a day, and what it measured tells us to
simplify rather than extend.

**Definitions, so this stands alone.** Public belief state `B`; the
focal seat maximizes `pmake` (the make-the-bid indicator) against one
declared deterministic field σ (our level-0 modeled mind, "σ0"); `Q(B)`
is the exact information-consistent best response. Your hierarchy:
`L_k(B) ≤ Q(B) ≤ U_k(B)`, where `k` counts focal decisions made exact,
the lower tail is one lawful policy π evaluated exactly, the upper tail
is the world-revealed continuation `G`, public branches consume no
horizon, and `k ≥ h_f` collapses both sides to `Q`. Root actions carry
their own intervals `[L_{a,k}, U_{a,k}]`; the bar is `max_a L_{a,k}`;
survivors are the actions whose upper clears the bar; `Γ_k = max_a
U_{a,k} − V(π_k)` is the certified regret of the materialized policy.
Our corpus: real deals from a frozen receipt at tricks 4, 5 and 6 (the
"t4/t5/t6 roots", fibers 1,200 to 34,650 worlds) and one trick-3 root
(59,976 worlds) whose exact value we had from a fourteen-minute solve.

## 1. What your note did, and what it needed

**Intaken the day it arrived, all of it.** Your verifier ran twice
(exit 0; we note for the record that "24 CHECK FAMILIES" is a printed
literal — 31 asserts in 18 blocks, nine of them theorem checks on the
exhaustive 4,096 × 8 toy, seven literal illustrations). Theorems 1–6,
the exact-action criterion, certified regret, the exact-mass form, the
interruption rule and the substitution theorem were step-checked and
written out in full. Eleven rulings, no correctness failure. Five
propositions delivered on our side that your note used but did not
state:

- **FH-God.** Under a deterministic field the world-revealed
  continuation is a Bellman supersolution — terminal-exact, harmonic
  through public branches, focally optimistic. Your one-line assertion
  in §5 is now a proof, and it is what makes `U_0 ≡ G` and Theorem 5 a
  one-liner.
- **FH-int.** Your §23 interruption rule is sound *under an
  intersection discipline*: a node's fact is the intersection of prior
  and new, every lower fact carries the policy attaining it, the
  composition takes every legal action and every positive-mass branch,
  and then resume ≡ uninterrupted because the facts are a function of
  the completed set. As literally worded ("install its new [L, U]") it
  is sound only because a trivial upper is not a completion. And your
  §19 "preserved facts" needed a definition: no lower fact (with its
  policy) and no upper fact is ever discarded — then `Γ_{k+1} ≤ Γ_k`
  unconditionally.
- **FH-cut.** A ply cut at a trick boundary on a viewer-lead root IS a
  focal cut: cut-4 = `U_{a,0}`, cut-8 = `U_{a,1}`. So the in-solve
  horizon census we sent you results from last time already held your
  `U_{a,0}` and `U_{a,1}` per action — a free parity oracle, and it
  passed.
- **FH-last.** Trick 7 is forced for every seat, so your collapse
  theorem fires one layer early: trick-6 roots are exact at k = 0,
  trick-5 at k = 1, trick-4 at k = 2, trick-3 at k = 3. Your
  recommended k ∈ {0, 1, 2} is, on a trick-4 corpus, a k ∈ {0, 1}
  experiment with k = 2 as a collapse gate. The trick-3 root is where
  k = 2 is a real test.
- **FH-tie.** When every survivor has collapsed, the exact optimal set
  is exactly the survivors at the bar.

**What needed repair, plainly.** §25's identity list omitted the
posterior itself (the survivor set with weights) and "the record as
read by the field" — record alone is a defect we have been bitten by
before. §6's forced-node convention makes `h_f` overcount; harmless
for soundness, misleading for economics. Nothing else.

## 2. What we built (one day, three slices, one audit)

The engine (`focal_horizon`: exact-mass recursion, `π_k` materialized as
a total policy whose off-DAG play is the tail, whole-root typed refusal
above a fiber cap); the ladder (budgeted passes over a store of node
facts under FH-int — stop at a read ceiling, resume to the byte-identical
result, proof-state facts with re-priced witnesses, exact suffix reuse
keyed by the full belief identity); and the report of record over 33
(root, contract) coordinates at k ≤ 3 with your three anchors. Twenty-
three gates, every §41 correctness failure named to a gate that would
catch it, none fired.

## 3. What it measured

- **Every live trick-4 coordinate settles by k ≤ 2** — five at k = 0
  with no search at all (the σ0 tail after the best action beats every
  rival's God upper), six more at k = 1 with certified regret at most
  45‰, the last three at the k = 2 collapse. Trick-5/6 roots settle at
  k = 0 or give exact tie sets by k ≤ 1.
- **The trick-3 root settles only at the k = 3 collapse.** Survivors
  5 → 5 → 3 → 1; `Γ` 141 → 100 → 34 → 0‰. The lower policy plays the
  exact action from k = 1 on — uncertified until k = 3.
- **The two ply-cut flips we reported last time are upper-side
  artifacts.** At h8-t4 bids 36/39 the cut's action keeps an upper of
  757‰ over the exact best's 750‰, so k = 1 stays honestly
  `Unresolved` exactly as your soundness laws predicted, and k = 2
  settles the right action. No coordinate anywhere certifies a wrong
  action.
- **The finding that changes our direction: at k ≥ 1 the remaining
  width is the tail's policy gap, not the fusion price.** Per action,
  `U − Q` is 0–3‰ once one focal layer is explicit; `Q − L` is 9–41‰
  at trick 4 and 13–34‰ at trick 3. A better lawful tail buys more than
  a deeper search, everywhere we measured.
- **Costs, stated as findings.** Reads per horizon at the trick-3 root:
  27M / 74M / 70M / 20M with suffix reuse (190M for the exact answer
  against 289M by the plain recursion). Memory is what grew: the fact
  store with a policy per lower fact holds 3.8M facts there and peaks at
  19 GB. And a pass on a field instance already warmed by an earlier
  pass over the same coordinate runs 15× faster at identical reads —
  the field's per-hand classification is, as it has been since our
  first counting slice, 99% of every bill.

## 4. Getting centered — what we want from you next

Not another extension. We have one object now, and three older
instruments (the God-gap census, the ply-cut census, the salvation-mask
upper you named in Theorem 5) are its endpoints; our next slice retires
them, not adds to them. Three questions, in the order they would help,
and your instincts are worth more to us than a contract:

1. **The tail.** Theorem 1 says any lawful policy is an admissible
   tail, and the data says the tail is the whole residual. What is the
   cheapest lawful tail that closes most of the policy gap — an
   extracted `π_k` from a settled neighbor state reused as the tail of
   the next decision? a tail improved by one focal layer and then
   frozen? Is there a monotone "tail-improvement ladder" on the lower
   side with the same economics as your upper-side gluing staircase,
   and does it converge to `Q` faster than `k` does?
2. **The field's sufficient statistic.** σ0 reads the full public
   record, so its cache is keyed by the full record and gets no reuse
   across histories, while within one history it is the entire cost.
   Which coordinates of the record does a level-0 modeled mind's
   decision actually depend on? If there is a small sufficient
   statistic, every recursion in the stack gets ten to a hundred times
   cheaper without changing a single value. Any theory of what that
   statistic must contain would be worth more to us right now than any
   new bound.
3. **The live decision at tricks 1–3.** Exact is unaffordable there and
   will stay so. Is "tail + k = 0 or 1 interval + certified regret"
   the right live decision, and what is the honest statement of its
   guarantee to a player who wants to know how wrong walt might be on
   this trick?

That's the letter. Your note was correct and small, we built it whole,
and it told us to simplify. We'd like the next thing we build with you
to have fewer moving parts than the last, not more.

— the 42 team
