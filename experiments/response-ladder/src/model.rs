//! Frozen lower-rung adapter and public-information-only outer sampling.
use crate::mechanics::*;
use std::sync::Arc;
use walt::rules::{Decl, Seat};
use walt::solver::{self, Deadline, Shared, Solver, SplitMix64, FULL_MASK};

pub fn revision(levels: [usize; 4], n0: usize, n1: usize) -> String {
    format!("v34-fixed-voidless-{levels:?}-n0={n0}-n1={n1}")
}

pub struct FrozenModel {
    solver: Solver,
    identity: String,
    levels: [usize; 4],
    decl: Decl,
    bid: u8,
    last_batch_modeled_queries: usize,
}
impl FrozenModel {
    pub fn new(problem: &Problem, levels: [usize; 4], n0: usize, n1: usize,
               deadline: Deadline) -> Result<Self, String> {
        if n0 == 0 || n1 == 0 || levels.iter().any(|&l| l > 1) { return Err("model requires positive n0/n1 and levels 0/1".into()); }
        let identity = revision(levels, n0, n1);
        if identity != problem.field_revision { return Err("frozen field revision mismatch".into()); }
        let (boundary_played, size, _) = frame(&problem.root)?;
        let shared = Arc::new(Shared::new(problem.decl, problem.bid, vec![n0, n1],
            boundary_played, size, deadline));
        // modeled_choice samples from the queried mind's own hand and public
        // frame. No outer scenarios are supplied to this adapter's Solver.
        let solver = Solver::new(shared, Seat::from_index(problem.viewer as usize).unwrap(),
            problem.hand, problem.viewer % 2 == 1, vec![], vec![], solver::Field::Dice);
        Ok(Self { solver, identity, levels, decl: problem.decl, bid: problem.bid,
            last_batch_modeled_queries: 0 })
    }
}
impl Field for FrozenModel {
    fn revision(&self) -> &str { &self.identity }
    fn choose(&mut self, q: FieldQuery<'_>, budget: &mut Budget) -> Option<u8> {
        budget.tick()?;
        assert_eq!((q.decl, q.bid), (self.decl, self.bid));
        assert_eq!(q.public.actor(), q.seat);
        let legal = q.public.legal(q.decl, q.hand);
        if legal == 0 { return None; }
        if legal.count_ones() == 1 { return Some(legal.trailing_zeros() as u8); }
        // The latent outer Dice tape is intentionally absent from this call.
        let tile = self.solver.modeled_choice(self.levels[q.seat as usize], &q.public.key(),
            Seat::from_index(q.seat as usize).unwrap(), q.hand, legal)?;
        if budget.deadline.passed() { return None; }
        assert!(legal & (1 << tile) != 0);
        Some(tile)
    }
    #[cfg(feature = "gpu")]
    fn choose_batch(&mut self, queries: &[OwnedFieldQuery], budget: &mut Budget) -> Option<Vec<u8>> {
        use rayon::prelude::*;
        use std::collections::HashMap;
        self.last_batch_modeled_queries = 0;
        // Reserve controller work before releasing independent exact queries.
        for _ in queries { budget.tick()?; }
        // A frozen v34 mind ignores outer tape and uses the complete legacy Key,
        // including both banked scores. The instance owns decl/bid/config/boundary.
        // Keep the generic GPU cache history-sensitive; this quotient is specific
        // to the declared frozen model and is not a viewer information quotient.
        let mut ids = HashMap::new();
        let mut unique = Vec::new();
        let mut fanout = Vec::with_capacity(queries.len());
        for q in queries {
            assert_eq!((q.decl,q.bid), (self.decl,self.bid));
            assert_eq!(q.public.actor(),q.seat);
            let key=(self.levels[q.seat as usize],q.seat,q.hand,q.public.key());
            let at=*ids.entry(key).or_insert_with(|| { let at=unique.len();unique.push(q);at });
            fanout.push(at);
        }
        self.last_batch_modeled_queries = unique.len();
        let deadline = budget.deadline;
        let values: Vec<Option<u8>> = unique.par_iter().map(|q| {
            if deadline.passed() { return None; }
            assert_eq!((q.decl,q.bid), (self.decl,self.bid));
            assert_eq!(q.public.actor(),q.seat);
            let legal=q.public.legal(q.decl,q.hand);
            if legal==0 { return None; }
            let tile=if legal.count_ones()==1 { legal.trailing_zeros() as u8 } else {
                self.solver.modeled_choice(self.levels[q.seat as usize], &q.public.key(),
                    Seat::from_index(q.seat as usize).unwrap(),q.hand,legal)?
            };
            if deadline.passed() { None } else { Some(tile) }
        }).collect();
        if deadline.passed() { return None; }
        let values:Vec<u8> = values.into_iter().collect::<Option<_>>()?;
        Some(fanout.into_iter().map(|at|values[at]).collect())
    }
    fn last_batch_modeled_queries(&self) -> Option<usize> { Some(self.last_batch_modeled_queries) }
}

