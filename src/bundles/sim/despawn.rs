use crate::{components::*, util::*};
use specs::prelude::*;

pub struct DespawnSystem;

impl DespawnSystem {
    const DESPAWN_FRAME: (D, D) = (-5., 300.);

    pub fn name() -> &'static str {
        "sim::despawn_system"
    }
}

impl<'s> specs::System<'s> for DespawnSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, ObjectKind>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (transforms, kinds, entities, updater): Self::SystemData) {
        if let Some((_, ptrans)) = (&kinds, &transforms)
            .join()
            .find(|(obj, _)| obj.is_player())
        {
            let oob_ents = (&transforms, &entities)
                .join()
                .filter(|(trans, _)| {
                    trans.position[2] < (ptrans.position[2] + Self::DESPAWN_FRAME.0)
                        || trans.position[2] > (ptrans.position[2] + Self::DESPAWN_FRAME.1)
                })
                .map(|(_, ent)| ent)
                .collect::<Vec<_>>();
            updater.exec_mut(move |world| {
                world
                    .delete_entities(oob_ents.as_slice())
                    .expect("Unable to remove entities!")
            })
        }
    }
}
