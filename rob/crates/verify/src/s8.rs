//! Stage S8 verification harness: unreachability regressions
//! (BRIEF_SLICE_02 §9, table S8).
//!
//! Exchange tier: `x-` rows draw on x:002 (CONFIRMED 2026-07-27). The
//! REACH-10 row is corpus-anchored (Math §7.13.5). Witness pool/NF data is
//! transcribed from the inbox/002 JSON as witness *data* (BRIEF_SLICE_02
//! §11.5); all logic is rob's own.

use rob_core::{
    algebra_for, all_ids, apply_auction_action, begin_deal_attempt, close_auction,
    contract_from_auction, natural_incidence, reduce, AbstractCells, Ambiguity, AuctionAction,
    BidValue, CloseAuctionOutcome, Contract, Declaration, Domino, DominoId, DominoSet,
    FeasibleSupportNormalForm, LedSuit, LedSuitSet, MatchState, Pip, Play, PointAmount,
    ReachabilityOuterNecessaryProfile, RulesConfig, Seat, SymbolicTraceCertificate,
    TotalSupportNormalForm, PIPS,
};

use crate::receipt::{fmt_commas, Receipt};

fn dom(h: u8, l: u8) -> DominoId {
    rob_core::domino_id(Domino::new(
        Pip::new(h).expect("pip"),
        Pip::new(l).expect("pip"),
    ))
}

/// One static generator at capacities (6,6,6) (Math §7.13.5): a
/// declaration together with either no void or one void context with a
/// nonempty hidden-seat membership pattern.
#[derive(Clone, Copy, Debug)]
pub struct StaticGenerator {
    /// The declaration.
    pub declaration: Declaration,
    /// The void context and its membership pattern bits (1..=7), or `None`
    /// for the no-void generator.
    pub void: Option<(LedSuit, u8)>,
}

/// Enumerate the 450 static generators: 9 declarations × (1 + 7 leadable
/// contexts × 7 nonempty membership patterns).
pub fn static_generators() -> Vec<StaticGenerator> {
    let mut out = Vec::new();
    for &declaration in &rob_core::GAME_DECLARATIONS {
        out.push(StaticGenerator {
            declaration,
            void: None,
        });
        let algebra = algebra_for(declaration);
        for q in algebra.lead_contexts() {
            for pattern in 1u8..8 {
                out.push(StaticGenerator {
                    declaration,
                    void: Some((q, pattern)),
                });
            }
        }
    }
    out
}

/// Decode one generator over a fixed pool through rob's own cells/NF
/// pipeline (Exec §18 decoder; never promoted to a certified type).
pub fn decode_generator(
    generator: &StaticGenerator,
    pool: &DominoSet,
) -> (TotalSupportNormalForm, ReachabilityOuterNecessaryProfile) {
    let mut void_masks = [
        LedSuitSet::empty(),
        LedSuitSet::empty(),
        LedSuitSet::empty(),
    ];
    if let Some((q, pattern)) = generator.void {
        for (seat, mask) in void_masks.iter_mut().enumerate() {
            if pattern & (1 << seat) != 0 {
                mask.insert(q);
            }
        }
    }
    let profile = ReachabilityOuterNecessaryProfile {
        declaration: generator.declaration,
        capacities: [6, 6, 6],
        void_masks,
        pool: *pool,
    };
    let (cells, _) = profile.decode_cells();
    (rob_core::compile_total_support(&cells, None), profile)
}

