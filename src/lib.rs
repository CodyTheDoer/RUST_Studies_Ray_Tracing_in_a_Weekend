pub mod ray;
pub mod rtvec3; 

pub use ray::{Ray, RayColor};
pub use rtvec3::{RtVec3, Point3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: RtVec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: RtVec3, t: f64) -> Self {
        HitRecord {
            p,
            normal,
            t,
        }
    }
}

pub trait Hittable {
    fn hit(
        &self, 
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut HitRecord,
    ) -> bool;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(
        &self, 
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut HitRecord,
    ) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        } 
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
        record.p = ray.at(record.t);
        record.normal = (record.p - self.center) / self.radius;
        record.t = root;
        
        true
    }
}