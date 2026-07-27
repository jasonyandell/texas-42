//! Stage S3 verification harness: cells as derived views and the
//! losslessness parity corpus (BRIEF §8, table S3).

use std::collections::BTreeSet;

use rob_core::{
    algebra_for, contract_from_auction, derive_auction_cells, derive_rule_cells,
    initial_contracted_mechanical, update_support, AbstractCells, AbstractWorld, AuctionAction,
    AuctionResult, AuctionState, BidValue, Contract, Declaration, Domino, DominoSet, LedSuit,
    MechanicalState, Pip, Play, PointAmount, RulesConfig, Seat, GAME_DECLARATIONS, HIDDEN_SEATS,
    PIPS,
};

use crate::corpus::{
    deterministic_auction_script, deterministic_deal, play_hand, DetRng, PlayedHand,
};
use crate::receipt::{fmt_commas, Receipt};

/// rob's generator-specific with-voids parity count, frozen in the committed
/// receipt (BRIEF §8 `r_cell_parity`: the 972 corpus shape is a hard
/// assertion; the with-voids count depends on the deterministic generator —
/// ingest's own generator yields 970).
pub const FROZEN_WITH_VOIDS: u64 = 970;

/// The S3 parity corpus: 12 deterministic contracted hands under each of the
/// nine declarations (index `0..108`), viewer seat 0 (BRIEF §8
/// `r_cell_parity`; CELL-07A corpus shape).
pub fn s3_corpus_hand(index: u64) -> PlayedHand {
    assert!(index < 108);
    let config = RulesConfig::new(7, 7).expect("valid config");
    let declaration = GAME_DECLARATIONS[(index / 12) as usize];
    let shaker = Seat::ALL[(index % 4) as usize];
    let mut rng = DetRng::new(0x0c0_3000 + index);
    let deal = deterministic_deal(&mut rng);
    let script = deterministic_auction_script(&mut rng, config, shaker);
    play_hand(config, shaker, deal, &script, declaration, |_, legal| {
        rng.below(legal.len())
    })
    .expect("deterministic corpus hand plays to completion")
}

/// The viewer-side mechanical trajectory of one played hand: the initial
/// contracted mechanical state plus one typed support update per play
/// (Exec §17).
pub fn mechanical_trajectory(hand: &PlayedHand, viewer: Seat) -> Vec<MechanicalState> {
    let contract = *hand.states[0].state().contract();
    let mut state = initial_contracted_mechanical(viewer, *hand.deal.hand(viewer), contract)
        .expect("dealt hand has seven tiles");
    let mut trajectory = vec![state.clone()];
    for step in &hand.steps {
        state = update_support(
            &state,
            Play {
                actor: step.actor,
                domino: step.domino,
            },
        )
        .expect("legal observed play updates support");
        trajectory.push(state.clone());
    }
    trajectory
}

/// `r_cell_initial` (AUC-07/08; TRANS-12 static half): after any straight
/// auction + declaration the pool is the 21 unseen tiles, every allowed set
/// is the pool, every capacity 7, and there are exactly 63 holder edges.
pub fn initial_check() -> u64 {
    let mut checked = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let viewer = Seat::ALL[0];
        let auction_cells = derive_auction_cells(hand.deal.hand(viewer)).expect("seven tiles");
        let initial = &mechanical_trajectory(&hand, viewer)[0];
        let cells = derive_rule_cells(initial);
        assert_eq!(cells, auction_cells, "initial cells equal auction cells");
        assert_eq!(cells.unseen_pool().len(), 21);
        for s in 0..HIDDEN_SEATS {
            assert_eq!(cells.possible(s), cells.unseen_pool(), "P_s = U initially");
            assert_eq!(cells.capacity(s), 7);
        }
        assert_eq!(cells.holder_edge_count(), 63, "21 × 3 = 63 holder edges");
        checked += 1;
    }
    checked
}

