//! Full-hand smoke through the string API: deal, auction, declare, and 28
//! plays with every seat a walt — the exact code path the browser runs.
//! walt-core referees independently (legality, trick winners, points), and
//! a repeated mid-hand request must be byte-identical (determinism).
//!
//! Small samples (n=6, n0=2): this tests lawfulness, conformance, and
//! determinism, not strength. Exploratory tier; not a P-A21 statement.

use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, Domino, Seat, Team};
use walt::solver::{bit, decl_of, mask_bits, mix, set_of, SplitMix64};
use walt_wasm::api::handle;

const N: usize = 6;
const N0: usize = 2;
const SEED: u64 = 0x9E37_79B9;

fn deal(hand_no: u64) -> [u32; 4] {
    let mut rng = SplitMix64(SEED ^ mix(hand_no));
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    [
        mask_slice(&tiles[0..7]),
        mask_slice(&tiles[7..14]),
        mask_slice(&tiles[14..21]),
        mask_slice(&tiles[21..28]),
    ]
}

fn hand_line(hand: u32) -> String {
    let ids: Vec<String> = mask_bits(hand).iter().map(u8::to_string).collect();
    format!("hand {}", ids.join(" "))
}

/// Minimal extractor for a top-level integer field of our flat responses.
fn field_i64(json: &str, key: &str) -> i64 {
    let pat = format!("\"{key}\":");
    let at = json.find(&pat).unwrap_or_else(|| panic!("{key} in {json}"));
    let rest = &json[at + pat.len()..];
    let end = rest
        .find(|c: char| !c.is_ascii_digit() && c != '-')
        .unwrap_or(rest.len());
    rest[..end]
        .parse()
        .unwrap_or_else(|_| panic!("{key} in {json}"))
}

/// First element of a top-level two-int array field like "points":[13,8].
fn field_arr0(json: &str, key: &str) -> i64 {
    let pat = format!("\"{key}\":[");
    let at = json.find(&pat).unwrap_or_else(|| panic!("{key} in {json}"));
    let rest = &json[at + pat.len()..];
    let end = rest.find(',').unwrap_or_else(|| panic!("{key} in {json}"));
    rest[..end]
        .parse()
        .unwrap_or_else(|_| panic!("{key} in {json}"))
}

fn field_str<'a>(json: &'a str, key: &str) -> &'a str {
    let pat = format!("\"{key}\":\"");
    let at = json.find(&pat).unwrap_or_else(|| panic!("{key} in {json}"));
    let rest = &json[at + pat.len()..];
    &rest[..rest.find('"').expect("closing quote")]
}

