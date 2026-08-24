use std::collections::BTreeMap;

use walt::rules::{legal_plays, Context, Decl, Domino, DominoSet, Pip};
use walt::spec::FIELD_SCALE;
use walt_gpu_ref::{
    direct_preflight, project_closed_form, project_direct, DirectPreflightV1, OpeningContext,
    OpeningError, ResponseRole, M1_DIRECT_WORLD_CAP_V1, MAX_OPENING_MATCHING_COUNT_V1,
    OPENING_DEAL_COUNT,
};

fn led_context() -> Context {
    Context::Natural(Pip::new(6).expect("six is a pip"))
}

/// Deterministic reduced fixture generation: use no-trump and natural six,
/// then take the first `m` matching tiles and the first `3g-m` nonmatching
/// tiles in stable domino-index order.  There is no PRNG or ambient corpus.
fn reduced_fixture(grade: u8, matching_count: usize) -> OpeningContext {
    let decl = Decl::NoTrump;
    let led = led_context();
    let matching = decl.effective_incidence(led);
    let nonmatching = DominoSet::FULL.difference(matching);
    let pool_size = usize::from(grade) * 3;
    assert!(matching_count <= matching.len());
    assert!(pool_size - matching_count <= nonmatching.len());
    let pool: DominoSet = matching
        .iter()
        .take(matching_count)
        .chain(nonmatching.iter().take(pool_size - matching_count))
        .collect();
    OpeningContext::try_reduced(decl, led, pool, grade).expect("deterministic reduced fixture")
}

/// Deterministic full opening generation uses the same stable selection as
/// `reduced_fixture`; the focal hand is the exact complement of the 21-tile
/// pool and therefore exercises the production constructor.
fn full_opening_fixture(matching_count: usize) -> OpeningContext {
    let reduced = reduced_fixture(7, matching_count);
    let focal_hand = DominoSet::FULL.difference(reduced.pool());
    OpeningContext::from_opening_hand(reduced.decl(), focal_hand, reduced.led())
        .expect("full fixture has a legal lead in its selected context")
}

#[test]
fn m1_context_validation_fails_closed() {
    let decl = Decl::NoTrump;
    let led = led_context();
    let six_tiles: DominoSet = Domino::ALL.into_iter().take(6).collect();
    assert!(matches!(
        OpeningContext::try_reduced(decl, led, six_tiles, 0),
        Err(OpeningError::GradeOutOfRange { grade: 0 })
    ));
    assert!(matches!(
        OpeningContext::try_reduced(decl, led, six_tiles, 8),
        Err(OpeningError::GradeOutOfRange { grade: 8 })
    ));
    assert!(matches!(
        OpeningContext::try_reduced(decl, led, six_tiles, 3),
        Err(OpeningError::PoolSizeMismatch { .. })
    ));
    assert!(matches!(
        OpeningContext::try_reduced(decl, Context::Called, six_tiles, 2),
        Err(OpeningError::ImpossibleLedContext { .. })
    ));

    assert!(matches!(
        OpeningContext::from_opening_hand(decl, six_tiles, led),
        Err(OpeningError::OpeningHandSize { actual: 6 })
    ));
    let hand_without_sixes: DominoSet = Domino::ALL.into_iter().take(7).collect();
    assert!(matches!(
        OpeningContext::from_opening_hand(decl, hand_without_sixes, led),
        Err(OpeningError::LedContextNotRepresented { .. })
    ));

    let all_sixes = decl.effective_incidence(led);
    assert_eq!(all_sixes.len(), MAX_OPENING_MATCHING_COUNT_V1 + 1);
    let grade_seven_pool: DominoSet = all_sixes
        .iter()
        .chain(DominoSet::FULL.difference(all_sixes).iter().take(14))
        .collect();
    assert!(matches!(
        OpeningContext::try_reduced(decl, led, grade_seven_pool, 7),
        Err(OpeningError::OpeningMatchingCountOutOfRange {
            actual: 7,
            max: MAX_OPENING_MATCHING_COUNT_V1,
        })
    ));
}

