//! walt trick-1 draw probe — Theorem T1-draw and Corollary T1-ruff at the
//! freeze-47 carrier: the first proved first-trick plays. EXPLORATORY TIER.
//!
//! Rulings: T1-A1..T1-A12 (walt/CENSUS-RULINGS.md, 2026-08-14) — Lemmas
//! T1-run and T1-force, Propositions T1-blind and T1-corner (the bounded
//! sandwich refuted), Theorem T1-draw, Corollary T1-ruff; freeze 47 (the
//! carrier: arm A = the closed drawing family, 294 coordinates, run in
//! full; arm B = the 13 corpus trick-1 hands; the reduced-grade authority
//! ladder; no library entry anywhere).
//!
//! Receipts (T1-A9): (T1-R1) rule-algebra discharge of (Z1)/(Z2)/(Z3) via
//! trick_key and threat; (T1-R2) the grade-reduced authority cross-check —
//! grades 2, 3, 4 mandatory, grade 5 attempted with a declared stop;
//! (T1-R3) exhaustive integer counts over all 399,072,960 worlds, no
//! decimation, integers until one final rational, asserted equal to the
//! closed form where derived; (T1-R4) an exhibited world-and-realisation
//! witness per excluded double; (T1-R5) the exclusion arithmetic as exact
//! rationals. (R1)/(R2)/(R3) of the separation design print as INAPPLICABLE
//! with reason (the concrete authority is structurally absent at grade 7,
//! not budget-stopped). U_a <= g is an arithmetic remark, never a receipt
//! (PG-A8).
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example trick1_draw`

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::replay::deal;
use walt_core::{ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};
use walt_strat::{ScalarHidden, ScalarValuation};

/// Freeze 26: the concrete authority's budget (reduced-grade ladder only).
const AUTHORITY_BUDGET: u64 = 200_000_000;

const T1_FIBER: u128 = 399_072_960; // C(21,7) * C(14,7)

fn tile(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

fn set_str(s: DominoSet) -> String {
    let v: Vec<String> = s.iter().map(tile).collect();
    v.join(" ")
}

/// The trump-suit rank order under PipTrump(p): p:p (DOUBLE_TOP) first, then
/// descending pip sum. Returns the called set sorted descending by rank.
fn trump_rank_order(decl: Decl) -> Vec<Domino> {
    let called = decl.called_set();
    let mut v: Vec<Domino> = called.iter().collect();
    v.sort_by(|a, b| decl.rank(*b).cmp(&decl.rank(*a)));
    v
}

/// A trick-1 kernel: viewer S0 on lead, hand H, all 21 other tiles unseen at
/// capacities 7/7/7, voids empty (the complete split set).
fn trick1_kernel(decl: Decl, hand: DominoSet) -> Kernel {
    let pool: DominoSet = Domino::ALL
        .into_iter()
        .filter(|d| !hand.contains(*d))
        .collect();
    let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
        seat: s,
        capacity: 7,
        voids: ContextSet::EMPTY,
    });
    Kernel::new(decl, Seat::S0, hand, pool, hidden).expect("trick-1 kernel")
}

// -- the exhaustive fiber count (T1-R3): Gosper enumeration over the 21
// unseen tiles; the sum is order-invariant and the world count is asserted
// against kernel.count(). ------------------------------------------------

fn gosper_next(v: u32) -> u32 {
    let c = v & v.wrapping_neg();
    let r = v + c;
    (((r ^ v) >> 2) / c) | r
}

/// One coordinate's per-double structures for the q(J) count.
struct DoubleCount {
    j: Domino,
    /// context-J tiles among the unseen, as a 21-bit mask.
    ctx_mask: u32,
    /// numerator table keyed by (m1', m2', m3') where m' = the seat's
    /// outstanding-trump mask if context-void else 0; value = the numerator
    /// (over 343) of P(an OPPONENT takes the J trick).
    table: Vec<u64>,
    /// accumulated numerator over worlds (denominator N * 343).
    total: u64,
}

/// Precompute the 512-entry table for one (coordinate, J): outstanding
/// trumps R (as ranked list), per-seat effective mask keys. Opponents are
/// seats 1 and 3 (the partner, seat 2, taking the trick keeps it with the
/// focal team). Each void seat plays each specific tile w.p. 1/7 (its hand
/// is 7 tiles at trick 1), so per seat: a specific trump = 1, no-trump =
/// 7 - |m'|; a compelled follower contributes no-trump with weight 7.
fn ruff_table(r_tiles: &[Domino], decl: Decl) -> Vec<u64> {
    let n_r = r_tiles.len();
    assert!(n_r <= 3, "outstanding trumps at t >= 4");
    // Rank order within R: r_tiles is already descending by rank.
    let keys = 1usize << n_r;
    let mut table = vec![0u64; keys * keys * keys];
    for m1 in 0..keys {
        for m2 in 0..keys {
            for m3 in 0..keys {
                // Enumerate each seat's play: one of its trumps, or none.
                let mut num: u64 = 0;
                let opts = |m: usize| -> Vec<(Option<usize>, u64)> {
                    let mut v: Vec<(Option<usize>, u64)> = Vec::new();
                    let mut cnt = 0u64;
                    for (bi, _) in r_tiles.iter().enumerate() {
                        if m & (1 << bi) != 0 {
                            v.push((Some(bi), 1));
                            cnt += 1;
                        }
                    }
                    v.push((None, 7 - cnt));
                    v
                };
                for (p1, w1) in opts(m1) {
                    for (p2, w2) in opts(m2) {
                        for (p3, w3) in opts(m3) {
                            // Winner: the highest-ranked trump played (lowest
                            // index in the descending r_tiles order); if none,
                            // the led double J wins for the focal team.
                            let best = [p1, p2, p3]
                                .iter()
                                .enumerate()
                                .filter_map(|(seat, p)| p.map(|bi| (bi, seat)))
                                .min_by_key(|(bi, _)| *bi);
                            let opponent_takes = match best {
                                Some((_, seat)) => seat == 0 || seat == 2, // seats S1, S3
                                None => false,
                            };
                            if opponent_takes {
                                num += w1 * w2 * w3;
                            }
                        }
                    }
                }
                let _ = decl;
                table[(m1 * keys + m2) * keys + m3] = num;
            }
        }
    }
    table
}

/// One coordinate's exhaustive fiber pass: counts every world once, folding
/// the q(J) numerators (and, when asked, the T1-force f sum for arm B).
/// Returns (world count, per-J totals, f_sum).
fn fiber_pass(
    hand: DominoSet,
    decl: Decl,
    doubles: &mut [DoubleCount],
    // For arm B: the trump rank order over ALL trumps in play with a side
    // marker for the focal's own (true = focal side holds it a priori).
    force: Option<&[(u32, bool)]>,
) -> (u128, u64) {
    let unseen: Vec<Domino> = Domino::ALL
        .into_iter()
        .filter(|d| !hand.contains(*d))
        .collect();
    assert_eq!(unseen.len(), 21, "trick-1 unseen");
    let r_masks: Vec<(u32, usize)> = {
        // Outstanding trumps among the unseen, descending rank, with bit
        // index into the per-double tables.
        let order = trump_rank_order(decl);
        order
            .iter()
            .filter(|d| !hand.contains(**d))
            .enumerate()
            .map(|(bi, d)| {
                let pos = unseen.iter().position(|u| u == d).expect("unseen trump");
                (1u32 << pos, bi)
            })
            .collect()
    };
    let full: u32 = (1 << 21) - 1;
    let mut worlds: u128 = 0;
    let mut f_sum: u64 = 0;
    let mut s1: u32 = (1 << 7) - 1;
    loop {
        let rem = full & !s1;
        //

        // Compact the 14 remaining bit positions.
        let mut rem_pos = [0u8; 14];
        let mut k = 0;
        for b in 0..21u8 {
            if rem & (1 << b) != 0 {
                rem_pos[k] = b;
                k += 1;
            }
        }
        let mut s2c: u32 = (1 << 7) - 1;
        loop {
            // Expand s2 compact to 21-bit.
            let mut s2: u32 = 0;
            let mut m = s2c;
            while m != 0 {
                let b = m.trailing_zeros() as usize;
                s2 |= 1 << rem_pos[b];
                m &= m - 1;
            }
            let s3 = rem & !s2;
            worlds += 1;
            for dc in doubles.iter_mut() {
                let keyify = |seat_mask: u32| -> usize {
                    if seat_mask & dc.ctx_mask != 0 {
                        return 0; // compelled follower: cannot ruff
                    }
                    let mut key = 0usize;
                    for (rm, bi) in &r_masks {
                        if seat_mask & rm != 0 {
                            key |= 1 << bi;
                        }
                    }
                    key
                };
                let keys = 1usize << r_masks.len();
                let k1 = keyify(s1);
                let k2 = keyify(s2);
                let k3 = keyify(s3);
                dc.total += dc.table[(k1 * keys + k2) * keys + k3];
            }
            if let Some(order) = force {
                // Lemma T1-force at the opponents' side: walk the trump rank
                // order from the top; extend while held by S1 or S3; c = max
                // per-seat count in the prefix.
                let (mut c1, mut c3) = (0u64, 0u64);
                for (mask, focal_side) in order {
                    if *focal_side {
                        break;
                    }
                    if s1 & mask != 0 {
                        c1 += 1;
                    } else if s3 & mask != 0 {
                        c3 += 1;
                    } else {
                        // partner (S2) holds it: focal side — prefix ends
                        break;
                    }
                }
                f_sum += c1.max(c3);
            }
            if s2c == ((1 << 7) - 1) << 7 {
                break;
            }
            s2c = gosper_next(s2c);
            if s2c > (1 << 14) - 1 {
                break;
            }
        }
        if s1 == ((1 << 7) - 1) << 14 {
            break;
        }
        s1 = gosper_next(s1);
        if s1 > full {
            break;
        }
    }
    (worlds, f_sum)
}

// -- arm A: the drawing family (freeze 47(a)) -------------------------------

struct ArmACoord {
    pip: u8,
    t: usize,
    doubles: Vec<Domino>,
    hand: DominoSet,
}

/// The closed family: pip ascending; t descending; the non-trump doubles'
/// pips ascending lexicographically. 294 coordinates.
fn arm_a_family() -> Vec<ArmACoord> {
    let mut out = Vec::new();
    for pip in 0..=6u8 {
        let decl = Decl::PipTrump(Pip::new(pip).expect("pip"));
        let order = trump_rank_order(decl);
        let non_trump_doubles: Vec<Domino> = (0..=6u8)
            .filter(|j| *j != pip)
            .map(|j| Domino::new(Pip::new(j).expect("pip"), Pip::new(j).expect("pip")))
            .collect();
        for t in (4..=7usize).rev() {
            let need = 7 - t;
            // combinations of `need` of the six doubles, lexicographic.
            let idxs: Vec<usize> = (0..6).collect();
            let mut combo: Vec<usize> = (0..need).collect();
            loop {
                let doubles: Vec<Domino> = combo.iter().map(|i| non_trump_doubles[*i]).collect();
                let mut hand = DominoSet::EMPTY;
                for d in order.iter().take(t) {
                    hand.insert(*d);
                }
                for d in &doubles {
                    hand.insert(*d);
                }
                out.push(ArmACoord {
                    pip,
                    t,
                    doubles: doubles.clone(),
                    hand,
                });
                if need == 0 {
                    break;
                }
                // next combination
                let mut i = need;
                loop {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                    if combo[i] != idxs.len() - need + i {
                        combo[i] += 1;
                        for j2 in i + 1..need {
                            combo[j2] = combo[j2 - 1] + 1;
                        }
                        break;
                    }
                    if i == 0 {
                        combo.clear();
                        break;
                    }
                }
                if combo.is_empty() {
                    break;
                }
            }
        }
    }
    assert_eq!(out.len(), 294, "the closed family is 294 coordinates");
    out
}

/// One arm-A coordinate: receipts + verdicts, rendered.
fn run_arm_a(c: &ArmACoord, out: &mut String) {
    let decl = Decl::PipTrump(Pip::new(c.pip).expect("pip"));
    let kernel = trick1_kernel(decl, c.hand);
    assert_eq!(kernel.count(), T1_FIBER, "|X| = 399,072,960");
    // Freeze-45-form identity + rebuild.
    let rebuilt = trick1_kernel(decl, kernel.viewer_hand());
    assert_eq!(rebuilt, kernel, "kernel rebuilt from printed identity");
    let order = trump_rank_order(decl);
    let called = decl.called_set();
    let h_trumps: DominoSet = c.hand.intersection(called);
    let t = h_trumps.len();
    assert_eq!(t, c.t, "t identity");

    // (T1-R1) the rule-algebra discharge.
    let top_t: DominoSet = order.iter().take(t).copied().collect();
    assert_eq!(h_trumps, top_t, "(Z1): H ∩ T is the top t of T under trick_key");
    assert!(2 * t >= 7, "(Z2): 2t >= |T| = 7");
    for j in &c.doubles {
        assert!(j.is_double() && !decl.is_called(*j), "(Z3): a natural double");
        assert!(
            decl.threat(*j).is_subset_of(called),
            "(Z3): threat(J) ⊆ called_set"
        );
    }

    let trivial = t == 7;
    let _ = writeln!(*out);
    let _ = writeln!(
        *out,
        "== t1-A coord decl=PipTrump({}) t={} hand=[{}] leader-offset=0 |X|={T1_FIBER} enumeration=freeze-7/23 (sum order-invariant; count asserted) ==",
        c.pip,
        c.t,
        set_str(c.hand)
    );
    let _ = writeln!(
        *out,
        "  (T1-R1) (Z1)+(Z2)+(Z3) discharged by the rule algebra (trick_key top-{t}, 2t>={}, threat(J) ⊆ called_set) — HELD",
        7
    );
    if trivial {
        let _ = writeln!(
            *out,
            "  TRIVIAL — every action takes every trick (the all-trump hand); membership Q^H(a) = +7 for every a by Theorem T1-draw; no competitor exists and no decision is certified."
        );
        return;
    }

    // The q(J) counts (T1-R3) — one exhaustive pass for all doubles at once.
    let unseen: Vec<Domino> = Domino::ALL
        .into_iter()
        .filter(|d| !c.hand.contains(*d))
        .collect();
    let r_tiles: Vec<Domino> = order.iter().filter(|d| !c.hand.contains(**d)).copied().collect();
    let mut dcs: Vec<DoubleCount> = c
        .doubles
        .iter()
        .map(|j| {
            let ctx = decl.effective_incidence(decl.led_context(*j));
            let mut ctx_mask: u32 = 0;
            for (i, u) in unseen.iter().enumerate() {
                if ctx.contains(*u) && *u != *j {
                    ctx_mask |= 1 << i;
                }
            }
            DoubleCount {
                j: *j,
                ctx_mask,
                table: ruff_table(&r_tiles, decl),
                total: 0,
            }
        })
        .collect();
    let (worlds, _) = fiber_pass(c.hand, decl, &mut dcs, None);
    assert_eq!(worlds, T1_FIBER, "exhaustive: every world enumerated once");

    let n_i128 = i128::try_from(T1_FIBER).expect("fits");
    for dc in &dcs {
        let qj = q(i128::try_from(dc.total).expect("fits"), n_i128 * 343);
        // (T1-R4) exhibited witness: top outstanding trump with S1, S1 void
        // in context-J.
        let top_out = r_tiles[0];
        let ctx = decl.effective_incidence(decl.led_context(dc.j));
        let mut s1 = DominoSet::EMPTY;
        s1.insert(top_out);
        for u in &unseen {
            if s1.len() == 7 {
                break;
            }
            if *u != top_out && !ctx.contains(*u) && !called.contains(*u) {
                s1.insert(*u);
            }
        }
        assert_eq!(s1.len(), 7, "witness hand fills");
        let mut rest: Vec<Domino> = unseen.iter().filter(|u| !s1.contains(**u)).copied().collect();
        let s2: DominoSet = rest.drain(..7).collect();
        let s3: DominoSet = rest.into_iter().collect();
        // Field realisation: S1 plays the trump; S2, S3 play their least
        // legal tiles.
        let led = decl.led_context(dc.j);
        let pick = |h: DominoSet| -> Domino {
            let legal = walt_core::legal_plays(decl, h, Some(led));
            legal.iter().min_by_key(|d| d.index()).expect("nonempty")
        };
        let p2 = pick(s2);
        let p3 = pick(s3);
        let trick = Trick::new(Seat::S0, [dc.j, top_out, p2, p3]).expect("distinct");
        assert_eq!(
            trick.winner(decl),
            Seat::S1,
            "(T1-R4): the exhibited realisation concedes trick 1 to an opponent"
        );
        assert!(dc.total > 0, "(T1-R4)-consistency: q(J) > 0");
        let _ = writeln!(
            *out,
            "  (T1-R4) witness for J={}: S1=[{}] S2=[{}] S3=[{}]; plays [{} {} {} {}] -> S1 takes trick 1 (a positive-probability realisation of the declared field)",
            tile(dc.j),
            set_str(s1),
            set_str(s2),
            set_str(s3),
            tile(dc.j),
            tile(top_out),
            tile(p2),
            tile(p3)
        );
        let bound = if c.doubles.len() == 1 {
            format!(
                "Q^H({}) = 7 − 2q = {} EXACTLY (Corollary T1-ruff equality, |H∖T| = 1)",
                tile(dc.j),
                qi(7) - qj * qi(2)
            )
        } else {
            format!(
                "Q^H({}) <= 7 − 2q = {} (Corollary T1-ruff; equality not claimed at |H∖T| > 1)",
                tile(dc.j),
                qi(7) - qj * qi(2)
            )
        };
        let _ = writeln!(
            *out,
            "  (T1-R3/T1-R5) q({}) = {} (exhaustive integer count over {T1_FIBER} worlds, no decimation, one final rational); {} — EXCLUDED from Opt^H (q > 0)",
            tile(dc.j),
            qj,
            bound
        );
    }
    let _ = writeln!(
        *out,
        "  VERDICT: Q^H(a) = +7 for every trump lead (Theorem T1-draw — membership belief-free and field-free, holding in every world against every field behaviour); every double lead strictly excluded (model-relative, under the declared uniform belief and uniform-random legal field); Opt^H = H ∩ T = [{}] EXACTLY. Theorem E6.4's member-not-set caveat is DISCHARGED, not waived: both sides are exact values, not bounds.",
        set_str(c.hand.intersection(called))
    );
    let _ = writeln!(
        *out,
        "  a drawing hand is a hand that plays itself; the theorem certifies a first-trick play where no search is needed to find it, and says nothing about hands that require judgement."
    );
}

// -- arm B: the 13 corpus trick-1 coordinates -------------------------------

fn run_arm_b(hand_idx: usize, receipt: &Receipt, out: &mut String) {
    let rh = &receipt.hands[hand_idx];
    let hands = deal(rh).expect("the corpus deal replays");
    let focal = rh.bidder;
    let decl = rh.decl;
    let Decl::PipTrump(_) = decl else {
        let _ = writeln!(
            *out,
            "== t1-B corpus hand {hand_idx}: declaration {decl:?} out of pip-trump scope (F1) =="
        );
        return;
    };
    let h = hands[focal.index()];
    // Trick-1 kernel from the focal's chair: everything else unseen.
    let pool: DominoSet = Domino::ALL
        .into_iter()
        .filter(|d| !h.contains(*d))
        .collect();
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    for (slot, k) in hidden.iter_mut().zip(1..=3) {
        *slot = Hidden {
            seat: focal.plus(k),
            capacity: 7,
            voids: ContextSet::EMPTY,
        };
    }
    let kernel = Kernel::new(decl, focal, h, pool, hidden).expect("trick-1 corpus kernel");
    assert_eq!(kernel.count(), T1_FIBER, "|X| = 399,072,960");

    let order = trump_rank_order(decl);
    let called = decl.called_set();
    // k: the focal's own top run in T.
    let mut k_run = 0usize;
    for d in &order {
        if h.contains(*d) {
            k_run += 1;
        } else {
            break;
        }
    }
    // T1-draw hypothesis check.
    let h_trumps = h.intersection(called);
    let t = h_trumps.len();
    let top_t: DominoSet = order.iter().take(t).copied().collect();
    let z1 = h_trumps == top_t;
    let z2 = 2 * t >= 7;
    let z3 = h
        .difference(called)
        .iter()
        .all(|j| j.is_double() && decl.threat(j).is_subset_of(called));
    let drawing = z1 && z2 && z3;

    // E_beta[f] by exhaustive count: the trump rank order with focal-side
    // markers (the focal's own trumps are focal-side a priori; the partner's
    // holdings vary per world and are handled inside the pass).
    let unseen: Vec<Domino> = Domino::ALL
        .into_iter()
        .filter(|d| !h.contains(*d))
        .collect();
    let force_order: Vec<(u32, bool)> = order
        .iter()
        .map(|d| {
            if h.contains(*d) {
                (0u32, true)
            } else {
                let pos = unseen.iter().position(|u| u == d).expect("unseen trump");
                (1u32 << pos, false)
            }
        })
        .collect();
    let mut no_doubles: Vec<DoubleCount> = Vec::new();
    let (worlds, f_sum) = fiber_pass(h, decl, &mut no_doubles, Some(&force_order));
    assert_eq!(worlds, T1_FIBER, "exhaustive");
    let ef = q(
        i128::try_from(f_sum).expect("fits"),
        i128::try_from(T1_FIBER).expect("fits"),
    );
    let gap = qi(7) - qi(i128::try_from(k_run).expect("fits")) - ef;
    let _ = writeln!(*out);
    let _ = writeln!(
        *out,
        "== t1-B corpus hand {hand_idx} decl={decl:?} focal={focal:?} hand=[{}] |X|={T1_FIBER} ==",
        set_str(h)
    );
    let _ = writeln!(
        *out,
        "  REAL-DEAL FENCE, both halves (T1-A7): the void-filtered ratio is 1 BY CONSTRUCTION at trick 1 (nothing has been played, no void is known) — the arithmetic half is vacuous and its vacuity is not a strengthening of the belief half, which stands entire: uniform over this fiber is NOBODY'S belief, and no seat at the table holds it. No row is a statement about correct play in this deal."
    );
    let _ = writeln!(
        *out,
        "  T1-draw hypothesis check: (Z1) {} (Z2) {} (Z3) {} -> {}",
        z1,
        z2,
        z3,
        if drawing {
            "DRAWING HAND (a real deal that plays itself — reported as what it is, not promoted by being real)"
        } else {
            "not a drawing hand (expected; filed as a result)"
        }
    );
    let _ = writeln!(
        *out,
        "  corner instruments: k = {k_run} (fiber-constant, Lemma T1-run); E_β[f] = {ef} (exhaustive count, Lemma T1-force); CORNER GAP 7 − k − E_β[f] = {gap} — the exact specification of what a tighter relaxation (Theorem E6.5's gluing, freeze 38, reserved) must beat. Typed: a cost-model and specification quantity; licenses no claim about the game; P-A21: not quoted for any other grade or the opening in general."
    );
}

// -- (T1-R2): the grade-reduced authority cross-check ladder ----------------

/// The declared reduced construction at grade g (freeze 47(c) ladder):
/// PipTrump(6); H = top (g−1) trumps + 5:5; the single outstanding trump is
/// the next-ranked 6-tile; the pool fills with the lowest-index tiles that
/// are neither trumps nor in H (deterministic).
fn reduced_coordinate(g: usize) -> (Kernel, Domino, Domino) {
    let decl = Decl::PipTrump(Pip::new(6).expect("pip"));
    let order = trump_rank_order(decl);
    let five5 = Domino::new(Pip::new(5).expect("pip"), Pip::new(5).expect("pip"));
    let mut hand = DominoSet::EMPTY;
    for d in order.iter().take(g - 1) {
        hand.insert(*d);
    }
    hand.insert(five5);
    let outstanding = order[g - 1];
    let mut pool = DominoSet::EMPTY;
    pool.insert(outstanding);
    for d in Domino::ALL {
        if pool.len() == 3 * g {
            break;
        }
        if !hand.contains(d) && !decl.is_called(d) && d != five5 {
            pool.insert(d);
        }
    }
    assert_eq!(pool.len(), 3 * g, "reduced pool fills");
    let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
        seat: s,
        capacity: g,
        voids: ContextSet::EMPTY,
    });
    (
        Kernel::new(decl, Seat::S0, hand, pool, hidden).expect("reduced kernel"),
        five5,
        outstanding,
    )
}

/// q at a reduced coordinate, by direct enumeration of the (small) fiber.
fn reduced_q(kernel: &Kernel, j: Domino, outstanding: Domino) -> Q {
    let decl = kernel.decl();
    let ctx = decl.effective_incidence(decl.led_context(j));
    let g = kernel.viewer_hand().len();
    let gi = i128::try_from(g).expect("fits");
    let mut num: i128 = 0;
    let mut worlds: i128 = 0;
    for w in kernel.worlds() {
        worlds += 1;
        let hands = w.hands();
        // Which seat holds the single outstanding trump?
        for (si, seat) in [Seat::S1, Seat::S2, Seat::S3].iter().enumerate() {
            let hh = hands[seat.index()];
            if hh.contains(outstanding) {
                let opponent = si == 0 || si == 2;
                let compelled = !hh.intersection(ctx).is_empty();
                if opponent && !compelled {
                    // plays it w.p. 1/g (uniform over its g tiles)
                    num += 1;
                }
                break;
            }
        }
    }
    // P = (worlds where playable by an opponent)/N * 1/g
    q(num, worlds * gi)
}

fn run_t1_r2(out: &mut String) {
    let _ = writeln!(*out);
    let _ = writeln!(
        *out,
        "== (T1-R2) the grade-reduced authority cross-check — the receipt that earns the trick-1 claim (grades 2, 3, 4 mandatory; grade 5 attempted with a declared stop) =="
    );
    for g in 2..=5usize {
        let (kernel, five5, outstanding) = reduced_coordinate(g);
        let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
        let auth = ScalarHidden::new(
            kernel.decl(),
            kernel.viewer(),
            kernel.viewer().team(),
            ScalarValuation::trick_only(),
        );
        let mut ab = AUTHORITY_BUDGET;
        let (maybe_av, stats) = auth.action_values_dag(&worlds, kernel.viewer(), &[], &mut ab);
        match maybe_av {
            None => {
                assert!(g == 5, "grades 2-4 are mandatory and must complete");
                let _ = writeln!(
                    *out,
                    "  grade {g}: DECLARED STOP — authority budget {AUTHORITY_BUDGET} exhausted (steps {}); the grade-5 attempt is reported either way (R-A18)",
                    stats.steps
                );
            }
            Some(av) => {
                let gq = qi(i128::try_from(g).expect("fits"));
                let qv = reduced_q(&kernel, five5, outstanding);
                let predicted_j = gq - qv * qi(2);
                for (a, v) in &av {
                    if kernel.decl().is_called(*a) {
                        assert_eq!(
                            *v, gq,
                            "(T1-R2) stop-and-report: the authority disagrees with Theorem T1-draw at a trump lead"
                        );
                    } else {
                        assert_eq!(*a, five5, "the only double");
                        assert_eq!(
                            *v, predicted_j,
                            "(T1-R2) stop-and-report: the authority disagrees with Corollary T1-ruff's closed form"
                        );
                    }
                }
                let _ = writeln!(
                    *out,
                    "  grade {g}: hand=[{}] |X|={} — authority values equal the theorem EXACTLY: +{g} at every trump lead; Q^H(55) = {g} − 2·{qv} = {predicted_j} — HELD (dag-v1 steps {})",
                    set_str(kernel.viewer_hand()),
                    kernel.count(),
                    stats.steps
                );
            }
        }
    }
}

fn out_dir(name: &str) -> PathBuf {
    let a = PathBuf::from(format!("walt-factory/{name}"));
    if a.exists() {
        return a;
    }
    let b = PathBuf::from(name);
    if b.exists() {
        b
    } else {
        a
    }
}

fn main() {
    let t0 = Instant::now();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "walt trick-1 draw probe — Theorem T1-draw and Corollary T1-ruff at the freeze-47 carrier — exploratory tier"
    );
    let _ = writeln!(
        out,
        "rulings: T1-A1..T1-A12 (walt/CENSUS-RULINGS.md 2026-08-14); freeze 47; mathematics: Lemmas T1-run and T1-force, Propositions T1-blind and T1-corner (the bounded-sandwich refutation, itself a filed result), Theorem T1-draw, Corollary T1-ruff; Theorem E6.4 (member-not-set, discharged where both sides are exact)"
    );
    let _ = writeln!(
        out,
        "regenerate: cargo run --release -p walt-factory --example trick1_draw"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "INAPPLICABLE RECEIPTS, with reason (T1-A9): (R1), (R2) and (R3) of the separation design have no referent at grade 7 — the concrete authority is STRUCTURALLY ABSENT there, not budget-stopped, which is a different situation from N4-A6's Tier 2 and borrows none of its language. Five receipts replace them: (T1-R1)..(T1-R5). U_a <= 7 is true by the range of the valuation and is an arithmetic remark, never a receipt (PG-A8)."
    );
    let _ = writeln!(
        out,
        "FENCES (T1-A8, verbatim): (i) the membership half is belief-free and field-free — it holds pointwise in every world of the fiber and against every field behaviour, and because the trick-1 fiber is the complete set of deals consistent with the focal's hand, it is a statement about the rules rather than the model; this is the one place in walt where R-A2's feasible-versus-reachable fence does not bind a verdict, not because it was relaxed but because the statement ranges over everything; the fence binds everything else in the row. (ii) the exclusion half IS model-relative: q(J) and Q^H(J) are expectations under the declared uniform belief and uniform-random legal field, named in place; no row lets (i)'s strength leak onto (ii). (iii) nothing about points or marks (trick_diff is count-free, E-A2); nothing about bidding; nothing about how real opponents play; nothing outside the declared carrier; no growth law and no opening claim from any grade (P-A21); arm A's coordinates are constructions, not deals. (iv) a drawing hand is a hand that plays itself; the theorem certifies a first-trick play where no search is needed to find it, and says nothing whatever about hands that require judgement."
    );
    let _ = writeln!(
        out,
        "RISK, carried (T1-A12): every statement here is proved relative to walt's implementation of the rule algebra as read from rules.rs at adjudication time; (T1-R2) checks the theorem against an independently written solver but not against the rules corpus; the corpus check is mandatory before any of this is cited outside walt."
    );
    let _ = writeln!(
        out,
        "no library entry is written at any coordinate of either arm (freeze 47(d))."
    );

    // (T1-R2) first: the receipt that earns the claim.
    run_t1_r2(&mut out);

    // The flagship closed-form assert (T1-R3's "asserted equal to the closed
    // form where one is derived") is checked inside arm A when the flagship
    // coordinate is reached; recorded here for the reader.
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "flagship closed form (Corollary T1-ruff, computed at adjudication): q(5:5) = (2/3)(1001/7752)(1/7) = 143/11628; Q^H(5:5) = 7 − 143/5814; asserted against the exhaustive count at the flagship coordinate below."
    );

    // Arm A, W-parallel over coordinates (pure integer functions; the
    // deterministic text is assembled in canonical order at the end; no
    // stop, no timing, no checkpoint discipline is engaged).
    let family = arm_a_family();
    let texts: Mutex<BTreeMap<usize, String>> = Mutex::new(BTreeMap::new());
    let next = AtomicUsize::new(0);
    let family_ref = &family;
    let texts_ref = &texts;
    let next_ref = &next;
    let flagship_checked = AtomicUsize::new(0);
    let flagship_ref = &flagship_checked;
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || loop {
                let i = next_ref.fetch_add(1, Ordering::SeqCst);
                if i >= family_ref.len() {
                    break;
                }
                let c = &family_ref[i];
                let mut text = String::new();
                run_arm_a(c, &mut text);
                // The flagship: p=6, t=6, doubles = {5:5}.
                if c.pip == 6 && c.t == 6 && c.doubles.len() == 1 && tile(c.doubles[0]) == "55" {
                    assert!(
                        text.contains("q(55) = 143/11628"),
                        "flagship closed-form assert: the exhaustive count must equal 143/11628"
                    );
                    flagship_ref.fetch_add(1, Ordering::SeqCst);
                }
                texts_ref.lock().expect("texts").insert(i, text);
            });
        }
    });
    assert_eq!(
        flagship_checked.load(Ordering::SeqCst),
        1,
        "the flagship coordinate was reached and its closed form asserted"
    );
    let texts = texts.into_inner().expect("texts");
    for i in 0..family.len() {
        out.push_str(texts.get(&i).expect("every coordinate ran"));
    }

    // Arm B: the 13 corpus trick-1 coordinates, corpus index ascending.
    let receipt = {
        let path =
            locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
        parse_file(&path).expect("the receipt parses")
    };
    let bt: Mutex<BTreeMap<usize, String>> = Mutex::new(BTreeMap::new());
    let bnext = AtomicUsize::new(0);
    let receipt_ref = &receipt;
    let bt_ref = &bt;
    let bnext_ref = &bnext;
    let n_hands = receipt.hands.len();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || loop {
                let i = bnext_ref.fetch_add(1, Ordering::SeqCst);
                if i >= n_hands {
                    break;
                }
                let mut text = String::new();
                run_arm_b(i, receipt_ref, &mut text);
                bt_ref.lock().expect("bt").insert(i, text);
            });
        }
    });
    let bt = bt.into_inner().expect("bt");
    for i in 0..n_hands {
        out.push_str(bt.get(&i).expect("every corpus hand ran"));
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend; computed W-parallel over pure integer functions with canonical-order assembly): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");
    let results = out_dir("results").join("trick1_draw_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    println!("{}", &out[..out.len().min(6000)]);
    println!("results: {}", results.display());
}
