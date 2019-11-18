use crate::{components::*, util::*};
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
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Extent>,
        ReadStorage<'s, ObjectKind>,
        WriteStorage<'s, Collision>,
        Entities<'s>,
    );

    fn run(&mut self, (transform, extent, objs, mut collisions, entities): Self::SystemData) {
        if let Some((_, pbox, ptrans)) = (&objs, &extent, &transform)
            .join()
            .find(|(obj, _ext, _trans)| obj.is_player())
        {
            let pbox = pbox.bbox().transform_by(&translate_trans(&ptrans));
            (&transform, &extent, &objs, &entities)
                .join()
                .filter(|(trans, ext, obj, _)| {
                    !obj.is_player()
                        && ext
                            .bbox()
                            .transform_by(&translate_trans(&trans))
                            .intersects(&pbox)
                })
                .for_each(|(_, _, _, ent)| {
                    if let Ok(entry) = collisions.entry(ent) {
                        debug!("Collision happened.");
                        entry.or_insert(Collision);
                    }
                })
        }
    }
}
