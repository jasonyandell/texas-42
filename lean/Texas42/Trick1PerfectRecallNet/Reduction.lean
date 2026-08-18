/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.MassObjectives

/-!
# Immutable reductions, continuation injections, and command bounds
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/-- Singleton ranges retire on the host and dispatch no reduction pass. -/
def reductionDestinationCount (count : ℕ) : ℕ :=
  if count ≤ 1 then 0 else (count + 1) / 2

/-- Count-one retirement is exact. -/
theorem count_one_retires : reductionDestinationCount 1 = 0 := by
  decide

/-- The maximum simultaneously active destination census. -/
def activeDestinationBound (count : ℕ) : ℕ := (2 * count) / 3

/-- Next-level rows from one range, with newly sealed singletons retired. -/
def activeNextCount (count : ℕ) : ℕ :=
  if count ≤ 2 then 0 else (count + 1) / 2

/-- The frozen expression is exactly `floor(2n/3)` in natural arithmetic. -/
theorem activeDestinationBound_eq (count : ℕ) :
    activeDestinationBound count = (2 * count) / 3 := rfl

/-- Every range individually spends at most two destination rows per three sources. -/
theorem three_mul_activeNextCount_le (count : ℕ) :
    3 * activeNextCount count ≤ 2 * count := by
  unfold activeNextCount
  split <;> omega

/-- Singleton retirement gives the frozen aggregate `floor(2B/3)` bound. -/
theorem activeNextCounts_le_bound (counts : List ℕ) :
    (counts.map activeNextCount).sum ≤ activeDestinationBound counts.sum := by
  have hscaled :
      3 * (counts.map activeNextCount).sum ≤ 2 * counts.sum := by
    induction counts with
    | nil => simp
    | cons count counts ih =>
        simp only [List.map_cons, List.sum_cons]
        have hcount := three_mul_activeNextCount_le count
        omega
  unfold activeDestinationBound
  omega

/-- Immutable dense compaction maps every new ordinal to one old ordinal. -/
structure CompactionMap (oldCount newCount : ℕ) where
  oldOrdinal : Fin newCount → Fin oldCount
  injective : Function.Injective oldOrdinal

/-- No two new dense ordinals rewrite the same old ordinal. -/
theorem compaction_oldOrdinal_unique {oldCount newCount : ℕ}
    (map : CompactionMap oldCount newCount) {a b : Fin newCount}
    (h : map.oldOrdinal a = map.oldOrdinal b) : a = b :=
  map.injective h

/-- Source positions used by pair `i`. -/
def reductionPair (i : ℕ) : ℕ × ℕ := (2 * i, 2 * i + 1)

/-- Consecutive reduction pairs have disjoint source ranges. -/
theorem reduction_pair_range_separation {i j : ℕ} (hne : i ≠ j) :
    (reductionPair i).1 ≠ (reductionPair j).1 ∧
    (reductionPair i).1 ≠ (reductionPair j).2 ∧
    (reductionPair i).2 ≠ (reductionPair j).1 ∧
    (reductionPair i).2 ≠ (reductionPair j).2 := by
  simp only [reductionPair]
  omega

/-- Total source volume over all nontrivial levels of one reduction range. -/
def reductionVolume (count : ℕ) : ℕ :=
  if _h : count < 2 then 0
  else count + reductionVolume ((count + 1) / 2)
termination_by count
decreasing_by omega

/-- The frozen reduction-volume recurrence. -/
theorem reductionVolume_eq {count : ℕ} (h : 2 ≤ count) :
    reductionVolume count = count + reductionVolume ((count + 1) / 2) := by
  rw [reductionVolume]
  simp [show ¬count < 2 by omega]

/-- Every immutable pair/carry reduction uses less than three source volumes. -/
theorem reductionVolume_lt_three_mul (count : ℕ) (hpos : 0 < count) :
    reductionVolume count < 3 * count := by
  induction count using Nat.strong_induction_on with
  | h count ih =>
      by_cases hsmall : count < 2
      · simp [reductionVolume, hsmall]
        omega
      · have htwo : 2 ≤ count := by omega
        have hhalf : (count + 1) / 2 < count := by omega
        have hhalfPos : 0 < (count + 1) / 2 := by omega
        rw [reductionVolume_eq htwo]
        have hrec := ih ((count + 1) / 2) hhalf hhalfPos
        omega

/-- Exact within-hand permutation code for the three hidden seats. -/
abbrev HiddenOrderCode :=
  Equiv.Perm (Fin 3) × Equiv.Perm (Fin 3) × Equiv.Perm (Fin 3)

/-- Two remaining S1 orders paired with all hidden-seat order codes. -/
abbrev AllTwoOrderCode := Fin 2 × HiddenOrderCode

/--
At the post-first-frontier cut, a fixed-S1-order continuation is completely
determined by the three hidden seats' within-hand permutations.  Legality may
remove codes, but cannot create two traces with one code.
-/
structure FixedOrderContinuation where
  hiddenOrders : HiddenOrderCode
  deriving Fintype

/-- With two S1 tiles left, its play-order bit joins the three hidden orders. -/
structure AllTwoOrderContinuation where
  s1Order : Fin 2
  hiddenOrders : HiddenOrderCode
  deriving Fintype

