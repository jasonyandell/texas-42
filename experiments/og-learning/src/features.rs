//! The typed expression dictionary: the versioned 14-clause relational
//! seed library (`scheme-relational-actor-v1/grammar-v1/straight-v0.4`)
//! plus constructor-admitted expressions, all consumed as SET-valued
//! features x_j(I, a) = 1 iff expression j's answer relation at the
//! viewer's lawful information contains legal action a (parent §2). The
//! campaign fixes the LANGUAGE (constructor.rs); the dictionary within it
//! grows only through the recorded admission rule.

use walt::policy_search::relational::{relational_grammar, RELATIONAL_GRAMMAR_VERSION};
use walt::rules::DominoSet;
use walt::scheme::{Budget, CompiledFix, Fix, Frame, Registry, Value};

/// One expression's candidate set at one decision.
pub fn set_for(
    fix: &CompiledFix,
    frame: &Frame,
    legal: DominoSet,
    budget: &mut Budget,
) -> Result<DominoSet, String> {
    let answers = fix.evaluate_viewer(frame, budget).map_err(|e| e.to_string())?;
    Ok(answers
        .iter()
        .filter_map(|a| match a.0.as_slice() {
            [Value::Domino(t)] if legal.contains(*t) => Some(*t),
            _ => None,
        })
        .collect())
}

pub struct ClauseDictionary {
    /// Composed version: the seed grammar version, plus a content digest of
    /// the learned expressions when any exist.
    pub version: String,
    pub ids: Vec<String>,
    /// Learned (id, canonical text) pairs, in admission order; the seed
    /// library occupies indices 0..seed_len and carries no stored text.
    pub learned: Vec<(String, String)>,
    pub seed_len: usize,
    compiled: Vec<CompiledFix>,
    registry: Registry,
}

fn learned_digest(learned: &[(String, String)]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (id, text) in learned {
        for b in id.bytes().chain([b'|']).chain(text.bytes()).chain([b'\n']) {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    h
}

impl ClauseDictionary {
    pub fn standard() -> Result<Self, String> {
        Self::with_learned(&[])
    }

    /// Seed library plus previously admitted expressions (from state).
    pub fn with_learned(learned: &[(String, String)]) -> Result<Self, String> {
        let registry = Registry::standard();
        let mut ids = Vec::new();
        let mut compiled = Vec::new();
        for template in relational_grammar() {
            let fix = template
                .guard
                .compile(&registry)
                .map_err(|e| format!("clause {} failed to compile: {e}", template.id))?;
            ids.push(template.id);
            compiled.push(fix);
        }
        let seed_len = ids.len();
        for (id, text) in learned {
            let fix: Fix = text
                .parse()
                .map_err(|e| format!("learned expression {id} failed to parse: {e:?}"))?;
            let cfix = fix
                .compile(&registry)
                .map_err(|e| format!("learned expression {id} failed to compile: {e}"))?;
            ids.push(id.clone());
            compiled.push(cfix);
        }
        let version = if learned.is_empty() {
            RELATIONAL_GRAMMAR_VERSION.to_string()
        } else {
            format!(
                "{RELATIONAL_GRAMMAR_VERSION}+og-constructor-v1/{:016x}",
                learned_digest(learned)
            )
        };
        Ok(ClauseDictionary {
            version,
            ids,
            learned: learned.to_vec(),
            seed_len,
            compiled,
            registry,
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn len(&self) -> usize {
        self.compiled.len()
    }

    pub fn is_empty(&self) -> bool {
        self.compiled.is_empty()
    }

    pub fn compiled(&self) -> &[CompiledFix] {
        &self.compiled
    }

    /// Admit a constructor candidate (already compiled + round-tripped).
    pub fn admit(&mut self, id: String, text: String, compiled: CompiledFix) {
        self.ids.push(id.clone());
        self.learned.push((id, text));
        self.compiled.push(compiled);
        self.version = format!(
            "{RELATIONAL_GRAMMAR_VERSION}+og-constructor-v1/{:016x}",
            learned_digest(&self.learned)
        );
    }

    /// Candidate sets per expression at one decision.
    pub fn candidate_sets(
        &self,
        frame: &Frame,
        legal: DominoSet,
        budget: &mut Budget,
    ) -> Result<Vec<DominoSet>, String> {
        let mut out = Vec::with_capacity(self.compiled.len());
        for (id, fix) in self.ids.iter().zip(&self.compiled) {
            let set =
                set_for(fix, frame, legal, budget).map_err(|e| format!("clause {id}: {e}"))?;
            out.push(set);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learned_expressions_round_trip_through_state_text() {
        // Law: an admitted expression rebuilt from its stored text yields
        // the same dictionary version and identity.
        let registry = Registry::standard();
        let (pool, _) = crate::constructor::generate_pool(&registry);
        let cand = pool.iter().find(|c| c.id == "x[takes-trick]").unwrap();
        let mut dict = ClauseDictionary::standard().unwrap();
        let before = dict.version.clone();
        dict.admit(
            cand.id.clone(),
            cand.text.clone(),
            cand.text.parse::<Fix>().unwrap().compile(&registry).unwrap(),
        );
        assert_ne!(dict.version, before);
        let rebuilt = ClauseDictionary::with_learned(&dict.learned).unwrap();
        assert_eq!(rebuilt.version, dict.version);
        assert_eq!(rebuilt.ids, dict.ids);
        assert_eq!(rebuilt.len(), 15);
        // PINNED strictness witness: the seed prefix is untouched.
        assert_eq!(rebuilt.seed_len, 14);
    }
}
