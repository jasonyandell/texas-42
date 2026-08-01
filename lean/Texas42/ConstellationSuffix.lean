import Texas42.ConstellationCore

namespace Constellation

/--
A depth-`k` suffix begins at a trick boundary. The four hands are kept as
`Finset`s because play is deletion-heavy and later constellation restrictions
will also be finite-set restrictions.
-/
structure SuffixPos (k : Nat) where
  declaration : Declaration
  hands : Fin 4 → Finset Domino
  leader : Fin 4
  card_hands : ∀ s, (hands s).card = k
  disjoint_hands : ∀ s t, s ≠ t → Disjoint (hands s) (hands t)

/-- Clockwise offset from a leader. -/
def seatAt (leader : Fin 4) (offset : Nat) : Fin 4 :=
  ⟨(leader.val + offset) % 4, Nat.mod_lt _ (by decide)⟩

/-- Seats `0,2` maximize the signed margin; seats `1,3` minimize it. -/
def onTeamZero (s : Fin 4) : Bool :=
  s.val % 2 == 0

/-- Trick points signed from the `(0,2)` partnership's perspective. -/
def signedAward (winnerSeat : Fin 4) (points : Nat) : Int :=
  if onTeamZero winnerSeat then Int.ofNat points else -Int.ofNat points

/--
Internal play state. `pending` contains the current trick in play order.
States reached from `SuffixPos` always have prefix length at most three.
-/
structure MidState where
  declaration : Declaration
  hands : Fin 4 → Finset Domino
  leader : Fin 4
  pending : List Domino

namespace MidState

/-- The seat whose turn it is. -/
def actor (st : MidState) : Fin 4 :=
  seatAt st.leader st.pending.length

/-- Total tiles still in hands; current-trick tiles have already been removed. -/
def remaining (st : MidState) : Nat :=
  (st.hands 0).card + (st.hands 1).card +
    (st.hands 2).card + (st.hands 3).card

/--
Leader freedom, then follow-if-possible. A follower set is selected exactly
when it is nonempty; otherwise the entire current hand is legal.
-/
def legalMoves (st : MidState) : Finset Domino :=
  let hand := st.hands st.actor
  match st.pending with
  | [] => hand
  | lead :: _ =>
      let q := ledContext st.declaration lead
      let followers := hand.filter (fun d => follows st.declaration d q = true)
      if followers = ∅ then hand else followers

def LegalMove (st : MidState) (d : Domino) : Prop :=
  d ∈ st.legalMoves

end MidState

/-- Remove one tile from one absolute seat. -/
def eraseAt
    (hands : Fin 4 → Finset Domino) (seat : Fin 4) (d : Domino) :
    Fin 4 → Finset Domino :=
  fun s => if s = seat then (hands s).erase d else hands s

/-- Control information produced by appending one play. -/
private structure ControlStep where
  leader : Fin 4
  pending : List Domino
  reward : Int

/--
Append one tile to the trick. On the fourth play, Stage 1's `winner` and
`award` resolve the trick, the winning absolute seat becomes leader, and the
prefix is cleared.
-/
private def controlStep (st : MidState) (d : Domino) : ControlStep :=
  match st.pending with
  | [] =>
      ⟨st.leader, [d], 0⟩
  | [a] =>
      ⟨st.leader, [a, d], 0⟩
  | [a, b] =>
      ⟨st.leader, [a, b, d], 0⟩
  | [a, b, c] =>
      let trick : Trick := ![a, b, c, d]
      let winningSeat :=
        seatAt st.leader (winner st.declaration trick).val
      ⟨winningSeat, [],
        signedAward winningSeat (award trick)⟩
  | _ =>
      -- Unreachable from a boundary through `step`; retained to keep the
      -- executable transition total on arbitrary `MidState`s.
      ⟨st.leader, st.pending ++ [d], 0⟩

structure StepResult where
  state : MidState
  reward : Int

/-- One legal atomic play transition. -/
def step (st : MidState) (d : Domino) : StepResult :=
  let c := controlStep st d
  { state :=
      { declaration := st.declaration
        hands := eraseAt st.hands st.actor d
        leader := c.leader
        pending := c.pending }
    reward := c.reward }

/-- Every listed legal move belongs to the acting hand. -/
theorem legalMoves_subset_hand (st : MidState) :
    st.legalMoves ⊆ st.hands st.actor := by
  intro d hd
  cases hp : st.pending with
  | nil =>
      simpa [MidState.legalMoves, hp] using hd
  | cons lead rest =>
      let followers :=
        (st.hands st.actor).filter
          (fun e =>
            follows st.declaration e (ledContext st.declaration lead) = true)
      by_cases h : followers = ∅
      · simpa [MidState.legalMoves, hp, followers, h] using hd
      · have hd' : d ∈ followers := by
          simpa [MidState.legalMoves, hp, followers, h] using hd
        exact Finset.filter_subset _ _ hd'

