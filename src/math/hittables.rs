use super::{HitRecord, HitResult, Hittable, Ray, Vec3};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }

    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for &HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult {
        let mut res = HitResult::NoHit;
        let mut closest_so_far = ray_tmax;

        for obj in &self.objects {
            match obj.hit(r, ray_tmin, ray_tmax) {
                HitResult::NoHit => (),
                HitResult::Hit(rec) => {
                    if closest_so_far > rec.t {
                        closest_so_far = rec.t;
                        res = HitResult::Hit(rec);
                    }
                }
            }
        }

        res
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &super::Ray, ray_tmin: f64, ray_tmax: f64) -> HitResult {
        let oc = self.center - r.origin;
        let a = r.dir.length_squared();
        let h = oc.dot(&r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return HitResult::NoHit;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies within the specified range
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return HitResult::NoHit;
            }
        }

        let mut record = HitRecord {
            t: root,
            point: r.at(root),
            normal: (r.at(root) - self.center) / self.radius,
            front_face: false,
        };

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(r, &outward_normal);

        HitResult::Hit(record)
    }
}
