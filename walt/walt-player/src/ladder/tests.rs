use super::*;
#[test]
fn native_reference_parity() {
    let fixtures: Value = serde_json::from_str(include_str!("golden.json")).unwrap();
    for c in fixtures["cases"].as_array().unwrap() {
        let samples = serde_json::from_value::<Vec<usize>>(c["samples"].clone()).unwrap();
        for offset in 0..4 {
            let mut r = c["request"].clone();
            for key in ["bidder", "seat"] {
                r[key] = json!((r[key].as_u64().unwrap() + offset) % 4);
            }
            for i in (0..r["plays"].as_array().unwrap().len()).step_by(2) {
                r["plays"][i] = json!((r["plays"][i].as_u64().unwrap() + offset) % 4);
            }
            let req: Request = serde_json::from_value(r).unwrap();
            let mut job = Job::new(&req, &samples, 30000).unwrap();
            let got = loop {
                if let Some(v) = job.step().unwrap() {
                    break v;
                }
            };
            assert_eq!(got["choice"], c["choice"], "{c}");
            assert_eq!(got["evaluation"]["options"], c["options"], "{c}");
        }
    }
}

#[test]
fn rules_match_authoritative_algebra() {
    use walt::rules::{Context, Decl, Domino};
    for decl in Decl::ALL {
        let id = match decl {
            Decl::PipTrump(p) => p.value() as usize,
            Decl::DoublesTrump => 7,
            Decl::DoublesSuit => 8,
            Decl::NoTrump => 9,
        };
        let r = Rules::new(id, 30, id == 8);
        for lead in 0..28 {
            let led = decl.led_context(Domino::from_index(lead).unwrap());
            assert_eq!(r.lead[lead], led.index());
        }
        for q in 0..8 {
            for t in 0..28 {
                let tile = Domino::from_index(t).unwrap();
                let ctx = Context::from_index(q).unwrap();
                assert_eq!(r.suit[q] & (1 << t) != 0, decl.follows(tile, ctx));
                for other in 0..28 {
                    // Rank ties among off-suit tiles never decide a trick. Compare
                    // the entire ordering as well, since both algebras define it.
                    let that = Domino::from_index(other).unwrap();
                    assert_eq!(
                        r.strength[q][t].cmp(&r.strength[q][other]),
                        decl.trick_key(tile, ctx).cmp(&decl.trick_key(that, ctx)),
                        "{decl:?} {q} {t} {other}"
                    );
                }
            }
        }
    }
}
#[test]
fn nello_keeps_four_physical_hands_and_has_three_active_seats() {
    let rules = Rules::new(8, 1, true);
    let mut p = Public::initial();
    let hand = (1 << 0) | (1 << 2) | (1 << 5) | (1 << 9) | (1 << 14) | (1 << 20) | (1 << 27);
    let support = support::Support::new(&rules, p, hand).unwrap();
    assert_eq!(support.total(), 399072960);
    for rank in [0, 1, 12345, support.total() - 1] {
        let w = support.unrank(hand, rank);
        assert_eq!(w[1], hand);
        assert!(w.iter().all(|h| h.count_ones() == 7));
        assert_eq!(w.iter().fold(0, |a, h| a | h), 0xfffffff);
    }
    // Declarer leads 0-0, defenders follow with larger doubles: no trump power.
    p = rules.play(p, 0);
    assert_eq!(rules.turn(p), 2);
    p = rules.play(p, 2);
    assert_eq!(rules.turn(p), 0);
    p = rules.play(p, 5);
    assert_eq!(p.leader, 0);
    assert_eq!(rules.sizes(p), [6, 6, 6, 7]);
    assert_eq!(rules.outcome(p), None);
    // Inactive seat 3 stays absent even when seat 2 leads.
    assert_eq!(
        (0..3).map(|i| rules.actor(2, i)).collect::<Vec<_>>(),
        vec![2, 0, 1]
    );
    let won = rules.play(rules.play(rules.play(Public::initial(), 27), 0), 2);
    assert_eq!(rules.outcome(won), Some(false));
}
#[test]
fn information_boundary_rejects_inconsistent_observations() {
    let request =
        json!({"decl":3,"bid":30,"bidder":1,"seat":1,"seed":0,"hand":[0,1,2,3,4,5,6],"plays":[]});
    for patch in [
        json!({"seat":0}),
        json!({"hand":[0,0,1,2,3,4,5]}),
        json!({"plays":[1,27,1,26]}),
        json!({"decl":8}),
    ] {
        let mut r = request.clone();
        for (k, v) in patch.as_object().unwrap() {
            r[k] = v.clone();
        }
        let req: Request = serde_json::from_value(r).unwrap();
        assert!(Job::new(&req, &[24, 160], 1000).is_err());
    }
    let mut hidden = request;
    hidden["hands"] = json!([[0], [1], [2], [3]]);
    assert!(serde_json::from_value::<Request>(hidden).is_err());
}

