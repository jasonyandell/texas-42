//! Every exhaustive count the spec states is asserted here (v0.4 §1.1--§1.4).

use walt::rules::rules::{Tier, TrickKey};
use walt::rules::{Context, Decl, DeclClass, Domino, DominoSet, Pip, Seat, Trick, NATURAL};

#[test]
fn universe_has_28_dominoes() {
    assert_eq!(Domino::COUNT, 28);
    assert_eq!(Domino::ALL.len(), 28);
    let distinct: DominoSet = Domino::ALL.into_iter().collect();
    assert_eq!(distinct.len(), 28);
    assert_eq!(distinct, DominoSet::FULL);
    for (i, d) in Domino::ALL.into_iter().enumerate() {
        assert_eq!(d.index(), i);
        assert_eq!(Domino::from_index(i), Some(d));
        assert!(d.hi() >= d.lo());
        assert_eq!(Domino::new(d.lo(), d.hi()), d, "{d} must normalize");
    }
}

#[test]
fn seven_doubles_and_twenty_one_mixed() {
    let doubles = Domino::ALL.into_iter().filter(|d| d.is_double()).count();
    assert_eq!(doubles, 7);
    assert_eq!(Domino::COUNT - doubles, 21);
}

#[test]
fn every_natural_incidence_set_has_seven_tiles() {
    for p in Pip::ALL {
        assert_eq!(NATURAL[p.value() as usize].len(), 7, "sigma_{p}");
    }
    // A double lies in one natural set, a mixed tile in two: 7*7 = 7 + 2*21.
    let total: usize = Pip::ALL
        .iter()
        .map(|p| NATURAL[p.value() as usize].len())
        .sum();
    assert_eq!(total, 49);
}

#[test]
fn effective_incidence_sizes_by_declaration() {
    for decl in Decl::ALL {
        let called = decl.called_set();
        match decl.class() {
            DeclClass::PipTrump => {
                assert_eq!(called.len(), 7, "{decl} called set");
                for p in Pip::ALL {
                    let eff = decl.effective_incidence(Context::Natural(p));
                    let expected = if Decl::PipTrump(p) == decl { 0 } else { 6 };
                    assert_eq!(eff.len(), expected, "{decl} effective sigma_{p}");
                }
            }
            DeclClass::DoublesTrump => {
                assert_eq!(called.len(), 7, "{decl} called set = the doubles");
                for p in Pip::ALL {
                    assert_eq!(
                        decl.effective_incidence(Context::Natural(p)).len(),
                        6,
                        "{decl} effective sigma_{p}"
                    );
                }
            }
            DeclClass::NoTrump => {
                assert_eq!(called.len(), 0, "{decl} called set is empty");
                for p in Pip::ALL {
                    assert_eq!(
                        decl.effective_incidence(Context::Natural(p)).len(),
                        7,
                        "{decl} effective sigma_{p}"
                    );
                }
            }
        }
        assert_eq!(decl.effective_incidence(Context::Called), called);
        // Absorption: no called tile survives in any natural effective set.
        for p in Pip::ALL {
            assert!(decl
                .effective_incidence(Context::Natural(p))
                .is_disjoint(called));
        }
        // The effective family covers the universe.
        let mut cover = DominoSet::EMPTY;
        for q in Context::ALL {
            cover = cover.union(decl.effective_incidence(q));
        }
        assert_eq!(cover, DominoSet::FULL, "{decl} effective family covers D");
    }
}

#[test]
fn led_context_is_a_context_the_tile_follows() {
    for decl in Decl::ALL {
        for d in Domino::ALL {
            let q = decl.led_context(d);
            assert!(decl.follows(d, q), "{decl}: {d} must follow its own lead");
            assert_ne!(decl.tier(d, q), Tier::Slough, "{decl}: {d} leads at tier 0");
            match q {
                Context::Called => assert!(decl.is_called(d)),
                Context::Natural(p) => {
                    assert!(!decl.is_called(d));
                    assert_eq!(p, d.hi(), "an uncalled tile leads its high pip");
                }
            }
        }
    }
}

#[test]
fn count_decoration_totals_35_and_a_hand_totals_42() {
    let five: Vec<Domino> = Domino::ALL.into_iter().filter(|d| d.count() == 5).collect();
    let ten: Vec<Domino> = Domino::ALL
        .into_iter()
        .filter(|d| d.count() == 10)
        .collect();
    assert_eq!(five.len(), 3, "5-count: 5-0, 4-1, 3-2");
    assert_eq!(ten.len(), 2, "10-count: 6-4, 5-5");
    assert_eq!(
        DominoSet::FULL.count_points(),
        walt::rules::TOTAL_COUNT_POINTS
    );
    assert_eq!(walt::rules::TOTAL_COUNT_POINTS, 35);
    // Seven tricks at one point each, plus the whole count decoration.
    assert_eq!(7 + walt::rules::TOTAL_COUNT_POINTS, 42);
    assert_eq!(walt::rules::replay::HAND_TOTAL_POINTS, 42);
}

/// Distinct tiles sharing a nonzero tier have distinct ranks. This is what
/// makes the maximum trick key unique, and it is the only place the rank
/// overlap between doubles-trump doubles (0..=6) and mixed pip sums is safe.
#[test]
fn ranks_are_injective_within_every_nonzero_tier() {
    let mut checked = 0usize;
    for decl in Decl::ALL {
        for led in Context::ALL {
            for (i, a) in Domino::ALL.into_iter().enumerate() {
                for b in Domino::ALL.into_iter().skip(i + 1) {
                    let (ka, kb) = (decl.trick_key(a, led), decl.trick_key(b, led));
                    if ka.tier == kb.tier && ka.tier != Tier::Slough {
                        assert_ne!(ka.rank, kb.rank, "{decl} led {led}: {a} vs {b}");
                        checked += 1;
                    }
                }
            }
        }
    }
    assert!(checked > 0);
}

