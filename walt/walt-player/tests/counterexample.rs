use serde_json::{json, Value};
fn request() -> Value {
    json!({"contract":"nello","decl":8,"bid":1,"bidder":0,"seat":3,
        "hand":[4,7,12,14,16,25,27],"plays":[0,3,1,23,3,12,1,11,3,25,0,13,3,27,0,9,1,2],"seed":1})
}
#[test]
fn opt_in_replans_retained_witnesses_and_preserves_ordinary_estimates() {
    let call = json!({"request":request(),"worlds":160,"partner":false,"budget_ms":20000});
    let ordinary = walt_player::handle(&call.to_string(), |_| {});
    let mut enabled = call;
    enabled["nello_counterexamples"] = json!(true);
    let mut checkpoints = Vec::new();
    let result = walt_player::handle(&enabled.to_string(), |v| checkpoints.push(v.clone()));
    assert!(result.get("error").is_none(), "{result}");
    assert_eq!(
        result["evaluation"]["options"],
        ordinary["evaluation"]["options"]
    );
    assert_eq!(ordinary["choice"], 7);
    let r = &result["counterexample_result"];
    assert_eq!(r["status"], "completed");
    assert_eq!(r["rounds"], 3);
    assert_eq!(r["witnesses"], 12);
    assert_eq!(result["choice"], 16);
    assert_eq!(result["route"], "baseline-counterexamples");
    for round in 1..=3 {
        let checkpoint = checkpoints
            .iter()
            .find(|v| v["counterexample_result"]["rounds"] == round)
            .unwrap();
        assert_eq!(checkpoint["counterexample_result"]["witnesses"], round * 4);
        assert_eq!(
            checkpoint["counterexample_result"]["options"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
        assert!(checkpoint["legal"]
            .as_array()
            .unwrap()
            .contains(&checkpoint["choice"]));
    }
    let archived: Value = serde_json::from_str(include_str!(
        "../../probes/nello-counterexample-2026-09-22/panel-v2/cases/ply9-n160-seed1.json"
    ))
    .unwrap();
    let expected: Vec<_> = archived["evaluations"][3]["training"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| json!([p["action"], p["declarer_make"][0], p["declarer_make"][1]]))
        .collect();
    assert_eq!(r["options"], json!(expected));
}
#[test]
fn disabled_mode_and_strict_input_boundary_are_preserved() {
    let base = json!({"request":request(),"worlds":1,"partner":false});
    let mut disabled = base.clone();
    disabled["nello_counterexamples"] = json!(false);
    let a = walt_player::handle(&base.to_string(), |_| {});
    let b = walt_player::handle(&disabled.to_string(), |_| {});
    for key in ["choice", "route", "counterexample_result"] {
        assert_eq!(a[key], b[key]);
    }
    assert!(b.get("counterexample_result").is_none());
    let mut bad = base;
    bad["nello_counterexamples"] = json!(true);
    bad["request"]["hands"] = json!([]);
    assert!(walt_player::handle(&bad.to_string(), |_| {})
        .get("error")
        .is_some());
}
