//! Stage S1 verification harness: declaration algebra receipts
//! (BRIEF §8, table S1).
//!
//! Every function is a hard exact check (INV-5): it panics on any violation
//! and returns the exact exhaustion counts printed in the receipt.

use rob_core::{
    algebra_for, all_ids, domino_from_id, domino_id, natural_incidence, pip_trump_transport,
    Declaration, DeclarationAlgebra, Domino, DominoId, DominoSet, LedSuit, PipPermutation, Play,
    Rank, Seat, TrickKey, UnscoredMechanicsClass, DOMINOES, DOMINO_COUNT, GAME_DECLARATIONS, PIPS,
};

use crate::prose_resolver::prose_resolve;
use crate::receipt::{fmt_commas, Receipt};

/// `r_alg_universe` (Math §2; ALG-01..): 28 unique dominoes, valid ends, id
/// round-trips, incidence sizes and memberships, count total 35.
pub fn universe_check() -> (usize, u32) {
    // 28 unique, valid ends, id round-trips.
    let mut seen = std::collections::HashSet::new();
    for id in all_ids() {
        let d = domino_from_id(id);
        assert!(d.low().value() <= d.high().value() && d.high().value() <= 6);
        assert!(
            seen.insert((d.high().value(), d.low().value())),
            "duplicate domino"
        );
        assert_eq!(domino_id(d), id, "id round-trip failed");
    }
    assert_eq!(seen.len(), DOMINO_COUNT);
    for d in DOMINOES {
        assert_eq!(domino_from_id(domino_id(d)), d, "domino round-trip failed");
    }
    // Natural incidences: size 7; doubles 1 membership, mixed 2.
    for p in PIPS {
        assert_eq!(natural_incidence(p).len(), 7, "|σ_p| != 7");
    }
    for id in all_ids() {
        let d = domino_from_id(id);
        let memberships = PIPS
            .iter()
            .filter(|&&p| natural_incidence(p).contains(id))
            .count();
        assert_eq!(memberships, if d.is_double() { 1 } else { 2 });
    }
    // Pairwise incidence intersections are the single connecting mixed tile.
    for p in PIPS {
        for q in PIPS {
            if p != q {
                let inter = natural_incidence(p).intersection(&natural_incidence(q));
                assert_eq!(inter.len(), 1);
                assert_eq!(inter.iter().next(), Some(domino_id(Domino::new(p, q))));
            }
        }
    }
    let count_total: u32 = all_ids()
        .map(|id| domino_from_id(id).count_points() as u32)
        .sum();
    assert_eq!(count_total, 35);
    (DOMINO_COUNT, count_total)
}

/// `r_alg_k7` (rec ALG-20/21): the universe is the looped-K₇ edge set,
/// incidences are closed stars, and count is the antidiagonal decoration.
pub fn k7_check() -> (usize, u32) {
    // Rebuild the edge set of complete looped K₇ from the graph presentation.
    let mut edges = Vec::new();
    for i in PIPS {
        for j in PIPS {
            if i.value() <= j.value() {
                edges.push(Domino::new(i, j));
            }
        }
    }
    assert_eq!(edges.len(), DOMINO_COUNT, "looped-K7 edge count");
    let universe: std::collections::HashSet<_> = DOMINOES
        .iter()
        .map(|d| (d.high().value(), d.low().value()))
        .collect();
    for e in &edges {
        assert!(universe.contains(&(e.high().value(), e.low().value())));
    }
    let loops = edges.iter().filter(|e| e.is_double()).count();
    assert_eq!(loops, 7, "one loop per vertex");

    // Incidences are closed stars: loop at p plus the six incident edges.
    for p in PIPS {
        let star = DominoSet::from_ids(PIPS.iter().map(|&k| domino_id(Domino::new(p, k))));
        assert_eq!(star, natural_incidence(p), "σ_p is not the closed star");
    }

    // Antidiagonal decoration versus the explicit prose list (R-SCORE-01).
    let mut total = 0u32;
    for id in all_ids() {
        let d = domino_from_id(id);
        let sum = d.pip_sum();
        let expected = if sum == 5 || sum == 10 { sum } else { 0 };
        assert_eq!(d.count_points(), expected, "antidiagonal count formula");
        total += expected as u32;
    }
    assert_eq!(total, 10 + 10 + 5 + 5 + 5);
    (DOMINO_COUNT, total)
}

