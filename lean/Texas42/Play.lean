/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Trick
import Texas42.Auction
import Texas42.Deal

/-!
# Texas 42 — contract and the objective contracted-play game

Layer B core (Math §§4.5, 5.1–5.6; ledger rows PA-B06–PA-B10): the
contract record, the reduced contracted-play state `X_t`, the exact legal
set (lead-anything / follow-if-possible), the atomic play transition with
trick resolution, the partition/location invariants it preserves, and the
conservation theorems (γ-descent, seven tricks, 28 plays, 42 points).

Design (Handoff §Layer B): the state is the spec's reduced `X_t` — hands,
leader, current-trick prefix, banked partnership scores, contract. Trick
count and scored tiles are *derived*, not stored (TYPE-02 discipline:
derived views, not fields). Invariants live in a separate `Inv` predicate
proved preserved (PA-B09); reachability subtyping comes later (PA-D09).
-/

namespace Texas42

/-- The opposite partnership. -/
def Team.other (t : Team) : Team := ⟨1 - t.val, by omega⟩

/-- PA-B06: the two contract kinds — point bids `P(n)` and mark bids
`M(m)` (Math §4.5). -/
inductive BidKind where
  | point
  | mark
deriving DecidableEq, Repr

/-- PA-B06: a contracted hand
`K = (bidder, kind, amount, declaration)`; threshold and stake are derived
(Math §4.5). -/
structure Contract where
  bidder : Seat
  kind : BidKind
  amount : ℕ
  declaration : Declaration
deriving DecidableEq

namespace Contract

/-- Declaring-team point threshold: `n` for `P(n)`, `42` for marks. -/
def threshold (K : Contract) : ℕ :=
  match K.kind with
  | .point => K.amount
  | .mark => 42

/-- Mark stake: `1` for point contracts, `m` for `M(m)`. -/
def stake (K : Contract) : ℕ :=
  match K.kind with
  | .point => 1
  | .mark => K.amount

/-- `make(K, P_D)`: the declaring team reached its threshold. -/
def Makes (K : Contract) (declPoints : ℕ) : Prop :=
  K.threshold ≤ declPoints

instance (K : Contract) (p : ℕ) : Decidable (K.Makes p) :=
  inferInstanceAs (Decidable (_ ≤ _))

/-- PA-B06: the receiving partnership and award amount as a deterministic
function of `(K, P_D)` (Math §4.5, theorem). -/
def award (K : Contract) (declPoints : ℕ) : Team × ℕ :=
  (if K.Makes declPoints then K.bidder.team else K.bidder.team.other, K.stake)

/-- Form the contract from a winning auction bid; `none` on `pass`
(which never wins a legal auction). -/
def ofBid (bidder : Seat) (b : Bid) (δ : Declaration) : Option Contract :=
  match b with
  | .pass => none
  | .point n => some ⟨bidder, .point, n, δ⟩
  | .mark m => some ⟨bidder, .mark, m, δ⟩

end Contract

/-- The seat acting `i` plays after leader `L` (clockwise). -/
def Seat.after (L : Seat) (i : ℕ) : Seat :=
  L + ⟨i % 4, Nat.mod_lt _ (by norm_num)⟩

/-- PA-B07: the reduced contracted-play state `X_t` (Math §5.2) — remaining
hands, current trick leader, current-trick prefix (with actors), banked
partnership scores, and the contract. -/
structure PlayState where
  hands : Seat → Finset Domino
  leader : Seat
  trick : List (Seat × Domino)
  score : Team → ℕ
  contract : Contract

namespace PlayState

/-- The declaration in force. -/
def declaration (X : PlayState) : Declaration := X.contract.declaration

/-- The acting seat `a(X) = L + |C| (mod 4)` (Math §5.2). -/
def actor (X : PlayState) : Seat := X.leader.after X.trick.length

/-- Remaining-tile grade `γ(X) = Σ_s |H_s|` (Math §5.6). -/
def gamma (X : PlayState) : ℕ := ∑ s : Seat, (X.hands s).card

/-- Tiles in the current trick. -/
def trickTiles (X : PlayState) : List Domino := X.trick.map Prod.snd

/-- Completed-trick count, derived: `(28 - γ - |C|) / 4` (Math §5.2's
phase residue; a field nowhere — TYPE-02). -/
def tricksDone (X : PlayState) : ℕ := (28 - X.gamma - X.trick.length) / 4

