/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Basic

/-!
# Texas 42 — declaration algebra and the unique trick winner

Layer A, second slice: the nine Straight 42 declarations, called/powered
sets, effective suits, led context, rank, tier, and the total trick key
(Math §3, ledger rows PA-A05–PA-A08), closing with

* PA-A09: a lead always occupies a nonzero tier;
* PA-A10: the trick key is injective among nonzero-tier dominoes;
* PA-A11: four distinct dominoes with a designated lead have a unique
  key-maximal winner.

Per the dependency spine (K3), the unique winner is derived from key
injectivity — a structural argument on the shared-pip arithmetic of
`Texas42.Domino.eq_of_hasPip_of_pipSum_eq` — not by enumerating the
737,100 finite trick cases (that exhaustive receipt is PA-A12, a separate
reflection target).
-/

namespace Texas42

/-- PA-A01/PA-A05: the nine Straight 42 declarations — seven pip trumps,
doubles-trump, and no-trump/follow-me (Math §3.1). -/
inductive Declaration where
  | pip (p : Pip)
  | doubles
  | notrump
deriving DecidableEq, Fintype

/-- Math §3.1: there are exactly nine Straight 42 declarations. -/
theorem card_declaration : Fintype.card Declaration = 9 := by
  decide

/-- PA-A01: a led context — one of the seven natural suits, or the called
suit (named `7` in Math §3.3). -/
inductive Suit where
  | natural (p : Pip)
  | called
deriving DecidableEq, Fintype

/-- Math §3.3: there are exactly eight led contexts. -/
theorem card_suit : Fintype.card Suit = 8 := by
  decide

namespace Declaration

open Domino

/-- PA-A05: the called set `κ_δ` — the pip-incidence set for a pip trump,
the doubles for doubles-trump, empty for no-trump (Math §3.2). -/
def called : Declaration → Domino → Prop
  | pip p, d => d.hasPip p
  | doubles, d => d.isDouble
  | notrump, _ => False

instance : ∀ (δ : Declaration) (d : Domino), Decidable (δ.called d)
  | pip _, d => inferInstanceAs (Decidable (d.hasPip _))
  | doubles, d => inferInstanceAs (Decidable d.isDouble)
  | notrump, _ => inferInstanceAs (Decidable False)

/-- PA-A05: the powered set `π_δ`. In Straight 42 every nonempty called set
is powered and `κ_NT = π_NT = ∅`, so the powered predicate coincides with
the called predicate (Math §3.2). -/
abbrev powered (δ : Declaration) (d : Domino) : Prop := δ.called d

/-- PA-A05: effective suit membership — the follow relation `F_δ(d, q)`.
A called domino is removed from all natural incidences and placed in the
called suit (Math §§3.2–3.4). -/
def effMem (δ : Declaration) (d : Domino) : Suit → Prop
  | .natural p => d.hasPip p ∧ ¬ δ.called d
  | .called => δ.called d

instance (δ : Declaration) (d : Domino) : ∀ q : Suit, Decidable (δ.effMem d q)
  | .natural _ => inferInstanceAs (Decidable (_ ∧ _))
  | .called => inferInstanceAs (Decidable (δ.called d))

/-- PA-A05: the led context `ℓ_δ(d)` — the called suit for a called lead,
otherwise the natural suit of the higher pip (Math §3.4). -/
def ledSuit (δ : Declaration) (d : Domino) : Suit :=
  if δ.called d then .called else .natural d.high

/-! ## PA-A06/PA-A07: effective membership bounds and follow exactness -/

/-- PA-A07: a domino belongs to its own led context — leads can always
follow themselves, so the effective family covers `𝒟` (PA-A06 clause 1). -/
theorem effMem_ledSuit (δ : Declaration) (d : Domino) :
    δ.effMem d (δ.ledSuit d) := by
  unfold ledSuit
  by_cases h : δ.called d
  · simpa [h] using show δ.effMem d .called from h
  · exact if_neg h ▸ ⟨Or.inl rfl, h⟩

/-- PA-A06 (called absorption): a called domino belongs to no natural
effective suit — only to the called suit. -/
theorem not_effMem_natural_of_called {δ : Declaration} {d : Domino}
    (h : δ.called d) (p : Pip) : ¬ δ.effMem d (.natural p) :=
  fun hm => hm.2 h

/-- PA-A06: an uncalled domino's natural effective memberships are exactly
its natural incidences. -/
theorem effMem_natural_iff {δ : Declaration} {d : Domino}
    (h : ¬ δ.called d) (p : Pip) :
    δ.effMem d (.natural p) ↔ d.hasPip p :=
  ⟨fun hm => hm.1, fun hp => ⟨hp, h⟩⟩

