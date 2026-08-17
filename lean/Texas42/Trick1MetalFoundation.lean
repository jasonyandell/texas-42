/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1Foundation

/-!
# Finite foundations for the trick-one M2 Metal gate

These theorems discharge the finite numeric and order facts named by freeze 56.
They deliberately do not claim Rust/Lean or Metal/Rust semantic correspondence,
nor do they formalize the opening projector's `A/C/W` formulas.
-/

namespace Texas42
namespace Trick1MetalFoundation

/-! ## Fixed arenas and carrier census -/

/-- Ordered distinct response triples from a `3 * grade` hidden pool. -/
def responseTripleCount (grade : ℕ) : ℕ :=
  (3 * grade) * (3 * grade - 1) * (3 * grade - 2)

/-- M2 reserves ten rectangular slots for every ordered response triple. -/
def candidateSlotCount (grade : ℕ) : ℕ :=
  10 * responseTripleCount grade

/-- Every supported grade fits the frozen 79,800-slot rectangular arena. -/
theorem candidateSlotCount_le_79800
    {grade : ℕ} (hpos : 1 ≤ grade) (hmax : grade ≤ 7) :
    candidateSlotCount grade ≤ 79800 := by
  interval_cases grade <;> norm_num [candidateSlotCount, responseTripleCount]

/-- Exact byte accounting for the one-task projector arena. -/
theorem projectorArenaBytes_eq :
    32 + 1936 + (79800 + 2) * 64 = 5109296 := by
  norm_num

/-- Exact byte accounting for the maximum arithmetic arena. -/
theorem arithmeticArenaBytes_eq :
    16384 * 80 + (16384 + 2) * 64 = 2359424 := by
  norm_num

/-- Number of feasible `m` regimes in the GradeMatching arm at one grade. -/
def gradeMatchingRegimeCount (grade : ℕ) : ℕ :=
  Nat.min 6 (3 * grade) + 1

/-- Every supported grade has at least its `m = 0` carrier coordinate. -/
theorem gradeMatching_covers_every_grade
    {grade : ℕ} (_hpos : 1 ≤ grade) (_hmax : grade ≤ 7) :
    0 < gradeMatchingRegimeCount grade := by
  simp [gradeMatchingRegimeCount]

/-- The frozen GradeMatching generator emits exactly 46 tasks. -/
theorem gradeMatchingTaskCount_eq_46 :
    (∑ offset ∈ Finset.range 7, gradeMatchingRegimeCount (offset + 1)) = 46 := by
  decide

/-! ## At-most-ten matching-vector bound -/

/-- Number of response seats which followed the selected context. -/
def followerCount (mask : ℕ) : ℕ :=
  (if mask.testBit 0 then 1 else 0) +
  (if mask.testBit 1 then 1 else 0) +
  (if mask.testBit 2 then 1 else 0)

/-- Exact finite M2 enumeration of post-response matching-count vectors. -/
def matchingVectorCount (grade mask remaining : ℕ) : ℕ :=
  (((Finset.range grade).product (Finset.range grade)).product
      (Finset.range grade) |>.filter fun item =>
        let first := item.1.1
        let second := item.1.2
        let third := item.2
        first + second + third = remaining ∧
          (mask.testBit 0 = true ∨ first = 0) ∧
          (mask.testBit 1 = true ∨ second = 0) ∧
          (mask.testBit 2 = true ∨ third = 0)).card

/-- Every response admitted by the M2 `(grade,m,roles)` domain has at most ten
feasible matching-count vectors.  This is the fixed-width slot proof, checked
over the complete finite domain rather than inferred from sampled carrier data. -/
theorem matchingVectorCount_le_ten
    {grade mask matching : ℕ}
    (hgradePos : 1 ≤ grade) (hgradeMax : grade ≤ 7)
    (hmask : mask < 8) (hmatching : matching ≤ 6)
    (hfollowers : followerCount mask ≤ matching) :
    matchingVectorCount grade mask (matching - followerCount mask) ≤ 10 := by
  interval_cases grade <;>
    interval_cases mask <;>
      interval_cases matching <;>
        decide

/-! ## Stable compaction and all-or-nothing acceptance -/

/-- `a` occurs before `b` in a list, with their concrete occurrences exposed. -/
def OccursBefore {α : Type*} (a b : α) (items : List α) : Prop :=
  ∃ before middle after,
    items = before ++ a :: middle ++ b :: after

/-- Filtering a fixed stream preserves the order of every pair of retained
records.  This is the order fact used by stable VALID-slot compaction. -/
theorem filter_preserves_occursBefore
    {α : Type*} (keep : α → Bool) {a b : α} {items : List α}
    (ha : keep a = true) (hb : keep b = true)
    (hbefore : OccursBefore a b items) :
    OccursBefore a b (items.filter keep) := by
  rcases hbefore with ⟨before, middle, after, rfl⟩
  refine ⟨before.filter keep, middle.filter keep, after.filter keep, ?_⟩
  simp [List.filter_append, ha, hb]

/-- The only two counters whose nonzero values would constitute a partial M2
promotion. -/
structure AcceptedEvidence where
  tasks : ℕ
  payloadBytes : ℕ
  deriving DecidableEq

/-- A failed conjunction discards the candidate instead of returning a prefix. -/
def acceptIf (allChecksPassed : Bool) (candidate : AcceptedEvidence) :
    AcceptedEvidence :=
  if allChecksPassed then candidate else ⟨0, 0⟩

/-- Failure has the exact zero-task, zero-payload acceptance shape. -/
theorem failed_conjunction_accepts_nothing
    (allChecksPassed : Bool) (candidate : AcceptedEvidence)
    (hfailed : allChecksPassed = false) :
    acceptIf allChecksPassed candidate = ⟨0, 0⟩ := by
  simp [acceptIf, hfailed]

#print axioms candidateSlotCount_le_79800
#print axioms projectorArenaBytes_eq
#print axioms arithmeticArenaBytes_eq
#print axioms gradeMatching_covers_every_grade
#print axioms gradeMatchingTaskCount_eq_46
#print axioms matchingVectorCount_le_ten
#print axioms filter_preserves_occursBefore
#print axioms failed_conjunction_accepts_nothing

end Trick1MetalFoundation
end Texas42
