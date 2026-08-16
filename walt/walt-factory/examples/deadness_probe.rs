//! The decision-deadness probe (S6c): cheap one-sided detectors under
//! rulings J-A1..J-A18, Lemma J, Propositions J-0 / J-1 / J-win
//! (`walt/CENSUS-RULINGS.md`, 2026-08-12). Design: `walt/DEADNESS-PROBE.md`.
//!
//! Three detectors (freeze 32), each one-sided (fires => dead; UNKNOWN is
//! lawful) and each carrying Jason's binding count guard as a conjunct:
//!   D0     (Proposition J-0): not-on-lead + no trumps + below every
//!          potential leader in every followable context. Tags: count-free
//!          AND trick-plus-count (Lemma J(c)).
//!   D1-sym (Proposition J-1): two-tile transposition isomorphism. Tags:
//!          count-free AND trick-plus-count (the guard lifts E-A2 for this
//!          transport).
//!   D1-win (Proposition J-win): focal sweeps under every order. Tag:
//!          COUNT-FREE ONLY (the guard does not rescue it; J-A3).
//! Forced nodes (|legal| = 1) are never call sites and are tallied in
//! their own column (J-A13, J-A15).
//!
//! Ground truth (freeze 34, J-A10, two denominators never fused): the
//! one-deviation tie classifier (argmax-indifference, a SUPERSET of
//! deadness) at every call site; the exact dead set (all policies tie,
//! subtree-enumerated) where the subtree's free-state count <= the
//! declared bound. Soundness receipt: a fired node must be tied under
//! both computable denominators — a disagreement is stop-and-report.
//!
//! Harvest (freeze 35, J-A13/A14): H-plain vs H-with-detector, same
//! solver, detector charged inside its arm, focal branching only ever
//! pruned, V and every root Q asserted bit-exact between arms.
//! Exploratory tier; no floats.

use std::collections::BTreeMap;
use std::time::Instant;

use walt_core::receipt::{locate_verify_player, parse_file, Receipt, ReceiptHand};
use walt_core::replay::state_before_trick;
use walt_core::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Hidden, Kernel, HIDDEN_SEATS};

// -- coordinates (S6a freeze 22) and the S5h receipt rung ------------------

