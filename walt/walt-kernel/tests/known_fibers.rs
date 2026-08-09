//! Fiber sizes cross-checked against the exp5 probe corpus.
//!
//! What is authoritative here is the internal agreement: the counting DP, the
//! enumeration, and `Kernel::contains` must all describe the same set, and the
//! receipt's actual deal must be in it.

mod common;

use common::{kernel_at, receipt, true_world, EXP5_FIBERS};
use walt_core::{Context, ContextSet, DominoSet};
use walt_kernel::{Kernel, World, HIDDEN_SEATS};

#[test]
fn counting_dp_matches_the_exp5_corpus() {
    let r = receipt();
    for &(hand_id, trick_no, expected, unconstrained) in EXP5_FIBERS {
        let (kernel, hand) = kernel_at(&r, hand_id, trick_no);
        assert_eq!(
            kernel.count(),
            expected,
            "h{hand_id}t{trick_no} ({}) fiber size",
            hand.decl
        );
        assert_eq!(
            kernel.unconstrained_count(),
            unconstrained,
            "h{hand_id}t{trick_no} unconstrained fiber size"
        );
        assert!(expected <= unconstrained, "voids can only cut");
        // Every viewer holds the trick's horizon; hidden capacities agree.
        let horizon = 8 - trick_no;
        assert_eq!(kernel.viewer_hand().len(), horizon);
        assert_eq!(kernel.pool().len(), HIDDEN_SEATS * horizon);
        for h in kernel.hidden() {
            assert_eq!(h.capacity, horizon);
        }
    }
    assert_eq!(EXP5_FIBERS.len(), 52);
}

#[test]
fn the_true_world_is_always_in_the_fiber() {
    let r = receipt();
    for &(hand_id, trick_no, _, _) in EXP5_FIBERS {
        let (kernel, hand) = kernel_at(&r, hand_id, trick_no);
        let world = true_world(&kernel, hand, trick_no);
        assert!(
            kernel.contains(&world),
            "h{hand_id}t{trick_no}: the receipt's own deal is outside its fiber"
        );
    }
}

/// Enumeration and the counting DP describe the same set. Restricted to the
/// horizon-2 and horizon-3 kernels so the test stays cheap; the DP is checked
/// against exp5 at every horizon above.
#[test]
fn enumeration_agrees_with_the_counting_dp() {
    let r = receipt();
    for &(hand_id, trick_no, expected, _) in EXP5_FIBERS {
        if trick_no < 5 {
            continue;
        }
        let (kernel, hand) = kernel_at(&r, hand_id, trick_no);
        let worlds: Vec<World> = kernel.worlds().collect();
        assert_eq!(worlds.len() as u128, expected, "h{hand_id}t{trick_no}");
        let distinct: std::collections::HashSet<_> = worlds.iter().map(|w| w.hands()).collect();
        assert_eq!(
            distinct.len(),
            worlds.len(),
            "h{hand_id}t{trick_no} repeats"
        );
        for w in &worlds {
            assert!(kernel.contains(w), "h{hand_id}t{trick_no}: {w:?}");
        }
        let truth = true_world(&kernel, hand, trick_no);
        assert_eq!(
            worlds.iter().filter(|w| **w == truth).count(),
            1,
            "h{hand_id}t{trick_no}: the true world appears once"
        );
    }
}

/// The brief's named pins, spelled out so a regression names itself.
#[test]
fn hand_0_trick_6_is_90_with_the_true_world_present() {
    let r = receipt();
    let (kernel, hand) = kernel_at(&r, 0, 6);
    assert_eq!(kernel.count(), 90);
    assert_eq!(kernel.unconstrained_count(), 90, "no voids bind here");
    assert!(kernel.contains(&true_world(&kernel, hand, 6)));
}

#[test]
fn hand_0_trick_5_is_1680_and_its_voids_are_vacuous() {
    let r = receipt();
    let (kernel, _) = kernel_at(&r, 0, 5);
    assert_eq!(kernel.count(), 1680);
    assert_eq!(kernel.unconstrained_count(), 1680);
    // All-trump tricks: nobody has failed to follow, so nothing is cut.
    for (i, h) in kernel.hidden().iter().enumerate() {
        assert_eq!(
            kernel.allowed(i),
            kernel.pool(),
            "{} is unconstrained",
            h.seat
        );
    }
}

