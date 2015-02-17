mod math;
use math::{Point3, Vec3};

mod shapes;
use shapes::{Shape, Cube, Sphere};

mod scene;
use scene::{Scene, Camera};

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

fn main() {

    let center = Point3 { x: 1., y: 2., z: 3. };
    let center3 = Point3 { x: 1., y: 2., z: 3. };
    let centerY = Point3 { x: 2., y: 2., z: 3. };
    let center2 = Point3 { x: 2., y: 2., z: 3. };
    let center4 = Point3 { x: 2., y: 2., z: 3. };

    let x = Sphere { radius: 10., center: center + center4 };
    let y = Sphere { radius: 10., center: centerY };
    let cube = Cube { min: Point3 { x: 1., y: 1., z: 1. }, max: Point3 { x: 1., y: 1., z: 1. } };
    let scene = Scene { objects: vec![ &x, &y, &cube ], camera: Camera { pos: Point3 { x: 1., y: 1., z: 1. }, look: Vec3 { x: 1., y: 1., z: 1. }, up: Vec3 { x: 1., y: 1., z: 1. }} };

    let raytracer = Raytracer { width: 800, height: 600, scene: scene };
    println!("raytracer image size: ({},{})", raytracer.width, raytracer.height);
    raytracer.generateImage();

    println!("{:?}", raytracer);
}



// extern crate image;

// use std::old_io::File;

// use image::GenericImage;

// fn main() {
//     //Use the open function to load an image from a PAth.
//     //```open``` returns a dynamic image.
//     let img = image::open(&Path::new("test.jpg")).unwrap();

//     //The dimensions method returns the images width and height
//     let (w, h) = img.dimensions();
//     println!("dimensions: ({}, {})", w, h);

//     //The color method returns the image's ColorType

//     let color = match img.color() {
//     	image::ColorType::RGB(_) => "RGB",
//     	image::ColorType::Gray(_) => "Gray",
//     	image::ColorType::Palette(_) => "Palette",
//     	image::ColorType::GrayA(_) => "GrayA",
//     	image::ColorType::RGBA(_) => "RGBA"
//     };
//     println!("color type: {}", color);

//     let fout = &mut File::create(&Path::new("test.png")).unwrap();

//     //Write the contents of this image to the Writer in PNG format.
//     let _ = img.save::<File>(fout, image::PNG);
// }