#[test]
fn m1_closed_form_matches_independent_physical_enumerator_at_grades_two_to_four() {
    for grade in 2..=4 {
        for matching_count in 0..=6 {
            let context = reduced_fixture(grade, matching_count);
            let closed = project_closed_form(context).expect("closed-form projection");
            let direct = project_direct(context).expect("bounded physical enumeration");

            assert_eq!(
                closed, direct,
                "cell parity failed at grade {grade}, m={matching_count}"
            );
            assert_eq!(
                closed.response_aggregates().expect("closed responses"),
                direct.response_aggregates().expect("direct responses"),
                "response-level parity failed at grade {grade}, m={matching_count}"
            );
            assert_eq!(
                closed.total_scaled_mass().expect("closed total"),
                closed.expected_scaled_mass().expect("expected total"),
                "mass conservation failed at grade {grade}, m={matching_count}"
            );
            assert!(closed
                .cells()
                .windows(2)
                .all(|pair| pair[0].key() < pair[1].key()));

            for cell in closed.cells() {
                assert_eq!(cell.remaining_capacities(context), [grade - 1; 3]);
                assert_eq!(
                    cell.remaining_pool(context).len(),
                    usize::from(grade - 1) * 3
                );
                assert_eq!(
                    cell.remaining_matching_mask(context).len(),
                    cell.key()
                        .matching_counts()
                        .into_iter()
                        .map(usize::from)
                        .sum::<usize>()
                );
                for (role, count) in cell
                    .key()
                    .roles(context)
                    .into_iter()
                    .zip(cell.key().matching_counts())
                {
                    if role == ResponseRole::Void {
                        assert_eq!(count, 0);
                    }
                }
            }
        }
    }
}

#[test]
fn m1_full_grade_counts_and_mass_are_exact_without_physical_enumeration() {
    let expected_counts = [7_980usize, 1_140, 2_166, 3_408, 5_172, 7_800, 11_730];
    let expected_mass = OPENING_DEAL_COUNT
        .checked_mul(u64::from(FIELD_SCALE).pow(3))
        .expect("opening mass fits in the narrow type");
    assert_eq!(expected_mass, 29_566_517_460_480_000);
    let mut observed_counts = Vec::new();

    for (matching_count, expected_count) in expected_counts.into_iter().enumerate() {
        let context = full_opening_fixture(matching_count);
        assert_eq!(context.physical_world_count(), Ok(OPENING_DEAL_COUNT));
        let projection = project_closed_form(context).expect("full-grade closed form");
        observed_counts.push(projection.cells().len());
        assert_eq!(projection.cells().len(), expected_count);
        assert_eq!(
            projection
                .total_scaled_mass()
                .expect("full-grade mass")
                .value(),
            expected_mass
        );
        assert_eq!(
            project_direct(context),
            Err(OpeningError::DirectWorldCapExceeded {
                world_count: OPENING_DEAL_COUNT,
                cap: M1_DIRECT_WORLD_CAP_V1,
            })
        );
    }

    assert_eq!(observed_counts.iter().copied().max(), Some(11_730));
    assert_eq!(observed_counts[6], 11_730);
}

#[test]
fn m1_direct_preflight_uses_exact_complete_world_work_units() {
    let expected = [(2, 90u64), (3, 1_680), (4, 34_650), (5, 756_756)];
    for (grade, world_count) in expected {
        let context = reduced_fixture(grade, usize::from(grade - 1));
        assert_eq!(context.physical_world_count(), Ok(world_count));
        if grade <= 4 {
            assert!(world_count <= M1_DIRECT_WORLD_CAP_V1);
            assert_eq!(
                direct_preflight(context),
                Ok(DirectPreflightV1::Admitted {
                    world_count,
                    cap: M1_DIRECT_WORLD_CAP_V1,
                })
            );
        } else {
            assert_eq!(
                direct_preflight(context),
                Ok(DirectPreflightV1::DeclaredStop {
                    world_count,
                    cap: M1_DIRECT_WORLD_CAP_V1,
                })
            );
            assert_eq!(
                project_direct(context),
                Err(OpeningError::DirectWorldCapExceeded {
                    world_count,
                    cap: M1_DIRECT_WORLD_CAP_V1,
                })
            );
        }
    }
}

#[test]
fn m1_response_kernel_reuse_holds_for_every_declaration_and_lead_tile() {
    let mut legal_leads_checked = 0usize;
    let mut repeated_context_checks = 0usize;

    // Four stable seven-tile hands partition the universe.  Thus every one of
    // the 28 physical lead identities is checked under all nine declarations.
    for hand_index in 0..4 {
        let start = hand_index * 7;
        let hand: DominoSet = Domino::ALL[start..start + 7].iter().copied().collect();
        for decl in Decl::ALL {
            let legal = legal_plays(decl, hand, None);
            assert_eq!(legal, hand);
            let mut by_context: BTreeMap<Context, walt_gpu_ref::OpeningProjection> =
                BTreeMap::new();
            for lead in legal.iter() {
                legal_leads_checked += 1;
                let led = decl.led_context(lead);
                let context = OpeningContext::from_opening_hand(decl, hand, led)
                    .expect("the selected legal lead represents its led context");
                let projection = project_closed_form(context).expect("response kernel");
                match by_context.get(&led) {
                    Some(previous) => {
                        repeated_context_checks += 1;
                        assert_eq!(previous, &projection);
                    }
                    None => {
                        by_context.insert(led, projection);
                    }
                }
            }
        }
    }

    assert_eq!(legal_leads_checked, Decl::COUNT * Domino::COUNT);
    assert!(repeated_context_checks > 0);
}
