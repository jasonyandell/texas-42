//! One deterministic selection schedule for real and modeled minds.
//!
//! The evaluator must return a COMPLETE vector on the requested candidates,
//! in the requested order, from one common bundle. Errors abort the selection;
//! neither partial values nor elapsed time participate in its ranking.
//! Values are sampled estimates. Racing is an exploratory elimination policy.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

use super::{best_of, binom_tail_leq, BlockRace};

pub type Values = Vec<(u8, BigRational)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Rule {
    #[default]
    Fixed,
    Refine,
    RaceRefine,
}

impl Rule {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Refine => "refine",
            Self::RaceRefine => "race-refine",
        }
    }
}

/// Blocks and bundles use the same lawful solver; the distinction lets callers
/// choose execution/cache layout without changing the selection mathematics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Batch {
    Bundle,
    Block,
}

pub struct Selected {
    pub choice: u8,
    /// Last completed estimates for the surviving comparison. These can come
    /// from different refinement stages, exactly as in the historical policy.
    pub values: Values,
    pub worlds: usize,
    pub batches: usize,
}

fn checked<E>(tiles: &[u8], result: Result<Values, E>) -> Result<Values, E> {
    let values = result?;
    assert!(
        values.iter().map(|(t, _)| t).eq(tiles.iter()),
        "complete ordered comparison"
    );
    assert!(
        values
            .iter()
            .all(|(_, v)| *v >= BigRational::zero() && *v <= BigRational::from_integer(1.into())),
        "contract values in [0,1]"
    );
    Ok(values)
}

fn tied(values: &[(u8, BigRational)], maximize: bool) -> Vec<u8> {
    let choice = best_of(values, maximize);
    let best = &values
        .iter()
        .find(|(t, _)| *t == choice)
        .expect("best is present")
        .1;
    values
        .iter()
        .filter(|(_, v)| v == best)
        .map(|(t, _)| *t)
        .collect()
}

pub fn select<E>(
    rule: Rule,
    tiles: &[u8],
    maximize: bool,
    n: usize,
    mut evaluate: impl FnMut(&[u8], usize, Batch) -> Result<Values, E>,
) -> Result<Selected, E> {
    assert!(n > 0 && !tiles.is_empty());
    assert!(
        tiles.windows(2).all(|w| w[0] < w[1]),
        "ascending unique candidates"
    );
    match rule {
        Rule::Fixed => {
            let values = checked(tiles, evaluate(tiles, n, Batch::Bundle))?;
            Ok(Selected {
                choice: best_of(&values, maximize),
                values,
                worlds: n,
                batches: 1,
            })
        }
        Rule::Refine => refine(tiles, maximize, n, evaluate),
        Rule::RaceRefine => race_refine(
            tiles,
            maximize,
            n.checked_mul(2).expect("sample cap"),
            n,
            evaluate,
        ),
    }
}

pub fn refine<E>(
    tiles: &[u8],
    maximize: bool,
    n: usize,
    mut evaluate: impl FnMut(&[u8], usize, Batch) -> Result<Values, E>,
) -> Result<Selected, E> {
    assert!(n > 0 && !tiles.is_empty());
    let cap = n.checked_mul(16).expect("refinement cap");
    let mut values = checked(tiles, evaluate(tiles, n, Batch::Bundle))?;
    let (mut size, mut worlds, mut batches) = (n, n, 1);
    loop {
        let ties = tied(&values, maximize);
        if ties.len() == 1 || size >= cap {
            break;
        }
        size *= 4;
        let next = checked(&ties, evaluate(&ties, size, Batch::Bundle))?;
        worlds += size;
        batches += 1;
        // Replacement, not pooling. Reconsider the full comparison after each
        // round: a formerly lower estimate may become the new leader.
        for (tile, value) in next {
            values
                .iter_mut()
                .find(|(t, _)| *t == tile)
                .expect("candidate")
                .1 = value;
        }
    }
    Ok(Selected {
        choice: best_of(&values, maximize),
        values,
        worlds,
        batches,
    })
}

pub fn race<E>(
    tiles: &[u8],
    maximize: bool,
    cap: usize,
    block: usize,
    mut evaluate: impl FnMut(&[u8], usize, Batch) -> Result<Values, E>,
) -> Result<BlockRace, E> {
    assert!(cap > 0 && block > 0 && !tiles.is_empty());
    if tiles.len() == 1 {
        return Ok(BlockRace {
            choice: tiles[0],
            worlds_used: 0,
            values: vec![],
            eliminated: vec![],
        });
    }
    let mut live = tiles.to_vec();
    let mut blocks: Vec<Vec<BigRational>> = vec![vec![]; tiles.len()];
    let slot = |t| tiles.iter().position(|&x| x == t).expect("candidate");
    let mut eliminated = vec![];
    let mut used = 0;
    while used < cap && live.len() > 1 {
        let size = block.min(cap - used);
        for (tile, value) in checked(&live, evaluate(&live, size, Batch::Block))? {
            blocks[slot(tile)].push(value);
        }
        used += size;
        let sums: Values = live
            .iter()
            .map(|&t| (t, blocks[slot(t)].iter().cloned().sum()))
            .collect();
        let leader = best_of(&sums, maximize);
        live.retain(|&rival| {
            if rival == leader {
                return true;
            }
            let (mut plus, mut minus) = (0, 0);
            for (lv, rv) in blocks[slot(leader)].iter().zip(&blocks[slot(rival)]) {
                if if maximize { lv > rv } else { lv < rv } {
                    plus += 1;
                } else if lv != rv {
                    minus += 1;
                }
            }
            let k = plus + minus;
            let remove = k >= 6 && binom_tail_leq(k, plus, 1, 128);
            if remove {
                eliminated.push((rival, used));
            }
            !remove
        });
    }
    let means: Values = live
        .iter()
        .map(|&t| {
            let b = &blocks[slot(t)];
            (
                t,
                b.iter().cloned().sum::<BigRational>()
                    / BigRational::from_integer(BigInt::from(b.len())),
            )
        })
        .collect();
    Ok(BlockRace {
        choice: best_of(&means, maximize),
        worlds_used: used,
        values: means.into_iter().map(|(t, v)| (t, v, used)).collect(),
        eliminated,
    })
}

pub fn race_refine<E>(
    tiles: &[u8],
    maximize: bool,
    cap: usize,
    n: usize,
    mut evaluate: impl FnMut(&[u8], usize, Batch) -> Result<Values, E>,
) -> Result<Selected, E> {
    let mut batches = 0;
    let raced = race(tiles, maximize, cap, 8, |ts, size, kind| {
        batches += 1;
        evaluate(ts, size, kind)
    })?;
    let values: Values = raced.values.into_iter().map(|(t, v, _)| (t, v)).collect();
    if values.len() <= 1 || tied(&values, maximize).len() == 1 {
        return Ok(Selected {
            choice: raced.choice,
            values,
            worlds: raced.worlds_used,
            batches,
        });
    }
    let mut result = refine(&tied(&values, maximize), maximize, n, evaluate)?;
    result.worlds += raced.worlds_used;
    result.batches += batches;
    Ok(result)
}
