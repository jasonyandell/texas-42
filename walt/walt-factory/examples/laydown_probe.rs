//! walt lay-down probe — Theorem LD's catalogue and the four-laydown search.
//! EXPLORATORY TIER.
//!
//! Rulings: LD-A1..LD-A10 (walt/CENSUS-RULINGS.md, 2026-08-14) — Theorem LD
//! (H is a lay down under PipTrump(p) iff (L1) the top run above the best
//! outstanding trump is at least as long as the outstanding set, and (L2)
//! threat(d) ⊆ T ∪ H for every non-trump d ∈ H), its five corollaries
//! (every lay down holds at least four trumps; a lay down sweeps all 42
//! points by arithmetic, E-A2 not engaged), and freeze 48 (enumeration
//! orders, catalogue record format, phase-2 search order).
//!
//! Phase 1: the catalogue — (L1) ∧ (L2) over all C(28,7) hands × 7 pip
//! declarations. Receipts: (LD-R1) the PipTrump(6) count asserted against
//! LD-A9(ii)'s independently derived closed form, 301; (LD-R2) every
//! freeze-47 T1-draw member present under its own declaration and the
//! containment STRICT per declaration; (LD-R3) for the first catalogue
//! member in canonical order at each t, the sweep asserted by exhaustive
//! play of the LD plan against ALL field behaviours at a declared
//! reduced-grade analogue (grade 3). Not a receipt (PG-A8): (L1)/(L2)
//! holding on a hand selected because they hold.
//!
//! Phase 2: the four-laydown search — four pairwise-disjoint catalogue
//! hands, distinct declarations, partitioning the 28 tiles; exhaustive from
//! the seven full-suit hands (LD-A6(D5): exactly one full suit is present in
//! any witness). Also the maximum number of pairwise-disjoint lay downs
//! completable to a deal. Both outcomes pre-declared results (F7).
//!
//! FENCES (LD-A10): the four-laydown question is COMBINATORIAL, not a
//! situation — only the bid winner declares, so four lay downs are never
//! realised together; the question is whether the tiles can be partitioned
//! so each hand WOULD sweep if it were the one to declare and lead. The
//! T1-A12 implementation-versus-corpus risk carries in full. No promotion:
//! not for bidding, not for real opponents, not for DoublesTrump/NoTrump.
//!
//! No floats. Regenerate:
//! `cargo run --release -p walt-factory --example laydown_probe`

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::Instant;

use walt_core::{legal_plays, Decl, Domino, DominoSet, Pip, Seat, Trick};

const LD_DIGEST: &str = "LD-catalogue-v1|freeze-48|tests=L1-L2|order=pip-asc,hand-lex-asc";

fn tile(d: Domino) -> String {
    format!("{}{}", d.hi().value(), d.lo().value())
}

fn set_str(s: DominoSet) -> String {
    let v: Vec<String> = s.iter().map(tile).collect();
    v.join(" ")
}

fn mask_of(s: DominoSet) -> u32 {
    s.bits()
}

fn set_of(mask: u32) -> DominoSet {
    (0..28)
        .filter(|i| mask & (1 << i) != 0)
        .map(|i| Domino::from_index(i).expect("domino"))
        .collect()
}

fn trump_rank_order(decl: Decl) -> Vec<Domino> {
    let mut v: Vec<Domino> = decl.called_set().iter().collect();
    v.sort_by_key(|d| std::cmp::Reverse(decl.rank(*d)));
    v
}

fn gosper_next(v: u32) -> u32 {
    let c = v & v.wrapping_neg();
    let r = v + c;
    (((r ^ v) >> 2) / c) | r
}

/// Per-declaration precomputation for the two bitset tests.
struct DeclTest {
    decl: Decl,
    called_mask: u32,
    /// rank-ordered trump masks, descending.
    order_masks: Vec<u32>,
    /// threat(d) as a mask, per domino index.
    threat_masks: [u32; 28],
}

impl DeclTest {
    fn new(pip: u8) -> DeclTest {
        let decl = Decl::PipTrump(Pip::new(pip).expect("pip"));
        let called_mask = mask_of(decl.called_set());
        let order_masks = trump_rank_order(decl)
            .iter()
            .map(|d| 1u32 << d.index())
            .collect();
        let mut threat_masks = [0u32; 28];
        for d in Domino::ALL {
            threat_masks[d.index()] = mask_of(decl.threat(d));
        }
        DeclTest {
            decl,
            called_mask,
            order_masks,
            threat_masks,
        }
    }

