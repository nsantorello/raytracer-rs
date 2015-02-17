mod cube;
pub use self::cube::Cube;

mod sphere;
pub use self::sphere::Sphere;

use math::Vec3;
use std::fmt::Debug;

pub trait Shape : Debug {
    fn intersects(&self, ray: &Vec3) -> bool;
}