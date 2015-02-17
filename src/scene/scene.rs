use math::{Point3, Vec3};
use super::shapes::Shape;
use super::camera::Camera;

#[derive(Debug)]
pub struct Scene<'a> {
    pub objects: Vec<&'a (Shape + 'a)>,
    pub camera: Camera
}