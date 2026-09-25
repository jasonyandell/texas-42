//! EXPLORATORY offline Nel-O counterexample/refinement instrument.
//!
//! Reuses the deployed contract, legal moves, modeled policy and sampled
//! information-set recurrence. The ordinary chooser does not call this module;
//! the explicitly enabled Nel-O preview review reuses it.
//! Policies are extracted on a discovery bundle, keyed by PUBLIC tile history,
//! and completed off that tree by the fixed own/public L0 policy. Examination
//! outcomes never mutate a policy. Every discovery world is replayed to check
//! extraction parity before a plan can be used.
//!
//! A failing trace refutes this policy, not every policy. `doom` separately
//! searches ALL focal continuations in one revealed world against the SAME
//! declared field. This adapts doom.rs's singleton quantifiers to Contract::Nello;
//! it does not port the counted-class census or assign probability to witnesses.

use super::*;
use std::collections::BTreeMap;

pub type World = [u32; 4];

#[derive(Clone)]
pub struct Root {
    pub key: Key,
    pub viewer: Seat,
    pub hand: u32,
    pub original_hand: u32,
    pub voids: [u32; 4],
    pub contract: Contract,
}

impl Root {
    pub fn legal(&self) -> Vec<u8> {
        mask_bits(mask_of(legal_plays(
            Decl::DoublesSuit,
            set_of(self.hand),
            led(&self.key),
        )))
    }

    /// Bounded, mechanically conditioned sampling. The empty-void stream is
    /// byte-for-byte the deployed shuffle stream; rejection attempts are capped.
    pub fn sample(&self, n: usize, seed: u64, deadline: Deadline) -> Option<Vec<World>> {
        let sizes = self.contract.sizes(&self.key, 0, 7);
        belief_frame_feasibility(
            self.viewer.index(),
            self.hand,
            self.key.played,
            sizes,
            self.voids,
        )
        .ok()?;
        let mut rng =
            SplitMix64(seed ^ mix(u64::from(self.original_hand)) ^ record_hash(&self.key));
        let mut tiles = mask_bits(FULL_MASK & !self.key.played & !self.hand);
        let others: Vec<usize> = (0..4).filter(|&s| s != self.viewer.index()).collect();
        let mut worlds = Vec::with_capacity(n);
        for attempt in 0..n.checked_mul(100_000)? {
            if attempt & 255 == 0 && deadline.passed() {
                return None;
            }
            for i in (1..tiles.len()).rev() {
                let j = rng.below((i + 1) as u64) as usize;
                tiles.swap(i, j);
            }
            let mut w = [0; 4];
            w[self.viewer.index()] = self.hand;
            let mut offset = 0;
            for &s in &others {
                w[s] = tiles[offset..offset + sizes[s]]
                    .iter()
                    .fold(0, |m, &t| m | (1u32 << t));
                offset += sizes[s];
            }
            if (0..4).all(|s| w[s] & self.voids[s] == 0) {
                worlds.push(w);
            }
            if worlds.len() == n {
                return Some(worlds);
            }
        }
        None
    }

    fn valid_world(&self, w: &World) -> bool {
        let sizes = self.contract.sizes(&self.key, 0, 7);
        let mut used = self.key.played;
        for s in 0..4 {
            if w[s].count_ones() as usize != sizes[s] || w[s] & (used | self.voids[s]) != 0 {
                return false;
            }
            used |= w[s];
        }
        used == FULL_MASK && w[self.viewer.index()] == self.hand
    }
}

fn led(key: &Key) -> Option<Context> {
    key.plays
        .first()
        .map(|&t| Decl::DoublesSuit.led_context(Domino::from_index(t as usize).unwrap()))
}

#[derive(Clone)]
pub struct Plan {
    pub action: u8,
    /// Declarer make mass / discovery bundle size, exactly as the live solver.
    pub declarer_value: BigRational,
    decisions: BTreeMap<Vec<u8>, u8>,
}