/// `r_unr_reach10` (REACH-10; Math §7.13.5): the corpus witness — 18-tile
/// pool `σ₀ ∪ doubles ∪ {2:1, 3:1, 3:2, 4:1, 4:2}` with `P₁ = U∖σ₀`,
/// `P₂ = P₃ = U`: Hall-feasible and already reduced; exactly 2 of the 450
/// generators decode to it, with lead-fiber sizes (7, 1) and every lead
/// tile inside the hidden pool (lead-witness failure ⇒ unreachable).
pub fn reach10_check() -> (u64, u64, (usize, usize)) {
    let pool = natural_incidence(PIPS[0])
        .union(&DominoSet::from_ids(
            PIPS.iter().map(|&p| rob_core::domino_id(Domino::new(p, p))),
        ))
        .union(&DominoSet::from_ids([
            dom(2, 1),
            dom(3, 1),
            dom(3, 2),
            dom(4, 1),
            dom(4, 2),
        ]));
    assert_eq!(pool.len(), 18);
    // The witness cells: P₁ = U ∖ σ₀, P₂ = P₃ = U, capacities (6,6,6).
    let tiles: Vec<DominoId> = pool.iter().collect();
    let sigma0 = natural_incidence(PIPS[0]);
    let possible: [Vec<bool>; 3] = [
        tiles.iter().map(|&d| !sigma0.contains(d)).collect(),
        vec![true; tiles.len()],
        vec![true; tiles.len()],
    ];
    let witness_cells =
        AbstractCells::new(tiles.len(), possible, [6, 6, 6]).expect("witness system");
    assert!(witness_cells.is_feasible(), "Hall-feasible");
    assert_eq!(reduce(&witness_cells), witness_cells, "already reduced");
    let target = rob_core::compile_total_support(&witness_cells, None);

    let generators = static_generators();
    assert_eq!(generators.len(), 450);
    let mut matches = Vec::new();
    for generator in &generators {
        let (nf, _) = decode_generator(generator, &pool);
        if nf == target {
            matches.push(*generator);
        }
    }
    assert_eq!(matches.len(), 2, "exactly two static matches");
    // The two matches: zeros-trump called context and NT context 0, each
    // with only hidden seat 1 (pattern 0b001) void.
    let mut fiber_sizes = Vec::new();
    for m in &matches {
        let (q, pattern) = m.void.expect("void generators");
        assert_eq!(pattern, 0b001, "only hidden seat 1 void");
        match m.declaration {
            Declaration::PipTrump(p) => {
                assert_eq!(p, PIPS[0], "zeros trump");
                assert_eq!(q, LedSuit::Called);
            }
            Declaration::NoTrump => assert_eq!(q, LedSuit::Natural(PIPS[0])),
            Declaration::DoublesTrump => panic!("DT is not a REACH-10 match"),
        }
        let fiber = algebra_for(m.declaration).lead_fiber(q);
        // Lead-witness necessity fails: the entire lead fiber is hidden.
        assert!(
            fiber.iter().all(|d| pool.contains(d)),
            "all lead tiles are inside the hidden pool"
        );
        fiber_sizes.push(fiber.len());
    }
    fiber_sizes.sort_unstable_by(|a, b| b.cmp(a));
    (450, 2, (fiber_sizes[0], fiber_sizes[1]))
}

/// The x:002 witness pool (transcribed from inbox/002 JSON as witness
/// data): `{6:0..6:5} ⊔ {0:0, 1:0, 1:1, 2:0, 2:1, 2:2, 3:0, 3:1, 3:2,
/// 3:3, 4:0, 4:1}`.
pub fn witness_pool() -> DominoSet {
    DominoSet::from_ids([
        dom(0, 0),
        dom(1, 0),
        dom(1, 1),
        dom(2, 0),
        dom(2, 1),
        dom(2, 2),
        dom(3, 0),
        dom(3, 1),
        dom(3, 2),
        dom(3, 3),
        dom(4, 0),
        dom(4, 1),
        dom(6, 0),
        dom(6, 1),
        dom(6, 2),
        dom(6, 3),
        dom(6, 4),
        dom(6, 5),
    ])
}