/-- Tiles already played and scored: everything outside hands and the
current trick. -/
def scoredTiles (X : PlayState) : Finset Domino :=
  (Finset.univ \ Finset.univ.biUnion X.hands) \ X.trickTiles.toFinset

/-- The actor's holdings that follow the led context of lead `d0`. -/
def followSet (X : PlayState) (d0 : Domino) : Finset Domino :=
  (X.hands X.actor).filter fun d =>
    X.declaration.effMem d (X.declaration.ledSuit d0)

/-- PA-B08: the exact legal set `A(X)` (Math §5.3) — lead-anything,
follow-if-possible, slough-if-void. -/
def legalSet (X : PlayState) : Finset Domino :=
  match X.trick.head? with
  | none => X.hands X.actor
  | some (_, d0) =>
      if (X.followSet d0).Nonempty then X.followSet d0 else X.hands X.actor

/-- Base point plus count points of a completed trick:
`g(C) = 1 + Σ c(e)` (Math §5.4). -/
def trickPoints (plays : List (Seat × Domino)) : ℕ :=
  1 + (plays.map fun p => p.2.countPoints).sum

/-- The trick winner: the play with the strictly maximal trick key in the
lead's context (earlier play retained on non-strict comparison; keys in
the winning tier are injective by PA-A10, so no tie is possible there). -/
def trickWinner (δ : Declaration) : List (Seat × Domino) → Seat
  | [] => 0
  | (s0, d0) :: rest =>
      (((s0, d0) :: rest).foldl
        (fun acc p =>
          if δ.key (δ.ledSuit d0) acc.2 < δ.key (δ.ledSuit d0) p.2 then p
          else acc)
        (s0, d0)).1

/-- The updated hands after the actor plays `d`. -/
def handsAfter (X : PlayState) (d : Domino) : Seat → Finset Domino :=
  Function.update X.hands X.actor ((X.hands X.actor).erase d)

/-- PA-B09: the atomic play transition (Math §5.4). Remove the tile from
the actor's hand and append it to the trick; a fourth play (the trick
prefix already holds three) resolves the trick — banking `g(C)` to the
winner's partnership, seating the winner as next leader, and clearing the
trick. -/
def step (X : PlayState) (d : Domino) : PlayState :=
  if X.trick.length = 3 then
    { hands := X.handsAfter d
      leader := trickWinner X.declaration (X.trick ++ [(X.actor, d)])
      trick := []
      score := fun t =>
        if t = (trickWinner X.declaration (X.trick ++ [(X.actor, d)])).team
        then X.score t + trickPoints (X.trick ++ [(X.actor, d)])
        else X.score t
      contract := X.contract }
  else
    { X with hands := X.handsAfter d, trick := X.trick ++ [(X.actor, d)] }

/-- The hand is over: no tiles remain and the last trick has resolved. -/
def Terminal (X : PlayState) : Prop := X.gamma = 0 ∧ X.trick = []

/-- The contracted hand's initial state: the dealt hands, the bidder
leading, empty trick, zero scores (Math §5.2). -/
def init (ω : Deal) (K : Contract) : PlayState :=
  ⟨ω.hands, K.bidder, [], fun _ => 0, K⟩

/-! ## PA-B08: legal-set characterization (Math §5.3) -/

/-- Legal plays come from the actor's hand. -/
theorem legalSet_subset (X : PlayState) : X.legalSet ⊆ X.hands X.actor := by
  unfold legalSet
  split
  · exact Finset.Subset.refl _
  · split
    · exact Finset.filter_subset _ _
    · exact Finset.Subset.refl _

/-- On lead, everything held is legal (Math §5.3 case 1). -/
theorem legalSet_lead (X : PlayState) (h : X.trick = []) :
    X.legalSet = X.hands X.actor := by
  unfold legalSet
  rw [h]
  rfl

/-- After a lead, holding a member of the led effective suit forces a
follow (Math §5.3 case 2). -/
theorem legalSet_follow (X : PlayState) {s0 : Seat} {d0 : Domino}
    (h : X.trick.head? = some (s0, d0)) (hne : (X.followSet d0).Nonempty) :
    X.legalSet = X.followSet d0 := by
  unfold legalSet
  rw [h]
  dsimp only
  rw [if_pos hne]