impl Plan {
    pub fn decision_count(&self) -> usize {
        self.decisions.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub set: bool,
    pub plays: Vec<(usize, u8)>,
    pub viewer_decisions: usize,
    pub fallback_decisions: usize,
    pub viewer_free_decisions: usize,
    pub fallback_free_decisions: usize,
}

pub struct Lab {
    pub root: Root,
    sh: Arc<Shared>,
    actor: Solver,
}

impl Lab {
    pub fn new(root: Root, n0: usize, deadline: Deadline) -> Option<Self> {
        if n0 == 0
            || !root.contract.is_nello()
            || root.viewer.team() != Team::T0
            || root.contract.terminal(&root.key).is_some()
            || root
                .contract
                .actor(root.key.leader as usize, root.key.plays.len())
                != root.viewer
            || root.key.voids.is_some()
            || root.key.alive != 0
        {
            return None;
        }
        let sh = Arc::new(
            Shared::new(Decl::DoublesSuit, 1, vec![n0], 0, 7, deadline)
                .with_contract(root.contract),
        );
        let actor = Solver::new(
            Arc::clone(&sh),
            root.viewer,
            root.hand,
            false,
            Vec::new(),
            Vec::new(),
            Field::Level(0),
        );
        Some(Self { root, sh, actor })
    }

    pub fn deadline(&self) -> Deadline {
        self.sh.deadline
    }

    fn check(&self) -> Option<()> {
        if self.sh.deadline.passed() || self.sh.dead.load(Ordering::Relaxed) {
            None
        } else {
            Some(())
        }
    }

    /// The only off-tree completion and field callback. No host world argument.
    pub fn modeled_action(&self, key: &Key, seat: Seat, own_hand: u32) -> Option<u8> {
        self.check()?;
        let legal = mask_of(legal_plays(Decl::DoublesSuit, set_of(own_hand), led(key)));
        if legal.count_ones() == 1 {
            Some(legal.trailing_zeros() as u8)
        } else {
            self.actor.pi(0, key, seat, own_hand, legal)
        }
    }

    fn solver(&self, worlds: Vec<World>) -> Option<Solver> {
        self.check()?;
        if worlds.is_empty() || !worlds.iter().all(|w| self.root.valid_world(w)) {
            return None;
        }
        Some(Solver::new(
            Arc::clone(&self.sh),
            self.root.viewer,
            self.root.hand,
            false,
            worlds,
            Vec::new(),
            Field::Level(0),
        ))
    }

    pub fn train(&self, worlds: &[World]) -> Option<Vec<Plan>> {
        let solver = self.solver(worlds.to_vec())?;
        let mut plans = Vec::new();
        for action in self.root.legal() {
            let child =
                solver.child_after_play(&self.root.key, Domino::from_index(action as usize)?, 0);
            let value = solver.solve(&child)?;
            let mut plan = Plan {
                action,
                declarer_value: value,
                decisions: BTreeMap::new(),
            };
            self.extract(&solver, &child, &mut vec![action], &mut plan)?;
            let makes = worlds.iter().try_fold(0usize, |n, w| {
                Some(n + usize::from(!self.replay(&plan, w)?.set))
            })?;
            if plan.declarer_value != BigRational::new(makes.into(), worlds.len().into()) {
                return None;
            }
            plans.push(plan);
        }
        solver.flush_nodes();
        Some(plans)
    }

