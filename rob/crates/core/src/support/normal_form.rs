//! The exact support normal form: trichotomy, one-assignment SCC compiler,
//! decoder, linear ternary validator, and fiber-local rank/unrank.
//!
//! Implements Math §7.10–§7.12 (CELL-12..20, 25/26) and the Exec §15 normal
//! form contracts. INV-6 REACHABLE-IMPLIES-FEASIBLE: the `Empty`-inclusive
//! total type appears only on the external-validation path; the certified
//! path returns the nonempty type or panics.

use num_bigint::BigUint;
use num_traits::Zero;

use crate::domino::{DominoId, DominoSet};
use crate::support::cells::{AbstractCells, AbstractWorld, RuleDerivedCellSystem, HIDDEN_SEATS};
use crate::support::count::assignment_count;

/// One feasible assignment (a witness world) by slot-expanded augmenting
/// paths (Math §7.7 slot expansion). `None` exactly when the system is
/// infeasible.
pub fn feasible_world(cells: &AbstractCells) -> Option<AbstractWorld> {
    let n = cells.universe();
    let mut slots: Vec<usize> = Vec::new();
    for s in 0..HIDDEN_SEATS {
        for _ in 0..cells.capacity(s) {
            slots.push(s);
        }
    }
    debug_assert_eq!(slots.len(), n, "structural conservation");
    let mut slot_tile: Vec<Option<usize>> = vec![None; slots.len()];
    let mut tile_slot: Vec<Option<usize>> = vec![None; n];
    for tile in 0..n {
        let mut visited = vec![false; slots.len()];
        if !augment(
            cells,
            tile,
            &slots,
            &mut slot_tile,
            &mut tile_slot,
            &mut visited,
        ) {
            return None;
        }
    }
    let mut world: AbstractWorld = [Vec::new(), Vec::new(), Vec::new()];
    for (tile, slot) in tile_slot.iter().enumerate() {
        let seat = slots[slot.expect("all tiles matched")];
        world[seat].push(tile);
    }
    Some(world)
}

fn augment(
    cells: &AbstractCells,
    tile: usize,
    slots: &[usize],
    slot_tile: &mut Vec<Option<usize>>,
    tile_slot: &mut Vec<Option<usize>>,
    visited: &mut Vec<bool>,
) -> bool {
    for slot in 0..slots.len() {
        if visited[slot] || !cells.possible(slots[slot])[tile] {
            continue;
        }
        visited[slot] = true;
        let displaced = slot_tile[slot];
        if displaced.is_none()
            || augment(
                cells,
                displaced.expect("checked"),
                slots,
                slot_tile,
                tile_slot,
                visited,
            )
        {
            slot_tile[slot] = Some(tile);
            tile_slot[tile] = Some(slot);
            return true;
        }
    }
    false
}

/// The one-assignment SCC marginal-support compiler (CELL-15): orient used
/// holder edges `s → d` and unused allowed edges `d → s`; an unused edge is
/// marginally supported exactly when its endpoints share a strongly
/// connected component.
pub fn marginal_by_scc(
    cells: &AbstractCells,
    witness: &AbstractWorld,
) -> [Vec<bool>; HIDDEN_SEATS] {
    let n = cells.universe();
    let vertices = n + HIDDEN_SEATS;
    let mut holder = vec![usize::MAX; n];
    for (s, hand) in witness.iter().enumerate() {
        for &tile in hand {
            holder[tile] = s;
        }
    }
    // Adjacency: tile vertices 0..n, seat vertices n..n+3.
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); vertices];
    for tile in 0..n {
        for s in 0..HIDDEN_SEATS {
            if !cells.possible(s)[tile] {
                continue;
            }
            if holder[tile] == s {
                adjacency[n + s].push(tile); // used: s -> d
            } else {
                adjacency[tile].push(n + s); // unused allowed: d -> s
            }
        }
    }
    let component = tarjan_scc(&adjacency);
    core::array::from_fn(|s| {
        (0..n)
            .map(|tile| {
                cells.possible(s)[tile]
                    && (holder[tile] == s || component[tile] == component[n + s])
            })
            .collect()
    })
}

