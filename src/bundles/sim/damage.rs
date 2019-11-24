use crate::components::*;
use specs::prelude::*;

pub struct DamageSystem;

impl DamageSystem {
    pub fn name() -> &'static str {
        "sim::damage_system"
    }
}

impl<'s> specs::System<'s> for DamageSystem {
    type SystemData = (
        WriteStorage<'s, Health>,
        WriteStorage<'s, Collision>,
        Entities<'s>,
    );

    fn run(&mut self, (mut healths, mut collisions, ents): Self::SystemData) {
        (&collisions, &mut healths, &ents)
            .join()
            .map(|(coll, mut health, ent)| {
                // TODO: Maybe have different damage per kind?
                let damage = match coll {
                    Collision(_) => 1,
                };
                health.current -= damage;
                ent
            })
            .collect::<Vec<_>>()
            .iter()
            .for_each(|&ent| {
                collisions.remove(ent);
            });
    }
}
