use crate::rtvec3::{RtVec3, Point3};
use crate::hit::{Hittable, HittableList, HitRecord};
use crate::Interval;

use std::fs::File;
use std::io::Write;

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
    world: &HittableList,
    sample_ray_bounce_max: u32,
) -> RtVec3 {
    if sample_ray_bounce_max <= 0 {
        return RtVec3::new(0.0, 0.0, 0.0)
    }

    let mut record: HitRecord = HitRecord::new(
        RtVec3::new(0.0, 0.0, 0.0),
        RtVec3::new(0.0, 0.0, 0.0),
        0.0,
        false,
    );
    if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut record) {
        let direction: RtVec3 = *&record.normal + RtVec3::random_unit_vector();
        return 0.5 * color(Ray::new(record.p, direction), world, sample_ray_bounce_max - 1);
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let lerp_res = (1.0 - a) * RtVec3::new(1.0, 1.0, 1.0) + a * RtVec3::new(0.5, 0.7, 1.0);
    lerp_res
}

pub fn write_color_to_pixel(
    color: RtVec3,
    file: &mut File,
) -> std::io::Result<()> {
    // Pixel Algo
    let r: f64 = color.x();
    let g: f64 = color.y();
    let b: f64 = color.z();

    let intensity: Interval = Interval::new(0.0, 0.999);
    let ir: u32 = (255.999 * intensity.clamp(r)) as u32;
    let ig: u32 = (255.999 * intensity.clamp(g)) as u32;
    let ib: u32 = (255.999 * intensity.clamp(b)) as u32;

    let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
    file.write_all(pixel_triplets.as_bytes())?;
    
    Ok(())
}