#[test]
fn full_hand_all_walt() {
    // Find the first deal whose auction produces a winner (deterministic).
    let mut contract: Option<(u64, [u32; 4], usize, u8)> = None;
    for hand_no in 1..=20 {
        let hands = deal(hand_no);
        let mut high: Option<(usize, u8)> = None;
        for (s, &hand_s) in hands.iter().enumerate() {
            let need = high.map(|(_, b)| b + 1).unwrap_or(30);
            if need > 42 {
                break;
            }
            let req = format!(
                "walt1 bid\n{}\nneed {need}\nn {N}\nn0 {N0}\nseed {}\n",
                hand_line(hand_s),
                SEED ^ mix(hand_no)
            );
            let resp = handle(&req);
            assert!(!resp.contains("error"), "bid errored: {resp}");
            if field_str(&resp, "action") == "bid" {
                high = Some((s, field_i64(&resp, "bid") as u8));
            }
        }
        if let Some((w, b)) = high {
            contract = Some((hand_no, hands, w, b));
            break;
        }
    }
    let (hand_no, hands, bidder, bid) = contract.expect("some auction closes within 20 deals");

    // The winner names trump through the API.
    let dreq = format!(
        "walt1 declare\n{}\nbid {bid}\nn {N}\nn0 {N0}\nseed {}\n",
        hand_line(hands[bidder]),
        SEED ^ mix(hand_no)
    );
    let dresp = handle(&dreq);
    assert!(!dresp.contains("error"), "declare errored: {dresp}");
    let decl_id = field_i64(&dresp, "decl") as usize;
    let dcl = decl_of(decl_id);

    // Play the whole hand; walt-core referees in the arena frame.
    let mut record: Vec<(usize, usize)> = Vec::new();
    let mut leader = bidder;
    let mut trick_tiles: Vec<u8> = Vec::new();
    let mut points = [0u8; 2]; // arena teams: 0 = seats {0,2}
    let mut played_mask = 0u32;
    let mut determinism_checked = false;
    for _trick in 0..7 {
        for pos in 0..4 {
            let actor = (leader + pos) % 4;
            let plays_flat: Vec<String> = record
                .iter()
                .flat_map(|&(a, t)| [a.to_string(), t.to_string()])
                .collect();
            let req = format!(
                "walt1 play\ndecl {decl_id}\nbid {bid}\nseat {actor}\nbidder {bidder}\n{}\nplays {}\nn {N}\nn0 {N0}\nseed {}\n",
                hand_line(hands[actor]),
                plays_flat.join(" "),
                SEED ^ mix(hand_no)
            );
            let resp = handle(&req);
            assert!(!resp.contains("error"), "play errored: {resp}");
            // Determinism: the first non-forced decision must reproduce.
            if !determinism_checked && resp.contains("\"forced\":false") {
                assert_eq!(resp, handle(&req), "same request, same bytes");
                determinism_checked = true;
            }
            // Conformance: walt's derived leader and points equal ours.
            assert_eq!(field_i64(&resp, "leader") as usize, leader);
            assert_eq!(field_arr0(&resp, "points") as u8, points[0]);
            let tile = field_i64(&resp, "choice") as u8;
            // Referee: the choice is legal at the actor's true state.
            let hand_now = hands[actor] & !played_mask;
            let led: Option<Context> = trick_tiles
                .first()
                .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led")));
            let legal = legal_plays(dcl, set_of(hand_now), led);
            let dm = Domino::from_index(usize::from(tile)).expect("tile");
            assert!(legal.contains(dm), "illegal choice {tile}");
            played_mask |= bit(dm);
            trick_tiles.push(tile);
            record.push((actor, usize::from(tile)));
        }
        let doms = [
            Domino::from_index(usize::from(trick_tiles[0])).expect("p0"),
            Domino::from_index(usize::from(trick_tiles[1])).expect("p1"),
            Domino::from_index(usize::from(trick_tiles[2])).expect("p2"),
            Domino::from_index(usize::from(trick_tiles[3])).expect("p3"),
        ];
        let trick = Trick::new(Seat::from_index(leader).expect("leader"), doms).expect("distinct");
        let winner = trick.winner(dcl);
        let pts = trick.points() as u8;
        // Arena teams by literal seat parity: points[0] = seats {0,2}.
        points[usize::from(winner.team() == Team::T1)] += pts;
        leader = winner.index();
        trick_tiles.clear();
    }
    assert_eq!(played_mask, walt::solver::FULL_MASK, "all 28 tiles played");
    assert!(determinism_checked, "at least one open decision occurred");
    assert_eq!(
        u32::from(points[0]) + u32::from(points[1]),
        42,
        "points partition 42"
    );
    let flat: Vec<String> = record
        .iter()
        .flat_map(|&(a, t)| [a.to_string(), t.to_string()])
        .collect();
    println!(
        "hand {hand_no}: S{bidder} bid {bid}, decl {decl_id}; points {}-{} (team1 = odd seats)",
        points[0], points[1]
    );
    println!("record: {}", flat.join(" "));
}

