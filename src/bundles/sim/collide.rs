use crate::{components::*, resources::*, util::*};
use log::debug;
use nc::bounding_volume::bounding_volume::*;
use specs::prelude::*;

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn name() -> &'static str {
        "sim::collision_system"
    }
}

impl<'s> specs::System<'s> for CollisionSystem {
    type SystemData = (
        ReadExpect<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Extent>,
        WriteStorage<'s, Collision>,
        Entities<'s>,
    );

    fn run(&mut self, (player, transform, extent, mut collisions, entities): Self::SystemData) {
        if let Some((pbox, ptrans)) = extent
            .get(**player)
            .and_then(|e| transform.get(**player).map(|t| (e, t)))
        {
            let pbox = pbox.bbox().transform_by(&translate_trans(&ptrans));
            (&transform, &extent, &entities)
                .join()
                .filter(|(trans, ext, ent)| {
                    *ent != **player
                        && ext
                            .bbox()
                            .transform_by(&translate_trans(&trans))
                            .intersects(&pbox)
                })
                .for_each(|(_, _, ent)| {
                    if let Ok(entry) = collisions.entry(ent) {
                        debug!("Collision happened.");
                        entry.or_insert(Collision);
                    }
                })
        }
    }
}
