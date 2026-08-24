//! Cross-fiber pricing lawfulness tests (`viewer_fiber_evaluate` — the
//! cheap first-order UI detector of LEVEL2-PROBE.md). Exploratory-tier
//! machinery: these tests pin lawfulness, determinism, and the identity
//! "full support = the plain solver path", not strength. Small samples;
//! estimates, never receipts; not a P-A21 statement.

use std::sync::Arc;

use num_rational::BigRational;
use num_traits::{One, Zero};

use walt::rules::{Decl, Seat, Team};
use walt::solver::{
    mask_bits, mix, sample_belief, viewer_fiber_evaluate, Deadline, Field, Key, Shared, Solver,
    SplitMix64,
};

const N: usize = 8;
const N0: usize = 2;
const SEED: u64 = 0x9E37_79B9;

fn deal(hand_no: u64) -> [u32; 4] {
    let mut rng = SplitMix64(SEED ^ mix(hand_no));
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    [
        mask_slice(&tiles[0..7]),
        mask_slice(&tiles[7..14]),
        mask_slice(&tiles[14..21]),
        mask_slice(&tiles[21..28]),
    ]
}

fn root_key() -> Key {
    Key {
        played: 0,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    }
}

/// Viewer == actor at the opening lead: every option has full support
/// (the actor's own tiles are lawful leads in every world of its own
/// fiber), and each price equals the plain solver value of the child on
/// the same worlds — the support-filter/alive-set path is the identity
/// when nothing is filtered.
#[test]
fn own_fiber_full_support_matches_plain_solver() {
    let hands = deal(1);
    let dcl = Decl::NoTrump;
    let bid = 30u8;
    let actor = Seat::from_index(1).expect("seat 1");
    let hand = hands[1];
    let options = mask_bits(hand);
    let key = root_key();

    let mut rng = SplitMix64(SEED ^ 0xA5A5);
    let priced = viewer_fiber_evaluate(
        dcl, bid, actor, actor, hand, &options, &key, [7; 4], [0; 4], 0, 7, N, N0, 120, &mut rng,
    )
    .expect("no deadline in tests");

    // The same worlds, drawn by an identically-seeded rng, priced by the
    // plain solver path (alive = the root all-worlds set).
    let mut rng2 = SplitMix64(SEED ^ 0xA5A5);
    let worlds = sample_belief(1, hand, 0, [7; 4], [0; 4], N, &mut rng2);
    let deadline = Deadline::after(std::time::Duration::from_secs(120));
    let sh = Arc::new(Shared::new(dcl, bid, vec![N0], 0, 7, deadline));
    let maximize = actor.team() == Team::T1;
    let solver = Solver::new(
        sh,
        actor,
        hand,
        maximize,
        worlds,
        Vec::new(),
        Field::Level(0),
    );

    assert_eq!(priced.len(), options.len(), "every option priced");
    for (t, v, support) in &priced {
        assert_eq!(*support, N, "own tiles are lawful leads in every world");
        let v = v.as_ref().expect("supported option has a price");
        assert!(*v >= BigRational::zero() && *v <= BigRational::one());
        let tile = walt::rules::Domino::from_index(usize::from(*t)).expect("tile");
        let child = solver.child_after_play(&key, tile, 0);
        let plain = solver.solve(&child).expect("no deadline in tests");
        assert_eq!(*v, plain, "full support equals the plain solver value");
    }
    solver.flush_nodes();
}

/// Cross fiber (viewer = an opponent of the seat to act): supports are
/// partial, supported prices are probabilities, and the whole evaluation
/// is deterministic per (position, rng seed) — same call, same values.
#[test]
fn cross_fiber_lawful_and_deterministic() {
    let hands = deal(1);
    let dcl = Decl::NoTrump;
    let bid = 30u8;
    let actor = Seat::from_index(1).expect("seat 1");
    let viewer = Seat::from_index(2).expect("seat 2");
    let options = mask_bits(hands[1]);
    let key = root_key();

    let run = || {
        let mut rng = SplitMix64(SEED ^ 0xF1BE);
        viewer_fiber_evaluate(
            dcl, bid, actor, viewer, hands[2], &options, &key, [7; 4], [0; 4], 0, 7, N, N0, 120,
            &mut rng,
        )
        .expect("no deadline in tests")
    };
    let a = run();
    let b = run();
    assert_eq!(a.len(), options.len(), "every option priced");
    for ((ta, va, wa), (tb, vb, wb)) in a.iter().zip(&b) {
        assert_eq!((ta, va, wa), (tb, vb, wb), "deterministic per seed");
        assert!(*wa <= N, "support within the sample");
        match va {
            Some(v) => {
                assert!(*wa > 0, "a priced option has supporting worlds");
                assert!(*v >= BigRational::zero() && *v <= BigRational::one());
            }
            None => assert_eq!(*wa, 0, "None price exactly at empty support"),
        }
    }
    // The actor's tiles are hidden from this viewer, so at least one
    // option must have partial (not full) support at any modest n; all-
    // full support would mean the filter never fired. (With N=8 worlds
    // and 7 unseen-tile options this is astronomically certain; if the
    // sampler ever changes enough to break it, look here first.)
    assert!(
        a.iter().any(|(_, _, w)| *w < N),
        "some option has partial support from the opponent's fiber"
    );
}