/-- After a lead, a void in the led effective suit frees the whole hand
(Math §5.3 case 3). -/
theorem legalSet_slough (X : PlayState) {s0 : Seat} {d0 : Domino}
    (h : X.trick.head? = some (s0, d0)) (he : ¬ (X.followSet d0).Nonempty) :
    X.legalSet = X.hands X.actor := by
  unfold legalSet
  rw [h]
  dsimp only
  rw [if_neg he]

/-- The legal set is nonempty whenever the actor still holds tiles —
no legal state is stuck. -/
theorem legalSet_nonempty (X : PlayState)
    (h : (X.hands X.actor).Nonempty) : X.legalSet.Nonempty := by
  unfold legalSet
  split
  · exact h
  · split
    · assumption
    · exact h

/-! ## Step projections -/

theorem step_hands (X : PlayState) (d : Domino) :
    (X.step d).hands = X.handsAfter d := by
  unfold step
  split <;> rfl

theorem step_contract (X : PlayState) (d : Domino) :
    (X.step d).contract = X.contract := by
  unfold step
  split <;> rfl

theorem step_trick_complete (X : PlayState) (d : Domino)
    (h : X.trick.length = 3) : (X.step d).trick = [] := by
  unfold step
  rw [if_pos h]

theorem step_trick_incomplete (X : PlayState) (d : Domino)
    (h : X.trick.length ≠ 3) :
    (X.step d).trick = X.trick ++ [(X.actor, d)] := by
  unfold step
  rw [if_neg h]

theorem step_score_complete (X : PlayState) (d : Domino)
    (h : X.trick.length = 3) :
    (X.step d).score = fun t =>
      if t = (trickWinner X.declaration (X.trick ++ [(X.actor, d)])).team
      then X.score t + trickPoints (X.trick ++ [(X.actor, d)])
      else X.score t := by
  unfold step
  rw [if_pos h]

theorem step_score_incomplete (X : PlayState) (d : Domino)
    (h : X.trick.length ≠ 3) : (X.step d).score = X.score := by
  unfold step
  rw [if_neg h]

/-! ## PA-B09: partition/location invariants and their preservation -/

/-- The contracted-play invariant (Math §§5.1, 5.6): hands pairwise
disjoint; trick tiles held by no one and mutually distinct; at most three
tiles in the current-trick prefix; trick actors are the clockwise
successors of the leader; and the progress/score equation — `γ`, the trick
prefix, and four tiles per completed trick account for all 28 tiles, while
the banked scores equal completed tricks plus the count points of scored
tiles. -/
structure Inv (X : PlayState) : Prop where
  disj : ∀ s t : Seat, s ≠ t → Disjoint (X.hands s) (X.hands t)
  trick_not_held : ∀ p ∈ X.trick, ∀ s : Seat, p.2 ∉ X.hands s
  trick_nodup : X.trickTiles.Nodup
  trick_len : X.trick.length ≤ 3
  trick_seats : X.trick.map Prod.fst =
    (List.range X.trick.length).map X.leader.after
  progress : ∃ k : ℕ, X.gamma + X.trick.length + 4 * k = 28 ∧
    X.score 0 + X.score 1 = k + ∑ d ∈ X.scoredTiles, d.countPoints

/-- Membership in the scored tiles, unfolded. -/
theorem mem_scoredTiles (X : PlayState) (x : Domino) :
    x ∈ X.scoredTiles ↔ (∀ s : Seat, x ∉ X.hands s) ∧ x ∉ X.trickTiles := by
  simp only [scoredTiles, Finset.mem_sdiff, Finset.mem_univ, true_and,
    Finset.mem_biUnion, List.mem_toFinset, not_exists]

/-- Hand membership after the play: erased for the played tile, otherwise
unchanged. -/
theorem mem_handsAfter (X : PlayState) (d x : Domino) (s : Seat) :
    x ∈ X.handsAfter d s ↔ x ∈ X.hands s ∧ ¬(x = d ∧ s = X.actor) := by
  unfold handsAfter
  rcases eq_or_ne s X.actor with rfl | hs
  · rw [Function.update_self, Finset.mem_erase]
    tauto
  · rw [Function.update_of_ne hs]
    tauto

