use super::SharedWindow;
use crate::{components::*, util::*};
use kiss3d::scene::SceneNode;
use specs::prelude::*;

pub struct DrawSystem {
    win: SharedWindow,
    nodes: Vec<SceneNode>,
}

impl DrawSystem {
    pub fn new(win: SharedWindow) -> Self {
        Self {
            win,
            nodes: Vec::new(),
        }
    }

    /// I'll be damned if this works.
    fn clear(&mut self) {
        let mut new_nodes = Vec::new();
        std::mem::swap(&mut self.nodes, &mut new_nodes);
        new_nodes
            .into_iter()
            .for_each(|mut node| self.win.borrow_mut().remove_node(&mut node));
    }
}

impl<'s> specs::System<'s> for DrawSystem {
    type SystemData = (ReadStorage<'s, Transform>, ReadStorage<'s, ObjectKind>);

    fn run(&mut self, (transforms, object_kinds): Self::SystemData) {
        self.clear();
        for (trans, kind) in (&transforms, &object_kinds).join() {
            let mut node = match kind {
                ObjectKind::Player => self.win.borrow_mut().add_cube(1., 1., 1.),
                ObjectKind::Obstacle => self.win.borrow_mut().add_sphere(1.),
            };
            node.append_transformation(&translate_trans(&trans));
            self.nodes.push(node);
        }
    }
}
