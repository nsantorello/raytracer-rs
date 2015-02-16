mod cube;
pub use self::cube::Cube;

mod sphere;
pub use self::sphere::Sphere;

use math::Vec3;

pub trait Shape {
    fn intersects(&self, ray: &Vec3) -> bool;
}