/-- `γ` drops by exactly one when the actor releases a held tile. -/
theorem gamma_handsAfter (X : PlayState) {d : Domino}
    (hd : d ∈ X.hands X.actor) :
    ∑ s : Seat, ((X.handsAfter d) s).card = X.gamma - 1 := by
  have hcongr : ∀ s ∈ Finset.univ.erase X.actor,
      ((X.handsAfter d) s).card = (X.hands s).card := by
    intro s hs
    unfold handsAfter
    rw [Function.update_of_ne (Finset.mem_erase.mp hs).1]
  have hL := Finset.sum_erase_add Finset.univ
    (fun s => ((X.handsAfter d) s).card) (Finset.mem_univ X.actor)
  have hR := Finset.sum_erase_add Finset.univ
    (fun s => (X.hands s).card) (Finset.mem_univ X.actor)
  have hu : ((X.handsAfter d) X.actor).card = (X.hands X.actor).card - 1 := by
    unfold handsAfter
    rw [Function.update_self, Finset.card_erase_of_mem hd]
  have hsum : ∑ s ∈ Finset.univ.erase X.actor, ((X.handsAfter d) s).card
      = ∑ s ∈ Finset.univ.erase X.actor, (X.hands s).card :=
    Finset.sum_congr rfl hcongr
  have h1 : 1 ≤ (X.hands X.actor).card := Finset.card_pos.mpr ⟨d, hd⟩
  unfold gamma
  omega

/-- One remaining tile keeps `γ` positive. -/
theorem gamma_pos (X : PlayState) {d : Domino} (hd : d ∈ X.hands X.actor) :
    1 ≤ X.gamma := by
  have h1 : 1 ≤ (X.hands X.actor).card := Finset.card_pos.mpr ⟨d, hd⟩
  have := Finset.single_le_sum (f := fun s => (X.hands s).card)
    (fun i _ => Nat.zero_le _) (Finset.mem_univ X.actor)
  unfold gamma
  omega

/-- Each legal play removes exactly one tile: `γ` descends by one
(Math §5.6 item 2; with `γ(init) = 28` this is "exactly 28 plays"). -/
theorem gamma_step {X : PlayState} {d : Domino} (hd : d ∈ X.legalSet) :
    (X.step d).gamma = X.gamma - 1 := by
  have hdh : d ∈ X.hands X.actor := X.legalSet_subset hd
  change ∑ s : Seat, ((X.step d).hands s).card = X.gamma - 1
  rw [step_hands]
  exact X.gamma_handsAfter hdh

private theorem team_split (wt : Team) (a b g : ℕ) :
    (if (0 : Team) = wt then a + g else a)
      + (if (1 : Team) = wt then b + g else b) = a + b + g := by
  have hv := wt.isLt
  have h0 : ((0 : Team) = wt) ↔ wt.val = 0 := by
    rw [eq_comm, Fin.ext_iff]
    simp
  have h1 : ((1 : Team) = wt) ↔ wt.val = 1 := by
    rw [eq_comm, Fin.ext_iff]
    simp
  simp only [h0, h1]
  split_ifs <;> omega

