//! The campaign target T: the complete experimental authority (parent §1).
//! Every number this crate reports names its target by `id()`.

use walt::rules::{ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt::solver::adaptive::{RootPosition, SlicePolicy};

/// Domain constant for the deal stream ("OGLD").
pub const DEAL_DOMAIN: u64 = 0x4F47_4C44;

/// The fixed other-player profile - part of the target's law; changing it
/// is a new target (parent §1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldKind {
    /// walt's `HashField`: hash-seeded uniform legal, O(1).
    HashLegal,
    /// walt's `Level0Field` (σ0): n0 no-void belief worlds from the frozen
    /// INNER_SEED derivation, best response against the Dice field - a pure
    /// function of seat, hand and record.
    Level0 { n0: usize },
    /// walt's maintained `GymField`: the learner's partner is the L1
    /// fixed-(w)/8 procedure, opponents are Level0Field(8); every choice is
    /// a pure function of public state (seed = mix(420600 ^ key digest)).
    Gym { partner_worlds: u64 },
}

#[derive(Clone, Debug)]
pub struct CampaignTarget {
    /// Human-readable target identity; frozen in every run record.
    pub id: String,
    pub bid: u32,
    pub bidder: Seat,
    /// The learner's seat(s): one shared coefficient vector, one lawful
    /// invocation per seat.
    pub learner_seats: Vec<Seat>,
    pub field: FieldKind,
}

impl CampaignTarget {
    /// og-v1: uniform seeded deals; S0 declares its longest pip suit at
    /// bid 30 and leads; the S0+S2 partnership is learned; S1+S3 play
    /// hash-seeded uniform legal (walt's `HashField`, a pure function of
    /// public state, so mirrored arms couple automatically).
    pub fn og_v1() -> Self {
        CampaignTarget {
            id: "og-v1/bid30-longest-pip/S0S2-learner/S1S3-hash-legal-v1".into(),
            bid: 30,
            bidder: Seat::S0,
            learner_seats: vec![Seat::S0, Seat::S2],
            field: FieldKind::HashLegal,
        }
    }

    /// og-v3: identical law except the fixed seats are the σ0 modeled mind
    /// (`Level0Field::new(8)` - the field the live level-1 player models).
    pub fn og_v3() -> Self {
        CampaignTarget {
            id: "og-v3/bid30-longest-pip/S0S2-learner/S1S3-level0-n8-v1".into(),
            bid: 30,
            bidder: Seat::S0,
            learner_seats: vec![Seat::S0, Seat::S2],
            field: FieldKind::Level0 { n0: 8 },
        }
    }

    /// og-v4: the maintained gym field. The learner holds S0 ALONE; S2 is
    /// the L1 fixed-40/8 partner, S1+S3 are Level0Field(8) - walt's
    /// `GymField::new(S0, 40)`. The expensive target of the multifidelity
    /// harness (parent §6).
    pub fn og_v4() -> Self {
        CampaignTarget {
            id: "og-v4/bid30-longest-pip/S0-learner/gym-field-l1p40-l0o8-v1".into(),
            bid: 30,
            bidder: Seat::S0,
            learner_seats: vec![Seat::S0],
            field: FieldKind::Gym { partner_worlds: 40 },
        }
    }

    /// og-v5: same lineup as og-v4 (the maintained gym field), but the
    /// declared proxy is the SAME architecture with a cheap L1 partner
    /// (GymField(S0, 4)) and promotion is anytime-valid (CE-T4/T5).
    pub fn og_v5() -> Self {
        CampaignTarget {
            id: "og-v5/bid30-longest-pip/S0-learner/gym-field-l1p40-l0o8-v1".into(),
            bid: 30,
            bidder: Seat::S0,
            learner_seats: vec![Seat::S0],
            field: FieldKind::Gym { partner_worlds: 40 },
        }
    }

    /// og-v5's declared cheap proxy: identical law, partner at n_outer = 4.
    pub fn og_v5_proxy() -> Self {
        CampaignTarget {
            id: "og-v5-proxy/bid30-longest-pip/S0-learner/gym-field-l1p4-l0o8-v1".into(),
            bid: 30,
            bidder: Seat::S0,
            learner_seats: vec![Seat::S0],
            field: FieldKind::Gym { partner_worlds: 4 },
        }
    }

    /// og-v4's declared cheap proxy: identical law and learner, but S1, S2
    /// and S3 all play Level0Field(8). Used for training, construction and
    /// the multifidelity cheap batches; NEVER for a direct target claim.
    pub fn og_v4_proxy() -> Self {
        CampaignTarget {
            id: "og-v4-proxy/bid30-longest-pip/S0-learner/S1S2S3-level0-n8-v1".into(),
            bid: 30,
            bidder: Seat::S0,
            learner_seats: vec![Seat::S0],
            field: FieldKind::Level0 { n0: 8 },
        }
    }

