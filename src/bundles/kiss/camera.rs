use super::SharedWindow;
use crate::{components::*, util::*};
use kiss3d::camera::ArcBall;
use specs::prelude::*;

pub struct CameraSystem {
    win: SharedWindow,
    camera: ArcBall,
}

/// System that moves the camera and uses it to render.
impl CameraSystem {
    pub fn new(win: SharedWindow) -> Self {
        let camera = ArcBall::new([0., 0.5, 0.].into(), [0., 0.5, 2.].into());
        Self { camera, win }
    }
}

impl<'s> specs::System<'s> for CameraSystem {
    type SystemData = (ReadStorage<'s, Transform>, ReadStorage<'s, ObjectKind>);

    fn run(&mut self, (transforms, object_kinds): Self::SystemData) {
        for (trans, kind) in (&transforms, &object_kinds).join() {
            if let ObjectKind::Player = kind {
                let mut new_pos = trans.position;
                new_pos[0] = 0.;
                new_pos[1] += 3.;
                new_pos[2] -= 10.;
                self.camera.set_at(new_pos.into());
                self.win.borrow_mut().render_with_camera(&mut self.camera);
                break;
            }
        }
    }
}
