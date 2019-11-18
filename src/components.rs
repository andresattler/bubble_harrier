use crate::util::*;
use nalgebra::origin;
use nc::bounding_volume::aabb::AABB;
use specs::prelude::*;
use std::convert::AsRef;

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

impl ObjectKind {
    #[inline]
    pub fn is_player(self) -> bool {
        match self {
            ObjectKind::Obstacle => false,
            ObjectKind::Player => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Collision;

impl Component for Collision {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct Vel([D; 3]);

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
