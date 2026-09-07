use walt::policy_search::{fixture, replay, ActionPool, HashField, Search, TablePolicy, Work};
use walt::rules::{Decl, Domino, DominoSet};
use walt::solver::adaptive::{PublicRecord, SlicePolicy};

struct AdversarialField;
impl SlicePolicy for AdversarialField {
    fn id(&self) -> &str {
        "test-hand-parity-extremes-v1"
    }
    fn choose(
        &self,
        _decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        let parity = (hand.bits() ^ (record.history.len() as u32)).count_ones() & 1;
        if parity == 0 {
            legal.iter().next().unwrap()
        } else {
            legal.iter().last().unwrap()
        }
    }
}

fn small(tiles: usize) -> walt::policy_search::Fixture {
    let decl = "P6".parse::<Decl>().unwrap();
    (1..100)
        .find_map(|seed| fixture(seed, decl, tiles, None).ok())
        .expect("a deterministic legal small fixture")
}

fn sampled(fixture: &walt::policy_search::Fixture, n: usize) -> Vec<walt::kernel::World> {
    fixture.exercise.root.worlds().take(n).collect()
}

#[test]
fn persistent_accumulation_matches_solving_the_accumulated_sample_afresh() {
    let fixture = small(3);
    let worlds = sampled(&fixture, 5);
    assert_eq!(worlds.len(), 5);
    let mut persistent = Search::new(&fixture.exercise, &HashField);
    for (i, world) in worlds.iter().copied().enumerate() {
        persistent.append(world).unwrap();
        let p = persistent.solve(&mut Work::new(2_000_000)).unwrap();

        let mut fresh = Search::new(&fixture.exercise, &HashField);
        for prior in worlds[..=i].iter().copied() {
            fresh.append(prior).unwrap();
        }
        let f = fresh.solve(&mut Work::new(2_000_000)).unwrap();
        assert_eq!(p.makes, f.makes);
        assert_eq!(TablePolicy::from_node(&p), TablePolicy::from_node(&f));
    }
}

#[test]
fn duplicate_samples_retain_multiplicity_as_sample_weight() {
    let fixture = small(3);
    let world = sampled(&fixture, 1)[0];
    let mut once = Search::new(&fixture.exercise, &HashField);
    once.append(world).unwrap();
    let one = once.solve(&mut Work::new(2_000_000)).unwrap();
    let mut twice = Search::new(&fixture.exercise, &HashField);
    twice.append(world).unwrap();
    twice.append(world).unwrap();
    let two = twice.solve(&mut Work::new(2_000_000)).unwrap();
    assert_eq!(two.makes, 2 * one.makes);
}

#[test]
fn donor_union_composition_matches_full_search_and_replays_independently() {
    let fixture = small(2);
    let worlds = sampled(&fixture, 8);
    let mut search = Search::new(&fixture.exercise, &HashField);
    for world in worlds.iter().copied() {
        search.append(world).unwrap();
    }
    let full = search.solve(&mut Work::new(2_000_000)).unwrap();
    let mut pool = ActionPool::new();
    for i in 0..worlds.len() {
        search
            .donor(i, &mut pool, &mut Work::new(2_000_000))
            .unwrap();
    }
    let composed = search.compose(&pool, &mut Work::new(2_000_000)).unwrap();
    assert_eq!(composed.makes, full.makes);

    let policy = TablePolicy::from_node(&composed);
    let replay_makes = worlds
        .iter()
        .filter(|world| {
            replay(&fixture.exercise, &HashField, world, &policy)
                .unwrap()
                .0
        })
        .count();
    assert_eq!(replay_makes, composed.makes);
}

#[test]
fn donor_collects_every_successful_root_alternative() {
    let fixture = small(2);
    let world = sampled(&fixture, 1)[0];
    let mut search = Search::new(&fixture.exercise, &HashField);
    search.append(world).unwrap();
    let mut pool = ActionPool::new();
    search
        .donor(0, &mut pool, &mut Work::new(2_000_000))
        .unwrap();
    let offered = pool.get(&Vec::new()).copied().unwrap_or(DominoSet::EMPTY);

    let legal = walt::rules::legal_plays(
        fixture.exercise.position.decl,
        fixture.exercise.root.kernel().viewer_hand(),
        fixture.exercise.frame.led_context(),
    );
    let mut independently_successful = DominoSet::EMPTY;
    for action in legal {
        let mut singleton = ActionPool::new();
        singleton.insert(Vec::new(), DominoSet::single(action));
        if search
            .compose(&singleton, &mut Work::new(2_000_000))
            .unwrap()
            .makes
            == 1
        {
            independently_successful.insert(action);
        }
    }
    assert_eq!(offered, independently_successful);
}

