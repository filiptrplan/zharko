use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use interval::Interval;
use rand::Rng;

use crate::renderers::{self, Color};
pub mod hittables;
pub mod interval;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // Dot product
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // Length/magnitude of the vector
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Unit vector (normalized)
    pub fn unit(&self) -> Vec3 {
        let len = self.length();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self::new(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        )
    }

    /// Generates a random vector on the surface of the unit sphere
    pub fn random_unit_vector() -> Self {
        // We use rejection sampling because it is much harder to directly generate
        // uniformly distributed points on a sphere
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let l = p.length_squared();
            if 1e-16 < l && l <= 1.0 {
                return p / l.sqrt();
            }
        }
    }

    /// Generates a random unit vector on the hemisphere described by normal
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(&normal.unit()) >= 0.0 {
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::rng();
        let range = min..max;
        Self::new(
            rng.random_range(range.clone()),
            rng.random_range(range.clone()),
            rng.random_range(range),
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vector subtraction
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Scalar multiplication (Vec3 * f64)
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Scalar multiplication (f64 * Vec3)
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3::new(self * vec.x, self * vec.y, self * vec.z)
    }
}

// Scalar multiplication (Vec3 * usize)
impl Mul<usize> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: usize) -> Self::Output {
        let scalar_f64 = scalar as f64;
        Vec3::new(
            self.x * scalar_f64,
            self.y * scalar_f64,
            self.z * scalar_f64,
        )
    }
}
// Scalar division
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self::Output {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// Scalar division (Vec3 / usize)
impl Div<usize> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: usize) -> Self::Output {
        let scalar_f64 = scalar as f64;
        Vec3::new(
            self.x / scalar_f64,
            self.y / scalar_f64,
            self.z / scalar_f64,
        )
    }
}

// Element-wise multiplication (Vec3 * Vec3)
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

fn linear_to_gamma(x: f64) -> f64 {
    if x > 0.0 {
        x.sqrt()
    } else {
        0.0
    }
}

impl From<Vec3> for renderers::Color {
    fn from(val: Vec3) -> Self {
        let interval = Interval::new(0.0, 0.9999);
        let r = linear_to_gamma(val.x);
        let g = linear_to_gamma(val.y);
        let b = linear_to_gamma(val.z);
        Color::new(
            (interval.clamp(r) * 256.0) as u8,
            (interval.clamp(g) * 256.0) as u8,
            (interval.clamp(b) * 256.0) as u8,
        )
    }
}

/// The `Ray` struct represents a ray with a certain origin and direction.
/// It also implements the function `at` that can calculate any point along the ray at a certain
/// scalar `t`
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    /// This is the scalar of the ray we hit it at
    pub t: f64,
    /// Tracks whether we hit the front face of the object
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the normal vector of the hit record. The `outward_normal` parameter is the normal that
    /// points to the outside of the object.
    /// This function sets the normal so it always points to the origin of the ray.
    /// We assume that outward_normal has a unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -1.0 * (*outward_normal);
        }
    }
}

pub enum HitResult {
    NoHit,
    Hit(HitRecord),
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> HitResult;
}
