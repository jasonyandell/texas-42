//! The typed expression dictionary: the versioned 14-clause relational
//! library (`scheme-relational-actor-v1/grammar-v1/straight-v0.4`), consumed
//! as SET-valued features x_j(I, a) = 1 iff clause j's answer relation at the
//! viewer's lawful information contains legal action a (parent §2). The
//! campaign fixes this language; expanding it is a new dictionary version.

use walt::policy_search::relational::{relational_grammar, RELATIONAL_GRAMMAR_VERSION};
use walt::rules::DominoSet;
use walt::scheme::{Budget, CompiledFix, Frame, Registry, Value};

pub struct ClauseDictionary {
    pub version: String,
    pub ids: Vec<String>,
    compiled: Vec<CompiledFix>,
}

impl ClauseDictionary {
    pub fn standard() -> Result<Self, String> {
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
        Ok(ClauseDictionary {
            version: RELATIONAL_GRAMMAR_VERSION.to_string(),
            ids,
            compiled,
        })
    }

    pub fn len(&self) -> usize {
        self.compiled.len()
    }

    pub fn is_empty(&self) -> bool {
        self.compiled.is_empty()
    }

    /// Candidate sets per clause at one decision: answers of each guard,
    /// intersected with the legal set. `budget` charges real inference work.
    pub fn candidate_sets(
        &self,
        frame: &Frame,
        legal: DominoSet,
        budget: &mut Budget,
    ) -> Result<Vec<DominoSet>, String> {
        let mut out = Vec::with_capacity(self.compiled.len());
        for (id, fix) in self.ids.iter().zip(&self.compiled) {
            let answers = fix
                .evaluate_viewer(frame, budget)
                .map_err(|e| format!("clause {id}: {e}"))?;
            let set: DominoSet = answers
                .iter()
                .filter_map(|a| match a.0.as_slice() {
                    [Value::Domino(t)] if legal.contains(*t) => Some(*t),
                    _ => None,
                })
                .collect();
            out.push(set);
        }
        Ok(out)
    }
}
