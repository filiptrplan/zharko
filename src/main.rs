use core::f64;
use std::rc::Rc;

use zharko::{
    camera::Camera,
    math::{
        hittables::{HittableList, Sphere},
        materials::{Dielectric, Lambertian, Metal},
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

    let r = (f64::consts::PI / 4.0).cos();

    // Materials
    let material_left = Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    camera.render(renderer, &world);
}
