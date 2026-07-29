/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Play

/-!
# Texas 42 — the public record, capacity cells, and the remainder fiber

Layer C core (Math §§6.3–6.4, 7.1–7.5; ledger rows PA-C01–PA-C07,
PA-C09/C10): the viewer's public mechanical record as a machine over
public actions, its coherence with the objective play state, the derived
capacity cells (pool, per-seat allowed sets, capacities), the
current-remainder fiber, and the **losslessness theorem** — the cell
fiber equals the rule-compatible deal set's remainder image, proved by
the spec's four-case induction on public plays.

Design (wiki `support-fiber`, discrepancy D2 resolution): cells are
*derived views* of the public record plus the viewer's hand — never
stored state. The public record (`PubState`) is computed by replaying the
action list alone; the objective state is computed by replaying from a
deal; `Coheres` ties them together and carries the void-soundness
invariant.
-/

namespace Texas42

/-- The effective suit `σ̂_q^δ` as a finset (Math §3.3). -/
def Declaration.effSuit (δ : Declaration) (q : Suit) : Finset Domino :=
  Finset.univ.filter fun d => δ.effMem d q

theorem Declaration.mem_effSuit {δ : Declaration} {q : Suit} {d : Domino} :
    d ∈ δ.effSuit q ↔ δ.effMem d q := by
  simp [Declaration.effSuit]

namespace PlayState

/-- Replay a list of plays from a state (total; legality separate). -/
def replayFrom (X : PlayState) (ds : List Domino) : PlayState :=
  ds.foldl PlayState.step X

/-- The play list is legal from `X`: each play lies in the legal set of
the state it is played from. -/
def LegalFrom : PlayState → List Domino → Prop
  | _, [] => True
  | X, d :: ds => d ∈ X.legalSet ∧ LegalFrom (X.step d) ds

@[simp] theorem replayFrom_nil (X : PlayState) : X.replayFrom [] = X := rfl

@[simp] theorem replayFrom_cons (X : PlayState) (d : Domino)
    (ds : List Domino) : X.replayFrom (d :: ds) = (X.step d).replayFrom ds :=
  rfl

theorem replayFrom_append (X : PlayState) (l₁ l₂ : List Domino) :
    X.replayFrom (l₁ ++ l₂) = (X.replayFrom l₁).replayFrom l₂ :=
  List.foldl_append ..

@[simp] theorem legalFrom_nil (X : PlayState) : X.LegalFrom [] := trivial

theorem legalFrom_append (X : PlayState) (l₁ l₂ : List Domino) :
    X.LegalFrom (l₁ ++ l₂) ↔
      X.LegalFrom l₁ ∧ (X.replayFrom l₁).LegalFrom l₂ := by
  induction l₁ generalizing X with
  | nil => simp [LegalFrom]
  | cons d ds ih =>
      simp only [List.cons_append, LegalFrom, replayFrom_cons, ih,
        and_assoc]

/-- Invariants persist along legal replays. -/
theorem inv_replayFrom {X : PlayState} (hX : X.Inv) {ds : List Domino}
    (hds : X.LegalFrom ds) : (X.replayFrom ds).Inv := by
  induction ds generalizing X with
  | nil => exact hX
  | cons d ds ih => exact ih (inv_step hX hds.1) hds.2

/-- Step projection: the next leader when the trick completes. -/
theorem step_leader_complete (X : PlayState) (d : Domino)
    (h : X.trick.length = 3) :
    (X.step d).leader = trickWinner X.declaration (X.trick ++ [(X.actor, d)]) := by
  unfold step
  rw [if_pos h]

/-- Step projection: the leader is unchanged mid-trick. -/
theorem step_leader_incomplete (X : PlayState) (d : Domino)
    (h : X.trick.length ≠ 3) : (X.step d).leader = X.leader := by
  unfold step
  rw [if_neg h]

/-- The declaration never changes during play. -/
theorem step_declaration (X : PlayState) (d : Domino) :
    (X.step d).declaration = X.declaration := by
  unfold declaration
  rw [step_contract]

end PlayState

/-! ## The public record machine (PA-C01/C02, Math §6.4) -/

