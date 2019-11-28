mod node_builder;

use crate::util::*;
use nc::bounding_volume::aabb::AABB;
pub use node_builder::NodeBuilder;
use specs::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Force(pub [D; 3]);

impl Force {
    const GRAVITATIONAL: D = -9.80;

    pub fn y(y: D) -> Self {
        Self([0., y, 0.])
    }

    pub fn gravity() -> Self {
        Self::y(Self::GRAVITATIONAL)
    }
}

impl Component for Force {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub enum ObjectKind {
    Player,
    Obstacle,
}

impl Component for ObjectKind {
    type Storage = DenseVecStorage<Self>;
}

// Velocity conversion to easily reset the velocity of a thing to something standardized
impl Into<Vel> for ObjectKind {
    fn into(self) -> Vel {
        match self {
            ObjectKind::Player => Vel([0., 0., 20.]),
            ObjectKind::Obstacle => Vel([0., 0., 0.]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Collision(pub ObjectKind);

impl Collision {
    pub fn new(k: ObjectKind) -> Self {
        Self(k)
    }
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct Vel(pub [D; 3]);

impl From<[D; 3]> for Vel {
    fn from(i: [D; 3]) -> Self {
        Vel(i)
    }
}

impl Into<[D; 3]> for &Vel {
    fn into(self) -> [D; 3] {
        self.0
    }
}

impl Into<Vector> for Vel {
    fn into(self) -> Vector {
        let points: [D; 3] = (&self).into();
        Vector::from(points)
    }
}

impl Component for Vel {
    type Storage = DenseVecStorage<Self>;
}

/// Hitbox with different shapes
pub type BBox = AABB<D>;
pub struct Extent(pub BBox);

impl Component for Extent {
    type Storage = DenseVecStorage<Self>;
}

impl Extent {
    pub fn new(scale: D) -> Self {
        Self(BBox::from_half_extents(
            Point::origin(),
            Vector::from([1., 1., 1.]).scale(scale),
        ))
    }

    pub fn bbox(&self) -> &BBox {
        &self.0
    }
}

pub struct Health {
    pub current: i32,
    pub full: i32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

impl Health {
    pub fn one() -> Self {
        Self::at_full(1)
    }

    pub fn at_full(full: i32) -> Self {
        Self {
            current: full,
            full: full,
        }
    }
}