/-- PA-A06 (membership bounds): every domino belongs to exactly one
effective suit if called or a double, exactly two otherwise. -/
theorem card_effMem : ∀ (δ : Declaration) (d : Domino),
    (Finset.univ.filter fun q => δ.effMem d q).card =
      if δ.called d ∨ d.isDouble then 1 else 2 := by
  decide

/-! ## PA-A08: rank, tier, and the total trick key -/

/-- PA-A08: the total declaration-relative rank `r_δ` (Math §3.5). Under
doubles-trump a double ranks by its pip; under any other declaration a
double ranks `⊤`; a mixed domino always ranks by its pip sum. -/
def rank (δ : Declaration) (d : Domino) : WithTop ℕ :=
  if d.isDouble then
    match δ with
    | doubles => (d.high.val : WithTop ℕ)
    | _ => ⊤
  else (d.pipSum : WithTop ℕ)

/-- PA-A08: the trick tier under led context `q` — powered dominoes occupy
tier two, unpowered followers of `q` tier one, everything else tier zero
(Math §3.5). -/
def tier (δ : Declaration) (q : Suit) (d : Domino) : ℕ :=
  if δ.powered d then 2
  else if δ.effMem d q then 1
  else 0

/-- PA-A08: the total trick key `τ_δ(d, q)`, compared lexicographically;
tier-zero dominoes are intentionally tied at the bottom (Math §3.5). -/
def key (δ : Declaration) (q : Suit) (d : Domino) : Lex (ℕ × WithTop ℕ) :=
  toLex (δ.tier q d, if δ.tier q d = 0 then 0 else δ.rank d)

/-! ## PA-A09: a lead always occupies a nonzero tier -/

/-- PA-A09: a lead has tier one or two in its own led context — a called
lead is powered, an uncalled lead follows its higher pip's natural suit
(Math §3.6, lemma). -/
theorem tier_ledSuit_pos (δ : Declaration) (d : Domino) :
    0 < δ.tier (δ.ledSuit d) d := by
  unfold tier
  by_cases h : δ.powered d
  · simp [h]
  · simp [h, effMem_ledSuit δ d]

/-! ## PA-A10: rank injectivity inside a winning tier -/

/-- A mixed domino ranks by its pip sum under every declaration. -/
theorem rank_of_not_isDouble {δ : Declaration} {d : Domino}
    (hd : ¬ d.isDouble) : δ.rank d = (d.pipSum : WithTop ℕ) :=
  if_neg hd

/-- Away from doubles-trump, a double ranks `⊤`. -/
theorem rank_of_isDouble_of_ne_doubles {δ : Declaration}
    (hδ : δ ≠ doubles) {d : Domino} (hd : d.isDouble) :
    δ.rank d = ⊤ := by
  unfold rank
  rw [if_pos hd]
  cases δ with
  | pip p => rfl
  | doubles => exact absurd rfl hδ
  | notrump => rfl

/-- Under doubles-trump, a double ranks by its pip. -/
theorem rank_doubles_of_isDouble {d : Domino} (hd : d.isDouble) :
    Declaration.doubles.rank d = (d.high.val : WithTop ℕ) := by
  unfold rank
  rw [if_pos hd]

/-- Rank injectivity on a shared pip incidence, away from doubles-trump:
doubles rank `⊤` and the only double on pip `p` is `p:p`; mixed dominoes
rank by distinct sums `p + k` (Math §3.6). -/
theorem eq_of_hasPip_of_rank_eq {δ : Declaration} (hδ : δ ≠ doubles)
    {p : Pip} {d₁ d₂ : Domino} (h₁ : d₁.hasPip p) (h₂ : d₂.hasPip p)
    (hr : δ.rank d₁ = δ.rank d₂) : d₁ = d₂ := by
  by_cases hd₁ : d₁.isDouble <;> by_cases hd₂ : d₂.isDouble
  · rw [eq_double_of_hasPip hd₁ h₁, eq_double_of_hasPip hd₂ h₂]
  · rw [rank_of_isDouble_of_ne_doubles hδ hd₁, rank_of_not_isDouble hd₂] at hr
    exact absurd hr.symm (WithTop.coe_ne_top)
  · rw [rank_of_not_isDouble hd₁, rank_of_isDouble_of_ne_doubles hδ hd₂] at hr
    exact absurd hr (WithTop.coe_ne_top)
  · rw [rank_of_not_isDouble hd₁, rank_of_not_isDouble hd₂] at hr
    exact eq_of_hasPip_of_pipSum_eq h₁ h₂ (by exact_mod_cast hr)

