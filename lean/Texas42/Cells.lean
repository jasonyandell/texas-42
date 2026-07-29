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

/-- The actor's hand loses exactly the played tile. -/
theorem hands_step_actor (X : PlayState) (d : Domino) :
    (X.step d).hands X.actor = (X.hands X.actor).erase d := by
  rw [step_hands]
  unfold handsAfter
  rw [Function.update_self]

/-- PA-C10 (viewer-play identity groundwork): a non-actor's hand is
untouched by the play. -/
theorem hands_step_ne (X : PlayState) (d : Domino) {s : Seat}
    (hs : s ≠ X.actor) : (X.step d).hands s = X.hands s := by
  rw [step_hands]
  unfold handsAfter
  rw [Function.update_of_ne hs]

/-- Legality is public: two states agreeing on leader, trick, contract,
and the actor's hand have the same legal set. -/
theorem legalSet_congr {X Y : PlayState} (hl : Y.leader = X.leader)
    (ht : Y.trick = X.trick) (hc : Y.contract = X.contract)
    (hh : Y.hands Y.actor = X.hands X.actor) : Y.legalSet = X.legalSet := by
  have hdecl : Y.declaration = X.declaration := by
    unfold declaration
    rw [hc]
  rcases htr : X.trick.head? with - | ⟨s0, d0⟩
  · have hX : X.trick = [] := List.head?_eq_none_iff.mp htr
    have hY : Y.trick = [] := by rw [ht, hX]
    rw [Y.legalSet_lead hY, X.legalSet_lead hX, hh]
  · have htrY : Y.trick.head? = some (s0, d0) := by rw [ht, htr]
    have hfs : Y.followSet d0 = X.followSet d0 := by
      unfold followSet
      rw [hh, hdecl]
    by_cases hne : (X.followSet d0).Nonempty
    · rw [Y.legalSet_follow htrY (hfs ▸ hne), X.legalSet_follow htr hne, hfs]
    · rw [Y.legalSet_slough htrY (hfs ▸ hne), X.legalSet_slough htr hne, hh]

end PlayState

/-- Updating one coordinate of an indexed family to insert an element
inserts it into the indexed union. -/
theorem biUnion_update_insert (f : Seat → Finset Domino) (a : Seat)
    (d : Domino) :
    Finset.univ.biUnion (Function.update f a (insert d (f a)))
      = insert d (Finset.univ.biUnion f) := by
  ext x
  simp only [Finset.mem_biUnion, Finset.mem_univ, true_and,
    Finset.mem_insert]
  constructor
  · rintro ⟨s, hs⟩
    rcases eq_or_ne s a with rfl | hsa
    · rw [Function.update_self, Finset.mem_insert] at hs
      rcases hs with rfl | hs
      · exact Or.inl rfl
      · exact Or.inr ⟨s, hs⟩
    · rw [Function.update_of_ne hsa] at hs
      exact Or.inr ⟨s, hs⟩
  · rintro (rfl | ⟨s, hs⟩)
    · exact ⟨a, by rw [Function.update_self]; exact Finset.mem_insert_self ..⟩
    · rcases eq_or_ne s a with rfl | hsa
      · exact ⟨s, by rw [Function.update_self]
                     exact Finset.mem_insert_of_mem hs⟩
      · exact ⟨s, by rw [Function.update_of_ne hsa]; exact hs⟩

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

theorem step_declaration (P : PubState) (d : Domino) :
    (P.step d).declaration = P.declaration := by
  unfold declaration
  rw [step_contract]

/-- Voids of a non-actor are untouched. -/
theorem voidsAfter_ne (P : PubState) (d : Domino) {s : Seat}
    (hs : s ≠ P.actor) : P.voidsAfter d s = P.voids s := by
  rcases htr : P.trick.head? with - | ⟨s0, d0⟩
  · rw [P.voidsAfter_lead d htr]
  · by_cases hf : P.declaration.effMem d (P.declaration.ledSuit d0)
    · rw [P.voidsAfter_follow d htr hf]
    · rw [P.voidsAfter_slough d htr hf, Function.update_of_ne hs]

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

theorem replay_snoc (K : Contract) (l : List Domino) (d : Domino) :
    replay K (l ++ [d]) = (replay K l).step d := by
  rw [replay_append]
  rfl

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

