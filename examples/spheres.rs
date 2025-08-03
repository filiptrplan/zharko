use core::f64;

use zharko::{
    math::{
        hittables::{HittableList, Sphere},
        interval::Interval,
        HitResult, Hittable, Ray, Vec3,
    },
    renderers::{Image, Renderer, PPM},
};

const IMAGE_WIDTH: usize = 500;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
    match world.hit(r, Interval::new(0.0, f64::INFINITY)) {
        HitResult::Hit(rec) => {
            return 0.5 * (rec.normal.unit() + Vec3::new(1.0, 1.0, 1.0));
        }
        HitResult::NoHit => (),
    }

    let unit_dir = r.dir.unit();
    let a = 0.5 * (unit_dir.y + 1.0);

    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let renderer = PPM::new();

    let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let mut image = Image::new(IMAGE_WIDTH, image_height);

    // We re-calculate the aspect ratio because when calculating the image width we can
    // introduce rounding errors.
    let viewport_width = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / image_height as f64);

    // Camera is placed at origin
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // We are using right-handed coordinates: y is up, x is right, negative z is the camera dir
    // Vectors describing the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Pixel-to-pixel deltas
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - (viewport_u / 2.0) - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -101.0, -1.0), 100.0)));

    for j in 0..image_height {
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + pixel_delta_u * i + pixel_delta_v * j;
            let ray_dir = pixel_center - camera_center;

            let ray = Ray::new(pixel_center, ray_dir);
            let color = ray_color(&ray, &world);

            image.set_pixel(i, j, color.into());
        }
    }

    renderer.draw(&image);
}
