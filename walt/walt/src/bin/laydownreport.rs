//! EXPLORATORY LAYDOWN-REPORT INSTRUMENT (anytime proof-state Phase
//! 7; `walt/math/anytime_proof_state_score_v0.1.md` §15–§17/§64,
//! ruling APS-A9) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: the §16 typed census —
//! Laydown / ForcedMake(+witness) / AdversarialPolicyMake /
//! PolicyCertainMake — on the structural fixtures and the t6 receipt
//! roots, with walk-node counts and wall time. Never a play-strength
//! claim; the universal walk is an endgame instrument (declared
//! domain) and ranges over a per-seat relaxation of the world set —
//! sound for certification, possibly conservative.
//!
//! Modes:
//!   `laydownreport report <out.txt>` — the Phase 7 probe
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;
use std::time::Instant;

use walt::kernel::{Hidden, Kernel};
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Pip, Seat};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::factor_belief::SupportOracle;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::laydown::classify_root;
use walt::solver::policy::{DecisionMode, TieRule};

fn d(a: usize, b: usize) -> Domino {
    Domino::new(Pip::ALL[a], Pip::ALL[b])
}

fn set(tiles: &[Domino]) -> DominoSet {
    let mut s = DominoSet::EMPTY;
    for t in tiles {
        s.insert(*t);
    }
    s
}

fn level0_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn synthetic_root(
    decl: Decl,
    viewer_hand: &[Domino],
    pool: &[Domino],
    bid: u32,
    decl_banked: u32,
) -> (CanonicalRoot, RootPosition) {
    let viewer = Seat::ALL[0];
    let hand = set(viewer_hand);
    let pool = set(pool);
    let in_play = hand.union(pool);
    let points_in_play: u32 = 3 + in_play.iter().map(|t| t.count()).sum::<u32>();
    let banked_total = 42 - points_in_play;
    let hidden = [Seat::ALL[1], Seat::ALL[2], Seat::ALL[3]].map(|seat| Hidden {
        seat,
        capacity: 3,
        voids: ContextSet::EMPTY,
    });
    let kernel = Kernel::new(decl, viewer, hand, pool, hidden).expect("a lawful kernel");
    let position = RootPosition {
        decl,
        bid,
        declaring_team: viewer.team(),
        leader: viewer,
        banked: [decl_banked, banked_total - decl_banked],
        trick_plays: vec![],
        prior_played: DominoSet::FULL.difference(in_play),
        voids: [ContextSet::EMPTY; 4],
    };
    (CanonicalRoot::new(kernel), position)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: laydownreport report <out.txt>");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let low = FixedPreference::lowest_first("focal:lowest-first");
    writeln!(
        out,
        "laydown-report instrument (anytime proof-state Phase 7, §15–§17/§64/APS-A9)\n\
         census: laydown / forced(witness) / adversarial(lowest-first) / policy-certain.\n"
    )
    .unwrap();

    let boss = [d(6, 6), d(6, 5), d(6, 4)];
    let pool_a = [
        d(6, 3),
        d(6, 2),
        d(6, 1),
        d(6, 0),
        d(5, 4),
        d(5, 3),
        d(5, 2),
        d(5, 1),
        d(4, 3),
    ];
    let loose = [d(6, 5), d(6, 4), d(6, 3)];
    let pool_b = [
        d(6, 6),
        d(6, 2),
        d(6, 1),
        d(6, 0),
        d(5, 4),
        d(5, 3),
        d(5, 2),
        d(5, 1),
        d(4, 3),
    ];
    let fixtures: Vec<(&str, (CanonicalRoot, RootPosition))> = vec![
        (
            "boss-chain bid33",
            synthetic_root(Decl::PipTrump(Pip::ALL[6]), &boss, &pool_a, 33, 20),
        ),
        (
            "already-made bid20",
            synthetic_root(Decl::PipTrump(Pip::ALL[6]), &boss, &pool_a, 20, 20),
        ),
        (
            "loose-boss bid33",
            synthetic_root(Decl::PipTrump(Pip::ALL[6]), &loose, &pool_b, 33, 20),
        ),
    ];
    for (name, (root, position)) in &fixtures {
        let start = Instant::now();
        let c = classify_root(&oracle, root, position, &field, &low);
        writeln!(
            out,
            "{name}: laydown={} forced={}({:?}) adversarial={} policy-certain={} nodes={} wall {} us",
            c.laydown,
            c.forced_make,
            c.forced_witness,
            c.adversarial_policy_make,
            c.policy_certain_make,
            c.universal_nodes,
            start.elapsed().as_micros()
        )
        .unwrap();
    }

    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");
    for (hand_id, trick_no) in [(12usize, 6usize), (10, 6), (5, 6), (4, 6)] {
        let hand = &receipt.hands[hand_id];
        let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
        let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
        let root = CanonicalRoot::new(kernel);
        let start = Instant::now();
        let c = classify_root(&oracle, &root, &position, &field, &low);
        writeln!(
            out,
            "h{hand_id}-t{trick_no}: laydown={} forced={}({:?}) adversarial={} policy-certain={} nodes={} wall {} us",
            c.laydown,
            c.forced_make,
            c.forced_witness,
            c.adversarial_policy_make,
            c.policy_certain_make,
            c.universal_nodes,
            start.elapsed().as_micros()
        )
        .unwrap();
    }
}
