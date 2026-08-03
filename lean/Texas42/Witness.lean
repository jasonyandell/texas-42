/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Strategic

/-!
# Texas 42 — the exact legal 90-world witness (PA-E10, Math §10.4)

The named counterexample of the acceptance standard (K15): inside
no-trump Straight 42, two legal public histories share the same
mechanical endpoint — same contract, same five-trick prefix, same
90-world rule fiber, same posterior support — yet their auction paths
tilt the posterior weights so that opposite root leads are optimal,
under both the signed-differential and contract-make lenses (the
remaining two lenses of §10.4 are positive affine transforms of these).
Mechanical state alone is not an exact strategic state.

Everything here is internal: the fiber is enumerated and checked
against the cells, the posteriors are exact rationals, and the values
`Q` come from kernel-evaluated deterministic lowest-ID rollouts of the
committed play machinery (`PlayState.step`) — no external receipt is
imported (TRUST-01).
-/

namespace Texas42

set_option maxRecDepth 1000000

namespace Witness

/-- Domino literal helper. -/
def D (h l : ℕ) (hh : h < 7 := by decide) (hl : l < 7 := by decide)
    (hle : l ≤ h := by decide) : Domino :=
  ⟨⟨h, hh⟩, ⟨l, hl⟩, hle⟩

/-- The canonical global domino identifier (Math §10.4's DominoId
order: `0-0, 1-0, 1-1, 2-0, …` — high pip major, low pip minor). -/
def dominoId (d : Domino) : ℕ := d.high.val * 7 + d.low.val

theorem dominoId_inj : ∀ a b : Domino, dominoId a = dominoId b → a = b := by
  decide +kernel
