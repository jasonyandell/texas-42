//! Gates for the σ1-repair slice: terminating the void-conditioned belief
//! sampler, and deduplicating its five copies onto one library authority.
//!
//! Brief: `walt/briefs/BRIEF-SIGMA1-REPAIR.md`. Hazard diagnosis:
//! `walt/briefs/MB0-COLLISION-NOTES.md` and `MB0-HANDOFF-BUILDER2.md`; the
//! "sigma1 boundary" section of
//! `walt/probes/factor_belief/modelbelief_run1.txt`.
//!
//! EXPLORATORY tier throughout. Nothing here is promoted; the numbers are
//! determinism pins, not strength claims.
//!
//! The gates:
//! - R1 the infeasible specimen terminates with the typed refusal.
//! - R2 before/after determinism: the committed before-side fixture
//!   (`tests/data/sigma1_before_v1.txt`, captured while the sampler was
//!   still the unbounded shuffle-and-reject loop) reproduces EXACTLY
//!   through the repaired library path.
//! - R3 dedup identity, by TWO witnesses: a source grep over `src/`, and
//!   the compile itself (the four deduplicated binaries import the library
//!   name at module scope, so a local `fn sample_belief` is an E0255
//!   collision — the gate keeps those imports armed).
//! - R4 the previously-blocked MB0 roots under the repaired sampler.
//! - R5 the feasibility oracle is faithful: it agrees with exhaustive
//!   exact-partition search on every frame of a swept corpus, and the
//!   library sampler's accepted deals are exactly the frames it admits.
//! - R6 `sample_open_belief` is total — a frame with no deduced voids
//!   cannot be refused, which is what lets the live player's auction path
//!   carry no error branch instead of an unreachable one.
//!
//! THE BEFORE-SIDE WITNESS. `capture_before_side_fixture` is `#[ignore]`d
//! and regenerates the fixture. It was run ONCE against the unpatched
//! sampler and its output committed; R2 then re-runs the identical corpus
//! through the repaired path and byte-compares. Regenerating it after the
//! repair would destroy the evidence, so the ignore marker stays and the
//! fixture is never rewritten in anger.

mod common;

use std::collections::BTreeMap;
use std::fmt::Write as _;

