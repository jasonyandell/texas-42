/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Play

/-!
# Trick-one arithmetic and finite-decision foundations

This module isolates the small, kernel-checkable facts used by the native
trick-one design.  It deliberately does not formalize the GPU implementation
or claim that its search is complete.  In particular, componentwise lower
bounds are combined only when they come from one shared lawful policy.
-/

namespace Texas42
namespace Trick1Foundation

/-! ## Contract arithmetic -/

/-- The most points that a declarer bidding `n` can lose and still make
(GPU-native v0.3 §§2.1, 3; GT1-A2). -/
def lossAllowance (n : ℕ) : ℕ := 42 - n

/-- The derived loss budget of a nonpass root bid.  A pass has no contracted
loss budget, every mark contract has budget zero, and a point contract derives
its budget from the bid rather than accepting a caller-supplied integer
(GPU-native v0.3 §§2.1, 3; GT1-A2). -/
def bidLossAllowance : Bid → Option ℕ
  | .pass => none
  | .point n => some (lossAllowance n)
  | .mark _ => some 0

/-- Every legal point bid permits the declarer to lose at most twelve points
(GPU-native v0.3 §§2.1, 3; GT1-A2). -/
theorem legal_point_lossAllowance_le_twelve
    {cfg : AuctionConfig} {auction : AuctionState} {n : ℕ}
    (hlegal : auction.legalBid cfg (.point n)) :
    lossAllowance n ≤ 12 := by
  simp only [AuctionState.legalBid] at hlegal
  unfold lossAllowance
  omega

/-- Every legal nonpass bid's derived loss budget is at most twelve, including
the zero budget of a mark contract (GPU-native v0.3 §§2.1, 3; GT1-A2). -/
theorem legal_bid_lossAllowance_le_twelve
    {cfg : AuctionConfig} {auction : AuctionState} {bid : Bid} {budget : ℕ}
    (hlegal : auction.legalBid cfg bid)
    (hbudget : bidLossAllowance bid = some budget) :
    budget ≤ 12 := by
  cases bid with
  | pass => simp [bidLossAllowance] at hbudget
  | point n =>
      simp only [bidLossAllowance, Option.some.injEq] at hbudget
      subst budget
      exact legal_point_lossAllowance_le_twelve hlegal
  | mark m =>
      simp only [bidLossAllowance, Option.some.injEq] at hbudget
      omega

/-! ## Exact integer scaling -/

/-- Every nonempty Texas 42 hand cardinality divides the common scale 420
(GPU-native v0.3 §4.1; GT1-A3). -/
theorem card_one_to_seven_dvd_420 {k : ℕ} (hpos : 1 ≤ k) (hle : k ≤ 7) :
    k ∣ 420 := by
  interval_cases k <;> norm_num

/-- Scaling a uniform choice of cardinality `k` by `420 / k` is exact
(GPU-native v0.3 §4.1; GT1-A3). -/
theorem scaled_cardinality_exact {k : ℕ} (hpos : 1 ≤ k) (hle : k ≤ 7) :
    (420 / k) * k = 420 := by
  exact Nat.div_mul_cancel (card_one_to_seven_dvd_420 hpos hle)

/-- Every hand has at most seven tiles.  This separate predicate is initialized
by a deal and preserved by every atomic play (GPU-native v0.3 §4.1; GT1-A3). -/
def HandCapSeven (X : PlayState) : Prop :=
  ∀ s, (X.hands s).card ≤ 7

/-- A dealt initial state has the seven-tile hand cap
(GPU-native v0.3 §§2.1, 4.1; GT1-A2/GT1-A3). -/
theorem handCapSeven_init (world : Deal) (contract : Contract) :
    HandCapSeven (PlayState.init world contract) := by
  intro s
  simpa [HandCapSeven, PlayState.init] using
    (Nat.le_of_eq (world.card_hands s))

