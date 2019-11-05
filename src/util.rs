use nalgebra::*;
use std::time::Duration;

/// Aliases and helpful functions that can be used throughout the application.

/// The precision of the game simulation.
pub type D = f32;

/// The Vector type used in the Simulation.
pub type Vector = Vector3<D>;

/// The Point type used in the Simulation.
pub type Point = Point3<D>;

pub fn duration_float(d: Duration) -> f32 {
    (d.as_secs() as f32 * 1.) + (d.subsec_millis() as f32 * 0.001)
}
