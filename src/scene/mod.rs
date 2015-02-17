use math::{Point3, Vec3};
use shapes::Shape;

#[derive(Debug)]
pub struct Scene<'a> {
    pub objects: Vec<&'a (Shape + 'a)>,
    pub camera: Camera
}

#[derive(Debug)]
pub struct Camera {
    pub pos: Point3,
    pub look: Vec3,
    pub up: Vec3,
}