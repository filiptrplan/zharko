use core::f64;

use zharko::{
    camera::Camera,
    math::{
        hittables::{HittableList, Sphere},
        Vec3,
    },
    renderers::{Image, PPM},
};

const IMAGE_WIDTH: usize = 300;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let renderer = PPM::new();

    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let image = Image::new(IMAGE_WIDTH, image_height);
    let mut camera = Camera::new(image);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -101.0, -1.0), 100.0)));

    camera.render(renderer, &world);
}