/-! ## Cell updates under one public play -/

/-- The viewer's public hand is untouched by a hidden actor's play. -/
theorem hand_step_ne (v : ViewerCtx) (P : PubState) (d : Domino)
    (hne : P.actor ≠ v.viewer) : v.hand (P.step d) = v.hand P := by
  unfold hand
  rw [PubState.step_played, Function.update_of_ne (Ne.symm hne)]

/-- A hidden actor's play removes exactly the played tile from the pool. -/
theorem pool_step_hidden (v : ViewerCtx) (P : PubState) (d : Domino)
    (hne : P.actor ≠ v.viewer) :
    v.pool (P.step d) = (v.pool P).erase d := by
  unfold pool
  rw [hand_step_ne v P d hne, PubState.step_played, biUnion_update_insert]
  ext x
  simp only [Finset.mem_sdiff, Finset.mem_union, Finset.mem_univ, true_and,
    Finset.mem_insert, Finset.mem_erase]
  tauto

/-- The viewer's own play leaves the pool unchanged: the tile was already
excluded through the viewer's hand. -/
theorem pool_step_viewer (v : ViewerCtx) (P : PubState) (d : Domino)
    (hva : P.actor = v.viewer) (hd : d ∈ v.hand P) :
    v.pool (P.step d) = v.pool P := by
  have hd0 : d ∈ v.hand0 ∧ d ∉ P.played v.viewer := by
    have hdm := hd
    unfold hand at hdm
    rw [Finset.mem_sdiff] at hdm
    exact hdm
  obtain ⟨hd1, hd2⟩ := hd0
  unfold pool hand
  rw [PubState.step_played, hva, Function.update_self, biUnion_update_insert]
  ext x
  simp only [Finset.mem_sdiff, Finset.mem_union, Finset.mem_univ, true_and,
    Finset.mem_insert]
  by_cases hxd : x = d
  · subst hxd
    tauto
  · tauto

/-- A non-actor's capacity is unchanged. -/
theorem capacity_step_ne (P : PubState) (d : Domino) {s : Seat}
    (hs : s ≠ P.actor) : capacity (P.step d) s = capacity P s := by
  unfold capacity
  rw [PubState.step_played, Function.update_of_ne hs]

/-- The actor's capacity drops by one. -/
theorem capacity_step_actor (P : PubState) (d : Domino)
    (hd : d ∉ P.played P.actor) :
    capacity (P.step d) P.actor = capacity P P.actor - 1 := by
  unfold capacity
  rw [PubState.step_played, Function.update_self,
    Finset.card_insert_of_notMem hd]
  omega

/-- A hidden play shrinks a non-actor's allowed set by exactly the played
tile. -/
theorem allowed_step_ne (v : ViewerCtx) (P : PubState) (d : Domino)
    {s : Seat} (hs : s ≠ P.actor) (hne : P.actor ≠ v.viewer) :
    v.allowed (P.step d) s = (v.allowed P s).erase d := by
  unfold allowed
  rw [pool_step_hidden v P d hne, PubState.step_voids,
    PubState.voidsAfter_ne P d hs, PubState.step_declaration]
  ext x
  simp only [Finset.mem_sdiff, Finset.mem_erase]
  tauto

/-- One public play never grows the pool. -/
theorem pool_step_subset (v : ViewerCtx) (P : PubState) (d : Domino) :
    v.pool (P.step d) ⊆ v.pool P := by
  intro x hx
  unfold pool at hx ⊢
  rw [Finset.mem_sdiff, Finset.mem_union] at hx ⊢
  refine ⟨hx.1, fun hc => hx.2 ?_⟩
  unfold hand at hc ⊢
  rw [PubState.step_played]
  rcases hc with hc | hc
  · rw [Finset.mem_sdiff] at hc
    by_cases hxp : x ∈ Function.update P.played P.actor
        (insert d (P.played P.actor)) v.viewer
    · right
      exact Finset.mem_biUnion.mpr ⟨v.viewer, Finset.mem_univ _, hxp⟩
    · left
      rw [Finset.mem_sdiff]
      exact ⟨hc.1, hxp⟩
  · right
    rw [Finset.mem_biUnion] at hc
    obtain ⟨t, -, hct⟩ := hc
    rw [Finset.mem_biUnion]
    refine ⟨t, Finset.mem_univ t, ?_⟩
    rcases eq_or_ne t P.actor with rfl | hta
    · rw [Function.update_self]
      exact Finset.mem_insert_of_mem hct
    · rw [Function.update_of_ne hta]
      exact hct

