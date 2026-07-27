//! The declaration-indexed relational algebra `A_δ` (Math §3).
//!
//! [`algebra_for`] selects the relational interpretation for one declaration
//! (Math §3.8: declaration is selection of a relational interpretation, not a
//! scalar feature).

pub mod order;
pub mod suits;
pub mod transport;
pub mod trick;

use crate::declaration::Declaration;
use crate::domino::{all_ids, DominoId, DominoSet, DOMINO_COUNT};
use order::{Rank, TrickKey};
use suits::{build_suit_tables, LedSuit, LedSuitSet, SuitTables, CONTEXT_COUNT};
use trick::{Play, TrickError, TrickResult};

/// The declaration algebra facade (Exec §5 `DeclarationAlgebra`).
///
/// All tables are pure functions of the declaration, precomputed from the
/// ingest definitions (Math §3.2–§3.5). The struct stores only the relational
/// data of `A_δ`; every query is a derived view of it.
pub struct DeclarationAlgebra {
    declaration: Declaration,
    called: DominoSet,
    powered: DominoSet,
    follow: [[bool; CONTEXT_COUNT]; DOMINO_COUNT],
    led_suit: [LedSuit; DOMINO_COUNT],
    rank: [Rank; DOMINO_COUNT],
}

/// Select the relational algebra of one declaration (`Sel_δ`, Math §3.8).
pub fn algebra_for(declaration: Declaration) -> DeclarationAlgebra {
    let SuitTables {
        called,
        powered,
        follow,
        led_suit,
    } = build_suit_tables(declaration);
    DeclarationAlgebra {
        declaration,
        called,
        powered,
        follow,
        led_suit,
        rank: order::build_ranks(declaration),
    }
}

impl DeclarationAlgebra {
    /// The selected declaration.
    pub fn declaration(&self) -> Declaration {
        self.declaration
    }

    /// The called set `κ_δ` (Math §3.2; ALG-05).
    pub fn called(&self) -> &DominoSet {
        &self.called
    }

    /// The powered set `π_δ` (Math §3.2; ALG-05).
    pub fn powered(&self) -> &DominoSet {
        &self.powered
    }

    /// Effective suits of one domino: the nonempty set
    /// `{q : d ∈ σ̂_q^δ}` (Math §3.3; ALG-06/07 called absorption).
    pub fn effective_suits(&self, id: DominoId) -> LedSuitSet {
        let mut suits = LedSuitSet::empty();
        for q in LedSuit::all() {
            if self.follow[id.index()][q.context_index()] {
                suits.insert(q);
            }
        }
        suits
    }

    /// Led suit `ℓ_δ(d)`: called context for a called domino, natural
    /// context of the higher pip otherwise (Math §3.4; R-PLAY-04/05).
    pub fn led_suit(&self, id: DominoId) -> LedSuit {
        self.led_suit[id.index()]
    }

    /// Follow relation `F_δ(d, q)`: effective-suit membership
    /// (Math §3.4; ALG-08; R-FOLLOW-01).
    pub fn follows(&self, id: DominoId, q: LedSuit) -> bool {
        self.follow[id.index()][q.context_index()]
    }

    /// Declaration-relative total rank `r_δ(d)` (Math §3.5; ALG-09).
    pub fn rank(&self, id: DominoId) -> Rank {
        self.rank[id.index()]
    }

    /// Contextual tier: 2 powered, 1 unpowered follower, 0 slough
    /// (Math §3.5; ALG-10).
    pub fn tier(&self, id: DominoId, q: LedSuit) -> u8 {
        if self.powered.contains(id) {
            2
        } else if self.follows(id, q) {
            1
        } else {
            0
        }
    }

    /// Lexicographic trick key `τ_δ(d, q)` with `(0,0)` sloughs
    /// (Math §3.5; ALG-11).
    pub fn trick_key(&self, id: DominoId, q: LedSuit) -> TrickKey {
        match self.tier(id, q) {
            0 => TrickKey::Slough,
            tier => TrickKey::Ranked {
                tier,
                rank: self.rank(id),
            },
        }
    }