/// Iterative Tarjan strongly-connected components; returns a component id
/// per vertex.
fn tarjan_scc(adjacency: &[Vec<usize>]) -> Vec<usize> {
    let n = adjacency.len();
    let mut index = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut component = vec![usize::MAX; n];
    let mut next_index = 0usize;
    let mut components = 0usize;
    for root in 0..n {
        if index[root] != usize::MAX {
            continue;
        }
        // Explicit call stack: (vertex, next child position).
        let mut call: Vec<(usize, usize)> = vec![(root, 0)];
        while let Some(&mut (v, ref mut child)) = call.last_mut() {
            if *child == 0 {
                index[v] = next_index;
                low[v] = next_index;
                next_index += 1;
                stack.push(v);
                on_stack[v] = true;
            }
            if *child < adjacency[v].len() {
                let w = adjacency[v][*child];
                *child += 1;
                if index[w] == usize::MAX {
                    call.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                if low[v] == index[v] {
                    while let Some(w) = stack.pop() {
                        on_stack[w] = false;
                        component[w] = components;
                        if w == v {
                            break;
                        }
                    }
                    components += 1;
                }
                call.pop();
                if let Some(&(parent, _)) = call.last() {
                    low[parent] = low[parent].min(low[v]);
                }
            }
        }
    }
    component
}

/// The canonical tagged ambiguity component (Math §7.10; CELL-12/13).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Ambiguity {
    /// No ambiguous tiles; every residual capacity is zero.
    Determinate,
    /// Two active seats; every ambiguous tile is possible at both.
    Binary {
        /// The single inactive hidden seat; the active pair is its
        /// canonically ordered complement.
        inactive_seat: usize,
        /// The ambiguous pool, sorted by tile.
        pool: Vec<usize>,
        /// Residual capacity of the first (lower-index) active seat;
        /// `1 <= q < |pool|`.
        first_active_residual: usize,
    },
    /// Three active seats; a sparse partial exclusion map.
    Ternary {
        /// The ambiguous pool, sorted by tile.
        pool: Vec<usize>,
        /// Residual capacity of seat 0.
        residual0: usize,
        /// Residual capacity of seat 1 (`r2` follows by conservation).
        residual1: usize,
        /// `(tile, excluded seat)` pairs, sorted by tile; a missing tile
        /// means all three seats are possible.
        excluded_seat: Vec<(usize, usize)>,
    },
}

/// The feasible exact support normal form `N(C)` (Math §7.10; CELL-13).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FeasibleSupportNormalForm {
    /// Certain hidden-location marks `K_s`, sorted by tile.
    pub certain_by_seat: [Vec<usize>; HIDDEN_SEATS],
    /// The delimited remaining ambiguity.
    pub ambiguity: Ambiguity,
}

/// The total exact support normal form (CELL-14): `Empty` exists only for
/// the external-validation path (INV-6).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TotalSupportNormalForm {
    /// The unique state of every infeasible system under extensional
    /// support semantics.
    Empty,
    /// A nonempty exact support fiber.
    Feasible(FeasibleSupportNormalForm),
}

