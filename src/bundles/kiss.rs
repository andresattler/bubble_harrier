use crate::{components::*, util::*};
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use nalgebra::{Translation3, UnitQuaternion};
use specs::prelude::*;
use specs_bundler::{Bundle, Bundler};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

struct DrawSystem {
    win: SharedWindow,
    nodes: Vec<SceneNode>,
}
struct CameraSystem {
    win: SharedWindow,
    camera: ArcBall,
}

/// System that moves the camera and uses it to render.
impl CameraSystem {
    pub fn new(win: SharedWindow) -> Self {
        let camera = ArcBall::new([0., 1., 0.].into(), [0., 1., 2.].into());
        Self { camera, win }
    }
}

impl<'s> specs::System<'s> for CameraSystem {
    type SystemData = (ReadStorage<'s, Transform>, ReadStorage<'s, ObjectKind>);

    fn run(&mut self, (transforms, object_kinds): Self::SystemData) {
        for (trans, kind) in (&transforms, &object_kinds).join() {
            if let ObjectKind::Player = kind {
                self.camera
                    .set_at(Point::from(trans.position) + Vector::from([0., 10., -10.0]));
                self.win.borrow_mut().render_with_camera(&mut self.camera);
                break;
            }
        }
    }
}

type SharedWindow = Rc<RefCell<Window>>;

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

#[derive(Debug, Default)]
pub struct KissBundle<'deps> {
    deps: Vec<&'deps str>,
}

impl<'deps, 'world, 'a, 'b> Bundle<'world, 'a, 'b> for KissBundle<'deps> {
    type Error = ();

    #[inline]
    fn bundle(
        mut self,
        mut bundler: Bundler<'world, 'a, 'b>,
    ) -> Result<Bundler<'world, 'a, 'b>, Self::Error> {
        let mut window = Window::new("Gamejam Yeah");

        setup_box(&mut window);
        setup_ground(&mut window);
        let shared_window = Rc::new(RefCell::new(window));
        let draw = DrawSystem::new(shared_window.clone());
        let camera = CameraSystem::new(shared_window.clone());

        bundler.dispatcher_builder = bundler
            .dispatcher_builder
            .with_thread_local(draw)
            .with_thread_local(camera);
        Ok(bundler)
    }
}

fn setup_box(window: &mut Window) {
    window.set_light(Light::StickToCamera);
    window.set_background_color(0.1, 0.1, 1.);
}

fn setup_ground(window: &mut Window) {
    let mut ground = window.add_quad(10., 100., 5, 5);
    let grot = UnitQuaternion::from_axis_angle(&Vector::x_axis(), PI / 2.);
    ground.append_rotation(&grot);
    ground.append_translation(&Translation3::new(0., 0., 0.));
}