/// Cross-fiber review (`viewer` + `viewer_hand` on a play request): the
/// response gains a second pricing column — the actor's legal options
/// priced from the viewer's fiber. Additive and backward compatible: the
/// legacy response bytes are a strict prefix of the extended response
/// (minus the closing brace), and the whole call is deterministic.
#[test]
fn play_viewer_fiber_review() {
    let hands = deal(1);
    let (hand_no, bidder, bid) = (1u64, 0usize, 42u8);
    let dreq = format!(
        "walt1 declare\n{}\nbid {bid}\nn {N}\nn0 {N0}\nseed {}\n",
        hand_line(hands[bidder]),
        SEED ^ mix(hand_no)
    );
    let dresp = handle(&dreq);
    assert!(!dresp.contains("error"), "declare errored: {dresp}");
    let decl_id = field_i64(&dresp, "decl") as usize;

    // Opening lead: S0 to act, S1 reviewing from its own fiber.
    let base = format!(
        "walt1 play\ndecl {decl_id}\nbid {bid}\nseat 0\nbidder 0\n{}\nplays \nn {N}\nn0 {N0}\nseed {}\n",
        hand_line(hands[0]),
        SEED ^ mix(hand_no)
    );
    let legacy = handle(&base);
    assert!(!legacy.contains("error"), "legacy play errored: {legacy}");
    assert!(!legacy.contains("viewer"), "legacy response has no viewer");

    let viewer_line = {
        let ids: Vec<String> = mask_bits(hands[1]).iter().map(u8::to_string).collect();
        format!("viewer 1\nviewer_hand {}\n", ids.join(" "))
    };
    let vreq = format!("{base}{viewer_line}");
    let vresp = handle(&vreq);
    assert!(!vresp.contains("error"), "viewer play errored: {vresp}");
    assert_eq!(vresp, handle(&vreq), "same request, same bytes");
    assert!(
        vresp.starts_with(&legacy[..legacy.len() - 1]),
        "legacy fields are byte-identical: {legacy} vs {vresp}"
    );
    assert!(vresp.contains("\"viewer\":1"), "viewer echoed: {vresp}");

    // Every legal option of the actor (all 7 at an opening lead) is
    // priced: entries are [tile, bp|null, support] with support <= n.
    let at = vresp
        .find("\"viewer_opts\":[")
        .expect("viewer_opts present");
    let arr = &vresp[at + "\"viewer_opts\":[".len()..vresp.len() - 2];
    let entries: Vec<&str> = arr
        .split("],[")
        .map(|e| e.trim_start_matches('[').trim_end_matches(']'))
        .collect();
    assert_eq!(entries.len(), 7, "seven opening options priced: {arr}");
    for (i, &t) in mask_bits(hands[0]).iter().enumerate() {
        let parts: Vec<&str> = entries[i].split(',').collect();
        assert_eq!(parts.len(), 3, "tile,bp,support: {}", entries[i]);
        assert_eq!(parts[0], t.to_string(), "options in ascending tile order");
        let support: usize = parts[2].parse().expect("support count");
        assert!(support <= N, "support within the sample");
        if parts[1] == "null" {
            assert_eq!(support, 0, "null price exactly at empty support");
        } else {
            let bp_v: i64 = parts[1].parse().expect("basis points");
            assert!(support > 0 && (0..=10_000).contains(&bp_v));
        }
    }

    // A viewer hand that overlaps the actor's is rejected, not guessed at.
    let bad_ids: Vec<String> = mask_bits(hands[0]).iter().map(u8::to_string).collect();
    let bad = format!("{base}viewer 1\nviewer_hand {}\n", bad_ids.join(" "));
    assert!(
        handle(&bad).contains("error"),
        "overlapping viewer_hand must error"
    );
}