/// `r_alg_contexts` (REACH-05): effective suits nonempty, called absorption,
/// exactly 7 leadable contexts, lead fibers partition with sizes `{1..7}`,
/// doubles-trump natural context 0 nonempty but unleadable.
pub fn contexts_check() -> usize {
    for &decl in &GAME_DECLARATIONS {
        let algebra = algebra_for(decl);
        for id in all_ids() {
            let suits = algebra.effective_suits(id);
            assert!(!suits.is_empty(), "effective suits must be nonempty");
            if algebra.called().contains(id) {
                assert_eq!(suits.len(), 1);
                assert!(suits.contains(LedSuit::Called), "called ⇒ {{CALLED}}");
            }
            assert_eq!(
                algebra.led_suit(id) == LedSuit::Called,
                algebra.called().contains(id)
            );
        }
        let contexts = algebra.lead_contexts();
        assert_eq!(contexts.len(), 7, "exactly seven leadable contexts");
        let mut sizes: Vec<usize> = contexts
            .iter()
            .map(|&q| algebra.lead_fiber(q).len())
            .collect();
        sizes.sort_unstable();
        assert_eq!(sizes, vec![1, 2, 3, 4, 5, 6, 7], "lead-fiber size multiset");
        // Fibers partition the universe (every domino has exactly one led suit).
        let total: usize = contexts.iter().map(|&q| algebra.lead_fiber(q).len()).sum();
        assert_eq!(total, DOMINO_COUNT);
    }
    // Doubles-trump: natural context 0 is a nonempty follow set (the follow
    // table is NOT deleted) yet unleadable.
    let dt = algebra_for(Declaration::DoublesTrump);
    let n0 = LedSuit::Natural(PIPS[0]);
    assert!(all_ids().any(|id| dt.follows(id, n0)), "DT σ̂_0 nonempty");
    assert!(dt.lead_fiber(n0).is_empty(), "DT context 0 unleadable");
    assert!(!dt.lead_contexts().contains(&n0));
    7
}

/// `r_alg_tiers` (Math §3.4–3.5): tier laws, `(0,0)` exactly at tier 0,
/// natural doubles top their live suit, trump double tops mixed trumps,
/// doubles-trump order 6-6 … 0-0.
pub fn tiers_check() {
    for &decl in &GAME_DECLARATIONS {
        let algebra = algebra_for(decl);
        for q in LedSuit::all() {
            for id in all_ids() {
                let tier = algebra.tier(id, q);
                assert_eq!(tier == 2, algebra.powered().contains(id));
                assert_eq!(
                    tier == 1,
                    !algebra.powered().contains(id) && algebra.follows(id, q)
                );
                assert_eq!(
                    tier == 0,
                    !algebra.powered().contains(id) && !algebra.follows(id, q)
                );
                assert_eq!(algebra.trick_key(id, q) == TrickKey::Slough, tier == 0);
            }
        }
        // Natural doubles top their live suit: an uncalled double q-q beats
        // every other tier-one follower of context q.
        for p in PIPS {
            let double = domino_id(Domino::new(p, p));
            let q = LedSuit::Natural(p);
            if !algebra.called().contains(double) {
                assert_eq!(algebra.rank(double), Rank::Top);
                for e in all_ids() {
                    if e != double && algebra.tier(e, q) == 1 {
                        assert!(
                            algebra.trick_key(double, q) > algebra.trick_key(e, q),
                            "natural double must top its live suit"
                        );
                    }
                }
            }
        }
        // Trump double tops mixed trumps in pip trump.
        if let Declaration::PipTrump(p) = decl {
            let trump_double = domino_id(Domino::new(p, p));
            for q in LedSuit::all() {
                for e in algebra.powered().iter() {
                    if e != trump_double {
                        assert!(
                            algebra.trick_key(trump_double, q) > algebra.trick_key(e, q),
                            "trump double must top mixed trumps"
                        );
                    }
                }
            }
        }
    }
    // Doubles-trump order 6-6 … 0-0 in every context.
    let dt = algebra_for(Declaration::DoublesTrump);
    for q in LedSuit::all() {
        for w in 0..6 {
            let lower = domino_id(Domino::new(PIPS[w], PIPS[w]));
            let higher = domino_id(Domino::new(PIPS[w + 1], PIPS[w + 1]));
            assert!(dt.trick_key(higher, q) > dt.trick_key(lower, q));
        }
    }
}

