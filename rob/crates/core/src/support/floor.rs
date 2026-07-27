//! The x:001 reachable-support floor: admissible modules, witness-mask
//! languages, and the exact upward-closure counter.
//!
//! Exchange tier throughout (x:001, CONFIRMED 2026-07-27,
//! `exchange/README.md`): re-implemented from inbox/001's prose proof
//! steps 1–14 and JSON tables — never from its Python (BRIEF_SLICE_02 §5).
//! rob's receipts are independent conformance evidence for the certified
//! disjoint family and the [35,46] interval; the wiki owns any framing
//! change.
//!
//! Mask convention: a 28-bit integer indexes tile identities for
//! *enumeration and counting only* (the same style as the S7 context-mask
//! sweeps); sets-first remains the primary game representation.

use std::collections::HashSet;

use crate::algebra::trick::Play;
use crate::algebra::{algebra_for, DeclarationAlgebra};
use crate::declaration::Declaration;
use crate::domino::{all_ids, DominoId, DominoSet};
use crate::pip::PIPS;
use crate::seat::Seat;

/// The eight star declarations of the module language (x:001 step 2): the
/// seven pip trumps and doubles trump (their called suits are the eight
/// vertex stars of K₈).
pub fn star_declarations() -> [Declaration; 8] {
    [
        Declaration::PipTrump(PIPS[0]),
        Declaration::PipTrump(PIPS[1]),
        Declaration::PipTrump(PIPS[2]),
        Declaration::PipTrump(PIPS[3]),
        Declaration::PipTrump(PIPS[4]),
        Declaration::PipTrump(PIPS[5]),
        Declaration::PipTrump(PIPS[6]),
        Declaration::DoublesTrump,
    ]
}

fn mask_of(set: &DominoSet) -> u32 {
    let mut mask = 0u32;
    for d in set.iter() {
        mask |= 1 << d.index();
    }
    mask
}

/// All admissible `r`-groups of one declaration (x:001 step 3): `r`-tile
/// subsets of one effective follow set containing at least one tile of
/// that context's lead fiber.
pub fn admissible_groups(algebra: &DeclarationAlgebra, r: usize) -> Vec<DominoSet> {
    let mut out = Vec::new();
    for q in algebra.lead_contexts() {
        let members: Vec<DominoId> = all_ids().filter(|&d| algebra.follows(d, q)).collect();
        let fiber = algebra.lead_fiber(q);
        let mut chosen = Vec::new();
        subsets(&members, r, &mut chosen, &mut |subset| {
            if subset.iter().any(|&d| fiber.contains(d)) {
                out.push(DominoSet::from_ids(subset.iter().copied()));
            }
        });
    }
    out
}

fn subsets(
    items: &[DominoId],
    r: usize,
    chosen: &mut Vec<DominoId>,
    f: &mut impl FnMut(&[DominoId]),
) {
    if chosen.len() == r {
        f(chosen);
        return;
    }
    let need = r - chosen.len();
    let start = chosen
        .last()
        .map(|last| items.iter().position(|d| d == last).expect("member") + 1)
        .unwrap_or(0);
    if items.len() - start < need {
        return;
    }
    for i in start..items.len() {
        chosen.push(items[i]);
        subsets(items, r, chosen, f);
        chosen.pop();
    }
}

/// `x-r_flo_modules`, part 1 (x:001 step 3): every admissible four-group of
/// every star declaration can be played as a complete trick with any
/// desired winner — verified through `resolve_trick`. Returns the number of
/// declaration/group/winner cases (8 · 119 · 4 = 3,808).
pub fn module_winner_cases() -> u64 {
    let mut cases = 0u64;
    for declaration in star_declarations() {
        let algebra = algebra_for(declaration);
        let groups = admissible_groups(&algebra, 4);
        assert_eq!(groups.len(), 119, "119 admissible four-groups per star");
        for group in &groups {
            let tiles: Vec<DominoId> = group.iter().collect();
            // The group's context and its unique maximum-key tile.
            let q = tiles
                .iter()
                .map(|&d| algebra.led_suit(d))
                .find(|&q| tiles.iter().all(|&d| algebra.follows(d, q)))
                .expect("an admissible group lies in one effective suit");
            let x = *tiles
                .iter()
                .max_by(|&&a, &&b| algebra.trick_key(a, q).cmp(&algebra.trick_key(b, q)))
                .expect("four tiles");
            let leadable = |d: DominoId| algebra.led_suit(d) == q;
            for winner in Seat::ALL {
                // x:001 step 3: if x is leadable, make the desired winner
                // the current leader (it leads x); otherwise give x to the
                // winner and a leadable group tile to a different leader.
                let (leader, x_holder) = if leadable(x) {
                    (winner, winner)
                } else {
                    (winner.next(), winner)
                };
                let lead_tile = if leadable(x) {
                    x
                } else {
                    tiles
                        .iter()
                        .copied()
                        .find(|&d| d != x && leadable(d))
                        .expect("an admissible group has a leadable tile")
                };
                let mut rest: Vec<DominoId> = tiles
                    .iter()
                    .copied()
                    .filter(|&d| d != x && d != lead_tile)
                    .collect();
                let plays: Vec<Play> = (0..4u8)
                    .map(|offset| {
                        let actor = leader.offset(offset);
                        let domino = if actor == leader {
                            lead_tile
                        } else if actor == x_holder {
                            x
                        } else {
                            rest.remove(0)
                        };
                        Play { actor, domino }
                    })
                    .collect();
                let result = algebra.resolve_trick(&plays).expect("legal trick");
                assert_eq!(result.winner, winner, "the desired winner wins");
                cases += 1;
            }
        }
    }
    cases
}

