use std::time::Duration;

use walt::rules::Decl;
use walt::solver::SplitMix64;
use walt_response_ladder::compiled::{Actor, SCHEMA};
use walt_response_ladder::compiled_field::CompiledField;
use walt_response_ladder::mechanics::{tiles, Budget, Field, FieldQuery, PublicState};

fn shuffled_hands(seed: u64) -> [u32; 4] {
    let mut deck: Vec<u8> = (0..28).collect();
    let mut rng = SplitMix64(seed);
    for i in (1..deck.len()).rev() {
        deck.swap(i, rng.below((i + 1) as u64) as usize);
    }
    std::array::from_fn(|seat| {
        deck[seat * 7..seat * 7 + 7]
            .iter()
            .fold(0, |mask, &tile| mask | (1 << tile))
    })
}

#[test]
fn compiled_field_action_matches_strict_actor_reference() {
    let actors = [
        Actor::empty(),
        Actor::new(SCHEMA, vec![0]).unwrap(),
        Actor::new(SCHEMA, vec![8]).unwrap(),
        Actor::new(SCHEMA, vec![5, 8]).unwrap(),
        Actor::new(SCHEMA, vec![5, 8, 2]).unwrap(),
        Actor::new(SCHEMA, vec![12, 1, 8]).unwrap(),
    ];

    for (decl_index, &decl) in Decl::ALL.iter().enumerate() {
        for leader in 0..4u8 {
            let original = shuffled_hands(0xC0DE_0000 + decl_index as u64 * 17 + leader as u64);
            let mut remaining = original;
            let mut public = PublicState::opening(leader);
            for ply in 0..28 {
                let viewer = public.actor();
                let hand = remaining[viewer as usize] & !public.played;
                let tape = (ply as u64).wrapping_mul(0x9E37_79B9);
                for actor in &actors {
                    let mut field = CompiledField::all(actor).unwrap();
                    let mut budget = Budget::new(2, Duration::from_secs(1));
                    let got = field
                        .choose(
                            FieldQuery {
                                decl,
                                bid: 30,
                                seat: viewer,
                                hand,
                                public: &public,
                                tape,
                            },
                            &mut budget,
                        )
                        .expect("lawful compiled field query");
                    let expected = actor.choose(decl, viewer, hand, &public).unwrap().action;
                    assert_eq!(got, expected, "decl {decl_index} leader {leader} ply {ply}");
                }
                let tile = tiles(public.legal(decl, hand))[0];
                remaining[viewer as usize] &= !(1 << tile);
                public = public.after(decl, tile);
            }
        }
    }
}
