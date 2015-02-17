use math::{Point3, Vec3};

#[derive(Debug)]
pub struct Camera {
    pub pos: Point3,
    pub look: Vec3,
    pub up: Vec3,
}