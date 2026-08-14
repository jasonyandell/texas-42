//! walt separation probe — Experiment E of the decision-sparse track:
//! root-action certification by primal and upper witnesses, at the three
//! canonical grade-3 coordinates. EXPLORATORY TIER.
//!
//! Design: `walt/SEPARATION-PROBE.md` (adjudicated). Rulings: SEP-A1..SEP-A18
//! (`walt/CENSUS-RULINGS.md`, 2026-08-13); freezes 36 (candidate-policy
//! library, SEP-A4) and 37 (action-conditioned upper witness, SEP-A6);
//! mathematics per the errata under DS-A17's citation rule — Lemma E3 (upper
//! witness), Lemma E4 + Corollary E4.1 (primal witness, ceiling, exact
//! negative), Theorem E6.4 (member-not-set), DS-A14/DS-A27 (structural
//! max-freedom).
//!
//! Convention (freeze 37(c)): every evaluator runs in the focal-minus-opponent
//! trick DIFFERENTIAL; the report is in S6a's COUNT convention across the
//! freeze-26 bridge Q_diff = 2*Q_count - grade, asserted exactly at the
//! boundary; the bridge is affine with slope 2 > 0, so every verdict is
//! convention-invariant.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example separation_probe`

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::Instant;

use walt_core::receipt::{locate_verify_player, parse_file, Receipt, ReceiptHand};
use walt_core::replay::{state_before_trick, voids_before_trick};
use walt_core::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};
use walt_strat::{
    information_prices, policy_value_receipt, Direction, InfoPartition, Policy, ScalarHidden,
    ScalarValuation,
};

/// Freeze 26: the concrete authority and its budget.
const AUTHORITY_BUDGET: u64 = 200_000_000;

/// Freeze 44(e): B walk-steps per (coordinate, action) for per-action
/// evaluators; 4B whole-call for `revealed_summary`. At grade 3 these are
/// non-binding ceilings, asserted so in-run (residuals strictly positive,
/// printed to stdout only — the results file carries no budget rows, per
/// N4-A10's two-permitted-differences contract for this receipt).
const B_WALK: u64 = 10_000_000_000;
const B_WALK_4: u64 = 40_000_000_000;
/// Freeze 44(e): the partition-state cap, checked at each insertion.
const P_MAX: usize = 32_000_000;

const GRADE: usize = 3;

/// Freeze 36(c): the library frame digest. A mismatch at load is corruption;
/// the file is discarded entire, never partially reused.
const LIB_DIGEST: &str =
    "SEP-lib-v1|freezes-22-26-36-37|contract=R-A11-full-record|field=uniform-legal-F4|belief=uniform-fiber-freeze7|grade=3";

/// One filed S6a per-action row: (hi, lo, Q^H numerator, Q^H denominator).
type FiledAction = (u8, u8, i128, i128);
/// One filed S6a coordinate: base index, per-action rows in ascending domino
/// order, aggregate fusion gap V^F - V^H as (num, den).
type FiledCoord = (u128, [FiledAction; 3], (i128, i128));

/// SEP-A14(ii): the S6a filed values enter as a frozen table, never re-parsed
/// from results text. Quoted from `predictive_rank_2026-08-12.txt`, S6a,
/// exploratory tier. Count convention.
const S6A_FILED: [FiledCoord; 3] = [
    (
        0,
        [(0, 0, 53, 21), (1, 0, 355, 168), (1, 1, 16319, 6720)],
        (9301, 120_960),
    ),
    (
        1_299_709,
        [(2, 1, 1, 1), (2, 2, 43, 42), (6, 3, 127, 126)],
        (2663, 181_440),
    ),
    (
        2_599_418,
        [(1, 1, 15, 14), (2, 2, 15, 14), (4, 2, 43, 42)],
        (23, 420),
    ),
];

// -- coordinate machinery (S6a freezes 22-25, copied verbatim shape) --------

#[derive(Clone, PartialEq, Eq, Debug)]
struct Interface {
    pip: u8,
    hand: DominoSet,
    pool: DominoSet,
    grade: usize,
}

impl Interface {
    fn decl(&self) -> Decl {
        Decl::PipTrump(Pip::new(self.pip).expect("pip"))
    }
    fn kernel(&self) -> Kernel {
        let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
            seat: s,
            capacity: self.grade,
            voids: ContextSet::EMPTY,
        });
        Kernel::new(self.decl(), Seat::S0, self.hand, self.pool, hidden).expect("kernel")
    }
}

fn binom(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    let mut out: u128 = 1;
    for i in 0..k.min(n - k) {
        out = out * (n - i) / (i + 1);
    }
    out
}

fn unrank_comb(n: usize, k: usize, mut rank: u128) -> Vec<usize> {
    let mut out = Vec::with_capacity(k);
    let (mut x, mut k, mut n_left) = (0usize, k, n);
    while k > 0 {
        let c = binom((n_left - 1) as u128, (k - 1) as u128);
        if rank < c {
            out.push(x);
            k -= 1;
        } else {
            rank -= c;
        }
        x += 1;
        n_left -= 1;
    }
    out
}

fn coordinate(grade: usize, index: u128) -> Interface {
    let live_c = binom(28, (4 * grade) as u128);
    let hand_c = binom((4 * grade) as u128, grade as u128);
    let pip = (index / (live_c * hand_c)) as u8;
    let rem = index % (live_c * hand_c);
    let live_idx = unrank_comb(28, 4 * grade, rem / hand_c);
    let hand_pos: std::collections::BTreeSet<usize> = unrank_comb(4 * grade, grade, rem % hand_c)
        .into_iter()
        .collect();
    let (mut pool, mut hand) = (DominoSet::EMPTY, DominoSet::EMPTY);
    for (pos, di) in live_idx.iter().enumerate() {
        let d = Domino::from_index(*di).expect("domino");
        if hand_pos.contains(&pos) {
            hand.insert(d);
        } else {
            pool.insert(d);
        }
    }
    Interface {
        pip,
        hand,
        pool,
        grade,
    }
}

fn tile(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

fn record_str(r: &[Domino]) -> String {
    let ts: Vec<String> = r.iter().map(|d| tile(*d)).collect();
    ts.join(" ")
}

// -- the freeze-36(f) seed: the unmemoized argmax-recording pooled H solve --
// (`policy_inspect.rs::Ctx::solve`'s shape; count convention internally —
// SEP-A8: the argmax sets are identical under the two conventions because the
// tricks remaining are action-independent at every node. The seed contributes
// no number to any reported L: only `choices` leaves this solve.)

struct Extract<'a> {
    decl: Decl,
    focal: Seat,
    worlds: &'a [[DominoSet; 4]],
}

impl Extract<'_> {
    fn hand_now(&self, wi: u32, seat: Seat, obs: &[Domino]) -> DominoSet {
        let mut h = self.worlds[wi as usize][seat.index()];
        for t in obs {
            h.remove(*t);
        }
        h
    }

    /// Pooled H solve recording the argmax choice per observation record
    /// (least-tile tie rule, freeze 26 — cited, not re-declared). Budgeted
    /// (freeze 44(c)): one step per (particle, node) visit, charged as the
    /// support size at entry; `None` on exhaustion, nothing partial kept.
    #[allow(clippy::too_many_arguments)]
    fn solve(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &mut BTreeMap<Vec<Domino>, Domino>,
        budget: &mut u64,
    ) -> Option<Q> {
        let cost = u64::try_from(support.len()).expect("support fits");
        if *budget < cost {
            return None;
        }
        *budget -= cost;
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            let mass: Q = support
                .iter()
                .map(|&(_, den)| q(1, i128::try_from(den).expect("den fits")))
                .sum();
            let banked = if winner.team() == self.focal.team() {
                mass
            } else {
                qi(0)
            };
            if self.hand_now(support[0].0, self.focal, obs).is_empty() {
                return Some(banked);
            }
            return Some(
                banked
                    + self.solve(
                        support,
                        winner,
                        [Domino::ALL[0]; 4],
                        0,
                        obs,
                        choices,
                        budget,
                    )?,
            );
        }
        let seat = leader.plus(k);
        let led: Option<Context> = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == self.focal {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let mut best: Option<(Q, Domino)> = None;
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.solve(support, leader, tiles, k + 1, obs, choices, budget);
                obs.pop();
                let v = v?;
                if best.as_ref().map(|(bv, _)| v > *bv).unwrap_or(true) {
                    best = Some((v, a));
                }
            }
            let (v, a) = best.expect("legal move exists");
            choices.insert(obs.clone(), a);
            return Some(v);
        }
        let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
        for &(wi, den) in support {
            let hand = self.hand_now(wi, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let n = legal.len() as u128;
            for t in legal.iter() {
                by_tile.entry(t.index()).or_default().push((wi, den * n));
            }
        }
        let mut sum = qi(0);
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            let v = self.solve(&sup, leader, tiles, k + 1, obs, choices, budget);
            obs.pop();
            sum += v?;
        }
        Some(sum)
    }
}

// -- conventions ------------------------------------------------------------

/// The freeze-26 bridge at the reporting boundary: count = (diff + grade)/2.
/// Implemented once, as a function of the coordinate's DECLARED grade — no
/// grade literal appears in bridge code, and every caller asserts the grade
/// it substitutes equals the coordinate identity's grade (N4-A11).
fn to_count(diff: Q, grade: usize) -> Q {
    (diff + qi(i128::try_from(grade).expect("grade"))) * q(1, 2)
}