/-- The actor's allowed set only shrinks. -/
theorem allowed_step_actor_subset (v : ViewerCtx) (P : PubState)
    (d : Domino) : v.allowed (P.step d) P.actor ⊆ v.allowed P P.actor := by
  unfold allowed
  intro x hx
  rw [Finset.mem_sdiff] at hx ⊢
  refine ⟨pool_step_subset v P d hx.1, fun hc => hx.2 ?_⟩
  rw [Finset.mem_biUnion] at hc ⊢
  obtain ⟨q, hq, hxq⟩ := hc
  refine ⟨q, ?_, ?_⟩
  · rw [PubState.step_voids]
    exact PubState.voids_mono P d P.actor hq
  · rwa [PubState.step_declaration]

/-- Cells seen by other seats are invariant under a viewer play. -/
theorem allowed_step_viewer (v : ViewerCtx) (P : PubState) (d : Domino)
    {s : Seat} (hs : s ≠ P.actor) (hva : P.actor = v.viewer)
    (hd : d ∈ v.hand P) : v.allowed (P.step d) s = v.allowed P s := by
  unfold allowed
  rw [pool_step_viewer v P d hva hd, PubState.step_voids,
    PubState.voidsAfter_ne P d hs, PubState.step_declaration]

/-! ## Losslessness, completeness direction (PA-C07 ⇒) -/

