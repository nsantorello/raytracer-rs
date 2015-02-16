use math::Vec3;

pub trait Shape {
    fn intersects(&self, ray: &Vec3) -> bool;
}

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3
}

impl Shape for Cube {
    fn intersects(&self, ray: &Vec3) -> bool {
        false
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Shape for Sphere {
    fn intersects(&self, ray: &Vec3) -> bool {
        true
    }
}