/// The x:002 witness profile: NT, capacities (6,6,6), hidden seat 1 void
/// in sixes.
pub fn witness_profile() -> ReachabilityOuterNecessaryProfile {
    let mut void_masks = [
        LedSuitSet::empty(),
        LedSuitSet::empty(),
        LedSuitSet::empty(),
    ];
    void_masks[0].insert(LedSuit::Natural(PIPS[6]));
    ReachabilityOuterNecessaryProfile {
        declaration: Declaration::NoTrump,
        capacities: [6, 6, 6],
        void_masks,
        pool: witness_pool(),
    }
}

/// The witness normal form as rob compiles it; checked against the
/// transcribed inbox/002 payload (no certains; Ternary residuals (6,6,6);
/// each of `6:0..6:5` excluding hidden seat 1).
pub fn witness_normal_form() -> TotalSupportNormalForm {
    let profile = witness_profile();
    let (cells, tile_order) = profile.decode_cells();
    assert!(cells.is_feasible());
    assert_eq!(reduce(&cells), cells, "the witness is already reduced");
    let nf = rob_core::compile_total_support(&cells, None);
    // Structural transcription check of the inbox/002 normal form.
    let TotalSupportNormalForm::Feasible(FeasibleSupportNormalForm {
        certain_by_seat,
        ambiguity,
    }) = &nf
    else {
        panic!("the witness is feasible");
    };
    assert!(
        certain_by_seat.iter().all(Vec::is_empty),
        "no certain marks"
    );
    let Ambiguity::Ternary {
        pool,
        residual0,
        residual1,
        excluded_seat,
    } = ambiguity
    else {
        panic!("the witness ambiguity is ternary");
    };
    assert_eq!(pool.len(), 18);
    assert_eq!(
        (*residual0, *residual1, 18 - residual0 - residual1),
        (6, 6, 6)
    );
    let sixes: Vec<usize> = (0..tile_order.len())
        .filter(|&t| rob_core::domino_from_id(tile_order[t]).contains(PIPS[6]))
        .collect();
    let expected: Vec<(usize, usize)> = sixes.into_iter().map(|t| (t, 0)).collect();
    assert_eq!(*excluded_seat, expected, "the six 6:x tiles exclude seat 1");
    nf
}

/// `x-r_unr_002_outer` (x:002): the witness passes all four classic outer
/// checks via rob's own S7 validators and is Hall-feasible and already
/// reduced. Returns the number of classic checks passed (4).
pub fn x002_outer_check() -> u64 {
    let profile = witness_profile();
    let report = profile.check_necessary();
    assert!(report.capacity_shape, "capacity shape 0 ≤ 1");
    assert!(report.schedule, "one used context ≤ j = 1");
    assert!(report.lead_witness, "6:6 lies outside the pool");
    assert!(report.hall, "Hall-feasible");
    // The fifth check is exactly what catches it (x:002).
    assert!(!report.follower_supply, "the follower-supply check fails");
    assert!(!report.all());
    let _ = witness_normal_form();
    4
}

/// `x-r_unr_002_static` (x:002): exactly 3 of the 450 generators decode to
/// the witness NF — sixes-trump called context, doubles-trump context 6,
/// NT context 6, each with only hidden seat 1 void; the doubles-trump
/// match already violates lead-witness necessity. Returns (matches,
/// lead-witness kills, the surviving candidates).
pub fn x002_static_check() -> (u64, u64, Vec<StaticGenerator>) {
    let pool = witness_pool();
    let target = witness_normal_form();
    let generators = static_generators();
    assert_eq!(generators.len(), 450);
    let mut matches = Vec::new();
    for generator in &generators {
        let (nf, _) = decode_generator(generator, &pool);
        if nf == target {
            matches.push(*generator);
        }
    }
    assert_eq!(matches.len(), 3, "exactly three static matches");
    let mut kills = 0u64;
    let mut survivors = Vec::new();
    for m in &matches {
        let (q, pattern) = m.void.expect("void generators");
        assert_eq!(pattern, 0b001, "only hidden seat 1 void");
        match m.declaration {
            Declaration::PipTrump(p) => {
                assert_eq!(p, PIPS[6], "sixes trump");
                assert_eq!(q, LedSuit::Called);
            }
            Declaration::DoublesTrump => assert_eq!(q, LedSuit::Natural(PIPS[6])),
            Declaration::NoTrump => assert_eq!(q, LedSuit::Natural(PIPS[6])),
        }
        let fiber = algebra_for(m.declaration).lead_fiber(q);
        if fiber.iter().all(|d| pool.contains(d)) {
            // DT context 6: the whole lead fiber is hidden.
            assert_eq!(m.declaration, Declaration::DoublesTrump);
            kills += 1;
        } else {
            survivors.push(*m);
        }
    }
    assert_eq!(kills, 1);
    assert_eq!(survivors.len(), 2);
    (3, kills, survivors)
}