/-- The lowest-ID member of the legal set — the §10.4 post-endpoint
field policy. -/
def lowestLegal (X : PlayState) : Domino :=
  if h : (X.legalSet.image dominoId).Nonempty then
    (X.legalSet.filter
        (fun d => dominoId d = (X.legalSet.image dominoId).min' h)).choose
      (fun _ => True) (by
        obtain ⟨d, hd, hid⟩ :=
          Finset.mem_image.mp (Finset.min'_mem _ h)
        refine ⟨d, ⟨Finset.mem_filter.mpr ⟨hd, hid⟩, trivial⟩, ?_⟩
        rintro b ⟨hb, -⟩
        rw [Finset.mem_filter] at hb
        exact dominoId_inj b d (hb.2.trans hid.symm))
  else ⟨0, 0, le_refl _⟩

/-- Run the lowest-ID field for `n` plies. -/
def rollout : ℕ → PlayState → PlayState
  | 0, X => X
  | n + 1, X => rollout n (X.step (lowestLegal X))

/-! ## The §10.4 scenario -/

/-- The common contract: seat 3 bid `P(31)`, no-trump. -/
def theContract : Contract :=
  ⟨3, BidKind.point, 31, Declaration.notrump⟩

/-- The tiles publicly played by each seat over the five tricks. -/
def played (s : Seat) : Finset Domino :=
  if s = 0 then {D 6 1, D 2 0, D 5 4, D 1 1, D 1 0}
  else if s = 1 then {D 6 4, D 0 0, D 4 3, D 3 0, D 6 6}
  else if s = 2 then {D 6 0, D 2 2, D 4 2, D 3 3, D 5 2}
  else {D 6 3, D 5 0, D 4 0, D 2 1, D 5 1}

/-- The twenty plays of the common public prefix, in play order. -/
def playList : List Domino :=
  [D 6 3, D 6 1, D 6 4, D 6 0,
   D 0 0, D 2 2, D 5 0, D 2 0,
   D 4 3, D 4 2, D 4 0, D 5 4,
   D 1 1, D 3 0, D 3 3, D 2 1,
   D 1 0, D 6 6, D 5 2, D 5 1]

/-- The viewer's remaining hand. -/
def viewerHand : Finset Domino := {D 4 1, D 3 1}

/-- The six-tile unseen pool. -/
def poolW : Finset Domino :=
  {D 5 5, D 4 4, D 3 2, D 6 5, D 5 3, D 6 2}

/-- The 90-world fiber, enumerated: seat 0's pair and seat 1's pair
(seat 2 takes the rest). -/
def worldPairs : Finset (Finset Domino × Finset Domino) :=
  (poolW.powersetCard 2).biUnion fun h0 =>
    ((poolW \ h0).powersetCard 2).image fun h1 => (h0, h1)

/-- Math §10.4: `|Φ(c)| = 6!/(2!)³ = 90`. -/
theorem card_worldPairs : worldPairs.card = 90 := by decide +kernel
/-- The hidden hands of a world. -/
def hiddenHands (w : Finset Domino × Finset Domino) :
    Seat → Finset Domino := fun s =>
  if s = 0 then w.1
  else if s = 1 then w.2
  else if s = 2 then poolW \ (w.1 ∪ w.2)
  else viewerHand

/-- The full original deal of a world: played tiles plus remainder. -/
def dealHands (w : Finset Domino × Finset Domino) :
    Seat → Finset Domino := fun s => played s ∪ hiddenHands w s

/-- The endpoint play state of a world. -/
def endpoint (w : Finset Domino × Finset Domino) : PlayState :=
  { hands := hiddenHands w
    leader := 3
    trick := []
    score := fun t => if t = (0 : Team) then 2 else 18
    contract := theContract }

/-- The initial play state of a world's deal. -/
def start (w : Finset Domino × Finset Domino) : PlayState :=
  { hands := dealHands w
    leader := 3
    trick := []
    score := fun _ => 0
    contract := theContract }

/-- Exact signed differential on the remaining 22 points after leading
`d` at the endpoint, under the lowest-ID field (Math §10.4). -/
def Q (w : Finset Domino × Finset Domino) (d : Domino) : ℤ :=
  let Xf := rollout 7 ((endpoint w).step d)
  ((Xf.score 1 : ℤ) - 18) - ((Xf.score 0 : ℤ) - 2)

/-! ## The §10.4 pointwise anchors -/

private theorem anchor1 :
    Q ({D 5 5, D 4 4}, {D 3 2, D 6 5}) (D 3 1) = 10 := by decide +kernel
private theorem anchor2 :
    Q ({D 5 5, D 4 4}, {D 3 2, D 6 5}) (D 4 1) = -22 := by decide +kernel
private theorem anchor3 :
    Q ({D 5 5, D 6 5}, {D 3 2, D 4 4}) (D 3 1) = -22 := by decide +kernel
private theorem anchor4 :
    Q ({D 5 5, D 6 5}, {D 3 2, D 4 4}) (D 4 1) = 22 := by decide +kernel

set_option maxRecDepth 100000 in
/-- Math §10.4 anchor table: the two displayed worlds' exact values. -/
theorem anchor_values :
    Q ({D 5 5, D 4 4}, {D 3 2, D 6 5}) (D 3 1) = 10
      ∧ Q ({D 5 5, D 4 4}, {D 3 2, D 6 5}) (D 4 1) = -22
      ∧ Q ({D 5 5, D 6 5}, {D 3 2, D 4 4}) (D 3 1) = -22
      ∧ Q ({D 5 5, D 6 5}, {D 3 2, D 4 4}) (D 4 1) = 22 := by
  exact ⟨anchor1, anchor2, anchor3, anchor4⟩

/-! ## The 90 worlds as an indexed list

The subtype-of-`Finset` route defeats kernel reduction (attach/pmap
normal forms), so the fiber is carried as an explicit computable list
indexed by `Fin 90`, and tied back to the `powersetCard` enumeration by
a decidable image equality. -/

/-- The pool in canonical ID order. -/
def poolList : List Domino := [D 3 2, D 4 4, D 5 3, D 5 5, D 6 2, D 6 5]

/-- The 90 worlds: seat 0's pair and seat 1's pair as sublists. -/
def worldsList : List (List Domino × List Domino) :=
  (poolList.sublistsLen 2).flatMap fun h0 =>
    ((poolList.filter (· ∉ h0)).sublistsLen 2).map fun h1 => (h0, h1)

theorem worldsList_length : worldsList.length = 90 := by decide +kernel
theorem worldsList_nodup : worldsList.Nodup := by decide +kernel
/-- The indexed world. -/
def wAt (i : Fin 90) : Finset Domino × Finset Domino :=
  let p := worldsList.get (Fin.cast worldsList_length.symm i)
  (p.1.toFinset, p.2.toFinset)

/-- The list enumeration is exactly the `powersetCard` fiber. -/
theorem wAt_image : Finset.univ.image wAt = worldPairs := by decide +kernel
/-! ## The two auction-induced posteriors (Math §10.4) -/

/-- The auction likelihood of history `A` (`0:pass, 1:P(30), 2:pass,
3:P(31)`): seat 0 declined at its `4-4`-graded bidding propensity,
seat 1 bid at its propensity. -/
def lA (w : Finset Domino × Finset Domino) : ℚ :=
  if D 4 4 ∈ w.1 then 1 / 9 else if D 4 4 ∈ w.2 then 4 / 9 else 2 / 9

/-- The auction likelihood of history `B` (`0:P(30), 1:pass, 2:pass,
3:P(31)`): seat 0 bid at its propensity; `P(30)` is then illegal for
seat 1, which passes with probability one. -/
def lB (w : Finset Domino × Finset Domino) : ℚ :=
  if D 4 4 ∈ w.1 then 2 / 3 else 1 / 3

/-- The uniform deal prior restricted to the fiber (Math §10.4: the 90
worlds are equiprobable under the chance law). -/
def uniformW : FinPMF (Fin 90) where
  mass := fun _ => 1 / 90
  nonneg := fun _ => by norm_num
  total := by
    rw [Finset.sum_const, Finset.card_univ, Fintype.card_fin,
      nsmul_eq_mul]
    norm_num

theorem lA_pos (w : Finset Domino × Finset Domino) : 0 < lA w := by
  unfold lA
  split_ifs <;> norm_num

theorem lB_pos (w : Finset Domino × Finset Domino) : 0 < lB w := by
  unfold lB
  split_ifs <;> norm_num

theorem ZA_pos : 0 < ∑ i : Fin 90, uniformW.mass i * lA (wAt i) :=
  Finset.sum_pos
    (fun i _ => mul_pos (by norm_num [uniformW]) (lA_pos (wAt i)))
    Finset.univ_nonempty

theorem ZB_pos : 0 < ∑ i : Fin 90, uniformW.mass i * lB (wAt i) :=
  Finset.sum_pos
    (fun i _ => mul_pos (by norm_num [uniformW]) (lB_pos (wAt i)))
    Finset.univ_nonempty

/-- The history-`A` posterior, by Bayes conditioning (PA-E02). -/
def μA : FinPMF (Fin 90) :=
  uniformW.condition (fun i => lA (wAt i))
    (fun i => (lA_pos (wAt i)).le) ZA_pos

/-- The history-`B` posterior, by Bayes conditioning (PA-E02). -/
def μB : FinPMF (Fin 90) :=
  uniformW.condition (fun i => lB (wAt i))
    (fun i => (lB_pos (wAt i)).le) ZB_pos

/-- Math §10.4: both posteriors give every one of the 90 worlds strictly
positive probability — the posterior supports are identical (the whole
rule fiber); only the weights differ. -/
theorem same_full_support (i : Fin 90) :
    0 < μA.mass i ∧ 0 < μB.mass i := by
  constructor <;>
    exact div_pos
      (mul_pos (by norm_num [uniformW]) (by first
        | exact lA_pos (wAt i)
        | exact lB_pos (wAt i)))
      (by first | exact ZA_pos | exact ZB_pos)

/-! ## Expectations through integer moments

ℚ arithmetic does not kernel-reduce (`Nat.gcd` is well-founded), so the
fiber moments are computed by `decide` in ℤ against integer likelihood
weights, and the exact rational expectations follow analytically. -/

/-- History-`A` integer likelihood weights (`lA = weightA/9`). -/
def weightA (w : Finset Domino × Finset Domino) : ℤ :=
  if D 4 4 ∈ w.1 then 1 else if D 4 4 ∈ w.2 then 4 else 2

/-- History-`B` integer likelihood weights (`lB = weightB/3`). -/
def weightB (w : Finset Domino × Finset Domino) : ℤ :=
  if D 4 4 ∈ w.1 then 2 else 1

theorem lA_eq_weight (w : Finset Domino × Finset Domino) :
    lA w = (weightA w : ℚ) / 9 := by
  unfold lA weightA
  split_ifs <;> norm_num

theorem lB_eq_weight (w : Finset Domino × Finset Domino) :
    lB w = (weightB w : ℚ) / 3 := by
  unfold lB weightB
  split_ifs <;> norm_num

/-- Conditioning the uniform prior reduces expectations to likelihood
ratios. -/
theorem condition_uniform_exp (L : Fin 90 → ℚ) (hL : ∀ i, 0 ≤ L i)
    (hZ : 0 < ∑ i, uniformW.mass i * L i) (f : Fin 90 → ℚ) :
    (uniformW.condition L hL hZ).exp f
      = (∑ i, L i * f i) / (∑ i, L i) := by
  unfold FinPMF.exp FinPMF.condition
  dsimp only
  have hmass : ∀ i : Fin 90, uniformW.mass i = 1 / 90 := fun _ => rfl
  have hZeq : (∑ i, uniformW.mass i * L i) = 1 / 90 * ∑ i, L i := by
    rw [Finset.mul_sum]
    exact Finset.sum_congr rfl fun i _ => by rw [hmass i]
  have hnum : (∑ i, uniformW.mass i * L i
        / (∑ j, uniformW.mass j * L j) * f i)
      = (1 / 90 * ∑ i, L i * f i) / (∑ j, uniformW.mass j * L j) := by
    rw [Finset.mul_sum, Finset.sum_div]
    exact Finset.sum_congr rfl fun i _ => by
      rw [hmass i, div_mul_eq_mul_div, mul_assoc]
  rw [hnum, hZeq, mul_div_mul_left _ _ (by norm_num : (1 : ℚ) / 90 ≠ 0)]

/-- Expectation under a scaled integer weighting is the integer moment
ratio. -/
theorem exp_weight_ratio (wt : Fin 90 → ℤ) (c : ℚ) (hc : 0 < c)
    (L : Fin 90 → ℚ) (hL : ∀ i, 0 ≤ L i)
    (hZ : 0 < ∑ i, uniformW.mass i * L i)
    (hLw : ∀ i, L i = (wt i : ℚ) / c) (f : Fin 90 → ℤ) :
    (uniformW.condition L hL hZ).exp (fun i => (f i : ℚ))
      = ((∑ i, wt i * f i : ℤ) : ℚ) / ((∑ i, wt i : ℤ) : ℚ) := by
  rw [condition_uniform_exp]
  have hnum : (∑ i, L i * (f i : ℚ))
      = (((∑ i, wt i * f i : ℤ) : ℚ)) / c := by
    push_cast
    rw [Finset.sum_div]
    exact Finset.sum_congr rfl fun i _ => by
      rw [hLw i, div_mul_eq_mul_div]
  have hden : (∑ i, L i) = (((∑ i, wt i : ℤ) : ℚ)) / c := by
    push_cast
    rw [Finset.sum_div]
    exact Finset.sum_congr rfl fun i _ => hLw i
  rw [hnum, hden, div_div_div_comm, div_self (ne_of_gt hc), div_one]

/-! ## The kernel-verified value tables

Each column of §10.4's exact backward induction, one entry per world,
verified once by `decide` (180 deterministic rollouts total); every
later moment is cheap arithmetic against these tables. -/

/-- The 90 values `Q(·, 3:1)` in world order. -/
def qTable31 : List ℤ :=
  [22, 10, 22, -22, -10, -22, 10, -22, 22, -22, 10, -22, 22, -10, 22,
   -22, 10, -22, -22, -22, -22, -22, -22, -22, -10, 10, -22, 10, -22,
   -22, -22, -22, -22, -22, -22, -22, -22, -10, 22, -22, -22, 10, -10,
   22, 22, -22, -22, 10, -22, 22, -10, 10, -22, -10, -22, -22, -22,
   -22, -22, -22, -22, -10, 22, -22, -22, 22, -22, 22, 22, -22, -22,
   22, -22, 22, -10, 10, -22, 22, -22, -10, -22, -10, -22, 10, -22,
   -10, -22, 22, 10, 22]

/-- The 90 values `Q(·, 4:1)` in world order. -/
def qTable41 : List ℤ :=
  [-22, 22, 0, 0, -22, 22, -22, 20, 20, -22, -22, 22, -22, 22, 20,
   -20, -22, 22, -22, 0, 22, -22, -22, 22, -22, 22, 22, -22, -22, 22,
   -22, 22, 0, 0, -22, 22, -22, -22, -22, -22, -22, -22, -20, 0, -22,
   0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -20, -22, -20, -22,
   0, -22, -22, -22, 0, 22, 20, -20, -22, -22, 22, 22, 20, -22, 0,
   -22, 22, 0, 0, -22, -20, -22, 22, 20, 22, -22, -22, -22, -22, -22,
   -22]

set_option maxHeartbeats 0 in
/-- The `3:1` column is exact (90 kernel rollouts). -/
theorem Q31_table : ∀ i : Fin 90,
    Q (wAt i) (D 3 1) = qTable31.get (Fin.cast (by decide) i) := by
  decide +kernel
set_option maxHeartbeats 0 in
/-- The `4:1` column is exact (90 kernel rollouts). -/
theorem Q41_table : ∀ i : Fin 90,
    Q (wAt i) (D 4 1) = qTable41.get (Fin.cast (by decide) i) := by
  decide +kernel
/-! ## The exact fiber moments -/

set_option maxHeartbeats 0 in
theorem sum_weightA : (∑ i : Fin 90, weightA (wAt i)) = 210 := by decide +kernel
set_option maxHeartbeats 0 in
theorem sum_weightB : (∑ i : Fin 90, weightB (wAt i)) = 120 := by decide +kernel
set_option maxHeartbeats 0 in
theorem momentA31 :
    (∑ i : Fin 90, weightA (wAt i) * Q (wAt i) (D 3 1)) = -1600 := by
  simp only [Q31_table]
  decide +kernel
set_option maxHeartbeats 0 in
theorem momentA41 :
    (∑ i : Fin 90, weightA (wAt i) * Q (wAt i) (D 4 1)) = 300 := by
  simp only [Q41_table]
  decide +kernel
set_option maxHeartbeats 0 in
theorem momentB31 :
    (∑ i : Fin 90, weightB (wAt i) * Q (wAt i) (D 3 1)) = -868 := by
  simp only [Q31_table]
  decide +kernel
set_option maxHeartbeats 0 in
theorem momentB41 :
    (∑ i : Fin 90, weightB (wAt i) * Q (wAt i) (D 4 1)) = -1248 := by
  simp only [Q41_table]
  decide +kernel
/-- The make indicator: the contract succeeds iff `q ≥ 4` (Math §10.4:
declaring points are `(q+22)/2`, make needs 31). -/
def makeInd (w : Finset Domino × Finset Domino) (d : Domino) : ℤ :=
  if 4 ≤ Q w d then 1 else 0

set_option maxHeartbeats 0 in
theorem makeA31 :
    (∑ i : Fin 90, weightA (wAt i) * makeInd (wAt i) (D 3 1)) = 70 := by
  simp only [makeInd, Q31_table]
  decide +kernel
set_option maxHeartbeats 0 in
theorem makeA41 :
    (∑ i : Fin 90, weightA (wAt i) * makeInd (wAt i) (D 4 1)) = 96 := by
  simp only [makeInd, Q41_table]
  decide +kernel
set_option maxHeartbeats 0 in
theorem makeB31 :
    (∑ i : Fin 90, weightB (wAt i) * makeInd (wAt i) (D 3 1)) = 40 := by
  simp only [makeInd, Q31_table]
  decide +kernel
set_option maxHeartbeats 0 in
theorem makeB41 :
    (∑ i : Fin 90, weightB (wAt i) * makeInd (wAt i) (D 4 1)) = 24 := by
  simp only [makeInd, Q41_table]
  decide +kernel
/-! ## Math §10.4: the exact expectations and the reversal -/

set_option maxHeartbeats 4000000 in
/-- The exact expected signed differentials over the full fiber — the
§10.4 table, kernel-exact: `μ_A` prefers the `4:1` lead, `μ_B` prefers
the `3:1` lead. -/
theorem expected_differentials :
    μA.exp (fun i => (Q (wAt i) (D 3 1) : ℚ)) = -160 / 21
      ∧ μA.exp (fun i => (Q (wAt i) (D 4 1) : ℚ)) = 10 / 7
      ∧ μB.exp (fun i => (Q (wAt i) (D 3 1) : ℚ)) = -217 / 30
      ∧ μB.exp (fun i => (Q (wAt i) (D 4 1) : ℚ)) = -52 / 5 := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · rw [show μA.exp (fun i => (Q (wAt i) (D 3 1) : ℚ))
        = ((∑ i, weightA (wAt i) * Q (wAt i) (D 3 1) : ℤ) : ℚ)
          / ((∑ i, weightA (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 9 (by norm_num) _ _ _
        (fun i => lA_eq_weight (wAt i)) _,
      momentA31, sum_weightA]
    norm_num
  · rw [show μA.exp (fun i => (Q (wAt i) (D 4 1) : ℚ))
        = ((∑ i, weightA (wAt i) * Q (wAt i) (D 4 1) : ℤ) : ℚ)
          / ((∑ i, weightA (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 9 (by norm_num) _ _ _
        (fun i => lA_eq_weight (wAt i)) _,
      momentA41, sum_weightA]
    norm_num
  · rw [show μB.exp (fun i => (Q (wAt i) (D 3 1) : ℚ))
        = ((∑ i, weightB (wAt i) * Q (wAt i) (D 3 1) : ℤ) : ℚ)
          / ((∑ i, weightB (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 3 (by norm_num) _ _ _
        (fun i => lB_eq_weight (wAt i)) _,
      momentB31, sum_weightB]
    norm_num
  · rw [show μB.exp (fun i => (Q (wAt i) (D 4 1) : ℚ))
        = ((∑ i, weightB (wAt i) * Q (wAt i) (D 4 1) : ℤ) : ℚ)
          / ((∑ i, weightB (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 3 (by norm_num) _ _ _
        (fun i => lB_eq_weight (wAt i)) _,
      momentB41, sum_weightB]
    norm_num

set_option maxHeartbeats 4000000 in
/-- The exact contract-make probabilities (`make ↔ q ≥ 4`, Math §10.4):
the same reversal under the contract lens. -/
theorem make_probabilities :
    μA.exp (fun i => (makeInd (wAt i) (D 3 1) : ℚ)) = 1 / 3
      ∧ μA.exp (fun i => (makeInd (wAt i) (D 4 1) : ℚ)) = 16 / 35
      ∧ μB.exp (fun i => (makeInd (wAt i) (D 3 1) : ℚ)) = 1 / 3
      ∧ μB.exp (fun i => (makeInd (wAt i) (D 4 1) : ℚ)) = 1 / 5 := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · rw [show μA.exp (fun i => (makeInd (wAt i) (D 3 1) : ℚ))
        = ((∑ i, weightA (wAt i) * makeInd (wAt i) (D 3 1) : ℤ) : ℚ)
          / ((∑ i, weightA (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 9 (by norm_num) _ _ _
        (fun i => lA_eq_weight (wAt i)) _,
      makeA31, sum_weightA]
    norm_num
  · rw [show μA.exp (fun i => (makeInd (wAt i) (D 4 1) : ℚ))
        = ((∑ i, weightA (wAt i) * makeInd (wAt i) (D 4 1) : ℤ) : ℚ)
          / ((∑ i, weightA (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 9 (by norm_num) _ _ _
        (fun i => lA_eq_weight (wAt i)) _,
      makeA41, sum_weightA]
    norm_num
  · rw [show μB.exp (fun i => (makeInd (wAt i) (D 3 1) : ℚ))
        = ((∑ i, weightB (wAt i) * makeInd (wAt i) (D 3 1) : ℤ) : ℚ)
          / ((∑ i, weightB (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 3 (by norm_num) _ _ _
        (fun i => lB_eq_weight (wAt i)) _,
      makeB31, sum_weightB]
    norm_num
  · rw [show μB.exp (fun i => (makeInd (wAt i) (D 4 1) : ℚ))
        = ((∑ i, weightB (wAt i) * makeInd (wAt i) (D 4 1) : ℤ) : ℚ)
          / ((∑ i, weightB (wAt i) : ℤ) : ℚ) from
      exp_weight_ratio _ 3 (by norm_num) _ _ _
        (fun i => lB_eq_weight (wAt i)) _,
      makeB41, sum_weightB]
    norm_num

/-- **PA-E10, the reversal**: the two same-support posteriors order the
root leads oppositely, under both the expected signed differential and
the contract-make probability (Math §10.4; the remaining two §10.4
lenses are positive affine transforms of these). -/
theorem posterior_action_reversal :
    (μA.exp (fun i => (Q (wAt i) (D 3 1) : ℚ))
        < μA.exp (fun i => (Q (wAt i) (D 4 1) : ℚ)))
      ∧ (μB.exp (fun i => (Q (wAt i) (D 4 1) : ℚ))
        < μB.exp (fun i => (Q (wAt i) (D 3 1) : ℚ)))
      ∧ (μA.exp (fun i => (makeInd (wAt i) (D 3 1) : ℚ))
          < μA.exp (fun i => (makeInd (wAt i) (D 4 1) : ℚ)))
      ∧ (μB.exp (fun i => (makeInd (wAt i) (D 4 1) : ℚ))
          < μB.exp (fun i => (makeInd (wAt i) (D 3 1) : ℚ))) := by
  obtain ⟨e1, e2, e3, e4⟩ := expected_differentials
  obtain ⟨m1, m2, m3, m4⟩ := make_probabilities
  rw [e1, e2, e3, e4, m1, m2, m3, m4]
  norm_num

/-! ## The two auction histories (Math §10.4) -/

/-- History `A`: `0:pass, 1:P(30), 2:pass, 3:P(31)`. -/
def histA : List Bid := [.pass, .point 30, .pass, .point 31]

/-- History `B`: `0:P(30), 1:pass, 2:pass, 3:P(31)`. -/
def histB : List Bid := [.point 30, .pass, .pass, .point 31]

/-- The standard configuration (mark cap 2 for an opening auction). -/
def cfg : AuctionConfig := ⟨2, by norm_num⟩

/-- Both auction histories are legal from shaker seat 3, they are
distinct, and they produce the identical result — seat 3 wins at
`P(31)` (hence the same contract after the same declaration): the
mechanical endpoint cannot separate them. -/
theorem auction_histories :
    AuctionState.LegalAuction cfg ⟨3, histA⟩ ∧ AuctionState.LegalAuction cfg ⟨3, histB⟩
      ∧ histA ≠ histB
      ∧ (⟨3, histA⟩ : AuctionState).result = some (3, .point 31)
      ∧ (⟨3, histB⟩ : AuctionState).result = some (3, .point 31) := by
  refine ⟨?_, ?_, by decide, by decide, by decide⟩
  · exact .step (.step (.step (.step (.init 3) (by decide)) (by decide))
      (by decide)) (by decide)
  · exact .step (.step (.step (.step (.init 3) (by decide)) (by decide))
      (by decide)) (by decide)

/-! ## The endpoint cells and the exact fiber (Math §10.4) -/

/-- The viewer's original hand: the five played tiles plus the two in
hand. -/
def viewerHand0 : Finset Domino := played 3 ∪ viewerHand

/-- The §10.4 viewer: seat 3. -/
def v : ViewerCtx := ⟨3, viewerHand0⟩

/-- The common public endpoint record after the five tricks. -/
def Pend : PubState := PubState.replay theContract playList

private theorem cells_pool : v.pool Pend = poolW := by decide +kernel
private theorem cells_allowed : ∀ s : Seat, s ≠ 3 →
    v.allowed Pend s = poolW ∧ ViewerCtx.capacity Pend s = 2 := by decide +kernel
private theorem cells_leader : Pend.leader = 3 := by decide +kernel
private theorem cells_trick : Pend.trick = [] := by decide +kernel
private theorem cells_score0 : Pend.score 0 = 2 := by decide +kernel
private theorem cells_score1 : Pend.score 1 = 18 := by decide +kernel
private theorem cells_hand : v.hand Pend = viewerHand := by decide +kernel
private theorem cells_voids0 : Pend.voids 0 = ∅ := by decide +kernel
private theorem cells_voids1 : Pend.voids 1 = {Suit.natural 1} := by decide +kernel
private theorem cells_voids2 :
    Pend.voids 2 = {Suit.natural 0, Suit.natural 1} := by decide +kernel
private theorem cells_voids3 : Pend.voids 3 = ∅ := by decide +kernel

set_option maxHeartbeats 0 in
/-- The endpoint cells, computed from the public record alone: the
six-tile pool, no binding void exclusion (`P_s = U`), capacity two per
hidden seat, the §10.4 derived voids, leader 3, empty trick, scores
`(2, 18)`, and the viewer's remaining hand. -/
theorem endpoint_cells :
    v.pool Pend = poolW
      ∧ (∀ s : Seat, s ≠ 3 → v.allowed Pend s = poolW
          ∧ ViewerCtx.capacity Pend s = 2)
      ∧ Pend.leader = 3 ∧ Pend.trick = []
      ∧ Pend.score 0 = 2 ∧ Pend.score 1 = 18
      ∧ v.hand Pend = viewerHand
      ∧ Pend.voids 0 = ∅
      ∧ Pend.voids 1 = {Suit.natural 1}
      ∧ Pend.voids 2 = {Suit.natural 0, Suit.natural 1}
      ∧ Pend.voids 3 = ∅ := by
  exact ⟨cells_pool, cells_allowed, cells_leader, cells_trick,
    cells_score0, cells_score1, cells_hand, cells_voids0, cells_voids1,
    cells_voids2, cells_voids3⟩

theorem seat_cases (s : Seat) : s = 0 ∨ s = 1 ∨ s = 2 ∨ s = 3 := by
  fin_cases s
  exacts [Or.inl rfl, Or.inr (Or.inl rfl), Or.inr (Or.inr (Or.inl rfl)),
    Or.inr (Or.inr (Or.inr rfl))]

/-- Seat-indexed unions expand to the four hands. -/
theorem biUnion_seats (B : Seat → Finset Domino) :
    Finset.univ.biUnion B = B 0 ∪ B 1 ∪ B 2 ∪ B 3 := by
  ext d
  rw [Finset.mem_biUnion, Finset.mem_union, Finset.mem_union,
    Finset.mem_union]
  constructor
  · rintro ⟨s, -, hd⟩
    rcases seat_cases s with rfl | rfl | rfl | rfl
    · exact Or.inl (Or.inl (Or.inl hd))
    · exact Or.inl (Or.inl (Or.inr hd))
    · exact Or.inl (Or.inr hd)
    · exact Or.inr hd
  · rintro (((hd | hd) | hd) | hd)
    · exact ⟨0, Finset.mem_univ _, hd⟩
    · exact ⟨1, Finset.mem_univ _, hd⟩
    · exact ⟨2, Finset.mem_univ _, hd⟩
    · exact ⟨3, Finset.mem_univ _, hd⟩

/-- **The exact fiber**: the endpoint cell fiber is precisely the 90
enumerated worlds (Math §10.4: `|Φ(c)| = 90`). With `wAt_image` and
`card_worldPairs`, the fiber is the image of the ninety indexed worlds. -/
theorem isWorld_iff (A : Seat → Finset Domino) :
    v.IsWorld Pend A
      ↔ (A 0, A 1) ∈ worldPairs
        ∧ A 2 = poolW \ (A 0 ∪ A 1) ∧ A 3 = ∅ := by
  obtain ⟨hpool, hcells, -⟩ := endpoint_cells
  have hall : ∀ s : Seat, s ≠ 3 → v.allowed Pend s = poolW :=
    fun s hs => (hcells s hs).1
  have hcap : ∀ s : Seat, s ≠ 3 → ViewerCtx.capacity Pend s = 2 :=
    fun s hs => (hcells s hs).2
  have hcard6 : poolW.card = 6 := by decide +kernel
  constructor
  · rintro ⟨h3, hcell, hdisj, hunion⟩
    have hA0 := hcell 0 (by decide)
    have hA1 := hcell 1 (by decide)
    have hA2 := hcell 2 (by decide)
    rw [hall 0 (by decide), hcap 0 (by decide)] at hA0
    rw [hall 1 (by decide), hcap 1 (by decide)] at hA1
    rw [hall 2 (by decide), hcap 2 (by decide)] at hA2
    have hd01 : Disjoint (A 0) (A 1) := hdisj 0 1 (by decide)
    have hA2eq : A 2 = poolW \ (A 0 ∪ A 1) := by
      apply Finset.eq_of_subset_of_card_le
      · rw [Finset.subset_sdiff]
        refine ⟨hA2.1, ?_⟩
        rw [Finset.disjoint_union_right]
        exact ⟨(hdisj 0 2 (by decide)).symm, (hdisj 1 2 (by decide)).symm⟩
      · rw [Finset.card_sdiff_of_subset
            (Finset.union_subset hA0.1 hA1.1),
          Finset.card_union_of_disjoint hd01, hcard6, hA0.2, hA1.2,
          hA2.2]
    refine ⟨?_, hA2eq, h3⟩
    exact Finset.mem_biUnion.mpr ⟨A 0,
      Finset.mem_powersetCard.mpr ⟨hA0.1, hA0.2⟩,
      Finset.mem_image.mpr ⟨A 1,
        Finset.mem_powersetCard.mpr
          ⟨Finset.subset_sdiff.mpr ⟨hA1.1, hd01.symm⟩, hA1.2⟩, rfl⟩⟩
  · rintro ⟨hmem, hA2, hA3⟩
    obtain ⟨h0, hh0, himg⟩ := Finset.mem_biUnion.mp hmem
    obtain ⟨h1, hh1, hpair⟩ := Finset.mem_image.mp himg
    rw [Prod.mk.injEq] at hpair
    obtain ⟨he0, he1⟩ := hpair
    subst he0
    subst he1
    rw [Finset.mem_powersetCard] at hh0 hh1
    obtain ⟨hsub0, hcard0⟩ := hh0
    obtain ⟨hsub1', hcard1⟩ := hh1
    rw [Finset.subset_sdiff] at hsub1'
    obtain ⟨hsub1, hdisj10⟩ := hsub1'
    have hsub2 : A 2 ⊆ poolW := by
      rw [hA2]
      exact Finset.sdiff_subset
    have hcard2 : (A 2).card = 2 := by
      rw [hA2, Finset.card_sdiff_of_subset
          (Finset.union_subset hsub0 hsub1),
        Finset.card_union_of_disjoint hdisj10.symm, hcard6, hcard0,
        hcard1]
    have hdisj2 : ∀ s : Seat, s = 0 ∨ s = 1 →
        Disjoint (A 2) (A s) := by
      rintro s (rfl | rfl) <;>
        · rw [hA2]
          refine Finset.disjoint_of_subset_right ?_
            Finset.sdiff_disjoint
          first
            | exact Finset.subset_union_left
            | exact Finset.subset_union_right
    refine ⟨hA3, ?_, ?_, ?_⟩
    · intro s hs
      rw [hall s hs, hcap s hs]
      rcases seat_cases s with rfl | rfl | rfl | rfl
      · exact ⟨hsub0, hcard0⟩
      · exact ⟨hsub1, hcard1⟩
      · exact ⟨hsub2, hcard2⟩
      · exact absurd rfl hs
    · intro s t hst
      rcases seat_cases s with rfl | rfl | rfl | rfl <;>
        rcases seat_cases t with rfl | rfl | rfl | rfl <;>
        first
          | exact absurd rfl hst
          | exact hdisj10.symm
          | exact hdisj10
          | exact (hdisj2 _ (Or.inl rfl)).symm
          | exact (hdisj2 _ (Or.inr rfl)).symm
          | exact hdisj2 _ (Or.inl rfl)
          | exact hdisj2 _ (Or.inr rfl)
          | (rw [hA3]; exact Finset.disjoint_empty_right _)
          | (rw [hA3]; exact Finset.disjoint_empty_left _)
    · rw [biUnion_seats, hA3, Finset.union_empty, hA2,
        Finset.union_sdiff_of_subset (Finset.union_subset hsub0 hsub1),
        hpool]

/-! ## The rule fiber: every world legally replays the prefix -/

/-- One-pass state fingerprint (forces a single kernel replay). -/
def encodeState : PlayState →
    Finset Domino × Finset Domino × Finset Domino × Finset Domino
      × Seat × List (Seat × Domino) × ℕ × ℕ
  | ⟨h, l, t, sc, _⟩ => (h 0, h 1, h 2, h 3, l, t, sc 0, sc 1)

set_option synthInstance.maxSize 2000 in
set_option maxHeartbeats 0 in
/-- Every one of the 90 worlds' complete deals legally replays the
public five-trick prefix to exactly the common endpoint (Math §10.4:
"the verifier replays the public prefix in all 90 complete deals"). -/
theorem replay_check : ∀ i : Fin 90,
    (start (wAt i)).LegalFrom playList
      ∧ encodeState ((start (wAt i)).replayFrom playList)
        = (hiddenHands (wAt i) 0, hiddenHands (wAt i) 1,
            hiddenHands (wAt i) 2, viewerHand, 3, [], 2, 18) := by
  decide +kernel
set_option maxHeartbeats 0 in
/-- The 90 worlds' deals are genuine deals: four seven-tile hands,
pairwise disjoint. -/
theorem deal_props : ∀ i : Fin 90,
    (∀ s : Seat, (dealHands (wAt i) s).card = 7)
      ∧ (∀ s t : Seat, s ≠ t →
          Disjoint (dealHands (wAt i) s) (dealHands (wAt i) t)) := by
  decide +kernel
/-- Every enumerated world is realized by a rule-compatible complete
deal (the `Ω(I)` side of losslessness). -/
theorem rule_fiber (i : Fin 90) :
    ∃ ω : Deal, ω.hands = dealHands (wAt i)
      ∧ v.Compatible theContract playList ω := by
  refine ⟨⟨dealHands (wAt i), (deal_props i).1, (deal_props i).2⟩, rfl,
    ?_, ?_⟩
  · show dealHands (wAt i) 3 = viewerHand0
    rfl
  · exact (replay_check i).1

/-! ## PA-E10, the named theorem -/

/-- **PA-E10 (Math §10.4; K15's named witness)**: inside no-trump
Straight 42, two distinct legal auction histories produce the same
winning seat and bid — hence, with the shared five-trick public prefix,
the same mechanical endpoint, whose cell fiber is exactly the 90
enumerated worlds, every one realized by a rule-compatible deal. Both
auction-induced posteriors put strictly positive mass on all 90 worlds
— identical support — yet the posterior weights order the two root
leads oppositely under the expected-differential and contract-make
lenses. Mechanical state alone is not an exact strategic state. -/
theorem ninety_world_witness :
    -- distinct legal histories, same auction outcome
    (histA ≠ histB
      ∧ AuctionState.LegalAuction cfg ⟨3, histA⟩ ∧ AuctionState.LegalAuction cfg ⟨3, histB⟩
      ∧ (⟨3, histA⟩ : AuctionState).result
        = (⟨3, histB⟩ : AuctionState).result)
      -- the endpoint cell fiber is exactly the 90 worlds
      ∧ (∀ A : Seat → Finset Domino,
          v.IsWorld Pend A
            ↔ (A 0, A 1) ∈ worldPairs
              ∧ A 2 = poolW \ (A 0 ∪ A 1) ∧ A 3 = ∅)
      ∧ worldPairs.card = 90
      -- every world is rule-realized
      ∧ (∀ i : Fin 90, ∃ ω : Deal, ω.hands = dealHands (wAt i)
          ∧ v.Compatible theContract playList ω)
      -- same (full) posterior support
      ∧ (∀ i : Fin 90, 0 < μA.mass i ∧ 0 < μB.mass i)
      -- the reversal
      ∧ (μA.exp (fun i => (Q (wAt i) (D 3 1) : ℚ))
          < μA.exp (fun i => (Q (wAt i) (D 4 1) : ℚ)))
      ∧ (μB.exp (fun i => (Q (wAt i) (D 4 1) : ℚ))
          < μB.exp (fun i => (Q (wAt i) (D 3 1) : ℚ))) := by
  obtain ⟨hA, hB, hne, hrA, hrB⟩ := auction_histories
  exact ⟨⟨hne, hA, hB, hrA.trans hrB.symm⟩, isWorld_iff,
    card_worldPairs, rule_fiber, same_full_support,
    posterior_action_reversal.1, posterior_action_reversal.2.1⟩

end Witness

end Texas42
