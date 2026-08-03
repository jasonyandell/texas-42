/-
Copyright (c) 2026 Jason Yandell. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.
Authors: Jason Yandell
-/
import Texas42.Cells

/-!
# Texas 42 — a mechanical state is not the original information state

PA-F05 (Math §6.6): the deal-local perfect-recall information record
retains the full public auction; the mechanical/support coordinate
deliberately drops the losing action sequence. Two auctions with the
same shaker, bidder, winning bid, and declaration — differing only in
which losing seat bid 30 — are distinct perfect-recall information
states with identical mechanical projections. Information equality must
never be silently coarsened to mechanical equality.
-/

namespace Texas42

/-- PA-C01 (deal-local record, Math §6.2): the viewer's perfect-recall
information for one deal attempt — own dealt hand plus the complete
public prefix (auction and plays). -/
structure DealLocalInfo where
  viewer : Seat
  hand0 : Finset Domino
  auction : AuctionState
  declaration : Declaration
  plays : List Domino

/-- The mechanical/support projection (Math §6.4): keep the viewer
context and the replayed public record under the auction's contract;
drop the auction's losing action sequence. -/
def DealLocalInfo.mech (I : DealLocalInfo) :
    Option (ViewerCtx × PubState) :=
  (I.auction.result.bind fun r =>
    Contract.ofBid r.1 r.2 I.declaration).map fun K =>
  (⟨I.viewer, I.hand0⟩, PubState.replay K I.plays)

/-- PA-F05 (Math §6.6, constructed counterexample): two distinct
perfect-recall information states with equal mechanical projections —
same shaker, bidder, winning bid `P(31)`, and declaration, differing
only in which losing seat opened `P(30)`. Every player observed the
losing bid and remembers its actor, so the perfect-recall partition
separates what the mechanical projection merges. -/
theorem mech_not_injective :
    ∃ I₁ I₂ : DealLocalInfo, I₁ ≠ I₂ ∧ I₁.mech = I₂.mech := by
  refine ⟨⟨0, ∅, ⟨0, [.pass, .pass, .point 30, .point 31]⟩,
      .notrump, []⟩,
    ⟨0, ∅, ⟨0, [.point 30, .pass, .pass, .point 31]⟩,
      .notrump, []⟩, ?_, ?_⟩
  · intro h
    have := congrArg (fun I => I.auction.bids) h
    simp at this
  · rfl

end Texas42