    /// Theorem LD: (L1) ∧ (L2). Returns (is_laydown, t, run_len).
    fn test(&self, hand: u32) -> (bool, u32, usize) {
        let ht = hand & self.called_mask;
        let t = ht.count_ones();
        if t < 4 {
            return (false, t, 0); // Corollary: every lay down holds >= 4 trumps
        }
        // run = number of top-ranked trumps in H before the first outstanding.
        let mut run = 0usize;
        for m in &self.order_masks {
            if hand & m != 0 {
                run += 1;
            } else {
                break;
            }
        }
        let outstanding = 7 - t as usize;
        if run < outstanding {
            return (false, t, run); // (L1) fails
        }
        // (L2): every non-trump's threat lies inside T ∪ H.
        let mut nt = hand & !self.called_mask;
        let cover = self.called_mask | hand;
        while nt != 0 {
            let i = nt.trailing_zeros() as usize;
            nt &= nt - 1;
            if self.threat_masks[i] & !cover != 0 {
                return (false, t, run);
            }
        }
        (true, t, run)
    }
}

/// The freeze-47 T1-draw family, per declaration: top-t run + chosen
/// non-trump doubles (the LD-R2 inner class).
fn t1_family(pip: u8) -> Vec<u32> {
    let decl = Decl::PipTrump(Pip::new(pip).expect("pip"));
    let order = trump_rank_order(decl);
    let doubles: Vec<Domino> = (0..=6u8)
        .filter(|j| *j != pip)
        .map(|j| Domino::new(Pip::new(j).expect("pip"), Pip::new(j).expect("pip")))
        .collect();
    let mut out = Vec::new();
    for t in (4..=7usize).rev() {
        let need = 7 - t;
        let base: u32 = order.iter().take(t).map(|d| 1u32 << d.index()).sum();
        let n = doubles.len();
        // all `need`-subsets of the six doubles
        let mut idx: Vec<usize> = (0..need).collect();
        loop {
            let mut m = base;
            for i in &idx {
                m |= 1 << doubles[*i].index();
            }
            out.push(m);
            if need == 0 {
                break;
            }
            let mut i = need;
            let mut done = false;
            loop {
                if i == 0 {
                    done = true;
                    break;
                }
                i -= 1;
                if idx[i] != n - need + i {
                    idx[i] += 1;
                    for j in i + 1..need {
                        idx[j] = idx[j - 1] + 1;
                    }
                    break;
                }
            }
            if done {
                break;
            }
        }
    }
    out
}

// -- (LD-R3): the reduced-grade sweep check ---------------------------------

/// The LD plan as a deterministic, information-consistent move rule: while
/// any trump remains outside the hand among the live tiles, lead the highest
/// trump in hand; then cash — remaining trumps first, then non-trumps, in
/// canonical ascending order. (Following is forced to a unique choice only
/// when one tile is legal; as the leader throughout a sweep, the plan never
/// needs a follow rule — a non-leading state in the DFS below means the
/// sweep already failed.)
fn ld_plan(decl: Decl, hand: DominoSet, live_outside: DominoSet) -> Domino {
    let called = decl.called_set();
    if !live_outside.intersection(called).is_empty() {
        // draw: highest-ranked trump in hand
        return hand
            .intersection(called)
            .iter()
            .max_by_key(|d| decl.rank(*d))
            .expect("(L1) guarantees a trump to lead");
    }
    let trumps = hand.intersection(called);
    if !trumps.is_empty() {
        return trumps.iter().min_by_key(|d| d.index()).expect("nonempty");
    }
    hand.iter().min_by_key(|d| d.index()).expect("nonempty")
}