    fn extract(
        &self,
        solver: &Solver,
        key: &Key,
        history: &mut Vec<u8>,
        plan: &mut Plan,
    ) -> Option<()> {
        self.check()?;
        if self.root.contract.terminal(key).is_some() {
            return Some(());
        }
        let seat = self
            .root
            .contract
            .actor(key.leader as usize, key.plays.len());
        if seat == self.root.viewer {
            let target = solver.solve_count(key)?;
            let legal = legal_plays(
                Decl::DoublesSuit,
                set_of(self.root.hand & !key.played),
                led(key),
            );
            let mut storage = [(0, Domino::ALL[0]); Domino::COUNT];
            let order = solver.viewer_visit_order_into(key, led(key), legal, &mut storage);
            let mut chosen = None;
            for &(_, tile) in order {
                let child = solver.child_after_play(key, tile, key.alive);
                if solver.solve_count(&child)? == target {
                    chosen = Some((tile, child));
                    break;
                }
            }
            let (tile, child) = chosen?;
            if let Some(old) = plan.decisions.insert(history.clone(), tile.index() as u8) {
                assert_eq!(old, tile.index() as u8, "one move per observable history");
            }
            history.push(tile.index() as u8);
            self.extract(solver, &child, history, plan)?;
            history.pop();
        } else {
            let mut buckets: BTreeMap<u8, Vec<u32>> = BTreeMap::new();
            for sid in solver.alive_of(key.alive).iter() {
                let hand = solver.worlds[sid as usize][seat.index()] & !key.played;
                let tile = self.modeled_action(key, seat, hand)?;
                buckets.entry(tile).or_default().push(sid);
            }
            for (tile, ids) in buckets {
                let child = solver.child_after_play(
                    key,
                    Domino::from_index(tile as usize)?,
                    solver.intern(ids),
                );
                history.push(tile);
                self.extract(solver, &child, history, plan)?;
                history.pop();
            }
        }
        Some(())
    }

    pub fn replay(&self, plan: &Plan, world: &World) -> Option<Trace> {
        if !self.root.valid_world(world) || !self.root.legal().contains(&plan.action) {
            return None;
        }
        let mut key = self.root.key.clone();
        let mut history = Vec::new();
        let mut trace = Trace {
            set: false,
            plays: Vec::new(),
            viewer_decisions: 0,
            fallback_decisions: 0,
            viewer_free_decisions: 0,
            fallback_free_decisions: 0,
        };
        while self.root.contract.terminal(&key).is_none() {
            self.check()?;
            let seat = self
                .root
                .contract
                .actor(key.leader as usize, key.plays.len());
            let hand = world[seat.index()] & !key.played;
            let legal = legal_plays(Decl::DoublesSuit, set_of(hand), led(&key));
            let tile = if history.is_empty() {
                plan.action
            } else if seat == self.root.viewer {
                trace.viewer_decisions += 1;
                trace.viewer_free_decisions += usize::from(legal.len() > 1);
                if let Some(&tile) = plan.decisions.get(&history) {
                    tile
                } else {
                    trace.fallback_decisions += 1;
                    trace.fallback_free_decisions += usize::from(legal.len() > 1);
                    self.modeled_action(&key, seat, hand)?
                }
            } else {
                self.modeled_action(&key, seat, hand)?
            };
            let domino = Domino::from_index(tile as usize)?;
            if !legal.contains(domino) {
                return None;
            }
            trace.plays.push((seat.index(), tile));
            history.push(tile);
            self.root.contract.step(&mut key, Decl::DoublesSuit, domino);
        }
        trace.set = !self.root.contract.terminal(&key)?;
        Some(trace)
    }