/// The race-then-refine play mode (`race 1`): a full hand with all four
/// seats racing. Same walt-core refereeing and conformance asserts as the
/// default-mode test; the record may lawfully differ from the full-mode
/// trace (different play policy), but every choice must be legal, every
/// response deterministic, and every open decision marked `"raced":true`.
#[test]
fn full_hand_all_walt_raced() {
    let hands = deal(1);
    // Reuse hand 1's known contract shape (auctioned in the default test):
    // S0 wins at 42; declare through the API for the declaration.
    let (hand_no, bidder, bid) = (1u64, 0usize, 42u8);
    let dreq = format!(
        "walt1 declare\n{}\nbid {bid}\nn {N}\nn0 {N0}\nseed {}\n",
        hand_line(hands[bidder]),
        SEED ^ mix(hand_no)
    );
    let dresp = handle(&dreq);
    assert!(!dresp.contains("error"), "declare errored: {dresp}");
    let decl_id = field_i64(&dresp, "decl") as usize;
    let dcl = decl_of(decl_id);

    let mut record: Vec<(usize, usize)> = Vec::new();
    let mut leader = bidder;
    let mut trick_tiles: Vec<u8> = Vec::new();
    let mut points = [0u8; 2];
    let mut played_mask = 0u32;
    let mut determinism_checked = false;
    for _trick in 0..7 {
        for pos in 0..4 {
            let actor = (leader + pos) % 4;
            let plays_flat: Vec<String> = record
                .iter()
                .flat_map(|&(a, t)| [a.to_string(), t.to_string()])
                .collect();
            let req = format!(
                "walt1 play\ndecl {decl_id}\nbid {bid}\nseat {actor}\nbidder {bidder}\n{}\nplays {}\nrace 1\nn {N}\nn0 {N0}\nseed {}\n",
                hand_line(hands[actor]),
                plays_flat.join(" "),
                SEED ^ mix(hand_no)
            );
            let resp = handle(&req);
            assert!(!resp.contains("error"), "play errored: {resp}");
            if resp.contains("\"forced\":false") {
                assert!(
                    resp.contains("\"raced\":true"),
                    "open decision raced: {resp}"
                );
                if !determinism_checked {
                    assert_eq!(resp, handle(&req), "same request, same bytes");
                    determinism_checked = true;
                }
            }
            assert_eq!(field_i64(&resp, "leader") as usize, leader);
            assert_eq!(field_arr0(&resp, "points") as u8, points[0]);
            let tile = field_i64(&resp, "choice") as u8;
            let hand_now = hands[actor] & !played_mask;
            let led: Option<Context> = trick_tiles
                .first()
                .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led")));
            let legal = legal_plays(dcl, set_of(hand_now), led);
            let dm = Domino::from_index(usize::from(tile)).expect("tile");
            assert!(legal.contains(dm), "illegal raced choice {tile}");
            played_mask |= bit(dm);
            trick_tiles.push(tile);
            record.push((actor, usize::from(tile)));
        }
        let doms = [
            Domino::from_index(usize::from(trick_tiles[0])).expect("p0"),
            Domino::from_index(usize::from(trick_tiles[1])).expect("p1"),
            Domino::from_index(usize::from(trick_tiles[2])).expect("p2"),
            Domino::from_index(usize::from(trick_tiles[3])).expect("p3"),
        ];
        let trick = Trick::new(Seat::from_index(leader).expect("leader"), doms).expect("distinct");
        let winner = trick.winner(dcl);
        let pts = trick.points() as u8;
        points[usize::from(winner.team() == Team::T1)] += pts;
        leader = winner.index();
        trick_tiles.clear();
    }
    assert_eq!(played_mask, walt::solver::FULL_MASK, "all 28 tiles played");
    assert!(determinism_checked, "at least one open decision occurred");
    assert_eq!(u32::from(points[0]) + u32::from(points[1]), 42);
    let flat: Vec<String> = record
        .iter()
        .flat_map(|&(a, t)| [a.to_string(), t.to_string()])
        .collect();
    println!(
        "raced hand {hand_no}: decl {decl_id}; points {}-{}; record: {}",
        points[0],
        points[1],
        flat.join(" ")
    );
}