/-- Completeness: every member of the derived cell fiber is realized by a
rule-compatible complete deal — the spec's four-case induction on the
public play prefix (Math §7.5). Viewer actions leave hidden cells
untouched; a hidden lead or follow removes exactly the played tile (the
tile itself is the witness — no positive follower clause survives); a
failure to follow deletes the whole follow set via the recorded void, and
adding the tile back reconstructs a legal slough. The reverse
construction also uses a fact the prose leaves implicit: a hidden seat's
publicly played tile respects that seat's previously recorded voids
(derived here from true-trajectory void soundness, `hd_allowed`). -/
theorem exists_deal_of_isWorld (ω : Deal) (K : Contract) (v : ViewerCtx)
    (hv : ω.hands v.viewer = v.hand0) :
    ∀ ds, (PlayState.init ω K).LegalFrom ds →
      ∀ A, v.IsWorld (PubState.replay K ds) A →
        ∃ ω' : Deal, v.Compatible K ds ω' ∧ v.remainder K ds ω' = A := by
  have hcard0 : v.hand0.card = 7 := by
    rw [← hv]
    exact ω.card_hands v.viewer
  intro ds
  induction ds using List.reverseRecOn with
  | nil =>
      intro _ A hA
      rw [PubState.replay_nil] at hA
      obtain ⟨hAm, hAcell, hAdisj, hAcons⟩ := hA
      have hbiu : (Finset.univ.biUnion fun _ : Seat => (∅ : Finset Domino))
          = ∅ := rfl
      have hallow0 : ∀ s, v.allowed (PubState.init K) s
          = Finset.univ \ v.hand0 := by
        intro s
        unfold allowed pool hand
        simp [PubState.init, hbiu]
      have hcap0 : ∀ s, capacity (PubState.init K) s = 7 := by
        intro s
        unfold capacity
        simp [PubState.init]
      have hAsub : ∀ s, s ≠ v.viewer → Disjoint v.hand0 (A s) := by
        intro s hs
        rw [Finset.disjoint_right]
        intro x hxA hx0
        have hmem := (hAcell s hs).1 hxA
        rw [hallow0 s, Finset.mem_sdiff] at hmem
        exact hmem.2 hx0
      refine ⟨⟨fun s => if s = v.viewer then v.hand0 else A s, ?_, ?_⟩,
        ⟨?_, trivial⟩, ?_⟩
      · intro s
        split_ifs with h
        · exact hcard0
        · rw [(hAcell s h).2, hcap0]
      · intro s t hst
        split_ifs with h1 h2 h2
        · exact absurd (h1.trans h2.symm) hst
        · exact hAsub t h2
        · exact (hAsub s h1).symm
        · exact hAdisj s t hst
      · simp
      · funext s
        unfold remainder
        split_ifs with h
        · rw [h]
          exact hAm.symm
        · exact if_neg h
  | append_singleton l d ih =>
      intro hlegal A' hA'
      rw [PlayState.legalFrom_append] at hlegal
      obtain ⟨hlegal_l, hstep⟩ := hlegal
      have hd_legal : d ∈ ((PlayState.init ω K).replayFrom l).legalSet :=
        hstep.1
      have hCt := Coheres.replay ω K hlegal_l
      rw [PubState.replay_snoc] at hA'
      obtain ⟨hAm', hAcell', hAdisj', hAcons'⟩ := hA'
      have hdh : d ∈ ((PlayState.init ω K).replayFrom l).hands
          ((PubState.replay K l).actor) := by
        rw [hCt.actor]
        exact PlayState.legalSet_subset _ hd_legal
      have hdP : d ∈ ω.hands ((PubState.replay K l).actor)
          ∧ d ∉ (PubState.replay K l).played ((PubState.replay K l).actor) := by
        rw [hCt.hands _, Finset.mem_sdiff] at hdh
        exact hdh
      have hdh2 : d ∈ ((PlayState.init ω K).replayFrom l).hands
          ((PubState.replay K l).actor) := by
        rw [hCt.hands _, Finset.mem_sdiff]
        exact hdP
      rcases eq_or_ne ((PubState.replay K l).actor) v.viewer with hva | hva
      · -- viewer action: hidden cells untouched (PA-C10)
        have hd_hand : d ∈ v.hand (PubState.replay K l) := by
          unfold hand
          rw [Finset.mem_sdiff]
          refine ⟨?_, ?_⟩
          · rw [← hv, ← hva]
            exact hdP.1
          · rw [← hva]
            exact hdP.2
        have hA : v.IsWorld (PubState.replay K l) A' := by
          refine ⟨hAm', ?_, hAdisj', ?_⟩
          · intro s hs
            have hsa : s ≠ (PubState.replay K l).actor := by
              rw [hva]
              exact hs
            have hcell := hAcell' s hs
            rwa [allowed_step_viewer v _ d hsa hva hd_hand,
              capacity_step_ne _ d hsa] at hcell
          · rwa [pool_step_viewer v _ d hva hd_hand] at hAcons'
        obtain ⟨ω', ⟨hω'hand, hω'legal⟩, hω'rem⟩ := ih hlegal_l A' hA
        have hC' := Coheres.replay ω' K hω'legal
        have hls : ((PlayState.init ω' K).replayFrom l).legalSet
            = ((PlayState.init ω K).replayFrom l).legalSet := by
          apply PlayState.legalSet_congr
          · rw [← hC'.leader, hCt.leader]
          · rw [← hC'.trick, hCt.trick]
          · rw [← hC'.contract, hCt.contract]
          · have h1 : ((PlayState.init ω' K).replayFrom l).hands
                ((PlayState.init ω' K).replayFrom l).actor
                = v.hand0 \ (PubState.replay K l).played v.viewer := by
              rw [← hC'.actor, hva, hC'.hands v.viewer, hω'hand]
            have h2 : ((PlayState.init ω K).replayFrom l).hands
                ((PlayState.init ω K).replayFrom l).actor
                = v.hand0 \ (PubState.replay K l).played v.viewer := by
              rw [← hCt.actor, hva, hCt.hands v.viewer, hv]
            rw [h1, h2]
        refine ⟨ω', ⟨hω'hand, ?_⟩, ?_⟩
        · rw [PlayState.legalFrom_append]
          refine ⟨hω'legal, ?_⟩
          rw [show ((PlayState.init ω' K).replayFrom l).LegalFrom [d]
              = (d ∈ ((PlayState.init ω' K).replayFrom l).legalSet ∧ True)
            from rfl, hls]
          exact ⟨hd_legal, trivial⟩
        · funext s
          unfold remainder
          split_ifs with h
          · rw [h]
            exact hAm'.symm
          · show ((PlayState.init ω' K).replayFrom (l ++ [d])).hands s = A' s
            rw [PlayState.replayFrom_append]
            have hsa : s ≠ ((PlayState.init ω' K).replayFrom l).actor := by
              rw [← hC'.actor, hva]
              exact h
            rw [show ((PlayState.init ω' K).replayFrom l).replayFrom [d]
                = ((PlayState.init ω' K).replayFrom l).step d from rfl,
              PlayState.hands_step_ne _ d hsa]
            have hr := congrFun hω'rem s
            unfold remainder at hr
            rwa [if_neg h] at hr
      · -- hidden action: remove/add the played tile (PA-C09)
        have hd_pool : d ∈ v.pool (PubState.replay K l) := by
          unfold pool
          rw [Finset.mem_sdiff, Finset.mem_union]
          refine ⟨Finset.mem_univ d, ?_⟩
          rintro (hc | hc)
          · unfold hand at hc
            rw [Finset.mem_sdiff, ← hv] at hc
            exact Finset.disjoint_left.mp
              (ω.disjoint _ v.viewer hva) hdP.1 hc.1
          · rw [Finset.mem_biUnion] at hc
            obtain ⟨t, -, hct⟩ := hc
            rcases eq_or_ne ((PubState.replay K l).actor) t with rfl | hta
            · exact hdP.2 hct
            · exact Finset.disjoint_left.mp (ω.disjoint _ t hta) hdP.1
                (hCt.played_sub t hct)
        have hd_allowed : d ∈ v.allowed (PubState.replay K l)
            ((PubState.replay K l).actor) := by
          unfold allowed
          rw [Finset.mem_sdiff]
          refine ⟨hd_pool, ?_⟩
          rw [Finset.mem_biUnion]
          rintro ⟨q, hq, hdq⟩
          rw [Declaration.mem_effSuit] at hdq
          exact hCt.voids_sound _ q hq d hdh2 (hCt.declaration ▸ hdq)
        have hd_nA' : d ∉ A' ((PubState.replay K l).actor) := by
          intro hc
          have hmem := (hAcell' _ hva).1 hc
          unfold allowed at hmem
          rw [Finset.mem_sdiff, pool_step_hidden v _ d hva,
            Finset.mem_erase] at hmem
          exact hmem.1.1 rfl
        have hcap_lt : ((PubState.replay K l).played
            ((PubState.replay K l).actor)).card < 7 := by
          have hss := Finset.card_lt_card
            ((Finset.ssubset_iff_of_subset (hCt.played_sub _)).mpr
              ⟨d, hdP.1, hdP.2⟩)
          rwa [ω.card_hands] at hss
        have hA : v.IsWorld (PubState.replay K l)
            (Function.update A' ((PubState.replay K l).actor)
              (insert d (A' ((PubState.replay K l).actor)))) := by
          refine ⟨?_, ?_, ?_, ?_⟩
          · rw [Function.update_of_ne (Ne.symm hva)]
            exact hAm'
          · intro s hs
            rcases eq_or_ne s ((PubState.replay K l).actor) with rfl | hsu
            · rw [Function.update_self]
              constructor
              · intro x hx
                rw [Finset.mem_insert] at hx
                rcases hx with rfl | hx
                · exact hd_allowed
                · exact allowed_step_actor_subset v _ d
                    ((hAcell' _ hs).1 hx)
              · rw [Finset.card_insert_of_notMem hd_nA',
                  (hAcell' _ hs).2, capacity_step_actor _ d hdP.2]
                have : 1 ≤ capacity (PubState.replay K l)
                    ((PubState.replay K l).actor) := by
                  unfold capacity
                  omega
                omega
            · rw [Function.update_of_ne hsu]
              have hcell := hAcell' s hs
              rw [allowed_step_ne v _ d hsu hva,
                capacity_step_ne _ d hsu] at hcell
              exact ⟨fun x hx => (Finset.mem_erase.mp (hcell.1 hx)).2,
                hcell.2⟩
          · intro s t hst
            rcases eq_or_ne s ((PubState.replay K l).actor) with rfl | hsu <;>
              rcases eq_or_ne t ((PubState.replay K l).actor) with rfl | htu
            · exact absurd rfl hst
            · rw [Function.update_self, Function.update_of_ne htu]
              rcases eq_or_ne t v.viewer with rfl | htv
              · rw [hAm']
                exact Finset.disjoint_empty_right _
              · rw [Finset.disjoint_left]
                intro x hx hxt
                rw [Finset.mem_insert] at hx
                rcases hx with rfl | hx
                · have hmem := (hAcell' t htv).1 hxt
                  unfold allowed at hmem
                  rw [Finset.mem_sdiff, pool_step_hidden v _ x hva,
                    Finset.mem_erase] at hmem
                  exact hmem.1.1 rfl
                · exact Finset.disjoint_left.mp (hAdisj' _ t hst) hx hxt
            · rw [Function.update_of_ne hsu, Function.update_self,
                Finset.disjoint_right]
              intro x hx hxs
              rw [Finset.mem_insert] at hx
              rcases hx with rfl | hx
              · rcases eq_or_ne s v.viewer with rfl | hsv
                · rw [hAm'] at hxs
                  exact absurd hxs (Finset.notMem_empty x)
                · have hmem := (hAcell' s hsv).1 hxs
                  unfold allowed at hmem
                  rw [Finset.mem_sdiff, pool_step_hidden v _ x hva,
                    Finset.mem_erase] at hmem
                  exact hmem.1.1 rfl
              · exact Finset.disjoint_left.mp (hAdisj' _ s hst.symm) hx hxs
            · rw [Function.update_of_ne hsu, Function.update_of_ne htu]
              exact hAdisj' s t hst
          · rw [biUnion_update_insert, hAcons',
              pool_step_hidden v _ d hva, Finset.insert_erase hd_pool]
        obtain ⟨ω', ⟨hω'hand, hω'legal⟩, hω'rem⟩ := ih hlegal_l _ hA
        have hC' := Coheres.replay ω' K hω'legal
        have hxa : ((PlayState.init ω' K).replayFrom l).actor
            = (PubState.replay K l).actor := hC'.actor.symm
        have hrem_u : ((PlayState.init ω' K).replayFrom l).hands
            ((PubState.replay K l).actor)
            = insert d (A' ((PubState.replay K l).actor)) := by
          have hr := congrFun hω'rem ((PubState.replay K l).actor)
          unfold remainder at hr
          rw [if_neg hva] at hr
          rw [hr, Function.update_self]
        have hd_legal' : d ∈ ((PlayState.init ω' K).replayFrom l).legalSet := by
          rcases htr : (PubState.replay K l).trick.head? with - | ⟨s0, d0⟩
          · have htr' : ((PlayState.init ω' K).replayFrom l).trick = [] := by
              rw [← hC'.trick]
              exact List.head?_eq_none_iff.mp htr
            rw [PlayState.legalSet_lead _ htr', hxa, hrem_u]
            exact Finset.mem_insert_self ..
          · have htr' : ((PlayState.init ω' K).replayFrom l).trick.head?
                = some (s0, d0) := by
              rw [← hC'.trick]
              exact htr
            by_cases hf : (PubState.replay K l).declaration.effMem d
                ((PubState.replay K l).declaration.ledSuit d0)
            · have hdf : d ∈ ((PlayState.init ω' K).replayFrom l).followSet
                  d0 := by
                unfold PlayState.followSet
                rw [Finset.mem_filter, hxa, hrem_u]
                refine ⟨Finset.mem_insert_self .., ?_⟩
                rw [← hC'.declaration]
                exact hf
              rw [PlayState.legalSet_follow _ htr' ⟨d, hdf⟩]
              exact hdf
            · have hq' : (PubState.replay K l).declaration.ledSuit d0
                  ∈ ((PubState.replay K l).step d).voids
                    ((PubState.replay K l).actor) := by
                rw [PubState.step_voids,
                  PubState.voidsAfter_slough _ d htr hf,
                  Function.update_self]
                exact Finset.mem_insert_self ..
              have hemp : ¬ (((PlayState.init ω' K).replayFrom l).followSet
                  d0).Nonempty := by
                rintro ⟨e, he⟩
                unfold PlayState.followSet at he
                rw [Finset.mem_filter, hxa, hrem_u,
                  Finset.mem_insert] at he
                obtain ⟨he1, he2⟩ := he
                rw [← hC'.declaration] at he2
                rcases he1 with rfl | he1
                · exact hf he2
                · have hmem := (hAcell' _ hva).1 he1
                  unfold allowed at hmem
                  rw [Finset.mem_sdiff, Finset.mem_biUnion] at hmem
                  apply hmem.2
                  refine ⟨_, hq', ?_⟩
                  rw [Declaration.mem_effSuit, PubState.step_declaration]
                  exact he2
              rw [PlayState.legalSet_slough _ htr' hemp, hxa, hrem_u]
              exact Finset.mem_insert_self ..
        refine ⟨ω', ⟨hω'hand, ?_⟩, ?_⟩
        · rw [PlayState.legalFrom_append]
          exact ⟨hω'legal, ⟨hd_legal', trivial⟩⟩
        · funext s
          unfold remainder
          split_ifs with h
          · rw [h]
            exact hAm'.symm
          · show ((PlayState.init ω' K).replayFrom (l ++ [d])).hands s = A' s
            rw [PlayState.replayFrom_append,
              show ((PlayState.init ω' K).replayFrom l).replayFrom [d]
                = ((PlayState.init ω' K).replayFrom l).step d from rfl]
            rcases eq_or_ne s ((PubState.replay K l).actor) with rfl | hsu
            · have h1 : (((PlayState.init ω' K).replayFrom l).step d).hands
                  ((PubState.replay K l).actor)
                  = (((PlayState.init ω' K).replayFrom l).hands
                      ((PubState.replay K l).actor)).erase d := by
                have h2 := PlayState.hands_step_actor
                  ((PlayState.init ω' K).replayFrom l) d
                rwa [hxa] at h2
              rw [h1, hrem_u, Finset.erase_insert hd_nA']
            · rw [PlayState.hands_step_ne _ d (by rw [hxa]; exact hsu)]
              have hr := congrFun hω'rem s
              unfold remainder at hr
              rw [if_neg h, Function.update_of_ne hsu] at hr
              exact hr

/-- PA-C07 (exact Straight 42 cell support, Math §7.5): for every legal
public play prefix, the derived cell fiber equals the remainder image of
the rule-compatible complete deals — **the losslessness theorem**. -/
theorem losslessness (ω : Deal) (K : Contract) (v : ViewerCtx)
    (hv : ω.hands v.viewer = v.hand0) {ds : List Domino}
    (hds : (PlayState.init ω K).LegalFrom ds) (A : Seat → Finset Domino) :
    v.IsWorld (PubState.replay K ds) A ↔
      ∃ ω' : Deal, v.Compatible K ds ω' ∧ v.remainder K ds ω' = A := by
  constructor
  · exact exists_deal_of_isWorld ω K v hv ds hds A
  · rintro ⟨ω', hω', rfl⟩
    exact isWorld_remainder v K ds hω'

/-- PA-C09 corollary (fixed-history bijection, Math §7.5): the remainder
map is injective on compatible deals — public attribution reconstructs
each seat's initial hand, hence the complete deal, from its current
remainder. With `losslessness` (surjectivity onto the fiber), the
remainder map is a bijection between compatible deals and the cell fiber. -/
theorem remainder_injective (v : ViewerCtx) (K : Contract)
    (ds : List Domino) {ω₁ ω₂ : Deal} (h₁ : v.Compatible K ds ω₁)
    (h₂ : v.Compatible K ds ω₂)
    (h : v.remainder K ds ω₁ = v.remainder K ds ω₂) : ω₁ = ω₂ := by
  have hC₁ := Coheres.replay ω₁ K h₁.2
  have hC₂ := Coheres.replay ω₂ K h₂.2
  apply Deal.ext
  funext s
  rcases eq_or_ne s v.viewer with rfl | hs
  · rw [h₁.1, h₂.1]
  · have e := congrFun h s
    unfold remainder at e
    rw [if_neg hs, if_neg hs, hC₁.hands s, hC₂.hands s] at e
    calc ω₁.hands s
        = ω₁.hands s \ (PubState.replay K ds).played s
            ∪ (PubState.replay K ds).played s :=
          (Finset.sdiff_union_of_subset (hC₁.played_sub s)).symm
      _ = ω₂.hands s \ (PubState.replay K ds).played s
            ∪ (PubState.replay K ds).played s := by rw [e]
      _ = ω₂.hands s := Finset.sdiff_union_of_subset (hC₂.played_sub s)

end ViewerCtx

end Texas42