/-- Atomic play cannot increase any hand, so it preserves the seven-tile cap
(GPU-native v0.3 §4.1; GT1-A3). -/
theorem handCapSeven_step {X : PlayState} (hcap : HandCapSeven X) (d : Domino) :
    HandCapSeven (X.step d) := by
  intro s
  apply (Finset.card_le_card ?_).trans (hcap s)
  intro x hx
  rw [PlayState.step_hands X d] at hx
  exact ((PlayState.mem_handsAfter X d x s).mp hx).1

/-- At a live state whose hand cap is maintained, the actual rule-derived legal
set has cardinality in `1..7` (GPU-native v0.3 §4.1; GT1-A3). -/
theorem legalSet_card_one_to_seven (X : PlayState) (hcap : HandCapSeven X)
    (hheld : (X.hands X.actor).Nonempty) :
    1 ≤ X.legalSet.card ∧ X.legalSet.card ≤ 7 := by
  constructor
  · exact Finset.card_pos.mpr (X.legalSet_nonempty hheld)
  · exact (Finset.card_le_card X.legalSet_subset).trans (hcap X.actor)

/-- Number of opening deals after one fixed seven-tile hand is removed
(GPU-native v0.3 §4.1; GT1-A3). -/
def openingDealCount : ℕ := 399072960

/-- Common per-action integer probability scale
(GPU-native v0.3 §4.1; GT1-A3). -/
def fieldScale : ℕ := 420

/-- The actual rule-derived legal-set cardinality therefore divides the frozen
field scale (GPU-native v0.3 §4.1; GT1-A3). -/
theorem legalSet_card_dvd_fieldScale (X : PlayState) (hcap : HandCapSeven X)
    (hheld : (X.hands X.actor).Nonempty) :
    X.legalSet.card ∣ fieldScale := by
  obtain ⟨hpos, hle⟩ := legalSet_card_one_to_seven X hcap hheld
  simpa [fieldScale] using card_one_to_seven_dvd_420 hpos hle

/-- Maximum number of field actions after the opening hand is fixed
(GPU-native v0.3 §§2.1, 4.1; GT1-A2/GT1-A3). -/
def fieldActionCount : ℕ := 21

/-- A conservative common denominator for one complete weighted traversal
(GPU-native v0.3 §4.1; GT1-A3). -/
def rootDenominator : ℕ := openingDealCount * fieldScale ^ fieldActionCount

/-- The opening-deal constant is the expected multinomial count
(GPU-native v0.3 §4.1; GT1-A3). -/
theorem openingDealCount_eq_multinomial :
    openingDealCount =
      Nat.factorial 21 / (Nat.factorial 7 * Nat.factorial 7 * Nat.factorial 7) := by
  norm_num [openingDealCount, Nat.factorial]

/-- `rootDenominator` needs exactly 212 unsigned magnitude bits
(GPU-native v0.3 §4.1; GT1-A3). -/
theorem rootDenominator_bit_window :
    2 ^ 211 ≤ rootDenominator ∧ rootDenominator < 2 ^ 212 := by
  norm_num [rootDenominator, openingDealCount, fieldScale, fieldActionCount]

/-- A utility numerator of magnitude at most `42D` needs 217 magnitude bits
(GPU-native v0.3 §4.1; GT1-A3). -/
theorem utilityMagnitude_bit_window :
    2 ^ 216 ≤ 42 * rootDenominator ∧ 42 * rootDenominator < 2 ^ 217 := by
  norm_num [rootDenominator, openingDealCount, fieldScale, fieldActionCount]

/-! ## Point-mass conservation -/

/--
Arithmetic regrouping of explicitly supplied point reservoirs.
`currentPrefixCount` is separate because count tiles already played into an
unresolved current trick are neither banked nor unplayed.  The state-tied
invariant below, not this helper alone, establishes that the reservoirs are
exhaustive (GPU-native v0.3 §3; GT1-A8).
-/
def unbankedPoints
    (unfinishedTrickAwards currentPrefixCount unplayedCount : ℕ) : ℕ :=
  unfinishedTrickAwards + currentPrefixCount + unplayedCount