/// Distinct union masks of pairwise-disjoint admissible groups with the
/// given sizes, over all eight star declarations (x:001 step 9). Group
/// order within equal sizes is quotiented by the union.
pub fn witness_unions(pattern: &[usize]) -> HashSet<u32> {
    let mut unions = HashSet::new();
    for declaration in star_declarations() {
        let algebra = algebra_for(declaration);
        let group_masks: Vec<Vec<u32>> = pattern
            .iter()
            .map(|&r| admissible_groups(&algebra, r).iter().map(mask_of).collect())
            .collect();
        let mut stack = Vec::new();
        extend_unions(&group_masks, 0, 0, &mut stack, &mut unions);
    }
    unions
}

fn extend_unions(
    group_masks: &[Vec<u32>],
    depth: usize,
    union: u32,
    stack: &mut Vec<usize>,
    unions: &mut HashSet<u32>,
) {
    if depth == group_masks.len() {
        unions.insert(union);
        return;
    }
    // Avoid ordered duplicates among equal-size levels: enforce
    // nondecreasing indices when the sizes repeat.
    let start = if depth > 0 && group_masks[depth].len() == group_masks[depth - 1].len() {
        stack[depth - 1] + 1
    } else {
        0
    };
    for (index, &mask) in group_masks[depth].iter().enumerate().skip(start) {
        if union & mask != 0 {
            continue;
        }
        stack.push(index);
        extend_unions(group_masks, depth + 1, union | mask, stack, unions);
        stack.pop();
    }
}

/// The exact upward-closure counter (x:001 step 10): mark every witness
/// mask in a 2²⁸-bit table, take the subset-zeta (upward closure: bit `T`
/// set iff `T` contains a witness), and count members per popcount, with an
/// optional forbidden-tile mask and a per-suit popcount predicate.
pub struct UpwardClosure {
    bits: Vec<u64>,
}

impl UpwardClosure {
    /// Build the closure of a witness set.
    pub fn new(witnesses: &HashSet<u32>) -> UpwardClosure {
        let mut builder = ClosureBuilder::new();
        for &w in witnesses {
            builder.mark(w);
        }
        builder.finish()
    }

    /// Count closure members with exactly `size` set bits, none of the
    /// `forbidden` bits, and satisfying `predicate` on the full mask.
    pub fn count(&self, size: u32, forbidden: u32, predicate: &mut impl FnMut(u32) -> bool) -> u64 {
        let mut count = 0u64;
        for (word_index, &word) in self.bits.iter().enumerate() {
            if word == 0 {
                continue;
            }
            let mut w = word;
            while w != 0 {
                let bit = w.trailing_zeros();
                w &= w - 1;
                let mask = ((word_index as u32) << 6) | bit;
                if mask.count_ones() == size && mask & forbidden == 0 && predicate(mask) {
                    count += 1;
                }
            }
        }
        count
    }
}

/// A fragment kind of the trace templates (x:001 step 8).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FragmentKind {
    /// `pair`: a lead-fiber tile plus one outsider (one hidden void).
    Pair,
    /// `m1`: two following tiles (at least one leadable) plus one outsider
    /// (one hidden void).
    M1,
    /// `m2`: one leadable following tile plus two outsiders (two hidden
    /// voids).
    M2,
}

