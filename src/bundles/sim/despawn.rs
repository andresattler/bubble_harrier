use crate::{resources::*, util::*};
use specs::prelude::*;
use std::ops::Range;

pub struct DespawnSystem;

impl DespawnSystem {
    const DESPAWN_FRAME: (D, D) = (-5., 300.);

    pub fn name() -> &'static str {
        "sim::despawn_system"
    }

    /// Returns the frame in which things are allowed to exist in this frame
    #[inline]
    fn frame_range(ptrans: &Transform) -> Range<D> {
        let (back, forw) = DespawnSystem::DESPAWN_FRAME;
        let behind = ptrans.position[2] + back;
        let front = ptrans.position[2] + forw;
        behind..front
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
            let current_range = Self::frame_range(ptrans);
            let oob_ents = (&transforms, &entities)
                .join()
                .filter(|(_, ent)| *ent != **player)
                .filter(|(trans, _)| !current_range.contains(&trans.position[2]))
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
