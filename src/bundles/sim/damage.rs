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
        ReadStorage<'s, ObjectKind>,
        Entities<'s>,
    );

    fn run(&mut self, (mut healths, mut collisions, objs, ents): Self::SystemData) {
        if let Some((_, phealth)) = (&objs, &mut healths)
            .join()
            .find(|(obj, _)| obj.is_player())
        {
            let collided_ents: Vec<Entity> = (&collisions, &objs, &ents)
                .join()
                .filter(|(_, obj, _)| !obj.is_player())
                .map(|(_, _, ent)| {
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
