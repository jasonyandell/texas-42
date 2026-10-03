use walt_response_ladder::{compiled::{Actor, SCHEMA}, compiled_player::{self, Config, Request}};

fn opening() -> Request {
    Request { decl: 6, bid: 30, bidder: 1, seat: 1, hand: vec![0, 6, 9, 15, 18, 24, 27],
        original_hand: vec![0, 6, 9, 15, 18, 24, 27], history: vec![], seed: 5538,
        budget_ms: 1000.0, config: Config { outer: 2, ..Default::default() } }
}

#[test]
fn equivalent_arena_rotations_have_identical_normalized_response_targets() {
    let request = opening();
    let mut rotated = request.clone(); rotated.bidder = 0; rotated.seat = 0;
    assert_eq!(compiled_player::normalize(&request).unwrap(), compiled_player::normalize(&rotated).unwrap());
    let c0 = Actor::new(SCHEMA, vec![8, 6, 2]).unwrap();
    let c1 = Actor::new(SCHEMA, vec![9, 4, 2]).unwrap();
    let a = compiled_player::decide(&request, &c0, &c1, None).unwrap();
    let b = compiled_player::decide(&rotated, &c0, &c1, None).unwrap();
    assert!(!a.fallback && !b.fallback);
    assert_eq!(a.tile, b.tile);
    assert_eq!(serde_json::to_value(a.report.unwrap()).unwrap(), serde_json::to_value(b.report.unwrap()).unwrap());
}

#[test]
fn zero_budget_retains_lawful_compiled_reserve_without_false_value() {
    let mut req = opening(); req.budget_ms = 0.0;
    let actor = Actor::new(SCHEMA, vec![8, 4]).unwrap();
    let (decl, viewer, hand, _, public) = compiled_player::normalize(&req).unwrap();
    let expected = actor.choose(decl, viewer, hand, &public).unwrap().action;
    let answer = compiled_player::decide(&req, &actor, &actor, None).unwrap();
    assert!(answer.fallback);
    assert_eq!(answer.tile, expected);
    assert!(answer.report.is_none());
}

#[test]
fn hidden_deal_fields_and_nonfinite_deadlines_are_rejected() {
    let mut value = serde_json::to_value(opening()).unwrap();
    value["opponent_hands"] = serde_json::json!([[1,2,3]]);
    assert!(serde_json::from_value::<Request>(value).is_err());
    let mut req = opening(); req.budget_ms = f64::INFINITY;
    assert!(compiled_player::decide(&req, &Actor::empty(), &Actor::empty(), None).is_err());
}