    /// The declared cheap proxy lineup, when this target has one.
    pub fn proxy(&self) -> Option<CampaignTarget> {
        if self.id.starts_with("og-v4/") {
            Some(Self::og_v4_proxy())
        } else if self.id.starts_with("og-v5/") {
            Some(Self::og_v5_proxy())
        } else {
            None
        }
    }

    /// Resolve a target from a recorded identity (state files).
    pub fn from_id(id: &str) -> Result<Self, String> {
        for t in [
            Self::og_v1(),
            Self::og_v3(),
            Self::og_v4(),
            Self::og_v4_proxy(),
            Self::og_v5(),
            Self::og_v5_proxy(),
        ] {
            if t.id == id {
                return Ok(t);
            }
        }
        Err(format!("unknown campaign target id {id:?}"))
    }

    /// The fixed seats' policy - constructed per deal; both variants are
    /// pure functions of public state, so coupling and replay hold.
    pub fn field_policy(&self) -> Box<dyn SlicePolicy> {
        match self.field {
            FieldKind::HashLegal => Box::new(walt::policy_search::HashField),
            FieldKind::Level0 { n0 } => Box::new(walt::solver::policy::Level0Field::new(n0)),
            FieldKind::Gym { partner_worlds } => {
                Box::new(walt::gym::GymField::new(self.learner_seats[0], partner_worlds))
            }
        }
    }

    pub fn declaring_team(&self) -> Team {
        self.bidder.team()
    }

    /// Uniform complete deal from a seed (the walt idiom, `policy_search::
    /// fixture` body, kept verbatim so deal identities match the project's
    /// discipline of domain-separated SplitMix64 streams).
    pub fn deal(&self, seed: u64) -> [DominoSet; 4] {
        let mut rng = walt::kernel::SplitMix64::new(seed ^ DEAL_DOMAIN);
        let mut deck: Vec<Domino> = DominoSet::FULL.iter().collect();
        rng.shuffle(&mut deck);
        let first: DominoSet = deck[..7].iter().copied().collect();
        let mut other: Vec<Domino> = DominoSet::FULL.difference(first).iter().collect();
        rng.shuffle(&mut other);
        let mut hands = [DominoSet::EMPTY; 4];
        hands[0] = first;
        for s in 1..4 {
            hands[s] = other[(s - 1) * 7..s * 7].iter().copied().collect();
        }
        hands
    }

    /// The declared declaration rule: the bidder's longest pip suit, ties to
    /// the higher pip. Deterministic, part of the target's law.
    pub fn declaration(&self, bidder_hand: DominoSet) -> Decl {
        let mut best: Option<(usize, Pip)> = None;
        for pip in Pip::ALL {
            let n = bidder_hand
                .iter()
                .filter(|d| d.hi() == pip || d.lo() == pip)
                .count();
            let better = match best {
                None => true,
                Some((bn, bp)) => n > bn || (n == bn && pip > bp),
            };
            if better {
                best = Some((n, pip));
            }
        }
        Decl::PipTrump(best.expect("seven pips scanned").1)
    }

    pub fn opening_root(&self, decl: Decl) -> RootPosition {
        RootPosition {
            decl,
            bid: self.bid,
            declaring_team: self.declaring_team(),
            leader: self.bidder,
            banked: [0, 0],
            trick_plays: Vec::new(),
            prior_played: DominoSet::EMPTY,
            voids: [ContextSet::EMPTY; 4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deals_partition_the_deck_and_are_seed_deterministic() {
        // Law: every deal is a 7-7-7-7 partition of all 28 tiles.
        let t = CampaignTarget::og_v1();
        for seed in [0u64, 1, 42, 999_999] {
            let hands = t.deal(seed);
            let mut union = DominoSet::EMPTY;
            for h in hands {
                assert_eq!(h.len(), 7);
                assert!(union.is_disjoint(h));
                union = union.union(h);
            }
            assert_eq!(union, DominoSet::FULL);
            assert_eq!(t.deal(seed), hands);
        }
        // PINNED strictness witness: two different seeds give different deals.
        assert_ne!(t.deal(1), t.deal(2));
    }

    #[test]
    fn declaration_is_the_longest_pip_suit_high_tie() {
        let t = CampaignTarget::og_v1();
        // A hand dense in sixes declares sixes.
        let hand: DominoSet = DominoSet::FULL
            .iter()
            .filter(|d| d.hi() == Pip::ALL[6])
            .collect();
        assert_eq!(t.declaration(hand), Decl::PipTrump(Pip::ALL[6]));
    }
}