/// Compile the exact support normal form with one assignment plus one SCC
/// pass (CELL-15), on the external-validation path where `Empty` is a value
/// (CELL-14).
pub fn compile_total_support(
    cells: &AbstractCells,
    witness: Option<&AbstractWorld>,
) -> TotalSupportNormalForm {
    let owned;
    let witness = match witness {
        Some(w) => w,
        None => match feasible_world(cells) {
            Some(w) => {
                owned = w;
                &owned
            }
            None => return TotalSupportNormalForm::Empty,
        },
    };
    let marginal = marginal_by_scc(cells, witness);
    let mut certain: [Vec<usize>; HIDDEN_SEATS] = core::array::from_fn(|_| Vec::new());
    let mut pool = Vec::new();
    let mut holders: Vec<Vec<usize>> = Vec::new();
    #[allow(clippy::needless_range_loop)] // tile indexes three parallel seat vectors
    for tile in 0..cells.universe() {
        let a: Vec<usize> = (0..HIDDEN_SEATS).filter(|&s| marginal[s][tile]).collect();
        assert!(
            !a.is_empty(),
            "InvariantViolation: a pool tile of a feasible system has a holder"
        );
        if a.len() == 1 {
            certain[a[0]].push(tile);
        } else {
            pool.push(tile);
            holders.push(a);
        }
    }
    let residual: [usize; HIDDEN_SEATS] =
        core::array::from_fn(|s| cells.capacity(s) - certain[s].len());
    let active: Vec<usize> = (0..HIDDEN_SEATS).filter(|&s| residual[s] > 0).collect();
    let ambiguity = match active.len() {
        0 => {
            assert!(pool.is_empty(), "trichotomy: no active seats means no pool");
            Ambiguity::Determinate
        }
        2 => {
            let inactive_seat = (0..HIDDEN_SEATS)
                .find(|s| !active.contains(s))
                .expect("one inactive seat");
            for a in &holders {
                assert_eq!(*a, active, "binary tiles are possible at both active seats");
            }
            Ambiguity::Binary {
                inactive_seat,
                pool,
                first_active_residual: residual[active[0]],
            }
        }
        3 => {
            let mut excluded_seat = Vec::new();
            for (&tile, a) in pool.iter().zip(holders.iter()) {
                if a.len() == 2 {
                    let excluded = (0..HIDDEN_SEATS)
                        .find(|s| !a.contains(s))
                        .expect("one excluded seat");
                    excluded_seat.push((tile, excluded));
                }
            }
            Ambiguity::Ternary {
                pool,
                residual0: residual[0],
                residual1: residual[1],
                excluded_seat,
            }
        }
        _ => panic!(
            "InvariantViolation: active-seat trichotomy is 0/2/3, never {}",
            active.len()
        ),
    };
    TotalSupportNormalForm::Feasible(FeasibleSupportNormalForm {
        certain_by_seat: certain,
        ambiguity,
    })
}

/// Compile the exact support normal form of an internally certified state
/// (INV-6): an empty fiber is an internal error, never a value.
pub fn compile_exact_support(
    cells: &AbstractCells,
    witness: Option<&AbstractWorld>,
) -> FeasibleSupportNormalForm {
    match compile_total_support(cells, witness) {
        TotalSupportNormalForm::Feasible(nf) => nf,
        TotalSupportNormalForm::Empty => {
            panic!("InvariantViolation: an internally certified state has nonempty support (INV-6)")
        }
    }
}

impl FeasibleSupportNormalForm {
    /// The ambiguous pool `W`.
    pub fn ambiguous_pool(&self) -> &[usize] {
        match &self.ambiguity {
            Ambiguity::Determinate => &[],
            Ambiguity::Binary { pool, .. } | Ambiguity::Ternary { pool, .. } => pool,
        }
    }

    /// The reconstructed residual capacities.
    pub fn residuals(&self) -> [usize; HIDDEN_SEATS] {
        match &self.ambiguity {
            Ambiguity::Determinate => [0; HIDDEN_SEATS],
            Ambiguity::Binary {
                inactive_seat,
                pool,
                first_active_residual,
            } => {
                let active: Vec<usize> = (0..HIDDEN_SEATS).filter(|s| s != inactive_seat).collect();
                let mut r = [0; HIDDEN_SEATS];
                r[active[0]] = *first_active_residual;
                r[active[1]] = pool.len() - first_active_residual;
                r
            }
            Ambiguity::Ternary {
                pool,
                residual0,
                residual1,
                ..
            } => [*residual0, *residual1, pool.len() - residual0 - residual1],
        }
    }