/// `r_cell_dependent` (CELL-01): the dependent-cells negative witness — two
/// active seats, two tiles, capacity one each: only 2 of the 4 componentwise
/// assignments are conserved worlds.
pub fn dependent_check() -> (usize, usize) {
    let cells = AbstractCells::new(
        2,
        [vec![true, true], vec![true, true], vec![false, false]],
        [1, 1, 0],
    )
    .expect("well-formed witness system");
    let componentwise = 2 * 2; // independent per-seat choices from P_0 × P_1
    let worlds = cells.worlds();
    assert_eq!(worlds.len(), 2);
    let as_sets: BTreeSet<AbstractWorld> = worlds.into_iter().collect();
    let expected: BTreeSet<AbstractWorld> =
        [[vec![0], vec![1], vec![]], [vec![1], vec![0], vec![]]]
            .into_iter()
            .collect();
    assert_eq!(as_sets, expected, "exactly the two disjoint assignments");
    (2, componentwise)
}

fn reindex_after_removal(world: &AbstractWorld, seat: usize, tile: usize) -> AbstractWorld {
    core::array::from_fn(|s| {
        world[s]
            .iter()
            .filter(|&&t| !(s == seat && t == tile))
            .map(|&t| if t > tile { t - 1 } else { t })
            .collect()
    })
}

