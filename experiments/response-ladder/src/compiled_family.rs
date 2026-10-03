//! Role-aware compiled actor artifacts.
//!
//! A legacy [`Actor`] is a single program used at every seat.  A family keeps
//! the same public actor wire format while allowing the normalized declaring
//! and defending teams to use different programs.

use crate::compiled::Actor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;
use walt::rules::Team;
use walt::scheme::{Atom, Fallback, PolicyProgram, PolicyRule, Role, Sort, Term, Value};

pub const FAMILY_SCHEMA: &str = "scheme-role-actors-v1";

/// Export namespace for role-aware Scheme policies.
pub struct FamilyScheme;

impl FamilyScheme {
    pub fn export(family: &Family, name: &str) -> Result<PolicyProgram, String> {
        family.policy_program_named(name)
    }
}

/// A backward-compatible compiled actor artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Family {
    /// Legacy artifact: one actor is used for every normalized seat.
    Single(Actor),
    /// Role artifact: odd normalized seats are declaring, even normalized
    /// seats are defending.
    Roles { declaring: Actor, defending: Actor },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleArtifact {
    schema: String,
    declaring: Actor,
    defending: Actor,
}

impl Family {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Single(actor) => actor.validate(),
            Self::Roles {
                declaring,
                defending,
            } => {
                declaring.validate()?;
                defending.validate()?;
                Ok(())
            }
        }
    }

    /// Select the actor for a normalized seat.  Normalized odd seats are T1,
    /// the declaring team; normalized even seats are T0, the defending team.
    pub fn actor(&self, viewer: u8) -> Result<&Actor, String> {
        if viewer >= 4 {
            return Err("compiled family viewer must be a seat 0..3".into());
        }
        self.validate()?;
        Ok(match self {
            Self::Single(actor) => actor,
            Self::Roles {
                declaring,
                defending,
            } => {
                if viewer % 2 == 1 {
                    declaring
                } else {
                    defending
                }
            }
        })
    }

    /// Expand a family to both role identities for ready handshakes and
    /// stable artifact reports.
    pub fn roles(&self) -> Result<(&Actor, &Actor), String> {
        self.validate()?;
        Ok(match self {
            Self::Single(actor) => (actor, actor),
            Self::Roles {
                declaring,
                defending,
            } => (declaring, defending),
        })
    }

    /// Export a role family as a Scheme program.  Legacy single actors retain
    /// their exact export.  A role family binds `viewer` in every rule and
    /// guards each rule by the viewer's normalized team.
    pub fn policy_program_named(&self, name: &str) -> Result<PolicyProgram, String> {
        self.validate()?;
        if !valid_name(name) {
            return Err(format!("invalid Scheme policy name {name:?}"));
        }
        match self {
            Self::Single(actor) => actor.policy_program_named(name),
            Self::Roles {
                declaring,
                defending,
            } => {
                let declaring = declaring.policy_program_named("declaring")?;
                let defending = defending.policy_program_named("defending")?;
                let mut rules = Vec::with_capacity(declaring.rules.len() + defending.rules.len());
                for rule in declaring.rules {
                    rules.push(role_rule(rule, Team::T1, "declaring"));
                }
                for rule in defending.rules {
                    rules.push(role_rule(rule, Team::T0, "defending"));
                }
                Ok(PolicyProgram {
                    name: name.to_owned(),
                    initial_mode: "shared".to_owned(),
                    bindings: Vec::new(),
                    exact_rules: Vec::new(),
                    rules,
                    fallback: Fallback::LowestLegal,
                })
            }
        }
    }

    pub fn policy_program(&self) -> Result<PolicyProgram, String> {
        self.policy_program_named("compiled-family-v1")
    }

    pub fn to_policy_program(&self) -> Result<PolicyProgram, String> {
        self.policy_program()
    }
}

impl Serialize for Family {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single(actor) => actor.serialize(serializer),
            Self::Roles {
                declaring,
                defending,
            } => {
                #[derive(Serialize)]
                struct Artifact<'a> {
                    schema: &'static str,
                    declaring: &'a Actor,
                    defending: &'a Actor,
                }
                Artifact {
                    schema: FAMILY_SCHEMA,
                    declaring,
                    defending,
                }
                .serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Family {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = JsonValue::deserialize(deserializer)?;
        let object = value
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("compiled family must be a JSON object"))?;
        if object.contains_key("declaring") || object.contains_key("defending") {
            for key in object.keys() {
                if !matches!(key.as_str(), "schema" | "declaring" | "defending") {
                    return Err(serde::de::Error::custom(format!(
                        "unknown compiled family field {key:?}"
                    )));
                }
            }
            ensure_actor_shape(object.get("declaring").ok_or_else(|| {
                serde::de::Error::custom("role family artifact requires declaring actor")
            })?)
            .map_err(serde::de::Error::custom)?;
            ensure_actor_shape(object.get("defending").ok_or_else(|| {
                serde::de::Error::custom("role family artifact requires defending actor")
            })?)
            .map_err(serde::de::Error::custom)?;
            let artifact: RoleArtifact =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            if artifact.schema != FAMILY_SCHEMA {
                return Err(serde::de::Error::custom(format!(
                    "unsupported compiled family schema {:?}",
                    artifact.schema
                )));
            }
            return Ok(Self::Roles {
                declaring: artifact.declaring,
                defending: artifact.defending,
            });
        }
        for key in object.keys() {
            if !matches!(key.as_str(), "schema" | "clauses" | "lead_clauses") {
                return Err(serde::de::Error::custom(format!(
                    "unknown compiled actor field {key:?}"
                )));
            }
        }
        let actor: Actor = serde_json::from_value(value).map_err(serde::de::Error::custom)?;
        Ok(Self::Single(actor))
    }
}

