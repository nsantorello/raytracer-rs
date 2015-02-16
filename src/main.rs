mod math;
use math::Vec3;

mod shapes;
use shapes::{Shape, Cube, Sphere};

mod scene;
use scene::{Scene, Camera};

fn main() {
    let ray = Vec3 { x: 1., y: 2., z: 3. };
    let center = Vec3 { x: 1., y: 2., z: 3. };
    let centerY = Vec3 { x: 2., y: 2., z: 3. };

    let x = Sphere { radius: 10., center: center };
    let y = Sphere { radius: 10., center: centerY };
    let cube = Cube { min: Vec3 { x: 1., y: 1., z: 1. }, max: Vec3 { x: 1., y: 1., z: 1. } };
    println!("{} {} {} {}", x.radius, x.center.x, x.center.y, x.center.z);
    
    let scene = Scene { objects: vec![ &x, &y, &cube ]};
    for obj in scene.objects.iter() {
        println!("{}", obj.intersects(&ray));
    }
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