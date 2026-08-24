use walt::rules::{Decl, Domino, DominoSet, Pip, Seat};
use walt_gpu_ref::{
    direct_preflight, project_closed_form, project_direct, DeclaringTeamMakesV1, DirectPreflightV1,
    IgnoreAuctionEvidenceV1, OpeningContractV1, OpeningError, OpeningRootV1,
    OpeningStraightHand21FieldActionsV1, ReducedOpeningCarrierV1, UniformCompatibleOpeningDealsV1,
    UniformRandomLegalV1, M1_DIRECT_WORLD_CAP_V1, OPENING_MODEL_PROFILE_V1,
};

fn brute_lex_least_pool(
    root_pool: DominoSet,
    matching: DominoSet,
    target: usize,
    matching_target: usize,
) -> Option<DominoSet> {
    // Keep the independent brute-force oracle's complete recursion state
    // explicit; bundling it would add a helper representation shared by no
    // production path and make this small test oracle harder to audit.
    #[allow(clippy::too_many_arguments)]
    fn search(
        tiles: &[Domino],
        matching: DominoSet,
        target: usize,
        matching_target: usize,
        start: usize,
        chosen: DominoSet,
        chosen_len: usize,
        chosen_matching: usize,
    ) -> Option<DominoSet> {
        if chosen_len == target {
            return (chosen_matching == matching_target).then_some(chosen);
        }
        let needed = target - chosen_len;
        if tiles.len().saturating_sub(start) < needed || chosen_matching > matching_target {
            return None;
        }
        for index in start..=tiles.len() - needed {
            let tile = tiles[index];
            let next_matching = chosen_matching + usize::from(matching.contains(tile));
            if next_matching > matching_target {
                continue;
            }
            if let Some(result) = search(
                tiles,
                matching,
                target,
                matching_target,
                index + 1,
                chosen.union(DominoSet::single(tile)),
                chosen_len + 1,
                next_matching,
            ) {
                return Some(result);
            }
        }
        None
    }

    let tiles: Vec<Domino> = root_pool.iter().collect();
    search(
        &tiles,
        matching,
        target,
        matching_target,
        0,
        DominoSet::EMPTY,
        0,
        0,
    )
}

fn hand(names: [&str; 7]) -> DominoSet {
    names
        .into_iter()
        .map(|name| name.parse::<Domino>().expect("explicit root tile"))
        .collect()
}

/// Small declared production-root corpus.  Both roots use the same explicit
/// focal identities so the declaration change exercises called absorption:
/// NT represents q5/q6, while P6 represents q5/called.
fn production_roots() -> [OpeningRootV1; 2] {
    let focal_hand = hand(["6-0", "6-1", "6-2", "6-3", "6-4", "6-5", "5-5"]);
    [
        OpeningRootV1::new(
            Decl::NoTrump,
            Seat::S0,
            focal_hand,
            OpeningContractV1::point_bid(30).expect("minimum point contract"),
        )
        .expect("declared no-trump root"),
        OpeningRootV1::new(
            Decl::PipTrump(Pip::new(6).expect("six is a pip")),
            Seat::S2,
            focal_hand,
            OpeningContractV1::Mark,
        )
        .expect("declared sixes root"),
    ]
}

fn expected_world_count(grade: u8) -> u64 {
    match grade {
        2 => 90,
        3 => 1_680,
        4 => 34_650,
        5 => 756_756,
        _ => panic!("carrier emitted an out-of-range grade"),
    }
}

#[test]
fn opening_root_v1_closes_roles_contract_public_state_and_profiles() {
    assert!(matches!(
        OpeningContractV1::point_bid(29),
        Err(OpeningError::PointBidOutOfRange { value: 29 })
    ));
    assert!(matches!(
        OpeningContractV1::point_bid(42),
        Err(OpeningError::PointBidOutOfRange { value: 42 })
    ));
    assert_eq!(
        OpeningContractV1::point_bid(30)
            .expect("minimum point bid")
            .loss_budget(),
        12
    );
    assert_eq!(
        OpeningContractV1::point_bid(41)
            .expect("maximum point bid")
            .loss_budget(),
        1
    );
    assert_eq!(OpeningContractV1::Mark.loss_budget(), 0);

    let roots = production_roots();
    assert_eq!(
        roots[0].led_contexts(),
        vec![
            walt::rules::Context::Natural(Pip::new(5).expect("five is a pip")),
            walt::rules::Context::Natural(Pip::new(6).expect("six is a pip")),
        ]
    );
    assert_eq!(
        roots[1].led_contexts(),
        vec![
            walt::rules::Context::Natural(Pip::new(5).expect("five is a pip")),
            walt::rules::Context::Called,
        ]
    );

    for root in roots {
        assert_eq!(root.focal(), root.bidder());
        assert_eq!(root.focal(), root.leader());
        assert_eq!(root.focal(), root.actor());
        assert_eq!(root.focal_hand().len(), 7);
        assert_eq!(root.legal_leads(), root.focal_hand());
        assert_eq!(root.hidden_pool().len(), 21);
        assert_eq!(root.public_play_count(), 0);
        assert_eq!(root.current_trick_len(), 0);
        assert!(root.loss_budget() <= 12);

        let profile = root.model_profile();
        assert_eq!(profile, OPENING_MODEL_PROFILE_V1);
        assert_eq!(profile.auction_evidence(), IgnoreAuctionEvidenceV1);
        assert_eq!(profile.prior(), UniformCompatibleOpeningDealsV1);
        assert_eq!(profile.field(), UniformRandomLegalV1);
        assert_eq!(profile.utility(), DeclaringTeamMakesV1);
        assert_eq!(profile.horizon(), OpeningStraightHand21FieldActionsV1);
    }
}