fn ensure_actor_shape(value: &JsonValue) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "compiled family actor must be a JSON object".to_owned())?;
    for key in object.keys() {
        if !matches!(key.as_str(), "schema" | "clauses" | "lead_clauses") {
            return Err(format!("unknown compiled actor field {key:?}"));
        }
    }
    Ok(())
}

fn role_rule(mut rule: PolicyRule, team: Team, prefix: &str) -> PolicyRule {
    rule.name = format!("{prefix}-{}", rule.name);
    let viewer = "viewer".to_owned();
    if !rule.guard.roles.iter().any(|role| role.name == viewer) {
        rule.guard.roles.push(Role {
            name: viewer.clone(),
            sort: Sort::Chair,
        });
    }
    for case in &mut rule.guard.cases {
        if !case
            .atoms
            .iter()
            .any(|atom| atom.predicate == "viewer" && atom.args == vec![Term::Role(viewer.clone())])
        {
            case.atoms.push(Atom {
                predicate: "viewer".into(),
                args: vec![Term::Role(viewer.clone())],
                negated: false,
            });
        }
        case.atoms.push(Atom {
            predicate: "team".into(),
            args: vec![Term::Role(viewer.clone()), Term::Literal(Value::Team(team))],
            negated: false,
        });
    }
    rule
}

fn valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiled::{SCHEMA, SCHEMA_V2};

    fn actor(clauses: &[usize]) -> Actor {
        Actor::new(SCHEMA, clauses.to_vec()).unwrap()
    }

    #[test]
    fn legacy_actor_round_trips_as_single_family() {
        let a = actor(&[8, 3]);
        let family: Family = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
        assert_eq!(family, Family::Single(a.clone()));
        assert_eq!(family.actor(0).unwrap(), &a);
        assert_eq!(family.actor(3).unwrap(), &a);
    }

    #[test]
    fn role_artifact_selects_normalized_team() {
        let declaring = actor(&[0]);
        let defending = actor(&[1]);
        let family = Family::Roles {
            declaring: declaring.clone(),
            defending: defending.clone(),
        };
        assert_eq!(family.actor(1).unwrap(), &declaring);
        assert_eq!(family.actor(3).unwrap(), &declaring);
        assert_eq!(family.actor(0).unwrap(), &defending);
        assert_eq!(family.actor(2).unwrap(), &defending);
    }

    #[test]
    fn malformed_or_mixed_role_artifacts_are_rejected() {
        assert!(serde_json::from_str::<Family>(
            r#"{"schema":"scheme-role-actors-v1","declaring":{}}"#
        )
        .is_err());
        assert!(serde_json::from_str::<Family>(
            r#"{"schema":"scheme-role-actors-v1","declaring":{},"defending":{},"clauses":[]}"#
        )
        .is_err());
        assert!(serde_json::from_str::<Family>(
            r#"{"schema":"wrong","declaring":{},"defending":{}}"#
        )
        .is_err());
        assert!(serde_json::from_str::<Family>(
            r#"{"schema":"scheme-relational-actor-v1","clauses":[],"extra":1}"#
        )
        .is_err());
    }

    #[test]
    fn role_artifact_allows_mixed_valid_actor_schema_versions() {
        let declaring = actor(&[0]);
        let defending = Actor::new(SCHEMA_V2, vec![14]).unwrap();
        let json = serde_json::json!({
            "schema": FAMILY_SCHEMA,
            "declaring": declaring,
            "defending": defending,
        });
        let family: Family = serde_json::from_value(json).unwrap();
        family.validate().unwrap();
    }

    #[test]
    fn family_export_guards_every_rule_by_viewer_team() {
        let family = Family::Roles {
            declaring: actor(&[8, 3]),
            defending: actor(&[5]),
        };
        let program = family.policy_program().unwrap();
        assert!(!program.rules.is_empty());
        for rule in program.rules {
            let case = &rule.guard.cases[0];
            assert!(rule.guard.roles.iter().any(|role| role.name == "viewer"));
            assert!(case.atoms.iter().any(|atom| atom.predicate == "viewer"));
            assert!(case.atoms.iter().any(|atom| atom.predicate == "team"));
        }
        assert!(matches!(program.fallback, Fallback::LowestLegal));
    }
}
