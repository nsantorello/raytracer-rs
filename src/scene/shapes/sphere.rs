use math::{Point3, Vec3};

use super::Shape;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Shape for Sphere {
    fn intersects(&self, ray: &Vec3) -> bool {
        true
    }
}