    /// Decode to the unique feasible reduced cell system (CELL-13 decoder).
    /// The pool must be a contiguous `0..n` tile range (as in every
    /// structural cell system; the native wrapper maps identities).
    pub fn decode(&self) -> AbstractCells {
        let mut tiles: Vec<usize> = self.ambiguous_pool().to_vec();
        for k in &self.certain_by_seat {
            tiles.extend_from_slice(k);
        }
        let universe = tiles.len();
        {
            let mut sorted = tiles.clone();
            sorted.sort_unstable();
            assert_eq!(
                sorted,
                (0..universe).collect::<Vec<_>>(),
                "abstract decode requires a contiguous pool"
            );
        }
        let residual = self.residuals();
        let mut possible: [Vec<bool>; HIDDEN_SEATS] =
            core::array::from_fn(|_| vec![false; universe]);
        for (s, k) in self.certain_by_seat.iter().enumerate() {
            for &tile in k {
                possible[s][tile] = true;
            }
        }
        match &self.ambiguity {
            Ambiguity::Determinate => {}
            Ambiguity::Binary {
                inactive_seat,
                pool,
                ..
            } => {
                for &tile in pool {
                    for (s, p) in possible.iter_mut().enumerate() {
                        if s != *inactive_seat {
                            p[tile] = true;
                        }
                    }
                }
            }
            Ambiguity::Ternary {
                pool,
                excluded_seat,
                ..
            } => {
                for &tile in pool {
                    let excluded = excluded_seat
                        .iter()
                        .find(|&&(t, _)| t == tile)
                        .map(|&(_, s)| s);
                    for (s, p) in possible.iter_mut().enumerate() {
                        if Some(s) != excluded {
                            p[tile] = true;
                        }
                    }
                }
            }
        }
        let capacity = core::array::from_fn(|s| self.certain_by_seat[s].len() + residual[s]);
        AbstractCells::new(universe, possible, capacity)
            .expect("decoded normal form is structurally valid")
    }

    /// Well-formedness validation (Exec §15 contracts), including the linear
    /// ternary validator (CELL-20) — three comparisons, no matching search.
    pub fn well_formed(&self) -> bool {
        // Certain sets pairwise disjoint and disjoint from the pool.
        let pool = self.ambiguous_pool();
        for (i, a) in self.certain_by_seat.iter().enumerate() {
            if a.iter().any(|t| pool.contains(t)) {
                return false;
            }
            for b in self.certain_by_seat.iter().skip(i + 1) {
                if a.iter().any(|t| b.contains(t)) {
                    return false;
                }
            }
        }
        let residual = self.residuals();
        let capacity: Vec<usize> = (0..HIDDEN_SEATS)
            .map(|s| self.certain_by_seat[s].len() + residual[s])
            .collect();
        if capacity.iter().any(|&k| k > 7) {
            return false;
        }
        let hidden_pool = pool.len() + self.certain_by_seat.iter().map(Vec::len).sum::<usize>();
        if hidden_pool > 21 || capacity.iter().sum::<usize>() != hidden_pool {
            return false;
        }
        match &self.ambiguity {
            Ambiguity::Determinate => residual == [0; HIDDEN_SEATS],
            Ambiguity::Binary {
                inactive_seat,
                pool,
                first_active_residual,
            } => {
                *inactive_seat < HIDDEN_SEATS
                    && !pool.is_empty()
                    && (1..pool.len()).contains(first_active_residual)
            }
            Ambiguity::Ternary {
                pool,
                excluded_seat,
                ..
            } => {
                let n = pool.len();
                let r = self.residuals();
                if r.contains(&0) || r.iter().sum::<usize>() != n {
                    return false;
                }
                if excluded_seat
                    .iter()
                    .any(|&(t, s)| !pool.contains(&t) || s >= HIDDEN_SEATS)
                {
                    return false;
                }
                // The linear ternary validator (CELL-20):
                // n − n_s ≥ r_s + 1 for each seat, written strictly.
                let mut n_s = [0usize; HIDDEN_SEATS];
                for &(_, s) in excluded_seat {
                    n_s[s] += 1;
                }
                (0..HIDDEN_SEATS).all(|s| n - n_s[s] > r[s])
            }
        }
    }
}

/// The linear ternary signature validator (CELL-20; Math §7.12.1 complete
/// six-integer signature): positive conserved residuals with
/// `n − n_s ≥ r_s + 1` per seat and a nonnegative unrestricted category.
pub fn ternary_signature_valid(r: [usize; 3], n_excluded: [usize; 3]) -> bool {
    let n: usize = r.iter().sum();
    let excluded: usize = n_excluded.iter().sum();
    // `n − n_s ≥ r_s + 1` per seat, written strictly.
    r.iter().all(|&x| (1..=7).contains(&x))
        && excluded <= n
        && (0..3).all(|s| n - n_excluded[s] > r[s])
}