#[derive(Clone)]
struct Root {
    label: String,
    decl: Decl,
    focal: Seat,
    worlds: Vec<[DominoSet; 4]>,
    leads: DominoSet,
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

fn grade3_root(index: u128) -> Root {
    let grade = 3usize;
    let live_c = binom(28, 12);
    let hand_c = binom(12, 3);
    let pip = (index / (live_c * hand_c)) as u8;
    let rem = index % (live_c * hand_c);
    let live_idx = unrank_comb(28, 12, rem / hand_c);
    let hand_pos: std::collections::BTreeSet<usize> =
        unrank_comb(12, 3, rem % hand_c).into_iter().collect();
    let (mut pool, mut hand) = (DominoSet::EMPTY, DominoSet::EMPTY);
    for (pos, di) in live_idx.iter().enumerate() {
        let d = Domino::from_index(*di).expect("domino");
        if hand_pos.contains(&pos) {
            hand.insert(d);
        } else {
            pool.insert(d);
        }
    }
    let decl = Decl::PipTrump(Pip::new(pip).expect("pip"));
    let hidden = [Seat::S1, Seat::S2, Seat::S3].map(|s| Hidden {
        seat: s,
        capacity: grade,
        voids: ContextSet::EMPTY,
    });
    let kernel = Kernel::new(decl, Seat::S0, hand, pool, hidden).expect("kernel");
    Root {
        label: format!("g3 idx={index} pip={pip}"),
        decl,
        focal: Seat::S0,
        worlds: kernel.worlds().map(|w| w.hands()).collect(),
        leads: hand,
    }
}

/// The S5h n=4 receipt rung: hands 0-12 of the frozen corpus at trick 4,
/// void-free typing (P-A1/P-A2), eligible where the bidder leads.
fn receipt_rung_roots() -> Vec<Root> {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");
    let mut out = Vec::new();
    for (i, hand) in receipt.hands.iter().enumerate().take(13) {
        if !matches!(hand.decl, Decl::PipTrump(_)) {
            continue;
        }
        let (root, eligible) = receipt_root(hand, 4, i);
        if eligible {
            out.push(root);
        }
    }
    out
}

fn receipt_root(hand: &ReceiptHand, trick_no: usize, idx: usize) -> (Root, bool) {
    let (hands, leader) = state_before_trick(hand, trick_no).expect("the receipt replays");
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
        .expect("void-free capacity kernel");
    (
        Root {
            label: format!("n4 hand={idx}"),
            decl: hand.decl,
            focal,
            worlds: kernel.worlds().map(|w| w.hands()).collect(),
            leads: hands[focal.index()],
        },
        leader == focal,
    )
}

// -- the detectors (freeze 32; all bitset; guard = a conjunct everywhere) --

fn count_mask() -> DominoSet {
    let mut m = DominoSet::EMPTY;
    for d in Domino::ALL {
        if d.count() > 0 {
            m.insert(d);
        }
    }
    m
}

struct Masks {
    decl: Decl,
    trumps: DominoSet,
    count: DominoSet,
    /// Per context q: {d : led_context(d) = q}.
    leaders: [DominoSet; Context::COUNT],
    /// Per context q: the follow incidence.
    inc: [DominoSet; Context::COUNT],
}

impl Masks {
    fn new(decl: Decl) -> Masks {
        let mut trumps = DominoSet::EMPTY;
        let mut leaders = [DominoSet::EMPTY; Context::COUNT];
        let mut inc = [DominoSet::EMPTY; Context::COUNT];
        for d in Domino::ALL {
            leaders[decl.led_context(d).index()].insert(d);
        }
        for (qi_, q) in Context::ALL.iter().enumerate() {
            inc[qi_] = decl.effective_incidence(*q);
        }
        if let Some(called) = Context::from_index(Context::CALLED_INDEX) {
            trumps = decl.effective_incidence(called);
        }
        Masks {
            decl,
            trumps,
            count: count_mask(),
            leaders,
            inc,
        }
    }
}

/// The node data every detector reads: focal hand H, the other live tiles O
/// (unplayed non-focal tiles plus the current trick's played tiles), the
/// current led tile if focal is following, and whether focal is on lead.
struct Node {
    hand: DominoSet,
    other: DominoSet,
    led: Option<Domino>,
    on_lead: bool,
}

/// D0, Proposition J-0. Fires => dead under count-free AND trick-plus-count.
fn d0(m: &Masks, n: &Node) -> bool {
    if n.on_lead || !n.hand.is_disjoint(m.trumps) || !n.hand.is_disjoint(m.count) {
        return false;
    }
    // Current trick: the actually-led tile must beat every focal tile that
    // could be played to it (followers if focal has them, else any: a
    // tier-0 slough is beaten by the led tile automatically, so only the
    // follow case needs the comparison).
    if let Some(d0_tile) = n.led {
        let led_q = m.decl.led_context(d0_tile);
        for t in n.hand.intersection(m.inc[led_q.index()]).iter() {
            if !m.decl.beats(led_q, t).contains(d0_tile) {
                return false;
            }
        }
    }
    // Future tricks: for every focal tile t, every context q it follows,
    // every potential leader of q outside H beats t.
    for t in n.hand.iter() {
        for (qi_, q) in Context::ALL.iter().enumerate() {
            if !m.inc[qi_].contains(t) {
                continue;
            }
            let potential = n.other.intersection(m.leaders[qi_]);
            if !potential.is_subset_of(m.decl.beats(*q, t)) {
                return false;
            }
        }
    }
    true
}

/// A context is still leadable if a non-focal live tile leads it, or it is
/// the current led context (Proposition J-1's clause).
fn still_leadable(m: &Masks, n: &Node, qi_: usize) -> bool {
    if !n.other.intersection(m.leaders[qi_]).is_empty() {
        return true;
    }
    n.led
        .map(|d| m.decl.led_context(d).index() == qi_)
        .unwrap_or(false)
}

/// D1-sym, Proposition J-1. Fires => dead under count-free AND
/// trick-plus-count.
fn d1_sym(m: &Masks, n: &Node) -> bool {
    if n.hand.len() != 2 || !n.hand.is_disjoint(m.count) {
        return false;
    }
    let mut it = n.hand.iter();
    let (t1, t2) = (it.next().expect("two"), it.next().expect("two"));
    if t1.is_double() != t2.is_double() {
        return false;
    }
    if m.trumps.contains(t1) != m.trumps.contains(t2) {
        return false;
    }
    // Unless focal provably never leads (D0's induction), the led-context
    // map on {t1, t2} must be preserved by the swap: l(t1) = l(t2).
    let never_leads = d0_never_leads(m, n);
    if !never_leads && m.decl.led_context(t1) != m.decl.led_context(t2) {
        return false;
    }
    for (qi_, q) in Context::ALL.iter().enumerate() {
        if !still_leadable(m, n, qi_) {
            continue;
        }
        if m.inc[qi_].contains(t1) != m.inc[qi_].contains(t2) {
            return false;
        }
        if m.inc[qi_].contains(t1) {
            // Adjacency in the surviving order: no live non-H tile sits
            // strictly between them (symmetric difference of beat sets).
            let between = m
                .decl
                .beats(*q, t1)
                .difference(m.decl.beats(*q, t2))
                .union(m.decl.beats(*q, t2).difference(m.decl.beats(*q, t1)));
            if !n.other.intersection(between).is_empty() {
                return false;
            }
        }
    }
    true
}

/// D0's induction premises minus the count guard: used by D1-sym to
/// discharge the led-context clause.
fn d0_never_leads(m: &Masks, n: &Node) -> bool {
    if n.on_lead || !n.hand.is_disjoint(m.trumps) {
        return false;
    }
    if let Some(d0_tile) = n.led {
        let led_q = m.decl.led_context(d0_tile);
        for t in n.hand.intersection(m.inc[led_q.index()]).iter() {
            if !m.decl.beats(led_q, t).contains(d0_tile) {
                return false;
            }
        }
    }
    for t in n.hand.iter() {
        for (qi_, q) in Context::ALL.iter().enumerate() {
            if !m.inc[qi_].contains(t) {
                continue;
            }
            if !n
                .other
                .intersection(m.leaders[qi_])
                .is_subset_of(m.decl.beats(*q, t))
            {
                return false;
            }
        }
    }
    true
}

/// D1-win, Proposition J-win. Fires => dead under COUNT-FREE ONLY (J-A3).
fn d1_win(m: &Masks, n: &Node) -> bool {
    if !n.hand.is_disjoint(m.count) {
        return false;
    }
    // Every trump outside H dead (i.e. none live outside H).
    if !n.other.intersection(m.trumps).is_empty() {
        return false;
    }
    for (qi_, q) in Context::ALL.iter().enumerate() {
        if !still_leadable(m, n, qi_) {
            continue;
        }
        let rivals = n.other.intersection(m.inc[qi_]);
        for t in n.hand.iter() {
            if !m.inc[qi_].contains(t) {
                return false;
            }
            if !rivals.is_subset_of(m.decl.beats(*q, t)) {
                return false;
            }
        }
    }
    // Each t beats every rival in its own led context.
    for t in n.hand.iter() {
        let q = m.decl.led_context(t);
        let rivals = n.other.intersection(m.inc[q.index()]);
        if !rivals.is_subset_of(m.decl.beats(q, t)) {
            return false;
        }
    }
    true
}

// -- the pooled walk: solve, ground truth, harvest --------------------------

struct Walker<'a> {
    decl: Decl,
    focal: Seat,
    worlds: &'a [[DominoSet; 4]],
    masks: &'a Masks,
}

