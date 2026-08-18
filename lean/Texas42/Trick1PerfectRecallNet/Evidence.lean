/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.Bounds

/-!
# Family usage, counter ownership, and all-or-nothing evidence
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/-- Every forward EDGE_A row has exactly one MASS_BUCKET destination. -/
inductive MassBucketUse (Child TerminalBucket : Type*) where
  | child (key : Child)
  | terminal (bucket : TerminalBucket)

/-- The two constructors are disjoint, so no edge enters both mass ranges. -/
theorem massBucket_child_terminal_disjoint {Child TerminalBucket : Type*}
    (child : Child) (bucket : TerminalBucket) :
    MassBucketUse.child child ≠ MassBucketUse.terminal bucket := by
  intro h
  cases h

/-- BACKWARD_VALUE uses one least witness per child group or one terminal edge. -/
inductive BackwardUse (Witness TerminalEdge : Type*) where
  | child (leastWitness : Witness)
  | terminal (edge : TerminalEdge)

/-- Child-witness and direct-terminal rows are disjoint. -/
theorem backward_child_terminal_disjoint {Witness TerminalEdge : Type*}
    (witness : Witness) (edge : TerminalEdge) :
    BackwardUse.child witness ≠ BackwardUse.terminal edge := by
  intro h
  cases h

/-- If each family uses at most E rows, the aggregate uses at most 2E. -/
theorem two_family_level_zero_bound {massRows backwardRows edges : ℕ}
    (hmass : massRows = edges) (hbackward : backwardRows ≤ edges) :
    massRows + backwardRows ≤ 2 * edges := by
  omega

/-- Signed rows enter exactly one of the nonnegative and negative lanes. -/
theorem sign_lanes_partition (value : ℤ) :
    (0 ≤ value ∧ ¬value < 0) ∨ (value < 0 ∧ ¬0 ≤ value) := by
  omega

/-- Serial lifecycle makes the two reduction scratch arenas noncoexistent. -/
inductive ScratchOwner where
  | none
  | massBucket
  | backwardValue
  deriving DecidableEq

/-- A scratch owner cannot name both semantic families. -/
theorem family_scratch_noncoexistence (owner : ScratchOwner) :
    owner = .massBucket → owner ≠ .backwardValue := by
  intro hmass hbackward
  simp_all

/-- Exact four-epoch receipt for one treatment path. -/
structure EpochReceipt where
  visits : Fin 4 → ℕ

/-- Total treatment visits are the sum of the four disjoint focal epochs. -/
def EpochReceipt.total (receipt : EpochReceipt) : ℕ :=
  ∑ epoch, receipt.visits epoch

/-- Epoch visits partition the treatment total by definition. -/
theorem epoch_partition (receipt : EpochReceipt) :
    (∑ epoch, receipt.visits epoch) = receipt.total := rfl

/--
A node is charged to the current epoch before descent.  Only descendants of a
focal choice advance the epoch, so the focal arrival itself is charged once to
the preceding epoch (M3 contract §§5, 12; GT1-A21).
-/
def chargedEpoch (current : ℕ) : ℕ := current

/-- Advance the epoch only after applying a focal action. -/
def descendantEpoch (current : ℕ) (focalAction : Bool) : ℕ :=
  if focalAction then current + 1 else current

/-- The focal arrival charge is the pre-action epoch, exactly once. -/
theorem focal_arrival_once_only (current : ℕ) :
    chargedEpoch current = current ∧ descendantEpoch current true = current + 1 := by
  simp [chargedEpoch, descendantEpoch]

/-- Independently owned oracle, production-CPU, and Metal counters. -/
structure TreatmentVisits where
  oracle : ℕ
  productionCpu : ℕ
  metal : ℕ
  epochs : EpochReceipt

/-- Acceptance requires all independent visit counters and epoch sum to agree. -/
def TreatmentVisits.Accepted (visits : TreatmentVisits) : Prop :=
  visits.oracle = visits.productionCpu ∧
    visits.productionCpu = visits.metal ∧
    visits.epochs.total = visits.oracle

/-- The receipt's treatment count is the common count, never their sum. -/
theorem accepted_visits_common (visits : TreatmentVisits)
    (h : visits.Accepted) :
    visits.oracle = visits.productionCpu ∧
      visits.oracle = visits.metal ∧
      visits.epochs.total = visits.oracle := by
  rcases h with ⟨hoc, hcm, hepoch⟩
  exact ⟨hoc, hoc.trans hcm, hepoch⟩

/-- Every independently owned treatment census named by freeze 57. -/
structure TreatmentCensus where
  visits : ℕ
  successorEmissions : ℕ
  edgeA : ℕ
  keys : ℕ
  singletonStates : ℕ
  actions : ℕ
  terminals : ℕ
  massBucketLevelZero : ℕ
  backwardValueLevelZero : ℕ
  deriving DecidableEq

/-- Oracle, production CPU, and post-Metal census owners remain separate. -/
structure IndependentCensuses where
  oracle : TreatmentCensus
  productionCpu : TreatmentCensus
  metal : TreatmentCensus

/-- Acceptance is exact fieldwise equality, not an inferred or shared counter. -/
def IndependentCensuses.Accepted (census : IndependentCensuses) : Prop :=
  census.oracle = census.productionCpu ∧
    census.productionCpu = census.metal

/-- Accepted independent censuses have one common complete record. -/
theorem accepted_censuses_common (census : IndependentCensuses)
    (h : census.Accepted) :
    census.oracle = census.productionCpu ∧ census.oracle = census.metal := by
  exact ⟨h.1, h.1.trans h.2⟩