/// The context data a fragment is built against: the following set, its
/// lead fiber, and the outsider pool (tiles that do not follow), all as
/// 28-bit masks; `forbidden` is excluded from every witness (the natural
/// families' omitted edge).
#[derive(Clone, Copy, Debug)]
pub struct FragmentContext {
    /// The effective follow set of the void context.
    pub follow: u32,
    /// Its lead fiber.
    pub fiber: u32,
    /// Tiles that do not follow the context (excluding `forbidden`).
    pub outsiders: u32,
    /// A tile that must remain unseen (0 when none).
    pub forbidden: u32,
}

fn bits(mask: u32) -> Vec<u32> {
    (0..28)
        .filter(|&i| mask & (1 << i) != 0)
        .map(|i| 1u32 << i)
        .collect()
}

/// Mark every disjoint (modules + fragment) union into `builder`
/// (x:001 step 9): `n_modules` admissible four-groups of `algebra`
/// (avoiding the forbidden tile) plus one fragment in `context`.
pub fn mark_fragment_unions(
    algebra: &DeclarationAlgebra,
    n_modules: usize,
    kind: FragmentKind,
    context: &FragmentContext,
    builder: &mut ClosureBuilder,
) {
    let module_masks: Vec<u32> = admissible_groups(algebra, 4)
        .iter()
        .map(mask_of)
        .filter(|m| m & context.forbidden == 0)
        .collect();
    // Enumerate fragments as masks.
    let mut fragments: Vec<u32> = Vec::new();
    let fiber_bits = bits(context.fiber);
    let follow_bits = bits(context.follow);
    let outsider_bits = bits(context.outsiders);
    match kind {
        FragmentKind::Pair => {
            for &l in &fiber_bits {
                for &o in &outsider_bits {
                    fragments.push(l | o);
                }
            }
        }
        FragmentKind::M1 => {
            for (i, &f1) in follow_bits.iter().enumerate() {
                for &f2 in follow_bits.iter().skip(i + 1) {
                    if (f1 | f2) & context.fiber == 0 {
                        continue; // at least one leadable
                    }
                    for &o in &outsider_bits {
                        fragments.push(f1 | f2 | o);
                    }
                }
            }
        }
        FragmentKind::M2 => {
            for &l in &fiber_bits {
                for (i, &o1) in outsider_bits.iter().enumerate() {
                    for &o2 in outsider_bits.iter().skip(i + 1) {
                        fragments.push(l | o1 | o2);
                    }
                }
            }
        }
    }
    // Recursive disjoint module selection around each fragment.
    for &fragment in &fragments {
        mark_modules(&module_masks, n_modules, 0, fragment, builder);
    }
}

fn mark_modules(
    modules: &[u32],
    remaining: usize,
    start: usize,
    union: u32,
    builder: &mut ClosureBuilder,
) {
    if remaining == 0 {
        builder.mark(union);
        return;
    }
    for (i, &m) in modules.iter().enumerate().skip(start) {
        if union & m == 0 {
            mark_modules(modules, remaining - 1, i + 1, union | m, builder);
        }
    }
}

/// A 2²⁸-bit witness table under construction.
pub struct ClosureBuilder {
    bits: Vec<u64>,
    marked: u64,
}

impl Default for ClosureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClosureBuilder {
    /// An empty table.
    pub fn new() -> ClosureBuilder {
        ClosureBuilder {
            bits: vec![0u64; 1usize << 22],
            marked: 0,
        }
    }

    /// Mark one witness mask.
    pub fn mark(&mut self, mask: u32) {
        let word = (mask >> 6) as usize;
        let bit = 1u64 << (mask & 63);
        if self.bits[word] & bit == 0 {
            self.bits[word] |= bit;
            self.marked += 1;
        }
    }

    /// Distinct witness masks marked so far.
    pub fn marked(&self) -> u64 {
        self.marked
    }

    /// Run the 28-dimension subset-zeta (upward closure).
    pub fn finish(self) -> UpwardClosure {
        UpwardClosure::from_bits(self.bits)
    }
}

impl UpwardClosure {
    fn from_bits(mut bits: Vec<u64>) -> UpwardClosure {
        let words = bits.len();
        for b in 6..28 {
            let stride = 1usize << (b - 6);
            for i in 0..words {
                if i & stride != 0 {
                    bits[i] |= bits[i ^ stride];
                }
            }
        }
        for b in 0..6 {
            let shift = 1u32 << b;
            let mut keep = 0u64;
            for p in 0..64u32 {
                if p & shift == 0 {
                    keep |= 1u64 << p;
                }
            }
            for word in bits.iter_mut() {
                *word |= (*word & keep) << shift;
            }
        }
        UpwardClosure { bits }
    }
}
