/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Cells

/-!
# Texas 42 — the finite belief layer

PA-E01–PA-E03 (Math §§8.1–8.3; K15): exact finite probability mass
functions with rational masses; Bayes conditioning against a nonnegative
likelihood at positive-probability observations (posterior normalization
and the likelihood chain rule); behavioral policy kernels with the
history-likelihood product over a public prefix; and the pushforward of
the posterior through the remainder map, whose support lies inside the
cell fiber — support bounds where belief can live and never determines
the weights (Math §6.7).
-/

namespace Texas42

set_option maxRecDepth 100000

/-- PA-E01: an exact finite probability mass function with rational
masses (finite-first discipline — no measure theory in the native game). -/
structure FinPMF (α : Type) [Fintype α] [DecidableEq α] where
  mass : α → ℚ
  nonneg : ∀ a, 0 ≤ mass a
  total : ∑ a, mass a = 1

namespace FinPMF

variable {α β : Type} [Fintype α] [DecidableEq α] [Fintype β]
  [DecidableEq β]

theorem ext {p q : FinPMF α} (h : p.mass = q.mass) : p = q := by
  cases p with
  | mk m hn ht =>
      cases q with
      | mk m' hn' ht' =>
          dsimp only at h
          subst h
          rfl

/-- The support of a finite PMF (as a set — masses need no
decidability to have a support). -/
def support (p : FinPMF α) : Set α := {a | 0 < p.mass a}

theorem mem_support {p : FinPMF α} {a : α} :
    a ∈ p.support ↔ 0 < p.mass a := Iff.rfl

/-- PA-E02 (posterior normalization, Math §8.2): Bayes conditioning of a
prior against a nonnegative likelihood with positive total mass — the
posterior is exactly `prior · likelihood / Z`. -/
def condition (p : FinPMF α) (L : α → ℚ) (hL : ∀ a, 0 ≤ L a)
    (hZ : 0 < ∑ a, p.mass a * L a) : FinPMF α where
  mass a := p.mass a * L a / (∑ b, p.mass b * L b)
  nonneg a := by
    apply div_nonneg
    · exact mul_nonneg (p.nonneg a) (hL a)
    · exact hZ.le
  total := by
    rw [← Finset.sum_div, div_self (ne_of_gt hZ)]

/-- Support–measure separation: observations can zero out worlds, never
resurrect them. -/
theorem support_condition_subset (p : FinPMF α) (L : α → ℚ)
    (hL : ∀ a, 0 ≤ L a) (hZ : 0 < ∑ a, p.mass a * L a) :
    (p.condition L hL hZ).support ⊆ p.support := by
  intro a ha
  have ha' : 0 < (p.condition L hL hZ).mass a := ha
  show 0 < p.mass a
  by_contra hp
  push_neg at hp
  have hp0 : p.mass a = 0 := le_antisymm hp (p.nonneg a)
  unfold condition at ha'
  dsimp only at ha'
  rw [hp0, zero_mul, zero_div] at ha'
  exact lt_irrefl 0 ha'

/-- Positive posterior mass certifies positive likelihood. -/
theorem likelihood_pos_of_mass_pos {p : FinPMF α} {L : α → ℚ}
    {hL : ∀ a, 0 ≤ L a} {hZ : 0 < ∑ a, p.mass a * L a} {a : α}
    (h : 0 < (p.condition L hL hZ).mass a) : 0 < L a := by
  by_contra hc
  push_neg at hc
  have h0 : L a = 0 := le_antisymm hc (hL a)
  unfold condition at h
  dsimp only at h
  rw [h0, mul_zero, zero_div] at h
  exact lt_irrefl 0 h

