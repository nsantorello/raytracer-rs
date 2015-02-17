use math::{Point3, Vec3};

use super::Shape;

#[derive(Debug)]
pub struct Cube {
    pub min: Point3,
    pub max: Point3
}

impl Shape for Cube {
    fn intersects(&self, ray: &Vec3) -> bool {
        false
    }
}