/// Boundary identity for frozen pi plus remaining per-seat capacities.
pub fn frame(public: &PublicState) -> Result<(u32, usize, [usize; 4]), String> {
    if public.leader >= 4 || public.plays.len() > 3 { return Err("invalid partial trick".into()); }
    let mut partial = 0u32;
    for &t in &public.plays {
        if t >= 28 || partial & (1 << t) != 0 { return Err("invalid partial tiles".into()); }
        partial |= 1 << t;
    }
    if partial & public.played != partial { return Err("partial tiles absent from played mask".into()); }
    let boundary = public.played & !partial;
    let count = boundary.count_ones() as usize;
    if count > 28 || count % 4 != 0 { return Err("invalid trick boundary".into()); }
    let size = 7 - count / 4;
    let mut sizes = [size; 4];
    for i in 0..public.plays.len() {
        let seat = (public.leader as usize + i) % 4;
        sizes[seat] = sizes[seat].checked_sub(1).ok_or("no tile for partial play")?;
    }
    Ok((boundary, size, sizes))
}

/// Historical shuffle-and-reject outer stream, bounded at each attempt.
/// The sampler receives no actual opponent hands. All worlds precede tapes.
#[allow(clippy::too_many_arguments)]
pub fn sample_problem(decl: Decl, bid: u8, viewer: u8, hand: u32,
    public: &PublicState, n: usize, seed: u64, field_revision: String,
    budget: &mut Budget) -> Result<Option<Problem>, String> {
    if viewer >= 4 || public.actor() != viewer || n == 0 || !(1..=42).contains(&bid) { return Err("invalid sampling request".into()); }
    let (_, _, sizes) = frame(public)?;
    if hand.count_ones() as usize != sizes[viewer as usize] || hand & public.played != 0 || hand & !FULL_MASK != 0 {
        return Err("viewer hand disagrees with public frame".into());
    }
    solver::belief_frame_feasibility(viewer as usize, hand, public.played, sizes, public.voids)
        .map_err(|e| format!("infeasible outer belief: {e:?}"))?;
    let mut rng = SplitMix64(seed);
    let mut unseen = tiles(FULL_MASK & !public.played & !hand);
    let mut scenarios = Vec::with_capacity(n);
    while scenarios.len() < n {
        if budget.tick().is_none() { return Ok(None); }
        for i in (1..unseen.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            unseen.swap(i, j);
        }
        let mut hands = [0; 4];
        hands[viewer as usize] = hand;
        let mut offset = 0;
        let mut accepted = true;
        for s in 0..4 {
            if s == viewer as usize { continue; }
            hands[s] = unseen[offset..offset + sizes[s]].iter().fold(0, |m, &t| m | (1 << t));
            offset += sizes[s];
            if hands[s] & public.voids[s] != 0 { accepted = false; break; }
        }
        if accepted { scenarios.push(Scenario { hands, tape: 0, weight: 1 }); }
    }
    for scenario in &mut scenarios { scenario.tape = rng.next_u64(); }
    if budget.deadline.passed() { return Ok(None); }
    let problem = Problem { decl, bid, viewer, hand, root: public.clone(), scenarios, field_revision };
    problem.validate()?;
    Ok(Some(problem))
}