/// The spec's uniqueness claim, checked over every four-tile trick with every
/// designated lead, for every declaration.
#[test]
fn every_four_tile_trick_has_a_unique_winner() {
    let mut tricks = 0usize;
    for decl in Decl::ALL {
        for a in 0..Domino::COUNT {
            for b in (a + 1)..Domino::COUNT {
                for c in (b + 1)..Domino::COUNT {
                    for d in (c + 1)..Domino::COUNT {
                        let set = [
                            Domino::ALL[a],
                            Domino::ALL[b],
                            Domino::ALL[c],
                            Domino::ALL[d],
                        ];
                        for lead in 0..4 {
                            let mut order = [set[lead]; 4];
                            let mut k = 1;
                            for (j, t) in set.into_iter().enumerate() {
                                if j != lead {
                                    order[k] = t;
                                    k += 1;
                                }
                            }
                            let trick = Trick::new(Seat::S0, order).expect("distinct tiles");
                            let led = trick.led(decl);
                            let keys: Vec<TrickKey> =
                                order.iter().map(|t| decl.trick_key(*t, led)).collect();
                            let best = keys.iter().max().copied().expect("four keys");
                            assert_eq!(
                                keys.iter().filter(|k| **k == best).count(),
                                1,
                                "{decl} led {led}: {order:?} has a tied maximum"
                            );
                            assert!(best.tier != Tier::Slough);
                            let at = keys.iter().position(|k| *k == best).expect("a maximum");
                            assert_eq!(trick.winner(decl), Seat::S0.plus(at));
                            tricks += 1;
                        }
                    }
                }
            }
        }
    }
    // C(28,4) four-tile sets, four choices of lead, nine declarations.
    assert_eq!(tricks, 20475 * 4 * Decl::COUNT);
}

#[test]
fn beats_and_threat_are_consistent_with_the_trick_key() {
    for decl in Decl::ALL {
        for d in Domino::ALL {
            let led = decl.led_context(d);
            let threat = decl.threat(d);
            assert_eq!(threat, decl.beats(led, d));
            assert!(!threat.contains(d), "{decl}: {d} cannot beat itself");
            for e in Domino::ALL {
                assert_eq!(
                    threat.contains(e),
                    decl.trick_key(e, led) > decl.trick_key(d, led),
                    "{decl} led {led}: {e} vs {d}"
                );
            }
            // A led called tile can only be beaten by another called tile.
            if decl.is_called(d) {
                assert!(threat.is_subset_of(decl.called_set()));
            }
        }
    }
}

/// Which leads nothing can beat. With a called suit there is exactly one --
/// the top called tile. Under no-trump there is no tier 2, so every double is
/// unbeatable in its own natural suit: seven of them.
#[test]
fn unbeatable_leads_by_declaration() {
    for decl in Decl::ALL {
        let unbeatable: Vec<Domino> = Domino::ALL
            .into_iter()
            .filter(|d| decl.threat(*d).is_empty())
            .collect();
        match decl {
            Decl::NoTrump => {
                assert_eq!(unbeatable.len(), 7, "{decl}: {unbeatable:?}");
                assert!(unbeatable.iter().all(|d| d.is_double()));
            }
            Decl::DoublesTrump => {
                assert_eq!(unbeatable, vec![Domino::new(Pip::ALL[6], Pip::ALL[6])])
            }
            Decl::PipTrump(p) => assert_eq!(unbeatable, vec![Domino::new(p, p)]),
        }
    }
}

#[test]
fn legality_follows_when_able_and_sloughs_otherwise() {
    for decl in Decl::ALL {
        for q in Context::ALL {
            let eff = decl.effective_incidence(q);
            for bits in [0b1u32, 0b101, 0b1010101, DominoSet::FULL.bits()] {
                let hand = DominoSet::from_bits(bits).expect("28-bit mask");
                let legal = walt::rules::legal_plays(decl, hand, Some(q));
                let follows = hand.intersection(eff);
                if follows.is_empty() {
                    assert_eq!(legal, hand, "{decl} led {q}: slough is unrestricted");
                } else {
                    assert_eq!(legal, follows, "{decl} led {q}: must follow");
                }
                assert!(legal.is_subset_of(hand));
                assert_eq!(walt::rules::legal_plays(decl, hand, None), hand);
            }
        }
    }
}

#[test]
fn seats_and_teams() {
    assert_eq!(Seat::COUNT, 4);
    for s in Seat::ALL {
        assert_eq!(s.successor().successor().successor().successor(), s);
        assert_ne!(s.team(), s.successor().team());
        assert_eq!(s.team(), s.plus(2).team());
        assert_eq!(s.plus(4), s);
    }
    assert_eq!(walt::rules::Team::T0.seats(), [Seat::S0, Seat::S2]);
    assert_eq!(walt::rules::Team::T1.seats(), [Seat::S1, Seat::S3]);
}

#[test]
fn there_are_nine_declarations() {
    assert_eq!(Decl::COUNT, 9);
    let mut seen = std::collections::BTreeSet::new();
    for d in Decl::ALL {
        assert!(seen.insert(d));
        assert_eq!(d.to_string().parse::<Decl>().expect("round trip"), d);
    }
    assert_eq!(seen.len(), 9);
    assert_eq!(
        Decl::ALL
            .iter()
            .filter(|d| d.class() == DeclClass::PipTrump)
            .count(),
        7
    );
}