/// `r_cell_tiny_updates` (TRANS-07): exhaust the typed-update algebra on
/// every abstract three-seat cell system with universe size 1..=3 — for
/// every actor with positive capacity, every tile in its allowed set, and
/// every abstract follow set, the exact image of legal predecessor worlds
/// equals the fiber of the typed update. Returns (leads, follows, sloughs).
pub fn tiny_updates_check() -> (u64, u64, u64) {
    let mut leads = 0u64;
    let mut follows = 0u64;
    let mut sloughs = 0u64;
    for n in 1..=3usize {
        let subsets: Vec<Vec<bool>> = (0..1u32 << n)
            .map(|mask| (0..n).map(|t| mask & (1 << t) != 0).collect())
            .collect();
        for p0 in &subsets {
            for p1 in &subsets {
                for p2 in &subsets {
                    for k0 in 0..=n {
                        for k1 in 0..=(n - k0) {
                            let k2 = n - k0 - k1;
                            let cells = AbstractCells::new(
                                n,
                                [p0.clone(), p1.clone(), p2.clone()],
                                [k0, k1, k2],
                            )
                            .expect("well-formed tiny system");
                            let worlds: BTreeSet<AbstractWorld> =
                                cells.worlds().into_iter().collect();
                            for seat in 0..HIDDEN_SEATS {
                                if cells.capacity(seat) == 0 {
                                    continue;
                                }
                                for tile in 0..n {
                                    if !cells.possible(seat)[tile] {
                                        continue;
                                    }
                                    // Lead: the tile is the whole witness.
                                    let image: BTreeSet<AbstractWorld> = worlds
                                        .iter()
                                        .filter(|w| w[seat].contains(&tile))
                                        .map(|w| reindex_after_removal(w, seat, tile))
                                        .collect();
                                    let updated: BTreeSet<AbstractWorld> = cells
                                        .removal_update(seat, tile)
                                        .expect("tile is allowed with capacity")
                                        .worlds()
                                        .into_iter()
                                        .collect();
                                    assert_eq!(image, updated, "lead update exactness");
                                    leads += 1;
                                    for follow_set in &subsets {
                                        if follow_set[tile] {
                                            // Successful follow: identical
                                            // removal; no positive clause
                                            // survives (CELL-06).
                                            assert_eq!(image, updated, "follow update exactness");
                                            follows += 1;
                                        } else {
                                            // Failure to follow: the whole
                                            // follow set leaves P_s.
                                            let legal: BTreeSet<AbstractWorld> = worlds
                                                .iter()
                                                .filter(|w| {
                                                    w[seat].contains(&tile)
                                                        && w[seat].iter().all(|&t| !follow_set[t])
                                                })
                                                .map(|w| reindex_after_removal(w, seat, tile))
                                                .collect();
                                            let slough_updated: BTreeSet<AbstractWorld> = cells
                                                .fail_follow_update(seat, tile, follow_set)
                                                .expect("well-typed slough update")
                                                .worlds()
                                                .into_iter()
                                                .collect();
                                            assert_eq!(
                                                legal, slough_updated,
                                                "slough update exactness"
                                            );
                                            sloughs += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (leads, follows, sloughs)
}

/// Independent parity side: enumerate every capacity-compatible complete
/// deal candidate for one prefix and keep those whose actor-attributed
/// replay is legal under the rules; return their remainder images
/// (Math §7.5 receipt steps 3–4). Never consults `derive_rule_cells`.
fn replayed_remainders(
    viewer: Seat,
    viewer_initial_hand: &DominoSet,
    declaration: Declaration,
    prefix: &[Play],
) -> BTreeSet<Vec<Vec<u8>>> {
    let algebra = algebra_for(declaration);
    let hidden: [Seat; 3] = [viewer.offset(1), viewer.offset(2), viewer.offset(3)];
    let mut played = [DominoSet::empty(); 4];
    for p in prefix {
        played[p.actor.index()].insert(p.domino);
    }
    let mut free = DominoSet::full().difference(viewer_initial_hand);
    for p in &played {
        free = free.difference(p);
    }
    let free_tiles: Vec<_> = free.iter().collect();
    let need: [usize; 3] = core::array::from_fn(|i| 7 - played[hidden[i].index()].len());

    let mut result = BTreeSet::new();
    let mut assignment = vec![0usize; free_tiles.len()];
    enumerate_partitions(0, &free_tiles, &need, &mut assignment, &mut |assignment| {
        // Candidate initial hidden hands: played tiles plus assigned free
        // tiles.
        let mut hands = [DominoSet::empty(); 4];
        hands[viewer.index()] = *viewer_initial_hand;
        for (i, &seat) in hidden.iter().enumerate() {
            let mut hand = played[seat.index()];
            for (t, &owner) in assignment.iter().enumerate() {
                if owner == i {
                    hand.insert(free_tiles[t]);
                }
            }
            hands[seat.index()] = hand;
        }
        // Replay the actor-attributed prefix under the rules.
        let mut current = hands;
        let mut trick: Vec<Play> = Vec::new();
        let mut legal = true;
        for play in prefix {
            let hand = &current[play.actor.index()];
            if !hand.contains(play.domino) {
                legal = false;
                break;
            }
            if let Some(first) = trick.first() {
                let q = algebra.led_suit(first.domino);
                if !algebra.follows(play.domino, q) && hand.iter().any(|d| algebra.follows(d, q)) {
                    legal = false;
                    break;
                }
            }
            current[play.actor.index()].remove(play.domino);
            trick.push(*play);
            if trick.len() == 4 {
                trick.clear();
            }
        }
        if legal {
            let remainder: Vec<Vec<u8>> = hidden
                .iter()
                .map(|&s| current[s.index()].iter().map(|d| d.index() as u8).collect())
                .collect();
            result.insert(remainder);
        }
    });
    result
}

fn enumerate_partitions(
    tile: usize,
    tiles: &[rob_core::DominoId],
    need: &[usize; 3],
    assignment: &mut Vec<usize>,
    f: &mut impl FnMut(&[usize]),
) {
    if tile == tiles.len() {
        f(assignment);
        return;
    }
    let mut used = [0usize; 3];
    for &owner in assignment[..tile].iter() {
        used[owner] += 1;
    }
    for owner in 0..3 {
        if used[owner] < need[owner] {
            assignment[tile] = owner;
            enumerate_partitions(tile + 1, tiles, need, assignment, f);
        }
    }
}

/// `r_cell_parity` (CELL-05/07/07A): for all 972 corpus prefixes, exact set
/// equality `Φ(derive_rule_cells(state)) = ρ(Ω(I))`. Returns
/// (prefixes, prefixes with at least one derived public void).
pub fn parity_check() -> (u64, u64) {
    let viewer = Seat::ALL[0];
    let mut prefixes = 0u64;
    let mut with_voids = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let trajectory = mechanical_trajectory(&hand, viewer);
        let plays: Vec<Play> = hand
            .steps
            .iter()
            .map(|s| Play {
                actor: s.actor,
                domino: s.domino,
            })
            .collect();
        for t in 20..=28usize {
            let state = &trajectory[t];
            let cells = derive_rule_cells(state);
            let fiber: BTreeSet<Vec<Vec<u8>>> = cells
                .fiber_worlds()
                .into_iter()
                .map(|w| {
                    w.hidden_hands
                        .iter()
                        .map(|h| h.iter().map(|d| d.index() as u8).collect())
                        .collect()
                })
                .collect();
            let replayed = replayed_remainders(
                viewer,
                hand.deal.hand(viewer),
                hand.declaration,
                &plays[..t],
            );
            assert_eq!(fiber, replayed, "Φ(cells) = ρ(Ω(I)) at prefix {t}");
            if state
                .hidden_seats()
                .iter()
                .any(|&s| !state.public_voids(s).is_empty())
            {
                with_voids += 1;
            }
            prefixes += 1;
        }
    }
    assert_eq!(prefixes, 972);
    (prefixes, with_voids)
}

/// `r_cell_transitions` (TRANS-01..05): along the parity corpus, the 864
/// per-play typed transitions from play 20 to 28 — hidden plays never
/// increase the fiber, viewer plays leave it unchanged. Returns
/// (total, hidden nonincrease, viewer equality).
pub fn transitions_check() -> (u64, u64, u64) {
    let viewer = Seat::ALL[0];
    let mut total = 0u64;
    let mut hidden_nonincrease = 0u64;
    let mut viewer_equality = 0u64;
    for index in 0..108 {
        let hand = s3_corpus_hand(index);
        let trajectory = mechanical_trajectory(&hand, viewer);
        // Corpus-shape check: plays 21..28 give each seat exactly two plays.
        let mut per_seat = [0usize; 4];
        for step in &hand.steps[20..28] {
            per_seat[step.actor.index()] += 1;
        }
        assert_eq!(per_seat, [2, 2, 2, 2], "two plays per seat in 21..28");
        for t in 20..28usize {
            let pre = derive_rule_cells(&trajectory[t]).fiber_worlds();
            let post = derive_rule_cells(&trajectory[t + 1]).fiber_worlds();
            let actor = hand.steps[t].actor;
            if actor == viewer {
                let as_indices = |worlds: &[rob_core::RemainderWorld]| -> BTreeSet<Vec<Vec<u8>>> {
                    worlds
                        .iter()
                        .map(|w| {
                            w.hidden_hands
                                .iter()
                                .map(|h| h.iter().map(|d| d.index() as u8).collect())
                                .collect()
                        })
                        .collect()
                };
                assert_eq!(
                    as_indices(&pre),
                    as_indices(&post),
                    "viewer plays leave the fiber unchanged"
                );
                viewer_equality += 1;
            } else {
                assert!(
                    post.len() <= pre.len(),
                    "hidden plays never increase the fiber"
                );
                hidden_nonincrease += 1;
            }
            total += 1;
        }
    }
    (total, hidden_nonincrease, viewer_equality)
}

fn dom(h: u8, l: u8) -> rob_core::DominoId {
    let high = Pip::new(h).expect("pip");
    let low = Pip::new(l).expect("pip");
    rob_core::domino_id(Domino::new(high, low))
}

/// Build the Math §10.4 witness contract from one auction history.
fn ninety_world_contract(actions: [AuctionAction; 4]) -> Contract {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = Seat::ALL[3];
    let mut auction = AuctionState::new(shaker);
    for action in actions {
        auction = auction.apply(action, config).expect("legal auction action");
    }
    let win = match auction.result().expect("complete") {
        AuctionResult::Win(win) => win,
        AuctionResult::AllPass => panic!("witness auctions have a winner"),
    };
    contract_from_auction(&win, Declaration::NoTrump, config).expect("certified contract")
}

/// `r_cell_ninety_world_support` (STR-06; Math §10.4, support half): replay
/// both legal auction histories α_A/α_B and the five fixed tricks; both
/// reach identical cells with fiber cardinality `6!/(2!)^3 = 90`.
pub fn ninety_world_check() -> u64 {
    let pass = AuctionAction::Pass;
    let p = |n: u8| AuctionAction::Bid(BidValue::Point(PointAmount::new(n).expect("valid")));
    let alpha_a = [pass, p(30), pass, p(31)];
    let alpha_b = [p(30), pass, pass, p(31)];
    let contract_a = ninety_world_contract(alpha_a);
    let contract_b = ninety_world_contract(alpha_b);
    assert_eq!(contract_a, contract_b, "same bidder, bid, and declaration");
    assert_eq!(contract_a.bidder(), Seat::ALL[3]);

    // Viewer seat 3's initial hand: the five tiles it plays plus 3-1, 4-1.
    let viewer = Seat::ALL[3];
    let own = DominoSet::from_ids([
        dom(6, 3),
        dom(5, 0),
        dom(4, 0),
        dom(2, 1),
        dom(5, 1),
        dom(3, 1),
        dom(4, 1),
    ]);
    // The five fixed tricks (Math §10.4), actor-attributed.
    let seat = |i: u8| Seat::ALL[i as usize];
    let tricks: [[(u8, (u8, u8)); 4]; 5] = [
        [(3, (6, 3)), (0, (6, 1)), (1, (6, 4)), (2, (6, 0))],
        [(1, (0, 0)), (2, (2, 2)), (3, (5, 0)), (0, (2, 0))],
        [(1, (4, 3)), (2, (4, 2)), (3, (4, 0)), (0, (5, 4))],
        [(0, (1, 1)), (1, (3, 0)), (2, (3, 3)), (3, (2, 1))],
        [(0, (1, 0)), (1, (6, 6)), (2, (5, 2)), (3, (5, 1))],
    ];

    let mut endpoints = Vec::new();
    for contract in [contract_a, contract_b] {
        let mut state =
            initial_contracted_mechanical(viewer, own, contract).expect("seven-tile viewer hand");
        for trick in &tricks {
            for &(actor, (h, l)) in trick {
                state = update_support(
                    &state,
                    Play {
                        actor: seat(actor),
                        domino: dom(h, l),
                    },
                )
                .expect("witness plays are legal observations");
            }
        }
        endpoints.push(state);
    }
    assert_eq!(endpoints[0], endpoints[1], "identical mechanical endpoints");

    let state = &endpoints[0];
    assert_eq!(state.hand_points(), [2, 18], "fixed-team scores (2,18)");
    assert_eq!(state.leader(), viewer, "seat 3 leads");
    assert!(state.current_trick().is_empty());
    assert_eq!(
        *state.own_remaining_hand(),
        DominoSet::from_ids([dom(3, 1), dom(4, 1)])
    );
    // Derived voids: seat 1 void in ones; seat 2 void in blanks and ones.
    let ones = LedSuit::Natural(PIPS[1]);
    let blanks = LedSuit::Natural(PIPS[0]);
    assert!(state.public_voids(seat(1)).contains(ones));
    assert_eq!(state.public_voids(seat(1)).len(), 1);
    assert!(state.public_voids(seat(2)).contains(blanks));
    assert!(state.public_voids(seat(2)).contains(ones));
    assert_eq!(state.public_voids(seat(2)).len(), 2);
    assert!(state.public_voids(seat(0)).is_empty());
    assert!(state.public_voids(seat(3)).is_empty());

    let cells = derive_rule_cells(state);
    let expected_pool = DominoSet::from_ids([
        dom(5, 5),
        dom(4, 4),
        dom(3, 2),
        dom(6, 5),
        dom(5, 3),
        dom(6, 2),
    ]);
    assert_eq!(*cells.unseen_pool(), expected_pool);
    for s in 0..HIDDEN_SEATS {
        assert_eq!(cells.capacity(s), 2);
        assert_eq!(
            cells.possible(s),
            &expected_pool,
            "no pool tile lies in an excluded suit"
        );
    }
    let count = cells.fiber_worlds().len() as u64;
    assert_eq!(count, 90, "6!/(2!)^3 = 90 worlds");
    count
}

/// Build the canonical S3 receipt (BRIEF §9). Panics on any check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("S3");
    let initial = initial_check();
    r.line(
        "r_cell_initial",
        &format!("{initial} deals; U=21; P_s=U; k_s=7; holder edges 63"),
    );
    let (worlds, componentwise) = dependent_check();
    r.line(
        "r_cell_dependent",
        &format!("{worlds} of {componentwise} componentwise assignments are worlds"),
    );
    let (leads, follows, sloughs) = tiny_updates_check();
    r.line(
        "r_cell_tiny_updates",
        &format!(
            "{} leads; {} follows; {} sloughs",
            fmt_commas(leads as u128),
            fmt_commas(follows as u128),
            fmt_commas(sloughs as u128)
        ),
    );
    let (prefixes, with_voids) = parity_check();
    r.line(
        "r_cell_parity",
        &format!("{prefixes} prefixes ({with_voids} with public voids)"),
    );
    let (total, hidden, viewer) = transitions_check();
    r.line(
        "r_cell_transitions",
        &format!("{total} total; {hidden} hidden nonincrease; {viewer} viewer equality"),
    );
    r.line(
        "r_cell_ninety_world_support",
        &ninety_world_check().to_string(),
    );
    r.finish()
}
