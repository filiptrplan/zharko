use super::{reflect, HitRecord, Ray, Vec3};

pub struct ScatterResult {
    /// How much of the incoming ray will be attenuated (absorbed)
    pub attenuation: Vec3,
    /// The scattered ray
    pub scattered: Ray,
}

pub trait Material {
    /// Some means that the ray scattered, `None` means that the ray was absorbed.
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

/// A material implementing lambertian reflectance. In this implementation
/// we always assume that the ray always scatters (no probabilistic scattering).
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();
        // If the scatter direction is near 0 then we don't want to deal with floating point
        // arithmetic near zero.
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        Some(ScatterResult {
            scattered: Ray::new(rec.point, scatter_dir),
            attenuation: self.albedo,
        })
    }
}

/// A reflective material
pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected =
            reflect(&r.dir, &rec.normal).unit() + (self.fuzziness * Vec3::random_unit_vector());
        // If the reflected fuzzed ray is below the surface, we just absorb the ray
        if reflected.dot(&rec.normal) < 0.0 {
            return None;
        }
        Some(ScatterResult {
            scattered: Ray::new(rec.point, reflected),
            attenuation: self.albedo,
        })
    }
}