#[derive(Default)]
struct Census {
    call_sites: usize,
    forced: usize,
    fired_d0: usize,
    fired_sym: usize,
    fired_win: usize,
    fired_any: usize,
    detector_ns: u128,
    detector_calls: usize,
}

impl Walker<'_> {
    fn hand_now(&self, wi: u32, seat: Seat, obs: &[Domino]) -> DominoSet {
        let mut h = self.worlds[wi as usize][seat.index()];
        for t in obs {
            h.remove(*t);
        }
        h
    }

    fn node_data(
        &self,
        support: &[(u32, u128)],
        tiles: &[Domino; 4],
        k: usize,
        obs: &[Domino],
    ) -> Node {
        let hand = self.hand_now(support[0].0, self.focal, obs);
        let mut universe = DominoSet::EMPTY;
        for s in Seat::ALL {
            universe = universe.union(self.worlds[support[0].0 as usize][s.index()]);
        }
        // Live non-focal tiles plus the current trick's table tiles.
        let mut other = universe;
        for t in obs {
            other.remove(*t);
        }
        other = other.difference(hand);
        for t in tiles.iter().take(k) {
            other.insert(*t);
        }
        Node {
            hand,
            other,
            led: (k > 0).then(|| tiles[0]),
            on_lead: k == 0,
        }
    }

    /// The pooled H solve (count convention, c in {0,1}); when `census` is
    /// given, the detectors run at every call site (charged in-arm) and a
    /// fired node collapses to its least legal tile (focal branching only,
    /// J-A14). `choices` records argmax picks for the ground-truth pass.
    #[allow(clippy::too_many_arguments)]
    fn solve(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: Option<&mut BTreeMap<Vec<Domino>, Domino>>,
        census: Option<&mut Census>,
    ) -> Q {
        let mut choices = choices;
        let mut census = census;
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            let mass: Q = support.iter().map(|&(_, den)| unit(den)).sum();
            let banked = if winner.team() == self.focal.team() {
                mass
            } else {
                qi(0)
            };
            if self.hand_now(support[0].0, self.focal, obs).is_empty() {
                return banked;
            }
            return banked
                + self.solve(
                    support,
                    winner,
                    [Domino::ALL[0]; 4],
                    0,
                    obs,
                    choices.as_deref_mut(),
                    census.as_deref_mut(),
                );
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == self.focal {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let mut consider = legal;
            if legal.len() == 1 {
                if let Some(c) = census.as_deref_mut() {
                    c.forced += 1;
                }
            } else if let Some(c) = census.as_deref_mut() {
                c.call_sites += 1;
                let node = self.node_data(support, &tiles, k, obs);
                let t0 = Instant::now();
                let f0 = d0(self.masks, &node);
                let fs = d1_sym(self.masks, &node);
                let fw = d1_win(self.masks, &node);
                c.detector_ns += t0.elapsed().as_nanos();
                c.detector_calls += 1;
                if f0 {
                    c.fired_d0 += 1;
                }
                if fs {
                    c.fired_sym += 1;
                }
                if fw {
                    c.fired_win += 1;
                }
                if f0 || fs || fw {
                    c.fired_any += 1;
                    // Collapse: least legal tile only (value equal by
                    // Lemma J / Propositions J-0/J-1/J-win).
                    consider = DominoSet::single(legal.iter().next().expect("legal"));
                }
            }
            let mut best: Option<(Q, Domino)> = None;
            for a in consider.iter() {
                let mut tiles = tiles;
                tiles[k] = a;
                obs.push(a);
                let v = self.solve(
                    support,
                    leader,
                    tiles,
                    k + 1,
                    obs,
                    choices.as_deref_mut(),
                    census.as_deref_mut(),
                );
                obs.pop();
                if best.as_ref().map(|(bv, _)| v > *bv).unwrap_or(true) {
                    best = Some((v, a));
                }
            }
            let (v, a) = best.expect("legal move exists");
            if let Some(ch) = choices.as_deref_mut() {
                ch.insert(obs.clone(), a);
            }
            return v;
        }
        let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
        for &(wi, den) in support {
            let hand = self.hand_now(wi, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let nl = legal.len() as u128;
            for t in legal.iter() {
                by_tile.entry(t.index()).or_default().push((wi, den * nl));
            }
        }
        let mut sum = qi(0);
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += self.solve(
                &sup,
                leader,
                tiles,
                k + 1,
                obs,
                choices.as_deref_mut(),
                census.as_deref_mut(),
            );
            obs.pop();
        }
        sum
    }

    /// Per-world expected count under the extracted playbook, with an
    /// optional override at the entry node (one-deviation evaluation).
    #[allow(clippy::too_many_arguments)]
    fn eval_world(
        &self,
        wi: u32,
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &BTreeMap<Vec<Domino>, Domino>,
        override_first: Option<Domino>,
    ) -> Q {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            let inc = if winner.team() == self.focal.team() {
                qi(1)
            } else {
                qi(0)
            };
            if self.hand_now(wi, self.focal, obs).is_empty() {
                return inc;
            }
            return inc
                + self.eval_world(
                    wi,
                    winner,
                    [Domino::ALL[0]; 4],
                    0,
                    obs,
                    choices,
                    override_first,
                );
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == self.focal {
            let a = match override_first {
                Some(d) => d,
                None => *choices.get(obs.as_slice()).expect("record pooled"),
            };
            let mut tiles = tiles;
            tiles[k] = a;
            obs.push(a);
            let v = self.eval_world(wi, leader, tiles, k + 1, obs, choices, None);
            obs.pop();
            return v;
        }
        let hand = self.hand_now(wi, seat, obs);
        let legal = legal_plays(self.decl, hand, led);
        let share = q(1, legal.len() as i128);
        let mut sum = qi(0);
        for d in legal.iter() {
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            sum += share * self.eval_world(wi, leader, tiles, k + 1, obs, choices, override_first);
            obs.pop();
        }
        sum
    }

    /// Ground-truth pass (freeze 34): walks the pooled tree under the
    /// extracted playbook's reachable records, classifying every call site
    /// with support <= GT_SUPPORT_BOUND by one-deviation tie, and recording
    /// detector verdicts for the recall table. Fired nodes ALWAYS get the
    /// receipt (budgeted, J-A11/P-A16).
    #[allow(clippy::too_many_arguments)]
    fn ground_truth(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &mut Vec<Domino>,
        choices: &BTreeMap<Vec<Domino>, Domino>,
        gt: &mut GtTally,
    ) {
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct");
            let winner = trick.winner(self.decl);
            if self.hand_now(support[0].0, self.focal, obs).is_empty() {
                return;
            }
            self.ground_truth(support, winner, [Domino::ALL[0]; 4], 0, obs, choices, gt);
            return;
        }
        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));
        if seat == self.focal {
            let hand = self.hand_now(support[0].0, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            if legal.len() >= 2 {
                let node = self.node_data(support, &tiles, k, obs);
                let f0 = d0(self.masks, &node);
                let fs = d1_sym(self.masks, &node);
                let fw = d1_win(self.masks, &node);
                let fired = f0 || fs || fw;
                let in_bound = support.len() <= GT_SUPPORT_BOUND;
                if in_bound || (fired && gt.receipt_budget > 0) {
                    if fired && !in_bound {
                        gt.receipt_budget -= 1;
                    }
                    let tie =
                        self.one_deviation_tie(support, leader, tiles, k, obs, choices, legal);
                    if in_bound {
                        gt.classified += 1;
                        if tie {
                            gt.tied += 1;
                            if f0 {
                                gt.hit_d0 += 1;
                            }
                            if fs {
                                gt.hit_sym += 1;
                            }
                            if fw {
                                gt.hit_win += 1;
                            }
                            if fired {
                                gt.hit_any += 1;
                            }
                        }
                    }
                    assert!(
                        !fired || tie,
                        "J-A11 soundness receipt: a fired node must be one-deviation tied (stop-and-report)"
                    );
                } else if fired {
                    gt.receipt_skipped += 1;
                }
            }
            let picked = *choices.get(obs.as_slice()).expect("record pooled");
            let mut tiles = tiles;
            tiles[k] = picked;
            obs.push(picked);
            self.ground_truth(support, leader, tiles, k + 1, obs, choices, gt);
            obs.pop();
            return;
        }
        let mut by_tile: BTreeMap<usize, Vec<(u32, u128)>> = BTreeMap::new();
        for &(wi, den) in support {
            let hand = self.hand_now(wi, seat, obs);
            let legal = legal_plays(self.decl, hand, led);
            let nl = legal.len() as u128;
            for t in legal.iter() {
                by_tile.entry(t.index()).or_default().push((wi, den * nl));
            }
        }
        for (ti, sup) in by_tile {
            let d = Domino::from_index(ti).expect("tile");
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            self.ground_truth(&sup, leader, tiles, k + 1, obs, choices, gt);
            obs.pop();
        }
    }

    /// TIE iff every legal option's one-deviation value equals every other's
    /// in every world of the local support (argmax-indifference; a SUPERSET
    /// of exact deadness, J-A10 — the label travels with every number).
    #[allow(clippy::too_many_arguments)]
    fn one_deviation_tie(
        &self,
        support: &[(u32, u128)],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        obs: &[Domino],
        choices: &BTreeMap<Vec<Domino>, Domino>,
        legal: DominoSet,
    ) -> bool {
        let opts: Vec<Domino> = legal.iter().collect();
        for &(wi, _) in support {
            let mut base: Option<Q> = None;
            for &a in &opts {
                let mut o = obs.to_owned();
                let v = self.eval_world(wi, leader, tiles, k, &mut o, choices, Some(a));
                match &base {
                    None => base = Some(v),
                    Some(b) => {
                        if v != *b {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

fn unit(den: u128) -> Q {
    q(1, i128::try_from(den).expect("den fits"))
}

/// Declared bounds (freeze 34): ground truth runs at call sites with
/// support <= this; fired nodes beyond it still get the soundness receipt
/// under a per-root budget (a stop is declared, never silent).
const GT_SUPPORT_BOUND: usize = 400;
const RECEIPT_BUDGET: usize = 50;

/// Freeze 41 (DS-A30): the checkpoint record format identity. Every record
/// carries this digest; a record whose digest differs from the running
/// freeze set is CORRUPT, not stale, and the whole cache is discarded —
/// never partially reused.
const FREEZE_DIGEST: &str = "S6c-ckpt-v1|freezes-32-35|gt400|rb50|fields=17|unit=label+lead";

#[derive(Default)]
struct GtTally {
    classified: usize,
    tied: usize,
    hit_d0: usize,
    hit_sym: usize,
    hit_win: usize,
    hit_any: usize,
    receipt_budget: usize,
    receipt_skipped: usize,
}

/// Per-(root,lead) unit result. Everything except the wall times is
/// execution-schedule-invariant: exact integer counts whose values cannot
/// depend on worker count or resume history.
struct UnitResult {
    plain_ms: u128,
    dead_ms: u128,
    census: Census,
    gt: GtTally,
}

fn unit_key(label: &str, lead: Domino) -> String {
    let mut k: String = label
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    k.push_str("_lead");
    k.push_str(&lead.hi().value().to_string());
    k.push_str(&lead.lo().value().to_string());
    k
}

/// Checkpoints share the gitignored store/ cache discipline (the S5j
/// endgame store's rule: a cache, never a result; results files are the
/// only quotable artifact).
fn ckpt_dir() -> std::path::PathBuf {
    if std::path::Path::new("walt-factory").exists() {
        std::path::PathBuf::from("walt-factory/store/deadness_ckpt")
    } else {
        std::path::PathBuf::from("store/deadness_ckpt")
    }
}

fn save_ckpt(key: &str, u: &UnitResult) {
    let dir = ckpt_dir();
    std::fs::create_dir_all(&dir).expect("ckpt dir");
    let mut s = String::new();
    use std::fmt::Write as _;
    let w = &mut s;
    let _ = writeln!(w, "digest={FREEZE_DIGEST}");
    let _ = writeln!(w, "plain_ms={}", u.plain_ms);
    let _ = writeln!(w, "dead_ms={}", u.dead_ms);
    let _ = writeln!(w, "call_sites={}", u.census.call_sites);
    let _ = writeln!(w, "forced={}", u.census.forced);
    let _ = writeln!(w, "fired_d0={}", u.census.fired_d0);
    let _ = writeln!(w, "fired_sym={}", u.census.fired_sym);
    let _ = writeln!(w, "fired_win={}", u.census.fired_win);
    let _ = writeln!(w, "fired_any={}", u.census.fired_any);
    let _ = writeln!(w, "detector_ns={}", u.census.detector_ns);
    let _ = writeln!(w, "detector_calls={}", u.census.detector_calls);
    let _ = writeln!(w, "gt_classified={}", u.gt.classified);
    let _ = writeln!(w, "gt_tied={}", u.gt.tied);
    let _ = writeln!(w, "gt_hit_d0={}", u.gt.hit_d0);
    let _ = writeln!(w, "gt_hit_sym={}", u.gt.hit_sym);
    let _ = writeln!(w, "gt_hit_win={}", u.gt.hit_win);
    let _ = writeln!(w, "gt_hit_any={}", u.gt.hit_any);
    let _ = writeln!(w, "gt_receipt_skipped={}", u.gt.receipt_skipped);
    let _ = writeln!(w, "complete=yes");
    // Write-then-rename so a killed run never leaves a torn checkpoint.
    let tmp = dir.join(format!("{key}.tmp"));
    std::fs::write(&tmp, &s).expect("ckpt write");
    std::fs::rename(&tmp, dir.join(format!("{key}.txt"))).expect("ckpt rename");
}

fn load_ckpt(key: &str) -> Option<UnitResult> {
    let s = std::fs::read_to_string(ckpt_dir().join(format!("{key}.txt"))).ok()?;
    let mut map = std::collections::BTreeMap::new();
    for line in s.lines() {
        let (k, v) = line.split_once('=')?;
        map.insert(k.to_string(), v.to_string());
    }
    if map.get("complete").map(String::as_str) != Some("yes")
        || map.get("digest").map(String::as_str) != Some(FREEZE_DIGEST)
    {
        return None;
    }
    let gu = |k: &str| -> u128 { map.get(k).and_then(|v| v.parse().ok()).expect("ckpt field") };
    let gs = |k: &str| -> usize { map.get(k).and_then(|v| v.parse().ok()).expect("ckpt field") };
    Some(UnitResult {
        plain_ms: gu("plain_ms"),
        dead_ms: gu("dead_ms"),
        census: Census {
            call_sites: gs("call_sites"),
            forced: gs("forced"),
            fired_d0: gs("fired_d0"),
            fired_sym: gs("fired_sym"),
            fired_win: gs("fired_win"),
            fired_any: gs("fired_any"),
            detector_ns: gu("detector_ns"),
            detector_calls: gs("detector_calls"),
        },
        gt: GtTally {
            classified: gs("gt_classified"),
            tied: gs("gt_tied"),
            hit_d0: gs("gt_hit_d0"),
            hit_sym: gs("gt_hit_sym"),
            hit_win: gs("gt_hit_win"),
            hit_any: gs("gt_hit_any"),
            receipt_budget: 0,
            receipt_skipped: gs("gt_receipt_skipped"),
        },
    })
}

/// DS-A30(ii): a record whose digest differs from the running freeze set is
/// corrupt, not stale — the cache is discarded ENTIRE, never partially
/// reused. Stray .tmp files (a kill mid-write) are also removed.
fn validate_cache() {
    let dir = ckpt_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    let mut corrupt = false;
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().map(|x| x == "tmp").unwrap_or(false) {
            let _ = std::fs::remove_file(&p);
            continue;
        }
        let ok = std::fs::read_to_string(&p)
            .map(|s| s.lines().any(|l| l == format!("digest={FREEZE_DIGEST}")))
            .unwrap_or(false);
        if !ok {
            corrupt = true;
        }
    }
    if corrupt {
        eprintln!("[cache] digest mismatch: discarding checkpoint cache entire (DS-A30)");
        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// DS-A30(iii)'s validation comparator: every execution-schedule-invariant
/// field, and no wall-clock field.
fn nontiming_eq(a: &UnitResult, b: &UnitResult) -> bool {
    a.census.call_sites == b.census.call_sites
        && a.census.forced == b.census.forced
        && a.census.fired_d0 == b.census.fired_d0
        && a.census.fired_sym == b.census.fired_sym
        && a.census.fired_win == b.census.fired_win
        && a.census.fired_any == b.census.fired_any
        && a.census.detector_calls == b.census.detector_calls
        && a.gt.classified == b.gt.classified
        && a.gt.tied == b.gt.tied
        && a.gt.hit_d0 == b.gt.hit_d0
        && a.gt.hit_sym == b.gt.hit_sym
        && a.gt.hit_win == b.gt.hit_win
        && a.gt.hit_any == b.gt.hit_any
        && a.gt.receipt_skipped == b.gt.receipt_skipped
}

/// One (root, lead) unit, exactly the adjudicated per-lead sequence:
/// plain arm (timed) -> detector arm (timed, detector charged in-arm,
/// J-A13) -> J-A14 bit-exact assert -> choices solve -> ground-truth walk.
/// Both timed arms run sequentially inside one worker thread, so under
/// W-way concurrency their contention is common-mode.
fn run_unit(root: &Root, lead: Domino) -> UnitResult {
    let masks = Masks::new(root.decl);
    let wk = Walker {
        decl: root.decl,
        focal: root.focal,
        worlds: &root.worlds,
        masks: &masks,
    };
    let support: Vec<(u32, u128)> = (0..root.worlds.len() as u32).map(|i| (i, 1)).collect();
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = lead;
    let tp = Instant::now();
    let mut obs = vec![lead];
    let q_plain = wk.solve(&support, root.focal, tiles, 1, &mut obs, None, None);
    let plain_ms = tp.elapsed().as_millis();
    let td = Instant::now();
    let mut census = Census::default();
    let mut obs = vec![lead];
    let q_dead = wk.solve(
        &support,
        root.focal,
        tiles,
        1,
        &mut obs,
        None,
        Some(&mut census),
    );
    let dead_ms = td.elapsed().as_millis();
    assert_eq!(q_plain, q_dead, "J-A14: V and root Q bit-exact across arms");
    let mut choices = BTreeMap::new();
    let mut obs = vec![lead];
    let _ = wk.solve(
        &support,
        root.focal,
        tiles,
        1,
        &mut obs,
        Some(&mut choices),
        None,
    );
    let mut gt = GtTally {
        receipt_budget: RECEIPT_BUDGET,
        ..Default::default()
    };
    let mut obs = vec![lead];
    wk.ground_truth(&support, root.focal, tiles, 1, &mut obs, &choices, &mut gt);
    UnitResult {
        plain_ms,
        dead_ms,
        census,
        gt,
    }
}

fn main() {
    let t0 = Instant::now();
    let mut out = String::new();
    use std::fmt::Write as _;
    let w = &mut out;
    let _ = writeln!(w, "walt decision-deadness probe (S6c) — exploratory tier");
    let _ = writeln!(w, "rulings: J-A1..J-A18, Lemma J, Propositions J-0/J-1/J-win (walt/CENSUS-RULINGS.md 2026-08-12); design walt/DEADNESS-PROBE.md; runner rulings DS-A29..DS-A36 (2026-08-13); freezes 1-40 in force, runner freezes 41-43 (checkpoint format digest; canonical unit order; sequential timing rung)");
    let _ = writeln!(
        w,
        "regenerate (cold, from an empty cache — DS-A31(iii)): rm -rf store/deadness_ckpt && cargo run --release -p walt-factory --example deadness_probe"
    );
    let _ = writeln!(w, "preconditions (DS-A29, asserted by construction): every stop criterion is a deterministic count (support bound, receipt budget) and never wall-clock; no clock, RNG or environment value enters any decision; workers share no mutable state on which any reported number depends (per-unit solver state only); exact rational arithmetic throughout. All counts, receipts, recall fractions and V/Q values below are therefore execution-schedule-invariant.");
    let _ = writeln!(w);
    let _ = writeln!(w, "TYPING (J-A1, mandatory): forced (|legal|=1, no decision, own column, never in any dead fraction) / decision-dead (all policies value-identical — THE object) / dominant (one Pareto-optimal vector). S6b's singleton frontiers are DOMINANCE, not deadness; the seventh specimen (108 resolved decisions) is the proof the two differ. No sentence here presents a singleton-frontier count as a deadness count.");
    let _ = writeln!(w, "TAGS (J-A3): D0 and D1-sym verdicts survive count-free AND trick-plus-count (Lemma J(c), Proposition J-1 — the count guard lifts E-A2 for the transposition transport). D1-win verdicts are COUNT-FREE ONLY and are void wholesale the instant count re-enters (Proposition J-win, E-A2). The count guard (Jason, binding) is a firing conjunct of every detector.");
    let _ = writeln!(w, "DENOMINATORS (J-A10): the one-deviation tie set is argmax-indifference, a SUPERSET of exact deadness; recall against it UNDERSTATES the detector. Ground truth runs at call sites with support <= {GT_SUPPORT_BOUND} (declared); fired nodes beyond the bound receive the soundness receipt under a per-root budget of {RECEIPT_BUDGET} (stops declared).");
    let _ = writeln!(w);

    // Roots: the three S6a/S6b grade-3 coordinates and the S5h n=4 receipt
    // rung (eligible = bidder leads trick 4). n=5 is a DECLARED STOP: the
    // full n=5 fiber (756,756 worlds) was never solved cold in S5h either;
    // no number is reported for it (P-A16).
    let npop3: u128 = 7 * binom(28, 12) * binom(12, 3);
    let g3: Vec<Root> = (0..3u128)
        .map(|i| grade3_root((i * 1_299_709) % npop3))
        .collect();
    let n4 = receipt_rung_roots();
    let _ = writeln!(w, "roots: 3 grade-3 fabricated void-free coordinates (S6a freeze 25 decimation, base declination) x their leads; {} eligible n=4 receipt-rung coordinates (S5h corpus, void-free typing, bidder leads trick 4) x their leads. n=5: DECLARED STOP, not measured.", n4.len());
    let _ = writeln!(w);

    // Canonical unit list: (root, lead) in declaration order. Assembly
    // below always walks this order, so every count in the results file is
    // independent of worker count, completion order, and resume history.
    let roots: Vec<Root> = g3.into_iter().chain(n4).collect();
    let units: Vec<(usize, Domino)> = roots
        .iter()
        .enumerate()
        .flat_map(|(ri, r)| r.leads.iter().map(|l| (ri, l)).collect::<Vec<_>>())
        .collect();
    // Freeze 43 (DS-A33): the sequential timing rung — the ONLY quotable
    // timing instrument. Selection by rule, declared here before any
    // parallel pass and never by result: the FIRST grade-3 unit and the
    // FIRST n=4 unit in canonical order. W = 1, one uninterrupted process,
    // both arms re-run; the control is the plain arm of this same pass,
    // never S5h's recorded numbers (P-A19).
    if std::env::args().any(|a| a == "timing-rung") {
        run_timing_rung(&roots, &units);
        return;
    }

    validate_cache();
    let pending: Vec<(usize, Domino)> = units
        .iter()
        .copied()
        .filter(|(ri, lead)| load_ckpt(&unit_key(&roots[*ri].label, *lead)).is_none())
        .collect();
    let computed_keys: std::collections::BTreeSet<String> = pending
        .iter()
        .map(|(ri, lead)| unit_key(&roots[*ri].label, *lead))
        .collect();
    let resumed = units.len() - pending.len();
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .saturating_sub(2)
        .max(1)
        .min(pending.len().max(1));
    if !pending.is_empty() {
        let next = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if i >= pending.len() {
                        break;
                    }
                    let (ri, lead) = pending[i];
                    let root = &roots[ri];
                    eprintln!(
                        "[progress] {} lead {}{} ({}/{})",
                        root.label,
                        lead.hi().value(),
                        lead.lo().value(),
                        i + 1,
                        pending.len()
                    );
                    let u = run_unit(root, lead);
                    save_ckpt(&unit_key(&root.label, lead), &u);
                });
            }
        });
    }
    // DS-A30(iii): the cache is never an authority. A resumed run re-runs a
    // declared sample of loaded units — the FIRST loaded unit in canonical
    // order — and asserts non-timing equality before quoting anything.
    let mut resume_line = String::from("resume-validation: n/a (fresh run, nothing loaded)");
    if resumed > 0 {
        let (ri, lead) = units
            .iter()
            .copied()
            .find(|(ri, lead)| !computed_keys.contains(&unit_key(&roots[*ri].label, *lead)))
            .expect("a loaded unit exists");
        let key = unit_key(&roots[ri].label, lead);
        let fresh = run_unit(&roots[ri], lead);
        let loaded = load_ckpt(&key).expect("loaded unit re-readable");
        assert!(
            nontiming_eq(&fresh, &loaded),
            "DS-A30(iii)/DS-A36: recomputation of loaded unit {key} disagrees with its checkpoint — checkpointing defect, stop-and-report"
        );
        resume_line = format!(
            "resume-validation: PASS (unit {key} recomputed; all non-timing fields bit-equal to its loaded checkpoint)"
        );
    }

    // -- DETERMINISTIC BLOCK (DS-A36): byte-identical across fresh,
    // resumed, and any worker count; assembled in canonical unit order. --
    let _ = writeln!(w, "== DETERMINISTIC BLOCK (DS-A36: byte-identical across fresh/resumed runs and any worker count; canonical unit order) ==");
    let mut grand = Census::default();
    let mut grand_gt = GtTally::default();
    let mut timing_rows: Vec<String> = Vec::new();
    let (mut run_ns, mut run_calls): (u128, usize) = (0, 0);
    for (ri, lead) in units.iter().copied() {
        let root = &roots[ri];
        let key = unit_key(&root.label, lead);
        let u = load_ckpt(&key).expect("unit checkpoint present");
        let (census, gt) = (&u.census, &u.gt);
        let _ = writeln!(
            w,
            "{} lead={}{}: call sites {} forced {} fired[d0={} sym={} win={} any={}]; gt(classified {} tied {}; hits d0={} sym={} win={} any={}; receipts-beyond-bound skipped {})",
            root.label,
            lead.hi().value(),
            lead.lo().value(),
            census.call_sites,
            census.forced,
            census.fired_d0,
            census.fired_sym,
            census.fired_win,
            census.fired_any,
            gt.classified,
            gt.tied,
            gt.hit_d0,
            gt.hit_sym,
            gt.hit_win,
            gt.hit_any,
            gt.receipt_skipped
        );
        let this_run = computed_keys.contains(&key);
        timing_rows.push(format!(
            "{} lead={}{}: plain {} ms, detector-arm {} ms [{}]",
            root.label,
            lead.hi().value(),
            lead.lo().value(),
            u.plain_ms,
            u.dead_ms,
            if this_run {
                "this process"
            } else {
                "inherited from a prior process — not one measurement with the rows above (DS-A31(iv))"
            }
        ));
        if this_run {
            run_ns += census.detector_ns;
            run_calls += census.detector_calls;
        }
        grand.call_sites += census.call_sites;
        grand.forced += census.forced;
        grand.fired_d0 += census.fired_d0;
        grand.fired_sym += census.fired_sym;
        grand.fired_win += census.fired_win;
        grand.fired_any += census.fired_any;
        grand.detector_ns += census.detector_ns;
        grand.detector_calls += census.detector_calls;
        grand_gt.classified += gt.classified;
        grand_gt.tied += gt.tied;
        grand_gt.hit_d0 += gt.hit_d0;
        grand_gt.hit_sym += gt.hit_sym;
        grand_gt.hit_win += gt.hit_win;
        grand_gt.hit_any += gt.hit_any;
        grand_gt.receipt_skipped += gt.receipt_skipped;
    }
    let _ = writeln!(w);
    let _ = writeln!(
        w,
        "TOTALS: call sites {} (forced nodes, separate column: {}); fired d0={} sym={} win={} any={}; detector calls {}",
        grand.call_sites,
        grand.forced,
        grand.fired_d0,
        grand.fired_sym,
        grand.fired_win,
        grand.fired_any,
        grand.detector_calls
    );
    let _ = writeln!(
        w,
        "RECALL vs the one-deviation tie denominator (classified {} sites, {} tied): d0 {}/{} | sym {}/{} | win {}/{} | any {}/{} — exact fractions, labels per J-A10; a recall against this denominator UNDERSTATES the detector. Receipts beyond the support bound skipped (declared stop): {}",
        grand_gt.classified,
        grand_gt.tied,
        grand_gt.hit_d0,
        grand_gt.tied,
        grand_gt.hit_sym,
        grand_gt.tied,
        grand_gt.hit_win,
        grand_gt.tied,
        grand_gt.hit_any,
        grand_gt.tied,
        grand_gt.receipt_skipped
    );
    let _ = writeln!(w, "coverage is TRAVERSAL- and MEMOIZATION-RELATIVE (J-A15): these fractions are inventory statistics of this walk, not properties of the game, and a node fraction is never a cost fraction — the sequential timing rung (freeze 43) is the only quotable cost instrument.");
    let _ = writeln!(w);
    let _ = writeln!(w, "THE FENCE (J-A17, verbatim): A deadness verdict says: from this node, under the declared field, every information-consistent focal policy has the identical value. It is not a similarity claim and not a tolerance claim — nothing here supports 'about the same' for any tolerance, and delta-similarity remains future mathematics with its own rulings pending. It is not a partition: the dead/live split is a response-equality object and v0.4 §12.4 bars using it as a solver's state partition. UNKNOWN is never evidence of liveness — the detectors are one-sided by construction and their misses are lawful. Each verdict carries the valuations it survives (J-A3); a D1-win verdict is void wholesale the instant count re-enters (E-A2). Deadness is relative to a field that does not condition on focal's tile identity: against an opponent who reads discards, the choice signals and the verdict does not transfer (§7.7). It is not relative to any world, belief or support — the conditions are functions of the focal information state and quantify over the whole live set — so it is one of the few objects in this file that crosses the Φ(C) ⊊ Φ(C₀) gap intact. No runtime claim follows from a coverage fraction; the harvest ratio is the only cost statement and it is coordinate- and traversal-relative.");
    let _ = writeln!(w);

    // -- TIMING BLOCK (DS-A31/DS-A32): recordable, never quotable. --
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let _ = writeln!(w, "== TIMING BLOCK (DS-A31/DS-A32: recordable, never quotable; the sequential timing rung is the only quotable timing instrument) ==");
    let _ = writeln!(
        w,
        "provenance: {}; checkpoint digest {FREEZE_DIGEST}; {} units loaded from checkpoint, {} computed by this process; W={workers} worker threads, {cores} cores, {profile} build. A resumed run inherits counts and receipts freely and inherits NO quotable timing at all (DS-A31).",
        if resumed == 0 { "FRESH" } else { "RESUMED" },
        resumed,
        pending.len()
    );
    let _ = writeln!(w, "{resume_line}");
    let _ = writeln!(w, "all wall-clock figures below are CONTENDED(W={workers}): these timings are biased in favour of the detector arm and are not the dividend (DS-A32 — contention discounts the detector's cache-resident work relative to the memory-heavy solve). Never compared with any sequential figure.");
    for r in &timing_rows {
        let _ = writeln!(w, "{r}");
    }
    let _ = writeln!(
        w,
        "detector cost, this process's units only: {} ns over {} calls ({} ns/call) [CONTENDED(W={workers}), same bar (J-A12/DS-A32)]",
        run_ns,
        run_calls,
        if run_calls > 0 {
            run_ns / run_calls as u128
        } else {
            0
        }
    );
    let _ = writeln!(
        w,
        "total: {} ms this invocation (wall; excludes checkpointed prior invocations)",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(w, "run complete: yes");
    print!("{out}");
    std::fs::write("walt-factory/results/deadness_2026-08-12.txt", &out)
        .or_else(|_| std::fs::write("results/deadness_2026-08-12.txt", &out))
        .expect("results written");
}

/// Freeze 43 (DS-A33): the sequential timing rung. Both arms of each rung
/// unit re-run at W = 1 in this single uninterrupted process; the control
/// is the plain arm of this same pass. Checkpoints are neither read nor
/// written here — rung timings never mix with another process's.
fn run_timing_rung(roots: &[Root], units: &[(usize, Domino)]) {
    let mut picks: Vec<(usize, Domino)> = Vec::new();
    for prefix in ["g3", "n4"] {
        if let Some(&(ri, lead)) = units
            .iter()
            .find(|(ri, _)| roots[*ri].label.starts_with(prefix))
        {
            picks.push((ri, lead));
        }
    }
    let mut out = String::new();
    use std::fmt::Write as _;
    let w = &mut out;
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let _ = writeln!(w, "S6c sequential timing rung (freeze 43; DS-A33) — the only quotable timing instrument. Selection rule (declared, never by result): the first grade-3 unit and the first n=4 unit in canonical order. W=1, single uninterrupted process, {cores} cores, {profile} build; control = the plain arm re-run in this same pass (S5h figures are background context only, P-A19).");
    for (ri, lead) in picks {
        let root = &roots[ri];
        eprintln!(
            "[rung] {} lead {}{}",
            root.label,
            lead.hi().value(),
            lead.lo().value()
        );
        let u = run_unit(root, lead);
        let _ = writeln!(
            w,
            "{} lead={}{}: plain {} ms, detector-arm {} ms; detector {} ns over {} calls ({} ns/call); call sites {} fired any={}",
            root.label,
            lead.hi().value(),
            lead.lo().value(),
            u.plain_ms,
            u.dead_ms,
            u.census.detector_ns,
            u.census.detector_calls,
            if u.census.detector_calls > 0 {
                u.census.detector_ns / u.census.detector_calls as u128
            } else {
                0
            },
            u.census.call_sites,
            u.census.fired_any
        );
    }
    let _ = writeln!(w, "run complete: yes");
    print!("{out}");
    std::fs::write("walt-factory/results/deadness_rung_2026-08-13.txt", &out)
        .or_else(|_| std::fs::write("results/deadness_rung_2026-08-13.txt", &out))
        .expect("rung results written");
}