/// Enumerate all `9 · 28 · C(27,3)` declaration/lead/three-subset trick
/// cases (ALG-12 exhaustion surface), calling `f` on each.
fn for_each_trick_case(mut f: impl FnMut(&DeclarationAlgebra, Declaration, &[Play; 4])) -> u64 {
    let ids: Vec<DominoId> = all_ids().collect();
    let mut cases = 0u64;
    for &decl in &GAME_DECLARATIONS {
        let algebra = algebra_for(decl);
        for lead in 0..DOMINO_COUNT {
            let rest: Vec<DominoId> = (0..DOMINO_COUNT)
                .filter(|&i| i != lead)
                .map(|i| ids[i])
                .collect();
            for x in 0..rest.len() {
                for y in (x + 1)..rest.len() {
                    for z in (y + 1)..rest.len() {
                        let plays = [
                            Play {
                                actor: Seat::ALL[0],
                                domino: ids[lead],
                            },
                            Play {
                                actor: Seat::ALL[1],
                                domino: rest[x],
                            },
                            Play {
                                actor: Seat::ALL[2],
                                domino: rest[y],
                            },
                            Play {
                                actor: Seat::ALL[3],
                                domino: rest[z],
                            },
                        ];
                        f(&algebra, decl, &plays);
                        cases += 1;
                    }
                }
            }
        }
    }
    cases
}

/// `r_alg_unique_winner` (ALG-12): every case has a unique maximal key.
pub fn unique_winner_count() -> u64 {
    for_each_trick_case(|algebra, _, plays| {
        let q = algebra.led_suit(plays[0].domino);
        let keys: Vec<TrickKey> = plays
            .iter()
            .map(|p| algebra.trick_key(p.domino, q))
            .collect();
        let max = keys.iter().max().copied().expect("four keys");
        assert_eq!(
            keys.iter().filter(|&&k| k == max).count(),
            1,
            "unique maximal trick key (Math §3.6)"
        );
        algebra.resolve_trick(plays).expect("resolvable trick");
    })
}

/// `r_alg_prose_agreement` (ALG-12; D4): the independent prose-rule resolver
/// agrees with `resolve_trick` on winner and points in every case.
pub fn prose_agreement_count() -> u64 {
    for_each_trick_case(|algebra, decl, plays| {
        let result = algebra.resolve_trick(plays).expect("resolvable trick");
        let prose_plays = plays.map(|p| (p.actor, p.domino));
        let (prose_winner, prose_points) = prose_resolve(decl, &prose_plays);
        assert_eq!(result.winner, prose_winner, "prose winner agreement");
        assert_eq!(result.points, prose_points, "prose points agreement");
    })
}

/// `r_alg_scoring` (R-SCORE-01..04): count 35, seven base trick points, hand
/// total 42; per-case: award is `1 +` payload, sloughs never beat a valid
/// lead, highest trump beats every nontrump, else the highest follower wins.
pub fn scoring_check() -> (u32, u32, u32) {
    let count_total: u32 = all_ids()
        .map(|id| domino_from_id(id).count_points() as u32)
        .sum();
    assert_eq!(count_total, 35);
    let trick_base: u32 = 7; // one point per completed trick, seven tricks
    let hand_total = count_total + trick_base;
    assert_eq!(hand_total, 42);

    for_each_trick_case(|algebra, _, plays| {
        let result = algebra.resolve_trick(plays).expect("resolvable trick");
        let q = algebra.led_suit(plays[0].domino);
        let payload: u8 = plays
            .iter()
            .map(|p| domino_from_id(p.domino).count_points())
            .sum();
        assert_eq!(result.points, 1 + payload, "award = 1 + count payload");

        let winner_play = plays
            .iter()
            .find(|p| p.actor == result.winner)
            .expect("winner is an actor");
        let winner_tier = algebra.tier(winner_play.domino, q);
        assert!(winner_tier > 0, "a slough never beats a valid lead");
        let max_tier = plays
            .iter()
            .map(|p| algebra.tier(p.domino, q))
            .max()
            .expect("four");
        // Highest trump beats every nontrump; else highest follower wins.
        assert_eq!(winner_tier, max_tier);
        for p in plays {
            if p.domino != winner_play.domino && algebra.tier(p.domino, q) == winner_tier {
                assert!(
                    algebra.trick_key(winner_play.domino, q) > algebra.trick_key(p.domino, q),
                    "winner is maximal in the winning tier"
                );
            }
        }
    });
    (count_total, trick_base, hand_total)
}

