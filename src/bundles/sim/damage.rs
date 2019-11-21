use crate::components::*;
use crate::resources::*;
use specs::prelude::*;

pub struct DamageSystem;

impl DamageSystem {
    pub fn name() -> &'static str {
        "sim::damage_system"
    }
}

impl<'s> specs::System<'s> for DamageSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, Collision>,
        Entities<'s>,
    );

    fn run(&mut self, (player, mut healths, mut collisions, ents): Self::SystemData) {
        if let Some(phealth) = healths.get_mut(**player) {
            let collided_ents: Vec<Entity> = (&collisions, &ents)
                .join()
                .filter(|(_, ent)| *ent != **player)
                .map(|(_, ent)| {
                    phealth.current -= 1;
                    ent
                })
                .collect();
            for ent in collided_ents {
                collisions.remove(ent);
                if let Some(health) = healths.get_mut(ent) {
                    health.current -= 1;
                }
            }
        }
    }
}
