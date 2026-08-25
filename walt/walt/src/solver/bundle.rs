//! `solver::bundle` — the bundled world evaluator: one shared-tree walk
//! per candidate carrying a whole world set, terminal pmake outcomes
//! attributed PER MEMBER WORLD (§22's exact route, work-shared).
//!
//! EXPLORATORY tier. The primitive computes exactly what
//! `solver::calibrate::exact_set_outcomes` computes — `outcomes[k][w]`,
//! candidate `k`'s viewer-objective terminal make indicator on world `w`
//! — but instead of replaying every world separately to terminal
//! (`solver::adaptive::replay_viewer_success`, the loop the controller's
//! §11.5 escalation endpoint runs per world), it walks the play tree once
//! per candidate carrying the full index set, splitting only where an
//! observation distinguishes members. The shared-node idiom is
//! `solver::exposure`'s [`PublicExec`]/reach-walk skeleton (one public
//! history serves a whole set of worlds; per-world hands are derived
//! views, never stored), specialized to the CONTROLLER's semantics: at
//! focal plies the candidate's ONE choice is followed (not a max over
//! legal), and at decided or terminal states the Boolean outcome is
//! written to every member index instead of counted.
//!
//! LAWFULNESS (E-A15, `walt/CENSUS-RULINGS.md`: order, not set).
//! Changing the ORDER of evaluation is lawful; changing the SET is a
//! declared exclusion. Bundling shares work across the SAME world set:
//! every world of the declared set is attributed exactly once, none
//! skipped — the partition at every field node is asserted exhaustive
//! and disjoint over the incoming index set, and the walk asserts total
//! attribution before any table is returned (the bundled analogue of
//! "enumeration visits the whole fiber exactly once"). The
//! [`decided_success`] cutoff DECIDES, it does not skip: decidedness is
//! monotone in banked points, so the early-attributed Boolean is the
//! exact terminal indicator of every continuation of every member. The
//! exact escalation route consumes only wins totals, so per-world
//! attribution is strictly stronger than that consumer requires.
//!
//! PURITY FENCES (O22/O29, `walt/SCENARIO-PLAYER.md` §10): every policy
//! choice — focal and field alike — is a pure function of (own remaining
//! hand, public record). The bundle key is the public history. Worlds
//! sharing a public history present the SAME focal information state:
//! the focal hand is the kernel's viewer hand, constant across the whole
//! set (asserted at setup, world by world), so a focal choice CANNOT
//! differ within a bundle and is taken once from public data. Field
//! hands vary per world, so field plies partition the bundle by the
//! chosen tile. No identity-bearing certificates anywhere: outcomes
//! attribute through the projected public state and enumeration indices
//! only.
//!
//! This slice ADDS the primitive; nothing in production consumes it yet
//! (the controller's routing is untouched).

use std::collections::HashMap;

use crate::kernel::World;
use crate::rules::rules::legal_plays;
use crate::rules::{Domino, DominoSet, Seat};
use crate::solver::adaptive::{decided_success, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::exposure::PublicExec;

/// The fully attributed outcome table of one bundled evaluation, plus the
/// walk's exact sharing and cutoff statistics.
///
/// The fields are private and no constructor is public: an outcome table
/// can only come out of the bundled walk, which asserts every
/// (candidate, world) cell was attributed exactly once before this value
/// exists. Partial attribution is unrepresentable:
///
/// ```compile_fail
/// // No public constructor exposes partial attribution (E0451: the
/// // fields are private to `solver::bundle`).
/// let partial = walt::solver::bundle::BundledOutcomes {
///     outcomes: vec![vec![true]],
///     nodes: 1,
///     early_settled: 0,
///     terminal_settled: 1,
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BundledOutcomes {
    /// `outcomes[k][w]`: candidate `k`'s terminal pmake indicator on
    /// world `w` of the declared enumeration order.
    outcomes: Vec<Vec<bool>>,
    /// Bundle nodes visited across all candidates' walks (each visit is
    /// one public history expanded for one candidate). The per-world
    /// route's comparable figure is `m · |worlds| · plays-to-terminal`.
    nodes: u64,
    /// (candidate, world) cells settled by the decided cutoff BEFORE the
    /// terminal depth.
    early_settled: u64,
    /// Cells attributed at the terminal depth itself.
    terminal_settled: u64,
}

impl BundledOutcomes {
    /// The full table, candidate-major, in the declared world order.
    pub fn outcomes(&self) -> &[Vec<bool>] {
        &self.outcomes
    }

    /// Candidate `k`'s per-world outcome vector.
    pub fn candidate(&self, k: usize) -> &[bool] {
        &self.outcomes[k]
    }

    /// Candidate `k`'s wins total — the only figure the exact escalation
    /// route consumes; derived from the attribution, never stored twice.
    pub fn wins(&self, k: usize) -> u128 {
        u128::try_from(self.outcomes[k].iter().filter(|u| **u).count()).expect("fits")
    }

    /// Bundle nodes visited across all candidates (sharing statistic).
    pub fn nodes(&self) -> u64 {
        self.nodes
    }

    /// Cells the decided cutoff settled before terminal depth.
    pub fn early_settled(&self) -> u64 {
        self.early_settled
    }

    /// Cells attributed at terminal depth.
    pub fn terminal_settled(&self) -> u64 {
        self.terminal_settled
    }
}

/// The immutable context of one bundled walk: the public root frame, the
/// focal (viewer) seat and its constant hand, the declared world list,
/// the one field model, and the post-root play count to terminal.
struct BundleWalk<'a> {
    position: &'a RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    worlds: &'a [World],
    field: &'a dyn SlicePolicy,
    total: usize,
}

