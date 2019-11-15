use nalgebra::*;
use specs_transform::Transform3D;
use std::time::Duration;
use std::f32::consts::PI;

/// Left bound of the level.
pub(crate) static LEFT_BOUND: D = 7.;
/// Right bound of the level.
pub(crate) static RIGHT_BOUND: D = -7.;

/// Left bound of the level.
pub(crate) static LEFT_BOUND: D = 7.;
/// Right bound of the level.
pub(crate) static RIGHT_BOUND: D = -7.;

/// Aliases and helpful functions that can be used throughout the application.

/// The precision of the game simulation.
pub type D = f32;

/// The Vector type used in the Simulation.
pub type Vector = Vector3<D>;

/// The Point type used in the Simulation.
pub type Point = Point3<D>;

pub type Transform = Transform3D<D>;

pub fn duration_float(d: Duration) -> f32 {
    (d.as_secs() as f32 * 1.) + (d.subsec_millis() as f32 * 0.001)
}

/// Translates a specs Transform to a nalgebra Isometry3 (Transform)
/// TODO Check whether this rotation is ok.
pub fn translate_trans(t: &Transform) -> Isometry3<D> {
    let translation: Translation3<D> = Translation3::from(Vector::from(t.position));
    Isometry3::from_parts(
        translation,
        UnitQuaternion::from_axis_angle(&Vector::x_axis(), PI / 2.),
    )
}