/-- PA-A10 (tier two): rank is injective among powered dominoes — under a
pip trump the trump double ranks `⊤` and mixed trumps have distinct sums;
under doubles-trump the seven doubles have distinct pip ranks (Math §3.6). -/
theorem eq_of_powered_of_rank_eq {δ : Declaration} {d₁ d₂ : Domino}
    (h₁ : δ.powered d₁) (h₂ : δ.powered d₂)
    (hr : δ.rank d₁ = δ.rank d₂) : d₁ = d₂ := by
  cases δ with
  | pip p => exact eq_of_hasPip_of_rank_eq (by simp) h₁ h₂ hr
  | doubles =>
      rw [rank_doubles_of_isDouble h₁, rank_doubles_of_isDouble h₂] at hr
      have hhigh : d₁.high = d₂.high := by
        have := WithTop.coe_inj.mp hr
        exact Fin.ext (by exact_mod_cast this)
      obtain ⟨a, b, hab⟩ := d₁
      obtain ⟨c, e, hce⟩ := d₂
      simp only [called, Domino.isDouble] at h₁ h₂
      simp only [Domino.mk.injEq]
      simp_all
  | notrump => exact absurd h₁ not_false

/-- PA-A10 (tier one): rank is injective among unpowered followers of a
led context. The called context has no unpowered followers; a natural
context's followers share its pip (Math §3.6). -/
theorem eq_of_effMem_of_rank_eq {δ : Declaration} {q : Suit} {d₁ d₂ : Domino}
    (hp₁ : ¬ δ.powered d₁) (h₁ : δ.effMem d₁ q) (h₂ : δ.effMem d₂ q)
    (hr : δ.rank d₁ = δ.rank d₂) : d₁ = d₂ := by
  cases q with
  | called => exact absurd h₁ hp₁
  | natural p =>
      cases δ with
      | doubles =>
          rw [rank_of_not_isDouble h₁.2, rank_of_not_isDouble h₂.2] at hr
          exact eq_of_hasPip_of_pipSum_eq h₁.1 h₂.1 (by exact_mod_cast hr)
      | pip p' => exact eq_of_hasPip_of_rank_eq (by simp) h₁.1 h₂.1 hr
      | notrump => exact eq_of_hasPip_of_rank_eq (by simp) h₁.1 h₂.1 hr

/-- PA-A10: the trick key is injective among dominoes occupying a nonzero
tier under a common led context. -/
theorem eq_of_key_eq {δ : Declaration} {q : Suit} {d₁ d₂ : Domino}
    (h₁ : δ.tier q d₁ ≠ 0) (hk : δ.key q d₁ = δ.key q d₂) : d₁ = d₂ := by
  have hpair : (δ.tier q d₁, if δ.tier q d₁ = 0 then (0 : WithTop ℕ) else δ.rank d₁)
      = (δ.tier q d₂, if δ.tier q d₂ = 0 then (0 : WithTop ℕ) else δ.rank d₂) :=
    congrArg ofLex hk
  have ht : δ.tier q d₁ = δ.tier q d₂ := congrArg Prod.fst hpair
  have h₂ : δ.tier q d₂ ≠ 0 := ht ▸ h₁
  have hr : δ.rank d₁ = δ.rank d₂ := by
    have := congrArg Prod.snd hpair
    rwa [if_neg h₁, if_neg h₂] at this
  by_cases hp₁ : δ.powered d₁ <;> by_cases hp₂ : δ.powered d₂
  · exact eq_of_powered_of_rank_eq hp₁ hp₂ hr
  · exfalso
    unfold tier at ht
    rw [if_pos hp₁, if_neg hp₂] at ht
    split_ifs at ht; omega
  · exfalso
    unfold tier at ht
    rw [if_neg hp₁, if_pos hp₂] at ht
    split_ifs at ht; omega
  · have hm₁ : δ.effMem d₁ q := by
      by_contra hm
      exact h₁ (by unfold tier; rw [if_neg hp₁, if_neg hm])
    have hm₂ : δ.effMem d₂ q := by
      by_contra hm
      exact h₂ (by unfold tier; rw [if_neg hp₂, if_neg hm])
    exact eq_of_effMem_of_rank_eq hp₁ hm₁ hm₂ hr

/-! ## PA-A13/PA-A14: contextual BEATS, threat, and monotone removal -/

/-- PA-A13: `e` beats `d` under declaration `δ` and led context `q` —
membership in `BEATS_δ(q, d)` (Math §3.7). -/
def Beats (δ : Declaration) (q : Suit) (d e : Domino) : Prop :=
  δ.key q d < δ.key q e

instance (δ : Declaration) (q : Suit) (d e : Domino) :
    Decidable (δ.Beats q d e) :=
  inferInstanceAs (Decidable (_ < _))

