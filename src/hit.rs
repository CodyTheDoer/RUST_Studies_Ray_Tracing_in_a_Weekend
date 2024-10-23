use crate::rtvec3::{Point3, RtVec3};
use crate::ray::Ray;
use crate::Interval;

use std::rc::Rc;

// Hit Record 

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: RtVec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: RtVec3, t: f64, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn set_face_normal (
        &mut self,
        ray: &Ray, 
        outward_normal: RtVec3,
    ) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

// Hittables & Container 

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn with_object(object: Rc<dyn Hittable>) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

// pub trait Hittable {
//     fn hit(
//         &self, 
//         ray: &Ray,
//         t_min: f64,
//         t_max: f64,
//         record: &mut HitRecord,
//     ) -> bool;
// }

// impl Hittable for HittableList {
//     fn hit(
//         &self, 
//         ray: &Ray,
//         t_min: f64,
//         t_max: f64,
//         record: &mut HitRecord,
//     ) -> bool {
//         let mut temp_record = HitRecord::new(Point3::new(0.0, 0.0, 0.0), RtVec3::new(0.0, 0.0, 0.0), 0.0, false);
//         let mut hit_anything: bool = false;
//         let mut closest_so_far = t_max;

//         for object in &self.objects {
//             if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
//                 hit_anything = true;
//                 closest_so_far = temp_record.t;
//                 *record = temp_record.clone();
//             }
//         }

//         hit_anything
//     }
// }

pub trait Hittable {
    fn hit(
        &self, 
        ray: &Ray,
        interval: Interval,
        record: &mut HitRecord,
    ) -> bool;
}

impl Hittable for HittableList {
    fn hit(
        &self, 
        ray: &Ray,
        interval: Interval,
        record: &mut HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::new(Point3::new(0.0, 0.0, 0.0), RtVec3::new(0.0, 0.0, 0.0), 0.0, false);
        let mut hit_anything: bool = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(interval.min, closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}

// Geometry: Sphere

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

// impl Hittable for Sphere {
//     fn hit(
//         &self, 
//         ray: &Ray,
//         t_min: f64,
//         t_max: f64,
//         record: &mut HitRecord,
//     ) -> bool {
//         let oc = ray.origin() - self.center;
//         let a = ray.direction().length_squared();
//         let half_b = ray.direction().dot(&oc);
//         let c = oc.length_squared() - self.radius * self.radius;

//         let discriminant = half_b * half_b - a * c;
//         if discriminant < 0.0 {
//             return false;
//         } 
//         let sqrtd = discriminant.sqrt();

//         // Find the nearest root that lies in the acceptable range.
//         let mut root = (-half_b - sqrtd) / a;
//         if root <= t_min || t_max <= root {
//             root = (-half_b + sqrtd) / a;
//             if root <= t_min || t_max <= root {
//                 return false;
//             }
//         }

//         record.t = root;
//         record.p = ray.at(record.t);
//         record.normal = (record.p - self.center) / self.radius;
//         record.set_face_normal(ray, record.normal);
        
//         true
//     }
// }

impl Hittable for Sphere {
    fn hit(
        &self, 
        ray: &Ray,
        interval: Interval,
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
        if !interval.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        record.normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, record.normal);
        
        true
    }
}
