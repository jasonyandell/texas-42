use std::collections::BTreeMap;
use walt::{policy_search::{fixture, learning_io::{lesson_text,rational,read_lessons,request_text}}, rules::legal_plays, solver::decl_of};

#[test]
fn public_lesson_roundtrip_preserves_costs_weights_and_observation() {
    let f=fixture(910002,decl_of(6),3,None).unwrap();
    let costs=legal_plays(f.exercise.position.decl,f.exercise.root.kernel().viewer_hand(),f.exercise.frame.led_context())
        .iter().enumerate().map(|(i,d)|(d,rational(if i==0{"0"}else{"1/3"}).unwrap())).collect::<BTreeMap<_,_>>();
    let text=lesson_text(&request_text(&f,&f.history,910002),&rational("2/7").unwrap(),&costs);
    let restored=read_lessons(&text).unwrap();
    assert_eq!(restored.len(),1);
    assert_eq!(restored[0].frame(),&f.exercise.frame);
    assert_eq!(restored[0].history(),f.history);
    assert_eq!(restored[0].costs(),&costs);
    assert_eq!(restored[0].group_weight(),&rational("2/7").unwrap());
    assert!(read_lessons(&text.replace("weight 2/7","weight 2/7\nworld 1 2 3")).is_err());
    assert!(read_lessons(&text.replace("weight 2/7","weight 2/7\nweight 1")).is_err());
}

#[test]
fn rational_wire_format_refuses_undefined_or_ambiguous_numbers() {
    for bad in ["", "1/0", "1/-2", "1/2/3", "NaN", "0.5"] {assert!(rational(bad).is_err(),"{bad}");}
    assert_eq!(rational("2/4").unwrap(),rational("1/2").unwrap());
    assert!(read_lessons("\n---\n").is_err());
}
