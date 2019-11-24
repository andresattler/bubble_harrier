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
        ReadStorage<'s, ObjectKind>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Vel>,
        ReadStorage<'s, Extent>,
        WriteStorage<'s, Collision>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (kinds, transform, vels, extent, mut collisions, entities): Self::SystemData,
    ) {
        (&kinds, &transform, &vels, &extent, &entities)
            .par_join()
            .map(|(dyn_kind, dtrans, _, ext, dyn_ent)| {
                let bbox = ext.bbox().transform_by(&translate_trans(&dtrans));
                (&kinds, &transform, !&vels, &extent, &entities)
                    .join()
                    .filter(|(_, static_trans, _, static_ext, _)| {
                        let static_box = static_ext
                            .bbox()
                            .transform_by(&translate_trans(&static_trans));
                        bbox.intersects(&static_box)
                    })
                    .map(|static_e| {
                        (
                            (static_e.4, Collision::new(*dyn_kind)),
                            (dyn_ent, Collision::new(*static_e.0)),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(coll1, coll2)| {
                collisions
                    .insert(coll1.0, coll1.1)
                    .expect("Unable to insert collision.");
                collisions
                    .insert(coll2.0, coll2.1)
                    .expect("Unable to insert collision.");
            });
    }
}
