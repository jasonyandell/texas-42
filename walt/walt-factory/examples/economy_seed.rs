//! walt economy-seed probe — the economy successor (SEP-A17): does the
//! sandwich close on a primal witness that is NOT an exact solve?
//! EXPLORATORY TIER.
//!
//! Design: `walt/ECONOMY-SUCCESSOR.md` as amended by its adjudication
//! EC-A1..EC-A14 (`walt/CENSUS-RULINGS.md`, 2026-08-13); freeze 46 (the
//! closed arm list), freeze 36 v2 (transport: identity and the declaration
//! fold only, EC-A8); mathematics per DS-A17 — Lemma E3, Lemma E4,
//! Non-theorem E4′, Corollary E4.1 (pending errata §4.3), Theorem E6.4,
//! Lemma E7 (§8.3), Lemma S-fold, Corollary R-fold, and Corollary
//! S-fold-val (delivered at EC-A4: value transport along the fold is
//! reading-independent, so every image row is a receipt).
//!
//! Arms (freeze 46(a), CLOSED), canonical order X, T(p′=6), T(p′∈{1..5},
//! idx=0 only), P1, P2, P3, P4, R:
//!   X  — exact control: the freeze-36(f) H-argmax seed, recomputed
//!        in-pass; rows are receipts (g = 0 by Corollary E4.1(2)).
//!   T  — the four library entries transported by the declaration fold
//!        φ_{0→p′}; every image row a receipt (Corollary S-fold-val);
//!        g = 0 by theorem; structure-proving, not economy.
//!   P1 — least legal tile by the canonical ascending domino-index order
//!        (cited to that order as a standing convention, NOT freeze 26).
//!   P2 — greatest legal tile by the same order.
//!   P3 — beat-if-able: not leading and some legal tile strictly beats
//!        every tile so far in the current trick under the declaration's
//!        winner-determining order → the least such; else least legal.
//!   P4 — trump-hoard: least legal non-trump if one exists, else least
//!        legal trump.
//!   R  — HEURISTIC RE-KEY (NOT A TRANSPORT): the idx=0 root-00 entry
//!        moved by the rank-within-live-set correspondence, fallback P1,
//!        fallback count printed.
//!
//! Convention: evaluators run in the focal-minus-opponent trick
//! DIFFERENTIAL; the report is in the COUNT convention across the
//! freeze-26 bridge (affine, slope 2 > 0; verdicts convention-invariant).
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example economy_seed`

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
const SRC_PIP: u8 = 0;

/// One filed S6a per-action row: (hi, lo, Q^H numerator, Q^H denominator).
type FiledAction = (u8, u8, i128, i128);
/// One filed S6a coordinate: base index, per-action rows in ascending domino
/// order, aggregate fusion gap V^F - V^H as (num, den).
type FiledCoord = (u128, [FiledAction; 3], (i128, i128));

/// SEP-A14(ii): the S6a filed values as a frozen table. Quoted from
/// `predictive_rank_2026-08-12.txt`, S6a, exploratory tier. Count convention.
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

/// The ranking inverse of `unrank_comb` (freeze 46(b); its absence was noted
/// at adjudication as part of this build). `subset` is strictly ascending.
fn rank_comb(n: usize, subset: &[usize]) -> u128 {
    let k0 = subset.len();
    let mut rank: u128 = 0;
    let (mut x, mut k, mut n_left) = (0usize, k0, n);
    for &s in subset {
        while x < s {
            // The element x was skipped: add the count of k-subsets that
            // would have started with x.
            rank += binom((n_left - 1) as u128, (k - 1) as u128);
            x += 1;
            n_left -= 1;
        }
        // Take x == s.
        x += 1;
        n_left -= 1;
        k -= 1;
    }
    rank
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

// -- the declaration fold φ_{p→p'} (Lemma S-fold; freeze 36 v2, EC-A8) ------

/// π: p → p′ plus the unique order isomorphism ℙ∖{p} → ℙ∖{p′}.
fn phi_pip(p_src: u8, p_dst: u8, x: u8) -> u8 {
    if x == p_src {
        return p_dst;
    }
    let rest_src: Vec<u8> = (0..=6).filter(|v| *v != p_src).collect();
    let rest_dst: Vec<u8> = (0..=6).filter(|v| *v != p_dst).collect();
    let pos = rest_src.iter().position(|v| *v == x).expect("pip in rest");
    rest_dst[pos]
}

fn phi_tile(p_src: u8, p_dst: u8, d: Domino) -> Domino {
    Domino::new(
        Pip::new(phi_pip(p_src, p_dst, d.hi().value())).expect("pip"),
        Pip::new(phi_pip(p_src, p_dst, d.lo().value())).expect("pip"),
    )
}

fn phi_set(p_src: u8, p_dst: u8, s: DominoSet) -> DominoSet {
    let mut out = DominoSet::EMPTY;
    for d in s.iter() {
        out.insert(phi_tile(p_src, p_dst, d));
    }
    out
}

/// The image key (freeze 46(b)): unrank the source, apply φ tilewise,
/// re-rank both combinations, assemble index′.
fn image_index(src: &Interface, p_dst: u8) -> u128 {
    let live_c = binom(28, (4 * GRADE) as u128);
    let hand_c = binom((4 * GRADE) as u128, GRADE as u128);
    let live_src: DominoSet = src.hand.union(src.pool);
    let live_img = phi_set(src.pip, p_dst, live_src);
    let hand_img = phi_set(src.pip, p_dst, src.hand);
    let live_indices: Vec<usize> = live_img.iter().map(|d| d.index()).collect();
    // R9 round-trip: rank then unrank reproduces the set.
    let live_rank = rank_comb(28, &live_indices);
    assert_eq!(
        unrank_comb(28, 4 * GRADE, live_rank),
        live_indices,
        "R9 round-trip: unrank(rank(live)) == live"
    );
    // Hand positions within the ascending live set.
    let hand_pos: Vec<usize> = live_indices
        .iter()
        .enumerate()
        .filter(|(_, di)| hand_img.contains(Domino::from_index(**di).expect("domino")))
        .map(|(pos, _)| pos)
        .collect();
    let hand_rank = rank_comb(4 * GRADE, &hand_pos);
    assert_eq!(
        unrank_comb(4 * GRADE, GRADE, hand_rank),
        hand_pos,
        "R9 round-trip: unrank(rank(hand)) == hand"
    );
    u128::from(p_dst) * (live_c * hand_c) + live_rank * hand_c + hand_rank
}

// -- the pooled traversal: argmax extraction + focal-state capture ----------

/// One captured focal information state with its public trick context.
struct FocalState {
    record: Vec<Domino>,
    legal: DominoSet,
    /// Tiles played so far in the current trick (positions 0..k).
    tiles: [Domino; 4],
    /// The viewer's position in the current trick; the viewer leads iff 0.
    k: usize,
}

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
    /// (least-tile tie rule, freeze 26 — a tie rule, cited, not a policy)
    /// AND capturing every focal state's public trick context for the
    /// hand-authored arms. One traversal serves all arms.
    #[allow(clippy::too_many_arguments)]
    fn solve(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &mut BTreeMap<Vec<Domino>, Domino>,
        states: &mut Vec<FocalState>,
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
            return banked
                + self.solve(
                    support,
                    winner,
                    [Domino::ALL[0]; 4],
                    0,
                    obs,
                    choices,
                    states,
                );
        }
        let seat = leader.plus(k);
        let led: Option<Context> = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == Seat::S0 {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            states.push(FocalState {
                record: obs.clone(),
                legal,
                tiles,
                k,
            });
            let mut best: Option<(Q, Domino)> = None;
            for a in legal.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.solve(support, leader, tiles, k + 1, obs, choices, states);
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
            sum += self.solve(&sup, leader, tiles, k + 1, obs, choices, states);
            obs.pop();
        }
        sum
    }
}

// -- the hand-authored rules (freeze 46(a)) ---------------------------------

fn least(legal: DominoSet) -> Domino {
    legal
        .iter()
        .min_by_key(|d| d.index())
        .expect("a live state has a legal move")
}

fn greatest(legal: DominoSet) -> Domino {
    legal
        .iter()
        .max_by_key(|d| d.index())
        .expect("a live state has a legal move")
}

/// P1..P4 as total functions of the observation record's public context and
/// the legal set — nothing else (information-consistency per the design's
/// §2.2 arguments, printed in the results header).
fn rule_choice(arm: usize, decl: Decl, st: &FocalState) -> Domino {
    match arm {
        1 => least(st.legal),
        2 => greatest(st.legal),
        3 => {
            // Beat-if-able. The tiles so far in the current trick and their
            // count k are functions of the public record alone (R-A11); the
            // viewer leads iff k == 0.
            if st.k == 0 {
                return least(st.legal);
            }
            let led = decl.led_context(st.tiles[0]);
            let beaters: Vec<Domino> = st
                .legal
                .iter()
                .filter(|c| (0..st.k).all(|i| decl.beats(led, st.tiles[i]).contains(*c)))
                .collect();
            beaters
                .into_iter()
                .min_by_key(|d| d.index())
                .unwrap_or_else(|| least(st.legal))
        }
        4 => {
            let Decl::PipTrump(p) = decl else {
                panic!("this probe's declarations are pip-trump")
            };
            let non_trump: Vec<Domino> = st.legal.iter().filter(|d| !d.has(p)).collect();
            non_trump
                .into_iter()
                .min_by_key(|d| d.index())
                .unwrap_or_else(|| least(st.legal))
        }
        _ => unreachable!("arms are P1..P4"),
    }
}

// -- conventions and evaluation helpers -------------------------------------

/// The freeze-26 bridge at the reporting boundary: count = (diff + grade)/2.
fn to_count(diff: Q) -> Q {
    (diff + qi(i128::try_from(GRADE).expect("grade"))) * q(1, 2)
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

/// Everything one coordinate's exact side supplies: actions in ascending
/// order, Q^H and U per action (differential), the H-optimal set, V^H.
struct CoordExact {
    itf: Interface,
    kernel: Kernel,
    worlds: Vec<[DominoSet; 4]>,
    actions: Vec<Domino>,
    qh: Vec<Q>,
    u: Vec<Q>,
    argmax: Vec<usize>,
}

/// Authority + envelope + prices + R1 at one coordinate. Shared by base and
/// image coordinates.
fn coord_exact(out: &mut String, itf: Interface) -> CoordExact {
    let kernel = itf.kernel();
    assert_eq!(kernel.count(), 1680, "the full void-free fiber (C2/T7)");
    assert_eq!(kernel.viewer(), Seat::S0, "viewer is the declaring leader");
    let worlds: Vec<[DominoSet; 4]> = kernel.worlds().map(|w| w.hands()).collect();
    let auth = ScalarHidden::new(
        itf.decl(),
        Seat::S0,
        Seat::S0.team(),
        ScalarValuation::trick_only(),
    );
    let mut budget = AUTHORITY_BUDGET;
    // Root is trick-leading (empty prefix), so the authority's action list is
    // the viewer hand (SEP-A5(i)).
    let (maybe_av, _stats) = auth.action_values_dag(&worlds, Seat::S0, &[], &mut budget);
    let av = maybe_av.unwrap_or_else(|| {
        panic!("DECLARED STOP: authority budget {AUTHORITY_BUDGET} exceeded; correctness gate unmet (R-A18) — this run's coordinates were chosen for checkability and a stop here is stop-and-report")
    });
    let dir = Direction::trick_diff();
    let prices = information_prices(&kernel, Seat::S0.team(), &dir);
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    assert_eq!(
        av.len(),
        actions.len(),
        "authority covers every root action"
    );
    let mut qh: Vec<Q> = Vec::new();
    let mut u: Vec<Q> = Vec::new();
    for (i, a) in actions.iter().enumerate() {
        let (ha, henv) = &prices.q_h[i];
        let (ca, cenv) = &prices.q_c[i];
        let (aa, aval) = &av[i];
        assert_eq!(ha, a, "H action order is the viewer hand");
        assert_eq!(ca, a, "C action order is the viewer hand");
        assert_eq!(aa, a, "authority action order is the viewer hand");
        let hd = henv.eval(qi(0));
        assert_eq!(
            hd, *aval,
            "R1 solver identification: envelope H equals scalar authority exactly at root {a:?}"
        );
        let price = prices.g_cont_by_root[i].1.eval(qi(0));
        assert!(price >= qi(0), "Lemma E3: U_a >= Q^H(a)");
        qh.push(hd);
        u.push(cenv.eval(qi(0)));
    }
    let _ = writeln!(
        *out,
        "  R1 solver identification: envelope H == scalar authority, per action, exactly, no bridge — HELD ({} actions)",
        actions.len()
    );
    let vh = qh.iter().copied().max().expect("nonempty");
    let argmax: Vec<usize> = (0..actions.len()).filter(|i| qh[*i] == vh).collect();
    CoordExact {
        itf,
        kernel,
        worlds,
        actions,
        qh,
        u,
        argmax,
    }
}

/// One extraction pass at (coordinate, root): argmax choices + captured
/// focal states + the partition.
fn extract(
    ce: &CoordExact,
    root: Domino,
) -> (
    BTreeMap<Vec<Domino>, Domino>,
    Vec<FocalState>,
    InfoPartition,
) {
    let ex = Extract {
        decl: ce.itf.decl(),
        worlds: &ce.worlds,
    };
    let support: Vec<(u32, u128)> = (0..ce.worlds.len() as u32).map(|i| (i, 1)).collect();
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = root;
    let mut obs = vec![root];
    let mut choices = BTreeMap::new();
    let mut states = Vec::new();
    let _ = ex.solve(
        &support,
        Seat::S0,
        tiles,
        1,
        &mut obs,
        &mut choices,
        &mut states,
    );
    let partition = InfoPartition::build(&ce.kernel, root);
    assert_eq!(
        choices.len(),
        partition.len(),
        "R6 totality: the traversal reaches exactly the partition's states"
    );
    assert_eq!(
        states.len(),
        partition.len(),
        "R6 totality: one captured context per partition state"
    );
    (choices, states, partition)
}

/// Build a Policy from a record-keyed choice map (R6: total, legal).
fn policy_from_map(partition: &InfoPartition, choices: &BTreeMap<Vec<Domino>, Domino>) -> Policy {
    let mut by_id: BTreeMap<walt_strat::InfoStateId, Domino> = BTreeMap::new();
    for (record, chosen) in choices {
        let id = partition
            .id(record)
            .expect("R6 stop-and-report: a seed record is not an InfoPartition state");
        by_id.insert(id, *chosen);
    }
    assert_eq!(by_id.len(), partition.len(), "R6: the seed map is total");
    Policy::build(partition, |id, _legal| {
        *by_id.get(&id).expect("total by the assertion above")
    })
}

/// Price one candidate (R2′/R5 machinery shared by every arm).
fn price(
    ce: &CoordExact,
    partition: &InfoPartition,
    policy: &Policy,
) -> (Q, walt_strat::MaxFreeReceipt) {
    let dir = Direction::trick_diff();
    let (line, receipt) =
        policy_value_receipt(&ce.kernel, Seat::S0.team(), &dir, partition, policy);
    assert_eq!(
        receipt.focal_states, receipt.singleton_expansions,
        "R5: every focal expansion was a singleton"
    );
    assert_eq!(
        receipt.focal_states, receipt.distinct_states,
        "R5: no state visited twice; every visited record is a partition state"
    );
    (line.eval(qi(0)), receipt)
}

/// One arm row's verdict against the exact side, plus the R8 identity.
/// Returns (separated, g).
#[allow(clippy::too_many_arguments)]
fn arm_row(
    out: &mut String,
    ce: &CoordExact,
    ai: usize,
    arm_label: &str,
    l_seed: Q,
    receipt: &walt_strat::MaxFreeReceipt,
    partition_len: usize,
    slack: Q,
    zero_slack: bool,
    exact_row: bool,
) -> (bool, Q) {
    let a_star = ce.actions[ai];
    let qh_star = ce.qh[ai];
    // R2′ — the primal sanity assert, a genuine receipt on non-exact arms
    // (Non-theorem E4′'s inversion is what a violation would prove).
    assert!(
        l_seed <= qh_star,
        "R2′ stop-and-report (NO-RESCUE) at {arm_label}, root {a_star:?}: L > Q^H proves a world-informed evaluator on the L path, a partition disagreement, or a seed that is not a total lawful map"
    );
    if exact_row {
        assert_eq!(
            l_seed, qh_star,
            "arm X is at the ceiling by Corollary E4.1(2); inequality is a pipeline defect"
        );
    }
    let g = qh_star - l_seed; // differential convention; halved for count print
    let mut separated = true;
    let mut pair_text = String::new();
    for (i, a) in ce.actions.iter().enumerate() {
        if i == ai {
            continue;
        }
        let ok = l_seed >= ce.u[i];
        if !ok {
            separated = false;
        }
        let _ = write!(
            pair_text,
            " vs {}: L{}U;",
            tile(*a),
            if ok { " >= " } else { " < " }
        );
    }
    // R8 — the slack identity, asserted against the independently computed
    // pairwise verdicts (two computations, one stop on divergence).
    assert_eq!(
        g <= slack,
        separated,
        "R8 stop-and-report: (g <= s) must equal the pairwise separation verdict"
    );
    let verdict = if separated && zero_slack && !exact_row {
        "ZERO-SLACK: SEED EXACTLY OPTIMAL (NOT ECONOMY)"
    } else if separated {
        "SEPARATED"
    } else {
        "NOT SEPARATED"
    };
    let _ = writeln!(
        *out,
        "    arm {arm_label}: L = {}  g = {}  |{}  -> {}",
        to_count(l_seed),
        g * q(1, 2),
        pair_text,
        verdict
    );
    let _ = writeln!(
        *out,
        "      R2′ {}; R5 counted receipt {} = {} = {} (reached, of partition {}); R6 total+legal HELD",
        if exact_row {
            "L = Q^H (arm-X ceiling receipt, Corollary E4.1(2))"
        } else {
            "L <= Q^H HELD (genuine receipt vs Non-theorem E4′)"
        },
        receipt.focal_states,
        receipt.singleton_expansions,
        receipt.distinct_states,
        partition_len
    );
    if separated && zero_slack && !exact_row {
        let _ = writeln!(
            *out,
            "      the seed exactly attains the optimal value at this action; this is not economy (EC-A10)"
        );
    }
    (separated, g)
}

fn main() {
    let t0 = Instant::now();
    let mut out = String::new();

    let _ = writeln!(
        out,
        "walt economy-seed probe — the economy successor (SEP-A17) — exploratory tier"
    );
    let _ = writeln!(
        out,
        "rulings: EC-A1..EC-A14; freeze 46; freeze 36 v2 (walt/CENSUS-RULINGS.md 2026-08-13); design walt/ECONOMY-SUCCESSOR.md as amended; standing rulings inherited by whole family (F, r3 Q, Y, P-A, X-A, E-A, S-A, R-A, PG-A, J-A, DS-A, SEP-A, N4-A); mathematics: errata Lemma E3, Lemma E4, Non-theorem E4′, Corollary E4.1 (pending errata §4.3), Theorem E6.4, Lemma E7; Lemma S-fold, Corollary R-fold, Corollary S-fold-val"
    );
    let _ = writeln!(
        out,
        "regenerate: cargo run --release -p walt-factory --example economy_seed"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "PRIMAL RETYPING (§1.1 replacement, arm-scoped per EC-A2): the primal witness at each seeded action is a NON-EXACT lawful policy re-priced by the fixed-policy evaluator. Corollary E4.1(2) does not apply and L < Q^H is expected: the quantity Q^H(a⋆) − L is this run's measurement, the economy gap. Unlike the adjudicated grade-3 run, the separation verdict here is decided by BOTH sides of the sandwich, and a failure to separate is a failure of THIS candidate, not the exact negative of Corollary E4.1(3). This applies to arms P1–P4 and R; arm X's rows are at the ceiling by Corollary E4.1(2) and carry SEP-A2's sentence as row typing; arm T's rows are exact relabellings and carry the arm-T honesty headline."
    );
    let _ = writeln!(
        out,
        "ARM-T HONESTY HEADLINE (§2.1): because the transport is a value-order isomorphism (Corollary R-fold; Corollary S-fold-val), the transported entry is an exact relabelling of an exact solve. Its economy gap is zero BY THEOREM, not by measurement. Arm T is a STRUCTURE-PROVING arm and is not an economy arm. The economy content of this run is arms P and R."
    );
    let _ = writeln!(
        out,
        "OUTCOME 3 UNAVAILABLE (§3): all three base coordinates already SEPARATED with the exact seed, so NOT-CERTIFIABLE-BY-ANY-SEED (Corollary E4.1(3)'s exact negative) cannot arise at this carrier; its absence is not evidence about seeds."
    );
    let _ = writeln!(
        out,
        "PROVENANCE TYPING (SEP-A12, sharpened): the separation's validity does not cite H, but this run's witnesses and its slack column were produced with H's help — s(a⋆) is computed from H and U, R1/R3/R4 are H cross-checks, and R2′ is asserted against H. The logic of Theorem E6.4 is H-free; the provenance of these particular numbers is not."
    );
    let _ = writeln!(
        out,
        "SCOPE (EC-A13): this run tests the PRIMAL half of the parent's economy claim [bracketed quotation: \"the solver does not need an exact solution for every action\"] — whether the WITNESS at a⋆ must be an exact solve. The run itself still computes U exactly at every competitor and H at every action for its receipts, so the RUN is not cheap; the full claim additionally requires the U side cheapened (Theorem E6.5's gluing ladder, Experiment D, freeze 38 reserved) and remains untested. No timing, cost, runtime or tractability claim of any kind (SEP-A15(iii), P-A19); no arm is compared against any other arm by cost."
    );
    let _ = writeln!(
        out,
        "ECONOMY-GAP COLUMN FENCE (§5.2): g is a distance between two exact values at one declared coordinate under one declared belief. It is not a quality score for the rule, not transferable to any other coordinate, not a policy ranking, and not a term in the DS-A2 ladder. A rule with a small gap at one coordinate has demonstrated nothing whatever about another."
    );
    let _ = writeln!(
        out,
        "FENCES: R-A2 (fiber members are FEASIBLE, never reachable; nothing here is identity-bearing); treatment-C naming (ξ = ω on this carrier, so C and C⁺ coincide); DS-A16 (entries remain valid primal-witness sources under count re-entry, evaluated under the richer valuation; their count-free quality verdicts do not survive); no image library entries are written in this run (EC-A8); information-consistency arguments for P3: the tiles so far in the current trick, their count, and the declaration are functions of the public record alone (R-A11) — no hidden hand is consulted."
    );

    // Base idx=0 extraction is arm R's source; hold it across coordinates.
    // (choices keyed by record, live set ascending.)
    type Src0 = (BTreeMap<Vec<Domino>, Domino>, Vec<Domino>);
    let mut src0: Option<Src0> = None;

    for (index, filed_q, filed_gap) in S6A_FILED {
        let itf = coordinate(GRADE, index);
        let hand: Vec<String> = itf.hand.iter().map(tile).collect();
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "== base coord idx={index} pip={} (derived) trump={} hand=[{}] |X|=1680 enumeration=freeze-7/23 ==",
            itf.pip,
            itf.pip,
            hand.join(" ")
        );
        let ce = coord_exact(&mut out, itf);

        // R4 — the S6a cross-check (base coordinates only; SEP-A14 entire).
        for (i, (hi, lo, num, den)) in filed_q.iter().enumerate() {
            let filed_action =
                Domino::new(Pip::new(*hi).expect("pip"), Pip::new(*lo).expect("pip"));
            assert_eq!(filed_action, ce.actions[i], "S6a filed action order");
            assert_eq!(
                to_count(ce.qh[i]),
                q(*num, *den),
                "R4: recomputed Q^H equals the filed S6a value exactly"
            );
        }
        let dirn = Direction::trick_diff();
        let gap_diff = information_prices(&ce.kernel, Seat::S0.team(), &dirn)
            .g_total
            .eval(qi(0));
        assert_eq!(
            gap_diff * q(1, 2),
            q(filed_gap.0, filed_gap.1),
            "R4: recomputed aggregate gap equals the filed fusion_gap exactly"
        );
        let _ = writeln!(
            out,
            "  R4 S6a cross-check: Q^H per action and aggregate gap equal the filed values exactly — HELD"
        );
        let _ = writeln!(
            out,
            "  aggregate gap V^F - V^H = {} (count). ONE-SIDED SCREEN (SEP-A15(i)): licenses only Corollary E3.2's zero case; nonzero gap-versus-headroom comparisons are not evidence in either direction.",
            gap_diff * q(1, 2)
        );

        // R3 rows and the slack per H-optimal action (no clamp, no s >= 0
        // assumption — s < 0 is Corollary E4.1(3)'s signature, EC-A3).
        for (i, a) in ce.actions.iter().enumerate() {
            let _ = writeln!(
                out,
                "  R3 root {}: Q^H = {}  U = {}  price U - Q^H = {}  (count)",
                tile(*a),
                to_count(ce.qh[i]),
                to_count(ce.u[i]),
                (ce.u[i] - ce.qh[i]) * q(1, 2)
            );
        }

        for &ai in &ce.argmax.clone() {
            let a_star = ce.actions[ai];
            let max_comp_u = ce
                .actions
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != ai)
                .map(|(i, _)| ce.u[i])
                .max()
                .expect("a competitor exists");
            let slack = ce.qh[ai] - max_comp_u;
            let zero_slack = slack == qi(0);
            let _ = writeln!(
                out,
                "  -- a⋆ = {}: Q^H(a⋆) = {}  slack s(a⋆) = {} (count){}",
                tile(a_star),
                to_count(ce.qh[ai]),
                slack * q(1, 2),
                if zero_slack {
                    "  [ZERO-SLACK CONTROL: a seed separates here iff g = 0 — a theorem (R8), pre-declared]"
                } else {
                    ""
                }
            );

            // Cheap-arm separation tracking for the §5.2 outcome block
            // (cheap = P1..P4 and R; X and T are exact by construction).
            let mut cheap_separated: Vec<&str> = Vec::new();

            // Arm X — the exact control (EC-A1): freeze-36(f) seed, in-pass.
            let (choices, states, partition) = extract(&ce, a_star);
            let policy_x = policy_from_map(&partition, &choices);
            let (l_x, r_x) = price(&ce, &partition, &policy_x);
            let (sep_x, _gx) = arm_row(
                &mut out,
                &ce,
                ai,
                "X (exact control)",
                l_x,
                &r_x,
                partition.len(),
                slack,
                zero_slack,
                true,
            );
            assert!(
                sep_x,
                "the exact seed separated in the adjudicated run and must here (R4-consistency)"
            );

            if index == 0 {
                let mut live: Vec<Domino> = ce.itf.hand.union(ce.itf.pool).iter().collect();
                live.sort_by_key(|d| d.index());
                src0 = Some((choices.clone(), live));
            }

            // Arm T — note line; rows live in the image blocks that follow.
            let _ = writeln!(
                out,
                "    arm T: transported-image receipt rows follow in this coordinate's image blocks (g = 0 by theorem; structure-proving)"
            );

            // Arms P1..P4.
            for arm in 1..=4usize {
                let mut cmap: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
                for st in &states {
                    cmap.insert(st.record.clone(), rule_choice(arm, ce.itf.decl(), st));
                }
                let pol = policy_from_map(&partition, &cmap);
                let (l_p, r_p) = price(&ce, &partition, &pol);
                let label = [
                    "",
                    "P1 least-tile",
                    "P2 greatest-tile",
                    "P3 beat-if-able",
                    "P4 trump-hoard",
                ][arm];
                let (sep_p, _) = arm_row(
                    &mut out,
                    &ce,
                    ai,
                    label,
                    l_p,
                    &r_p,
                    partition.len(),
                    slack,
                    zero_slack,
                    false,
                );
                if sep_p {
                    cheap_separated.push(label);
                }
            }

            // Arm R — heuristic re-key from the idx=0 root-00 entry, at the
            // OTHER base coordinates only (freeze 46(a)).
            if index != 0 {
                let (src_choices, src_live) =
                    src0.as_ref().expect("base 0 runs first in canonical order");
                let mut tgt_live: Vec<Domino> = ce.itf.hand.union(ce.itf.pool).iter().collect();
                tgt_live.sort_by_key(|d| d.index());
                let tgt_rank: BTreeMap<usize, usize> = tgt_live
                    .iter()
                    .enumerate()
                    .map(|(r, d)| (d.index(), r))
                    .collect();
                let src_root = Domino::new(Pip::new(0).expect("pip"), Pip::new(0).expect("pip"));
                let mut fallbacks: u64 = 0;
                let mut cmap: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
                for st in &states {
                    // Map the target record into the source coordinate: root
                    // position -> source root by declaration; each subsequent
                    // play -> the source tile of equal rank in the canonical
                    // ascending live-set order.
                    let mut mapped: Vec<Domino> = Vec::with_capacity(st.record.len());
                    mapped.push(src_root);
                    let mut ok = true;
                    for t in &st.record[1..] {
                        match tgt_rank.get(&t.index()) {
                            Some(r) => mapped.push(src_live[*r]),
                            None => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    let choice = if ok {
                        match src_choices.get(&mapped) {
                            Some(src_tile) => {
                                let sr = src_live
                                    .iter()
                                    .position(|d| d == src_tile)
                                    .expect("source choice is a live tile");
                                let back = tgt_live[sr];
                                if st.legal.contains(back) {
                                    Some(back)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    } else {
                        None
                    };
                    let final_choice = choice.unwrap_or_else(|| {
                        fallbacks += 1;
                        least(st.legal)
                    });
                    cmap.insert(st.record.clone(), final_choice);
                }
                let pol = policy_from_map(&partition, &cmap);
                let (l_r, r_r) = price(&ce, &partition, &pol);
                let _ = writeln!(
                    out,
                    "    arm R — HEURISTIC RE-KEY (NOT A TRANSPORT): source idx=0 root 00; fallback-to-P1 states: {fallbacks} of {}",
                    partition.len()
                );
                let (sep_r, _) = arm_row(
                    &mut out,
                    &ce,
                    ai,
                    "R re-key [HEURISTIC RE-KEY (NOT A TRANSPORT)]",
                    l_r,
                    &r_r,
                    partition.len(),
                    slack,
                    zero_slack,
                    false,
                );
                if sep_r {
                    cheap_separated.push("R re-key");
                }
            }

            // The §5.2 outcome block, pre-declared verbatim.
            if zero_slack {
                let _ = writeln!(
                    out,
                    "  OUTCOME (a⋆ = {}): ZERO-SLACK: SEED EXACTLY OPTIMAL (NOT ECONOMY) — s = 0, so by the R8 identity a seed separates here iff it exactly attains the optimal value at this action; separating cheap arms ({}) demonstrate attainment, not economy (EC-A10). This coordinate is the control, pre-declared as a theorem.",
                    tile(a_star),
                    if cheap_separated.is_empty() {
                        "none".to_owned()
                    } else {
                        cheap_separated.join(", ")
                    }
                );
            } else if cheap_separated.is_empty() {
                let _ = writeln!(
                    out,
                    "  OUTCOME (a⋆ = {}): CERTIFIED-EXACT-ONLY — at this coordinate the declared cheap family does not supply a witness within the slack. FENCE (mandatory): this is NOT an exact negative and is not Corollary E4.1(3). It says these declared rules failed; it says nothing about candidate sets in general, and a better candidate could close it. A primal failure is a failure of the candidate; only a U-side failure is a proof about all candidates.",
                    tile(a_star)
                );
            } else {
                let _ = writeln!(
                    out,
                    "  OUTCOME (a⋆ = {}): CERTIFIED-CHEAP by {} — at this coordinate an exact solve at a⋆ was not necessary to obtain the primal witness; the parent's economy claim [\"the solver does not need an exact solution for every action\"] is exercised on the PRIMAL side at a⋆. FENCE (mandatory and adjacent): this run still computes U exactly at every competitor and still computes H at every action for its receipts, so the RUN is not cheap. What is tested is whether the WITNESS must be exact, which is SEP-A17's scope and is narrower than \"the solver is cheap\". No timing, cost, runtime or tractability claim of any kind follows; no arm is compared against any other arm by cost.",
                    tile(a_star),
                    cheap_separated.join(", ")
                );
            }
        }

        // -- image blocks for this base coordinate (arm T + R9 only) --------
        let images: Vec<u8> = if index == 0 {
            vec![6, 1, 2, 3, 4, 5] // p'=6 first (canonical: images ascending by image index — p'=6's index is smallest? computed below; order by computed image index ascending)
        } else {
            vec![6]
        };
        // Compute image indices and order ascending (EC-A1(c)).
        let mut img_list: Vec<(u128, u8)> = images
            .iter()
            .map(|p| (image_index(&ce.itf, *p), *p))
            .collect();
        img_list.sort();
        for (idx_img, p_dst) in img_list {
            let img_itf = coordinate(GRADE, idx_img);
            // R9 — the key correspondence reproduces (p', φL, φH).
            assert_eq!(img_itf.pip, p_dst, "R9: image pip");
            assert_eq!(
                img_itf.hand,
                phi_set(SRC_PIP, p_dst, ce.itf.hand),
                "R9: image hand is φ(hand)"
            );
            assert_eq!(
                img_itf.hand.union(img_itf.pool),
                phi_set(SRC_PIP, p_dst, ce.itf.hand.union(ce.itf.pool)),
                "R9: image live set is φ(live)"
            );
            let ihand: Vec<String> = img_itf.hand.iter().map(tile).collect();
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "== image coord idx′={idx_img} p′={p_dst} (φ of base idx={index}) trump={} hand=[{}] |X|=1680 — arm-T receipt rows and R9 only ==",
                p_dst,
                ihand.join(" ")
            );
            let ice = coord_exact(&mut out, img_itf);
            // R9 value equalities: Q^H and U per corresponding action
            // (Corollary S-fold-val; EC-A14's strengthening). Stop-and-report
            // at every image (EC-A4).
            for (i, a) in ce.actions.iter().enumerate() {
                let fa = phi_tile(SRC_PIP, p_dst, *a);
                let j = ice
                    .actions
                    .iter()
                    .position(|d| *d == fa)
                    .expect("R9: φ(action) is an image action");
                assert_eq!(
                    ce.qh[i], ice.qh[j],
                    "R9 stop-and-report: Q^H differs at source/image for {a:?} -> {fa:?} (a defect in walt-core's rules, the fold, or the key — never a finding, EC-A4)"
                );
                assert_eq!(
                    ce.u[i], ice.u[j],
                    "R9 stop-and-report: U differs at source/image for {a:?} -> {fa:?}"
                );
            }
            let _ = writeln!(
                out,
                "  R9: key round-trips HELD; coordinate(3, idx′) reproduces (p′, φL, φH) — HELD; |X′| = 1680 — HELD; Q^H and U equal at source/image per corresponding action — HELD (values only, never per-world byte-equality)"
            );
            // Arm-T rows: each library entry of this base coordinate,
            // transported and re-priced at the image.
            for &ai in &ce.argmax {
                let a_star = ce.actions[ai];
                let fa = phi_tile(SRC_PIP, p_dst, a_star);
                let (src_choices, _states, _src_part) = extract(&ce, a_star);
                let mut mapped: BTreeMap<Vec<Domino>, Domino> = BTreeMap::new();
                for (rec, ch) in &src_choices {
                    let mrec: Vec<Domino> =
                        rec.iter().map(|d| phi_tile(SRC_PIP, p_dst, *d)).collect();
                    mapped.insert(mrec, phi_tile(SRC_PIP, p_dst, *ch));
                }
                let ipart = InfoPartition::build(&ice.kernel, fa);
                let ipol = policy_from_map(&ipart, &mapped);
                let (l_t, r_t) = price(&ice, &ipart, &ipol);
                assert_eq!(
                    l_t, ce.qh[ai],
                    "R9 stop-and-report: L of the transported policy differs from the source Q^H(a⋆)"
                );
                let ji = ice.actions.iter().position(|d| *d == fa).expect("image a⋆");
                let _ = writeln!(
                    out,
                    "    arm T entry (idx={index}, root {}) -> image root {}: L = {} = source Q^H(a⋆) — receipt HELD (g = 0 by theorem); R5 {} = {} = {} (of {})",
                    tile(a_star),
                    tile(fa),
                    to_count(l_t),
                    r_t.focal_states,
                    r_t.singleton_expansions,
                    r_t.distinct_states,
                    ipart.len()
                );
                assert_eq!(ice.qh[ji], l_t, "image Q^H(φ a⋆) equals transported L");
            }
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only, never a dividend): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    let results = out_dir("results").join("economy_seed_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    print!("{out}");
    println!("results: {}", results.display());
}