fn out_dir(name: &str) -> PathBuf {
    let a = PathBuf::from(format!("walt-factory/{name}"));
    if a.parent().map(|p| p.exists()).unwrap_or(false) && a.exists() {
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
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("n4") => n4_main(),
        Some("n4-rung") => n4_rung_main(),
        None => g3_main(),
        Some(other) => panic!("unknown rung argument {other:?}: expected none, n4, or n4-rung"),
    }
}

fn g3_main() {
    let t0 = Instant::now();
    let mut out = String::new();
    let mut lib = String::new();
    let dir = Direction::trick_diff();

    let _ = writeln!(
        out,
        "walt separation probe — Experiment E (decision-sparse track) — exploratory tier"
    );
    let _ = writeln!(
        out,
        "rulings: SEP-A1..SEP-A18; freezes 36, 37 (walt/CENSUS-RULINGS.md 2026-08-13); design walt/SEPARATION-PROBE.md; mathematics: errata Lemma E3, Lemma E4, Corollary E4.1, Theorem E6.4 (DS-A17 citation rule)"
    );
    let _ = writeln!(
        out,
        "regenerate: cargo run --release -p walt-factory --example separation_probe"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "PRIMAL CEILING (SEP-A2, verbatim): the primal witness at each H-optimal action is an H-optimal policy re-priced by the fixed-policy evaluator, so L = Q^H by Corollary E4.1(2); the separation verdict at this coordinate is determined entirely by the upper witness."
    );
    let _ = writeln!(
        out,
        "PROVENANCE TYPING (SEP-A12): the separation's validity does not cite H, but this run's witnesses were produced with H's help — L's seed is an H solve and receipts R1-R4 are H cross-checks; the logic of Theorem E6.4 is H-free, the provenance of these witnesses is not."
    );
    let _ = writeln!(
        out,
        "BUDGET HONESTY (freeze 44): the scalar authority is budgeted (freeze 26, {AUTHORITY_BUDGET} particle-steps) and its exhaustion is a declared stop; the walk-based evaluators carry declared budgets under freeze 44; in this run every declared budget was asserted non-binding and every residual asserted strictly positive."
    );
    let _ = writeln!(
        out,
        "SCOPE: this run is a certification probe, not a library harvest (six of the seven S6b singleton roots collapse by indifference); the n=4 rung is out of v1 scope (SEP-A10). No cost, timing, runtime or tractability claim of any kind (SEP-A15(iii)); wall-clock below is provenance only."
    );
    let _ = writeln!(
        out,
        "FENCE (R-A2, SEP-A1): the fiber is the void-free capacity fiber; its members are FEASIBLE and never reachable; no object here is identity-bearing and none asserts that any state arises in play. Relaxation naming (SEP-A6(b)): on this carrier the latent is xi = omega, so treatment C and the document's C+ coincide."
    );

    let _ = writeln!(lib, "digest={LIB_DIGEST}");
    let _ = writeln!(
        lib,
        "# candidate-policy library v1 (freeze 36, SEP-A4). A cache, never an authority (X-A17): no values, no verdicts, no ranks; a loaded entry is re-priced by policy_value before anything is reported; a digest mismatch is corruption and the file is discarded entire. Transport: identity only (SEP-A3(vi), Lemma E7)."
    );
    let _ = writeln!(
        lib,
        "# DS-A16: entries remain valid primal-witness sources under count re-entry, evaluated under the richer valuation; their count-free quality verdicts do not survive."
    );

    for (index, filed_q, filed_gap) in S6A_FILED {
        run_coordinate(&mut out, &mut lib, index, &filed_q, filed_gap, &dir);
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    let results = out_dir("results").join("separation_2026-08-13.txt");
    std::fs::write(&results, &out).expect("write results");
    let store = out_dir("store");
    std::fs::create_dir_all(&store).expect("store dir");
    std::fs::write(store.join("candidate_library.txt"), &lib).expect("write library");
    print!("{out}");
    println!("results: {}", results.display());
}

#[allow(clippy::too_many_lines)]
fn run_coordinate(
    out: &mut String,
    lib: &mut String,
    index: u128,
    filed_q: &[(u8, u8, i128, i128); 3],
    filed_gap: (i128, i128),
    dir: &Direction,
) {
    let itf = coordinate(GRADE, index);
    let kernel = itf.kernel();
    let n_worlds = kernel.count();

    // Step 1 (SEP-A14(iii)): coordinate identity first.
    assert_eq!(itf.grade, GRADE, "grade identity");
    assert_eq!(n_worlds, 1680, "the full void-free fiber, |X| = 1680 (T7)");
    assert_eq!(kernel.viewer(), Seat::S0, "viewer is the declaring leader");
    let hand: Vec<String> = itf.hand.iter().map(tile).collect();
    let _ = writeln!(out);
    let _ = writeln!(
        *out,
        "== coord idx={index} pip={} (derived, never a key component) trump={} hand=[{}] |X|={n_worlds} enumeration=freeze-7/23 ==",
        itf.pip,
        itf.pip,
        hand.join(" ")
    );

    // Step 2: the scalar H authority (freeze 26), root trick-leading.
    let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
    let auth = ScalarHidden::new(
        itf.decl(),
        Seat::S0,
        Seat::S0.team(),
        ScalarValuation::trick_only(),
    );
    let mut budget = AUTHORITY_BUDGET;
    // The root is trick-leading (empty prefix) so the authority's action list
    // is the viewer hand — the SEP-A5(i) assertion.
    let (maybe_av, stats) = auth.action_values_dag(&worlds, Seat::S0, &[], &mut budget);
    let Some(av) = maybe_av else {
        let _ = writeln!(
            *out,
            "  DECLARED STOP: authority budget {AUTHORITY_BUDGET} exceeded; correctness gate unmet — every row of this coordinate is voided (R-A18)"
        );
        return;
    };
    let _ = writeln!(
        *out,
        "  authority: gate MET (dag-v1 steps {}, boundary hits {})",
        stats.steps, stats.hits
    );

    // Step 3 + the U side: the envelope path, prices, and R1.
    let mut revealed_budget = B_WALK_4;
    let mut revealed_stop = None;
    let prices = information_prices(
        &kernel,
        Seat::S0.team(),
        dir,
        B_WALK,
        &mut revealed_budget,
        &mut revealed_stop,
    )
    .expect("freeze-44 budgets are non-binding at grade 3");
    for r in &prices.h_residuals {
        assert!(*r > 0, "R0: H residual strictly positive");
    }
    assert!(
        revealed_budget > 0,
        "R0: revealed residual strictly positive"
    );
    println!(
        "  [stdout only] freeze-44 residuals idx={index}: H {:?}, revealed {revealed_budget}",
        prices.h_residuals
    );
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    assert_eq!(
        av.len(),
        actions.len(),
        "authority covers every root action"
    );
    let mut qh_diff: Vec<Q> = Vec::new();
    let mut u_diff: Vec<Q> = Vec::new();
    for (i, a) in actions.iter().enumerate() {
        let (ha, henv) = &prices.q_h[i];
        let (ca, cenv) = &prices.q_c[i];
        let (aa, aval) = &av[i];
        assert_eq!(ha, a, "H action order is the viewer hand");
        assert_eq!(ca, a, "C action order is the viewer hand");
        assert_eq!(
            aa, a,
            "authority action order is the viewer hand (root trick-leading)"
        );
        let hd = henv.eval(qi(0));
        // R1: two independently built H solvers, same units, equal exactly,
        // NO bridge (SEP-A5, freeze 37(g)).
        assert_eq!(
            hd, *aval,
            "R1 solver identification: envelope H equals scalar authority exactly at root {a:?}"
        );
        qh_diff.push(hd);
        u_diff.push(cenv.eval(qi(0)));
    }
    let _ = writeln!(
        *out,
        "  R1 solver identification: envelope H == scalar authority, per action, exactly, no bridge — HELD ({} actions)",
        actions.len()
    );

    // Step 4, R4: the S6a cross-check across the freeze-26 bridge.
    for (i, (hi, lo, num, den)) in filed_q.iter().enumerate() {
        let filed_action = Domino::new(Pip::new(*hi).expect("pip"), Pip::new(*lo).expect("pip"));
        assert_eq!(
            filed_action, actions[i],
            "S6a filed action order matches the viewer hand"
        );
        let filed_count = q(*num, *den);
        assert_eq!(
            to_count(qh_diff[i], GRADE),
            filed_count,
            "R4 S6a cross-check: recomputed Q^H equals the filed value exactly at root {filed_action:?}"
        );
    }
    let v_h_diff = qh_diff.iter().copied().max().expect("nonempty");
    let gap_diff = prices.g_total.eval(qi(0));
    assert_eq!(
        gap_diff * q(1, 2),
        q(filed_gap.0, filed_gap.1),
        "R4 S6a cross-check: recomputed aggregate gap V^F - V^H equals the filed fusion_gap exactly"
    );
    let _ = writeln!(
        *out,
        "  R4 S6a cross-check: Q^H per action and aggregate gap equal the filed S6a values exactly (frozen table, count convention, freeze-26 bridge) — HELD"
    );
    let _ = writeln!(
        *out,
        "  aggregate gap V^F - V^H = {} (count convention). ONE-SIDED SCREEN (SEP-A15(i)): this column licenses only Corollary E3.2's zero case; when nonzero, U_a <= V^H + gap never establishes U_a <= V^H, and gap-versus-headroom comparisons are not evidence in either direction.",
        gap_diff * q(1, 2)
    );

    // The H-optimal set.
    let argmax: Vec<usize> = (0..actions.len())
        .filter(|&i| qh_diff[i] == v_h_diff)
        .collect();
    let _ = writeln!(
        *out,
        "  V^H = {} (count); H-optimal set {{{}}}",
        to_count(v_h_diff, GRADE),
        argmax
            .iter()
            .map(|&i| tile(actions[i]))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Step 7, R3 — the measurement: the per-action price, every action.
    for (i, a) in actions.iter().enumerate() {
        let price_diff = prices.g_cont_by_root[i].1.eval(qi(0));
        assert_eq!(price_diff, u_diff[i] - qh_diff[i], "price is U - Q^H");
        assert!(price_diff >= qi(0), "Lemma E3: U_a >= Q^H(a)");
        let _ = writeln!(
            *out,
            "  R3 measurement root {}: Q^H = {}  U = {}  price U - Q^H = {}  (count convention)",
            tile(*a),
            to_count(qh_diff[i], GRADE),
            to_count(u_diff[i], GRADE),
            price_diff * q(1, 2)
        );
    }

    // Steps 5-6 per H-optimal action: extraction, library entry, L, R2, R5.
    let mut l_diff: BTreeMap<usize, Q> = BTreeMap::new();
    for &ai in &argmax {
        let a = actions[ai];
        let extract = Extract {
            decl: itf.decl(),
            focal: Seat::S0,
            worlds: &worlds,
        };
        let support: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|i| (i, 1)).collect();
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = a;
        let mut obs = vec![a];
        let mut choices: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
        let mut eb = B_WALK;
        let _ = extract
            .solve(
                &support,
                Seat::S0,
                tiles,
                1,
                &mut obs,
                &mut choices,
                &mut eb,
            )
            .expect("freeze-44 budget non-binding at grade 3");
        assert!(eb > 0, "R0: extraction residual strictly positive");

        let mut pb = B_WALK;
        let mut cap_hit = false;
        let partition = InfoPartition::build(&kernel, a, &mut pb, P_MAX, &mut cap_hit)
            .expect("freeze-44 budget non-binding at grade 3");
        assert!(
            !cap_hit && pb > 0,
            "R0: partition residual strictly positive"
        );
        // SEP-A11: totality before pricing; a failure names its defect.
        assert_eq!(
            choices.len(),
            partition.len(),
            "SEP-A11 stop-and-report at root {a:?}: extraction state count differs from InfoPartition — the extraction did not produce an argmax at every state, the extraction's partition disagrees with InfoPartition::build, or the two H authorities disagree"
        );
        let mut by_id: BTreeMap<walt_strat::InfoStateId, Domino> = BTreeMap::new();
        for (record, chosen) in &choices {
            let id = partition.id(record).expect(
                "SEP-A11 stop-and-report: an extracted record is not an InfoPartition state",
            );
            by_id.insert(id, *chosen);
        }
        assert_eq!(by_id.len(), partition.len(), "the choice map is total");
        let policy = Policy::build(&partition, |id, _legal| {
            *by_id.get(&id).expect("total by the assertion above")
        });

        // Freeze 36 entry: key, frame, sorted records; no values, no verdicts.
        let _ = writeln!(*lib);
        let _ = writeln!(
            *lib,
            "entry grade={GRADE} index={index} decl=PipTrump({}) root={} pip-derived={} X={n_worlds}",
            itf.pip,
            tile(a),
            itf.pip
        );
        for (record, chosen) in &choices {
            let _ = writeln!(*lib, "  {} -> {}", record_str(record), tile(*chosen));
        }
        let _ = writeln!(*lib, "end");

        // R2 + R5: price the candidate through the structurally max-free path.
        let mut lb = B_WALK;
        let (line, receipt) =
            policy_value_receipt(&kernel, Seat::S0.team(), dir, &partition, &policy, &mut lb)
                .expect("freeze-44 budget non-binding at grade 3");
        assert!(lb > 0, "R0: L-walk residual strictly positive");
        let ld = line.eval(qi(0));
        assert_eq!(
            ld, qh_diff[ai],
            "R2 primal receipt (Corollary E4.1(2)): L = Q^H exactly at root {a:?}; a strict inequality is a pipeline defect (SEP-A11), never a finding"
        );
        assert_eq!(
            receipt.focal_states, receipt.singleton_expansions,
            "R5: every focal expansion was a singleton"
        );
        assert_eq!(
            receipt.focal_states, receipt.distinct_states,
            "R5: no state visited twice and every visited record is a partition state"
        );
        l_diff.insert(ai, ld);
        let _ = writeln!(
            *out,
            "  root {}: candidate extracted ({} states, {} with genuine choice; freeze-26 tie rule); library entry written; R2 primal receipt L = Q^H = {} — HELD; R5 max-freedom counted receipt: {} focal callback invocations = {} singleton expansions = {} distinct partition states reached (of {} in the partition; a fixed policy prunes counterfactual branches) — HELD",
            tile(a),
            partition.len(),
            partition.choice_states(),
            to_count(ld, GRADE),
            receipt.focal_states,
            receipt.singleton_expansions,
            receipt.distinct_states,
            partition.len()
        );
        let _ = writeln!(
            *out,
            "    SEP-A19(b) typing: reached {} of partition {} — the reached count is an exact computational observable of the EXHIBITED witness, tie-break-relative to freeze 26, never a term in the DS-A2 ladder; the partition count is the tie-break-free quantity E_B(a) (§10.9). Their ratio is not a measurement and is not printed.",
            receipt.distinct_states,
            partition.len()
        );
    }

    // Step 8: the separation table.
    let mut coordinate_separated = false;
    for &ai in &argmax {
        let a_star = actions[ai];
        let l = l_diff[&ai];
        let mut all = true;
        for (i, a) in actions.iter().enumerate() {
            if i == ai {
                continue;
            }
            let margin = l - u_diff[i];
            let verdict = if margin >= qi(0) {
                "SEPARATED"
            } else {
                all = false;
                "NOT SEPARATED"
            };
            let _ = writeln!(
                *out,
                "  pair (a*={}, a={}): L = {}  U = {}  margin L - U = {}  -> {}",
                tile(a_star),
                tile(*a),
                to_count(l, GRADE),
                to_count(u_diff[i], GRADE),
                margin * q(1, 2),
                verdict
            );
            if margin < qi(0) {
                let _ = writeln!(
                    *out,
                    "    EXACT NEGATIVE (Corollary E4.1(3)): Q^H({}) < U_{} by {} (count); no candidate policy set whatsoever separates this pair under relaxation C at this coordinate — the remaining lever is a gluing cut (Theorem E6.5, DS-A3), never a better candidate.",
                    tile(a_star),
                    tile(*a),
                    (u_diff[i] - l) * q(1, 2)
                );
            }
        }
        if all {
            coordinate_separated = true;
            let _ = writeln!(
                *out,
                "  VERDICT: root action {} SEPARATED against every competitor — {} is in Opt^H at this coordinate. Receipt: the action lies in H's argmax (asserted). MEMBER-NOT-SET (Theorem E6.4, verbatim): non-strict separation certifies membership in the optimal set and never uniqueness.",
                tile(a_star),
                tile(a_star)
            );
            assert!(
                qh_diff[ai] == v_h_diff,
                "the separated action lies in H's argmax"
            );
        }
    }
    if !coordinate_separated {
        let _ = writeln!(
            *out,
            "  VERDICT: coordinate NOT SEPARATED — no H-optimal action clears every competitor's upper witness."
        );
        if argmax.len() > 1 {
            let _ = writeln!(
                *out,
                "  Reported as treatment H's fact, never the witnesses' (SEP-A9): the set {{{}}} exhausts Opt^H at this coordinate per H.",
                argmax
                    .iter()
                    .map(|&i| tile(actions[i]))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    if index == 0 {
        let _ = writeln!(
            *out,
            "  THESIS SCOPE (SEP-A15(ii)), two sentences, neither stands for the other: (1) a SEPARATED verdict here demonstrates the Pareto frontier at the competing leads — which S6b could not complete (PG-A13, cap 16384) — is not needed for the root decision. (2) This run does NOT test the parent's economy claim [\"the solver does not need an exact solution for every action\"]: it computes the exact H solve at every action because DS-A10's receipts require it; the economy experiment is SEP-A17's successor."
        );
    }
}

// ======================= the n = 4 separation rung =========================
// Design walt/SEPARATION-RUNG-N4.md as amended by N4-A1..N4-A12; freezes 44
// (walk-step unit, budgets, constants, canonical unit order, fallback rule)
// and 45 (content-based coordinate identity; corpus handle provenance only;
// no library entry at n = 4). Checkpointed per DS-A29..DS-A36 at
// (coordinate, action) granularity. Exploratory tier throughout.

const N4_GRADE: usize = 4;
const N4_TRICK: usize = 4;
/// Freeze 44(e): the §5 rung's decimation prime (fresh; deliberately not a
/// freeze-25 constant — the no-cross-wiring clause).
const N4_G: u128 = 15_485_863;
const N4_FIBER: u128 = 34_650;
const N4_CKPT_DIGEST: &str =
    "N4-ckpt-v1|freezes-26-36f-37-44-45|unit=hand+action|fields=unit-v1|R0=blocking";

const N4_IN_SCOPE: [usize; 9] = [0, 1, 2, 4, 5, 6, 8, 9, 12];
#[allow(dead_code)] // printed in the header text; the loop never visits them
const N4_OUT_OF_SCOPE: [usize; 4] = [3, 7, 10, 11];

/// (R6) frozen source: the S5h dag-v1 step and boundary-hit counts per hand,
/// quoted from `fiber_probe_h_2026-08-11.txt` (exploratory tier). A
/// determinism check on the unchanged scalar path and on DS-A29(a)-(b)
/// (N4-A7), never a check on any value; an undeclared mismatch is
/// stop-and-report.
const S5H_DAG: [(usize, u64, u64); 9] = [
    (0, 140_226_166, 14_809_754),
    (1, 123_882_398, 6_756_331),
    (2, 78_359_234, 7_668_849),
    (4, 123_162_862, 11_613_188),
    (5, 191_841_542, 19_118_225),
    (6, 145_749_144, 19_087_948),
    (8, 136_677_010, 15_725_964),
    (9, 177_671_552, 13_256_467),
    (12, 176_464_986, 17_140_506),
];

/// §2.2: the void-filtered fiber sizes quoted from the S5h P-A2 receipt,
/// recomputed in-run and asserted equal. Provenance only; licenses nothing.
const S5H_VOIDED: [(usize, u128); 9] = [
    (0, 34_650),
    (1, 34_650),
    (2, 23_100),
    (4, 34_650),
    (5, 14_700),
    (6, 34_650),
    (8, 1_200),
    (9, 34_650),
    (12, 34_650),
];

fn n4_receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

/// The void-free capacity kernel (P-A1/P-A4): focal = the declaring seat,
/// voids deliberately EMPTY. The same construction as
/// `fiber_probe.rs::void_free_kernel`.
fn n4_void_free_kernel(hand: &ReceiptHand) -> (Kernel, Seat) {
    let (hands, leader) = state_before_trick(hand, N4_TRICK).expect("the receipt replays");
    let focal = hand.bidder;
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    let mut pool = DominoSet::EMPTY;
    for (slot, k) in hidden.iter_mut().zip(1..=3) {
        let seat = focal.plus(k);
        *slot = Hidden {
            seat,
            capacity: hands[seat.index()].len(),
            voids: ContextSet::EMPTY,
        };
        pool = pool.union(hands[seat.index()]);
    }
    let kernel = Kernel::new(hand.decl, focal, hands[focal.index()], pool, hidden)
        .expect("the void-free capacity kernel is well formed");
    (kernel, leader)
}

/// The void-filtered count (P-A2), recomputed for the provenance column.
fn n4_voided_count(hand: &ReceiptHand) -> u128 {
    let (hands, _) = state_before_trick(hand, N4_TRICK).expect("the receipt replays");
    let voids = voids_before_trick(hand, N4_TRICK);
    let focal = hand.bidder;
    let mut hidden = [Hidden {
        seat: focal,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; HIDDEN_SEATS];
    let mut pool = DominoSet::EMPTY;
    for (slot, k) in hidden.iter_mut().zip(1..=3) {
        let seat = focal.plus(k);
        *slot = Hidden {
            seat,
            capacity: hands[seat.index()].len(),
            voids: voids[seat.index()],
        };
        pool = pool.union(hands[seat.index()]);
    }
    Kernel::new(hand.decl, focal, hands[focal.index()], pool, hidden)
        .expect("the voided kernel is well formed")
        .count()
}

/// One (coordinate, action) unit record: every non-timing field the
/// deterministic block needs. `status` is "ok" or a declared-stop marker;
/// value fields are meaningful only when "ok".
#[derive(Clone, PartialEq, Eq, Debug)]
struct UnitRec {
    hand: usize,
    action: Domino,
    gate_met: bool,
    auth_steps: u64,
    auth_hits: u64,
    status: String,
    qh: Q,
    u: Q,
    optimal: bool,
    l: Option<Q>,
    partition: usize,
    choice_states: usize,
    r5: (u64, u64, u64),
    h_residual: u64,
    revealed_steps: u64,
    extract_residual: u64,
    l_residual: u64,
    part_residual: u64,
}

fn q_ser(x: Q) -> String {
    format!("{}/{}", x.numer(), x.denom())
}

fn q_de(s: &str) -> Q {
    let (n, d) = s.split_once('/').expect("num/den");
    q(
        n.parse::<i128>().expect("numerator"),
        d.parse::<i128>().expect("denominator"),
    )
}

impl UnitRec {
    fn ser(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "digest={N4_CKPT_DIGEST}");
        let _ = writeln!(s, "hand={}", self.hand);
        let _ = writeln!(s, "action={}", tile(self.action));
        let _ = writeln!(s, "gate_met={}", self.gate_met);
        let _ = writeln!(s, "auth_steps={}", self.auth_steps);
        let _ = writeln!(s, "auth_hits={}", self.auth_hits);
        let _ = writeln!(s, "status={}", self.status);
        let _ = writeln!(s, "qh={}", q_ser(self.qh));
        let _ = writeln!(s, "u={}", q_ser(self.u));
        let _ = writeln!(s, "optimal={}", self.optimal);
        let _ = writeln!(
            s,
            "l={}",
            self.l.map(q_ser).unwrap_or_else(|| "none".to_owned())
        );
        let _ = writeln!(s, "partition={}", self.partition);
        let _ = writeln!(s, "choice_states={}", self.choice_states);
        let _ = writeln!(s, "r5={}/{}/{}", self.r5.0, self.r5.1, self.r5.2);
        let _ = writeln!(s, "h_residual={}", self.h_residual);
        let _ = writeln!(s, "revealed_steps={}", self.revealed_steps);
        let _ = writeln!(s, "extract_residual={}", self.extract_residual);
        let _ = writeln!(s, "l_residual={}", self.l_residual);
        let _ = writeln!(s, "part_residual={}", self.part_residual);
        let _ = writeln!(s, "complete=yes");
        s
    }

    fn de(text: &str) -> Option<UnitRec> {
        let mut m: BTreeMap<&str, &str> = BTreeMap::new();
        for line in text.lines() {
            let (k, v) = line.split_once('=')?;
            m.insert(k, v);
        }
        if m.get("complete") != Some(&"yes") || m.get("digest") != Some(&N4_CKPT_DIGEST) {
            return None;
        }
        let action_s = m.get("action")?;
        let hi: u8 = action_s[0..1].parse().ok()?;
        let lo: u8 = action_s[1..2].parse().ok()?;
        let r5s = m.get("r5")?;
        let mut r5p = r5s.split('/');
        Some(UnitRec {
            hand: m.get("hand")?.parse().ok()?,
            action: Domino::new(Pip::new(hi)?, Pip::new(lo)?),
            gate_met: m.get("gate_met")?.parse().ok()?,
            auth_steps: m.get("auth_steps")?.parse().ok()?,
            auth_hits: m.get("auth_hits")?.parse().ok()?,
            status: (*m.get("status")?).to_owned(),
            qh: q_de(m.get("qh")?),
            u: q_de(m.get("u")?),
            optimal: m.get("optimal")?.parse().ok()?,
            l: match *m.get("l")? {
                "none" => None,
                s => Some(q_de(s)),
            },
            partition: m.get("partition")?.parse().ok()?,
            choice_states: m.get("choice_states")?.parse().ok()?,
            r5: (
                r5p.next()?.parse().ok()?,
                r5p.next()?.parse().ok()?,
                r5p.next()?.parse().ok()?,
            ),
            h_residual: m.get("h_residual")?.parse().ok()?,
            revealed_steps: m.get("revealed_steps")?.parse().ok()?,
            extract_residual: m.get("extract_residual")?.parse().ok()?,
            l_residual: m.get("l_residual")?.parse().ok()?,
            part_residual: m.get("part_residual")?.parse().ok()?,
        })
    }
}

fn n4_ckpt_dir() -> PathBuf {
    out_dir("store").join("separation_n4_ckpt")
}

/// Freeze-41 discipline: any record failing digest or parse makes the cache
/// CORRUPT, not stale — discarded entire.
fn n4_validate_cache() -> BTreeMap<(usize, usize), UnitRec> {
    let dir = n4_ckpt_dir();
    let mut loaded = BTreeMap::new();
    let Ok(rd) = std::fs::read_dir(&dir) else {
        return loaded;
    };
    let mut corrupt = false;
    for e in rd.flatten() {
        let path = e.path();
        if path.extension().map(|x| x == "tmp").unwrap_or(false) {
            corrupt = true;
            break;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            corrupt = true;
            break;
        };
        match UnitRec::de(&text) {
            Some(rec) => {
                let ai = rec.action.index();
                loaded.insert((rec.hand, ai), rec);
            }
            None => {
                corrupt = true;
                break;
            }
        }
    }
    if corrupt {
        let _ = std::fs::remove_dir_all(&dir);
        eprintln!("n4 checkpoint cache CORRUPT (digest or parse) — discarded entire (DS-A30)");
        return BTreeMap::new();
    }
    // N4-A9(i): all loaded unit records of one coordinate must carry the same
    // gate outcome; a disagreement is corruption and the cache is discarded
    // entire.
    let mut gate_by_hand: BTreeMap<usize, bool> = BTreeMap::new();
    for rec in loaded.values() {
        match gate_by_hand.get(&rec.hand) {
            None => {
                gate_by_hand.insert(rec.hand, rec.gate_met);
            }
            Some(g) if *g == rec.gate_met => {}
            Some(_) => {
                let _ = std::fs::remove_dir_all(&dir);
                eprintln!("n4 checkpoint cache CORRUPT (gate-outcome disagreement within a coordinate) — discarded entire (N4-A9(i))");
                return BTreeMap::new();
            }
        }
    }
    loaded
}

fn n4_save_unit(rec: &UnitRec) {
    let dir = n4_ckpt_dir();
    std::fs::create_dir_all(&dir).expect("ckpt dir");
    let name = format!("h{}_a{}.txt", rec.hand, tile(rec.action));
    let tmp = dir.join(format!("{name}.tmp"));
    let fin = dir.join(name);
    std::fs::write(&tmp, rec.ser()).expect("write tmp");
    std::fs::rename(&tmp, &fin).expect("atomic rename");
}

/// Compute one coordinate's four unit records (the expensive path). The
/// revealed and authority calls span the coordinate's four units in one call
/// each; the units carry the coordinate-level gate outcome denormalised.
fn n4_compute_coordinate(hand_idx: usize, hand: &ReceiptHand) -> Vec<UnitRec> {
    let (kernel, leader) = n4_void_free_kernel(hand);
    assert_eq!(leader, kernel.viewer(), "leader offset from focal is 0");
    assert_eq!(kernel.count(), N4_FIBER, "|X| = 34,650");
    let grade = kernel.viewer_hand().len();
    assert_eq!(
        grade, N4_GRADE,
        "grade 4 (N4-A11: asserted, and the bridge is a function of it)"
    );
    let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();

    // The scalar authority (freeze 26, unchanged).
    let auth = ScalarHidden::new(
        kernel.decl(),
        kernel.viewer(),
        kernel.viewer().team(),
        ScalarValuation::trick_only(),
    );
    let mut ab = AUTHORITY_BUDGET;
    let (maybe_av, stats) = auth.action_values_dag(&worlds, kernel.viewer(), &[], &mut ab);
    let gate_met = maybe_av.is_some();
    if gate_met {
        // (R6) step determinism against the quoted S5h counts — the scalar
        // path is unchanged since that receipt; an undeclared mismatch is a
        // load-invariance failure, stop-and-report (N4-A7).
        let (_, s5h_steps, s5h_hits) = S5H_DAG
            .iter()
            .find(|(h, _, _)| *h == hand_idx)
            .expect("in-scope hand");
        assert_eq!(
            stats.steps, *s5h_steps,
            "R6 stop-and-report: dag-v1 steps moved without a declared cause (DS-A29)"
        );
        assert_eq!(
            stats.hits, *s5h_hits,
            "R6 stop-and-report: boundary hits moved without a declared cause (DS-A29)"
        );
    }

    // The envelope side: H per action and the revealed pass, budgeted.
    let dir = Direction::trick_diff();
    let mut revealed_budget = 4 * B_WALK;
    let mut revealed_stop = None;
    let prices = information_prices(
        &kernel,
        kernel.viewer().team(),
        &dir,
        B_WALK,
        &mut revealed_budget,
        &mut revealed_stop,
    );
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();

    let Some(prices) = prices else {
        // Tier 3 — WITNESS STOP at the coordinate level: no U (or no
        // envelope H) for any action, so no verdict of either kind
        // (revealed U's complete all-or-none here; PG-A13).
        let what = match revealed_stop {
            Some(rs) => format!(
                "stop:revealed@action={},world={},charged={}",
                tile(rs.action),
                rs.world_index,
                4 * B_WALK - revealed_budget
            ),
            None => "stop:envelope-h".to_owned(),
        };
        return actions
            .iter()
            .map(|a| UnitRec {
                hand: hand_idx,
                action: *a,
                gate_met,
                auth_steps: stats.steps,
                auth_hits: stats.hits,
                status: what.clone(),
                qh: qi(0),
                u: qi(0),
                optimal: false,
                l: None,
                partition: 0,
                choice_states: 0,
                r5: (0, 0, 0),
                h_residual: 0,
                revealed_steps: 0,
                extract_residual: 0,
                l_residual: 0,
                part_residual: 0,
            })
            .collect();
    };

    // R1 at Tier 1 only (N4-A6: at Tier 2 the envelope path is the sole H
    // authority and the rows say so).
    if let Some(av) = &maybe_av {
        assert_eq!(av.len(), actions.len(), "authority covers every action");
        for (i, a) in actions.iter().enumerate() {
            let (ha, henv) = &prices.q_h[i];
            assert_eq!(ha, a, "H action order");
            assert_eq!(av[i].0, *a, "authority action order (root trick-leading)");
            assert_eq!(
                henv.eval(qi(0)),
                av[i].1,
                "R1 solver identification: envelope H equals scalar authority exactly at {a:?}"
            );
        }
    }

    let qh: Vec<Q> = prices.q_h.iter().map(|(_, e)| e.eval(qi(0))).collect();
    let u: Vec<Q> = prices.q_c.iter().map(|(_, e)| e.eval(qi(0))).collect();
    for (i, _) in actions.iter().enumerate() {
        let price = prices.g_cont_by_root[i].1.eval(qi(0));
        assert!(price >= qi(0), "Lemma E3: U_a >= Q^H(a)");
        assert_eq!(price, u[i] - qh[i], "price is U - Q^H");
    }
    let vh = qh.iter().copied().max().expect("nonempty");

    let mut recs = Vec::new();
    for (i, a) in actions.iter().enumerate() {
        let optimal = qh[i] == vh;
        let (l, partition_len, choice_states, r5, er, lr, pr) = if optimal {
            // The primal pipeline at a⋆: partition, extraction (freeze
            // 36(f)), the L walk. No library entry is written (freeze 45).
            let mut pb = B_WALK;
            let mut cap_hit = false;
            let partition = InfoPartition::build(&kernel, *a, &mut pb, P_MAX, &mut cap_hit);
            let Some(partition) = partition else {
                let cause = if cap_hit {
                    "stop:p-max"
                } else {
                    "stop:partition-budget"
                };
                recs.push(UnitRec {
                    hand: hand_idx,
                    action: *a,
                    gate_met,
                    auth_steps: stats.steps,
                    auth_hits: stats.hits,
                    status: cause.to_owned(),
                    qh: qh[i],
                    u: u[i],
                    optimal,
                    l: None,
                    partition: 0,
                    choice_states: 0,
                    r5: (0, 0, 0),
                    h_residual: prices.h_residuals[i],
                    revealed_steps: prices.revealed_action_steps[i].1,
                    extract_residual: 0,
                    l_residual: 0,
                    part_residual: pb,
                });
                continue;
            };
            let extract = Extract {
                decl: kernel.decl(),
                focal: kernel.viewer(),
                worlds: &worlds,
            };
            let support: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|w| (w, 1)).collect();
            let mut tiles = [Domino::ALL[0]; 4];
            tiles[0] = *a;
            let mut obs = vec![*a];
            let mut choices: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
            let mut eb = B_WALK;
            let solved = extract.solve(
                &support,
                kernel.viewer(),
                tiles,
                1,
                &mut obs,
                &mut choices,
                &mut eb,
            );
            if solved.is_none() {
                recs.push(UnitRec {
                    hand: hand_idx,
                    action: *a,
                    gate_met,
                    auth_steps: stats.steps,
                    auth_hits: stats.hits,
                    status: "stop:extraction-budget".to_owned(),
                    qh: qh[i],
                    u: u[i],
                    optimal,
                    l: None,
                    partition: partition.len(),
                    choice_states: partition.choice_states(),
                    r5: (0, 0, 0),
                    h_residual: prices.h_residuals[i],
                    revealed_steps: prices.revealed_action_steps[i].1,
                    extract_residual: eb,
                    l_residual: 0,
                    part_residual: pb,
                });
                continue;
            }
            assert_eq!(
                choices.len(),
                partition.len(),
                "SEP-A11 stop-and-report: extraction state count differs from InfoPartition"
            );
            let mut by_id: BTreeMap<walt_strat::InfoStateId, Domino> = BTreeMap::new();
            for (record, chosen) in &choices {
                let id = partition.id(record).expect(
                    "SEP-A11 stop-and-report: an extracted record is not a partition state",
                );
                by_id.insert(id, *chosen);
            }
            let policy = Policy::build(&partition, |id, _legal| {
                *by_id.get(&id).expect("total by the assertion above")
            });
            let mut lb = B_WALK;
            let priced = policy_value_receipt(
                &kernel,
                kernel.viewer().team(),
                &dir,
                &partition,
                &policy,
                &mut lb,
            );
            let Some((line, receipt)) = priced else {
                recs.push(UnitRec {
                    hand: hand_idx,
                    action: *a,
                    gate_met,
                    auth_steps: stats.steps,
                    auth_hits: stats.hits,
                    status: "stop:l-walk-budget".to_owned(),
                    qh: qh[i],
                    u: u[i],
                    optimal,
                    l: None,
                    partition: partition.len(),
                    choice_states: partition.choice_states(),
                    r5: (0, 0, 0),
                    h_residual: prices.h_residuals[i],
                    revealed_steps: prices.revealed_action_steps[i].1,
                    extract_residual: eb,
                    l_residual: lb,
                    part_residual: pb,
                });
                continue;
            };
            let ld = line.eval(qi(0));
            // (R2): at Tier 1 this ties three code paths; at Tier 2 the
            // equality is asserted against the ENVELOPE H only, the sole
            // authority at this coordinate (N4-A6(i)).
            assert_eq!(
                ld, qh[i],
                "R2 stop-and-report (Corollary E4.1(2)): L = Q^H exactly; a strict inequality is a pipeline defect (SEP-A11)"
            );
            assert_eq!(receipt.focal_states, receipt.singleton_expansions, "R5");
            assert_eq!(receipt.focal_states, receipt.distinct_states, "R5");
            (
                Some(ld),
                partition.len(),
                partition.choice_states(),
                (
                    receipt.focal_states,
                    receipt.singleton_expansions,
                    receipt.distinct_states,
                ),
                eb,
                lb,
                pb,
            )
        } else {
            (None, 0, 0, (0, 0, 0), 0, 0, 0)
        };
        recs.push(UnitRec {
            hand: hand_idx,
            action: *a,
            gate_met,
            auth_steps: stats.steps,
            auth_hits: stats.hits,
            status: "ok".to_owned(),
            qh: qh[i],
            u: u[i],
            optimal,
            l,
            partition: partition_len,
            choice_states,
            r5,
            h_residual: prices.h_residuals[i],
            revealed_steps: prices.revealed_action_steps[i].1,
            extract_residual: er,
            l_residual: lr,
            part_residual: pr,
        });
    }
    recs
}

/// Render one coordinate's deterministic block from its unit records plus
/// cheap recomputed identity data. Fresh and resumed runs produce this text
/// byte-identically (DS-A36).
fn n4_render_coordinate(hand_idx: usize, hand: &ReceiptHand, recs: &[UnitRec], out: &mut String) {
    let (kernel, _leader) = n4_void_free_kernel(hand);
    let grade = kernel.viewer_hand().len();
    assert_eq!(grade, N4_GRADE, "grade identity");
    // Freeze 45: the content-based identity, printed and asserted; the
    // kernel is rebuilt from the printed identity and asserted equal.
    let hand_tiles: Vec<String> = kernel.viewer_hand().iter().map(tile).collect();
    let pool_tiles: Vec<String> = kernel.pool().iter().map(tile).collect();
    let Decl::PipTrump(p) = kernel.decl() else {
        panic!("pip-trump only (F1)")
    };
    let rebuilt = {
        let focal = kernel.viewer();
        let mut hidden = [Hidden {
            seat: focal,
            capacity: 0,
            voids: ContextSet::EMPTY,
        }; HIDDEN_SEATS];
        let mut remaining = kernel.pool();
        // Capacities are recovered from the receipt state per freeze 45; the
        // rebuild asserts identity of every component.
        let (hands, _) = state_before_trick(hand, N4_TRICK).expect("replays");
        for (slot, k) in hidden.iter_mut().zip(1..=3) {
            let seat = focal.plus(k);
            *slot = Hidden {
                seat,
                capacity: hands[seat.index()].len(),
                voids: ContextSet::EMPTY,
            };
            remaining = remaining.union(hands[seat.index()]);
        }
        Kernel::new(
            kernel.decl(),
            focal,
            kernel.viewer_hand(),
            kernel.pool(),
            hidden,
        )
        .expect("rebuild")
    };
    assert_eq!(
        rebuilt, kernel,
        "freeze 45: kernel rebuilt from printed identity equals void_free_kernel's"
    );
    let voided = n4_voided_count(hand);
    let quoted = S5H_VOIDED
        .iter()
        .find(|(h, _)| *h == hand_idx)
        .expect("in-scope")
        .1;
    assert_eq!(
        voided, quoted,
        "P-A2 void-filtered size equals the quoted S5h receipt value"
    );
    let fence_marked = matches!(hand_idx, 2 | 5 | 8);

    let _ = writeln!(*out);
    let _ = writeln!(
        *out,
        "== n4 coord grade={N4_GRADE} pip={} hand=[{}] pool=[{}] leader-offset=0 |X|={N4_FIBER} enumeration=freeze-7/23 ==",
        p.value(),
        hand_tiles.join(" "),
        pool_tiles.join(" ")
    );
    let _ = writeln!(
        *out,
        "  provenance only: corpus hand id {hand_idx}, trick {N4_TRICK} (never identity components, freeze 45)"
    );
    let _ = writeln!(
        *out,
        "  void-filtered fiber {voided} of {N4_FIBER} — this column licenses nothing: it is not a belief, not a weight, not an error bar, and no verdict in this file is conditioned on it."
    );
    let _ = writeln!(
        *out,
        "  REAL-DEAL FENCE (§2.2): the hand and pool of this coordinate are taken from a real deal in rob's receipt corpus; its belief is not. The voids the play record had already revealed are deliberately discarded (P-A2's void-free carrier), and support is not belief in any case. No row in this file is a statement about correct play in that deal, about reachability, or about any belief other than the declared one. Fiber members are FEASIBLE and never reachable (R-A2, P-A1)."
    );

    let gate_met = recs[0].gate_met;
    let tier = if !recs[0].status.starts_with("ok")
        && recs.iter().all(|r| r.status.starts_with("stop:"))
    {
        3
    } else if gate_met {
        1
    } else {
        2
    };
    if gate_met {
        let _ = writeln!(
            *out,
            "  authority: gate MET (dag-v1 steps {}, boundary hits {}); R6 step-determinism vs the quoted S5h counts — HELD",
            recs[0].auth_steps, recs[0].auth_hits
        );
        let _ = writeln!(
            *out,
            "  R1 solver identification: envelope H == scalar authority, per action, exactly, no bridge — HELD (TIER 1)"
        );
    } else {
        let _ = writeln!(
            *out,
            "  correctness gate unmet — authority budget {AUTHORITY_BUDGET} exhausted (steps charged {}); every row of this coordinate carries TIER 2 language (R1 unavailable; the envelope path is the sole H authority here and a defect in it would be invisible)",
            recs[0].auth_steps
        );
    }

    if tier == 3 {
        let _ = writeln!(
            *out,
            "  WITNESS STOP ({}): no U for any action, hence no verdict of either kind at this coordinate — a stop can complete a negative and can never complete a positive, and there is no completed pair here (PG-A13, N4-A6(ii)). Budgets: B = {B_WALK} per (coordinate, action), 4B whole-call revealed.",
            recs[0].status
        );
        return;
    }

    let vh = recs.iter().map(|r| r.qh).max().expect("nonempty");
    for r in recs {
        let _ = writeln!(
            *out,
            "  R3 root {}: Q^H = {}  U = {}  price U - Q^H = {}  (count; grade {N4_GRADE} bridge asserted)  [revealed walk-steps {} — a SEP-A19(b)-class observable of the declared traversal, never a cost claim]",
            tile(r.action),
            to_count(r.qh, N4_GRADE),
            to_count(r.u, N4_GRADE),
            (r.u - r.qh) * q(1, 2),
            r.revealed_steps
        );
    }
    for r in recs.iter().filter(|r| r.optimal) {
        if r.status.starts_with("stop:") {
            let _ = writeln!(
                *out,
                "  a⋆ = {}: PRIMAL PIPELINE STOP ({}) — no L, hence no verdict for this a⋆ in either direction (N4-A6(ii)).",
                tile(r.action),
                r.status
            );
            continue;
        }
        let l = r.l.expect("ok optimal row has L");
        let _ = writeln!(
            *out,
            "  a⋆ = {}: candidate extracted ({} states, {} with genuine choice; freeze-26 tie rule; no library entry, freeze 45); R2 primal receipt L = Q^H = {} — HELD{}; R5 counted receipt: {} = {} = {} distinct states reached (of {} in the partition). SEP-A19(b): the reached count is an observable of the EXHIBITED witness, tie-break-relative to freeze 26, never a DS-A2 term; the partition count is the tie-break-free E_B(a). No ratio is printed.",
            tile(r.action),
            r.partition,
            r.choice_states,
            to_count(l, N4_GRADE),
            if gate_met {
                ""
            } else {
                " (TIER 2: asserted against the ENVELOPE H only, the sole authority at this coordinate)"
            },
            r.r5.0,
            r.r5.1,
            r.r5.2,
            r.partition
        );
        let mut all = true;
        for c in recs.iter().filter(|c| c.action != r.action) {
            let margin = l - c.u;
            let sep = margin >= qi(0);
            if !sep {
                all = false;
            }
            let _ = writeln!(
                *out,
                "  pair (a*={}, a={}): L = {}  U = {}  margin = {}  -> {}{}",
                tile(r.action),
                tile(c.action),
                to_count(l, N4_GRADE),
                to_count(c.u, N4_GRADE),
                margin * q(1, 2),
                if sep { "SEPARATED" } else { "NOT SEPARATED" },
                if fence_marked {
                    format!(
                        " [real-deal fence applies: void-filtered fiber {voided} of {N4_FIBER}]"
                    )
                } else {
                    String::new()
                }
            );
            if !sep {
                let _ = writeln!(
                    *out,
                    "    EXACT NEGATIVE (Corollary E4.1(3)): Q^H({}) < U_{} by {} (count); no candidate policy set whatsoever separates this pair under relaxation C at this coordinate — the remaining lever is a gluing cut (Theorem E6.5). This failing pair is an input Experiment D needs.{}",
                    tile(r.action),
                    tile(c.action),
                    (c.u - l) * q(1, 2),
                    if gate_met {
                        String::new()
                    } else {
                        " (TIER 2: the provenance of this exact negative is a single uncrosschecked H solve.)".to_owned()
                    }
                );
            }
        }
        if all {
            let _ = writeln!(
                *out,
                "  VERDICT: root action {} SEPARATED against every competitor — in Opt^H at this coordinate (asserted ∈ argmax_H). MEMBER-NOT-SET (Theorem E6.4, verbatim): non-strict separation certifies membership in the optimal set and never uniqueness.{}{}",
                tile(r.action),
                if fence_marked {
                    format!(" [real-deal fence applies: void-filtered fiber {voided} of {N4_FIBER}]")
                } else {
                    String::new()
                },
                if gate_met {
                    String::new()
                } else {
                    " VERDICT UNCROSSCHECKED. The validity of this separation does not cite H (Theorem E6.4 is H-free), and the verdict is mathematically sound under the declared belief, field, valuation and observation contract. Its provenance is a single uncrosschecked H solve: L's seed is an extraction solve and Q^H is the envelope path, with no independent authority agreeing with either. This row is outside the receipt set DS-A10 authorised for Experiment E, and it is exploratory tier as every row here is.".to_owned()
                }
            );
            assert_eq!(r.qh, vh, "the separated action lies in H's argmax");
        } else {
            let _ = writeln!(
                *out,
                "  VERDICT: a⋆ = {} NOT SEPARATED (some competitor's upper witness exceeds L).",
                tile(r.action)
            );
        }
    }
    let _ = writeln!(
        *out,
        "  unit budgets: B = {B_WALK} per (coordinate, action), 4B revealed whole-call; residuals (exact, deterministic): H {:?}, extraction {:?}, L {:?}, partition {:?}",
        recs.iter().map(|r| r.h_residual).collect::<Vec<_>>(),
        recs.iter().filter(|r| r.optimal).map(|r| r.extract_residual).collect::<Vec<_>>(),
        recs.iter().filter(|r| r.optimal).map(|r| r.l_residual).collect::<Vec<_>>(),
        recs.iter().filter(|r| r.optimal).map(|r| r.part_residual).collect::<Vec<_>>()
    );
}

fn n4_header(out: &mut String, m_max_gib: u64) {
    let _ = writeln!(
        *out,
        "walt separation probe — the n = 4 rung (Experiment E, four tricks out, receipt-corpus coordinates) — exploratory tier"
    );
    let _ = writeln!(
        *out,
        "rulings: N4-A1..N4-A12; freezes 44, 45 (walt/CENSUS-RULINGS.md 2026-08-13); design walt/SEPARATION-RUNG-N4.md as amended; standing rulings inherited by whole family; mathematics: errata Lemma E3 + (C1)-(C4), Lemma E4, Corollary E4.1 (pending errata §4.3), Theorem E6.4, Theorem E6.5"
    );
    let _ = writeln!(
        *out,
        "regenerate: cargo run --release -p walt-factory --example separation_probe -- n4"
    );
    let _ = writeln!(*out);
    let _ = writeln!(
        *out,
        "SEP-A10's closing sentence, the premise of this design: \"Nothing about the n = 4 rung is unlawful; it is unspecified.\" This rung is its specification."
    );
    let _ = writeln!(
        *out,
        "R0 (BLOCKING, N4-A10): PASSED before any n = 4 unit ran — the grade-3 receipt separation_2026-08-13.txt reproduced under the budgeted walk with exactly the two enumerated permitted differences (the wall-clock provenance line; the freeze-37(h) header sentence replaced by the freeze-44 form), and the candidate library byte-identical."
    );
    let _ = writeln!(
        *out,
        "PRIMAL CEILING (SEP-A2, verbatim): the primal witness at each H-optimal action is an H-optimal policy re-priced by the fixed-policy evaluator, so L = Q^H by Corollary E4.1(2); the separation verdict at this coordinate is determined entirely by the upper witness."
    );
    let _ = writeln!(
        *out,
        "PROVENANCE TYPING (SEP-A12): the separation's validity does not cite H, but this run's witnesses were produced with H's help; the logic of Theorem E6.4 is H-free, the provenance of these witnesses is not."
    );
    let _ = writeln!(
        *out,
        "BUDGET HONESTY (freeze 44): the scalar authority is budgeted (freeze 26, {AUTHORITY_BUDGET} particle-steps); the walk-based evaluators carry declared budgets under freeze 44 (B = {B_WALK} per (coordinate, action); 4B whole-call revealed; P_max = {P_MAX} states checked at each insertion); every stop is a declared stop printed R-A18-style. The freeze-37(h) sentence remains true of the grade-3 receipt as filed and is superseded for this rung by freeze 44."
    );
    let _ = writeln!(
        *out,
        "THE THREE SENTENCES THAT MUST NOT BE BLURRED (§10): (1) this rung tests whether root-action separation closes four tricks out at real-deal coordinates, one grade above the adjudicated grade-3 run. (2) It does NOT test the parent's economy claim [\"the solver does not need an exact solution for every action\"]: it computes the exact H solve at every action; that experiment is walt/ECONOMY-SUCCESSOR.md. (3) It licenses no cost, timing, runtime or tractability claim of any kind (SEP-A15(iii), P-A19, DS-A32, DS-A33); wall-clock is provenance; step counts are exact observables of declared traversals, per-traversal named, and are not a complexity statement."
    );
    let _ = writeln!(
        *out,
        "COST-MODEL INPUTS (P-A21 printed): three rungs are not a law, and no growth rate measured at grades <= 4 is quoted for the opening. Treatment C and C+ coincide on this carrier (xi = omega). Fallback rule (N4-A12, declared in advance): on a §5 gate failure the failure is filed first as a result; the reduced rung is {{h6, h4, h8}} by the declared cheapest-three rule; a second gate failure returns to the rulings file; a reduced pass is labelled REDUCED RUNG, GATE FAILURE FILED."
    );
    let _ = writeln!(
        *out,
        "M_max = {m_max_gib} GiB — declared by the run owner before the rung (N4-A4), provenance beside the machine identity, never a freeze; P_max is not derived from it."
    );
    let _ = writeln!(
        *out,
        "out of scope (leader is not the declaring seat): h3, h7, h10, h11 — printed as in the S5h receipt."
    );
}

fn n4_main() {
    let t0 = Instant::now();
    let m_max_gib: u64 = std::env::var("M_MAX_GIB")
        .ok()
        .and_then(|v| v.parse().ok())
        .expect("M_MAX_GIB must be declared by the run owner (N4-A4): a rung run without a declared M_max is not run");
    let receipt = n4_receipt();
    let mut det = String::new();
    n4_header(&mut det, m_max_gib);

    let loaded = n4_validate_cache();
    let n_loaded = loaded.len();
    let mut computed = 0usize;
    let fresh = loaded.is_empty();
    let mut timing = String::new();
    let mut resume_validated: Option<String> = None;

    for &hand_idx in &N4_IN_SCOPE {
        let hand = &receipt.hands[hand_idx];
        let (kernel, _) = n4_void_free_kernel(hand);
        let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
        let have: Vec<Option<&UnitRec>> = actions
            .iter()
            .map(|a| loaded.get(&(hand_idx, a.index())))
            .collect();
        let n_have = have.iter().filter(|h| h.is_some()).count();
        let tc = Instant::now();
        let recs: Vec<UnitRec> = if n_have == actions.len() {
            let recs: Vec<UnitRec> = have.into_iter().map(|h| h.expect("all").clone()).collect();
            // DS-A30(iii): the declared re-run sample — the first loaded unit
            // in canonical order — recomputed (whole coordinate, since the
            // revealed and authority calls span the coordinate) and asserted
            // non-timing-equal.
            if resume_validated.is_none() {
                let fresh_recs = n4_compute_coordinate(hand_idx, hand);
                assert_eq!(
                    fresh_recs, recs,
                    "resume-validation stop-and-report: recomputed unit records differ from loaded (DS-A30(iii))"
                );
                resume_validated = Some(format!(
                    "resume-validation: PASS (coordinate h{hand_idx} recomputed whole-call; all non-timing fields equal the loaded records)"
                ));
            }
            recs
        } else if n_have > 0 {
            // N4-A9(ii), the shared-call clause: a partially-resumed
            // coordinate re-runs the whole call and asserts the loaded
            // units' values equal the recomputed ones.
            let fresh_recs = n4_compute_coordinate(hand_idx, hand);
            for (a, h) in actions.iter().zip(have) {
                if let Some(rec) = h {
                    let fr = fresh_recs
                        .iter()
                        .find(|r| r.action == *a)
                        .expect("recomputed unit");
                    assert_eq!(
                        fr, rec,
                        "N4-A9(ii) stop-and-report: a partially-resumed coordinate's loaded unit differs from the whole-call recomputation"
                    );
                }
            }
            for r in &fresh_recs {
                n4_save_unit(r);
                computed += 1;
            }
            fresh_recs
        } else {
            let fresh_recs = n4_compute_coordinate(hand_idx, hand);
            for r in &fresh_recs {
                n4_save_unit(r);
                computed += 1;
            }
            fresh_recs
        };
        n4_render_coordinate(hand_idx, hand, &recs, &mut det);
        let _ = writeln!(
            timing,
            "h{hand_idx}: {} ms [this process]",
            tc.elapsed().as_millis()
        );
    }

    let mut out = det.clone();
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== TIMING BLOCK (provenance only, never a dividend; DS-A31/DS-A36) =="
    );
    let _ = writeln!(
        out,
        "provenance: {} (digest {N4_CKPT_DIGEST}; {n_loaded} units loaded, {computed} computed; W = 1 recorded not frozen, DS-A34; single process, DS-A35; cold regenerate: delete store/separation_n4_ckpt and re-run)",
        if fresh { "FRESH" } else { "RESUMED" }
    );
    if let Some(rv) = resume_validated {
        let _ = writeln!(out, "{rv}");
    }
    let _ = writeln!(
        out,
        "M_max = {m_max_gib} GiB (run-owner declaration, N4-A4); a resumed run inherits no quotable timing (DS-A31) — and this rung quotes no timing as a dividend anywhere"
    );
    out.push_str(&timing);
    let _ = writeln!(out, "total wall-clock: {} ms", t0.elapsed().as_millis());
    let _ = writeln!(out, "run complete: yes");

    let results = out_dir("results").join("separation_n4_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    let det_path = out_dir("results").join("separation_n4_2026-08-14_deterministic_block.txt");
    std::fs::write(&det_path, &det).expect("write det block");
    print!("{out}");
    println!("results: {}", results.display());
}

/// The §5 measured single-world rung (SEP-A10(a)): W = 1, fresh process, no
/// checkpoint I/O, selection declared in advance and never by result.
fn n4_rung_main() {
    let t0 = Instant::now();
    let m_max_gib: u64 = std::env::var("M_MAX_GIB")
        .ok()
        .and_then(|v| v.parse().ok())
        .expect("M_MAX_GIB must be declared by the run owner (N4-A4): a rung run without a declared M_max is not run");
    let mut out = String::new();
    let _ = writeln!(
        out,
        "walt separation n4 §5 measured rung — COST MODEL INPUT — exploratory tier"
    );
    let _ = writeln!(
        out,
        "regenerate: cargo run --release -p walt-factory --example separation_probe -- n4-rung (M_MAX_GIB env, run-owner declared)"
    );
    let _ = writeln!(
        out,
        "TYPING (DS-A32, DS-A33, SEP-A15(iii), verbatim from the design): this rung is a cost model input. It produces no ratio, compares no arm against any other arm, and is not a dividend. Its walk-step counts are exact deterministic observables and are load-invariant; its wall-clock and resident-size figures are load-relative provenance and are quotable as nothing. No number in this rung is a result about the game. Because the rung contains a single arm and forms no ratio, DS-A32's contention bias has nothing to bias — stated so the absence of a CONTENDED label is not read as an exemption."
    );
    let _ = writeln!(
        out,
        "selection (freeze 44(e), declared in advance, never by result): first coordinate h0; first root action ascending; world sample = fiber indices (i*g mod {N4_FIBER}) for i = 0..15 with g = {N4_G}; W = 1. M_max = {m_max_gib} GiB (run-owner declaration, N4-A4)."
    );

    // gcd(g, 34650) = 1, asserted in-run (freeze-8 pattern).
    fn gcd(a: u128, b: u128) -> u128 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    assert_eq!(gcd(N4_G, N4_FIBER), 1, "gcd(g, 34650) = 1 (freeze 44(e))");

    let receipt = n4_receipt();
    let hand = &receipt.hands[0];
    let (kernel, leader) = n4_void_free_kernel(hand);
    assert_eq!(leader, kernel.viewer(), "leader offset 0");
    assert_eq!(kernel.count(), N4_FIBER, "|X| = 34,650");
    let first_action = kernel
        .viewer_hand()
        .iter()
        .min_by_key(|d| d.index())
        .expect("nonempty");
    let _ = writeln!(
        out,
        "unit: h0, root action {} (first ascending)",
        tile(first_action)
    );

    // 1. revealed_world_root_values at the 16 sampled worlds: exact
    //    walk-steps per world.
    let dir = Direction::trick_diff();
    let worlds: Vec<walt_kernel::World> = kernel.worlds().collect();
    let mut per_world: Vec<u64> = Vec::new();
    let tw = Instant::now();
    for i in 0..16u128 {
        let wi = usize::try_from((i * N4_G) % N4_FIBER).expect("fits");
        let mut budget = 4 * B_WALK;
        let solved = walt_strat::revealed_world_root_values(
            &kernel,
            &worlds[wi],
            kernel.viewer().team(),
            &dir,
            &mut budget,
        );
        assert!(
            solved.is_some(),
            "a one-world revealed solve fits any sane budget"
        );
        per_world.push(4 * B_WALK - budget);
    }
    let revealed_wall_ms = tw.elapsed().as_millis();
    let sum: u64 = per_world.iter().sum();
    let min = per_world.iter().min().expect("nonempty");
    let max = per_world.iter().max().expect("nonempty");
    let _ = writeln!(
        out,
        "revealed per-world walk-steps (traversal: revealed one-world, all four root actions per world; 16 sampled worlds): min {min}, max {max}, sum {sum}"
    );
    // The pre-declared extrapolation, an estimate and labelled one.
    let est_num = u128::from(sum) * N4_FIBER;
    let _ = writeln!(
        out,
        "pre-declared extrapolation (ESTIMATE, not a measurement of the unit): whole-fiber revealed walk-steps ~= sum * {N4_FIBER} / 16 = {} / 16 = {} (observed per-world spread {min}..{max} printed so the width is visible)",
        est_num,
        est_num / 16
    );

    // 2. InfoPartition::build at (h0, first action) to completion or P_max.
    let tp = Instant::now();
    let mut pb = B_WALK;
    let mut cap_hit = false;
    let partition = InfoPartition::build(&kernel, first_action, &mut pb, P_MAX, &mut cap_hit);
    let partition_wall_ms = tp.elapsed().as_millis();
    let (p_states, p_steps) = match &partition {
        Some(p) => (p.len(), B_WALK - pb),
        None => (0, B_WALK - pb),
    };
    let _ = writeln!(
        out,
        "partition (traversal: partition build): {} — states {}, walk-steps {}, cap_hit {}",
        if partition.is_some() {
            "COMPLETED"
        } else {
            "STOPPED"
        },
        p_states,
        p_steps,
        cap_hit
    );

    // 3. The extraction solve and one policy_value walk, if (2) completed.
    let mut extract_steps: u64 = 0;
    let mut l_steps: u64 = 0;
    let mut extract_wall_ms: u128 = 0;
    if let Some(partition) = &partition {
        let worlds_h: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
        let extract = Extract {
            decl: kernel.decl(),
            focal: kernel.viewer(),
            worlds: &worlds_h,
        };
        let support: Vec<(u32, u128)> = (0..worlds_h.len() as u32).map(|w| (w, 1)).collect();
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = first_action;
        let mut obs = vec![first_action];
        let mut choices: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
        let te = Instant::now();
        let mut eb = B_WALK;
        let solved = extract.solve(
            &support,
            kernel.viewer(),
            tiles,
            1,
            &mut obs,
            &mut choices,
            &mut eb,
        );
        extract_steps = B_WALK - eb;
        if solved.is_some() {
            assert_eq!(choices.len(), partition.len(), "extraction totality");
            let mut by_id: BTreeMap<walt_strat::InfoStateId, Domino> = BTreeMap::new();
            for (record, chosen) in &choices {
                by_id.insert(partition.id(record).expect("a state"), *chosen);
            }
            let policy = Policy::build(partition, |id, _| *by_id.get(&id).expect("total"));
            let mut lb = B_WALK;
            let priced = policy_value_receipt(
                &kernel,
                kernel.viewer().team(),
                &dir,
                partition,
                &policy,
                &mut lb,
            );
            l_steps = B_WALK - lb;
            assert!(priced.is_some(), "the L walk fits its budget at this unit");
            let (_, receipt) = priced.expect("just asserted");
            let _ = writeln!(
                out,
                "extraction (traversal: extraction solve): COMPLETED, steps {extract_steps}; L walk (traversal: policy_value): steps {l_steps}; SEP-A13/SEP-A19 counted receipt {} = {} = {} (of {})",
                receipt.focal_states,
                receipt.singleton_expansions,
                receipt.distinct_states,
                partition.len()
            );
        } else {
            let _ = writeln!(out, "extraction: STOPPED at budget, steps {extract_steps}");
        }
        extract_wall_ms = te.elapsed().as_millis();
    }

    // 4. Wall-clock and peak resident size, as provenance.
    let rss_kb: u64 = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let _ = writeln!(
        out,
        "provenance (load-relative, quotable as nothing): revealed sample wall {revealed_wall_ms} ms; partition wall {partition_wall_ms} ms; extraction+L wall {extract_wall_ms} ms; process resident size {rss_kb} KiB"
    );

    // The go/no-go gate (§5), declared before the rung ran.
    let est_whole_revealed = est_num / 16;
    let budget_ok = est_whole_revealed <= u128::from(4 * B_WALK)
        && u128::from(p_steps) <= u128::from(B_WALK)
        && u128::from(extract_steps) <= u128::from(B_WALK)
        && u128::from(l_steps) <= u128::from(B_WALK);
    let p_ok = !cap_hit && p_states <= P_MAX;
    // Estimated wall-clock of the largest unit: scale the measured h0
    // estimate by the quoted S5h tree-v0 ratio h9/h0 (arithmetic on quoted
    // receipts; a cost-model input licensing nothing).
    let est_h0_wall_ms = (revealed_wall_ms * N4_FIBER) / 16 + partition_wall_ms + extract_wall_ms;
    let est_h9_wall_ms = est_h0_wall_ms * 16_211_488_002 / 3_727_724_856;
    let wall_ok = est_h9_wall_ms <= 600_000;
    let mem_ok = rss_kb / (1024 * 1024) < m_max_gib;
    let _ = writeln!(
        out,
        "gate arithmetic (cost-model inputs, licensing nothing): est whole-fiber revealed steps {est_whole_revealed} vs 4B = {}; partition states {p_states} vs P_max = {P_MAX}; est h0 unit wall {est_h0_wall_ms} ms; est largest-unit (h9, scaled by quoted tree-v0 ratio) wall {est_h9_wall_ms} ms vs 600000 ms; resident {rss_kb} KiB vs M_max {m_max_gib} GiB",
        4 * B_WALK
    );
    let go = budget_ok && p_ok && wall_ok && mem_ok;
    let _ = writeln!(
        out,
        "GO/NO-GO: {} — {}",
        if go { "GO" } else { "NO-GO" },
        if go {
            "the full pass may run"
        } else {
            "gate failure is FILED AS A RESULT (F7); the declared fallback is the reduced rung {h6, h4, h8} (N4-A12), and a second gate failure returns to the rulings file"
        }
    );
    let _ = writeln!(out, "total wall-clock: {} ms", t0.elapsed().as_millis());
    let _ = writeln!(out, "run complete: yes");
    let results = out_dir("results").join("separation_n4_rung_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{out}");
    println!("results: {}", results.display());
}
