pub mod camera;
pub mod hit; 
pub mod ray;
pub mod rtvec3; 

use std::fs::File;
use std::io::Write;

pub use hit::{Hittable, HittableList, HitRecord};
pub use ray::{Ray, RayColor};
pub use ray::{write_color_to_pixel, color};
pub use rtvec3::{Point3, RtVec3};

// Utility functions 
pub fn degrees_to_radians(
    degrees: f64,
) -> f64 {
    const PI: f64 = 3.14159265358979323846264338327950288_f64;
    let res = degrees * PI / 180.0;
    res
}

#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Custom New interval
    pub fn new(min: f64, max: f64) -> Interval {
        Interval {min, max}
    }

    pub fn new_empty(min: f64, max: f64) -> Interval {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn new_universe(min: f64, max: f64) -> Interval {
        Interval {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}