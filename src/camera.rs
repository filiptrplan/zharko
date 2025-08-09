use indicatif::ProgressBar;
use rand::Rng;

use crate::{
    math::{interval::Interval, HitResult, Hittable, Ray, Vec3},
    renderers::{Image, Renderer},
};

pub struct CameraBuilder {}

const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    samples_per_pixel: u8,
    /// How much should we scale the color of each sample for a pixel
    pixel_scale_factor: f64,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    image: Image,
    max_depth: i32,
}

impl Camera {
    pub fn new(image: Image) -> Self {
        let image_width = image.width;
        let image_height = image.height;

        // We re-calculate the aspect ratio because when calculating the image width we can
        // introduce rounding errors.
        let viewport_width = VIEWPORT_HEIGHT * (image_width as f64 / image_height as f64);

        // Camera is placed at origin
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        // We are using right-handed coordinates: y is up, x is right, negative z is the camera dir
        // Vectors describing the viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        // Pixel-to-pixel deltas
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Location of the upper left pixel
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            samples_per_pixel: 20,
            max_depth: 10,
            pixel_scale_factor: 1.0 / 20.0,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            image,
        }
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        )
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = self.camera_center;
        let ray_dir = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }

    pub fn render(&mut self, renderer: impl Renderer, world: &impl Hittable) {
        let bar = ProgressBar::new(self.image.height as u64);

        for j in 0..self.image.height {
            bar.inc(1);
            for i in 0..self.image.width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color = color + Camera::ray_color(&ray, self.max_depth, world);
                }

                color = color * self.pixel_scale_factor;

                self.image.set_pixel(i, j, color.into());
            }
        }

        bar.finish();
        renderer.draw(&self.image);
    }

    fn ray_color(r: &Ray, depth: i32, world: &impl Hittable) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        // We ignore hits very close to the surface as they could be below the surface and
        // cause rays to bounce inside the object
        // The phenomen is called "shadow acne"
        match world.hit(r, Interval::new(0.0001, f64::INFINITY)) {
            HitResult::Hit(rec) => {
                let dir = Vec3::random_on_hemisphere(rec.normal);
                return 0.5 * Camera::ray_color(&Ray::new(rec.point, dir), depth - 1, world);
            }
            HitResult::NoHit => (),
        }

        let unit_dir = r.dir.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
