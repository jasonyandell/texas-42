/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick1Foundation

/-!
# Typed semantic carriers for the freeze-57 perfect-recall net

This module describes the typed, post-parse carrier checked by the M3 gate.
The byte layouts live in the frozen contract; proving that the Rust parser and
Metal records implement these types remains an explicit correspondence debt.
Keeping that boundary explicit prevents an executable parity check from being
misreported as a theorem.
-/

namespace Texas42
namespace Trick1PerfectRecallNet

/-- The two freeze-57 treatments (M3 contract §§3--4; GT1-A19/GT1-A20). -/
inductive Treatment where
  | hidden
  | worldRevealed
  deriving DecidableEq, Repr

/-- The two independent reduction families (M3 contract §4; GT1-A21). -/
inductive ReductionFamily where
  | massBucket
  | backwardValue
  deriving DecidableEq, Repr

/-- Semantic actor ID after the wire parser has checked the range `0..3`. -/
abbrev ActorId := Fin 4

/-- Semantic domino index after the wire parser has checked the range `0..27`. -/
abbrev TileId := Fin 28

/-- Original support ordinal; it is never renumbered by C microblocks. -/
abbrev WorldOrdinal := Fin 1200

/-- One public perfect-recall play pair, in chronological order. -/
structure PublicPlay where
  actor : ActorId
  tile : TileId
  deriving DecidableEq, Repr

/--
The complete observation payload common to H and C keys.  `currentHandMask`
is the checked low-28-bit S1 mask and `record` begins with the twelve public
prefix plays followed by the fixed physical root (M3 contract §3; GT1-A20).
-/
structure Observation where
  currentHandMask : Fin (2 ^ 28)
  record : List PublicPlay
  recordMin : 13 ≤ record.length
  recordMax : record.length ≤ 28

/-- The semantic content of `M3PerfectRecallKeyV1` after byte validation. -/
structure HKey where
  focal : ActorId
  observation : Observation

/-- The semantic content of `M3WorldRevealedKeyV1` after byte validation. -/
structure CKey where
  world : WorldOrdinal
  focal : ActorId
  observation : Observation

/--
The disjoint post-parse key carrier.  Its constructors model codec tags 1 and
2; no C value can contain an embedded H value (M3 contract §3; GT1-A20).
-/
inductive WireKey where
  | hidden : HKey → WireKey
  | worldRevealed : CKey → WireKey

/-- Typed H encoder into the post-parse wire carrier. -/
def encodeH (key : HKey) : WireKey := .hidden key

/-- Typed C encoder into the post-parse wire carrier. -/
def encodeC (key : CKey) : WireKey := .worldRevealed key

/-- Typed H decoder; tag 2 is rejected rather than reinterpreted. -/
def decodeH : WireKey → Option HKey
  | .hidden key => some key
  | .worldRevealed _ => none

/-- Typed C decoder; tag 1 is rejected rather than embedded. -/
def decodeC : WireKey → Option CKey
  | .hidden _ => none
  | .worldRevealed key => some key

/-- H encode/decode is exact. -/
theorem decodeH_encodeH (key : HKey) : decodeH (encodeH key) = some key := rfl

/-- C encode/decode is exact. -/
theorem decodeC_encodeC (key : CKey) : decodeC (encodeC key) = some key := rfl

/-- The H codec is injective. -/
theorem encodeH_injective : Function.Injective encodeH := by
  intro a b h
  cases h
  rfl

/-- The C codec is injective. -/
theorem encodeC_injective : Function.Injective encodeC := by
  intro a b h
  cases h
  rfl

/-- Codec tags give semantic type separation. -/
theorem encodeH_ne_encodeC (h : HKey) (c : CKey) : encodeH h ≠ encodeC c := by
  intro hEq
  cases hEq

/--
The frozen task frame scopes key equality.  Only the task ordinal is needed in
the abstract model because the executable gate reconstructs every redundant
frame field before admitting a value (M3 contract §§2--3; GT1-A20).
-/
structure TaskScope where
  task : Fin 8
  deriving DecidableEq

/-- A typed key together with the task frame that fixes objective and root. -/
structure ScopedKey where
  scope : TaskScope
  key : WireKey

/-- Scoped equality is exactly frame equality plus complete typed-key equality. -/
theorem scopedKey_eq_iff (a b : ScopedKey) :
    a = b ↔ a.scope = b.scope ∧ a.key = b.key := by
  constructor
  · intro h
    subst b
    exact ⟨rfl, rfl⟩
  · rintro ⟨hs, hk⟩
    cases a
    cases b
    simp_all

/-- Exact exponent-framed nonnegative mass used by both reduction families. -/
structure ExponentMass where
  numerator : ℕ
  exponent : ℕ
  deriving DecidableEq, Repr

/-- The four focal epochs in every official freeze-57 task. -/
abbrev FocalEpoch := Fin 4

/-- Root is the only parent sentinel admitted by the semantic model. -/
inductive ParentRef where
  | root
  | state (key : WireKey)

end Trick1PerfectRecallNet
end Texas42