/// Fiber-local world rank (CELL-26): the exact position of one world in the
/// deletion-recurrence order of its fiber. Tiles are consumed in canonical
/// order; at each step the rank accumulates the counts of subtrees at
/// earlier seats.
pub fn rank_world(cells: &AbstractCells, world: &AbstractWorld) -> BigUint {
    let mut rank = BigUint::zero();
    let mut current = cells.clone();
    let mut remaining: AbstractWorld = world.clone();
    while current.universe() > 0 {
        let holder = (0..HIDDEN_SEATS)
            .find(|&s| remaining[s].contains(&0))
            .expect("world assigns every pool tile");
        for s in 0..holder {
            if current.capacity(s) > 0 && current.possible(s)[0] {
                rank += assignment_count(&current.removal_update(s, 0).expect("allowed successor"));
            }
        }
        current = current
            .removal_update(holder, 0)
            .expect("world edge is allowed");
        remaining = core::array::from_fn(|s| {
            remaining[s]
                .iter()
                .filter(|&&t| !(s == holder && t == 0))
                .map(|&t| t - 1)
                .collect()
        });
    }
    rank
}

/// Inverse of [`rank_world`] on the same fiber ordering.
pub fn unrank_world(cells: &AbstractCells, rank: &BigUint) -> AbstractWorld {
    let mut world: AbstractWorld = [Vec::new(), Vec::new(), Vec::new()];
    let mut offsets: Vec<usize> = (0..cells.universe()).collect();
    let mut current = cells.clone();
    let mut remaining = rank.clone();
    while current.universe() > 0 {
        let mut chosen = None;
        for s in 0..HIDDEN_SEATS {
            if current.capacity(s) == 0 || !current.possible(s)[0] {
                continue;
            }
            let successor = current.removal_update(s, 0).expect("allowed successor");
            let block = assignment_count(&successor);
            if remaining < block {
                chosen = Some((s, successor));
                break;
            }
            remaining -= block;
        }
        let (seat, successor) = chosen.expect("rank lies inside the fiber");
        world[seat].push(offsets[0]);
        offsets.remove(0);
        current = successor;
    }
    assert!(remaining.is_zero(), "rank lies inside the fiber");
    world
}

/// A native (domino-identity) support normal form: the abstract normal form
/// transported along the canonical pool order (Exec §15 payload).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NativeSupportNormalForm {
    /// Certain hidden-location marks per hidden seat.
    pub certain_by_seat: [DominoSet; HIDDEN_SEATS],
    /// The abstract ambiguity over pool positions, together with the pool
    /// tile order mapping positions to identities.
    pub abstract_form: FeasibleSupportNormalForm,
    /// Pool tiles in canonical identity order (abstract tile `i` is
    /// `tile_order[i]`).
    pub tile_order: Vec<DominoId>,
}

/// Compile the native exact support of an internally certified mechanical
/// state (INV-6: panics on an empty fiber — never a value on this path).
pub fn native_compile_exact_support(cells: &RuleDerivedCellSystem) -> NativeSupportNormalForm {
    let (abstract_cells, tile_order) = cells.to_abstract();
    let abstract_form = compile_exact_support(&abstract_cells, None);
    let certain_by_seat = core::array::from_fn(|s| {
        DominoSet::from_ids(
            abstract_form.certain_by_seat[s]
                .iter()
                .map(|&t| tile_order[t]),
        )
    });
    NativeSupportNormalForm {
        certain_by_seat,
        abstract_form,
        tile_order,
    }
}

/// External-validation path (CELL-14): the `Empty`-inclusive total form for
/// foreign systems; `None` denotes the single `Empty` state.
pub fn native_validate_support(cells: &RuleDerivedCellSystem) -> Option<NativeSupportNormalForm> {
    let (abstract_cells, _) = cells.to_abstract();
    match compile_total_support(&abstract_cells, None) {
        TotalSupportNormalForm::Empty => None,
        TotalSupportNormalForm::Feasible(_) => Some(native_compile_exact_support(cells)),
    }
}
