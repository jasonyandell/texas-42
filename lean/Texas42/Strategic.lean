/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Belief

/-!
# Texas 42 — strategic sufficiency (PA-E07, Math §§10.1–10.2)

The exact augmented strategic state: for a fixed admissible strategy
over a finite-horizon decision problem with latent state, expected
continuation values are functions of `B = (c, e, β)` alone. Formalized
finite-first (handoff §4): `S` is the reconstructed viewer state (the
`(c, e)` pair), `L` the latent domain (the ambient `Φ(c) × 𝒵` of
§10.1), with a finite Markov observation/latent kernel (§10.1
assumptions 1–3), deterministic viewer record transition (assumption 4,
record half), Bayes-filtered belief (assumption 4, filter half),
segment reward and terminal utility (assumptions 5–6), and backward
induction on the remaining-play grade — exactly the §10.1 proof.

`beliefVal` is the §10.2 Bellman recursion under one fixed boundary
convention with no double counting; `latentVal` is the ground truth
along true latent trajectories. The theorem: they agree under every
belief, at every horizon — so the expected-utility functional of the
fixed strategy factors through `(s, β)`. Zero-probability observation
segments contribute zero and their successor state is irrelevant,
exactly as §10.2 stipulates.
-/

namespace Texas42

namespace FinPMF

variable {α : Type} [Fintype α] [DecidableEq α]

/-- Expectation of a rational payoff under a finite PMF. -/
def exp (p : FinPMF α) (f : α → ℚ) : ℚ := ∑ a, p.mass a * f a

end FinPMF

/-- PA-E07 (define): a finite-horizon viewer decision process — the
Markov kernel gives the joint law of the next observation segment and
latent state; `stepS` is the exact viewer record transition; `reward`
the segment reward; `util` the terminal utility (Math §10.1). -/
structure BeliefProc (S A O L : Type) [Fintype O] [DecidableEq O]
    [Fintype L] [DecidableEq L] where
  kernel : S → A → L → FinPMF (O × L)
  reward : S → A → L → O → ℚ
  stepS : S → A → O → S
  util : S → L → ℚ

namespace BeliefProc

variable {S A O L : Type} [Fintype O] [DecidableEq O] [Fintype L]
  [DecidableEq L]

/-- Ground truth: the fixed-strategy expected value along true latent
trajectories, by backward induction on the remaining-play grade. -/
def latentVal (P : BeliefProc S A O L) (σ : S → A) :
    ℕ → S → L → ℚ
  | 0, s, l => P.util s l
  | n + 1, s, l =>
      ∑ x : O × L, (P.kernel s (σ s) l).mass x *
        (P.reward s (σ s) l x.1
          + latentVal P σ n (P.stepS s (σ s) x.1) x.2)

/-- The one-step predictive mass of `(o, l')` under a belief. -/
def predMass (P : BeliefProc S A O L) (s : S) (a : A) (β : FinPMF L)
    (x : O × L) : ℚ :=
  ∑ l, β.mass l * (P.kernel s a l).mass x

/-- The predictive probability of an observation segment. -/
def obsProb (P : BeliefProc S A O L) (s : S) (a : A) (β : FinPMF L)
    (o : O) : ℚ :=
  ∑ l', P.predMass s a β (o, l')

theorem predMass_nonneg (P : BeliefProc S A O L) (s : S) (a : A)
    (β : FinPMF L) (x : O × L) : 0 ≤ P.predMass s a β x :=
  Finset.sum_nonneg fun l _ =>
    mul_nonneg (β.nonneg l) ((P.kernel s a l).nonneg x)

theorem obsProb_nonneg (P : BeliefProc S A O L) (s : S) (a : A)
    (β : FinPMF L) (o : O) : 0 ≤ P.obsProb s a β o :=
  Finset.sum_nonneg fun l' _ => P.predMass_nonneg s a β (o, l')

/-- On a zero-probability observation every predictive mass vanishes. -/
theorem predMass_eq_zero {P : BeliefProc S A O L} {s : S} {a : A}
    {β : FinPMF L} {o : O} (h : P.obsProb s a β o = 0) (l' : L) :
    P.predMass s a β (o, l') = 0 := by
  have := Finset.sum_eq_zero_iff_of_nonneg
    (fun l' (_ : l' ∈ Finset.univ) => P.predMass_nonneg s a β (o, l'))
  rw [show (∑ l', P.predMass s a β (o, l')) = P.obsProb s a β o from rfl,
    h] at this
  exact (this.mp rfl) l' (Finset.mem_univ l')

/-- The observation probability grouped by latent state. -/
theorem obsProb_eq_sum_latent (P : BeliefProc S A O L) (s : S) (a : A)
    (β : FinPMF L) (o : O) :
    P.obsProb s a β o
      = ∑ l, β.mass l * ∑ l', (P.kernel s a l).mass (o, l') := by
  unfold obsProb predMass
  rw [Finset.sum_comm]
  exact Finset.sum_congr rfl fun l _ => by rw [Finset.mul_sum]

