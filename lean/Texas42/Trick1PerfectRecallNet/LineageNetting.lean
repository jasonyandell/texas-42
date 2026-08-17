/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.CodecReplay

/-!
# Parent-local netting, complete-run merge, and C-world separation

These generic theorems are the mathematical license for the host's full-byte
sort/merge.  They do not license hash equality or slab-local sealing.
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/-- One lawful successor emission before byte-equal netting. -/
structure Emission (Key Action Mass : Type*) where
  parent : ParentRef
  action : Action
  child : Option Key
  source : WorldOrdinal
  arrival : ℕ
  mass : Mass

/-- The complete parent/action/child identity used for semantic grouping. -/
def Emission.groupKey {Key Action Mass : Type*}
    (edge : Emission Key Action Mass) : ParentRef × Action × Option Key :=
  (edge.parent, edge.action, edge.child)

/--
Two emissions net exactly when their complete parent, focal action, and child
or terminal identity agree.  Source, arrival, and mass never weaken the key.
-/
theorem emission_group_eq_iff {Key Action Mass : Type*}
    (a b : Emission Key Action Mass) :
    a.groupKey = b.groupKey ↔
      a.parent = b.parent ∧ a.action = b.action ∧ a.child = b.child := by
  simp only [Emission.groupKey, Prod.mk.injEq]

/-- Deterministic multi-run merge consumes every input run exactly once. -/
def mergeRuns {α : Type*} (runs : List (List α)) : List α := runs.flatten

/-- Membership in the complete merge is membership in some submitted run. -/
theorem mem_mergeRuns_iff {α : Type*} {item : α} {runs : List (List α)} :
    item ∈ mergeRuns runs ↔ ∃ run ∈ runs, item ∈ run := by
  simp [mergeRuns]

/-- Concatenating a second run family cannot remove an earlier record. -/
theorem mergeRuns_monotone_left {α : Type*} (left right : List (List α)) :
    ∀ item ∈ mergeRuns left, item ∈ mergeRuns (left ++ right) := by
  intro item h
  simp only [mergeRuns, List.flatten_append, List.mem_append]
  exact Or.inl h

/-- Complete merge preserves the exact sum of every input record. -/
theorem mergeRuns_sum {α M : Type*} [AddCommMonoid M]
    (value : α → M) (runs : List (List α)) :
    ((mergeRuns runs).map value).sum =
      (runs.map fun run => (run.map value).sum).sum := by
  induction runs with
  | nil => rfl
  | cons run runs ih =>
      simp [mergeRuns, Function.comp_def, ih]

/-- A revealed edge carries its semantic world in the grouping identity. -/
structure RevealedEdge (Key Action Mass : Type*) where
  world : WorldOrdinal
  parent : ParentRef
  action : Action
  child : Option Key
  mass : Mass

/-- Exact C grouping includes world identity before every observation field. -/
def RevealedEdge.groupKey {Key Action Mass : Type*}
    (edge : RevealedEdge Key Action Mass) :
    WorldOrdinal × ParentRef × Action × Option Key :=
  (edge.world, edge.parent, edge.action, edge.child)

/-- Byte-equal C groups cannot cross revealed worlds. -/
theorem revealed_group_world_eq {Key Action Mass : Type*}
    {a b : RevealedEdge Key Action Mass} (h : a.groupKey = b.groupKey) :
    a.world = b.world := by
  exact congrArg Prod.fst h

/-- A physical C microblock is a list of still-world-tagged records. -/
abbrev WorldBlock (α : Type*) := List α

/-- A block partition is semantic when flattening it is a permutation of input. -/
def IsBlockPartition {α : Type*} (input : List α)
    (blocks : List (WorldBlock α)) : Prop :=
  blocks.flatten.Perm input

/-- Any semantic C block partition preserves every commutative fold. -/
theorem blockPartition_fold_invariant {α M : Type*} [AddCommMonoid M]
    (value : α → M) {input : List α} {blocks : List (WorldBlock α)}
    (h : IsBlockPartition input blocks) :
    (blocks.flatten.map value).sum = (input.map value).sum := by
  exact (h.map value).sum_eq

/-- Block-size-one and block-size-sixteen schedules have identical folds. -/
theorem blockSize_one_sixteen_identical {α M : Type*} [AddCommMonoid M]
    (value : α → M) {input : List α}
    {one sixteen : List (WorldBlock α)}
    (hOne : IsBlockPartition input one)
    (hSixteen : IsBlockPartition input sixteen) :
    (one.flatten.map value).sum = (sixteen.flatten.map value).sum := by
  calc
    (one.flatten.map value).sum = (input.map value).sum :=
      blockPartition_fold_invariant value hOne
    _ = (sixteen.flatten.map value).sum :=
      (blockPartition_fold_invariant value hSixteen).symm

/-- Semantic identity of one net edge group. -/
structure EdgeGroup (Key Action : Type*) where
  parent : ParentRef
  action : Action
  child : Option Key

/-- A canonical witness retains its group identity and source coordinates. -/
structure EdgeWitness (Key Action : Type*) where
  group : EdgeGroup Key Action
  source : WorldOrdinal
  arrival : ℕ

/-- Any witness selector that retains its group is injective across groups. -/
theorem witness_selector_injective {Key Action : Type*}
    (select : EdgeGroup Key Action → EdgeWitness Key Action)
    (hgroup : ∀ group, (select group).group = group) :
    Function.Injective select := by
  intro a b h
  have := congrArg EdgeWitness.group h
  simpa [hgroup] using this

/-- The explicit group coordinate makes source-witness reuse impossible. -/
theorem distinct_groups_have_distinct_witnesses {Key Action : Type*}
    (select : EdgeGroup Key Action → EdgeWitness Key Action)
    (hgroup : ∀ group, (select group).group = group)
    {a b : EdgeGroup Key Action} (hne : a ≠ b) :
    select a ≠ select b := by
  exact fun h => hne (witness_selector_injective select hgroup h)

end Trick1PerfectRecallNet
end Texas42
