use math::Vec3;

use super::Shape;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Shape for Sphere {
    fn intersects(&self, ray: &Vec3) -> bool {
        true
    }
}