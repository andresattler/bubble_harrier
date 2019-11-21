use crate::{components::*, resources::*, util::*};
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
        ReadExpect<'s, Player>,
        ReadStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (player, transforms, entities, updater): Self::SystemData) {
        if let Some(ptrans) = transforms.get(**player) {
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