/-- PA-E02 (likelihood chain rule, Math §§8.1–8.2): conditioning on a
product likelihood is sequential conditioning — the history-likelihood
product factors through Bayes. -/
theorem condition_mul (p : FinPMF α) (L₁ L₂ : α → ℚ)
    (h₁ : ∀ a, 0 ≤ L₁ a) (h₂ : ∀ a, 0 ≤ L₂ a)
    (hZ₁ : 0 < ∑ a, p.mass a * L₁ a)
    (hZ : 0 < ∑ a, p.mass a * (L₁ a * L₂ a))
    (hZ' : 0 < ∑ a, (p.condition L₁ h₁ hZ₁).mass a * L₂ a) :
    p.condition (fun a => L₁ a * L₂ a)
        (fun a => mul_nonneg (h₁ a) (h₂ a)) hZ
      = (p.condition L₁ h₁ hZ₁).condition L₂ h₂ hZ' := by
  have hZ₁ne : (∑ a, p.mass a * L₁ a) ≠ 0 := ne_of_gt hZ₁
  have hZne : (∑ a, p.mass a * (L₁ a * L₂ a)) ≠ 0 := ne_of_gt hZ
  have hsum : ∑ b, (p.mass b * L₁ b / (∑ c, p.mass c * L₁ c)) * L₂ b
      = (∑ b, p.mass b * (L₁ b * L₂ b)) / (∑ c, p.mass c * L₁ c) := by
    rw [Finset.sum_div]
    exact Finset.sum_congr rfl fun a _ => by ring
  apply ext
  funext a
  unfold condition
  dsimp only
  rw [hsum]
  field_simp

/-- PA-E03 (pushforward, Math §8.3): the image measure along a map —
`μ(b) = Σ_{a : f(a) = b} ν(a)`. -/
def map (p : FinPMF α) (f : α → β) : FinPMF β where
  mass b := ∑ a ∈ Finset.univ.filter (fun a => f a = b), p.mass a
  nonneg b := Finset.sum_nonneg fun a _ => p.nonneg a
  total := by
    rw [← p.total]
    simp only [Finset.sum_filter]
    rw [Finset.sum_comm]
    apply Finset.sum_congr rfl
    intro a _
    simp

/-- A pushforward support point has a positive-mass preimage. -/
theorem exists_preimage_of_mem_support_map {p : FinPMF α} {f : α → β}
    {b : β} (h : b ∈ (p.map f).support) :
    ∃ a, 0 < p.mass a ∧ f a = b := by
  rw [mem_support] at h
  unfold map at h
  dsimp only at h
  by_contra hc
  push_neg at hc
  have hzero : ∑ a ∈ Finset.univ.filter (fun a => f a = b), p.mass a = 0 := by
    apply Finset.sum_eq_zero
    intro a ha
    rw [Finset.mem_filter] at ha
    rcases lt_or_eq_of_le (p.nonneg a) with hpos | heq
    · exact absurd ha.2 (hc a hpos)
    · exact heq.symm
  rw [hzero] at h
  exact lt_irrefl 0 h

end FinPMF

/-! ## Policy kernels and history likelihood (PA-E01/E02, Math §8.1) -/

/-- PA-E01: a behavioral policy kernel — the actor's private context and
the public record weight each action (`σ̃_j(a | I)`, Math §8.1). -/
def PolicyKernel : Type := ViewerCtx → PubState → Domino → ℚ

/-- The history-likelihood product along a public prefix, in the kernel
representation (Math §8.1): each observed action contributes its exact
conditional probability given the actor's information. -/
def likelihoodFrom (π : Seat → PolicyKernel) (ω : Deal) :
    PlayState → PubState → List Domino → ℚ
  | _, _, [] => 1
  | X, P, d :: ds =>
      π X.actor ⟨X.actor, ω.hands X.actor⟩ P d
        * likelihoodFrom π ω (X.step d) (P.step d) ds

/-- PA-E02 (product structure): the likelihood of a concatenated prefix
is the product of the prefix likelihoods — the chain rule over public
actions. -/
theorem likelihoodFrom_append (π : Seat → PolicyKernel) (ω : Deal)
    (X : PlayState) (P : PubState) (l₁ l₂ : List Domino) :
    likelihoodFrom π ω X P (l₁ ++ l₂)
      = likelihoodFrom π ω X P l₁
        * likelihoodFrom π ω (X.replayFrom l₁) (l₁.foldl PubState.step P) l₂ := by
  induction l₁ generalizing X P with
  | nil => simp [likelihoodFrom]
  | cons d ds ih =>
      simp only [List.cons_append, likelihoodFrom, ih, PlayState.replayFrom_cons,
        List.foldl_cons]
      ring

/-- Nonnegative kernels give nonnegative history likelihoods. -/
theorem likelihoodFrom_nonneg {π : Seat → PolicyKernel}
    (hπ : ∀ s v P d, 0 ≤ π s v P d) (ω : Deal) :
    ∀ (X : PlayState) (P : PubState) (ds : List Domino),
      0 ≤ likelihoodFrom π ω X P ds
  | _, _, [] => zero_le_one
  | X, P, d :: ds =>
      mul_nonneg (hπ ..) (likelihoodFrom_nonneg hπ ω (X.step d) (P.step d) ds)

/-! ## The deal domain is finite -/

/-- Deals as a subtype of hand assignments (with membership-level
disjointness, which is directly decidable). -/
def Deal.equivSubtype : Deal ≃
    {f : Seat → Finset Domino //
      (∀ s, (f s).card = 7)
        ∧ ∀ s t : Seat, s ≠ t → ∀ x ∈ f s, x ∉ f t} where
  toFun ω := ⟨ω.hands, ω.card_hands, fun s t hst x hx =>
    Finset.disjoint_left.mp (ω.disjoint s t hst) hx⟩
  invFun p := ⟨p.1, p.2.1, fun s t hst =>
    Finset.disjoint_left.mpr (p.2.2 s t hst)⟩

instance : DecidableEq Deal := fun ω₁ ω₂ =>
  decidable_of_iff (ω₁.hands = ω₂.hands) ⟨Deal.ext, fun h => by rw [h]⟩

set_option maxRecDepth 8192 in
noncomputable instance : Fintype Deal :=
  Fintype.ofEquiv _ Deal.equivSubtype.symm

instance PlayState.decidableLegalFrom :
    ∀ (X : PlayState) (ds : List Domino), Decidable (X.LegalFrom ds)
  | _, [] => .isTrue trivial
  | X, d :: ds =>
      haveI := PlayState.decidableLegalFrom (X.step d) ds
      inferInstanceAs (Decidable (_ ∧ _))

instance (v : ViewerCtx) (K : Contract) (ds : List Domino) (ω' : Deal) :
    Decidable (v.Compatible K ds ω') :=
  inferInstanceAs (Decidable (_ ∧ _))

/-! ## The game posterior and its pushforward (PA-E02/E03, Math §§8.2–8.3) -/

/-- The Bayes weight of a candidate deal: the rule-compatibility
indicator times the history likelihood (Math §8.2's numerator shape). -/
def bayesWeight (v : ViewerCtx) (π : Seat → PolicyKernel) (K : Contract)
    (ds : List Domino) (ω' : Deal) : ℚ :=
  (if v.Compatible K ds ω' then 1 else 0)
    * likelihoodFrom π ω' (PlayState.init ω' K) (PubState.init K) ds

theorem bayesWeight_nonneg {π : Seat → PolicyKernel}
    (hπ : ∀ s v P d, 0 ≤ π s v P d) (v : ViewerCtx) (K : Contract)
    (ds : List Domino) (ω' : Deal) : 0 ≤ bayesWeight v π K ds ω' := by
  unfold bayesWeight
  apply mul_nonneg
  · split_ifs <;> norm_num
  · exact likelihoodFrom_nonneg hπ ω' _ _ ds

/-- Positive Bayes weight certifies rule compatibility. -/
theorem compatible_of_bayesWeight_pos {v : ViewerCtx}
    {π : Seat → PolicyKernel} {K : Contract} {ds : List Domino}
    {ω' : Deal} (h : 0 < bayesWeight v π K ds ω') :
    v.Compatible K ds ω' := by
  by_contra hc
  unfold bayesWeight at h
  rw [if_neg hc, zero_mul] at h
  exact lt_irrefl 0 h

/-- PA-E02: the posterior on the fixed current-attempt deal domain
(Math §8.2) — Bayes' rule with the compatibility indicator and history
likelihood; the domain variable remains the fixed initial deal. -/
noncomputable def posterior (prior : FinPMF Deal) (v : ViewerCtx)
    (π : Seat → PolicyKernel) (hπ : ∀ s v P d, 0 ≤ π s v P d)
    (K : Contract) (ds : List Domino)
    (hZ : 0 < ∑ ω', prior.mass ω' * bayesWeight v π K ds ω') :
    FinPMF Deal :=
  prior.condition (bayesWeight v π K ds)
    (bayesWeight_nonneg hπ v K ds) hZ

/-- PA-E03: the current physical belief — the pushforward of the
posterior through the remainder map (Math §8.3). -/
noncomputable def physicalBelief (prior : FinPMF Deal) (v : ViewerCtx)
    (π : Seat → PolicyKernel) (hπ : ∀ s v P d, 0 ≤ π s v P d)
    (K : Contract) (ds : List Domino)
    (hZ : 0 < ∑ ω', prior.mass ω' * bayesWeight v π K ds ω') :
    FinPMF (Seat → Finset Domino) :=
  (posterior prior v π hπ K ds hZ).map (v.remainder K ds)

/-- PA-E03 + the losslessness tie-in: the physical belief is supported
inside the derived cell fiber — belief lives on the support the cells
describe exactly, while the weights remain the policy model's business
(support is not belief, Math §6.7). -/
theorem physicalBelief_support_isWorld (prior : FinPMF Deal)
    (v : ViewerCtx) (π : Seat → PolicyKernel)
    (hπ : ∀ s v P d, 0 ≤ π s v P d) (K : Contract) (ds : List Domino)
    (hZ : 0 < ∑ ω', prior.mass ω' * bayesWeight v π K ds ω')
    {A : Seat → Finset Domino}
    (hA : A ∈ (physicalBelief prior v π hπ K ds hZ).support) :
    v.IsWorld (PubState.replay K ds) A := by
  obtain ⟨ω', hmass, hrem⟩ :=
    FinPMF.exists_preimage_of_mem_support_map
      (p := posterior prior v π hπ K ds hZ)
      (f := v.remainder K ds) hA
  have hw : 0 < bayesWeight v π K ds ω' :=
    FinPMF.likelihood_pos_of_mass_pos hmass
  have hcompat := compatible_of_bayesWeight_pos hw
  rw [← hrem]
  exact v.isWorld_remainder K ds hcompat

end Texas42