/// `r_alg_beats` (Math §3.7; ALG-13): membership in `BEATS` is exactly key
/// dominance, for every declaration, context, and ordered pair.
pub fn beats_check() -> u64 {
    let mut checks = 0u64;
    for &decl in &GAME_DECLARATIONS {
        let algebra = algebra_for(decl);
        for q in LedSuit::all() {
            for d in all_ids() {
                let beats = algebra.beats(q, d);
                let key = algebra.trick_key(d, q);
                for e in all_ids() {
                    assert_eq!(
                        beats.contains(e),
                        algebra.trick_key(e, q) > key,
                        "BEATS membership ⇔ key dominance"
                    );
                    checks += 1;
                }
            }
        }
    }
    checks
}

/// `r_alg_threat_witness` (ALG-15): in no-trump, `0-0` and `1-1` both have
/// empty when-led threat sets yet follow different natural suits — threat is
/// an exact diagonal query, not a complete play ontology.
pub fn threat_witness_check() {
    let nt = algebra_for(Declaration::NoTrump);
    let d00 = domino_id(Domino::new(PIPS[0], PIPS[0]));
    let d11 = domino_id(Domino::new(PIPS[1], PIPS[1]));
    assert!(nt.threat(d00).is_empty());
    assert!(nt.threat(d11).is_empty());
    let n0 = LedSuit::Natural(PIPS[0]);
    let n1 = LedSuit::Natural(PIPS[1]);
    assert!(nt.follows(d00, n0) && !nt.follows(d00, n1));
    assert!(nt.follows(d11, n1) && !nt.follows(d11, n0));
}

/// `r_alg_scored_transport` (ALG-17/18/19): of all 5,040 pip permutations
/// exactly two preserve every count label (identity and `2<->3`); the swap is
/// a game-order isomorphism exactly between declaration layers 2 and 3, and
/// transports order, not literal numeric rank labels.
pub fn scored_transport_check() -> (usize, usize) {
    use rob_core::algebra::transport::{
        is_scored_game_order_transport, preserves_count_labels, transport_domino,
    };
    let all = PipPermutation::all();
    assert_eq!(all.len(), 5040);
    let preserving: Vec<&PipPermutation> =
        all.iter().filter(|p| preserves_count_labels(p)).collect();
    assert_eq!(preserving.len(), 2);
    assert!(preserving.contains(&&PipPermutation::identity()));
    let swap = PipPermutation::swap_2_3();
    assert!(preserving.contains(&&swap));

    // The swap transports the scored game order exactly between layers 2
    // and 3 (and no other Straight layer).
    for &decl in &GAME_DECLARATIONS {
        let expected = matches!(
            decl,
            Declaration::PipTrump(p) if p == PIPS[2] || p == PIPS[3]
        );
        assert_eq!(
            is_scored_game_order_transport(&swap, decl),
            expected,
            "scoped 2<->3 transport classification at {decl:?}"
        );
    }

    // Order is transported, not literal numeric rank labels: 2-0 has rank 2
    // in twos while its image 3-0 has rank 3 in threes (Math §3.9).
    let twos = algebra_for(Declaration::PipTrump(PIPS[2]));
    let threes = algebra_for(Declaration::PipTrump(PIPS[3]));
    let d20 = domino_id(Domino::new(PIPS[2], PIPS[0]));
    let image = transport_domino(&swap, d20);
    assert_eq!(image, domino_id(Domino::new(PIPS[3], PIPS[0])));
    assert_ne!(
        twos.rank(d20),
        threes.rank(image),
        "literal numeric rank labels are not preserved"
    );
    (all.len(), preserving.len())
}

/// `r_alg_unscored_transport` (rec ALG-22): all 49 ordered pip-trump
/// transports succeed on the count-blind relation surface; returns the
/// transport count and total pairwise order comparisons.
pub fn unscored_transport_check() -> (usize, u64) {
    use rob_core::algebra::transport::check_unscored_transport;
    let mut transports = 0usize;
    let mut comparisons = 0u64;
    for t in PIPS {
        for u in PIPS {
            let transport = pip_trump_transport(t, u);
            comparisons += check_unscored_transport(&transport)
                .unwrap_or_else(|e| panic!("transport {t:?}->{u:?} failed: {e:?}"));
            transports += 1;
        }
    }
    (transports, comparisons)
}

