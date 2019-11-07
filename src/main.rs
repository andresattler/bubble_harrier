extern crate kiss3d;

mod sim;
mod util;

use kiss3d::{camera::ArcBall, light::Light, window::Window};
use nalgebra::{Translation3, UnitQuaternion};
use std::f32::consts::PI;
use util::*;

fn main() {
    let mut window = Window::new("GameJam 2019");
    let mut c = window.add_cube(1.0, 1.0, 1.0);
    let mut ground = window.add_quad(10., 100., 5, 5);
    let mut camera = ArcBall::new([0., 1., 0.].into(), [0., 1., 2.].into());

    ground.set_color(1., 1., 1.);
    c.set_color(1.0, 0.0, 0.0);
    c.append_translation(&Translation3::new(0., 0.5, 5.));

    window.set_light(Light::StickToCamera);
    window.set_background_color(0.1, 0.1, 1.);

    let rot = UnitQuaternion::from_axis_angle(&Vector::y_axis(), PI / 4.);
    let grot = UnitQuaternion::from_axis_angle(&Vector::x_axis(), PI / 2.);

    ground.append_rotation(&grot);
    ground.append_translation(&Translation3::new(0., 0., 0.));
    c.prepend_to_local_rotation(&rot);
    while window.render_with_camera(&mut camera) {
        let translation = Vector::new(0., 0., 0.1);
        c.append_translation(&Translation3::from(translation));
        camera.set_at(camera.at() + translation);
    }
}
