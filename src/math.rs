use std::ops::{Add, Div, Mul, Sub};

use crate::renderers::{self, Color};
pub mod hittables;

const PI: f64 = 3.1415926535897932385;

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
        self.length().powi(2)
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

impl From<Vec3> for renderers::Color {
    fn from(val: Vec3) -> Self {
        Color::new(
            (val.x * 255.99) as u8,
            (val.y * 255.99) as u8,
            (val.z * 255.99) as u8,
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
            self.normal = outward_normal.clone();
        } else {
            self.normal = -1.0 * (outward_normal.clone());
        }
    }
}

pub enum HitResult {
    NoHit,
    Hit(HitRecord),
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult;
}