#[test]
fn restricted_composition_cache_cannot_poison_later_full_solve() {
    let fixture = small(3);
    let worlds = sampled(&fixture, 6);
    let mut search = Search::new(&fixture.exercise, &HashField);
    let mut fresh = Search::new(&fixture.exercise, &HashField);
    for world in worlds {
        search.append(world).unwrap();
        fresh.append(world).unwrap();
    }
    let legal = walt::rules::legal_plays(
        fixture.exercise.position.decl,
        fixture.exercise.root.kernel().viewer_hand(),
        fixture.exercise.frame.led_context(),
    );
    let mut restricted = ActionPool::new();
    restricted.insert(Vec::new(), DominoSet::single(legal.iter().next().unwrap()));
    search
        .compose(&restricted, &mut Work::new(2_000_000))
        .unwrap();
    assert_eq!(search.cache_len(), 0);
    let after = search.solve(&mut Work::new(2_000_000)).unwrap();
    let expected = fresh.solve(&mut Work::new(2_000_000)).unwrap();
    assert_eq!(after.makes, expected.makes);
    assert_eq!(
        TablePolicy::from_node(&after),
        TablePolicy::from_node(&expected)
    );
}

#[test]
fn donor_budget_refusal_does_not_publish_a_partial_pool() {
    let fixture = small(3);
    let world = sampled(&fixture, 1)[0];
    let mut search = Search::new(&fixture.exercise, &HashField);
    search.append(world).unwrap();
    let mut pool = ActionPool::new();
    pool.insert(
        vec![255],
        DominoSet::single(fixture.original.iter().next().unwrap()),
    );
    let before = pool.clone();
    assert!(search.donor(0, &mut pool, &mut Work::new(1)).is_err());
    assert_eq!(pool, before);
}

#[test]
fn search_refuses_a_defending_viewer_instead_of_maximizing_the_wrong_payoff() {
    let mut fixture = small(3);
    fixture.exercise.position.declaring_team = fixture.exercise.position.declaring_team.other();
    let world = fixture.exercise.root.worlds().next().unwrap();
    let mut search = Search::new(&fixture.exercise, &HashField);
    search.append(world).unwrap();
    let error = search.solve(&mut Work::new(2_000_000)).unwrap_err();
    assert!(error.contains("viewer must be on the declaring team"));
}

#[test]
fn extracted_policy_replays_on_heldout_worlds() {
    let fixture = small(3);
    let worlds = sampled(&fixture, 10);
    let mut search = Search::new(&fixture.exercise, &HashField);
    for world in worlds[..5].iter().copied() {
        search.append(world).unwrap();
    }
    let node = search.solve(&mut Work::new(2_000_000)).unwrap();
    let policy = TablePolicy::from_node(&node);
    for world in &worlds[5..] {
        let (_, trace) = replay(&fixture.exercise, &HashField, world, &policy).unwrap();
        let distinct: DominoSet = trace.iter().map(|(_, tile)| *tile).collect();
        assert_eq!(distinct.len(), trace.len());
        assert_eq!(trace.len(), fixture.exercise.root.kernel().live().len());
    }
}

#[test]
fn donor_alternative_union_finds_a_common_policy_against_branching_field() {
    let fixture = small(3);
    let worlds = sampled(&fixture, 20);
    for i in 0..worlds.len() {
        for j in i + 1..worlds.len() {
            let mut search = Search::new(&fixture.exercise, &AdversarialField);
            search.append(worlds[i]).unwrap();
            search.append(worlds[j]).unwrap();
            let mut pool = ActionPool::new();
            search
                .donor(0, &mut pool, &mut Work::new(2_000_000))
                .unwrap();
            search
                .donor(1, &mut pool, &mut Work::new(2_000_000))
                .unwrap();
            let Some(root_actions) = pool.get(&Vec::new()) else {
                continue;
            };
            if root_actions.len() < 2 {
                continue;
            }
            let composed = search.compose(&pool, &mut Work::new(2_000_000)).unwrap();
            if composed.makes == 2 {
                let policy = TablePolicy::from_node(&composed);
                assert!(
                    replay(&fixture.exercise, &AdversarialField, &worlds[i], &policy)
                        .unwrap()
                        .0
                );
                assert!(
                    replay(&fixture.exercise, &AdversarialField, &worlds[j], &policy)
                        .unwrap()
                        .0
                );
                return;
            }
        }
    }
    panic!("fixture should contain a branching-field pair with a common composed policy");
}