/// Build the contract with a given bidder and declaration through the S2
/// auction machinery.
fn contract_with(bidder: Seat, declaration: Declaration) -> Contract {
    let config = RulesConfig::new(2, 7).expect("valid config");
    let shaker = bidder.offset(3);
    let (m0, _) = MatchState::start(config, shaker);
    // A deal is required by the lifecycle; its content is irrelevant to the
    // contract. Use the canonical chunk deal.
    let ids: Vec<DominoId> = all_ids().collect();
    let hands: [DominoSet; 4] =
        core::array::from_fn(|s| DominoSet::from_ids(ids[s * 7..(s + 1) * 7].iter().copied()));
    let deal = rob_core::DealWorld::new(hands).expect("valid deal");
    let (mut attempt, m1, _, _) = begin_deal_attempt(&m0, deal, 0).expect("begin");
    let p30 = AuctionAction::Bid(BidValue::Point(PointAmount::new(30).expect("30")));
    for action in [
        p30,
        AuctionAction::Pass,
        AuctionAction::Pass,
        AuctionAction::Pass,
    ] {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m1, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!(),
    };
    contract_from_auction(pending.win(), declaration, config).expect("certified contract")
}

/// `x-r_unr_002_traces` (x:002 steps 7–12): complete shallow-prefix
/// exhaustion over all three static matches — 3 × 720 × 197 candidates,
/// zero realizing traces. Returns (candidates, realizers).
pub fn x002_trace_search() -> (u64, u64) {
    let pool = witness_pool();
    let target = witness_normal_form();
    let complement: Vec<DominoId> = all_ids().filter(|d| !pool.contains(*d)).collect();
    assert_eq!(complement.len(), 10);
    let (_, _, survivors) = x002_static_check();
    // The exhaustion runs over all three matches (including the
    // lead-witness-killed DT candidate).
    let candidates: Vec<Declaration> = {
        let mut v: Vec<Declaration> = survivors.iter().map(|s| s.declaration).collect();
        v.push(Declaration::DoublesTrump);
        v
    };
    let viewer = Seat::ALL[0];
    let mut total = 0u64;
    let mut realizers = 0u64;
    for &declaration in &candidates {
        // Contracts per possible first leader.
        let contracts: [Contract; 4] =
            core::array::from_fn(|s| contract_with(Seat::ALL[s], declaration));
        // 720 ordered choices of the hidden seats' distinct played tiles.
        for i1 in 0..10 {
            for i2 in 0..10 {
                for i3 in 0..10 {
                    if i1 == i2 || i1 == i3 || i2 == i3 {
                        continue;
                    }
                    let hidden_tiles = [complement[i1], complement[i2], complement[i3]];
                    let viewer_hand = DominoSet::from_ids(
                        complement
                            .iter()
                            .copied()
                            .filter(|d| !hidden_tiles.contains(d)),
                    );
                    let hidden_play = |seat: usize| Play {
                        actor: Seat::ALL[seat],
                        domino: hidden_tiles[seat - 1],
                    };
                    let mut try_trace = |leader: Seat, trace: Vec<Play>| {
                        let certificate = SymbolicTraceCertificate {
                            viewer,
                            viewer_initial_hand: viewer_hand,
                            contract: contracts[leader.index()],
                            trace,
                            claimed_pool: pool,
                            claimed_final: target.clone(),
                        };
                        total += 1;
                        if rob_core::validate_symbolic_trace(&certificate).is_ok() {
                            realizers += 1;
                        }
                    };
                    // Skeleton (a): three hidden plays only, leader seat 1.
                    try_trace(
                        Seat::ALL[1],
                        vec![hidden_play(1), hidden_play(2), hidden_play(3)],
                    );
                    // Skeletons (b): one completed trick, each leader ×
                    // each viewer tile.
                    let viewer_tiles: Vec<DominoId> = viewer_hand.iter().collect();
                    for leader_index in 0..4usize {
                        let leader = Seat::ALL[leader_index];
                        for &v1 in &viewer_tiles {
                            let trick: Vec<Play> = (0..4)
                                .map(|offset| {
                                    let actor = leader.offset(offset as u8);
                                    if actor == viewer {
                                        Play { actor, domino: v1 }
                                    } else {
                                        hidden_play(actor.index())
                                    }
                                })
                                .collect();
                            try_trace(leader, trick.clone());
                            // Skeletons (c): + a viewer lead of the next
                            // trick.
                            for &v2 in &viewer_tiles {
                                if v2 == v1 {
                                    continue;
                                }
                                let mut extended = trick.clone();
                                extended.push(Play {
                                    actor: viewer,
                                    domino: v2,
                                });
                                try_trace(leader, extended);
                            }
                        }
                    }
                }
            }
        }
    }
    (total, realizers)
}

