use derive_more::{Add, AddAssign, Mul};

use crate::aabb::AxisAlignedBoundingBox;

use rand::Rng;

#[derive(Default, Debug, Clone, Copy, Add, AddAssign, Mul)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    pub fn random_uniform_in_centered_unit_square(rng: &mut impl Rng) -> Self {
        Point2D {
            x: rng.gen_range(-0.5..0.5),
            y: rng.gen_range(-0.5..0.5),
        }
    }

    pub fn random_uniform_in_square(center: &Point2D, size: f64, rng: &mut impl Rng) -> Self {
        Point2D {
            x: center.x + 0.5 * size * rng.gen_range(-1.0..1.0),
            y: center.y + 0.5 * size * rng.gen_range(-1.0..1.0),
        }
    }

    pub fn random_uniform_in_aabb(aabb: &AxisAlignedBoundingBox, rng: &mut impl Rng) -> Self {
        Point2D {
            x: rng.gen_range(aabb.min_x..aabb.max_x),
            y: rng.gen_range(aabb.min_y..aabb.max_y),
        }
    }
}
