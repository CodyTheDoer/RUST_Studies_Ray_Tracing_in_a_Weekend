use crate::Interval;

use crate::hit::{Hittable, HittableList, HitRecord};

use crate::material::default_material_lambertian;

use crate::rtvec3::{RtVec3, Point3};

use crate::random_float_range;

use std::fs::File;
use std::io::Write;
use std::ops::Mul;
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
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn random() -> Self {
        let r: f64 = random_float_range(Interval::new(0.0, 1.0));
        let g: f64 = random_float_range(Interval::new(0.0, 1.0));
        let b: f64 = random_float_range(Interval::new(0.0, 1.0));
        Color {
            r,
            g,
            b,
        }
    }
    
    pub fn random_range(interval: Interval) -> Self {
        let r: f64 = random_float_range(interval.clone());
        let g: f64 = random_float_range(interval.clone());
        let b: f64 = random_float_range(interval.clone());
        Color {
            r,
            g,
            b,
        }
    }
    
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Color {
            r,
            g,
            b,
        }
    }

    pub fn from_vec(color: RtVec3) -> Self {
        Color {
            r: color.x(),
            g: color.y(),
            b: color.z(),
        }
    }
}

impl Mul<RtVec3> for Color {
    type Output = RtVec3;

    fn mul(self, other: RtVec3) -> RtVec3 {
        RtVec3 {
            x: self.r * other.x,
            y: self.g * other.y,
            z: self.b * other.z,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
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
    sample_bounce_max: u32,
) -> RtVec3 {
    if sample_bounce_max <= 0 {
        return RtVec3::new(0.0, 0.0, 0.0);
    }
    // let reflectance = 0.5;
    let default_material = default_material_lambertian();
    let mut record: HitRecord = HitRecord::new(
        RtVec3::new(0.0, 0.0, 0.0),
        RtVec3::new(0.0, 0.0, 0.0),
        0.0,
        false,
        Rc::clone(&default_material),
    );
    if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut record) {
        if let Some((attenuation, scattered)) = record.material.clone().scatter(ray, record) {
            return attenuation * color(scattered, world, sample_bounce_max - 1);
        }
        return RtVec3::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let lerp_res = (1.0 - a) * RtVec3::new(1.0, 1.0, 1.0) + a * RtVec3::new(0.5, 0.7, 1.0);
    lerp_res
}

// corrects colors to consider gamma space alterations
pub fn linear_to_gamma(linear_component: f64) -> f64
{
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

pub fn write_color_to_pixel(
    color: RtVec3,
    file: &mut File,
) -> std::io::Result<()> {
    // Pixel Algo
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());

    let intensity: Interval = Interval::new(0.0, 0.999);
    let ir: u32 = (255.999 * intensity.clamp(r)) as u32;
    let ig: u32 = (255.999 * intensity.clamp(g)) as u32;
    let ib: u32 = (255.999 * intensity.clamp(b)) as u32;

    let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
    file.write_all(pixel_triplets.as_bytes())?;
    
    Ok(())
}