/-- PA-B09: the atomic play transition preserves the partition/location
invariants (Math §5.4/§5.6). -/
theorem inv_step {X : PlayState} (hX : X.Inv) {d : Domino}
    (hd : d ∈ X.legalSet) : (X.step d).Inv := by
  have hdh : d ∈ X.hands X.actor := X.legalSet_subset hd
  have hdisj' : ∀ s t : Seat, s ≠ t →
      Disjoint (X.handsAfter d s) (X.handsAfter d t) := by
    intro s t hst
    apply Finset.disjoint_left.mpr
    intro x hxs hxt
    rw [mem_handsAfter] at hxs hxt
    exact Finset.disjoint_left.mp (hX.disj s t hst) hxs.1 hxt.1
  have hnot' : ∀ x : Domino, (∀ s, x ∉ X.hands s) →
      ∀ s, x ∉ X.handsAfter d s := by
    intro x hx s hmem
    exact hx s ((X.mem_handsAfter d x s).mp hmem).1
  have hd_not' : ∀ s, d ∉ X.handsAfter d s := by
    intro s hmem
    rw [mem_handsAfter] at hmem
    rcases eq_or_ne s X.actor with rfl | hs
    · exact hmem.2 ⟨rfl, rfl⟩
    · exact Finset.disjoint_left.mp (hX.disj s X.actor hs) hmem.1 hdh
  have hd_tiles : d ∉ X.trickTiles := by
    intro hmem
    unfold trickTiles at hmem
    rw [List.mem_map] at hmem
    obtain ⟨p, hp, hpd⟩ := hmem
    exact hX.trick_not_held p hp X.actor (hpd ▸ hdh)
  rcases eq_or_ne X.trick.length 3 with h3 | h3
  · -- the trick completes
    have hset : (X.step d).scoredTiles
        = X.scoredTiles ∪ X.trickTiles.toFinset ∪ {d} := by
      ext x
      rw [mem_scoredTiles]
      unfold trickTiles
      rw [step_hands, step_trick_complete X d h3]
      simp only [List.map_nil, List.not_mem_nil, not_false_iff, and_true,
        Finset.mem_union, Finset.mem_singleton, List.mem_toFinset,
        mem_scoredTiles]
      constructor
      · intro hnh
        by_cases hxd : x = d
        · exact Or.inr hxd
        · by_cases hxt : x ∈ X.trickTiles
          · exact Or.inl (Or.inr hxt)
          · refine Or.inl (Or.inl ⟨fun s hxs => hnh s ?_, hxt⟩)
            rw [mem_handsAfter]
            exact ⟨hxs, fun hc => hxd hc.1⟩
      · rintro ((⟨hnh, -⟩ | hxt) | rfl)
        · exact hnot' x hnh
        · intro s hmem
          rw [mem_handsAfter] at hmem
          obtain ⟨p, hp, hpd⟩ := List.mem_map.mp hxt
          exact hX.trick_not_held p hp s (hpd ▸ hmem.1)
        · exact hd_not'
    constructor
    · rw [step_hands]; exact hdisj'
    · intro p hp s
      rw [step_trick_complete X d h3] at hp
      cases hp
    · unfold trickTiles
      rw [step_trick_complete X d h3]
      simp
    · rw [step_trick_complete X d h3]
      simp
    · rw [step_trick_complete X d h3]
      simp
    · obtain ⟨k, hcount, hscore⟩ := hX.progress
      have hγ := gamma_step hd
      have hγpos := X.gamma_pos hdh
      refine ⟨k + 1, ?_, ?_⟩
      · rw [hγ, step_trick_complete X d h3]
        simp only [List.length_nil]
        omega
      · rw [step_score_complete X d h3]
        simp only []
        rw [team_split, hset]
        have hdisj₁ : Disjoint (X.scoredTiles ∪ X.trickTiles.toFinset)
            ({d} : Finset Domino) := by
          simp only [Finset.disjoint_singleton_right, Finset.mem_union,
            List.mem_toFinset]
          rintro (hsc | ht)
          · exact ((X.mem_scoredTiles d).mp hsc).1 X.actor hdh
          · exact hd_tiles ht
        have hdisj₂ : Disjoint X.scoredTiles X.trickTiles.toFinset := by
          apply Finset.disjoint_left.mpr
          intro x hx ht
          exact ((X.mem_scoredTiles x).mp hx).2 (List.mem_toFinset.mp ht)
        rw [Finset.sum_union hdisj₁, Finset.sum_union hdisj₂,
          Finset.sum_singleton]
        have htsum : ∑ x ∈ X.trickTiles.toFinset, x.countPoints
            = (X.trickTiles.map Domino.countPoints).sum :=
          List.sum_toFinset _ hX.trick_nodup
        have hpts : trickPoints (X.trick ++ [(X.actor, d)]) =
            1 + (X.trickTiles.map Domino.countPoints).sum
              + d.countPoints := by
          unfold trickPoints trickTiles
          simp [List.map_append, Function.comp_def]
          omega
        rw [hpts, htsum]
        omega
  · -- the trick continues
    have hlen : X.trick.length + 1 ≤ 3 := by
      have := hX.trick_len
      omega
    have hset : (X.step d).scoredTiles = X.scoredTiles := by
      ext x
      rw [mem_scoredTiles, mem_scoredTiles]
      unfold trickTiles
      rw [step_hands, step_trick_incomplete X d h3]
      simp only [List.map_append, List.map_singleton, List.mem_append,
        List.mem_singleton]
      constructor
      · rintro ⟨hnh, hnt⟩
        have hxd : x ≠ d := fun hxd => hnt (Or.inr hxd)
        refine ⟨fun s hxs => hnh s ?_, fun hxt => hnt (Or.inl hxt)⟩
        rw [mem_handsAfter]
        exact ⟨hxs, fun hc => hxd hc.1⟩
      · rintro ⟨hnh, hnt⟩
        have hxd : x ≠ d := by
          rintro rfl
          exact hnh X.actor hdh
        refine ⟨fun s hmem => hnh s ((X.mem_handsAfter d x s).mp hmem).1, ?_⟩
        rintro (hxt | rfl)
        · exact hnt hxt
        · exact hxd rfl
    constructor
    · rw [step_hands]; exact hdisj'
    · intro p hp s
      rw [step_trick_incomplete X d h3] at hp
      rw [step_hands]
      simp only [List.mem_append, List.mem_singleton] at hp
      rcases hp with hp | rfl
      · exact hnot' p.2 (fun t => hX.trick_not_held p hp t) s
      · exact hd_not' s
    · unfold trickTiles
      rw [step_trick_incomplete X d h3]
      rw [List.map_append, List.map_singleton, List.nodup_append]
      refine ⟨hX.trick_nodup, List.nodup_singleton d, ?_⟩
      intro x hx b hb
      rw [List.mem_singleton] at hb
      subst hb
      rintro rfl
      exact hd_tiles hx
    · rw [step_trick_incomplete X d h3]
      simpa using hlen
    · rw [step_trick_incomplete X d h3]
      show (X.trick ++ [(X.actor, d)]).map Prod.fst
        = (List.range (X.trick ++ [(X.actor, d)]).length).map
            ((X.step d).leader).after
      have hleader : (X.step d).leader = X.leader := by
        unfold step
        rw [if_neg h3]
      rw [hleader]
      rw [List.map_append, List.map_singleton, List.length_append,
        List.length_singleton, List.range_succ, List.map_append,
        hX.trick_seats]
      rfl
    · obtain ⟨k, hcount, hscore⟩ := hX.progress
      have hγ := gamma_step hd
      have hγpos := X.gamma_pos hdh
      refine ⟨k, ?_, ?_⟩
      · rw [hγ, step_trick_incomplete X d h3]
        simp only [List.length_append, List.length_singleton]
        omega
      · rw [step_score_incomplete X d h3, hset]
        exact hscore