/-- The Bayes-filtered successor belief after observing `o` (§10.1
assumption 4). On a zero-probability observation the value is the junk
prior — that term contributes zero to every Bellman sum. -/
def filter (P : BeliefProc S A O L) (s : S) (a : A) (β : FinPMF L)
    (o : O) : FinPMF L :=
  if h : 0 < P.obsProb s a β o then
    { mass := fun l' => P.predMass s a β (o, l') / P.obsProb s a β o
      nonneg := fun l' =>
        div_nonneg (P.predMass_nonneg s a β (o, l')) h.le
      total := by
        rw [← Finset.sum_div,
          show (∑ l', P.predMass s a β (o, l')) = P.obsProb s a β o
            from rfl]
        exact div_self (ne_of_gt h) }
  else β

theorem filter_mass_of_pos {P : BeliefProc S A O L} {s : S} {a : A}
    {β : FinPMF L} {o : O} (h : 0 < P.obsProb s a β o) (l' : L) :
    (P.filter s a β o).mass l'
      = P.predMass s a β (o, l') / P.obsProb s a β o := by
  unfold filter
  rw [dif_pos h]

/-- The conditional expected segment reward `R̄` given the observation
(§10.2). -/
def condReward (P : BeliefProc S A O L) (s : S) (a : A) (β : FinPMF L)
    (o : O) : ℚ :=
  (∑ l, β.mass l * (∑ l', (P.kernel s a l).mass (o, l'))
      * P.reward s a l o)
    / P.obsProb s a β o

/-- §10.2: the Bellman recursion in `(s, β)` — one boundary convention,
segment reward plus filtered successor value, no double counting. -/
def beliefVal (P : BeliefProc S A O L) (σ : S → A) :
    ℕ → S → FinPMF L → ℚ
  | 0, s, β => β.exp (P.util s)
  | n + 1, s, β =>
      ∑ o : O, P.obsProb s (σ s) β o *
        (P.condReward s (σ s) β o
          + beliefVal P σ n (P.stepS s (σ s) o) (P.filter s (σ s) β o))

/-- PA-E07 (Math §10.1, strategic sufficiency): for every fixed
admissible strategy, the expected continuation value is a function of
the strategic state `(s, β)` — the belief-Bellman recursion integrates
the ground-truth latent value exactly, at every horizon. Consequently
any well-defined fixed-field best-response value over the strategy
class is a function of `B = (c, e, β)`. -/
theorem beliefVal_eq_exp_latentVal (P : BeliefProc S A O L) (σ : S → A)
    (n : ℕ) (s : S) (β : FinPMF L) :
    P.beliefVal σ n s β = β.exp (P.latentVal σ n s) := by
  induction n generalizing s β with
  | zero => rfl
  | succ n ih =>
    -- the per-observation identity
    have hkey : ∀ o : O,
        P.obsProb s (σ s) β o *
          (P.condReward s (σ s) β o
            + (P.filter s (σ s) β o).exp
                (P.latentVal σ n (P.stepS s (σ s) o)))
        = (∑ l, β.mass l * (∑ l', (P.kernel s (σ s) l).mass (o, l'))
              * P.reward s (σ s) l o)
          + ∑ l', P.predMass s (σ s) β (o, l')
              * P.latentVal σ n (P.stepS s (σ s) o) l' := by
      intro o
      rcases eq_or_lt_of_le (P.obsProb_nonneg s (σ s) β o) with hz | hpos
      · -- zero-probability segment: both sides vanish
        rw [← hz, zero_mul]
        have hr : (∑ l, β.mass l
            * (∑ l', (P.kernel s (σ s) l).mass (o, l'))
            * P.reward s (σ s) l o) = 0 := by
          apply Finset.sum_eq_zero
          intro l _
          have hnn : ∀ l ∈ Finset.univ, 0 ≤ β.mass l
              * ∑ l', (P.kernel s (σ s) l).mass (o, l') := fun l _ =>
            mul_nonneg (β.nonneg l) (Finset.sum_nonneg fun l' _ =>
              (P.kernel s (σ s) l).nonneg (o, l'))
          have hzero := (Finset.sum_eq_zero_iff_of_nonneg hnn).mp
            (by rw [← P.obsProb_eq_sum_latent s (σ s) β o, ← hz])
            l (Finset.mem_univ l)
          rw [hzero, zero_mul]
        have hw : (∑ l', P.predMass s (σ s) β (o, l')
            * P.latentVal σ n (P.stepS s (σ s) o) l') = 0 := by
          apply Finset.sum_eq_zero
          intro l' _
          rw [predMass_eq_zero hz.symm l', zero_mul]
        rw [hr, hw, add_zero]
      · -- positive segment: the normalizations cancel
        have hterm : ∀ l', (P.filter s (σ s) β o).mass l'
            * P.latentVal σ n (P.stepS s (σ s) o) l'
            = P.predMass s (σ s) β (o, l')
              * P.latentVal σ n (P.stepS s (σ s) o) l'
              / P.obsProb s (σ s) β o := by
          intro l'
          rw [filter_mass_of_pos hpos l', div_mul_eq_mul_div]
        have hexp : (P.filter s (σ s) β o).exp
            (P.latentVal σ n (P.stepS s (σ s) o))
            = (∑ l', P.predMass s (σ s) β (o, l')
                * P.latentVal σ n (P.stepS s (σ s) o) l')
              / P.obsProb s (σ s) β o := by
          unfold FinPMF.exp
          simp only [hterm]
          rw [← Finset.sum_div]
        rw [hexp,
          show P.condReward s (σ s) β o
            = (∑ l, β.mass l
                * (∑ l', (P.kernel s (σ s) l).mass (o, l'))
                * P.reward s (σ s) l o) / P.obsProb s (σ s) β o
          from rfl,
          ← add_div, mul_comm, div_mul_cancel₀ _ (ne_of_gt hpos)]
    -- expand the latent side into the per-observation form
    have hexpand : β.exp (P.latentVal σ (n + 1) s)
        = ∑ o : O,
            ((∑ l, β.mass l * (∑ l', (P.kernel s (σ s) l).mass (o, l'))
                * P.reward s (σ s) l o)
              + ∑ l', P.predMass s (σ s) β (o, l')
                  * P.latentVal σ n (P.stepS s (σ s) o) l') := by
      unfold FinPMF.exp
      -- both sides are the triple sum over o, l, l'
      have hL : (∑ l, β.mass l * P.latentVal σ (n + 1) s l)
          = ∑ l, ∑ o : O, ∑ l', β.mass l
              * ((P.kernel s (σ s) l).mass (o, l')
                * (P.reward s (σ s) l o
                  + P.latentVal σ n (P.stepS s (σ s) o) l')) := by
        apply Finset.sum_congr rfl
        intro l _
        rw [show P.latentVal σ (n + 1) s l
            = ∑ x : O × L, (P.kernel s (σ s) l).mass x *
                (P.reward s (σ s) l x.1
                  + P.latentVal σ n (P.stepS s (σ s) x.1) x.2)
          from rfl]
        rw [Finset.mul_sum, Fintype.sum_prod_type]
      rw [hL, Finset.sum_comm]
      apply Finset.sum_congr rfl
      intro o _
      have h1 : (∑ l, β.mass l
            * (∑ l', (P.kernel s (σ s) l).mass (o, l'))
            * P.reward s (σ s) l o)
          = ∑ l, ∑ l', β.mass l * ((P.kernel s (σ s) l).mass (o, l')
              * P.reward s (σ s) l o) := by
        apply Finset.sum_congr rfl
        intro l _
        rw [mul_assoc, Finset.sum_mul, Finset.mul_sum]
      have h2 : (∑ l', P.predMass s (σ s) β (o, l')
            * P.latentVal σ n (P.stepS s (σ s) o) l')
          = ∑ l, ∑ l', β.mass l * ((P.kernel s (σ s) l).mass (o, l')
              * P.latentVal σ n (P.stepS s (σ s) o) l') := by
        rw [Finset.sum_comm]
        apply Finset.sum_congr rfl
        intro l' _
        unfold predMass
        rw [Finset.sum_mul]
        apply Finset.sum_congr rfl
        intro l _
        ring
      rw [h1, h2, ← Finset.sum_add_distrib]
      apply Finset.sum_congr rfl
      intro l _
      rw [← Finset.sum_add_distrib]
      apply Finset.sum_congr rfl
      intro l' _
      ring
    -- assemble
    show (∑ o : O, P.obsProb s (σ s) β o *
        (P.condReward s (σ s) β o
          + P.beliefVal σ n (P.stepS s (σ s) o)
              (P.filter s (σ s) β o)))
      = β.exp (P.latentVal σ (n + 1) s)
    rw [hexpand]
    apply Finset.sum_congr rfl
    intro o _
    rw [ih]
    exact hkey o

/-- PA-E07 (consequence): over any finite admissible strategy class,
the best-response value computed from ground-truth latent expectations
is the best response of the `(s, β)`-Bellman values — a function of the
strategic state `B` alone. When the maximum is attained the
best-response correspondence is likewise determined by `B` (§10.1). -/
theorem bestResponse_eq (P : BeliefProc S A O L)
    (Cls : Finset (S → A)) (hne : Cls.Nonempty) (n : ℕ) (s : S)
    (β : FinPMF L) :
    Cls.sup' hne (fun σ => β.exp (P.latentVal σ n s))
      = Cls.sup' hne (fun σ => P.beliefVal σ n s β) :=
  Finset.sup'_congr hne rfl fun σ _ =>
    (P.beliefVal_eq_exp_latentVal σ n s β).symm

end BeliefProc

end Texas42
