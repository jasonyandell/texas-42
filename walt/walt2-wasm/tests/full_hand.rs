//! Full-hand smoke through the string API: deal, auction, declare, and 28
//! plays with every seat a level-2 walt — the exact code path the browser
//! runs. walt-core referees independently (legality, trick winners,
//! points), and a repeated mid-hand request must be byte-identical
//! (determinism). The auction handlers are additionally pinned
//! byte-identical to walt-wasm's (the declared duplication cannot drift).
//!
//! Small samples (play n=4/n1=2/n0=2, auction n=6/n0=2): this tests
//! lawfulness, conformance, and determinism, not strength. Exploratory
//! tier; not a P-A21 statement.

use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, Domino, Seat, Team};
use walt::solver::{bit, decl_of, mask_bits, mix, set_of, SplitMix64};
use walt2_wasm::api::handle;

const N_BID: usize = 6;
const N_PLAY: usize = 4;
const N1: usize = 2;
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
fn full_hand_all_walt2() {
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
                "walt2 bid\n{}\nneed {need}\nn {N_BID}\nn0 {N0}\nseed {}\n",
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
        "walt2 declare\n{}\nbid {bid}\nn {N_BID}\nn0 {N0}\nseed {}\n",
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
                "walt2 play\ndecl {decl_id}\nbid {bid}\nseat {actor}\nbidder {bidder}\n{}\nplays {}\nn {N_PLAY}\nn1 {N1}\nn0 {N0}\nseed {}\n",
                hand_line(hands[actor]),
                plays_flat.join(" "),
                SEED ^ mix(hand_no)
            );
            let resp = handle(&req);
            assert!(!resp.contains("error"), "play errored: {resp}");
            assert!(
                resp.contains("\"level\":2"),
                "level-2 marker present: {resp}"
            );
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

/// The declared duplication pin: walt2's `bid` and `declare` answer
/// byte-identically to walt-wasm's for the same fields. If either copy
/// drifts, this fails — the duplication is a frozen equality, not a fork.
#[test]
fn auction_matches_walt1() {
    for hand_no in 1..=5u64 {
        let hands = deal(hand_no);
        for (s, &hand_s) in hands.iter().enumerate() {
            let fields = format!(
                "{}\nneed 30\nn {N_BID}\nn0 {N0}\nseed {}\n",
                hand_line(hand_s),
                SEED ^ mix(hand_no) ^ (s as u64)
            );
            let ours = handle(&format!("walt2 bid\n{fields}"));
            let theirs = walt_wasm::api::handle(&format!("walt1 bid\n{fields}"));
            assert_eq!(ours, theirs, "bid drifted from walt-wasm");
        }
        let fields = format!(
            "{}\nbid 30\nn {N_BID}\nn0 {N0}\nseed {}\n",
            hand_line(hands[0]),
            SEED ^ mix(hand_no)
        );
        let ours = handle(&format!("walt2 declare\n{fields}"));
        let theirs = walt_wasm::api::handle(&format!("walt1 declare\n{fields}"));
        assert_eq!(ours, theirs, "declare drifted from walt-wasm");
    }
}

/// Protocol hygiene: the magic is the contract — a walt1 request is not
/// silently accepted, and errors come back as JSON, never a trap.
#[test]
fn protocol_rejections() {
    let resp = handle("walt1 play\n");
    assert!(
        resp.contains("error") && resp.contains("walt2"),
        "wrong magic names the expected one: {resp}"
    );
    let resp = handle("walt2 race\n");
    assert!(resp.contains("error"), "unknown kind errors: {resp}");
    let resp = handle("");
    assert!(resp.contains("error"), "empty request errors: {resp}");
}