/-- Regrouping five explicitly supplied point reservoirs preserves an asserted
total; this is algebra, not a `PlayState` invariant (GPU-native v0.3 §3;
GT1-A8). -/
theorem unbankedPoints_conservation
    {declarerBanked defenderBanked unfinishedTrickAwards currentPrefixCount
      unplayedCount : ℕ} :
    declarerBanked + defenderBanked +
          unbankedPoints unfinishedTrickAwards currentPrefixCount unplayedCount = 42 ↔
      declarerBanked + defenderBanked + unfinishedTrickAwards +
          currentPrefixCount + unplayedCount = 42 := by
  simp only [unbankedPoints]
  omega

/-- Under an explicit conservation premise, supplied unbanked points are total
minus banked points (GPU-native v0.3 §3; GT1-A8). -/
theorem unbankedPoints_eq_total_sub_banked
    {declarerBanked defenderBanked unfinishedTrickAwards currentPrefixCount
      unplayedCount : ℕ}
    (hconserve :
      declarerBanked + defenderBanked + unfinishedTrickAwards +
          currentPrefixCount + unplayedCount = 42) :
    unbankedPoints unfinishedTrickAwards currentPrefixCount unplayedCount =
      42 - (declarerBanked + defenderBanked) := by
  unfold unbankedPoints
  omega

/-- Any defender continuation lies inside the interval induced by supplied
unbanked mass (GPU-native v0.3 §3; GT1-A8). -/
theorem defender_final_points_interval
    {defenderBanked defenderFuture unfinishedTrickAwards currentPrefixCount
      unplayedCount : ℕ}
    (hfuture : defenderFuture ≤
      unbankedPoints unfinishedTrickAwards currentPrefixCount unplayedCount) :
    defenderBanked ≤ defenderBanked + defenderFuture ∧
      defenderBanked + defenderFuture ≤ defenderBanked +
        unbankedPoints unfinishedTrickAwards currentPrefixCount unplayedCount := by
  omega

/-- Tiles whose tricks have not yet been banked in the actual play state.
Because `PlayState.scoredTiles` excludes the unresolved prefix, this set contains
both every held tile and every tile already played into the current trick
(GPU-native v0.3 §3; GT1-A8). -/
def stateUnbankedTiles (X : PlayState) : Finset Domino :=
  Finset.univ \ X.scoredTiles

/-- Count value carried by all not-yet-banked tiles in an actual play state
(GPU-native v0.3 §3; GT1-A8). -/
def stateUnbankedCountPoints (X : PlayState) : ℕ :=
  ∑ d ∈ stateUnbankedTiles X, d.countPoints

/-- Actual unbanked point mass: one base point for each unfinished trick plus
the count value on every tile whose trick is not banked
(GPU-native v0.3 §3; GT1-A8). -/
def stateUnbankedPoints (X : PlayState) : ℕ :=
  (7 - X.tricksDone) + stateUnbankedCountPoints X

/-- Every tile in the unresolved current-trick prefix is in the state's
unbanked tile set.  This is the formal guard against the v0.2 mid-trick omission
(GPU-native v0.3 §3; GT1-A8). -/
theorem trickTiles_subset_stateUnbankedTiles (X : PlayState) :
    X.trickTiles.toFinset ⊆ stateUnbankedTiles X := by
  intro d hd
  simp only [stateUnbankedTiles, Finset.mem_sdiff, Finset.mem_univ, true_and]
  intro hscored
  exact ((PlayState.mem_scoredTiles X d).mp hscored).2
    (List.mem_toFinset.mp hd)