use common::receipt;
use walt::kernel::Kernel;
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::Pip;
use walt::rules::{Decl, Domino, DominoSet, Seat};
use walt::solver::adaptive::{replay_viewer_success, CanonicalRoot, RootPosition, SlicePolicy};
use walt::solver::factor_belief::{
    response_success_mass, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{continuation_frame, DecisionMode, TieRule};
use walt::solver::{
    belief_frame_feasibility, mask_of, mix, sample_belief, sample_open_belief, SplitMix64,
    FULL_MASK,
};

/// A pip by value, for writing specimen tiles the way the audit notes do.
fn pip(v: u8) -> Pip {
    Pip::new(v).expect("a pip 0..=6")
}

/// The committed before-side capture.
const FIXTURE: &str = include_str!("data/sigma1_before_v1.txt");

// ---------------------------------------------------------------------------
// Corpus A — sampler transcripts over FEASIBLE void-bearing frames.
// ---------------------------------------------------------------------------

/// One declared call of the belief sampler. Every field is an explicit
/// argument of `solver::sample_belief`, so the transcript is reproducible
/// from the fixture text alone.
struct SamplerFrame {
    gen_seed: u64,
    depth: usize,
    decl: Decl,
    viewer: usize,
    viewer_hand: u32,
    played: u32,
    sizes: [usize; 4],
    voids: [u32; 4],
    n: usize,
    seed: u64,
}

/// Deal 28 tiles by Fisher-Yates over a declared stream, play `depth`
/// plies under a deterministic churning rule, and read off the belief
/// frame the seat to move would hand the sampler. The dealt world is a
/// witness that the frame is FEASIBLE — which is exactly why this corpus
/// is a lawful before-side capture against an unbounded sampler.
fn frame_at(gen_seed: u64, depth: usize) -> SamplerFrame {
    let decl = Decl::ALL[(gen_seed as usize) % Decl::COUNT];
    let mut rng = SplitMix64(mix(0xC0FF_EE00_5164_0001 ^ gen_seed));
    let mut tiles: Vec<usize> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mut hands = [DominoSet::EMPTY; 4];
    for (slot, &tile) in tiles.iter().enumerate() {
        hands[slot / 7].insert(Domino::from_index(tile).expect("index < 28"));
    }
    let mut leader = 0usize;
    let mut plays: Vec<Domino> = Vec::new();
    let mut played = 0u32;
    let mut voids = [0u32; 4];
    for step in 0..depth {
        let seat = (leader + plays.len()) % 4;
        let led = plays.first().map(|d| decl.led_context(*d));
        let legal = legal_plays(decl, hands[seat], led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        // Alternate lowest/highest legal so suits are exhausted at
        // different rates and deduced voids actually appear.
        let choice = if (seat + step).is_multiple_of(2) {
            legal.iter().next()
        } else {
            legal.iter().last()
        }
        .expect("a nonempty legal set");
        if let Some(&first) = plays.first() {
            let ctx = decl.led_context(first);
            if !decl.follows(choice, ctx) {
                voids[seat] |= mask_of(decl.effective_incidence(ctx));
            }
        }
        hands[seat].remove(choice);
        played |= 1u32 << choice.index();
        plays.push(choice);
        if plays.len() == 4 {
            let trick = Trick::new(
                Seat::from_index(leader).expect("seat < 4"),
                [plays[0], plays[1], plays[2], plays[3]],
            )
            .expect("distinct tiles");
            leader = trick.winner(decl).index();
            plays.clear();
        }
    }
    let viewer = (leader + plays.len()) % 4;
    let sizes = [
        hands[0].len(),
        hands[1].len(),
        hands[2].len(),
        hands[3].len(),
    ];
    SamplerFrame {
        gen_seed,
        depth,
        decl,
        viewer,
        viewer_hand: mask_of(hands[viewer]),
        played,
        sizes,
        voids,
        n: 4,
        seed: mix(0x5164_A000_0000_0000 ^ gen_seed ^ (depth as u64)),
    }
}

/// The one call of the library sampler this file makes. Isolated so the
/// before-side capture and the after-side gate differ in exactly this
/// function's body — the typed refusal is `expect`ed here because every
/// corpus-A frame is certified feasible by the deal that generated it.
fn draw(f: &SamplerFrame, rng: &mut SplitMix64) -> Vec<[u32; 4]> {
    sample_belief(
        f.viewer,
        f.viewer_hand,
        f.played,
        f.sizes,
        f.voids,
        f.n,
        rng,
    )
    .unwrap_or_else(|frame| {
        panic!("every corpus-A frame is dealt from a real world, so it is feasible: {frame}")
    })
}

/// The declared corpus: twelve deals crossed with four play depths.
fn corpus_a() -> Vec<SamplerFrame> {
    let mut frames = Vec::new();
    for gen_seed in 0u64..12 {
        for depth in [6usize, 10, 14, 18] {
            frames.push(frame_at(gen_seed, depth));
        }
    }
    frames
}

// ---------------------------------------------------------------------------
// Corpus B — σ1 field actions on the two receipt roots the raw Level1
// authority terminates on (the MB0 boundary section's terminating domain).
// ---------------------------------------------------------------------------

/// F₁ at the MB0 declared test epoch.
fn level1_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

/// UNDECIDED receipt roots — the ones where `decided_success` does not
/// settle the indicator at the root, so the replay actually consults the
/// σ1 field and drives `level1_evaluate` → `sample_belief`. (The two roots
/// the raw σ1 authority prices, h12-t6 and h10-t6, are exactly the
/// root-decided fixtures: a replay there never reaches the field, so they
/// witness nothing about the sampler.)
///
/// Every frame these replays construct is feasible by construction: the
/// world being replayed is itself a lawful completion of the deduced void
/// structure at every state along the line. That is the structural reason
/// the LIVE player never met this hazard, and it is why this corpus is a
/// lawful before-side capture against an unbounded sampler.
const B_ROOTS: [(usize, usize); 4] = [(5, 6), (4, 6), (8, 5), (3, 5)];

fn root_at(hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let r = receipt();
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

// ---------------------------------------------------------------------------
// The capture itself: one deterministic text, generated identically before
// and after the repair.
// ---------------------------------------------------------------------------

fn render_capture() -> String {
    let mut out = String::new();
    out.push_str("== walt sigma1 before-side determinism capture v1\n");
    out.push_str(
        "== EXPLORATORY tier. Captured against the UNPATCHED shuffle-and-reject\n\
         == sampler (solver/mod.rs:897 as of commit 1824d6a); re-checked byte for\n\
         == byte through the repaired library path by gate R2.\n",
    );

    let frames = corpus_a();
    let with_voids = frames
        .iter()
        .filter(|f| f.voids.iter().any(|v| *v != 0))
        .count();
    out.push_str("\n== A: sampler transcripts over feasible void-bearing frames\n");
    let _ = writeln!(
        out,
        "A.census frames={} with-deduced-voids={}",
        frames.len(),
        with_voids
    );
    for f in &frames {
        let _ = writeln!(
            out,
            "A gen={} depth={} decl={} viewer={} hand={:08x} played={:08x} \
             sizes=[{},{},{},{}] voids=[{:08x},{:08x},{:08x},{:08x}] n={} seed={:016x}",
            f.gen_seed,
            f.depth,
            f.decl,
            f.viewer,
            f.viewer_hand,
            f.played,
            f.sizes[0],
            f.sizes[1],
            f.sizes[2],
            f.sizes[3],
            f.voids[0],
            f.voids[1],
            f.voids[2],
            f.voids[3],
            f.n,
            f.seed,
        );
        let mut rng = SplitMix64(f.seed);
        let worlds = draw(f, &mut rng);
        for w in &worlds {
            let _ = writeln!(
                out,
                "A.w {:08x} {:08x} {:08x} {:08x}",
                w[0], w[1], w[2], w[3]
            );
        }
        // The RNG state AFTER the declared draws: a one-word witness that
        // the accept/reject path consumed exactly the same stream.
        let _ = writeln!(out, "A.rng {:016x}", rng.0);
    }

    out.push_str("\n== B: sigma1 field actions over undecided receipt roots\n");
    for (hand_id, trick_no) in B_ROOTS {
        let (root, position) = root_at(hand_id, trick_no);
        let field = FieldModel::new(level1_spec());
        let viewer = root.kernel().viewer();
        let _ = writeln!(
            out,
            "B root=h{hand_id}-t{trick_no} fiber={} viewer={}",
            root.count(),
            viewer.index()
        );
        for (i, world) in root.worlds().enumerate() {
            let made = replay_viewer_success(&position, viewer, &world, &field, &field);
            let _ = writeln!(out, "B.w {i} pmake={}", u8::from(made));
        }
        // The materialized action cache: every σ1 information state the
        // replay reached, with the tile it chose. Sorted by the key's own
        // 64-bit fold so the rendering is order-independent.
        let snapshot: BTreeMap<u64, usize> = field
            .cache_snapshot()
            .iter()
            .map(|(key, tile)| (key.digest64(), tile.index()))
            .collect();
        let _ = writeln!(out, "B.cache-len {}", snapshot.len());
        for (digest, tile) in &snapshot {
            let _ = writeln!(out, "B.cache {digest:016x} {tile}");
        }
    }
    out
}

/// Regenerate the before-side fixture. `#[ignore]`d on purpose: it was run
/// once against the unpatched sampler and its output committed. Running it
/// again after the repair would overwrite the evidence R2 exists to check.
#[test]
#[ignore = "regenerates the committed before-side evidence; see the module doc"]
fn capture_before_side_fixture() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("sigma1_before_v1.txt");
    std::fs::write(&path, render_capture()).expect("the fixture path is writable");
}

// ---------------------------------------------------------------------------
// R1 — the pinned infeasible specimen.
// ---------------------------------------------------------------------------

/// Gate R1 — the live specimen from the MB0 audit
/// (`walt/briefs/MB0-COLLISION-NOTES.md`): seat S3, hand {4-2 4-4},
/// history [4-1, 4-3, 1-1], sizes [1, 1, 1, 2], voids
/// [16786368, 69173248, 33586176, 16786368]. Against the unpatched
/// sampler this frame spins forever; here it must come back with the
/// typed refusal, and the refusal must name a real blocking set.
///
/// The frame is rebuilt through the library's own authorities
/// (`continuation_frame` at the h5-t6 receipt root) rather than pasted as
/// literals, so every pinned coordinate is re-derived and re-checked.
#[test]
fn the_pinned_infeasible_specimen_refuses_instead_of_spinning() {
    let (_, position) = root_at(5, 6);
    let history: Vec<Domino> = [(4, 1), (4, 3), (1, 1)]
        .into_iter()
        .map(|(a, b)| Domino::new(pip(a), pip(b)))
        .collect();
    let frame = continuation_frame(position.decl, &position, &history);
    let hand: DominoSet = [(4, 2), (4, 4)]
        .into_iter()
        .map(|(a, b)| Domino::new(pip(a), pip(b)))
        .collect();

    assert_eq!(frame.seat, Seat::S3, "the specimen's acting seat");
    assert_eq!(frame.sizes(), [1, 1, 1, 2], "the specimen's declared sizes");
    assert_eq!(
        frame.voids,
        [16_786_368, 69_173_248, 33_586_176, 16_786_368],
        "the specimen's deduced void masks"
    );
    assert_eq!(hand.len(), frame.sizes()[Seat::S3.index()]);

    let refusal = belief_frame_feasibility(
        Seat::S3.index(),
        mask_of(hand),
        frame.key.played,
        frame.sizes(),
        frame.voids,
    )
    .expect_err("the pinned specimen has an empty lawful-completion fiber");
    assert!(
        refusal.confined.count_ones() as usize > refusal.room,
        "a refusal names a blocking set that really overflows"
    );

    // The sampler itself terminates with the same refusal — no spin.
    let mut rng = SplitMix64(0x5164_5EC1_0000_0001);
    let drawn = sample_belief(
        Seat::S3.index(),
        mask_of(hand),
        frame.key.played,
        frame.sizes(),
        frame.voids,
        4,
        &mut rng,
    );
    assert_eq!(
        drawn.expect_err("the sampler refuses the specimen"),
        refusal,
        "the sampler's refusal is the oracle's refusal"
    );
    assert_eq!(
        rng.0, 0x5164_5EC1_0000_0001,
        "a refused frame consumes no randomness"
    );

    // And the exhaustive search agrees: no exact partition exists.
    assert!(
        !exact_partition_exists(
            FULL_MASK & !frame.key.played & !mask_of(hand),
            &[
                (frame.sizes()[0], frame.voids[0]),
                (frame.sizes()[1], frame.voids[1]),
                (frame.sizes()[2], frame.voids[2]),
            ],
        ),
        "exhaustive search confirms the specimen is unsatisfiable"
    );
}

// ---------------------------------------------------------------------------
// R5 — the feasibility oracle is faithful.
// ---------------------------------------------------------------------------

/// Exhaustive exact-partition search: deal `unseen` out to the listed
/// (size, void-mask) seats, every tile used. A test-local reference
/// implementation of the sampler's acceptance predicate — never library
/// source, and never used outside this file.
fn exact_partition_exists(unseen: u32, seats: &[(usize, u32)]) -> bool {
    match seats.split_first() {
        None => unseen == 0,
        Some((&(size, void), rest)) => {
            let usable: Vec<u32> = (0..28)
                .map(|i| 1u32 << i)
                .filter(|b| unseen & b != 0 && void & b == 0)
                .collect();
            fn choose(
                usable: &[u32],
                size: usize,
                acc: u32,
                unseen: u32,
                rest: &[(usize, u32)],
            ) -> bool {
                if size == 0 {
                    return exact_partition_exists(unseen & !acc, rest);
                }
                if usable.len() < size {
                    return false;
                }
                for (i, b) in usable.iter().enumerate() {
                    if choose(&usable[i + 1..], size - 1, acc | b, unseen, rest) {
                        return true;
                    }
                }
                false
            }
            choose(&usable, size, 0, unseen, rest)
        }
    }
}

/// Gate R5 — the counting oracle decides exactly what the shuffle-and-
/// reject loop accepts. Over a swept corpus of small frames with adversarial
/// void structure, `belief_frame_feasibility` agrees with exhaustive
/// exact-partition search on EVERY frame, in both directions: no feasible
/// frame is refused (which would silently perturb the live player) and no
/// infeasible frame is admitted (which would restore the hang).
#[test]
fn the_counting_oracle_agrees_with_exhaustive_partition_search() {
    // A small unseen pool, so exhaustive search is affordable and the
    // adversarial void structures are dense: 9 tiles to three seats.
    let pool: Vec<usize> = (0..9).collect();
    let unseen = pool.iter().fold(0u32, |m, &i| m | (1u32 << i));
    let played = FULL_MASK & !unseen & !0x0007_0000;
    let viewer_hand = 0x0007_0000;
    assert_eq!(FULL_MASK & !played & !viewer_hand, unseen);

    let mut rng = SplitMix64(mix(0x5164_FA17_0000_0001));
    let mut checked = 0usize;
    let mut infeasible = 0usize;
    for size_split in [[3usize, 3, 3], [1, 3, 5], [5, 3, 1], [2, 4, 3], [4, 4, 1]] {
        // Void density is swept as well as the size split: dense frames are
        // mostly infeasible, sparse ones mostly feasible, and the interesting
        // disagreements would live in between.
        for density in [2u64, 3, 4, 6, 9] {
            for _ in 0..80 {
                // Each hidden seat is void in a random subset of the pool.
                // Voids are really contexts, not arbitrary tiles, but the
                // sampler only ever tests `w[s] & voids[s]`, so an arbitrary
                // mask is the widest lawful stress of that predicate.
                let mut voids = [0u32; 4];
                for seat in [1usize, 2, 3] {
                    let mut mask = 0u32;
                    for &tile in &pool {
                        if rng.below(density) == 0 {
                            mask |= 1u32 << tile;
                        }
                    }
                    voids[seat] = mask;
                }
                let sizes = [0usize, size_split[0], size_split[1], size_split[2]];
                let counted = belief_frame_feasibility(0, viewer_hand, played, sizes, voids);
                let searched = exact_partition_exists(
                    unseen,
                    &[
                        (sizes[1], voids[1]),
                        (sizes[2], voids[2]),
                        (sizes[3], voids[3]),
                    ],
                );
                assert_eq!(
                    counted.is_ok(),
                    searched,
                    "the counting oracle decides the sampler's acceptance region \
                     (sizes {sizes:?}, voids {voids:?})"
                );
                if let Err(frame) = &counted {
                    assert!(
                        frame.confined.count_ones() as usize > frame.room,
                        "a refusal names a blocking set that really overflows"
                    );
                    infeasible += 1;
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 2_000);
    assert!(
        infeasible > 100 && infeasible < checked - 100,
        "the sweep exercises both verdicts (saw {infeasible} refusals of {checked})"
    );
}

/// Gate R5, part two — the LEFTOVER case. The sampler slices a prefix of
/// the shuffled pool, so when the declared sizes ask for fewer tiles than
/// the pool holds, the tail is never dealt and a tile no seat may hold is
/// harmless. The oracle must model that, not the exact partition.
#[test]
fn the_oracle_models_the_prefix_slicing_not_an_exact_partition() {
    // Nine unseen tiles, three seats wanting two apiece: three tiles are
    // dealt to nobody. One tile is void for every seat — feasible anyway.
    let unseen = 0x0000_01FFu32;
    let viewer_hand = 0x0007_0000u32;
    let played = FULL_MASK & !unseen & !viewer_hand;
    let sizes = [0usize, 2, 2, 2];
    let mut voids = [0u32; 4];
    for seat in [1usize, 2, 3] {
        voids[seat] = 0x0000_0001;
    }
    belief_frame_feasibility(0, viewer_hand, played, sizes, voids)
        .expect("an undealt tile no seat may hold does not block the frame");

    // Four such tiles cannot all sit in a three-tile tail.
    for seat in [1usize, 2, 3] {
        voids[seat] = 0x0000_000F;
    }
    let refusal = belief_frame_feasibility(0, viewer_hand, played, sizes, voids)
        .expect_err("four universally-void tiles overflow a three-tile tail");
    assert!(refusal.blocking_seats.is_empty(), "no seat can take them");
    assert_eq!(refusal.confined, 0x0000_000F);
    assert_eq!(refusal.room, 3, "the leftover is the only room they have");

    // And the sampler agrees on the feasible side: it draws, and every
    // drawn seat hand respects its voids.
    for seat in [1usize, 2, 3] {
        voids[seat] = 0x0000_0001;
    }
    let mut rng = SplitMix64(mix(0x5164_1EF7_0000_0001));
    let worlds = sample_belief(0, viewer_hand, played, sizes, voids, 32, &mut rng)
        .expect("the feasible leftover frame draws");
    for w in &worlds {
        for seat in [1usize, 2, 3] {
            assert_eq!(w[seat].count_ones() as usize, sizes[seat]);
            assert_eq!(w[seat] & voids[seat], 0, "a drawn hand respects its voids");
        }
    }
}

/// Gate R6 — `sample_open_belief` is total, so the call sites that use it
/// have no refusal branch to get wrong.
///
/// This is the gate that lets the live player's auction path carry no
/// error handling at all. With every void mask zero the sampler's
/// rejection test cannot fire, so the acceptance region is the whole deal
/// space; the sweep confirms the oracle never refuses such a frame across
/// every viewer, every reachable played mask depth, and every size split
/// that fits the pool — and that the drawn deals really are complete and
/// disjoint.
#[test]
fn the_open_frame_entry_point_cannot_refuse() {
    let mut rng = SplitMix64(mix(0x5164_09E4_0000_0001));
    let mut checked = 0usize;
    for viewer in 0..4usize {
        for tricks_played in 0..7usize {
            let remaining = 7 - tricks_played;
            // A played mask of whole tricks, and a viewer hand drawn from
            // what is left — the shape every auction/pre-play caller has.
            let played = (0..(4 * tricks_played)).fold(0u32, |m, i| m | (1u32 << i));
            let pool = FULL_MASK & !played;
            let mut viewer_hand = 0u32;
            let mut candidates = walt::solver::mask_bits(pool);
            for i in (1..candidates.len()).rev() {
                let j = rng.below((i + 1) as u64) as usize;
                candidates.swap(i, j);
            }
            for &tile in candidates.iter().take(remaining) {
                viewer_hand |= 1u32 << tile;
            }
            let mut sizes = [remaining; 4];
            sizes[viewer] = remaining;
            belief_frame_feasibility(viewer, viewer_hand, played, sizes, [0; 4])
                .expect("an open frame is always feasible");
            if remaining == 0 {
                checked += 1;
                continue;
            }
            let worlds = sample_open_belief(viewer, viewer_hand, played, sizes, 8, &mut rng);
            assert_eq!(worlds.len(), 8);
            for w in &worlds {
                assert_eq!(w[viewer], viewer_hand, "the viewer keeps its own hand");
                let union = w.iter().fold(0u32, |a, b| a | b);
                let total: u32 = w.iter().map(|h| h.count_ones()).sum();
                assert_eq!(union, FULL_MASK & !played, "the deal covers the pool");
                assert_eq!(total, union.count_ones(), "the four hands are disjoint");
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 28, "every viewer at every trick depth");
}

/// Gate R2 — before/after determinism. The identical corpus, re-rendered
/// through the repaired library path, must equal the committed capture
/// byte for byte: same drawn worlds, same post-draw RNG state, same σ1
/// field actions, same pmake indicators.
#[test]
fn the_repaired_sampler_reproduces_the_before_side_capture_exactly() {
    let now = render_capture();
    if now != FIXTURE {
        let mut first_difference = String::from("(the two renderings differ in length only)");
        for (i, (a, b)) in now.lines().zip(FIXTURE.lines()).enumerate() {
            if a != b {
                first_difference = format!("line {}: now {a:?} vs captured {b:?}", i + 1);
                break;
            }
        }
        panic!(
            "the repaired sampler must reproduce the before-side capture exactly \
             ({first_difference})"
        );
    }
}

// ---------------------------------------------------------------------------
// R3 — one sampler in the workspace.
// ---------------------------------------------------------------------------

/// Gate R3 — dedup identity, gated BY GREP over the crate's own sources
/// — the first of TWO witnesses, which cover each other's blind spots.
///
/// GREP (this test): exactly one `fn sample_belief(` definition exists
/// under `src/`, and it is the library's. This catches a local copy that
/// compiles because nothing calls it, and a local copy in a module that
/// never imported the library name — neither of which the compiler would
/// object to.
///
/// COMPILE (the build itself, re-armed by the assertion below): each of
/// the four deduplicated binaries carries a MODULE-SCOPE
/// `use walt::solver::sample_belief`, and Rust rejects a local `fn
/// sample_belief` beside it as E0255 — "the name `sample_belief` is
/// defined multiple times". So `cargo build` succeeding is itself proof
/// that no local copy sits in those four modules, and this test asserts
/// the imports are still present, because deleting one would silently
/// disarm that proof. The compile witness also catches what grep cannot:
/// if the library authority were renamed, these imports would fail to
/// resolve, whereas a grep for a fixed name would pass over a wholesale
/// rename in silence.
///
/// Neither witness catches a local copy under a DIFFERENT name that
/// reimplements the loop; that is a review obligation, not a gate.
#[test]
fn exactly_one_sampler_definition_survives_in_the_crate() {
    fn rs_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("the source tree is readable") {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                rs_files(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rs_files(&root, &mut files);
    files.sort();
    assert!(files.len() > 40, "the sweep really read the source tree");

    let mut definitions: Vec<String> = Vec::new();
    for path in &files {
        let text = std::fs::read_to_string(path).expect("a readable source file");
        for line in text.lines() {
            if line.trim_start().starts_with("fn sample_belief(")
                || line.trim_start().starts_with("pub fn sample_belief(")
            {
                definitions.push(
                    path.strip_prefix(&root)
                        .expect("under src")
                        .display()
                        .to_string(),
                );
            }
        }
    }
    assert_eq!(
        definitions,
        vec!["solver/mod.rs".to_string()],
        "one belief sampler, in the library; the four bin copies are gone"
    );

    // The compile witness, kept armed: each deduplicated binary imports the
    // library name at module scope, which is what makes a local definition
    // an E0255 collision rather than a silent shadow.
    for bin in [
        "walt_bridge.rs",
        "playout.rs",
        "playtable.rs",
        "divergence.rs",
    ] {
        let text = std::fs::read_to_string(root.join("bin").join(bin))
            .expect("a readable deduplicated binary");
        // The use-tree may be wrapped across lines by rustfmt, so read the
        // whole `use walt::solver::{ … };` item rather than one line.
        // `sample_open_belief` is not a superstring of `sample_belief`, so
        // the containment test names exactly the item it means.
        let imported = text
            .match_indices("use walt::solver::{")
            .filter_map(|(at, _)| text[at..].find("};").map(|end| &text[at..at + end]))
            .any(|item| item.contains("sample_belief"));
        assert!(
            imported,
            "{bin} keeps its module-scope `use walt::solver::sample_belief`, so a \
             local redefinition is an E0255 compile error and the build itself \
             witnesses the dedup"
        );
    }
}

// ---------------------------------------------------------------------------
// R4 — the previously-blocked MB0 roots.
// ---------------------------------------------------------------------------

/// The MB0 gate file's raw fixed-field authority, reproduced here as
/// test-local reference code: a plain uniform-root factor belief with NO
/// model belief and NO positive-support tightening, priced by
/// `response_success_mass` per legal root action. This is exactly the
/// route the σ1 boundary blocked.
fn raw_authority(
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
) -> Vec<(Domino, u128)> {
    let oracle = SupportOracle;
    let belief = FactorBelief::uniform_root(root, position, field);
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    legal
        .iter()
        .map(|tile| {
            let mut stats = ResponseStats::default();
            let q = response_success_mass(&oracle, &belief.focal_play(tile), field, &mut stats);
            (tile, q)
        })
        .collect()
}

/// Gate R4 — the four roots the MB0 report pinned as the raw σ1
/// authority's refusal set: h5-t6, h4-t6, h8-t5, h3-t5. Against the
/// unpatched sampler each of these HANGS. Under the repair each must
/// finish, and finish in one of exactly two ways: with exact per-action
/// masses, or with a named refusal carrying the unsatisfiable frame.
///
/// The refusal reaches the caller as a panic rather than a value because
/// `SlicePolicy::choose` returns a tile and has no typed channel — the
/// same shape MB0's `GuardedF1` instrument used, and the same shape MB0's
/// G2 pins with `catch_unwind`. The typed outcome is at the sampler; this
/// gate checks the boundary reports it faithfully instead of spinning.
#[test]
fn the_blocked_roots_now_terminate_with_a_value_or_a_named_refusal() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut outcomes: Vec<(String, String)> = Vec::new();
    for (hand_id, trick_no) in [(5usize, 6usize), (4, 6), (8, 5), (3, 5)] {
        let (root, position) = root_at(hand_id, trick_no);
        let field = FieldModel::new(level1_spec());
        let label = format!("h{hand_id}-t{trick_no}");
        let started = std::time::Instant::now();
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            raw_authority(&root, &position, &field)
        }));
        let micros = started.elapsed().as_micros();
        match outcome {
            Ok(per_action) => {
                let rendered: Vec<String> = per_action
                    .iter()
                    .map(|(t, q)| format!("{t:?}={q}"))
                    .collect();
                outcomes.push((
                    label,
                    format!("VALUES in {micros} us: {}", rendered.join(" ")),
                ));
            }
            Err(payload) => {
                let message = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                    .unwrap_or_else(|| "(non-string panic payload)".to_string());
                assert!(
                    message.contains("unsatisfiable belief frame"),
                    "a σ1 refusal names the frame it refused, not something else: {message}"
                );
                outcomes.push((label, format!("REFUSED in {micros} us: {message}")));
            }
        }
    }
    std::panic::set_hook(previous);
    for (label, outcome) in &outcomes {
        println!("R4 {label}: {outcome}");
    }
    assert_eq!(outcomes.len(), 4, "all four blocked roots ran to a verdict");
}
