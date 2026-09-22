//! The autonomous constructor (parent §5): a fixed typed language of
//! mechanical, own-hand and public-record fragments, enumerated whole - a
//! human declares the language and its caps, never the next strategic
//! relation. Candidate expressions are deduplicated on the versioned probe
//! panel (empirical agreement is NOT semantic equivalence - recorded as
//! such), scored by the covariance derivative g_F at coefficient zero, and
//! a bounded number are admitted per generation at weight 1, where they are
//! semantically invisible until the gradient moves them.

use walt::rules::{Context, DominoSet};
use walt::scheme::{Atom, Budget, CompiledFix, Fix, Frame, Registry, Role, Scheme, Sort, Term, Value};

/// One reusable typed fragment: named roles plus atoms. Fragments sharing a
/// role name (e.g. `w` = current-winner) merge by atom equality when
/// combined, so conjunctions stay consistent by construction.
#[derive(Clone, Debug)]
pub struct Fragment {
    pub id: &'static str,
    pub roles: Vec<Role>,
    pub atoms: Vec<Atom>,
}

fn role(name: &str, sort: Sort) -> Role {
    Role {
        name: name.to_string(),
        sort,
    }
}

fn r(name: &str) -> Term {
    Term::Role(name.to_string())
}

fn atom(predicate: &str, args: Vec<Term>, negated: bool) -> Atom {
    Atom {
        predicate: predicate.to_string(),
        args,
        negated,
    }
}

fn n_lit(n: u32) -> Term {
    Term::Literal(Value::Number(n))
}

fn called_lit() -> Term {
    Term::Literal(Value::Context(Context::Called))
}

/// Action-flavored fragments: each constrains the candidate `action`.
pub fn action_fragments() -> Vec<Fragment> {
    let mut out = Vec::new();
    for (id, neg) in [("double", false), ("not-double", true)] {
        out.push(Fragment {
            id,
            roles: vec![],
            atoms: vec![atom("double", vec![r("action")], neg)],
        });
    }
    for (id, neg) in [("master", false), ("not-master", true)] {
        out.push(Fragment {
            id,
            roles: vec![],
            atoms: vec![atom("master", vec![r("action")], neg)],
        });
    }
    for (id, n, neg) in [
        ("count0", 0u32, false),
        ("not-count0", 0, true),
        ("count5", 5, false),
        ("not-count5", 5, true),
        ("count10", 10, false),
        ("not-count10", 10, true),
    ] {
        out.push(Fragment {
            id,
            roles: vec![],
            atoms: vec![atom("count", vec![r("action"), n_lit(n)], neg)],
        });
    }
    let led = || role("led", Sort::Context);
    for (id, pred, neg) in [
        ("follows-led", "in", false),
        ("off-led", "in", true),
        ("boss-led", "boss", false),
        ("not-boss-led", "boss", true),
        ("would-lead-led", "leads-context", false),
        ("would-not-lead-led", "leads-context", true),
    ] {
        out.push(Fragment {
            id,
            roles: vec![led()],
            atoms: vec![
                atom("led-context", vec![r("led")], false),
                atom(pred, vec![r("action"), r("led")], neg),
            ],
        });
    }
    for (id, pred, neg) in [
        ("trump", "in", false),
        ("not-trump", "in", true),
        ("trump-boss", "boss", false),
        ("not-trump-boss", "boss", true),
    ] {
        out.push(Fragment {
            id,
            roles: vec![],
            atoms: vec![atom(pred, vec![r("action"), called_lit()], neg)],
        });
    }
    for (id, neg) in [("takes-trick", false), ("loses-trick", true)] {
        out.push(Fragment {
            id,
            roles: vec![
                role("led", Sort::Context),
                role("w", Sort::Chair),
                role("t", Sort::Domino),
            ],
            atoms: vec![
                atom("led-context", vec![r("led")], false),
                atom("current-winner", vec![r("w")], false),
                atom("trick-play", vec![r("w"), r("t")], false),
                atom("beats", vec![r("action"), r("t"), r("led")], neg),
            ],
        });
    }
    out
}

/// Qualifier fragments: public-record context, action-independent.
pub fn qualifier_fragments() -> Vec<Fragment> {
    let mut out = Vec::new();
    let v = || role("v", Sort::Chair);
    let w = || role("w", Sort::Chair);
    let l = || role("l", Sort::Chair);
    for (id, pred) in [
        ("partner-winning", "partner"),
        ("opponent-winning", "opponent"),
    ] {
        out.push(Fragment {
            id,
            roles: vec![v(), w()],
            atoms: vec![
                atom("viewer", vec![r("v")], false),
                atom("current-winner", vec![r("w")], false),
                atom(pred, vec![r("v"), r("w")], false),
            ],
        });
    }
    out.push(Fragment {
        id: "pos-lead",
        roles: vec![v()],
        atoms: vec![
            atom("viewer", vec![r("v")], false),
            atom("leader", vec![r("v")], false),
        ],
    });
    for (id, pred, flip) in [
        ("pos-second", "successor", false),
        ("pos-third", "partner", false),
        ("pos-last", "successor", true),
    ] {
        let args = if flip {
            vec![r("v"), r("l")]
        } else {
            vec![r("l"), r("v")]
        };
        out.push(Fragment {
            id,
            roles: vec![v(), l()],
            atoms: vec![
                atom("viewer", vec![r("v")], false),
                atom("leader", vec![r("l")], false),
                atom(pred, args, false),
            ],
        });
    }
    for n in 1..=7u32 {
        let id: &'static str = match n {
            1 => "hand-1",
            2 => "hand-2",
            3 => "hand-3",
            4 => "hand-4",
            5 => "hand-5",
            6 => "hand-6",
            _ => "hand-7",
        };
        out.push(Fragment {
            id,
            roles: vec![v()],
            atoms: vec![
                atom("viewer", vec![r("v")], false),
                atom("quota", vec![r("v"), n_lit(n)], false),
            ],
        });
    }
    out
}