    /// True means no focal defense after this action can set this world,
    /// even when the focal player knows the deal. Other seats stay modeled L0.
    /// This is a single-world, fixed-field fact, NOT a probability estimate.
    pub fn doom(&self, action: u8, world: &World) -> Option<bool> {
        if !self.root.legal().contains(&action) {
            return None;
        }
        let solver = self.solver(vec![*world])?;
        let child =
            solver.child_after_play(&self.root.key, Domino::from_index(action as usize)?, 0);
        let value = solver.solve_count(&child)?;
        solver.flush_nodes();
        Some(value == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Root, World) {
        let tile = |h: u32, l: u32| 1u32 << (h * (h + 1) / 2 + l);
        let hand = tile(6, 6) | tile(4, 4);
        let declarer = tile(5, 5) | tile(5, 0);
        let defender = tile(1, 1) | tile(1, 0);
        let unused = mask_bits(FULL_MASK & !(hand | declarer | defender));
        let inactive = unused[..7].iter().fold(0, |m, &t| m | (1u32 << t));
        let played = FULL_MASK & !(hand | declarer | defender | inactive);
        let root = Root {
            key: Key {
                voids: None,
                played,
                leader: 0,
                plays: vec![],
                banked_t1: 0,
                banked_t0: 0,
                alive: 0,
            },
            viewer: Seat::ALL[0],
            hand,
            original_hand: hand,
            voids: [0; 4],
            contract: Contract::Nello {
                declarer: Seat::ALL[1],
            },
        };
        (root, [hand, declarer, defender, inactive])
    }

    fn brute_can_set(lab: &Lab, world: &World, key: &Key) -> bool {
        if let Some(made) = lab.root.contract.terminal(key) {
            return !made;
        }
        let actor = lab
            .root
            .contract
            .actor(key.leader as usize, key.plays.len());
        let hand = world[actor.index()] & !key.played;
        let moves = if actor == lab.root.viewer {
            mask_bits(mask_of(legal_plays(
                Decl::DoublesSuit,
                set_of(hand),
                led(key),
            )))
        } else {
            vec![lab.modeled_action(key, actor, hand).unwrap()]
        };
        moves.into_iter().any(|t| {
            let mut next = key.clone();
            lab.root.contract.step(
                &mut next,
                Decl::DoublesSuit,
                Domino::from_index(t as usize).unwrap(),
            );
            brute_can_set(lab, world, &next)
        })
    }

    #[test]
    fn singleton_doom_matches_independent_focal_enumeration_and_known_double_trap() {
        let (root, world) = fixture();
        let lab = Lab::new(root, 8, Deadline::after(Duration::from_secs(10))).unwrap();
        assert_eq!(
            lab.doom(27, &world),
            Some(true),
            "6-6 safely unloads the only higher double"
        );
        assert_eq!(
            lab.doom(14, &world),
            Some(false),
            "4-4 forces declarer's 5-5 to win now"
        );
        for action in lab.root.legal() {
            let mut key = lab.root.key.clone();
            lab.root.contract.step(
                &mut key,
                Decl::DoublesSuit,
                Domino::from_index(action as usize).unwrap(),
            );
            assert_eq!(
                lab.doom(action, &world),
                Some(!brute_can_set(&lab, &world, &key))
            );
        }
    }

    #[test]
    fn extracted_plans_replay_exactly_and_hidden_world_does_not_enter_policy_guard() {
        let (root, world) = fixture();
        let lab = Lab::new(root, 8, Deadline::after(Duration::from_secs(10))).unwrap();
        let worlds = lab.root.sample(16, 71, lab.deadline()).unwrap();
        let plans = lab.train(&worlds).unwrap();
        for plan in &plans {
            let makes = worlds
                .iter()
                .filter(|w| !lab.replay(plan, w).unwrap().set)
                .count();
            assert_eq!(
                plan.declarer_value,
                BigRational::new(makes.into(), worlds.len().into())
            );
            for w in &worlds {
                assert_eq!(lab.replay(plan, w).unwrap().fallback_decisions, 0);
            }
        }
        // Sample membership is solver-private, never a field-policy input.
        let key = lab.root.key.clone();
        let mut different_membership = key.clone();
        different_membership.alive = 123;
        assert_eq!(
            lab.modeled_action(&key, lab.root.viewer, world[0]),
            lab.modeled_action(&different_membership, lab.root.viewer, world[0])
        );
        let prior = plans[0].decisions.clone();
        let _ = lab.replay(&plans[0], &world).unwrap();
        assert_eq!(
            prior, plans[0].decisions,
            "fresh-world replay cannot train the plan"
        );
    }

    #[test]
    fn invalid_worlds_and_expired_work_are_refused_not_scored() {
        let (root, mut world) = fixture();
        let lab = Lab::new(root.clone(), 8, Deadline::after(Duration::from_secs(10))).unwrap();
        world[3] &= world[3] - 1;
        assert!(
            lab.train(&[world]).is_none(),
            "inactive hand must retain seven tiles"
        );
        assert!(lab.doom(14, &world).is_none());
        let expired = Lab::new(root, 8, Deadline::after(Duration::ZERO)).unwrap();
        assert!(expired.train(&[]).is_none());
        assert!(expired.root.sample(1, 1, expired.deadline()).is_none());
    }
}
