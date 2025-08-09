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

    // Camera settings
    camera.set_samples_per_pixel(100);
    camera.set_max_depth(50);
    camera.set_vfov(90.0);
    camera.set_camera_pos(Vec3::new(-2.0, 2.0, 1.0), Vec3::new(0.0, 0.0, -1.0));

    // Materials
    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
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
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    camera.render(renderer, &world);
}
