use indicatif::ProgressBar;
use rand::Rng;

use crate::{
    math::{degrees_to_radians, interval::Interval, HitResult, Hittable, Ray, Vec3},
    renderers::{Image, Renderer},
};

pub struct CameraBuilder {}

const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    samples_per_pixel: u16,
    /// How much should we scale the color of each sample for a pixel
    pixel_scale_factor: f64,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    image: Image,
    max_depth: i32,
    /// Vertical FOV
    vfov: f64,
    /// Where is the camera looking from
    lookfrom: Vec3,
    /// Where is the camera looking at
    lookat: Vec3,
    /// The up direction for the camera
    vup: Vec3,
    /// The three basis vectors for the camera coordinate system
    u: Vec3,
    v: Vec3,
    w: Vec3,
    /// Angle of disk that is the base of a cone with the apex at the focus plane and the base at
    /// the camera center
    defocus_angle: f64,
    /// Distance from the `lookfrom` point to the focus plane
    focus_dist: f64,
    /// Defocus dist vertical radius
    defocus_disk_v: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
}

impl Camera {
    pub fn new(image: Image) -> Self {
        let vfov = 90.0;
        let mut camera = Self {
            defocus_disk_v: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            samples_per_pixel: 20,
            max_depth: 10,
            pixel_scale_factor: 1.0 / 20.0,
            camera_center: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel00_loc: Vec3::zero(),
            image,
            vfov,
            lookat: Vec3::new(0.0, 0.0, -1.0),
            lookfrom: Vec3::zero(),
            u: Vec3::zero(),
            vup: Vec3::new(0.0, 1.0, 0.0),
            v: Vec3::zero(),
            w: Vec3::zero(),
        };

        camera.initialize();
        camera
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        )
    }

    pub fn set_aperture(&mut self, defocus_angle: f64, focus_dist: f64) {
        self.defocus_angle = defocus_angle;
        self.focus_dist = focus_dist;
        self.initialize();
    }

    /// Construct a ray with the origin point randomly sampled from the defocus disk and pointing
    /// through the pixel at (i,j)
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let rand_disk = Vec3::random_in_unit_disk();
        self.camera_center + rand_disk.x * self.defocus_disk_u + rand_disk.y * self.defocus_disk_v
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

    pub fn set_samples_per_pixel(&mut self, samples: u16) {
        self.samples_per_pixel = samples;
        self.pixel_scale_factor = 1.0 / samples as f64;
    }

    pub fn set_max_depth(&mut self, depth: i32) {
        self.max_depth = depth;
    }

    pub fn set_vfov(&mut self, vfov: f64) {
        self.vfov = vfov;
        self.initialize();
    }

    pub fn set_camera_pos(&mut self, lookfrom: Vec3, lookat: Vec3) {
        self.lookat = lookat;
        self.lookfrom = lookfrom;
        self.initialize();
    }

    fn initialize(&mut self) {
        let image_width = self.image.width;
        let image_height = self.image.height;

        self.camera_center = self.lookfrom;

        // Calculate viewport dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;

        // We re-calculate the aspect ratio because when calculating the image width we can
        // introduce rounding errors.
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(&self.w);
        self.v = self.w.cross(&self.u);

        // We are using right-handed coordinates: y is up, x is right, negative z is the camera dir
        // Vectors describing the viewport
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -1.0 * self.v;

        // Pixel-to-pixel deltas
        self.pixel_delta_u = viewport_u / image_width as f64;
        self.pixel_delta_v = viewport_v / image_height as f64;

        // Location of the upper left pixel
        let viewport_upper_left = self.camera_center
            - (self.focus_dist * self.w)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the defocus disk dimensions
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(r: &Ray, depth: i32, world: &impl Hittable) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        // We ignore hits very close to the surface as they could be below the surface and
        // cause rays to bounce inside the object
        // The phenomen is called "shadow acne"
        if let HitResult::Hit(rec) = world.hit(r, Interval::new(0.0001, f64::INFINITY)) {
            // We choose the direction according to the Lambertian distribution.
            if let Some(scatter_res) = rec.mat.scatter(r, &rec) {
                return scatter_res.attenuation
                    * Camera::ray_color(&scatter_res.scattered, depth - 1, world);
            } else {
                return Vec3::zero();
            }
        }

        let unit_dir = r.dir.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
