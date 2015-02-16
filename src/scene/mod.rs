use math::Vec3;
use shapes::Shape;

pub struct Scene<'a> {
    pub objects: Vec<&'a (Shape + 'a)>
}

pub struct Camera {
    pub loc: Vec3,
    pub up: Vec3,
    pub look: Vec3
}