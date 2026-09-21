use serde_json::{json, Value};
use walt::rules::{Decl, Domino, Seat, DOUBLES};
use walt::solver::{self, Contract, Key};

fn fixtures() -> Vec<Value> {
    serde_json::from_str(include_str!("fixtures/nello.json")).unwrap()
}
fn key(st: &solver::Replayed) -> Key {
    Key {
        voids: None,
        played: st.played,
        leader: st.leader,
        plays: st.plays.clone(),
        banked_t1: st.banked_t1,
        banked_t0: st.banked_t0,
        alive: 0,
    }
}
fn wire(f: &Value, prefix: &[usize], seat: usize) -> String {
    let words = |xs: Vec<usize>| {
        xs.iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    };
    let hand = f["hands"][seat]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect();
    format!(
        "status\ncontract 1\ndecl 8\nbid 1\nbidder {}\nseat {seat}\nhand {}\nplays {}\nseed 1\n",
        f["declarer"],
        words(hand),
        words(prefix.to_vec())
    )
}

#[test]
fn original_doubles_suit_algebra_and_every_ordered_triple() {
    assert_eq!(Decl::ALL.len(), 10);
    assert_eq!(Decl::STRAIGHT.len(), 9);
    assert!(!Decl::STRAIGHT.contains(&Decl::DoublesSuit));
    let d = Decl::DoublesSuit;
    assert_eq!(d.called_set(), DOUBLES);
    assert!(d.powered_set().is_empty());
    assert_eq!("DS".parse::<Decl>().unwrap(), d);
    for declarer in Seat::ALL {
        let c = Contract::Nello { declarer };
        for leader in Seat::ALL.into_iter().filter(|s| *s != declarer.plus(2)) {
            for a in Domino::ALL {
                for b in Domino::ALL {
                    for e in Domino::ALL {
                        if a == b || a == e || b == e {
                            continue;
                        }
                        let tiles = [a, b, e];
                        // Independent prose rules: highest double on a double lead;
                        // otherwise highest mixed follower of the lead's high pip.
                        let follows = |t: Domino| {
                            if a.is_double() {
                                t.is_double()
                            } else {
                                !t.is_double() && t.has(a.hi())
                            }
                        };
                        let rank = |t: Domino| {
                            if t.is_double() {
                                t.hi().value()
                            } else {
                                t.pip_sum()
                            }
                        };
                        let mut best = 0;
                        for i in 1..3 {
                            if follows(tiles[i]) && rank(tiles[i]) > rank(tiles[best]) {
                                best = i;
                            }
                        }
                        let mut expected = leader;
                        for _ in 0..best {
                            expected = expected.successor();
                            if expected == declarer.plus(2) {
                                expected = expected.successor();
                            }
                        }
                        assert_eq!(
                            c.winner(d, leader.index(), &tiles.map(|t| t.index() as u8)),
                            expected
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn pinned_plunge_histories_all_seats_and_hidden_capacities() {
    for f in fixtures() {
        let bidder = f["declarer"].as_u64().unwrap() as usize;
        let plays = f["plays"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_u64().unwrap() as usize)
            .collect::<Vec<_>>();
        for end in (0..=plays.len()).step_by(2) {
            let pairs = plays[..end]
                .chunks_exact(2)
                .map(|p| (p[0], p[1]))
                .collect::<Vec<_>>();
            let st = solver::replay_contract(Decl::DoublesSuit, bidder, &pairs, true);
            let c = Contract::Nello {
                declarer: Seat::from_index((bidder + st.r) % 4).unwrap(),
            };
            let k = key(&st);
            let sizes = c.sizes(&k, 0, 7);
            assert_eq!(sizes[c.inactive().unwrap().index()], 7);
            assert_eq!(sizes.iter().sum::<usize>() + end / 2, 28);
            if end < plays.len() {
                assert_eq!(c.terminal(&k), None);
                let actor = plays[end];
                let reply: Value = serde_json::from_str(
                    &solver::partnership_wire::run(&wire(&f, &plays[..end], actor)).unwrap(),
                )
                .unwrap();
                assert!(reply["legal"]
                    .as_array()
                    .unwrap()
                    .contains(&json!(plays[end + 1])));
                assert_eq!(reply["trick"], json!(end / 6 + 1));
                let missing_contract = wire(&f, &plays[..end], actor).replace("contract 1\n", "");
                assert!(solver::partnership_wire::run(&missing_contract).is_err());
            } else {
                assert_eq!(c.terminal(&k), Some(f["kind"] == "make"));
                assert_eq!(st.completed, f["winners"].as_array().unwrap().len());
                let points = if st.r == 0 {
                    [st.banked_t0, st.banked_t1]
                } else {
                    [st.banked_t1, st.banked_t0]
                };
                assert_eq!(json!(points), f["points"]);
                assert!(
                    solver::partnership_wire::run(&wire(&f, &plays, (bidder + 1) % 4)).is_err()
                );
                if f["kind"] == "make" {
                    assert_eq!(points.iter().sum::<u8>(), 37);
                }
            }
        }
    }
}

#[test]
fn budgeted_nello_preserves_legal_checkpoint_and_contract() {
    let f = &fixtures()[4];
    let req = json!({"contract":"nello","decl":8,"bid":1,"bidder":0,"seat":3,
        "hand":f["hands"][3],"plays":f["plays"].as_array().unwrap()[..4],"seed":20});
    let mut checkpoints = Vec::new();
    let response = walt_player::handle(
        &json!({"request":req,"worlds":40,"partner":true,"budget_ms":100}).to_string(),
        |v| checkpoints.push(v.clone()),
    );
    assert!(response.get("error").is_none(), "{response}");
    assert!(!checkpoints.is_empty());
    for value in checkpoints.iter().chain(std::iter::once(&response)) {
        assert_eq!(value["contract"], "nello");
        assert_eq!(value["inactive"], 2);
        assert_eq!(value["review"], "inapplicable-nello");
        assert!(value["legal"]
            .as_array()
            .unwrap()
            .contains(&value["choice"]));
    }
    let mut bad = req.clone();
    bad["plays"] = json!([0, 23, 2, 22]);
    assert!(
        walt_player::handle(&json!({"request":bad}).to_string(), |_| {})
            .get("error")
            .is_some()
    );
    let mut bad = req;
    bad["hands"] = f["hands"].clone();
    assert!(
        walt_player::handle(&json!({"request":bad}).to_string(), |_| {})
            .get("error")
            .is_some()
    );
}

#[test]
fn contract_identity_prevents_cross_contract_policy_cache_reuse() {
    use solver::{mask_of, set_of, Deadline, Field, Shared, Solver};
    use std::{sync::Arc, time::Duration};
    let f = &fixtures()[4];
    let st = solver::replay_contract(Decl::DoublesSuit, 0, &[], true);
    let k = key(&st);
    let hand = f["hands"][0]
        .as_array()
        .unwrap()
        .iter()
        .fold(0u32, |h, t| h | 1 << t.as_u64().unwrap());
    let contract = Contract::Nello { declarer: Seat::S1 };
    let fresh = || {
        Shared::new(
            Decl::DoublesSuit,
            1,
            vec![2],
            0,
            7,
            Deadline::after(Duration::from_secs(10)),
        )
    };
    let shared = Arc::new(fresh().with_contract(contract));
    let host = Solver::new(
        Arc::clone(&shared),
        Seat::S1,
        hand,
        true,
        vec![],
        vec![],
        Field::Level(0),
    );
    let legal = mask_of(walt::rules::legal_plays(
        Decl::DoublesSuit,
        set_of(hand),
        None,
    ));
    let choice = host.modeled_choice(0, &k, Seat::S1, hand, legal).unwrap();
    drop(host);
    let mut source = Arc::try_unwrap(shared).ok().unwrap();
    let entries = source.pi_cache_len();
    assert!(entries > 0);
    assert_eq!(fresh().take_policy_cache_from(&mut source), 0);
    assert_eq!(
        fresh()
            .with_contract(Contract::Nello { declarer: Seat::S3 })
            .take_policy_cache_from(&mut source),
        0
    );
    let mut same = fresh().with_contract(contract);
    assert_eq!(same.take_policy_cache_from(&mut source), entries);
    let host = Solver::new(
        Arc::new(same),
        Seat::S1,
        hand,
        true,
        vec![],
        vec![],
        Field::Level(0),
    );
    assert_eq!(
        host.modeled_choice(0, &k, Seat::S1, hand, legal),
        Some(choice)
    );
}