/-- PA-C02: the viewer-independent public record of a contracted hand —
declaration and contract, leader, current trick, actor-attributed played
sets `B_s` (including tiles in the current trick), publicly established
void contexts `V_s`, and banked scores. Computable from the public action
list alone (`pubReplay`); the capacity cells are *derived* from it. -/
structure PubState where
  contract : Contract
  leader : Seat
  trick : List (Seat × Domino)
  played : Seat → Finset Domino
  voids : Seat → Finset Suit
  score : Team → ℕ

namespace PubState

open PlayState (trickWinner trickPoints)

/-- The declaration in force. -/
def declaration (P : PubState) : Declaration := P.contract.declaration

/-- The acting seat. -/
def actor (P : PubState) : Seat := P.leader.after P.trick.length

/-- The void record after the actor publicly plays `d`: a new void for
the actor at the led context exactly when the play fails to follow it
(Math §7.1's `V_s`; the upper-bound-only update of PA-C06). -/
def voidsAfter (P : PubState) (d : Domino) : Seat → Finset Suit :=
  match P.trick.head? with
  | none => P.voids
  | some (_, d0) =>
      if P.declaration.effMem d (P.declaration.ledSuit d0) then P.voids
      else Function.update P.voids P.actor
        (insert (P.declaration.ledSuit d0) (P.voids P.actor))

/-- The public transition, mirroring `PlayState.step` on public fields
and accumulating attribution and voids. -/
def step (P : PubState) (d : Domino) : PubState where
  contract := P.contract
  leader :=
    if P.trick.length = 3
    then trickWinner P.declaration (P.trick ++ [(P.actor, d)])
    else P.leader
  trick := if P.trick.length = 3 then [] else P.trick ++ [(P.actor, d)]
  played := Function.update P.played P.actor (insert d (P.played P.actor))
  voids := P.voidsAfter d
  score :=
    if P.trick.length = 3
    then fun t =>
      if t = (trickWinner P.declaration (P.trick ++ [(P.actor, d)])).team
      then P.score t + trickPoints (P.trick ++ [(P.actor, d)])
      else P.score t
    else P.score

@[simp] theorem step_played (P : PubState) (d : Domino) :
    (P.step d).played
      = Function.update P.played P.actor (insert d (P.played P.actor)) := rfl

@[simp] theorem step_voids (P : PubState) (d : Domino) :
    (P.step d).voids = P.voidsAfter d := rfl

@[simp] theorem step_contract (P : PubState) (d : Domino) :
    (P.step d).contract = P.contract := rfl

theorem voidsAfter_lead (P : PubState) (d : Domino)
    (h : P.trick.head? = none) : P.voidsAfter d = P.voids := by
  unfold voidsAfter
  rw [h]

theorem voidsAfter_follow (P : PubState) (d : Domino) {s0 : Seat}
    {d0 : Domino} (h : P.trick.head? = some (s0, d0))
    (hf : P.declaration.effMem d (P.declaration.ledSuit d0)) :
    P.voidsAfter d = P.voids := by
  unfold voidsAfter
  rw [h]
  dsimp only
  rw [if_pos hf]

theorem voidsAfter_slough (P : PubState) (d : Domino) {s0 : Seat}
    {d0 : Domino} (h : P.trick.head? = some (s0, d0))
    (hf : ¬ P.declaration.effMem d (P.declaration.ledSuit d0)) :
    P.voidsAfter d = Function.update P.voids P.actor
      (insert (P.declaration.ledSuit d0) (P.voids P.actor)) := by
  unfold voidsAfter
  rw [h]
  dsimp only
  rw [if_neg hf]

/-- The public record at the start of a contracted hand. -/
def init (K : Contract) : PubState :=
  ⟨K, K.bidder, [], fun _ => ∅, fun _ => ∅, fun _ => 0⟩

/-- Replay the public machine over an action list. -/
def replay (K : Contract) (ds : List Domino) : PubState :=
  ds.foldl step (init K)

@[simp] theorem replay_nil (K : Contract) : replay K [] = init K := rfl

theorem replay_append (K : Contract) (l₁ l₂ : List Domino) :
    replay K (l₁ ++ l₂) = l₂.foldl step (replay K l₁) :=
  List.foldl_append ..

/-- PA-C06 (upper-bound-only update): voids only accumulate — no public
observation ever removes a recorded void. -/
theorem voids_mono (P : PubState) (d : Domino) (s : Seat) :
    P.voids s ⊆ (P.step d).voids s := by
  rw [step_voids]
  rcases htr : P.trick.head? with - | ⟨s0, d0⟩
  · rw [P.voidsAfter_lead d htr]
  · by_cases hf : P.declaration.effMem d (P.declaration.ledSuit d0)
    · rw [P.voidsAfter_follow d htr hf]
    · rw [P.voidsAfter_slough d htr hf]
      rcases eq_or_ne s P.actor with rfl | hs
      · rw [Function.update_self]
        exact Finset.subset_insert _ _
      · rw [Function.update_of_ne hs]

end PubState

/-! ## Coherence of the objective and public replays -/

/-- The coherence invariant between a deal's objective replay and the
public replay: public fields agree, hands are the deal minus the played
record, attribution is sound, and every recorded void is semantically
sound for the current hidden remainder (a seat recorded void at `q`
holds no member of `σ̂_q^δ`). -/
structure Coheres (ω : Deal) (X : PlayState) (P : PubState) : Prop where
  contract : P.contract = X.contract
  leader : P.leader = X.leader
  trick : P.trick = X.trick
  score : P.score = X.score
  hands : ∀ s, X.hands s = ω.hands s \ P.played s
  played_sub : ∀ s, P.played s ⊆ ω.hands s
  voids_sound : ∀ s q, q ∈ P.voids s → ∀ e ∈ X.hands s,
    ¬ X.declaration.effMem e q

namespace Coheres

theorem declaration {ω : Deal} {X : PlayState} {P : PubState}
    (h : Coheres ω X P) : P.declaration = X.declaration := by
  unfold PubState.declaration PlayState.declaration
  rw [h.contract]

theorem actor {ω : Deal} {X : PlayState} {P : PubState}
    (h : Coheres ω X P) : P.actor = X.actor := by
  unfold PubState.actor PlayState.actor
  rw [h.leader, h.trick]

/-- Coherence holds at the start of the contracted hand. -/
theorem init (ω : Deal) (K : Contract) :
    Coheres ω (PlayState.init ω K) (PubState.init K) := by
  refine ⟨rfl, rfl, rfl, rfl, ?_, ?_, ?_⟩
  · intro s
    simp [PlayState.init, PubState.init]
  · intro s
    simp [PubState.init]
  · intro s q hq
    simp [PubState.init] at hq

/-- PA-C09/C10 groundwork: coherence is preserved by every legal play. -/
theorem step {ω : Deal} {X : PlayState} {P : PubState} (h : Coheres ω X P)
    {d : Domino} (hd : d ∈ X.legalSet) : Coheres ω (X.step d) (P.step d) := by
  have hdh : d ∈ X.hands X.actor := X.legalSet_subset hd
  have hdω : d ∈ ω.hands X.actor := by
    have hmem := hdh
    rw [h.hands X.actor, Finset.mem_sdiff] at hmem
    exact hmem.1
  have hactor := h.actor
  have hdecl := h.declaration
  -- the hands equation after the step
  have hhands : ∀ s, (X.step d).hands s
      = ω.hands s \ (P.step d).played s := by
    intro s
    rw [PlayState.step_hands, PubState.step_played]
    rcases eq_or_ne s X.actor with rfl | hs
    · rw [hactor, Function.update_self]
      unfold PlayState.handsAfter
      rw [Function.update_self, h.hands X.actor]
      ext x
      simp only [Finset.mem_erase, Finset.mem_sdiff, Finset.mem_insert]
      tauto
    · rw [Function.update_of_ne (hactor ▸ hs)]
      unfold PlayState.handsAfter
      rw [Function.update_of_ne hs]
      exact h.hands s
  -- new-void soundness: a failed follow certifies an empty follow set
  have hvoids : ∀ s q, q ∈ (P.step d).voids s → ∀ e ∈ (X.step d).hands s,
      ¬ (X.step d).declaration.effMem e q := by
    intro s q hq e he
    rw [PubState.step_voids] at hq
    rw [PlayState.step_declaration]
    have hesub : e ∈ X.hands s := by
      rw [PlayState.step_hands] at he
      exact ((X.mem_handsAfter d e s).mp he).1
    rcases htr : P.trick.head? with - | ⟨s0, d0⟩
    · rw [P.voidsAfter_lead d htr] at hq
      exact h.voids_sound s q hq e hesub
    · by_cases hf : P.declaration.effMem d (P.declaration.ledSuit d0)
      · rw [P.voidsAfter_follow d htr hf] at hq
        exact h.voids_sound s q hq e hesub
      · rw [P.voidsAfter_slough d htr hf] at hq
        rcases eq_or_ne s P.actor with rfl | hs
        · rw [Function.update_self, Finset.mem_insert] at hq
          rcases hq with rfl | hq
          · -- the fresh void: the true follow set was empty
            have htrX : X.trick.head? = some (s0, d0) := h.trick ▸ htr
            have hfollow : ¬ (X.followSet d0).Nonempty := by
              intro hne
              rw [X.legalSet_follow htrX hne] at hd
              unfold PlayState.followSet at hd
              rw [Finset.mem_filter] at hd
              rw [hdecl] at hf
              exact hf hd.2
            intro heff
            apply hfollow
            refine ⟨e, ?_⟩
            unfold PlayState.followSet
            rw [Finset.mem_filter]
            have : e ∈ X.hands X.actor := hactor ▸ hesub
            rw [hdecl] at heff
            exact ⟨this, heff⟩
          · exact h.voids_sound _ q hq e hesub
        · rw [Function.update_of_ne hs] at hq
          exact h.voids_sound s q hq e hesub
  refine ⟨?_, ?_, ?_, ?_, hhands, ?_, hvoids⟩
  · rw [PlayState.step_contract]
    exact h.contract
  · show (if P.trick.length = 3 then _ else P.leader) = _
    rcases eq_or_ne X.trick.length 3 with h3 | h3
    · rw [if_pos (h.trick ▸ h3), X.step_leader_complete d h3,
        h.trick, hactor, hdecl]
    · rw [if_neg (fun hc => h3 (h.trick ▸ hc)),
        X.step_leader_incomplete d h3, h.leader]
  · show (if P.trick.length = 3 then [] else _) = _
    rcases eq_or_ne X.trick.length 3 with h3 | h3
    · rw [if_pos (h.trick ▸ h3), X.step_trick_complete d h3]
    · rw [if_neg (fun hc => h3 (h.trick ▸ hc)),
        X.step_trick_incomplete d h3, h.trick, hactor]
  · show (if P.trick.length = 3 then _ else P.score) = _
    rcases eq_or_ne X.trick.length 3 with h3 | h3
    · rw [if_pos (h.trick ▸ h3), X.step_score_complete d h3,
        h.trick, hactor, hdecl, h.score]
    · rw [if_neg (fun hc => h3 (h.trick ▸ hc)),
        X.step_score_incomplete d h3, h.score]
  · intro s
    rw [PubState.step_played]
    rcases eq_or_ne s P.actor with rfl | hs
    · rw [Function.update_self]
      intro x hx
      rw [Finset.mem_insert] at hx
      rcases hx with rfl | hx
      · exact hactor ▸ hdω
      · exact h.played_sub _ hx
    · rw [Function.update_of_ne hs]
      exact h.played_sub s

/-- Coherence along a whole legal replay. -/
theorem replay (ω : Deal) (K : Contract) {ds : List Domino}
    (hds : (PlayState.init ω K).LegalFrom ds) :
    Coheres ω ((PlayState.init ω K).replayFrom ds) (PubState.replay K ds) := by
  suffices h : ∀ (X : PlayState) (P : PubState), Coheres ω X P →
      ∀ ds, X.LegalFrom ds → Coheres ω (X.replayFrom ds) (ds.foldl PubState.step P) by
    exact h _ _ (Coheres.init ω K) ds hds
  intro X P hXP ds
  induction ds generalizing X P with
  | nil => exact fun _ => hXP
  | cons d ds ih =>
      intro hlegal
      exact ih _ _ (hXP.step hlegal.1) hlegal.2

end Coheres

/-! ## The derived capacity cells (PA-C02/C03, Math §7.1) -/

/-- PA-C01 (deal-local): the viewer's fixed private context for one
contracted hand — seat and initially dealt hand. The full perfect-recall
record adds the public history, which is carried by `PubState`. -/
structure ViewerCtx where
  viewer : Seat
  hand0 : Finset Domino

namespace ViewerCtx

/-- The viewer's current hand `H_m^t = H_m^0 \ B_m`. -/
def hand (v : ViewerCtx) (P : PubState) : Finset Domino :=
  v.hand0 \ P.played v.viewer

/-- The common unseen pool
`U = 𝒟 \ (H_m^t ∪ ⋃_s B_s)` (Math §7.1). -/
def pool (v : ViewerCtx) (P : PubState) : Finset Domino :=
  Finset.univ \ (v.hand P ∪ Finset.univ.biUnion P.played)

/-- The upper-bound possible-holder set
`P_s = U \ ⋃_{q ∈ V_s} σ̂_q^δ` (Math §7.1). -/
def allowed (v : ViewerCtx) (P : PubState) (s : Seat) : Finset Domino :=
  v.pool P \ (P.voids s).biUnion P.declaration.effSuit

/-- The exact remaining capacity `k_s = 7 − |B_s|` (Math §7.1). -/
def capacity (P : PubState) (s : Seat) : ℕ :=
  7 - (P.played s).card

/-- PA-C04: membership in the current-remainder fiber `Φ(𝐂)` (Math §7.3)
— hidden hands within their allowed sets, at exact capacities, disjoint,
and conserving the pool. The viewer's coordinate is normalized to `∅`. -/
def IsWorld (v : ViewerCtx) (P : PubState) (A : Seat → Finset Domino) : Prop :=
  A v.viewer = ∅
    ∧ (∀ s, s ≠ v.viewer → A s ⊆ v.allowed P s ∧ (A s).card = capacity P s)
    ∧ (∀ s t : Seat, s ≠ t → Disjoint (A s) (A t))
    ∧ Finset.univ.biUnion A = v.pool P

/-- PA-C04: a deal is compatible with the viewer's record — it deals the
viewer's hand and legally replays the observed public prefix (Math §6.3's
`Ω`). -/
def Compatible (v : ViewerCtx) (K : Contract) (ds : List Domino)
    (ω' : Deal) : Prop :=
  ω'.hands v.viewer = v.hand0 ∧ (PlayState.init ω' K).LegalFrom ds

