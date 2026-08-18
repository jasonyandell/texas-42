/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1PerfectRecallNet.Types

/-!
# Perfect-recall codec, replay determination, and lineage

The executable parser proves byte well-formedness before constructing the
types in this file.  Here we prove the semantic consequences of admitting the
complete observation rather than a lossy information label.
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/--
The public facts deterministically reconstructed by replay.  `objectiveState`
is an abstract natural fingerprint because M3A and M3B deliberately use
different objective automata (M3 contract §§3--4; GT1-A20/GT1-A22).
-/
structure ReplayFacts where
  actor : ActorId
  currentHandMask : Fin (2 ^ 28)
  leader : ActorId
  objectiveState : ℕ
  terminal : Bool
  deriving DecidableEq

/--
A reviewed replay model consumes only the complete public record and explicit
current focal hand.  It has no hidden-world argument (M3 contract §3;
GT1-A20).
-/
structure ReplayModel where
  actor : List PublicPlay → ActorId
  leader : List PublicPlay → ActorId
  objectiveState : List PublicPlay → ℕ
  terminal : List PublicPlay → Bool

/-- Replay all public facts determined by one observation payload. -/
def replayFacts (model : ReplayModel) (observation : Observation) : ReplayFacts :=
  { actor := model.actor observation.record
    currentHandMask := observation.currentHandMask
    leader := model.leader observation.record
    objectiveState := model.objectiveState observation.record
    terminal := model.terminal observation.record }

/-- Equal H keys determine actor, hand, leader, objective state, and terminality. -/
theorem hKey_eq_determines_replay (model : ReplayModel) {a b : HKey}
    (h : encodeH a = encodeH b) :
    replayFacts model a.observation = replayFacts model b.observation := by
  exact congrArg (fun key => replayFacts model key.observation)
    (encodeH_injective h)

/-- Equal C keys determine the identical replay facts. -/
theorem cKey_eq_determines_replay (model : ReplayModel) {a b : CKey}
    (h : encodeC a = encodeC b) :
    replayFacts model a.observation = replayFacts model b.observation := by
  exact congrArg (fun key => replayFacts model key.observation)
    (encodeC_injective h)

/-- Constructing an H key intentionally ignores hidden-world identity. -/
def projectH (_world : WorldOrdinal) (focal : ActorId)
    (observation : Observation) : HKey :=
  { focal, observation }

/-- H bytes cannot reveal which compatible support world supplied a particle. -/
theorem projectH_world_independent (w₁ w₂ : WorldOrdinal)
    (focal : ActorId) (observation : Observation) :
    encodeH (projectH w₁ focal observation) =
      encodeH (projectH w₂ focal observation) := rfl

/-- C equality implies equality of both revealed world and complete observation. -/
theorem cKey_eq_iff (a b : CKey) :
    encodeC a = encodeC b ↔
      a.world = b.world ∧ a.focal = b.focal ∧
        a.observation = b.observation := by
  constructor
  · intro h
    have hab : a = b := encodeC_injective h
    subst b
    exact ⟨rfl, rfl, rfl⟩
  · rintro ⟨hw, hf, ho⟩
    cases a
    cases b
    simp_all

/--
Inductive public history.  A child stores exactly one focal action and the
subsequent public field suffix, so parent and action are constructor-unique.
The empty suffix is the required zero-field successor case; terminality is a
typed outcome rather than a surrogate information state (M3 contract §4;
GT1-A20/GT1-A21).
-/
inductive RecallRecord where
  | root
  | child (parent : RecallRecord) (action : TileId)
      (fieldSuffix : List PublicPlay) (terminal : Bool)

/-- Number of focal decisions represented by a recall record. -/
def RecallRecord.depth : RecallRecord → ℕ
  | .root => 0
  | .child parent _ _ _ => parent.depth + 1

/-- The unique parent, with `none` reserved for the root sentinel. -/
def RecallRecord.parent? : RecallRecord → Option RecallRecord
  | .root => none
  | .child parent _ _ _ => some parent

/-- The unique focal action, absent only at the root sentinel. -/
def RecallRecord.action? : RecallRecord → Option TileId
  | .root => none
  | .child _ action _ _ => some action

/-- Every child strictly extends its parent in focal-decision depth. -/
theorem child_strict_extension (parent : RecallRecord) (action : TileId)
    (fieldSuffix : List PublicPlay) (terminal : Bool) :
    parent.depth < (RecallRecord.child parent action fieldSuffix terminal).depth := by
  simp [RecallRecord.depth]

/-- Parent and action are recovered exactly, including an empty field suffix. -/
theorem child_unique_parent_action (parent : RecallRecord) (action : TileId)
    (fieldSuffix : List PublicPlay) (terminal : Bool) :
    (RecallRecord.child parent action fieldSuffix terminal).parent? = some parent ∧
      (RecallRecord.child parent action fieldSuffix terminal).action? = some action := by
  exact ⟨rfl, rfl⟩

/-- The root has the sole sentinel-shaped lineage. -/
theorem root_sentinel_unique (record : RecallRecord) :
    record.parent? = none ↔ record = .root := by
  cases record <;> simp [RecallRecord.parent?]

/-- A zero-field successor still has its exact unique parent and action. -/
theorem zeroField_successor_unique (parent : RecallRecord) (action : TileId)
    (terminal : Bool) :
    (RecallRecord.child parent action [] terminal).parent? = some parent ∧
      (RecallRecord.child parent action [] terminal).action? = some action := by
  exact ⟨rfl, rfl⟩

/-- A terminal child remains an edge and has no terminal information-state proxy. -/
def terminalChild (parent : RecallRecord) (action : TileId)
    (fieldSuffix : List PublicPlay) : RecallRecord :=
  .child parent action fieldSuffix true

/-- Terminal lineage retains the same unique parent and action. -/
theorem terminalChild_unique (parent : RecallRecord) (action : TileId)
    (fieldSuffix : List PublicPlay) :
    (terminalChild parent action fieldSuffix).parent? = some parent ∧
      (terminalChild parent action fieldSuffix).action? = some action := by
  exact ⟨rfl, rfl⟩

end Trick1PerfectRecallNet
end Texas42