/// Exhaustive adversarial verification at a reduced coordinate: for every
/// world (split of the pool among three seats) and EVERY field behaviour,
/// the plan takes every trick. Returns the number of (world, behaviour)
/// leaves checked.
fn verify_sweep_reduced(decl: Decl, hand: DominoSet, pool: DominoSet, g: usize) -> u64 {
    // enumerate worlds: pool split g/g/g among S1,S2,S3
    let pool_v: Vec<Domino> = pool.iter().collect();
    let n = pool_v.len();
    assert_eq!(n, 3 * g);
    let full: u32 = (1 << n) - 1;
    let mut leaves: u64 = 0;
    let mut s1c: u32 = (1 << g) - 1;
    loop {
        let rem = full & !s1c;
        let mut s2c: u32;
        // enumerate g-subsets of rem (compact walk over set bits)
        let rem_pos: Vec<usize> = (0..n).filter(|i| rem & (1 << i) != 0).collect();
        let m2 = rem_pos.len();
        let mut c2: u32 = (1 << g) - 1;
        loop {
            s2c = 0;
            let mut mm = c2;
            while mm != 0 {
                let b = mm.trailing_zeros() as usize;
                s2c |= 1 << rem_pos[b];
                mm &= mm - 1;
            }
            let s3c = rem & !s2c;
            let to_set = |m: u32| -> DominoSet {
                (0..n)
                    .filter(|i| m & (1 << i) != 0)
                    .map(|i| pool_v[i])
                    .collect()
            };
            let mut hands = [hand, to_set(s1c), to_set(s2c), to_set(s3c)];
            leaves += dfs_sweep(decl, &mut hands, Seat::S0, &mut Vec::new());
            if c2 == ((1u32 << g) - 1) << (m2 - g) {
                break;
            }
            c2 = gosper_next(c2);
            if c2 > (1 << m2) - 1 {
                break;
            }
        }
        if s1c == ((1u32 << g) - 1) << (n - g) {
            break;
        }
        s1c = gosper_next(s1c);
        if s1c > full {
            break;
        }
    }
    leaves
}

/// DFS over all field behaviours from a trick-leading state; asserts the
/// focal seat (S0) wins every trick. Returns leaves counted.
fn dfs_sweep(decl: Decl, hands: &mut [DominoSet; 4], leader: Seat, trail: &mut Vec<Domino>) -> u64 {
    assert_eq!(leader, Seat::S0, "the LD plan retains the lead throughout");
    if hands[0].is_empty() {
        return 1;
    }
    let live_outside = hands[1].union(hands[2]).union(hands[3]);
    let lead = ld_plan(decl, hands[0], live_outside);
    hands[0].remove(lead);
    let led = decl.led_context(lead);
    let mut leaves = 0u64;
    // three field seats, all legal choices each
    let l1: Vec<Domino> = legal_plays(decl, hands[1], Some(led)).iter().collect();
    for d1 in &l1 {
        hands[1].remove(*d1);
        let l2: Vec<Domino> = legal_plays(decl, hands[2], Some(led)).iter().collect();
        for d2 in &l2 {
            hands[2].remove(*d2);
            let l3: Vec<Domino> = legal_plays(decl, hands[3], Some(led)).iter().collect();
            for d3 in &l3 {
                hands[3].remove(*d3);
                let trick = Trick::new(Seat::S0, [lead, *d1, *d2, *d3]).expect("distinct");
                assert_eq!(
                    trick.winner(decl),
                    Seat::S0,
                    "(LD-R3) stop-and-report: the LD plan lost a trick at trail {:?} lead {} vs [{} {} {}]",
                    trail,
                    tile(lead),
                    tile(*d1),
                    tile(*d2),
                    tile(*d3)
                );
                trail.push(lead);
                leaves += dfs_sweep(decl, hands, Seat::S0, trail);
                trail.pop();
                hands[3].insert(*d3);
            }
            hands[2].insert(*d2);
        }
        hands[1].insert(*d1);
    }
    hands[0].insert(lead);
    leaves
}

