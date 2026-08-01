import Mathlib

namespace Constellation

open scoped BigOperators

/-- Pips are the seven values `0, ..., 6`. -/
abbrev Pip := Fin 7

/--
A domino is stored by its canonical ordered ends `(high, low)` together with
`low ≤ high`. This subtype has exactly 28 inhabitants and inherits finite,
decidable equality, making later `decide`-based finite checks direct.
-/
abbrev Domino := {p : Pip × Pip // p.2 ≤ p.1}

namespace Domino

/-- Construct a canonical domino from ordered ends. -/
def ofEnds (high low : Pip) (h : low ≤ high) : Domino :=
  ⟨(high, low), h⟩

/-- The higher pip in the canonical name. -/
def high (d : Domino) : Pip := d.1.1

/-- The lower pip in the canonical name. -/
def low (d : Domino) : Pip := d.1.2

/-- Natural pip incidence. -/
def contains (d : Domino) (p : Pip) : Bool :=
  d.high == p || d.low == p

/-- Whether the tile is a double. -/
def isDouble (d : Domino) : Bool :=
  d.high == d.low

/-- Pip sum. -/
def pipSum (d : Domino) : Nat :=
  d.high.val + d.low.val

end Domino

/-- Straight count decoration: the sum-five tiles score 5 and sum-ten tiles score 10. -/
def countPoints (d : Domino) : Nat :=
  if d.pipSum = 5 then 5
  else if d.pipSum = 10 then 10
  else 0

/-- The nine Straight declarations. -/
inductive Declaration where
  | pipTrump (p : Pip)
  | doublesTrump
  | noTrump
  deriving DecidableEq, Fintype, Repr

/-- Seven natural contexts plus the called context. -/
inductive SuitContext where
  | natural (p : Pip)
  | called
  deriving DecidableEq, Fintype, Repr

/-- Boolean called-set membership. In Straight play every called tile is powered. -/
def isCalled : Declaration → Domino → Bool
  | .pipTrump p, d => d.contains p
  | .doublesTrump, d => d.isDouble
  | .noTrump, _ => false

/-- The finite called set selected by a declaration. -/
def calledSet (δ : Declaration) : Finset Domino :=
  Finset.univ.filter (fun d => isCalled δ d = true)

/-- A called tile leads called context; otherwise a tile leads its higher pip. -/
def ledContext (δ : Declaration) (d : Domino) : SuitContext :=
  if isCalled δ d then .called else .natural d.high

/-- Effective-suit following: called tiles are removed from every natural incidence. -/
def follows (δ : Declaration) (d : Domino) : SuitContext → Bool
  | .called => isCalled δ d
  | .natural p => Bool.not (isCalled δ d) && d.contains p

/-- Rank code `0, ..., 12, TOP`, with `TOP = 13`. -/
def rankCode (δ : Declaration) (d : Domino) : Fin 14 :=
  if d.isDouble then
    match δ with
    | .doublesTrump =>
        ⟨d.high.val, by
          have hh := d.high.isLt
          omega⟩
    | _ => 13
  else
    ⟨d.pipSum, by
      have hh := d.high.isLt
      have hl := d.low.isLt
      change d.high.val + d.low.val < 14
      omega⟩

/-- Tier 2 is called-and-powered, tier 1 is an uncalled follower, and tier 0 is a slough. -/
def tierCode (δ : Declaration) (d : Domino) (q : SuitContext) : Fin 3 :=
  if isCalled δ d then 2
  else if follows δ d q then 1
  else 0

/--
A trick key is the lexicographic pair `(tier, rank)` encoded as
`14 * tier + rank : Fin 42`. The inherited order on `Fin 42` is exactly the
lexicographic order because every rank code lies in one 14-element block.
Tier-zero keys are forced to rank zero and intentionally tie.
-/
abbrev TrickKey := Fin 42

namespace TrickKey

/-- Recover the tier as a natural number; useful for executable checks. -/
def tier (k : TrickKey) : Nat := k.val / 14

/-- Recover the rank block offset as a natural number. -/
def rank (k : TrickKey) : Nat := k.val % 14

end TrickKey

/-- Total contextual trick key. -/
def trickKey (δ : Declaration) (d : Domino) (q : SuitContext) : TrickKey :=
  let t := tierCode δ d q
  let r : Fin 14 := if t = 0 then 0 else rankCode δ d
  ⟨t.val * 14 + r.val, by
    have ht := t.isLt
    have hr := r.isLt
    omega⟩

/-- A completed trick, indexed in play order from its leader. -/
abbrev Trick := Fin 4 → Domino

/-- The context fixed by the first play. -/
def trickContext (δ : Declaration) (t : Trick) : SuitContext :=
  ledContext δ (t 0)

/-- Key of one play in the context fixed by the trick lead. -/
def playKey (δ : Declaration) (t : Trick) (i : Fin 4) : TrickKey :=
  trickKey δ (t i) (trickContext δ t)

private def chooseMax (key : Fin 4 → TrickKey) (i j : Fin 4) : Fin 4 :=
  if key i < key j then j else i

private def max4Index (key : Fin 4 → TrickKey) : Fin 4 :=
  chooseMax key (chooseMax key 0 1) (chooseMax key 2 3)

/--
Winning play index. Ties are resolved to the earlier candidate, but
`unique_winner` proves that no tie can occur at the maximum when the four
physical tiles are distinct.
-/
def winner (δ : Declaration) (t : Trick) : Fin 4 :=
  max4Index (playKey δ t)

/-- One trick point plus all count carried by the four played tiles. -/
def award (t : Trick) : Nat :=
  1 + ∑ i : Fin 4, countPoints (t i)

private theorem le_chooseMax_left
    (key : Fin 4 → TrickKey) (i j : Fin 4) :
    key i ≤ key (chooseMax key i j) := by
  unfold chooseMax
  split_ifs with h
  · exact le_of_lt h
  · exact le_rfl

private theorem le_chooseMax_right
    (key : Fin 4 → TrickKey) (i j : Fin 4) :
    key j ≤ key (chooseMax key i j) := by
  unfold chooseMax
  split_ifs with h
  · exact le_rfl
  · exact le_of_not_gt h

private theorem le_max4Index
    (key : Fin 4 → TrickKey) (j : Fin 4) :
    key j ≤ key (max4Index key) := by
  have h0 := le_chooseMax_left key (0 : Fin 4) 1
  have h1 := le_chooseMax_right key (0 : Fin 4) 1
  have h2 := le_chooseMax_left key (2 : Fin 4) 3
  have h3 := le_chooseMax_right key (2 : Fin 4) 3
  have hL := le_chooseMax_left key (chooseMax key 0 1) (chooseMax key 2 3)
  have hR := le_chooseMax_right key (chooseMax key 0 1) (chooseMax key 2 3)
  change key j ≤ key (chooseMax key (chooseMax key 0 1) (chooseMax key 2 3))
  fin_cases j
  · exact h0.trans hL
  · exact h1.trans hL
  · exact h2.trans hR
  · exact h3.trans hR

/-- The chosen winner has maximal key even before distinctness is assumed. -/
theorem winner_maximal (δ : Declaration) (t : Trick) :
    ∀ j : Fin 4, playKey δ t j ≤ playKey δ t (winner δ t) := by
  simpa [winner] using (le_max4Index (playKey δ t))

section DecideChecks
-- Kernel `decide` over all declarations x tiles (and below, x contexts x tile
-- pairs: 56,448 cases); the default budgets are far too small for these
-- closed finite checks.
set_option maxHeartbeats 5000000
set_option maxRecDepth 100000

private theorem lead_key_positive :
    ∀ (δ : Declaration) (d : Domino),
      (0 : TrickKey) < trickKey δ d (ledContext δ d) := by
  decide

/--
Positive contextual keys are injective on physical dominoes.
Tier zero is deliberately excluded because all sloughs have key zero.
-/
theorem positive_key_injective :
    ∀ (δ : Declaration) (q : SuitContext) (d e : Domino),
      (0 : TrickKey) < trickKey δ d q →
      trickKey δ d q = trickKey δ e q →
      d = e := by
  decide

/--
For four distinct physical dominoes, the context led by the first play has a
unique maximal key. Follower legality is not needed for uniqueness: the lead
ensures the maximal tier is positive, and positive keys are injective.
-/
theorem unique_winner (δ : Declaration) (t : Trick)
    (distinct : Function.Injective t) :
    ∃! i : Fin 4, ∀ j : Fin 4, playKey δ t j ≤ playKey δ t i := by
  refine ⟨winner δ t, winner_maximal δ t, ?_⟩
  intro i hi
  have hWinnerLe : playKey δ t (winner δ t) ≤ playKey δ t i := hi _
  have hILe : playKey δ t i ≤ playKey δ t (winner δ t) :=
    winner_maximal δ t i
  have hKey : playKey δ t (winner δ t) = playKey δ t i :=
    le_antisymm hWinnerLe hILe
  have hLead : (0 : TrickKey) < playKey δ t 0 := by
    simpa [playKey, trickContext] using (lead_key_positive δ (t 0))
  have hWinner : (0 : TrickKey) < playKey δ t (winner δ t) :=
    lt_of_lt_of_le hLead (winner_maximal δ t 0)
  have hTile : t (winner δ t) = t i :=
    positive_key_injective
      δ (trickContext δ t) (t (winner δ t)) (t i)
      (by simpa [playKey] using hWinner)
      (by simpa [playKey] using hKey)
  exact (distinct hTile).symm

end DecideChecks

private def pipTrumpExample : Trick :=
  ![Domino.ofEnds 6 6 (by decide),
    Domino.ofEnds 6 5 (by decide),
    Domino.ofEnds 6 4 (by decide),
    Domino.ofEnds 4 4 (by decide)]

-- Fours are trump: `4-4` wins; `6-4` contributes ten count, so the award is 11.
example :
    winner (.pipTrump 4) pipTrumpExample = 3 ∧
    award pipTrumpExample = 11 := by
  decide

private def noTrumpExample : Trick :=
  ![Domino.ofEnds 5 0 (by decide),
    Domino.ofEnds 5 5 (by decide),
    Domino.ofEnds 6 5 (by decide),
    Domino.ofEnds 6 6 (by decide)]

-- No-trump has no tier 2: `5-5` is the top natural-five follower; the award is 16.
example :
    winner .noTrump noTrumpExample = 1 ∧
    award noTrumpExample = 16 ∧
    (∀ i : Fin 4, TrickKey.tier (playKey .noTrump noTrumpExample i) < 2) := by
  decide

end Constellation
