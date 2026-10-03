use walt::rules::Decl;

use walt_response_ladder::compiled::{Actor, SCHEMA};
use walt_response_ladder::mechanics::PublicState;
use walt_response_ladder::policy::{Decision, Policy};

#[test]
fn old_json_deserializes_without_a_compiled_tail_and_omits_none() {
    let json = r#"{"viewer":0,"priority":[1,2],"decisions":[]}"#;
    let policy: Policy = serde_json::from_str(json).unwrap();
    assert!(policy.compiled_tail.is_none());
    let encoded = serde_json::to_string(&policy).unwrap();
    assert!(!encoded.contains("compiled_tail"));
}

#[test]
fn explicit_history_decisions_override_a_total_tail() {
    let public = PublicState {
        played: 1,
        leader: 0,
        plays: vec![],
        banked_t1: 0,
        banked_t0: 0,
        voids: [0; 4],
        history: vec![0],
    };
    let tail = Actor::new(SCHEMA, vec![0]).unwrap();
    let mut policy = Policy::priority(0, vec![6, 5, 4, 3, 2, 1]);
    policy.compiled_tail = Some(tail);
    let hand = (1u32 << 1) | (1u32 << 2) | (1u32 << 6);
    assert_eq!(policy.choose(Decl::NoTrump, &public, hand), Some(1));
    policy.decisions.push(Decision {
        history: public.history.clone(),
        action: 6,
    });
    assert_eq!(policy.choose(Decl::NoTrump, &public, hand), Some(6));
}

#[test]
fn tail_receives_only_fixed_own_hand_after_public_tiles_on_unseen_history() {
    let public = PublicState {
        played: (1u32 << 0) | (1u32 << 4),
        leader: 0,
        plays: vec![],
        banked_t1: 0,
        banked_t0: 0,
        voids: [0; 4],
        history: vec![0, 4],
    };
    let mut policy = Policy::priority(0, vec![6, 5, 3, 2, 1]);
    policy.compiled_tail = Some(Actor::new(SCHEMA, vec![0]).unwrap());
    let hand = (1u32 << 0) | (1u32 << 1) | (1u32 << 2) | (1u32 << 6);
    assert_eq!(policy.choose(Decl::NoTrump, &public, hand), Some(1));
}

#[test]
fn malformed_tail_is_rejected_before_pricing() {
    let policy = Policy {
        viewer: 0,
        priority: vec![],
        decisions: vec![],
        compiled_tail: Some(Actor {
            schema: "wrong-schema".into(),
            clauses: vec![],
            lead_clauses: None,
        }),
    };
    assert!(policy.validate().is_err());
}