/// The declared reduced-grade analogue (grade 3) of a catalogue member:
/// preserve the shape (run length capped, one outstanding trump iff the
/// original had any, first non-trump kept iff any), deterministic.
fn reduced_analogue(dt: &DeclTest, hand_mask: u32) -> (DominoSet, DominoSet) {
    let hand = set_of(hand_mask);
    let decl = dt.decl;
    let called = decl.called_set();
    let order = trump_rank_order(decl);
    let has_outstanding = hand.intersection(called).len() < 7;
    let non_trumps: Vec<Domino> = {
        let mut v: Vec<Domino> = hand.difference(called).iter().collect();
        v.sort_by_key(|d| d.index());
        v
    };
    let g = 3usize;
    let mut h = DominoSet::EMPTY;
    let mut pool = DominoSet::EMPTY;
    if has_outstanding {
        // hand: top 2 run trumps (+ first non-trump if any, else 3rd trump);
        // pool: the 3rd-ranked trump as the single outstanding + fillers.
        h.insert(order[0]);
        h.insert(order[1]);
        if let Some(nt) = non_trumps.first() {
            h.insert(*nt);
        } else {
            h.insert(order[2]);
        }
        let outstanding = if h.contains(order[2]) {
            order[3]
        } else {
            order[2]
        };
        pool.insert(outstanding);
    } else {
        h.insert(order[0]);
        h.insert(order[1]);
        h.insert(order[2]);
    }
    // fill pool to 3g with the lowest-index tiles outside h, outside trumps,
    // and outside any KEPT non-trump's threat (preserving (L2) exactly).
    let mut forbidden: u32 = mask_of(h) | dt.called_mask;
    for d in h.difference(called).iter() {
        forbidden |= dt.threat_masks[d.index()];
    }
    for d in Domino::ALL {
        if pool.len() == 3 * g {
            break;
        }
        if forbidden & (1 << d.index()) == 0 && !pool.contains(d) {
            pool.insert(d);
        }
    }
    assert_eq!(pool.len(), 3 * g, "the reduced analogue's pool fills");
    (h, pool)
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
        "walt lay-down probe — Theorem LD's catalogue and the four-laydown search — exploratory tier"
    );
    let _ = writeln!(
        out,
        "rulings: LD-A1..LD-A10, freeze 48 (walt/CENSUS-RULINGS.md 2026-08-14); Theorem LD ((L1) ∧ (L2), an exact characterization); Corollary: every lay down holds >= 4 trumps; Corollary LD-sweep (all 42 points by arithmetic, E-A2 not engaged)"
    );
    let _ = writeln!(
        out,
        "regenerate: cargo run --release -p walt-factory --example laydown_probe"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "FENCE (LD-A10(i), on every phase-2 row): the four-laydown question is COMBINATORIAL, not a situation — in a dealt hand only the bid winner declares and only one seat leads trick 1, so four lay downs can never be realised together; the question is whether the 28 tiles can be partitioned so that each hand WOULD sweep if it were the one to declare and lead."
    );
    let _ = writeln!(
        out,
        "RISK (LD-A10(ii), T1-A12 carried): Theorem LD and this probe's evidence are both computed against rules.rs's implementation of rank, tier, follow and compelled-follow; the corpus check is mandatory before any of this is cited outside walt."
    );
    let _ = writeln!(
        out,
        "NOT a receipt (PG-A8): (L1)/(L2) holding on a catalogue member selected because they hold. The term 'lay down' is the family's, used as LD-A1's defined technical term."
    );

    // ---- Phase 1: the catalogue (freeze 48 order: pip asc, hand lex asc) --
    let mut catalogue: Vec<Vec<u32>> = Vec::new();
    let mut cat_text = String::new();
    let _ = writeln!(cat_text, "digest={LD_DIGEST}");
    let _ = writeln!(
        cat_text,
        "# The lay-down catalogue (Theorem LD; freeze 48). record: decl=<pip> hand=<tiles> t=<trumps> run=<top-run> o=<outstanding>. A cache of a theorem's extension, exploratory tier; re-derivable by the regenerate line of laydown_2026-08-14.txt."
    );
    for pip in 0..=6u8 {
        let dt = DeclTest::new(pip);
        let mut members: Vec<u32> = Vec::new();
        let full: u32 = (1 << 28) - 1;
        let mut hand: u32 = (1 << 7) - 1;
        loop {
            let (ok, t, run) = dt.test(hand);
            if ok {
                members.push(hand);
                let _ = writeln!(
                    cat_text,
                    "decl={pip} hand={} t={t} run={run} o={}",
                    set_str(set_of(hand)),
                    7 - t
                );
            }
            if hand == ((1u32 << 7) - 1) << 21 {
                break;
            }
            hand = gosper_next(hand);
            if hand > full {
                break;
            }
        }
        let _ = writeln!(
            out,
            "catalogue PipTrump({pip}): {} lay downs (of {} hands tested)",
            members.len(),
            1_184_040
        );
        catalogue.push(members);
    }

    // (LD-R1): the PipTrump(6) count against LD-A9(ii)'s closed form.
    assert_eq!(
        catalogue[6].len(),
        301,
        "(LD-R1) stop-and-report: the PipTrump(6) catalogue count disagrees with LD-A9(ii)'s independently derived 301"
    );
    let _ = writeln!(
        out,
        "(LD-R1): PipTrump(6) count == 301, LD-A9(ii)'s closed form — HELD (independently derived, so this could fail)"
    );

    // (LD-R4): all seven per-declaration counts equal — Corollary LD-fold
    // (LD-A12), a direct exhibition of the declaration transport for this
    // count-free form-level predicate (Lemma S-fold is the PRECEDENT that
    // the fold is the right object, not the licensing authority; citing
    // S-fold-val here would borrow a value-transport result for a
    // form-level claim). Contentful (PG-A8): it fails if the fold is not
    // an isomorphism, if the enumeration is asymmetric, or if the rank
    // algebra is not a function of the non-declared pip order — which also
    // makes it the cheapest available probe of the LD-A10(ii)
    // implementation-versus-corpus risk.
    for pip in 0..=6u8 {
        assert_eq!(
            catalogue[pip as usize].len(),
            catalogue[6].len(),
            "(LD-R4) stop-and-report: per-declaration counts differ — Corollary LD-fold violated"
        );
    }
    let _ = writeln!(
        out,
        "(LD-R4): all seven per-declaration counts equal — Corollary LD-fold — HELD. Receipted: 301 lay downs per declaration, 2,107 (hand, declaration) pairs."
    );

    // (LD-R2): the freeze-47 family is contained, strictly, per declaration.
    for pip in 0..=6u8 {
        let fam = t1_family(pip);
        assert_eq!(fam.len(), 42, "the T1-draw family is 42 per declaration");
        let cat: BTreeSet<u32> = catalogue[pip as usize].iter().copied().collect();
        for m in &fam {
            assert!(
                cat.contains(m),
                "(LD-R2) stop-and-report: a freeze-47 T1-draw member is missing from the catalogue under its own declaration"
            );
        }
        assert!(
            cat.len() > fam.len(),
            "(LD-R2): the containment is STRICT per declaration (LD-A3's demotion of (Z1) is real)"
        );
    }
    let _ = writeln!(
        out,
        "(LD-R2): all 294 freeze-47 T1-draw members present under their own declarations; containment STRICT at every declaration — HELD"
    );

    // (LD-R3): the declared sample — the first catalogue member in canonical
    // (freeze-48) order at each t — swept exhaustively at the declared
    // grade-3 analogue against ALL field behaviours.
    for want_t in 4..=7u32 {
        let mut found = None;
        'outer: for pip in 0..=6u8 {
            let dt = DeclTest::new(pip);
            for m in &catalogue[pip as usize] {
                let (_, t, _) = dt.test(*m);
                if t == want_t {
                    found = Some((pip, *m));
                    break 'outer;
                }
            }
        }
        let (pip, m) = found.expect("every t has members");
        let dt = DeclTest::new(pip);
        let (h, pool) = reduced_analogue(&dt, m);
        // The analogue must itself satisfy (L1)/(L2) in its reduced universe;
        // then the sweep is verified against every field behaviour.
        let leaves = verify_sweep_reduced(dt.decl, h, pool, 3);
        let _ = writeln!(
            out,
            "(LD-R3) t={want_t}: sample decl={pip} hand=[{}]; declared grade-3 analogue hand=[{}] pool=[{}] — LD plan swept ALL tricks in every world against EVERY field behaviour ({} leaves) — HELD",
            set_str(set_of(m)),
            set_str(h),
            set_str(pool),
            leaves
        );
    }

    // ---- Phase 2: the four-laydown search (freeze 48 order) ---------------
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "== phase 2: the four-laydown search (exhaustive over the catalogue, complete by Theorem LD; from the seven full-suit hands per LD-A6(D5)) =="
    );
    let full28: u32 = (1 << 28) - 1;
    // (declaration pip, hand mask) per hand of a witness deal.
    type Witness = ((u8, u32), (u8, u32), (u8, u32), (u8, u32));
    let mut witness: Option<Witness> = None;
    'search: for p0 in 0..=6u8 {
        let suit_mask = mask_of(Decl::PipTrump(Pip::new(p0).expect("pip")).called_set());
        let rem0 = full28 & !suit_mask;
        let pips: Vec<u8> = (0..=6u8).filter(|p| *p != p0).collect();
        for (ai, pa) in pips.iter().enumerate() {
            for ha in &catalogue[*pa as usize] {
                if ha & !rem0 != 0 {
                    continue;
                }
                let rem1 = rem0 & !ha;
                for pb in pips.iter().skip(ai + 1) {
                    for hb in &catalogue[*pb as usize] {
                        if hb & !rem1 != 0 {
                            continue;
                        }
                        let rem2 = rem1 & !hb;
                        // the fourth hand is forced: the remaining 7 tiles
                        for pc in &pips {
                            if pc == pa || pc == pb {
                                continue;
                            }
                            let dt = DeclTest::new(*pc);
                            if dt.test(rem2).0 {
                                witness =
                                    Some(((p0, suit_mask), (*pa, *ha), (*pb, *hb), (*pc, rem2)));
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
    }
    match &witness {
        Some(((p0, m0), (pa, ma), (pb, mb), (pc, mc))) => {
            let _ = writeln!(
                out,
                "WITNESS DEAL FOUND — a COUNTEREXAMPLE to the family's <= 3 conjecture:"
            );
            for (p, m) in [(p0, m0), (pa, ma), (pb, mb), (pc, mc)] {
                let dt = DeclTest::new(*p);
                let (ok, t, run) = dt.test(*m);
                assert!(ok);
                let _ = writeln!(
                    out,
                    "  hand [{}] is a lay down under PipTrump({p}): (L1) run {run} >= outstanding {}; (L2) every non-trump's threat inside T ∪ H — discharged",
                    set_str(set_of(*m)),
                    7 - t
                );
            }
        }
        None => {
            let _ = writeln!(
                out,
                "NO FOUR-LAYDOWN DEAL EXISTS — exhaustive over the complete catalogue (Theorem LD), from every full-suit anchor (LD-A6(D5)), every declaration triple, every disjoint pair, the forced fourth hand tested under every remaining declaration. The family's <= 3 conjecture is PROVED relative to Theorem LD and the LD-A10(ii) implementation caveat. Filed as the result it is (F7)."
            );
        }
    }

    // Max pairwise-disjoint lay downs completable to a deal.
    let mut best3: Option<(u8, u32, u8, u32, u8, u32)> = None;
    'three: for (ai, pa) in (0..=6u8).enumerate() {
        for ha in &catalogue[pa as usize] {
            for pb in (0..=6u8).skip(ai + 1) {
                for hb in &catalogue[pb as usize] {
                    if ha & hb != 0 {
                        continue;
                    }
                    for pc in (0..=6u8).skip((pb + 1) as usize) {
                        for hc in &catalogue[pc as usize] {
                            if (ha | hb) & hc != 0 {
                                continue;
                            }
                            best3 = Some((pa, *ha, pb, *hb, pc, *hc));
                            break 'three;
                        }
                    }
                }
            }
        }
    }
    match (&witness, &best3) {
        (Some(_), _) => {
            let _ = writeln!(
                out,
                "maximum pairwise-disjoint lay downs completable to a deal: 4 (the witness above)."
            );
        }
        (None, Some((pa, ma, pb, mb, pc, mc))) => {
            let _ = writeln!(
                out,
                "maximum pairwise-disjoint lay downs completable to a deal: 3 (4 impossible above). First witness in freeze-48 order — the fourth hand is the 7 leftover tiles, unconstrained:"
            );
            for (p, m) in [(pa, ma), (pb, mb), (pc, mc)] {
                let _ = writeln!(
                    out,
                    "  lay down under PipTrump({p}): [{}]",
                    set_str(set_of(*m))
                );
            }
            let rest = full28 & !(ma | mb | mc);
            let _ = writeln!(out, "  the fourth hand: [{}]", set_str(set_of(rest)));
        }
        (None, None) => {
            let _ = writeln!(out, "maximum pairwise-disjoint lay downs completable to a deal: fewer than 3 (no disjoint triple found).");
        }
    }

    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "total wall-clock (provenance only): {} ms",
        t0.elapsed().as_millis()
    );
    let _ = writeln!(out, "run complete: yes");

    let results = out_dir("results").join("laydown_2026-08-14.txt");
    std::fs::write(&results, &out).expect("write results");
    let cat_path = out_dir("results").join("laydown_catalogue_2026-08-14.txt");
    std::fs::write(&cat_path, &cat_text).expect("write catalogue");
    print!("{out}");
    println!("results: {}", results.display());
}