/-- The model's preserved play invariant implies exact current-prefix-aware
point conservation at every state (GPU-native v0.3 §3; GT1-A8). -/
theorem stateUnbankedPoints_conservation {X : PlayState} (hX : X.Inv) :
    X.score 0 + X.score 1 + stateUnbankedPoints X = 42 := by
  obtain ⟨k, hprogress, hscore⟩ := hX.progress
  have htricks : X.tricksDone = k := by
    unfold PlayState.tricksDone
    have hremaining : 28 - X.gamma - X.trick.length = 4 * k := by omega
    rw [hremaining]
    omega
  have hk : k ≤ 7 := by omega
  have hscored : ∑ d ∈ X.scoredTiles, d.countPoints ≤ 35 := by
    calc
      ∑ d ∈ X.scoredTiles, d.countPoints ≤ ∑ d : Domino, d.countPoints :=
        Finset.sum_le_sum_of_subset (Finset.subset_univ X.scoredTiles)
      _ = 35 := Domino.total_countPoints
  have hsplit :
      (∑ d ∈ Finset.univ \ X.scoredTiles, d.countPoints) +
          ∑ d ∈ X.scoredTiles, d.countPoints = 35 := by
    calc
      (∑ d ∈ Finset.univ \ X.scoredTiles, d.countPoints) +
            ∑ d ∈ X.scoredTiles, d.countPoints =
          ∑ d : Domino, d.countPoints :=
        Finset.sum_sdiff (Finset.subset_univ X.scoredTiles)
      _ = 35 := Domino.total_countPoints
  have hunbanked :
      stateUnbankedCountPoints X =
        35 - ∑ d ∈ X.scoredTiles, d.countPoints := by
    unfold stateUnbankedCountPoints stateUnbankedTiles
    omega
  unfold stateUnbankedPoints
  rw [htricks, hunbanked, hscore]
  omega

/-- One legal atomic transition preserves the same state-tied point accounting
(GPU-native v0.3 §3; GT1-A8). -/
theorem stateUnbankedPoints_conservation_step
    {X : PlayState} (hX : X.Inv) {d : Domino} (hd : d ∈ X.legalSet) :
    (X.step d).score 0 + (X.step d).score 1 +
        stateUnbankedPoints (X.step d) = 42 :=
  stateUnbankedPoints_conservation (PlayState.inv_step hX hd)

/-! ## Opening partition cardinalities -/

/-- Number of ordered selections of `k` distinct objects from `n`
(GPU-native v0.3 §5.1; GT1-A4). -/
def orderedSelections (n k : ℕ) : ℕ :=
  if k ≤ n then Nat.factorial n / Nat.factorial (n - k) else 0

/-- Number of ordered response triples with exactly `f` followers in a mask of
size `m` (GPU-native v0.3 §5.1; GT1-A4). -/
def responseTripleCount (m f : ℕ) : ℕ :=
  Nat.choose 3 f * orderedSelections m f * orderedSelections (21 - m) (3 - f)

/-- Number of first-failure strata associated with one such response triple
(GPU-native v0.3 §5.1; GT1-A4). -/
def firstFailureStratumCount (m f : ℕ) : ℕ :=
  if m = 0 ∧ f = 0 then 1
  else if 1 ≤ f ∧ f ≤ m then Nat.choose (m - 1) (f - 1)
  else 0

/-- Total nonempty opening cells for a lead-follow mask of size `m`
(GPU-native v0.3 §5.1; GT1-A4). -/
def openingCellCount (m : ℕ) : ℕ :=
  ∑ f ∈ Finset.range 4,
    responseTripleCount m f * firstFailureStratumCount m f

/-- Exact opening cell counts for mask cardinalities zero through six
(GPU-native v0.3 §5.1; GT1-A4). -/
theorem openingCellCount_values :
    openingCellCount 0 = 7980 ∧
    openingCellCount 1 = 1140 ∧
    openingCellCount 2 = 2166 ∧
    openingCellCount 3 = 3408 ∧
    openingCellCount 4 = 5172 ∧
    openingCellCount 5 = 7800 ∧
    openingCellCount 6 = 11730 := by
  decide

/-- The exact opening-cell count never exceeds 11730 on the declared range
(GPU-native v0.3 §5.1; GT1-A4). -/
theorem openingCellCount_le_11730 {m : ℕ} (hm : m ≤ 6) :
    openingCellCount m ≤ 11730 := by
  interval_cases m <;> decide

