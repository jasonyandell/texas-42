---
number: 12
slug: carrier-staircase-burnside
channel: new-chat
status: cleared by Jason 2026-08-01 (explicit go, up to 8 requests Aug 1)
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
deliverable: exact orbit counts of j-edge subgraphs of K7-with-loops under S7 for all j = 0..28, pure and count-labeled, by cycle index — the complete carrier staircase, program-verified against direct canonicalization
---
You are performing adversarial combinatorial research for a formal foundation of Straight Texas 42 co-authored in a separate conversation. The attachment governs on any conflict. Adjudication is mechanical: your program is executed, your numbers diffed against independently computed values (several of the small cases are already frozen on the corpus side and are withheld deliberately). Hedged or unverifiable claims score zero.

## 1. Setup

The 28 dominoes are exactly the edges of K7-with-loops: vertices P = {0..6} (pips), one edge {i,j} for every unordered pair including i = j (21 proper edges + 7 loops). The symmetric group S7 acts on P and hence on the 28 edges. Count labels: c = 10 on the edges {5,5} and {6,4}; c = 5 on {5,0}, {4,1}, {3,2}; c = 0 on the other 23. Count labels are NOT S7-invariant; they matter only in the labeled variant below.

A **j-carrier** is a j-element subset of the 28 edges (the "live tiles"). Two j-carriers are isomorphic iff some element of S7 maps one to the other ("colors, not letters": pip names carry no meaning). A **count-labeled j-carrier** is a j-carrier together with the multiset structure induced by c restricted to it; isomorphism must additionally preserve each edge's count label — equivalently, orbits of the S7-action on subsets, refined so that only label-preserving permutations identify carriers. Precisely: define the labeled universe as the edge set with each edge carrying its fixed label in {0,5,10}; the labeled automorphism group G <= S7 is NOT the stabilizer of the labeling — rather, two labeled carriers A, B are equivalent iff some pi in S7 has pi(A) = B AND c(pi(e)) = c(e) for every e in A. Note this is orbit-counting of PAIRS (subset, labeling-restriction) under the full S7 action where the labeling is a fixed function being transported — think it through carefully and state the correct Burnside formulation; a naive "stabilizer subgroup" shortcut gives wrong answers.

## 2. THE TASK

1. **Pure staircase.** For every j = 0..28, the exact number a_j of isomorphism classes of j-carriers under S7. Method: Burnside/cycle-index — for each of the 15 conjugacy classes of S7 (cycle types of the vertex action), derive the induced cycle structure on the 28 edges (loops and proper edges separately; show the derivation for at least the classes 1^7, 2 1^5, 3 2 1^2, 7), then count fixed j-subsets per class via the generating polynomial prod (1 + x^{l_i}) over induced edge-cycle lengths l_i, and average. Present: the full table a_0..a_28, the generating polynomial sum a_j x^j, the total sum (orbits of all subsets), and the symmetry check a_j vs a_{28-j} (complementation — state whether it holds and why).
2. **Count-labeled staircase.** For every j = 0..28, the exact number b_j of count-labeled j-carrier classes as defined above. Derive the correct Burnside computation over the labeled structure (per conjugacy class, only permutations preserving the label function c pointwise on the whole edge set contribute fixed labeled subsets? — no: work out what "fixed" means for a transported labeling and get it right; this subtlety is the adversarial core of the dispatch). Present the full table b_0..b_28 and its total.
3. **Sanity anchors.** Your program must verify by direct canonicalization (explicit orbit enumeration over the <= 5040 vertex permutations) the values of a_j and b_j for all j <= 5 and, by complementation or feasibility, any larger j you can reach within the time budget — these overlap the corpus's frozen values and will be diffed. Also state a_4 and b_4 prominently (the last-trick carrier layer) and b_8 (the k=2 layer).
4. **Structure bonus (optional, scored if correct):** identify the sequence a_j (or its total) in OEIS if it exists, and give the count of ROLE-DECORATED 4-carriers: 4-carriers with one edge marked "led", one "partner", two unordered "opponent" (decorations transported by S7; labels preserved as in the count-labeled case, applied to the count-labeled variant). The corpus holds this number frozen too.

## 3. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. `FINAL ANSWER:` followed by the four headline numbers: a_4, b_4, b_8, and the role-decorated count-labeled 4-carrier count (or `UNATTEMPTED` for the last).
2. The derivation as numbered steps labeled `[USES: ...]` — the induced edge-cycle-structure table for all 15 conjugacy classes of S7 must appear in full, and the labeled-Burnside formulation must be stated as a theorem with proof.
3. One self-contained Python 3 program (single fenced block, stdlib only, deterministic, no network/file I/O, under 30 minutes one core) that computes both full staircases a_0..a_28 and b_0..b_28 by cycle index, verifies j <= 5 (both tables) by direct orbit enumeration, computes the role-decorated 4-carrier count by direct enumeration, prints every table row as `a[j]=... b[j]=...`, and exits 0 iff every cross-check passes with `PASS`/`FAIL` lines.