/-- Erasing a legal play consumes exactly one remaining tile. -/
theorem step_remaining (st : MidState) (d : Domino)
    (hd : st.LegalMove d) :
    (step st d).state.remaining = st.remaining - 1 := by
  have hmem : d ∈ st.hands st.actor :=
    legalMoves_subset_hand st (by
      simpa [MidState.LegalMove] using hd)
  have hpos : 0 < (st.hands st.actor).card :=
    Finset.card_pos.mpr ⟨d, hmem⟩
  have h4 : ∀ i : Fin 4, ((step st d).state.hands i).card =
      if i = st.actor then (st.hands i).card - 1 else (st.hands i).card := by
    intro i
    by_cases h : i = st.actor
    · subst h
      simp [step, eraseAt, Finset.card_erase_of_mem hmem]
    · simp [step, eraseAt, h]
  have hA : st.actor = 0 ∨ st.actor = 1 ∨ st.actor = 2 ∨ st.actor = 3 := by
    omega
  simp only [MidState.remaining, h4]
  rcases hA with h | h | h | h <;>
    rw [h] at hpos ⊢ <;> simp <;> omega

/-- Boundary state viewed as an empty-prefix play state. -/
def initialState {k : Nat} (X : SuffixPos k) : MidState :=
  { declaration := X.declaration
    hands := X.hands
    leader := X.leader
    pending := [] }

/--
The initial fuel `4*k` is exactly the physical grade: there are four hands of
cardinality `k`, and `step_remaining` proves that every legal play consumes one.
-/
theorem initial_remaining {k : Nat} (X : SuffixPos k) :
    (initialState X).remaining = 4 * k := by
  simp [initialState, MidState.remaining, X.card_hands] <;>
    omega

/-- Maximum for team zero, minimum for team one; empty is unreachable in valid play. -/
def chooseExtremum (maximize : Bool) (scores : Finset Int) : Int :=
  if h : scores.Nonempty then
    if maximize then scores.max' h else scores.min' h
  else
    0

/--
Fuel-indexed exact minimax. Recursive calls consume one unit exactly when one
tile is played. Immediate signed reward is emitted only on trick completion.
-/
def minimax : Nat → MidState → Int
  | 0, _ => 0
  | fuel + 1, st =>
      let scores :=
        st.legalMoves.image fun d =>
          let out := step st d
          out.reward + minimax fuel out.state
      chooseExtremum (onTeamZero st.actor) scores

/-- Dominoes ordered by their canonical numeric code `7*high + low`. -/
instance : LinearOrder Domino :=
  LinearOrder.lift'
    (fun d : Domino => 7 * (Domino.high d).val + (Domino.low d).val)
    (by
      intro a b h
      dsimp only at h
      have hla := (Domino.low a).isLt
      have hlb := (Domino.low b).isLt
      have hha : (Domino.high a).val = (Domino.high b).val := by omega
      have hlo : (Domino.low a).val = (Domino.low b).val := by omega
      exact Subtype.ext (Prod.ext_iff.mpr ⟨Fin.ext hha, Fin.ext hlo⟩))

/-- Every hand of a depth-one suffix has a computable unique tile. -/
def SuffixPos.onlyTile (X : SuffixPos 1) (s : Fin 4) : Domino :=
  (X.hands s).min' (by
    apply Finset.card_pos.mp
    rw [X.card_hands s]
    decide)

/-- The forced depth-one trick in clockwise play order. -/
def SuffixPos.forcedTrick (X : SuffixPos 1) : Trick :=
  fun i => X.onlyTile (seatAt X.leader i.val)

/-- Absolute seat winning the forced depth-one trick. -/
def SuffixPos.forcedWinnerSeat (X : SuffixPos 1) : Fin 4 :=
  seatAt X.leader (winner X.declaration X.forcedTrick).val

/-- Forced one-trick margin from team zero's perspective. -/
def SuffixPos.forcedMargin (X : SuffixPos 1) : Int :=
  signedAward X.forcedWinnerSeat (award X.forcedTrick)

/--
Exact suffix value. Depth zero is terminal. The depth-one equation is exposed
directly because there are no choices; deeper positions use the `4*k`-fuel
minimax above.
-/
def value : {k : Nat} → SuffixPos k → Int
  | 0, _ => 0
  | 1, X => X.forcedMargin
  | k + 2, X => minimax (4 * (k + 2)) (initialState X)

/-- At depth one the sole legal trick determines the complete minimax margin. -/
theorem value_k1_forced (X : SuffixPos 1) :
    value X =
      signedAward X.forcedWinnerSeat (award X.forcedTrick) := by
  rfl

private def pipTrumpK1 : SuffixPos 1 where
  declaration := .pipTrump 4
  hands :=
    ![{Domino.ofEnds 6 6 (by decide)},
      {Domino.ofEnds 6 5 (by decide)},
      {Domino.ofEnds 6 4 (by decide)},
      {Domino.ofEnds 4 4 (by decide)}]
  leader := 0
  card_hands := by decide
  disjoint_hands := by decide

-- Fours trump: seat 3's `4-4` wins 1 + 10 (`6-4`) = 11.
-- Seat 3 is on team one, so the team-zero margin is -11.
example : value pipTrumpK1 = -11 := by
  decide

private def noTrumpK1 : SuffixPos 1 where
  declaration := .noTrump
  hands :=
    ![{Domino.ofEnds 6 6 (by decide)},
      {Domino.ofEnds 5 0 (by decide)},
      {Domino.ofEnds 5 5 (by decide)},
      {Domino.ofEnds 6 5 (by decide)}]
  leader := 1
  card_hands := by decide
  disjoint_hands := by decide

-- Order from leader 1 is `5-0, 5-5, 6-5, 6-6`.
-- Seat 2 wins with `5-5`; 1 + 5 + 10 = 16, positive for team zero.
example : value noTrumpK1 = 16 := by
  decide

end Constellation