// Small independent, unpruned L1 traversal: partition before choosing. This
// intentionally has no caches, frame arena, singleton shortcut or bounds.
fn oracle(r: &Rules, p: Public, me: usize, worlds: &[World], fibers: &[usize], path: u64) -> i32 {
    if let Some(made) = r.outcome(p) {
        return if made { fibers.len() as i32 } else { 0 };
    }
    let actor = r.turn(p);
    let mut children: [Vec<usize>; 28] = std::array::from_fn(|_| Vec::new());
    for &i in fibers {
        let mut legal = r.legal(worlds[i][actor] & !p.played, p);
        if actor != me && legal.count_ones() > 1 {
            let mut rng = Random(mix(path ^ mix((i as u64).wrapping_add(0xd1b54a32d192ed03))));
            for _ in 0..rng.bounded(legal.count_ones() as u64) {
                legal &= legal - 1;
            }
            legal &= legal.wrapping_neg();
        }
        while legal != 0 {
            let t = legal.trailing_zeros() as usize;
            legal &= legal - 1;
            children[t].push(i);
        }
    }
    let values = children
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.is_empty())
        .map(|(t, c)| oracle(r, r.play(p, t), me, worlds, c, child_path(path, t)))
        .collect::<Vec<_>>();
    if actor == me {
        if me % 2 == 1 {
            *values.iter().max().unwrap()
        } else {
            *values.iter().min().unwrap()
        }
    } else {
        values.iter().sum()
    }
}
#[test]
fn shared_choices_match_unpruned_oracle_and_differ_from_strategy_fusion() {
    let fixtures: Value = serde_json::from_str(include_str!("golden.json")).unwrap();
    let mut witness = None;
    for c in fixtures["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["request"]["plays"].as_array().unwrap().len() >= 24)
    {
        let req: Request = serde_json::from_value(c["request"].clone()).unwrap();
        for seed in 0..16 {
            let mut job = Job::new(&req, &[8], 30000).unwrap();
            let p = job.public;
            let me = job.search.rules.turn(p);
            let path = mix(seed);
            let shared = oracle(&job.search.rules, p, me, &job.worlds, &job.fibers, path);
            let got = job
                .search
                .search(1, p, me, &job.worlds, &job.fibers, path, None)
                .unwrap();
            assert!(got.exact);
            assert_eq!(got.count, shared);
            let fused = job
                .fibers
                .iter()
                .map(|&i| oracle(&job.search.rules, p, me, &job.worlds, &[i], path))
                .sum::<i32>();
            if shared != fused {
                witness = Some((seed, p.depth, shared, fused));
                break;
            }
        }
        if witness.is_some() {
            break;
        }
    }
    assert!(
        witness.is_some(),
        "keep a distinguishing shared-choice witness"
    );
    eprintln!("delta-1 fusion witness: {witness:?}");
}

#[test]
fn nello_search_matches_shared_choice_oracle() {
    let fixtures: Value = serde_json::from_str(include_str!("nello.json")).unwrap();
    let mut cases = 0;
    for c in fixtures
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["request"]["plays"].as_array().unwrap().len() >= 24)
    {
        let req: Request = serde_json::from_value(c["request"].clone()).unwrap();
        let mut job = Job::new(&req, &[8], 30000).unwrap();
        let p = job.public;
        let me = job.search.rules.turn(p);
        let want = oracle(&job.search.rules, p, me, &job.worlds, &job.fibers, job.path);
        let got = job
            .search
            .search(1, p, me, &job.worlds, &job.fibers, job.path, None)
            .unwrap();
        assert!(got.exact);
        assert_eq!(got.count, want);
        cases += 1;
        assert!(job.worlds.iter().all(|w| w[3].count_ones() == 7));
    }
    assert!(cases > 0);
}
#[test]
fn interleaving_and_expired_jobs_do_not_change_a_field() {
    let fixtures: Value = serde_json::from_str(include_str!("golden.json")).unwrap();
    let req: Request = serde_json::from_value(fixtures["cases"][0]["request"].clone()).unwrap();
    let mut a = Job::new(&req, &[4, 8], 30000).unwrap();
    let mut b = Job::new(&req, &[4, 8, 3], 30000).unwrap();
    let expected =
        a.search
            .action_for_test(1, a.public, a.worlds[0][a.search.rules.turn(a.public)]);
    let other = b
        .search
        .action_for_test(1, b.public, b.worlds[0][b.search.rules.turn(b.public)]);
    assert_eq!(expected, other);
    let done = loop {
        let av = a.step().unwrap();
        let _ = b.step().unwrap();
        if let Some(v) = av {
            break v;
        }
    };
    assert_eq!(done["choice"], fixtures["cases"][1]["choice"]);
    let mut expired = Job::new(&req, &[24, 160], 1).unwrap();
    std::thread::sleep(Duration::from_millis(2));
    assert!(matches!(expired.step(), Err(Error::Timeout)));
    assert!(expired.finished.is_none());
    assert!(expired.step().is_err());
}
