use crate::rtvec3::{RtVec3, Point3};

use std::fs::File;
use std::io::Write;
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: RtVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: RtVec3) -> Self {
        Ray {
            origin,
            direction,
        }
    }
    
    // getter functions
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> RtVec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> RtVec3 {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RayColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RayColor {
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        RayColor {
            r,
            g,
            b,
        }
    }

    pub fn from_vec(color: RtVec3) -> Self {
        RayColor {
            r: color.x(),
            g: color.y(),
            b: color.z(),
        }
    }
}

pub fn hit_sphere(
    center: Point3,
    radius: f64,
    ray: Ray,
) -> f64 {
    // Compute the vector from the ray's origin to the sphere center.
    let oc = center - ray.origin();

    // Quadratic formula coefficients.
    let a = ray.direction().length_squared();
    let h = ray.direction().dot(&oc);
    let c = oc.length_squared()- radius * radius;

    // Discriminant calculation.
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

pub fn color(
    ray: Ray,
) -> RtVec3 {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        // let color = RtVec3::new(1.0, 0.0, 0.0);
        // return color;
        let n = (ray.at(t) - RtVec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * RtVec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    } 
    let unit_direction = ray.direction();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let lerp_res = (1.0 - a) * RtVec3::new(1.0, 1.0, 1.0) + a * RtVec3::new(0.5, 0.7, 1.0);
    lerp_res
}

pub fn write_color_to_pixel(
    pixel: RtVec3,
    file: &mut File,
) -> std::io::Result<()> {
    // Pixel Algo
    let r: f64 = pixel.x();
    let g: f64 = pixel.y();
    let b: f64 = pixel.z();

    let ir: u32 = (255.999 * r) as u32;
    let ig: u32 = (255.999 * g) as u32;
    let ib: u32 = (255.999 * b) as u32;

    let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
    file.write_all(pixel_triplets.as_bytes())?;
    
    Ok(())
}

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

pub trait Hittable {
    fn hit(
        &self, 
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut HitRecord,
    ) -> bool;
}

#[derive(Clone, Copy, Debug)]
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
        record.t = root;
        record.p = ray.at(record.t);
        record.normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, record.normal);
        
        true
    }
}

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

impl Hittable for HittableList {
    fn hit(
        &self, 
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        record: &mut HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::new(Point3::new(0.0, 0.0, 0.0), RtVec3::new(0.0, 0.0, 0.0), 0.0, false);
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        hit_anything
    }
}