/// Assemble a guard from fragments: `(own-legal action)` plus the merged
/// fragment atoms; roles merged by name, duplicate atoms deduplicated.
fn assemble(fragments: &[&Fragment]) -> Fix {
    let mut roles = vec![role("action", Sort::Domino)];
    let mut atoms = vec![atom("own-legal", vec![r("action")], false)];
    for f in fragments {
        for ro in &f.roles {
            if !roles.iter().any(|existing| existing.name == ro.name) {
                roles.push(ro.clone());
            }
        }
        for a in &f.atoms {
            if !atoms.contains(a) {
                atoms.push(a.clone());
            }
        }
    }
    Fix {
        roles,
        outputs: vec!["action".to_string()],
        cases: vec![Scheme {
            equal: vec![],
            atoms,
        }],
    }
}

pub struct Candidate {
    pub id: String,
    pub text: String,
    pub compiled: CompiledFix,
}

/// The whole bounded pool, in deterministic order: every action fragment
/// alone, crossed with every qualifier, and every unordered pair of
/// distinct action fragments. Each candidate must compile against the
/// standard registry AND round-trip through its canonical text (the state
/// file stores learned expressions as text); failures are dropped and
/// counted, never silently repaired.
pub fn generate_pool(registry: &Registry) -> (Vec<Candidate>, u64) {
    let actions = action_fragments();
    let qualifiers = qualifier_fragments();
    let mut shapes: Vec<(String, Vec<&Fragment>)> = Vec::new();
    for a in &actions {
        shapes.push((format!("x[{}]", a.id), vec![a]));
    }
    for a in &actions {
        for q in &qualifiers {
            shapes.push((format!("x[{}|{}]", a.id, q.id), vec![a, q]));
        }
    }
    for (i, a) in actions.iter().enumerate() {
        for b in actions.iter().skip(i + 1) {
            shapes.push((format!("x[{}&{}]", a.id, b.id), vec![a, b]));
        }
    }
    let mut pool = Vec::new();
    let mut dropped = 0u64;
    for (id, frags) in shapes {
        let fix = assemble(&frags);
        // Single-line canonical text (the state file is line-based).
        let text = fix
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let round_trip: Result<Fix, _> = text.parse();
        let ok = matches!(&round_trip, Ok(back) if back == &fix);
        match (ok, fix.compile(registry)) {
            (true, Ok(compiled)) => pool.push(Candidate { id, text, compiled }),
            _ => dropped += 1,
        }
    }
    (pool, dropped)
}

/// Panel filter: drop candidates that are constant on every panel decision
/// (always empty or always the whole legal set) or that duplicate an
/// existing dictionary column or an earlier candidate's column on the
/// panel. Returns surviving indices into `pool`.
pub fn panel_filter(
    pool: &[Candidate],
    dictionary_columns: &[Vec<u32>],
    panel: &[(Frame, DominoSet)],
    budget: &mut Budget,
) -> Result<Vec<usize>, String> {
    let mut seen: std::collections::BTreeSet<Vec<u32>> = dictionary_columns
        .iter()
        .cloned()
        .collect();
    let mut survivors = Vec::new();
    for (idx, cand) in pool.iter().enumerate() {
        let mut column = Vec::with_capacity(panel.len());
        let mut all_empty = true;
        let mut all_full = true;
        for (frame, legal) in panel {
            let set = crate::features::set_for(&cand.compiled, frame, *legal, budget)
                .map_err(|e| format!("candidate {}: {e}", cand.id))?;
            if !set.is_empty() {
                all_empty = false;
            }
            if set != *legal {
                all_full = false;
            }
            column.push(set.bits());
        }
        if all_empty || all_full {
            continue;
        }
        if seen.insert(column) {
            survivors.push(idx);
        }
    }
    Ok(survivors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pool_compiles_and_round_trips_whole() {
        // Law: every declared shape survives compile + text round-trip; the
        // language is exactly what the generator says it is.
        let registry = Registry::standard();
        let (pool, dropped) = generate_pool(&registry);
        assert_eq!(dropped, 0, "every pool shape must compile and round-trip");
        // 22 singles + 22*13 qualified + C(22,2) pairs = 539.
        assert_eq!(pool.len(), 539);
        // PINNED strictness witnesses: the tactically central composites
        // exist under their declared ids.
        assert!(pool.iter().any(|c| c.id == "x[takes-trick]"));
        assert!(pool.iter().any(|c| c.id == "x[trump-boss|pos-last]"));
    }

    #[test]
    fn negation_complements_within_the_legal_set() {
        // Law: x[double] and x[not-double] partition every legal set.
        use crate::features::set_for;
        use crate::target::CampaignTarget;
        let registry = Registry::standard();
        let (pool, _) = generate_pool(&registry);
        let double = pool.iter().find(|c| c.id == "x[double]").unwrap();
        let not_double = pool.iter().find(|c| c.id == "x[not-double]").unwrap();
        let target = CampaignTarget::og_v1();
        let hands = target.deal(3);
        let decl = target.declaration(hands[0]);
        let frame = crate::rollout::opening_frame_for_test(decl, &hands);
        let legal = walt::rules::legal_plays(decl, hands[0], None);
        let mut budget = Budget::new(10_000_000);
        let a = set_for(&double.compiled, &frame, legal, &mut budget).unwrap();
        let b = set_for(&not_double.compiled, &frame, legal, &mut budget).unwrap();
        assert_eq!(a.union(b), legal);
        assert!(a.is_disjoint(b));
    }
}