/// `x-r_unr_002_supply` (x:002): the follower-supply obstruction exhibited
/// — for both surviving candidates the effective follow set has exactly
/// one member outside the pool (`{6:6}`), but a realizing trick needs two
/// distinct public followers of that context. Returns the outside count.
pub fn x002_supply_check() -> u64 {
    let pool = witness_pool();
    let (_, _, survivors) = x002_static_check();
    let mut outside_counts = Vec::new();
    for survivor in &survivors {
        let (q, _) = survivor.void.expect("void generator");
        let algebra = algebra_for(survivor.declaration);
        let outside: Vec<DominoId> = all_ids()
            .filter(|&d| algebra.follows(d, q) && !pool.contains(d))
            .collect();
        assert_eq!(outside, vec![dom(6, 6)], "F ∖ U = {{6:6}}");
        outside_counts.push(outside.len() as u64);
    }
    assert_eq!(outside_counts, vec![1, 1], "1 < 2 for both candidates");
    1
}

/// Build the canonical S8 receipt (§9.1 tier labeling).
pub fn receipt() -> String {
    let mut r = Receipt::new("S8");
    r.line("# exchange", "002 (CONFIRMED 2026-07-27)");
    let (generators, matches, sizes) = reach10_check();
    r.line(
        "r_unr_reach10",
        &format!(
            "{generators} generators; {matches} matches; lead-fiber sizes ({}, {}); all lead tiles hidden",
            sizes.0, sizes.1
        ),
    );
    r.line(
        "x-r_unr_002_outer",
        &format!(
            "{}/4 classic outer checks pass; Hall-feasible; already reduced; fifth check fails",
            x002_outer_check()
        ),
    );
    let (static_matches, kills, _) = x002_static_check();
    r.line(
        "x-r_unr_002_static",
        &format!("{static_matches} static matches; {kills} lead-witness kill (DT)"),
    );
    let (candidates, realizers) = x002_trace_search();
    r.line(
        "x-r_unr_002_traces",
        &format!(
            "{} shallow candidates; {realizers} realizers",
            fmt_commas(candidates as u128)
        ),
    );
    r.line(
        "x-r_unr_002_supply",
        &format!(
            "{} follower outside the pool < 2 required, both surviving candidates",
            x002_supply_check()
        ),
    );
    r.finish()
}
