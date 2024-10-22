use crate::rtvec3::{RtVec3, Point3};

use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: RtVec3,
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

    pub fn at(&self, t: f32) -> RtVec3 {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RayColor {
    r: f32,
    g: f32,
    b: f32,
}

impl RayColor {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
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

pub fn color(
    ray: Ray,
) -> RtVec3 {
    let unit_direction = ray.direction();
    let a = 0.5 * (unit_direction.y() + 1.0);
    let res = (1.0 - a) * RtVec3::new(1.0, 1.0, 1.0) + a * RtVec3::new(0.5, 0.7, 1.0);
    res
}

pub fn write_color_to_pixel(
    pixel: RtVec3,
    file: &mut File,
) -> std::io::Result<()> {
    // Pixel Algo
    let r: f32 = pixel.x();
    let g: f32 = pixel.y();
    let b: f32 = pixel.z();

    let ir: u32 = (255.999 * r) as u32;
    let ig: u32 = (255.999 * g) as u32;
    let ib: u32 = (255.999 * b) as u32;

    let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
    file.write_all(pixel_triplets.as_bytes())?;
    
    Ok(())
}