/-- PA-A13 (contextual `BEATS` exactness): if `d` is the current winner of a
trick led in context `q` — its key is maximal among the plays seen — then a
later play `e` becomes current winner exactly when `e ∈ BEATS_δ(q, d)`
(Math §3.7). -/
theorem beats_exact {δ : Declaration} {q : Suit} {seen : Finset Domino}
    {d e : Domino} (hd : d ∈ seen)
    (hmax : ∀ f ∈ seen, δ.key q f ≤ δ.key q d) :
    (∀ f ∈ seen, δ.key q f < δ.key q e) ↔ δ.Beats q d e :=
  ⟨fun h => h d hd, fun h f hf => lt_of_le_of_lt (hmax f hf) h⟩

/-- The `BEATS_δ(q, d)` set as a finset. -/
def beatsSet (δ : Declaration) (q : Suit) (d : Domino) : Finset Domino :=
  Finset.univ.filter (fun e => δ.Beats q d e)

/-- PA-A13: the when-led threat set
`THREAT_δ(d) = BEATS_δ(ℓ_δ(d), d)` (Math §3.7). -/
def threat (δ : Declaration) (d : Domino) : Finset Domino :=
  δ.beatsSet (δ.ledSuit d) d

/-- PA-A14 (monotone threat removal): the live threat query
`R_δ(d; O) = THREAT_δ(d) ∩ O` is monotone in the live external set. This is
monotonicity of one relational query, not of action value (Math §3.7). -/
theorem threat_removal_mono (δ : Declaration) (d : Domino)
    {O O' : Finset Domino} (h : O' ⊆ O) :
    δ.threat d ∩ O' ⊆ δ.threat d ∩ O :=
  Finset.inter_subset_inter (Finset.Subset.refl _) h

/-- PA-A15 (lead-threat incompleteness witness): in no-trump, `0:0` and
`1:1` both have empty when-led threat sets, yet `0:0` follows blanks and not
ones while `1:1` follows ones and not blanks — threat does not determine
follow behavior (Math §3.7, constructed counterexample). -/
theorem lead_threat_incomplete :
    (Declaration.notrump.threat ⟨0, 0, le_refl 0⟩ = ∅
      ∧ Declaration.notrump.threat ⟨1, 1, le_refl 1⟩ = ∅)
    ∧ (Declaration.notrump.effMem ⟨0, 0, le_refl 0⟩ (.natural 0)
      ∧ ¬ Declaration.notrump.effMem ⟨0, 0, le_refl 0⟩ (.natural 1))
    ∧ (Declaration.notrump.effMem ⟨1, 1, le_refl 1⟩ (.natural 1)
      ∧ ¬ Declaration.notrump.effMem ⟨1, 1, le_refl 1⟩ (.natural 0)) := by
  decide

/-! ## PA-A11: the unique trick winner -/

/-- PA-A11: for any four distinct dominoes with the designated lead played
first, the four contextual trick keys have a unique maximum (Math §3.6).
Legality of the three follower plays is not needed; distinctness and a
specified lead suffice. -/
theorem existsUnique_winner (δ : Declaration) (plays : Fin 4 → Domino)
    (hinj : Function.Injective plays) :
    ∃! i : Fin 4, ∀ j : Fin 4,
      δ.key (δ.ledSuit (plays 0)) (plays j)
        ≤ δ.key (δ.ledSuit (plays 0)) (plays i) := by
  set q := δ.ledSuit (plays 0) with hq
  obtain ⟨i, hi⟩ := Finite.exists_max fun j => δ.key q (plays j)
  refine ⟨i, hi, fun i' hi' => ?_⟩
  have hkey : δ.key q (plays i') = δ.key q (plays i) :=
    le_antisymm (hi i') (hi' i)
  -- the lead occupies a nonzero tier in its own context (PA-A09), so the
  -- maximal key sits in a nonzero tier
  have h0 : δ.tier q (plays 0) ≠ 0 :=
    Nat.pos_iff_ne_zero.mp (hq ▸ tier_ledSuit_pos δ (plays 0))
  have hti : δ.tier q (plays i) ≠ 0 := by
    have hle : δ.key q (plays 0) ≤ δ.key q (plays i) := hi 0
    simp only [key, Prod.Lex.le_iff, ofLex_toLex] at hle
    rcases hle with hlt | ⟨heq, -⟩ <;> omega
  have hti' : δ.tier q (plays i') ≠ 0 := by
    have hcomp : δ.tier q (plays i') = δ.tier q (plays i) :=
      congrArg Prod.fst (congrArg ofLex hkey)
    omega
  exact hinj (eq_of_key_eq hti' hkey)

end Declaration

end Texas42
