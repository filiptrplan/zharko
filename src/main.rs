use core::f64;
use std::rc::Rc;

use zharko::{
    camera::Camera,
    math::{
        hittables::{HittableList, Sphere},
        materials::{Lambertian, Metal},
        Vec3,
    },
    renderers::{Image, PPM},
};

const IMAGE_WIDTH: usize = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let renderer = PPM::new();

    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let image = Image::new(IMAGE_WIDTH, image_height);
    let mut camera = Camera::new(image);

    // Materials
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    camera.render(renderer, &world);
}