/-- Explicit injection of fixed-order continuations into `(S₃)^3`. -/
def encodeFixedOrderContinuation
    (continuation : FixedOrderContinuation) : HiddenOrderCode :=
  continuation.hiddenOrders

/-- The fixed-order encoding is injective, not merely cardinal arithmetic. -/
theorem encodeFixedOrderContinuation_injective :
    Function.Injective encodeFixedOrderContinuation := by
  intro a b h
  cases a
  cases b
  simp_all [encodeFixedOrderContinuation]

/-- Explicit injection of all-two-order continuations into `{0,1} × (S₃)^3`. -/
def encodeAllTwoOrderContinuation
    (continuation : AllTwoOrderContinuation) : AllTwoOrderCode :=
  (continuation.s1Order, continuation.hiddenOrders)

/-- The all-two-order encoding is injective. -/
theorem encodeAllTwoOrderContinuation_injective :
    Function.Injective encodeAllTwoOrderContinuation := by
  intro a b h
  cases a
  cases b
  simp_all [encodeAllTwoOrderContinuation]

/-- The explicit `(S₃)^3` target has 216 elements. -/
theorem hiddenOrderCode_card : Fintype.card HiddenOrderCode = 216 := by
  norm_num [HiddenOrderCode, Fintype.card_perm, Nat.factorial]

/-- The explicit `{0,1} × (S₃)^3` target has 432 elements. -/
theorem allTwoOrderCode_card : Fintype.card AllTwoOrderCode = 432 := by
  norm_num [AllTwoOrderCode, HiddenOrderCode, Fintype.card_perm, Nat.factorial]

/-- Any fixed-S1-order continuation injection yields the 216 cap. -/
theorem continuation_card_le_216 {Continuation : Type*} [Fintype Continuation]
    (encode : Continuation → HiddenOrderCode) (hinj : Function.Injective encode) :
    Fintype.card Continuation ≤ 216 := by
  rw [← hiddenOrderCode_card]
  exact Fintype.card_le_of_injective encode hinj

/-- Both S1 orders injected with hidden orders yield the 432 cap. -/
theorem all_two_order_continuation_card_le_432
    {Continuation : Type*} [Fintype Continuation]
    (encode : Continuation → AllTwoOrderCode) (hinj : Function.Injective encode) :
    Fintype.card Continuation ≤ 432 := by
  rw [← allTwoOrderCode_card]
  exact Fintype.card_le_of_injective encode hinj

/-- The explicit fixed-order continuation carrier has at most 216 members. -/
theorem fixedOrderContinuation_card_le_216 :
    Fintype.card FixedOrderContinuation ≤ 216 := by
  exact continuation_card_le_216 encodeFixedOrderContinuation
    encodeFixedOrderContinuation_injective

/-- The explicit all-two-order continuation carrier has at most 432 members. -/
theorem allTwoOrderContinuation_card_le_432 :
    Fintype.card AllTwoOrderContinuation ≤ 432 := by
  exact all_two_order_continuation_card_le_432 encodeAllTwoOrderContinuation
    encodeAllTwoOrderContinuation_injective

/-- Exact continuation and first-frontier arithmetic. -/
theorem continuation_numeric_identities :
    6 ^ 3 = 216 ∧ 2 * 216 = 432 ∧
      1200 * 432 = 518400 ∧
      1200 * 4 ^ 3 * 3 ^ 3 = 2073600 := by
  norm_num

/-- Both frozen semantic ranges fit the strict 21-level window. -/
theorem semantic_ranges_lt_two_pow_21 :
    2073600 < 2 ^ 21 ∧ 1327104 < 2 ^ 21 := by
  norm_num

/-- Ceiling division used by the closed reduction scheduler. -/
def ceilDiv (numerator denominator : ℕ) : ℕ :=
  (numerator + denominator - 1) / denominator

/-- Exact sequence counts for H and the 75 C microblocks. -/
theorem reduction_sequence_counts :
    2 * 4 * 21 = 168 ∧ 2 * 75 * 4 * 21 = 12600 := by
  norm_num

/-- Exact aggregate command base from `6N/(M-1)`. -/
theorem aggregate_reduction_base_eq_769 :
    ceilDiv (6 * 2 ^ 26) (2 ^ 19 - 1) = 769 := by
  norm_num [ceilDiv]

/-- Exact H/C reduction-command bounds. -/
theorem aggregate_reduction_command_bounds :
    769 + (168 - 1) = 936 ∧
      769 + (12600 - 1) = 13368 := by
  norm_num

/-- Exact H/C treatment command compositions. -/
theorem treatment_command_bounds :
    139 + 936 + 1 + 64 = 1140 ∧ 1140 < 2048 ∧
      1028 + 13368 + 1 + 64 = 14461 ∧ 14461 < 16384 := by
  norm_num

/-- Exact task, run-command, and frame compositions. -/
theorem run_command_frame_bounds :
    2048 + 16384 + 2048 = 20480 ∧ 20480 < 32768 ∧
      8 * 32768 + 1024 = 263168 ∧ 263168 < 524288 ∧
      2 * 32768 + 64 = 65600 ∧ 65600 < 131072 ∧
      2 * 524288 + 1024 = 1049600 ∧ 1049600 < 2097152 := by
  norm_num

end Trick1PerfectRecallNet
end Texas42
