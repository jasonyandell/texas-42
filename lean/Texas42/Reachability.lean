/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Cells

/-!
# Texas 42 — reachability and certified states

PA-D09/PA-D10: the reachability predicate on viewer mechanical states —
some deal and legal public prefix produces the record — and the
certified-state subtype whose equality flows through the projection only
(TYPE-01 discipline: reachability is proof-irrelevant semantic evidence;
witnesses are erasable).
-/

namespace Texas42

/-- PA-D09: a viewer mechanical state `(v, P)` is reachable under
contract `K` when some deal consistent with the viewer's hand and some
legal public play prefix replays to it. -/
def Reachable (K : Contract) (v : ViewerCtx) (P : PubState) : Prop :=
  ∃ (ω : Deal) (ds : List Domino),
    ω.hands v.viewer = v.hand0
      ∧ (PlayState.init ω K).LegalFrom ds
      ∧ PubState.replay K ds = P

/-- PA-D10: the certified mechanical state — the reachability witness is
propositional, so equality, hashing, and serialization factor through
the underlying state alone (`CertifiedState := { s // Reachable(s) }`,
Handoff §5). -/
def CertifiedState (K : Contract) (v : ViewerCtx) : Type :=
  { P : PubState // Reachable K v P }

namespace CertifiedState

/-- PA-D10: certified states are equal iff their mechanical projections
are — reachability evidence never separates states. In Lean this is
definitional proof irrelevance surfaced as subtype extensionality. -/
theorem ext {K : Contract} {v : ViewerCtx}
    {c₁ c₂ : CertifiedState K v} (h : c₁.val = c₂.val) : c₁ = c₂ :=
  Subtype.ext h

theorem ext_iff {K : Contract} {v : ViewerCtx}
    (c₁ c₂ : CertifiedState K v) : c₁ = c₂ ↔ c₁.val = c₂.val :=
  ⟨congrArg Subtype.val, Subtype.ext⟩

end CertifiedState

/-- The initial public record is reachable for any viewer holding a
seven-tile hand disjoint-completable to a deal — in particular, for any
hand actually dealt. -/
theorem reachable_init (K : Contract) (v : ViewerCtx) (ω : Deal)
    (hv : ω.hands v.viewer = v.hand0) :
    Reachable K v (PubState.init K) :=
  ⟨ω, [], hv, trivial, rfl⟩

end Texas42