/-! ## Additive component bounds -/

/-- Weighted value of a finite family of additive components
(GPU-native v0.3 §7; GT1-A7). -/
def weightedValue {ι : Type*} (components : Finset ι)
    (weight value : ι → ℚ) : ℚ :=
  ∑ i ∈ components, weight i * value i

/-- Nonnegative weights preserve componentwise upper bounds under summation
(GPU-native v0.3 §7; GT1-A7). -/
theorem componentwise_upper_sum
    {ι : Type*} (components : Finset ι)
    (weight actual upper : ι → ℚ)
    (hweight : ∀ i ∈ components, 0 ≤ weight i)
    (hupper : ∀ i ∈ components, actual i ≤ upper i) :
    weightedValue components weight actual ≤
      weightedValue components weight upper := by
  classical
  unfold weightedValue
  apply Finset.sum_le_sum
  intro i hi
  exact mul_le_mul_of_nonneg_left (hupper i hi) (hweight i hi)

/-- Weighted value obtained by using one policy in every component
(GPU-native v0.3 §7; GT1-A7). -/
def policyValue {ι π : Type*} (components : Finset ι)
    (weight : ι → ℚ) (componentValue : ι → π → ℚ) (policy : π) : ℚ :=
  ∑ i ∈ components, weight i * componentValue i policy

/-- The best value among a nonempty finite family of globally lawful policies
(GPU-native v0.3 §7; GT1-A7). -/
def actionValue {ι π : Type*} (components : Finset ι)
    (policies : Finset π) (hne : policies.Nonempty)
    (weight : ι → ℚ) (componentValue : ι → π → ℚ) : ℚ :=
  policies.sup' hne (policyValue components weight componentValue)

/-- Componentwise upper bounds constrain every globally lawful policy
(GPU-native v0.3 §7; GT1-A7). -/
theorem componentwise_upper_bounds_action
    {ι π : Type*}
    (components : Finset ι) (policies : Finset π) (hne : policies.Nonempty)
    (weight : ι → ℚ) (componentValue : ι → π → ℚ) (upper : ι → ℚ)
    (hweight : ∀ i ∈ components, 0 ≤ weight i)
    (hupper : ∀ policy ∈ policies, ∀ i ∈ components,
      componentValue i policy ≤ upper i) :
    actionValue components policies hne weight componentValue ≤
      weightedValue components weight upper := by
  classical
  apply Finset.sup'_le
  intro policy hpolicy
  simpa [policyValue, weightedValue] using
    componentwise_upper_sum components weight
      (fun i => componentValue i policy) upper hweight (hupper policy hpolicy)

/--
Componentwise lower witnesses may be summed when one shared lawful policy
realizes all of them simultaneously (GPU-native v0.3 §7; GT1-A7).
-/
theorem shared_policy_lower_bounds_action
    {ι π : Type*}
    (components : Finset ι) (policies : Finset π) (hne : policies.Nonempty)
    (weight : ι → ℚ) (componentValue : ι → π → ℚ) (lower : ι → ℚ)
    (hweight : ∀ i ∈ components, 0 ≤ weight i)
    (shared : π) (hshared : shared ∈ policies)
    (hlower : ∀ i ∈ components, lower i ≤ componentValue i shared) :
    weightedValue components weight lower ≤
      actionValue components policies hne weight componentValue := by
  classical
  calc
    weightedValue components weight lower ≤
        policyValue components weight componentValue shared :=
      by
        simpa [policyValue, weightedValue] using
          componentwise_upper_sum components weight lower
            (fun i => componentValue i shared) hweight hlower
    _ ≤ actionValue components policies hne weight componentValue := by
      exact Finset.le_sup' _ hshared

/-! ## Interval adjudication -/

