use scene::Scene;
use math::Vec3;
use scene::shapes::Shape;

#[derive(Debug)]
pub struct Raytracer<'a> {
    pub width: i32,
    pub height: i32,
    pub scene: Scene<'a>
}

impl<'a> Raytracer<'a> {
    pub fn generate_image(&self) {
        let ray = Vec3 { x: 1., y: 2., z: 3. };
        for obj in self.scene.objects.iter() {
            println!("is circle: {}", obj.intersects(&ray));
        }
    }
}