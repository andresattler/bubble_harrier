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

gfx_vertex_struct!(Vertex {
    a_pos: [i8; 4] = "a_pos",
    a_color: [i8; 4] = "a_color",
});

impl Vertex {
    pub fn new(pos: [i8; 3], color: [i8; 4]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1],
            a_color: color,
        }
    }
}