#[test]
fn reduced_carrier_v1_is_generated_in_order_and_drives_complete_bounded_parity() {
    for root in production_roots() {
        let carrier = ReducedOpeningCarrierV1::from_root(root).expect("generated carrier");
        assert_eq!(carrier.root(), root);
        assert_eq!(carrier.coordinates().len(), 32);

        let keys: Vec<(u8, usize, u8)> = carrier
            .coordinates()
            .iter()
            .map(|coordinate| {
                (
                    coordinate.grade(),
                    coordinate.led().index(),
                    coordinate.matching_count(),
                )
            })
            .collect();
        assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));

        let root_contexts = root.led_contexts();
        assert!(root_contexts
            .windows(2)
            .all(|pair| pair[0].index() < pair[1].index()));
        assert_eq!(root_contexts.len(), 2);
        let mut expected_keys = Vec::new();
        for grade in 2u8..=5 {
            for &led in &root_contexts {
                let matching = root
                    .hidden_pool()
                    .intersection(root.decl().effective_incidence(led));
                let nonmatching = root.hidden_pool().difference(matching);
                for matching_count in 0u8..=6 {
                    let selected_matching = usize::from(matching_count);
                    let target = usize::from(grade) * 3;
                    if selected_matching <= matching.len()
                        && target - selected_matching <= nonmatching.len()
                    {
                        expected_keys.push((grade, led.index(), matching_count));
                    }
                }
            }
        }
        assert_eq!(keys, expected_keys);

        for coordinate in carrier.coordinates() {
            let grade = coordinate.grade();
            let context = coordinate
                .opening_context()
                .expect("generated coordinate is valid");
            let selected_matching = usize::from(coordinate.matching_count());
            assert_eq!(context.pool().len(), usize::from(grade) * 3);
            assert!(context.pool().is_subset_of(root.hidden_pool()));
            assert_eq!(context.matching_pool().len(), selected_matching);
            assert_eq!(
                context.physical_world_count(),
                Ok(expected_world_count(grade))
            );

            let root_matching = root
                .hidden_pool()
                .intersection(root.decl().effective_incidence(coordinate.led()));
            let root_nonmatching = root.hidden_pool().difference(root_matching);
            let selected_nonmatching = usize::from(grade) * 3 - selected_matching;
            let expected_pool: DominoSet = root_matching
                .iter()
                .take(selected_matching)
                .chain(root_nonmatching.iter().take(selected_nonmatching))
                .collect();
            assert_eq!(coordinate.pool(), expected_pool);
            assert_eq!(
                coordinate.pool(),
                brute_lex_least_pool(
                    root.hidden_pool(),
                    root_matching,
                    usize::from(grade) * 3,
                    selected_matching,
                )
                .expect("the generated carrier coordinate is feasible")
            );

            let closed = project_closed_form(context).expect("closed carrier coordinate");
            assert_eq!(
                closed.total_scaled_mass().expect("carrier mass"),
                closed
                    .expected_scaled_mass()
                    .expect("expected carrier mass")
            );
            match grade {
                2..=4 => {
                    assert_eq!(
                        direct_preflight(context),
                        Ok(DirectPreflightV1::Admitted {
                            world_count: expected_world_count(grade),
                            cap: M1_DIRECT_WORLD_CAP_V1,
                        })
                    );
                    let direct = project_direct(context).expect("bounded direct coordinate");
                    assert_eq!(closed, direct);
                    assert_eq!(
                        closed.response_aggregates().expect("closed responses"),
                        direct.response_aggregates().expect("direct responses")
                    );
                }
                5 => {
                    assert_eq!(
                        direct_preflight(context),
                        Ok(DirectPreflightV1::DeclaredStop {
                            world_count: 756_756,
                            cap: M1_DIRECT_WORLD_CAP_V1,
                        })
                    );
                    assert_eq!(
                        project_direct(context),
                        Err(OpeningError::DirectWorldCapExceeded {
                            world_count: 756_756,
                            cap: M1_DIRECT_WORLD_CAP_V1,
                        })
                    );
                }
                _ => panic!("carrier emitted an out-of-range grade"),
            }
        }
    }
}
