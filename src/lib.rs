pub mod camera;
pub mod hit;
pub mod material; 
pub mod ray;
pub mod rtvec3;

// use hit::{Hittable, HittableList, HitRecord};
// use ray::{Ray, Color};
// use ray::{write_color_to_pixel, color};
use rtvec3::{Point3, RtVec3};

use rand::prelude::*;

// Utility functions 
pub fn degrees_to_radians(
    degrees: f64,
) -> f64 {
    const PI: f64 = 3.14159265358979323846264338327950288_f64;
    let res = degrees * PI / 180.0;
    res
}

pub fn random_float() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_float_range(interval: Interval) -> f64 {
    if interval.min.is_infinite() || interval.max.is_infinite() {
        panic!("Cannot generate a random value for infinite intervals");
    }
    if interval.min >= interval.max {
        panic!("Invalid interval: min must be less than max");
    }
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(interval.min..interval.max);
    y
}

pub fn sample_square() -> RtVec3 {
    RtVec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
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

    pub fn new_empty() -> Interval {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn new_universe() -> Interval {
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

    pub fn clamp(&self, x:f64) -> f64 {
        if x < self.min {return self.min};
        if x > self.max {return self.max};
        return x;
    }
}