/-- The initial grade is 28 (Math §5.6 item 1). -/
theorem gamma_init (ω : Deal) (K : Contract) : (init ω K).gamma = 28 := by
  change (∑ s : Seat, (ω.hands s).card) = 28
  simp [ω.card_hands]

/-- The initial contracted state satisfies the invariant. -/
theorem inv_init (ω : Deal) (K : Contract) : (init ω K).Inv := by
  constructor
  · exact ω.disjoint
  · intro p hp
    cases hp
  · simp [trickTiles, init]
  · simp [init]
  · simp [init]
  · refine ⟨0, ?_, ?_⟩
    · have hγ := gamma_init ω K
      have hlen : (init ω K).trick.length = 0 := by simp [init]
      omega
    · have hempty : (init ω K).scoredTiles = ∅ := by
        ext x
        rw [mem_scoredTiles]
        simp only [Finset.notMem_empty, iff_false, not_and]
        intro hnh
        obtain ⟨s, hs⟩ := (ω.existsUnique_mem x).exists
        exact absurd hs (hnh s)
      rw [hempty]
      simp [init]

/-! ## PA-B10: conservation — seven tricks and 42 points -/

/-- PA-B10 (Math §5.6 items 4–5): at a terminal invariant state, exactly
seven tricks have completed and the banked partnership scores total
`35 + 7 = 42`. Together with `gamma_init` and `gamma_step` (28 plays),
this is the conservation package. -/
theorem terminal_scores {X : PlayState} (hX : X.Inv) (hT : X.Terminal) :
    X.tricksDone = 7 ∧ X.score 0 + X.score 1 = 42 := by
  obtain ⟨hγ, htrick⟩ := hT
  obtain ⟨k, hcount, hscore⟩ := hX.progress
  rw [hγ, htrick] at hcount
  simp only [List.length_nil] at hcount
  have hk : k = 7 := by omega
  have hall : X.scoredTiles = Finset.univ := by
    ext x
    rw [mem_scoredTiles]
    simp only [Finset.mem_univ, iff_true]
    refine ⟨fun s hs => ?_, ?_⟩
    · have h1 : 1 ≤ (X.hands s).card := Finset.card_pos.mpr ⟨x, hs⟩
      have := Finset.single_le_sum (f := fun s => (X.hands s).card)
        (fun i _ => Nat.zero_le _) (Finset.mem_univ s)
      unfold gamma at hγ
      omega
    · simp [trickTiles, htrick]
  refine ⟨?_, ?_⟩
  · unfold tricksDone
    rw [hγ, htrick]
    simp
  · rw [hscore, hall, hk, Domino.total_countPoints]

end PlayState

end Texas42