/// Exact integer counters of one evaluation's walks.
struct WalkStats {
    nodes: u64,
    early_settled: u64,
    terminal_settled: u64,
}

impl BundleWalk<'_> {
    /// Walk one candidate's shared tree from `exec`, carrying the member
    /// indices `idxs`, writing each member's Boolean outcome into `out`
    /// exactly once.
    fn run(
        &self,
        focal: &dyn SlicePolicy,
        exec: &PublicExec,
        idxs: &[u32],
        out: &mut [Option<bool>],
        stats: &mut WalkStats,
    ) {
        assert!(!idxs.is_empty(), "a bundle carries at least one world");
        stats.nodes += 1;
        let at_terminal = exec.history.len() == self.total;
        if let Some(u) = decided_success(self.position, self.viewer, exec.banked, at_terminal) {
            // The viewer-objective indicator is a constant of every
            // continuation of every member here (decidedness is monotone
            // in banked points): attribute the whole bundle and stop.
            for &i in idxs {
                let slot = &mut out[usize::try_from(i).expect("fits")];
                assert!(slot.is_none(), "every world is attributed exactly once");
                *slot = Some(u);
            }
            let settled = u64::try_from(idxs.len()).expect("fits");
            if at_terminal {
                stats.terminal_settled += settled;
            } else {
                stats.early_settled += settled;
            }
            return;
        }
        assert!(
            exec.history.len() < self.total,
            "the 42-point pool exhausts at terminal, so an undecided state has plays left"
        );
        let seat = exec.seat();
        let led = exec
            .plays
            .first()
            .map(|d| self.position.decl.led_context(*d));
        let record = exec.record(self.position);
        if seat == self.viewer {
            // The focal ply: the choice is a pure function of (viewer
            // hand, public record) — both constant across the bundle
            // (O22; the hand constancy is asserted at setup) — so ONE
            // choice serves every member and no split can occur here.
            let hand = self.viewer_hand.difference(exec.played_since());
            let legal = legal_plays(self.position.decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let tile = focal.choose(self.position.decl, hand, legal, &record);
            assert!(legal.contains(tile), "a policy chooses a legal tile");
            let mut child = exec.clone();
            child.play(self.position, tile);
            self.run(focal, &child, idxs, out, stats);
        } else {
            // A field ply: the per-world hand varies, so the bundle
            // partitions by the field's chosen tile (O29: the choice
            // reads the member's own hand and the shared public record,
            // nothing hidden). The record is constant at this node, so
            // the choice is a pure function of the member's remaining
            // hand alone — one field query per DISTINCT hand serves
            // every member holding it (E-A15: the same choices are made;
            // only redundant queries are shared away).
            let played = exec.played_since();
            let mut by_hand: HashMap<u32, Domino> = HashMap::new();
            let mut groups: Vec<(Domino, Vec<u32>)> = Vec::new();
            for &i in idxs {
                let hand = self.worlds[usize::try_from(i).expect("fits")]
                    .hand(seat)
                    .difference(played);
                let tile = match by_hand.get(&hand.bits()) {
                    Some(tile) => *tile,
                    None => {
                        let legal = legal_plays(self.position.decl, hand, led);
                        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
                        let tile = self.field.choose(self.position.decl, hand, legal, &record);
                        assert!(legal.contains(tile), "a policy chooses a legal tile");
                        by_hand.insert(hand.bits(), tile);
                        tile
                    }
                };
                match groups.iter_mut().find(|(t, _)| *t == tile) {
                    Some((_, group)) => group.push(i),
                    None => groups.push((tile, vec![i])),
                }
            }
            // Exhaustive and disjoint over the incoming index set: every
            // index joined exactly one group.
            assert_eq!(
                groups.iter().map(|(_, group)| group.len()).sum::<usize>(),
                idxs.len(),
                "a field partition covers the incoming bundle exactly once"
            );
            for (tile, group) in groups {
                let mut child = exec.clone();
                child.play(self.position, tile);
                self.run(focal, &child, &group, out, stats);
            }
        }
    }
}

