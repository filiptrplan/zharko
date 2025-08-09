use core::f64;
use std::rc::Rc;

use rand::Rng;
use zharko::{
    camera::Camera,
    math::{
        hittables::{HittableList, Sphere},
        materials::{Dielectric, Lambertian, Metal},
        Vec3,
    },
    renderers::{Image, PPM},
};

const IMAGE_WIDTH: usize = 1200;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let renderer = PPM::new();

    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let image = Image::new(IMAGE_WIDTH, image_height);
    let mut camera = Camera::new(image);

    // Camera settings
    camera.set_samples_per_pixel(500.0);
    camera.set_max_depth(50);
    camera.set_vfov(20.0);
    camera.set_camera_pos(Vec3::new(13.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0));
    camera.set_aperture(0.6, 10.0);

    // World
    let mut world = HittableList::new();
    let mut rng = rand::thread_rng();

    // Ground
    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Generate many small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
                        * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Three large spheres
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    camera.render(renderer, &world);
}