/-- The current hidden-remainder map `ρ` (Math §6.3), normalized to `∅`
at the viewer. -/
def remainder (v : ViewerCtx) (K : Contract) (ds : List Domino) (ω' : Deal) :
    Seat → Finset Domino := fun s =>
  if s = v.viewer then ∅
  else ((PlayState.init ω' K).replayFrom ds).hands s

end ViewerCtx

/-! ## Losslessness, soundness direction (PA-C07 ⇐) -/

namespace ViewerCtx

/-- Soundness: every compatible deal's current remainder satisfies the
derived cells. Direct from the coherence invariant — capacities from the
played record, exclusions from void soundness, disjointness and
conservation from the deal partition. -/
theorem isWorld_remainder (v : ViewerCtx) (K : Contract) (ds : List Domino)
    {ω' : Deal} (hω : v.Compatible K ds ω') :
    v.IsWorld (PubState.replay K ds) (v.remainder K ds ω') := by
  obtain ⟨hhand, hlegal⟩ := hω
  have hC := Coheres.replay ω' K hlegal
  have hview : v.hand (PubState.replay K ds)
      = ((PlayState.init ω' K).replayFrom ds).hands v.viewer := by
    unfold hand
    rw [hC.hands v.viewer, hhand]
  have hrem : ∀ s, s ≠ v.viewer → v.remainder K ds ω' s
      = ((PlayState.init ω' K).replayFrom ds).hands s := by
    intro s hs
    unfold remainder
    rw [if_neg hs]
  have hmem : ∀ s, ∀ x ∈ ((PlayState.init ω' K).replayFrom ds).hands s,
      x ∈ ω'.hands s ∧ x ∉ (PubState.replay K ds).played s := by
    intro s x hx
    rw [hC.hands s, Finset.mem_sdiff] at hx
    exact hx
  have hpool : ∀ s, s ≠ v.viewer →
      ∀ x ∈ ((PlayState.init ω' K).replayFrom ds).hands s,
      x ∈ v.pool (PubState.replay K ds) := by
    intro s hs x hx
    obtain ⟨hxs, hxp⟩ := hmem s x hx
    unfold pool
    rw [Finset.mem_sdiff, Finset.mem_union]
    refine ⟨Finset.mem_univ x, ?_⟩
    rintro (hxv | hxq)
    · rw [hview, hC.hands v.viewer, Finset.mem_sdiff] at hxv
      exact Finset.disjoint_left.mp (ω'.disjoint s v.viewer hs) hxs hxv.1
    · rw [Finset.mem_biUnion] at hxq
      obtain ⟨t, -, hxt⟩ := hxq
      rcases eq_or_ne s t with rfl | hst
      · exact hxp hxt
      · exact Finset.disjoint_left.mp (ω'.disjoint s t hst) hxs
          (hC.played_sub t hxt)
  refine ⟨by unfold remainder; rw [if_pos rfl], ?_, ?_, ?_⟩
  · intro s hs
    rw [hrem s hs]
    refine ⟨?_, ?_⟩
    · intro x hx
      unfold allowed
      rw [Finset.mem_sdiff]
      refine ⟨hpool s hs x hx, ?_⟩
      rw [Finset.mem_biUnion]
      rintro ⟨q, hq, hxq⟩
      rw [Declaration.mem_effSuit] at hxq
      exact hC.voids_sound s q hq x hx (hC.declaration ▸ hxq)
    · rw [hC.hands s]
      unfold capacity
      rw [Finset.card_sdiff_of_subset (hC.played_sub s), ω'.card_hands s]
  · intro s t hst
    unfold remainder
    rcases eq_or_ne s v.viewer with rfl | hs
    · rw [if_pos rfl]
      simp
    · rcases eq_or_ne t v.viewer with rfl | ht
      · rw [if_neg hs, if_pos rfl]
        simp
      · rw [if_neg hs, if_neg ht, Finset.disjoint_left]
        intro x hxs hxt
        rw [hC.hands s, Finset.mem_sdiff] at hxs
        rw [hC.hands t, Finset.mem_sdiff] at hxt
        exact Finset.disjoint_left.mp (ω'.disjoint s t hst) hxs.1 hxt.1
  · ext x
    rw [Finset.mem_biUnion]
    constructor
    · rintro ⟨s, -, hxs⟩
      have hs : s ≠ v.viewer := by
        rintro rfl
        unfold remainder at hxs
        rw [if_pos rfl] at hxs
        simp at hxs
      rw [hrem s hs] at hxs
      exact hpool s hs x hxs
    · intro hxpool
      obtain ⟨s, hxs⟩ := (ω'.existsUnique_mem x).exists
      have hpx := hxpool
      unfold pool at hpx
      rw [Finset.mem_sdiff, Finset.mem_union] at hpx
      have hxnp : ∀ t : Seat, x ∉ (PubState.replay K ds).played t := by
        intro t hxt
        exact hpx.2 (Or.inr (Finset.mem_biUnion.mpr
          ⟨t, Finset.mem_univ t, hxt⟩))
      have hs : s ≠ v.viewer := by
        rintro rfl
        apply hpx.2
        left
        rw [hview, hC.hands v.viewer, Finset.mem_sdiff]
        exact ⟨hxs, hxnp v.viewer⟩
      refine ⟨s, Finset.mem_univ s, ?_⟩
      rw [hrem s hs, hC.hands s, Finset.mem_sdiff]
      exact ⟨hxs, hxnp s⟩

end ViewerCtx

end Texas42
