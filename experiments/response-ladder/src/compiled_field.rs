//! Frozen compiled lower rungs. Each query sees only its own hand and public play.
use crate::{compiled::Actor, compiled_family::Family};
use crate::mechanics::{Budget, Field, FieldQuery};

/// The three other seats share C0, or the focal partner uses C1. The focal
/// policy is chosen by the outer response solver and is never called here.
pub struct CompiledField {
    actors: [Actor; 4],
    identity: String,
}

impl CompiledField {
    pub fn all(actor: &Actor) -> Result<Self, String> {
        Self::all_family(&Family::Single(actor.clone()))
    }

    pub fn all_family(family: &Family) -> Result<Self, String> {
        family.validate()?;
        Self::new(std::array::from_fn(|seat| family.actor(seat as u8).unwrap().clone()))
    }

    pub fn partner(viewer: u8, c0: &Actor, c1: &Actor) -> Result<Self, String> {
        Self::partner_family(viewer, &Family::Single(c0.clone()), &Family::Single(c1.clone()))
    }

    pub fn partner_family(viewer: u8, c0: &Family, c1: &Family) -> Result<Self, String> {
        if viewer >= 4 {
            return Err("compiled field viewer must be a seat 0..3".into());
        }
        c0.validate()?;
        c1.validate()?;
        let partner = (viewer + 2) % 4;
        let actors = std::array::from_fn(|seat| {
            let seat = seat as u8;
            if seat == partner {
                c1.actor(seat).unwrap().clone()
            } else {
                c0.actor(seat).unwrap().clone()
            }
        });
        Self::new(actors)
    }

    fn new(actors: [Actor; 4]) -> Result<Self, String> {
        for actor in &actors {
            actor.validate()?;
        }
        // Full canonical program data is the identity, not a lossy hash. No
        // mutation API is exposed while a response target uses this field.
        let identity = format!(
            "compiled-field-v1/{}",
            serde_json::to_string(&actors).map_err(|error| error.to_string())?
        );
        Ok(Self { actors, identity })
    }

    pub fn actors(&self) -> &[Actor; 4] {
        &self.actors
    }
}

impl Field for CompiledField {
    fn revision(&self) -> &str {
        &self.identity
    }

    fn compiled_actors(&self) -> Option<&[Actor; 4]> {
        Some(&self.actors)
    }

    fn choose(&mut self, query: FieldQuery<'_>, budget: &mut Budget) -> Option<u8> {
        budget.tick()?;
        self.actors.get(query.seat as usize)?.action_unchecked(
            query.decl,
            query.seat,
            query.hand,
            query.public,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiled::SCHEMA;
    use crate::mechanics::PublicState;
    use std::time::Duration;
    use walt::rules::Decl;

    #[test]
    fn partner_roles_and_program_content_determine_frozen_identity() {
        let c0 = Actor::empty();
        let c1 = Actor::new(SCHEMA, vec![8, 3]).unwrap();
        for viewer in 0..4 {
            let field = CompiledField::partner(viewer, &c0, &c1).unwrap();
            for seat in 0..4 {
                assert_eq!(
                    field.actors()[seat],
                    if seat == (viewer as usize + 2) % 4 {
                        c1.clone()
                    } else {
                        c0.clone()
                    }
                );
            }
            assert_ne!(
                field.revision(),
                CompiledField::all(&c0).unwrap().revision()
            );
            assert_eq!(
                field.revision(),
                CompiledField::partner(viewer, &c0, &c1).unwrap().revision()
            );
        }
    }

    #[test]
    fn compiled_choices_do_not_depend_on_latent_dice_tape() {
        let mut field = CompiledField::all(&Actor::new(SCHEMA, vec![8, 3]).unwrap()).unwrap();
        let public = PublicState::opening(1);
        let mut budget = Budget::new(10, Duration::from_secs(1));
        let mut actions = Vec::new();
        for tape in [0, 1, u64::MAX] {
            actions.push(
                field
                    .choose(
                        FieldQuery {
                            decl: Decl::ALL[0],
                            bid: 30,
                            seat: 1,
                            hand: 127,
                            public: &public,
                            tape,
                        },
                        &mut budget,
                    )
                    .unwrap(),
            );
        }
        assert!(actions.iter().all(|action| *action == actions[0]));
    }

    #[test]
    fn family_field_assigns_role_specific_actors_and_revision() {
        let declaring = Actor::new(SCHEMA, vec![8, 3]).unwrap();
        let defending = Actor::new(SCHEMA, vec![5]).unwrap();
        let family = Family::Roles { declaring: declaring.clone(), defending: defending.clone() };
        let all = CompiledField::all_family(&family).unwrap();
        assert_eq!(all.actors()[1], declaring);
        assert_eq!(all.actors()[3], declaring);
        assert_eq!(all.actors()[0], defending);
        assert_eq!(all.actors()[2], defending);
        assert_ne!(all.revision(), CompiledField::all(&declaring).unwrap().revision());
        let partnered = CompiledField::partner_family(1, &family, &Family::Single(declaring.clone())).unwrap();
        assert_eq!(partnered.actors()[3], declaring);
        assert_eq!(partnered.actors()[0], defending);
    }
}
