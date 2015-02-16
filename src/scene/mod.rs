use math::Vec3;
use shapes::Shape;

pub struct Scene<'a> {
    pub objects: Vec<&'a (Shape + 'a)>
}

pub struct Camera {
    pub pos: Vec3,
    pub look: Vec3,
    pub up: Vec3,
}