    /// The seven leadable contexts `{ℓ_δ(d) : d ∈ D}` in context-index
    /// order (Math §7.13.2; REACH-05).
    pub fn lead_contexts(&self) -> Vec<LedSuit> {
        LedSuit::all()
            .into_iter()
            .filter(|&q| !self.lead_fiber(q).is_empty())
            .collect()
    }

    /// The lead fiber `{d : ℓ_δ(d) = q}` (Math §7.13.2; REACH-05).
    pub fn lead_fiber(&self, q: LedSuit) -> DominoSet {
        DominoSet::from_ids(all_ids().filter(|&id| self.led_suit(id) == q))
    }

    /// `BEATS_δ(q, d)`: exactly the dominoes whose contextual key exceeds
    /// `d`'s (Math §3.7; ALG-13).
    pub fn beats(&self, q: LedSuit, id: DominoId) -> DominoSet {
        let key = self.trick_key(id, q);
        DominoSet::from_ids(all_ids().filter(|&e| self.trick_key(e, q) > key))
    }

    /// When-led threat set `THREAT_δ(d) = BEATS_δ(ℓ_δ(d), d)`
    /// (Math §3.7; ALG-14/15 — an exact diagonal query, not a complete play
    /// ontology).
    pub fn threat(&self, id: DominoId) -> DominoSet {
        self.beats(self.led_suit(id), id)
    }

    /// Competitive-strength ordinal `ord_δ,q` (rec Math §7.16.1; PLAY-12/13):
    /// zero exactly for a slough; positive ordinals are the 1-based position
    /// in the trick-key order of the nonzero-tier dominoes of context `q`.
    /// Order-isomorphic to the trick key within each context; at most 13.
    pub fn competitive_ordinal(&self, q: LedSuit, id: DominoId) -> u8 {
        if self.tier(id, q) == 0 {
            return 0;
        }
        let key = self.trick_key(id, q);
        let below = all_ids()
            .filter(|&e| self.tier(e, q) > 0 && self.trick_key(e, q) < key)
            .count();
        (below + 1) as u8
    }

    /// Actor-preserving trick resolution (Exec §5; Math §3.6; ALG-12).
    ///
    /// Receives exactly four distinct dominoes with actors in clockwise
    /// order, derives the led suit from the first domino, requires a unique
    /// maximum trick key, and returns the winning actor together with
    /// `1 + sum(countPoints(d))` (R-SCORE-03).
    pub fn resolve_trick(&self, plays: &[Play]) -> Result<TrickResult, TrickError> {
        let plays: &[Play; 4] = plays.try_into().map_err(|_| TrickError::WrongLength)?;
        for i in 0..4 {
            for j in (i + 1)..4 {
                if plays[i].domino == plays[j].domino {
                    return Err(TrickError::DuplicateDomino);
                }
            }
            if plays[i].actor != plays[0].actor.offset(i as u8) {
                return Err(TrickError::MalformedActorSequence);
            }
        }
        let q = self.led_suit(plays[0].domino);
        let keys = plays.map(|p| self.trick_key(p.domino, q));
        let max = keys.iter().max().copied().unwrap_or(TrickKey::Slough);
        let mut winners = keys.iter().enumerate().filter(|&(_, &k)| k == max);
        let (winner_index, _) = winners.next().ok_or(TrickError::NoUniqueMaximum)?;
        if winners.next().is_some() {
            return Err(TrickError::NoUniqueMaximum);
        }
        let points = 1 + plays
            .iter()
            .map(|p| crate::domino::domino_from_id(p.domino).count_points())
            .sum::<u8>();
        Ok(TrickResult {
            winner: plays[winner_index].actor,
            points,
        })
    }
}