/// `r_alg_mechanics_classes` (rec ALG-23/24): the structural signature yields
/// exactly three unscored classes with the proved invariant values.
pub fn mechanics_classes_check() -> usize {
    use rob_core::algebra::transport::mechanics_signature;
    let mut signatures = std::collections::BTreeSet::new();
    for &decl in &GAME_DECLARATIONS {
        let signature = mechanics_signature(decl);
        let class = rob_core::unscored_mechanics_class(decl);
        let expected = match decl {
            Declaration::PipTrump(_) => ((7, 6), UnscoredMechanicsClass::PipTrumpClass),
            Declaration::DoublesTrump => ((7, 0), UnscoredMechanicsClass::DoublesTrumpClass),
            Declaration::NoTrump => ((0, 7), UnscoredMechanicsClass::NoTrumpClass),
        };
        assert_eq!((signature, class), expected);
        signatures.insert(signature);
    }
    assert_eq!(signatures.len(), 3);
    signatures.len()
}

/// `r_alg_competitive_ordinal` (rec PLAY-12/13): the ordinal is
/// order-isomorphic to the trick key within each context; the maximum
/// competitive-class size over all (declaration, context) is 13.
pub fn competitive_ordinal_check() -> usize {
    let mut max_class = 0usize;
    for &decl in &GAME_DECLARATIONS {
        let algebra = algebra_for(decl);
        for q in LedSuit::all() {
            let competitive: Vec<DominoId> =
                all_ids().filter(|&id| algebra.tier(id, q) > 0).collect();
            max_class = max_class.max(competitive.len());
            assert!(competitive.len() <= 13);
            // Zero exactly for sloughs; ordinals are a bijection onto 1..=n.
            let mut seen = vec![false; competitive.len() + 1];
            for id in all_ids() {
                let ord = algebra.competitive_ordinal(q, id) as usize;
                if algebra.tier(id, q) == 0 {
                    assert_eq!(ord, 0);
                } else {
                    assert!((1..=competitive.len()).contains(&ord));
                    assert!(!seen[ord], "ordinal must be injective");
                    seen[ord] = true;
                }
            }
            // Order isomorphism with the trick key on competitive tiles.
            for &a in &competitive {
                for &b in &competitive {
                    assert_eq!(
                        algebra.competitive_ordinal(q, a) < algebra.competitive_ordinal(q, b),
                        algebra.trick_key(a, q) < algebra.trick_key(b, q)
                    );
                }
            }
        }
    }
    assert_eq!(max_class, 13);
    max_class
}

/// Build the canonical S1 receipt (BRIEF §9). Panics on any check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("S1");
    let (universe, count_total) = universe_check();
    r.line(
        "r_alg_universe",
        &format!("{universe} dominoes; count total {count_total}"),
    );
    let (edges, k7_total) = k7_check();
    r.line(
        "r_alg_k7",
        &format!("{edges} looped-K7 edges; antidiagonal counts 10+10+5+5+5 = {k7_total}"),
    );
    let contexts = contexts_check();
    r.line(
        "r_alg_contexts",
        &format!("{contexts} leadable contexts per declaration; lead-fiber sizes 1..7"),
    );
    tiers_check();
    r.line(
        "r_alg_tiers",
        "tier and key laws hold for all 9 declarations",
    );
    r.line(
        "r_alg_unique_winner",
        &fmt_commas(unique_winner_count() as u128),
    );
    r.line(
        "r_alg_prose_agreement",
        &fmt_commas(prose_agreement_count() as u128),
    );
    let (c, t, h) = scoring_check();
    r.line(
        "r_alg_scoring",
        &format!("count {c}; seven trick points: {t}; total: {h}"),
    );
    r.line("r_alg_beats", &fmt_commas(beats_check() as u128));
    threat_witness_check();
    r.line(
        "r_alg_threat_witness",
        "NT 0-0 and 1-1 empty threat, different natural follow",
    );
    let (perms, preserving) = scored_transport_check();
    r.line(
        "r_alg_scored_transport",
        &format!(
            "{} permutations; {preserving} count-preserving; 2<->3 order isomorphism only between declarations 2 and 3",
            fmt_commas(perms as u128)
        ),
    );
    let (transports, comparisons) = unscored_transport_check();
    r.line(
        "r_alg_unscored_transport",
        &format!(
            "{transports} transports; {} comparisons",
            fmt_commas(comparisons as u128)
        ),
    );
    r.line(
        "r_alg_mechanics_classes",
        &mechanics_classes_check().to_string(),
    );
    r.line(
        "r_alg_competitive_ordinal",
        &format!("max {}", competitive_ordinal_check()),
    );
    r.finish()
}
