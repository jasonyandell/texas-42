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

    fn node_data(&self, support: &[(u32, u128)], tiles: &[Domino; 4], k: usize, obs: &[Domino]) -> Node {
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
                    let tie = self.one_deviation_tie(support, leader, tiles, k, obs, choices, legal);
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
        obs: &Vec<Domino>,
        choices: &BTreeMap<Vec<Domino>, Domino>,
        legal: DominoSet,
    ) -> bool {
        let opts: Vec<Domino> = legal.iter().collect();
        for &(wi, _) in support {
            let mut base: Option<Q> = None;
            for &a in &opts {
                let mut o = obs.clone();
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

fn main() {
    let t0 = Instant::now();
    let mut out = String::new();
    use std::fmt::Write as _;
    let w = &mut out;
    let _ = writeln!(w, "walt decision-deadness probe (S6c) — exploratory tier");
    let _ = writeln!(w, "rulings: J-A1..J-A18, Lemma J, Propositions J-0/J-1/J-win (walt/CENSUS-RULINGS.md 2026-08-12); design walt/DEADNESS-PROBE.md; freezes 1-31 in force, new 32-35");
    let _ = writeln!(w, "regenerate: cargo run --release -p walt-factory --example deadness_probe");
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

    let mut grand = Census::default();
    let mut grand_gt = GtTally::default();
    let mut harvest_rows: Vec<String> = Vec::new();
    for root in g3.iter().chain(n4.iter()) {
        let masks = Masks::new(root.decl);
        let wk = Walker {
            decl: root.decl,
            focal: root.focal,
            worlds: &root.worlds,
            masks: &masks,
        };
        let support: Vec<(u32, u128)> = (0..root.worlds.len() as u32).map(|i| (i, 1)).collect();
        for lead in root.leads.iter() {
            eprintln!("[progress] {} lead {}{}", root.label, lead.hi().value(), lead.lo().value());
            let mut tiles = [Domino::ALL[0]; 4];
            tiles[0] = lead;
            // Plain arm (timed).
            let tp = Instant::now();
            let mut obs = vec![lead];
            let q_plain = wk.solve(&support, root.focal, tiles, 1, &mut obs, None, None);
            let plain_ms = tp.elapsed().as_millis();
            // Detector arm (timed; detector charged inside, J-A13).
            let td = Instant::now();
            let mut census = Census::default();
            let mut obs = vec![lead];
            let q_dead = wk.solve(&support, root.focal, tiles, 1, &mut obs, None, Some(&mut census));
            let dead_ms = td.elapsed().as_millis();
            assert_eq!(q_plain, q_dead, "J-A14: V and root Q bit-exact across arms");
            // Ground truth (choices first, then the classification walk).
            let mut choices = BTreeMap::new();
            let mut obs = vec![lead];
            let _ = wk.solve(&support, root.focal, tiles, 1, &mut obs, Some(&mut choices), None);
            let mut gt = GtTally {
                receipt_budget: RECEIPT_BUDGET,
                ..Default::default()
            };
            let mut obs = vec![lead];
            wk.ground_truth(&support, root.focal, tiles, 1, &mut obs, &choices, &mut gt);
            harvest_rows.push(format!(
                "{} lead={}{}: plain {} ms, detector-arm {} ms; call sites {} forced {} fired[d0={} sym={} win={} any={}]; gt(classified {} tied {}; hits d0={} sym={} win={} any={}; receipts-beyond-bound skipped {})",
                root.label,
                lead.hi().value(),
                lead.lo().value(),
                plain_ms,
                dead_ms,
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
            ));
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
    }
    for r in &harvest_rows {
        let _ = writeln!(w, "{r}");
    }
    let _ = writeln!(w);
    let _ = writeln!(
        w,
        "TOTALS: call sites {} (forced nodes, separate column: {}); fired d0={} sym={} win={} any={}; detector cost {} ns over {} calls ({} ns/call)",
        grand.call_sites,
        grand.forced,
        grand.fired_d0,
        grand.fired_sym,
        grand.fired_win,
        grand.fired_any,
        grand.detector_ns,
        grand.detector_calls,
        if grand.detector_calls > 0 {
            grand.detector_ns / grand.detector_calls as u128
        } else {
            0
        }
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
    let _ = writeln!(w, "coverage is TRAVERSAL- and MEMOIZATION-RELATIVE (J-A15): these fractions are inventory statistics of this walk, not properties of the game, and a node fraction is never a cost fraction — the harvest arm times above are the only cost statement.");
    let _ = writeln!(w);
    let _ = writeln!(w, "THE FENCE (J-A17, verbatim): A deadness verdict says: from this node, under the declared field, every information-consistent focal policy has the identical value. It is not a similarity claim and not a tolerance claim — nothing here supports 'about the same' for any tolerance, and delta-similarity remains future mathematics with its own rulings pending. It is not a partition: the dead/live split is a response-equality object and v0.4 §12.4 bars using it as a solver's state partition. UNKNOWN is never evidence of liveness — the detectors are one-sided by construction and their misses are lawful. Each verdict carries the valuations it survives (J-A3); a D1-win verdict is void wholesale the instant count re-enters (E-A2). Deadness is relative to a field that does not condition on focal's tile identity: against an opponent who reads discards, the choice signals and the verdict does not transfer (§7.7). It is not relative to any world, belief or support — the conditions are functions of the focal information state and quantify over the whole live set — so it is one of the few objects in this file that crosses the Φ(C) ⊊ Φ(C₀) gap intact. No runtime claim follows from a coverage fraction; the harvest ratio is the only cost statement and it is coordinate- and traversal-relative.");
    let _ = writeln!(w, "total: {} ms", t0.elapsed().as_millis());
    let _ = writeln!(w, "run complete: yes");
    print!("{out}");
    std::fs::write("walt-factory/results/deadness_2026-08-12.txt", &out)
        .or_else(|_| std::fs::write("results/deadness_2026-08-12.txt", &out))
        .expect("results written");
}
