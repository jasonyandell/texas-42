//! Optional, bounded investigation of an existing player's count-offer choice.
//! The Scheme relation proposes alternatives; it supplies no reward bonus.
//! Root values use a uniform mechanical fiber and the declared GymField.
//! This is model-relative exact comparison, not a guarantee about live players.
use std::collections::BTreeMap;

use crate::gym::{self, ExerciseRoot, GymField};
use crate::rules::{legal_plays, Domino, DominoSet};
use crate::solver::adaptive::SlicePolicy;

use super::{Search, State, Work};

pub const ID: &str = "partner-count-review-v1";
pub const QUERY: &str = include_str!("../../../gym/queries/offer-count.scheme");
pub const MAX_WORLDS: u128 = 400;
pub const MAX_NODES: u64 = 100_000;

pub struct Review {
    pub choice: Domino,
    pub offers: DominoSet,
    pub values: BTreeMap<Domino, usize>,
    pub worlds: u128,
    pub status: &'static str,
    pub work: Work,
    pub field_id: String,
}

pub fn review(ex: &ExerciseRoot, baseline: Domino) -> Result<Review, String> {
    let viewer = ex.root.kernel().viewer();
    let legal = legal_plays(
        ex.position.decl,
        ex.root.kernel().viewer_hand(),
        ex.frame.led_context(),
    );
    if !legal.contains(baseline) {
        return Err("review baseline must be legal".into());
    }
    let mut result = Review {
        choice: baseline,
        offers: DominoSet::EMPTY,
        values: BTreeMap::new(),
        worlds: ex.root.count(),
        status: "inactive",
        work: Work::new(MAX_NODES),
        field_id: String::new(),
    };
    if viewer.team() != ex.position.declaring_team
        || ex.root.kernel().viewer_hand().len() > 3
        || legal.len() < 2
        || State::from_root(&ex.position)
            .success(&ex.position)
            .is_some()
    {
        return Ok(result);
    }
    let matched = gym::match_query(ex, QUERY, MAX_WORLDS, 1_000_000)?;
    assert!(
        matched.public,
        "deployment detector must remain viewer-only"
    );
    for (tile, mass) in matched.presence {
        assert_eq!(mass, result.worlds, "public predicate has uniform presence");
        result
            .offers
            .insert(Domino::from_index(tile).expect("query domino"));
    }
    if result.offers.is_empty() || result.offers.contains(baseline) {
        return Ok(result);
    }
    if result.worlds > MAX_WORLDS {
        result.status = "unresolved-world-cap";
        return Ok(result);
    }
    let field = GymField::new(viewer, 40);
    result.field_id = field.id().into();
    let mut search = Search::new(ex, &field);
    for world in ex.root.worlds() {
        search.append(world)?;
    }
    let mut candidates = result.offers;
    candidates.insert(baseline);
    result.values = search.compare_root(candidates, &mut result.work)?;
    let baseline_value = result.values[&baseline];
    let mut best = baseline_value;
    for (&tile, &value) in &result.values {
        if value > best {
            best = value;
            result.choice = tile;
        }
    }
    result.status = if result.choice == baseline {
        "retained"
    } else {
        "changed"
    };
    Ok(result)
}
