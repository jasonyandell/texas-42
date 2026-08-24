use walt::rules::{legal_plays, Context, Decl, DominoSet, Seat};

use crate::{OpeningContext, OpeningError};

pub const MIN_POINT_BID_V1: u8 = 30;
pub const MAX_POINT_BID_V1: u8 = 41;

/// A checked ordinary point bid in the M1 opening-root profile.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PointBidV1(u8);

impl PointBidV1 {
    pub fn new(value: u8) -> Result<PointBidV1, OpeningError> {
        if !(MIN_POINT_BID_V1..=MAX_POINT_BID_V1).contains(&value) {
            return Err(OpeningError::PointBidOutOfRange { value });
        }
        Ok(PointBidV1(value))
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

/// Closed contract normal form for the first opening root.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OpeningContractV1 {
    PointBid(PointBidV1),
    Mark,
}

impl OpeningContractV1 {
    pub fn point_bid(value: u8) -> Result<OpeningContractV1, OpeningError> {
        PointBidV1::new(value).map(OpeningContractV1::PointBid)
    }

    pub const fn loss_budget(self) -> u8 {
        match self {
            OpeningContractV1::PointBid(bid) => 42 - bid.value(),
            OpeningContractV1::Mark => 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IgnoreAuctionEvidenceV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UniformCompatibleOpeningDealsV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UniformRandomLegalV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeclaringTeamMakesV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningStraightHand21FieldActionsV1;

/// The five fixed model identities carried by every `OpeningRootV1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningModelProfileV1 {
    auction_evidence: IgnoreAuctionEvidenceV1,
    prior: UniformCompatibleOpeningDealsV1,
    field: UniformRandomLegalV1,
    utility: DeclaringTeamMakesV1,
    horizon: OpeningStraightHand21FieldActionsV1,
}

pub const OPENING_MODEL_PROFILE_V1: OpeningModelProfileV1 = OpeningModelProfileV1 {
    auction_evidence: IgnoreAuctionEvidenceV1,
    prior: UniformCompatibleOpeningDealsV1,
    field: UniformRandomLegalV1,
    utility: DeclaringTeamMakesV1,
    horizon: OpeningStraightHand21FieldActionsV1,
};

impl OpeningModelProfileV1 {
    pub const fn auction_evidence(self) -> IgnoreAuctionEvidenceV1 {
        self.auction_evidence
    }

    pub const fn prior(self) -> UniformCompatibleOpeningDealsV1 {
        self.prior
    }

    pub const fn field(self) -> UniformRandomLegalV1 {
        self.field
    }

    pub const fn utility(self) -> DeclaringTeamMakesV1 {
        self.utility
    }

    pub const fn horizon(self) -> OpeningStraightHand21FieldActionsV1 {
        self.horizon
    }
}

/// The closed production root admitted by the M1 scalar reference.
///
/// One seat occupies all four public roles.  No history or current-trick
/// fields exist, so a constructed value is necessarily at the empty public
/// opening.  Auction reachability is deliberately outside this model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningRootV1 {
    decl: Decl,
    focal: Seat,
    focal_hand: DominoSet,
    contract: OpeningContractV1,
}

impl OpeningRootV1 {
    pub fn new(
        decl: Decl,
        focal: Seat,
        focal_hand: DominoSet,
        contract: OpeningContractV1,
    ) -> Result<OpeningRootV1, OpeningError> {
        if focal_hand.len() != 7 {
            return Err(OpeningError::OpeningHandSize {
                actual: focal_hand.len(),
            });
        }
        if legal_plays(decl, focal_hand, None) != focal_hand {
            return Err(OpeningError::OpeningHandNotFullyLegal);
        }
        let root = OpeningRootV1 {
            decl,
            focal,
            focal_hand,
            contract,
        };
        if root.loss_budget() > 12 {
            return Err(OpeningError::LossBudgetOutOfRange {
                value: root.loss_budget(),
            });
        }
        Ok(root)
    }

    pub const fn decl(self) -> Decl {
        self.decl
    }

    pub const fn focal(self) -> Seat {
        self.focal
    }

    pub const fn bidder(self) -> Seat {
        self.focal
    }

    pub const fn leader(self) -> Seat {
        self.focal
    }

    pub const fn actor(self) -> Seat {
        self.focal
    }

    pub const fn focal_hand(self) -> DominoSet {
        self.focal_hand
    }

    pub const fn contract(self) -> OpeningContractV1 {
        self.contract
    }

    pub const fn loss_budget(self) -> u8 {
        self.contract.loss_budget()
    }

    pub const fn public_play_count(self) -> usize {
        0
    }

    pub const fn current_trick_len(self) -> usize {
        0
    }

    pub const fn model_profile(self) -> OpeningModelProfileV1 {
        OPENING_MODEL_PROFILE_V1
    }

    pub fn hidden_pool(self) -> DominoSet {
        DominoSet::FULL.difference(self.focal_hand)
    }

    pub fn legal_leads(self) -> DominoSet {
        legal_plays(self.decl, self.focal_hand, None)
    }

    /// Distinct represented contexts in canonical `Context::index` order.
    pub fn led_contexts(self) -> Vec<Context> {
        let mut represented = [false; Context::COUNT];
        for lead in self.legal_leads().iter() {
            represented[self.decl.led_context(lead).index()] = true;
        }
        Context::ALL
            .into_iter()
            .filter(|context| represented[context.index()])
            .collect()
    }

    pub fn opening_context(self, led: Context) -> Result<OpeningContext, OpeningError> {
        OpeningContext::from_opening_hand(self.decl, self.focal_hand, led)
    }
}