/-- Separated valid intervals prove strict action dominance
(GPU-native v0.3 §7; GT1-A7). -/
theorem action_dominance
    {candidateValue candidateUpper incumbentLower incumbentValue : ℚ}
    (hcandidate : candidateValue ≤ candidateUpper)
    (hincumbent : incumbentLower ≤ incumbentValue)
    (hseparate : candidateUpper < incumbentLower) :
    candidateValue < incumbentValue :=
  lt_of_le_of_lt hcandidate (lt_of_lt_of_le hseparate hincumbent)

/-- A chosen action is a maximizer on a finite action set
(GPU-native v0.3 §7; GT1-A7). -/
def IsOptimalOn {α : Type*} [DecidableEq α] (actions : Finset α)
    (value : α → ℚ) (chosen : α) : Prop :=
  chosen ∈ actions ∧ ∀ other ∈ actions, value other ≤ value chosen

/-- A chosen action is the unique maximizer on a finite action set
(GPU-native v0.3 §7; GT1-A7). -/
def IsUniqueOptimalOn {α : Type*} [DecidableEq α] (actions : Finset α)
    (value : α → ℚ) (chosen : α) : Prop :=
  chosen ∈ actions ∧
    ∀ other ∈ actions, other ≠ chosen → value other < value chosen

/-- Non-strict interval separation proves membership in the optimal set
(GPU-native v0.3 §7; GT1-A7). -/
theorem nonstrict_interval_certifies_optimal_member
    {α : Type*} [DecidableEq α] (actions : Finset α)
    (value lower upper : α → ℚ) (chosen : α)
    (hchosen : chosen ∈ actions)
    (hlower : lower chosen ≤ value chosen)
    (hupper : ∀ other ∈ actions, value other ≤ upper other)
    (hseparate : ∀ other ∈ actions, other ≠ chosen →
      upper other ≤ lower chosen) :
    IsOptimalOn actions value chosen := by
  refine ⟨hchosen, ?_⟩
  intro other hother
  by_cases heq : other = chosen
  · subst other
    exact le_rfl
  · exact (hupper other hother).trans ((hseparate other hother heq).trans hlower)

/-- Strict interval separation proves a unique optimal action
(GPU-native v0.3 §7; GT1-A7). -/
theorem strict_interval_certifies_unique_optimal
    {α : Type*} [DecidableEq α] (actions : Finset α)
    (value lower upper : α → ℚ) (chosen : α)
    (hchosen : chosen ∈ actions)
    (hlower : lower chosen ≤ value chosen)
    (hupper : ∀ other ∈ actions, value other ≤ upper other)
    (hseparate : ∀ other ∈ actions, other ≠ chosen →
      upper other < lower chosen) :
    IsUniqueOptimalOn actions value chosen := by
  refine ⟨hchosen, ?_⟩
  intro other hother hne
  exact lt_of_le_of_lt (hupper other hother)
    (lt_of_lt_of_le (hseparate other hother hne) hlower)

#print axioms legal_point_lossAllowance_le_twelve
#print axioms legal_bid_lossAllowance_le_twelve
#print axioms card_one_to_seven_dvd_420
#print axioms scaled_cardinality_exact
#print axioms handCapSeven_init
#print axioms handCapSeven_step
#print axioms legalSet_card_one_to_seven
#print axioms legalSet_card_dvd_fieldScale
#print axioms openingDealCount_eq_multinomial
#print axioms rootDenominator_bit_window
#print axioms utilityMagnitude_bit_window
#print axioms unbankedPoints_conservation
#print axioms unbankedPoints_eq_total_sub_banked
#print axioms defender_final_points_interval
#print axioms trickTiles_subset_stateUnbankedTiles
#print axioms stateUnbankedPoints_conservation
#print axioms stateUnbankedPoints_conservation_step
#print axioms openingCellCount_values
#print axioms openingCellCount_le_11730
#print axioms componentwise_upper_sum
#print axioms componentwise_upper_bounds_action
#print axioms shared_policy_lower_bounds_action
#print axioms action_dominance
#print axioms nonstrict_interval_certifies_optimal_member
#print axioms strict_interval_certifies_unique_optimal

end Trick1Foundation
end Texas42
