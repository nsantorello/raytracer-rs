use math::Vec3;

struct Cube {
    min: Vec3,
    max: Vec3
}

impl Shape for Cube {
    fn intersects(&self, ray: &Vec3) -> bool {
        false
    }
}