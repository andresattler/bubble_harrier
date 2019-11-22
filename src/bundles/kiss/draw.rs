use super::SharedWindow;
use crate::{components::*, util::*};
use kiss3d::scene::SceneNode;
use specs::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

pub struct DrawSystem {
    win: SharedWindow,
    nodes: BTreeMap<Entity, SceneNode>,
}

impl DrawSystem {
    pub fn new(win: SharedWindow) -> Self {
        Self {
            win,
            nodes: Default::default(),
        }
    }

    /// Clean all nodes except for those that were seen in the rendering run.
    fn clean_except(&mut self, seen: &BTreeSet<Entity>) {
        let nodes = &mut self.nodes;
        let mut win = self.win.borrow_mut();
        let not_seen = {
            nodes
                .keys()
                .filter(|ent| !seen.contains(&ent))
                .cloned()
                .collect::<Vec<_>>()
        };
        not_seen
            .iter()
            .filter_map(|ent| nodes.remove(ent))
            .for_each(|mut node| {
                win.remove_node(&mut node);
            });
    }
}

impl<'s> specs::System<'s> for DrawSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, NodeBuilder>,
        Entities<'s>,
    );

    /// Updates all nodes that are still attached to entities.
    /// If entities are not seen, they are cleaned up afterwards.
    fn run(&mut self, (transforms, mut node_builders, entities): Self::SystemData) {
        let seen = {
            let nodes = &mut self.nodes;
            let mut win = self.win.borrow_mut();
            (&transforms, &mut node_builders, &entities)
                .join()
                .map(|(trans, builder, ent)| {
                    nodes
                        .entry(ent)
                        .or_insert_with(|| builder.build(&mut win))
                        .set_local_transformation(translate_trans(&trans));
                    ent
                })
                .collect()
        };
        self.clean_except(&seen);
    }
}
