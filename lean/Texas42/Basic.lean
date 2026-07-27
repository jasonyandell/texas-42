/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Mathlib

/-!
# Texas 42 — Layer A finite algebra (first slice)

Kernel-checked foundations for the domino set, following the mechanization
ledger rows PA-A01 (finite types), PA-A02 (exactly 28 canonical dominoes),
and PA-A04 (count-point function and total 35).

A domino is represented canonically as an ordered pair `(high, low)` with a
proof `low ≤ high`, per the handoff's recommendation (Layer A): prefer the
canonical pair over a quotient by pair permutation.
-/

namespace Texas42

/-- PA-A01: a pip value, `0` through `6`. -/
abbrev Pip := Fin 7

/-- PA-A01: a domino, as the canonical ordered pair of pips with `low ≤ high`. -/
structure Domino where
  high : Pip
  low : Pip
  le : low ≤ high
deriving DecidableEq

namespace Domino

/-- The canonical-pair representation is equivalent to the subtype of ordered
pip pairs; this transports `Fintype` and makes cardinality kernel-computable. -/
def equivSubtype : Domino ≃ { p : Pip × Pip // p.2 ≤ p.1 } where
  toFun d := ⟨(d.high, d.low), d.le⟩
  invFun p := ⟨p.1.1, p.1.2, p.2⟩

instance : Fintype Domino := Fintype.ofEquiv _ equivSubtype.symm

/-- PA-A02: there are exactly 28 canonical dominoes. -/
theorem card_domino : Fintype.card Domino = 28 := by
  rw [Fintype.card_congr equivSubtype]
  decide

/-- The pip sum of a domino. -/
def pipSum (d : Domino) : ℕ :=
  d.high.val + d.low.val

/-- PA-A04: the count-point function. The three tiles of pip sum 5 —
`(5,0)`, `(4,1)`, `(3,2)` — score 5; the two tiles of pip sum 10 —
`(5,5)`, `(6,4)` — score 10; every other tile scores 0. -/
def countPoints (d : Domino) : ℕ :=
  if pipSum d = 5 then 5
  else if pipSum d = 10 then 10
  else 0

/-- PA-A04: the count points over all 28 dominoes total 35. -/
theorem total_countPoints : ∑ d : Domino, countPoints d = 35 := by
  decide

end Domino

end Texas42
