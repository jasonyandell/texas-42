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

use walt_core::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel};
use walt_strat::{
    information_prices, policy_value_receipt, Direction, InfoPartition, Policy, ScalarHidden,
    ScalarValuation,
};

/// Freeze 26: the concrete authority and its budget.
const AUTHORITY_BUDGET: u64 = 200_000_000;

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
    /// (least-tile tie rule, freeze 26 — cited, not re-declared).
    fn solve(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &mut BTreeMap<Vec<Domino>, Domino>,
    ) -> Q {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            let mass: Q = support
                .iter()
                .map(|&(_, den)| q(1, i128::try_from(den).expect("den fits")))
                .sum();
            let banked = if winner.team() == Seat::S0.team() {
                mass
            } else {
                qi(0)
            };
            if self.hand_now(support[0].0, Seat::S0, obs).is_empty() {
                return banked;
            }
            return banked + self.solve(support, winner, [Domino::ALL[0]; 4], 0, obs, choices);
        }
        let seat = leader.plus(k);
        let led: Option<Context> = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let mut best: Option<(Q, Domino)> = None;
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.solve(support, leader, tiles, k + 1, obs, choices);
                obs.pop();
                if best.as_ref().map(|(bv, _)| v > *bv).unwrap_or(true) {
                    best = Some((v, a));
                }
            }
            let (v, a) = best.expect("legal move exists");
            choices.insert(obs.clone(), a);
            return v;
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
            sum += self.solve(&sup, leader, tiles, k + 1, obs, choices);
            obs.pop();
        }
        sum
    }
}

// -- conventions ------------------------------------------------------------

/// The freeze-26 bridge at the reporting boundary: count = (diff + grade)/2.
fn to_count(diff: Q) -> Q {
    (diff + qi(i128::try_from(GRADE).expect("grade"))) * q(1, 2)
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
        "BUDGET HONESTY (freeze 37(h)): the scalar authority is budgeted (freeze 26, {AUTHORITY_BUDGET} particle-steps) and its exhaustion is a declared stop; hidden/revealed/price/policy_value carry no budget and no stop."
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
    let prices = information_prices(&kernel, Seat::S0.team(), dir);
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
            to_count(qh_diff[i]),
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
        to_count(v_h_diff),
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
            to_count(qh_diff[i]),
            to_count(u_diff[i]),
            price_diff * q(1, 2)
        );
    }

    // Steps 5-6 per H-optimal action: extraction, library entry, L, R2, R5.
    let mut l_diff: BTreeMap<usize, Q> = BTreeMap::new();
    for &ai in &argmax {
        let a = actions[ai];
        let extract = Extract {
            decl: itf.decl(),
            worlds: &worlds,
        };
        let support: Vec<(u32, u128)> = (0..worlds.len() as u32).map(|i| (i, 1)).collect();
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = a;
        let mut obs = vec![a];
        let mut choices: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
        let _ = extract.solve(&support, Seat::S0, tiles, 1, &mut obs, &mut choices);

        let partition = InfoPartition::build(&kernel, a);
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
        let (line, receipt) =
            policy_value_receipt(&kernel, Seat::S0.team(), dir, &partition, &policy);
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
            to_count(ld),
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
                to_count(l),
                to_count(u_diff[i]),
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