/-- Each reduction family is separately bounded by the EDGE_A census. -/
def TreatmentCensus.FamilyBounded (census : TreatmentCensus) : Prop :=
  census.massBucketLevelZero = census.edgeA ∧
    census.backwardValueLevelZero ≤ census.edgeA

/-- The two admitted families are therefore bounded by twice EDGE_A. -/
theorem accepted_family_aggregate_bound (census : TreatmentCensus)
    (h : census.FamilyBounded) :
    census.massBucketLevelZero + census.backwardValueLevelZero ≤
      2 * census.edgeA := by
  exact two_family_level_zero_bound h.1 h.2

/-- The sixteen task/treatment coordinates in one official eight-task run. -/
abbrev RunTreatment := Fin 16

/-- GLOBAL visit total is the exact sum of all sixteen treatment values. -/
def globalVisitTotal (visits : RunTreatment → ℕ) : ℕ := ∑ treatment, visits treatment

/-- No hidden seventeenth term occurs in the GLOBAL visit sum. -/
theorem global_visit_sum_sixteen (visits : RunTreatment → ℕ) :
    globalVisitTotal visits = ∑ treatment : Fin 16, visits treatment := rfl

/-- One selected-policy terminal edge with exact root-scale contribution. -/
structure SelectedTerminal where
  edgeOrdinal : ℕ
  contribution : ℤ
  deriving DecidableEq

/-- Evidence-only selected-policy fold; every list occurrence is visited once. -/
def selectedTerminalFold (edges : List SelectedTerminal) : ℤ :=
  (edges.map SelectedTerminal.contribution).sum

/-- Equal disjoint CPU-raw and post-Metal terminal streams have equal folds. -/
theorem selected_terminal_cpu_metal_equal {cpu metal : List SelectedTerminal}
    (h : cpu = metal) : selectedTerminalFold cpu = selectedTerminalFold metal := by
  rw [h]

/-- A selected-policy bucket check cannot alter the already sealed root value. -/
theorem evidence_fold_matches_sealed_root (edges : List SelectedTerminal)
    (sealedRoot : ℤ) (h : selectedTerminalFold edges = sealedRoot) :
    selectedTerminalFold edges = sealedRoot := h

/-- Physical writer identity for one of seven field-output slots. -/
def fieldSlotIndex (input : ℕ) (slot : Fin 7) : ℕ :=
  1 + 7 * input + slot

/-- Distinct `(input,slot)` pairs write disjoint output records. -/
theorem fieldSlotIndex_injective :
    Function.Injective (fun pair : ℕ × Fin 7 => fieldSlotIndex pair.1 pair.2) := by
  rintro ⟨inputA, slotA⟩ ⟨inputB, slotB⟩ h
  simp only [fieldSlotIndex] at h
  have ha := slotA.isLt
  have hb := slotB.isLt
  have hinput : inputA = inputB := by omega
  subst inputB
  have hslot : slotA = slotB := by
    apply Fin.ext
    omega
  subst slotB
  rfl

/-- Purpose-22 kinds 13--15 have distinct typed owners. -/
inductive CounterHighWaterKind where
  | successorEmission
  | edgeA
  | aggregateLevelZero
  deriving DecidableEq

/-- Exact all-or-nothing candidate evidence shape. -/
structure CandidateEvidence where
  tasks : ℕ
  payloadBytes : ℕ
  deriving DecidableEq

/-- A failed complete conjunction accepts no prefix. -/
def acceptEvidence (allChecksPassed : Bool)
    (candidate : CandidateEvidence) : CandidateEvidence :=
  if allChecksPassed then candidate else ⟨0, 0⟩

/-- Failure has the exact zero-task, zero-payload shape. -/
theorem failed_conjunction_accepts_nothing (candidate : CandidateEvidence) :
    acceptEvidence false candidate = ⟨0, 0⟩ := by
  rfl

/-- A successful conjunction accepts the complete candidate unchanged. -/
theorem successful_conjunction_accepts_all (candidate : CandidateEvidence) :
    acceptEvidence true candidate = candidate := by
  rfl

/-- Evaluate the complete ordered check list before accepting any evidence. -/
def acceptChecks (checks : List Bool) (candidate : CandidateEvidence) :
    CandidateEvidence :=
  acceptEvidence (checks.all id) candidate

/-- Any failed member of the conjunction forces zero accepted evidence. -/
theorem failed_check_accepts_nothing {checks : List Bool}
    (candidate : CandidateEvidence) {check : Bool}
    (hmember : check ∈ checks) (hfailed : check = false) :
    acceptChecks checks candidate = ⟨0, 0⟩ := by
  subst check
  simp [acceptChecks, acceptEvidence, List.all_eq_true, hmember]

/-- Immutable skeleton-to-final replacement is expressed as a true bijection. -/
structure ImmutableReplacement (Skeleton Final : Type*) where
  correspondence : Skeleton ≃ Final

/-- Every final record has one unique skeleton antecedent. -/
theorem immutable_replacement_unique {Skeleton Final : Type*}
    (replacement : ImmutableReplacement Skeleton Final) (final : Final) :
    ∃! skeleton, replacement.correspondence skeleton = final := by
  refine ⟨replacement.correspondence.symm final, ?_, ?_⟩
  · exact replacement.correspondence.apply_symm_apply final
  · intro other hother
    exact replacement.correspondence.injective
      (hother.trans (replacement.correspondence.apply_symm_apply final).symm)

end Trick1PerfectRecallNet
end Texas42