#[test]
fn void_constrained_kernels_match_exp5() {
    let r = receipt();
    // kid: h2t6, h10t6, h8t6, h12t6, h3t5, h5t5.
    for &(hand_id, trick_no, expected) in &[
        (2usize, 6usize, 36u128),
        (10, 6, 19),
        (8, 6, 7),
        (12, 6, 6),
        (3, 5, 200),
        (5, 5, 560),
    ] {
        let (kernel, hand) = kernel_at(&r, hand_id, trick_no);
        assert_eq!(kernel.count(), expected, "h{hand_id}t{trick_no}");
        assert!(
            kernel.count() < kernel.unconstrained_count(),
            "h{hand_id}t{trick_no}: voids must bind here"
        );
        let bound: usize = kernel.hidden().iter().map(|h| h.voids.len()).sum();
        assert!(
            bound > 0,
            "h{hand_id}t{trick_no}: some void must be observed"
        );
        assert!(kernel.contains(&true_world(&kernel, hand, trick_no)));
    }
}

/// A void cuts by effective incidence, not by raw pip: a seat shown void in
/// natural context `p` may still hold the called tile carrying pip `p`,
/// because absorption already removed it from that natural context.
#[test]
fn a_natural_void_does_not_bar_absorbed_tiles_carrying_that_pip() {
    let r = receipt();
    let mut absorbed_tiles_checked = 0usize;
    for &(hand_id, trick_no, _, _) in EXP5_FIBERS {
        let (kernel, _) = kernel_at(&r, hand_id, trick_no);
        for (i, h) in kernel.hidden().iter().enumerate() {
            if h.voids.contains(Context::Called) {
                continue;
            }
            for q in h.voids.iter() {
                let Context::Natural(p) = q else { continue };
                for d in kernel.pool().iter().filter(|d| kernel.decl().is_called(*d)) {
                    assert!(
                        kernel.allowed(i).contains(d),
                        "h{hand_id}t{trick_no}: {} void in {q} wrongly barred called {d}",
                        h.seat
                    );
                    if d.has(p) {
                        absorbed_tiles_checked += 1;
                    }
                }
            }
        }
    }
    assert!(
        absorbed_tiles_checked > 0,
        "the corpus never exercised an absorbed tile under a natural void"
    );
}

#[test]
fn a_kernel_with_an_impossible_void_has_an_empty_fiber() {
    let r = receipt();
    let (kernel, _) = kernel_at(&r, 0, 6);
    let mut hidden = *kernel.hidden();
    // Bar every context at one seat: nothing can sit there, and the capacity
    // cannot be met.
    hidden[0].voids = ContextSet::FULL;
    let starved = Kernel::new(
        kernel.decl(),
        kernel.viewer(),
        kernel.viewer_hand(),
        kernel.pool(),
        hidden,
    )
    .expect("a well-formed capacity system");
    assert_eq!(starved.allowed(0), DominoSet::EMPTY);
    assert_eq!(starved.count(), 0);
    assert_eq!(starved.worlds().count(), 0);
}

#[test]
fn kernel_construction_rejects_malformed_capacity_systems() {
    let r = receipt();
    let (kernel, _) = kernel_at(&r, 0, 6);
    let mut hidden = *kernel.hidden();
    hidden[0].capacity += 1;
    assert!(Kernel::new(
        kernel.decl(),
        kernel.viewer(),
        kernel.viewer_hand(),
        kernel.pool(),
        hidden,
    )
    .is_err());

    let mut hidden = *kernel.hidden();
    hidden[1].seat = kernel.viewer();
    assert!(Kernel::new(
        kernel.decl(),
        kernel.viewer(),
        kernel.viewer_hand(),
        kernel.pool(),
        hidden,
    )
    .is_err());

    assert!(Kernel::new(
        kernel.decl(),
        kernel.viewer(),
        kernel.viewer_hand(),
        kernel.pool().union(kernel.viewer_hand()),
        *kernel.hidden(),
    )
    .is_err());
}
