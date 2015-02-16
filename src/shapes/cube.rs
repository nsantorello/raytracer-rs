use math::Vec3;

use super::Shape;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3
}

impl Shape for Cube {
    fn intersects(&self, ray: &Vec3) -> bool {
        false
    }
}