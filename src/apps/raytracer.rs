#[derive(Debug)]
struct Raytracer<'a> {
    width: i32,
    height: i32,
    scene: Scene<'a>
}

impl<'a> Raytracer<'a> {
    fn generateImage(&self) {
        let ray = Vec3 { x: 1., y: 2., z: 3. };
        for obj in self.scene.objects.iter() {
            println!("is circle: {}", obj.intersects(&ray));
        }
    }
}