//! Exhaustive M1 same-context reuse and action-identity boundary.
//!
//! The frozen field law permits response-kernel reuse for two opening leads
//! with the same effective led context.  It does not permit the two root
//! actions, or their later semantic successors, to become one identity.

use walt_core::{Decl, Domino, DominoSet, Seat};
use walt_gpu_ref::{
    project_closed_form, validate_opening_run_envelope_v1, BuildIdentityV1, OpeningContractV1,
    OpeningRootV1,
};

const TEST_BUILD_IDENTITY: [u8; 32] = [0x3c; 32];

fn pair_root(decl: Decl, first: Domino, second: Domino) -> OpeningRootV1 {
    let hand: DominoSet = [first, second]
        .into_iter()
        .chain(
            Domino::ALL
                .into_iter()
                .filter(|tile| *tile != first && *tile != second)
                .take(5),
        )
        .collect();
    assert_eq!(hand.len(), 7);
    OpeningRootV1::new(
        decl,
        Seat::S0,
        hand,
        OpeningContractV1::point_bid(30).expect("minimum point contract"),
    )
    .expect("every distinct lead pair extends to a valid opening root")
}

#[test]
fn m1_same_context_reuse_and_action_identity_hold_for_every_physical_lead_pair() {
    let build_identity =
        BuildIdentityV1::new(TEST_BUILD_IDENTITY).expect("nonzero test build identity");
    let mut total_pairs = 0usize;

    for decl in Decl::ALL {
        let mut declaration_pairs = 0usize;
        for first_index in 0..Domino::COUNT {
            let first = Domino::ALL[first_index];
            for second in Domino::ALL.into_iter().skip(first_index + 1) {
                let led = decl.led_context(first);
                if decl.led_context(second) != led {
                    continue;
                }
                declaration_pairs += 1;
                total_pairs += 1;

                let root = pair_root(decl, first, second);
                assert!(root.legal_leads().contains(first));
                assert!(root.legal_leads().contains(second));

                // Derive the two queries through their distinct physical
                // actions before observing that the reusable kernel input is
                // identical.
                let first_context = root
                    .opening_context(decl.led_context(first))
                    .expect("first action context");
                let second_context = root
                    .opening_context(decl.led_context(second))
                    .expect("second action context");
                assert_eq!(first_context, second_context);
                let first_projection =
                    project_closed_form(first_context).expect("first response projection");
                let second_projection =
                    project_closed_form(second_context).expect("second response projection");
                assert_eq!(
                    first_projection, second_projection,
                    "response payload drift for {decl:?}, {first}, {second}"
                );

                let first_envelope = first_projection
                    .canonical_run_envelope_bytes(root, first, build_identity)
                    .expect("first action-bound envelope");
                let second_envelope = second_projection
                    .canonical_run_envelope_bytes(root, second, build_identity)
                    .expect("second action-bound envelope");
                assert_ne!(
                    first_envelope, second_envelope,
                    "same-context actions collapsed for {decl:?}, {first}, {second}"
                );

                let first_verified =
                    validate_opening_run_envelope_v1(&first_envelope, build_identity)
                        .expect("first bound envelope");
                let second_verified =
                    validate_opening_run_envelope_v1(&second_envelope, build_identity)
                        .expect("second bound envelope");
                assert_eq!(first_verified.selected_action(), first);
                assert_eq!(second_verified.selected_action(), second);
                assert_eq!(
                    first_verified.projector_payload_sha256(),
                    second_verified.projector_payload_sha256(),
                    "same-context payload hash drift for {decl:?}, {first}, {second}"
                );
                assert_ne!(
                    first_verified.semantic_identity_sha256(),
                    second_verified.semantic_identity_sha256(),
                    "root-action identity collapsed for {decl:?}, {first}, {second}"
                );
            }
        }

        // Each declaration partitions the 28 physical leads into effective
        // contexts whose within-context unordered pair count is 56.
        assert_eq!(declaration_pairs, 56, "pair census drift for {decl:?}");
    }

    assert_eq!(total_pairs, Decl::COUNT * 56);
    assert_eq!(total_pairs, 504);
}