/// The bundled exact route: enumerate the complete fiber once, then walk
/// each candidate's shared tree over it. `outcomes[k][w]` follows the
/// kernel's enumeration order — element-wise comparable to
/// `solver::calibrate::exact_set_outcomes` on the same root.
pub fn bundled_set_outcomes(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[&dyn SlicePolicy],
    field: &dyn SlicePolicy,
) -> BundledOutcomes {
    let worlds: Vec<World> = root.worlds().collect();
    assert_eq!(
        u128::try_from(worlds.len()).expect("fits"),
        root.count(),
        "the bundled exact route enumerates the whole fiber exactly once"
    );
    bundled_set_outcomes_declared(root, position, candidates, field, &worlds)
}

/// The walk over a DECLARED world list (E-A15: the caller's domain
/// declaration is explicit in the name — a subset here is the caller's
/// declared exclusion, never this module's). Every exact consumer goes
/// through [`bundled_set_outcomes`] instead. Within the declared list the
/// charter holds unchanged: every listed world is attributed exactly
/// once.
pub fn bundled_set_outcomes_declared(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[&dyn SlicePolicy],
    field: &dyn SlicePolicy,
    worlds: &[World],
) -> BundledOutcomes {
    assert!(!candidates.is_empty(), "an outcome table names a candidate");
    assert!(!worlds.is_empty(), "a declared world set holds a world");
    assert!(
        u32::try_from(worlds.len()).is_ok(),
        "an enumerable bundled world list fits u32 indices"
    );
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let viewer_hand = kernel.viewer_hand();
    for world in worlds {
        // The O22 consequence made structural: with the focal hand
        // constant and choices information-consistent, a focal choice is
        // a pure function of the public history and cannot differ within
        // any bundle sharing one.
        assert_eq!(
            world.hand(viewer),
            viewer_hand,
            "the focal hand is constant across the world set, so a bundle's \
             focal choice cannot differ across members (O22)"
        );
    }
    let total = viewer_hand.len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    let walk = BundleWalk {
        position,
        viewer,
        viewer_hand,
        worlds,
        field,
        total,
    };
    let idxs: Vec<u32> = (0..u32::try_from(worlds.len()).expect("fits")).collect();
    let mut stats = WalkStats {
        nodes: 0,
        early_settled: 0,
        terminal_settled: 0,
    };
    let mut outcomes: Vec<Vec<bool>> = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let mut out: Vec<Option<bool>> = vec![None; worlds.len()];
        let exec = PublicExec::start(position);
        walk.run(*candidate, &exec, &idxs, &mut out, &mut stats);
        outcomes.push(
            out.into_iter()
                .map(|slot| slot.expect("every world is attributed exactly once"))
                .collect(),
        );
    }
    assert_eq!(
        stats.early_settled + stats.terminal_settled,
        u64::try_from(candidates.len() * worlds.len()).expect("fits"),
        "the walk attributes each candidate-world cell exactly once"
    );
    BundledOutcomes {
        outcomes,
        nodes: stats.nodes,
        early_settled: stats.early_settled,
        terminal_settled: stats.terminal_settled,
    }
}
