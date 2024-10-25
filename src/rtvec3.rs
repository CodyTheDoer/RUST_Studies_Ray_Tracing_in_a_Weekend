use crate::Interval;
use crate::{random_float, random_float_interval};

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp;
use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RtVec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Type alias for geometric clarity, similar to using in C++
pub type Point3 = RtVec3;

impl RtVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        RtVec3 {
            x,
            y,
            z,
        }
    }
    
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
    
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn random() -> Self {
        RtVec3 {
            x: random_float(),
            y: random_float(),
            z: random_float(),
        }
    }

    pub fn random_range(interval: Interval) -> Self {
        RtVec3 {
            x: random_float_interval(interval.clone()),
            y: random_float_interval(interval.clone()),
            z: random_float_interval(interval),
        }
    }
    
    // Calculating length squared
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Calculating magnitude
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }

    // Scalar multiplication
    pub fn multiply_scalar(&self, t: f64) -> RtVec3 {
        RtVec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    // Scalar Division
    pub fn divide_scalar(&self, t: f64) -> RtVec3 {
        RtVec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> RtVec3 {
        RtVec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> RtVec3 {
        let uv = self.length();
        if uv == 0.0 {
            panic!("Cannot normalize a zero-length vector");
        }
        *self / uv
    }

    pub fn random_unit_vector() -> RtVec3 {
        loop {
            let p = RtVec3::random_range(Interval::new(-1.0, 1.0));
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &RtVec3) -> RtVec3 {
        let on_unit_sphere: RtVec3 = RtVec3::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere // In the same hemisphere as the normal
        } else {
            -on_unit_sphere // Flip the vector to ensure it's in the correct hemisphere
        }
    }

    pub fn reflect(v: RtVec3, n: RtVec3) -> RtVec3 {
        v - 2.0 * v.dot(&n) * n
    }

    pub fn refract(
        uv: RtVec3,
        n: RtVec3,
        etai_over_etat: f64,
    ) -> RtVec3 {
        let cos_theta = f64::min(-uv.dot(&n), 1.0);
        let r_out_perp: RtVec3 =  etai_over_etat * (uv + (cos_theta * n));
        let r_out_parallel: RtVec3 = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        r_out_perp + r_out_parallel
    }

    pub fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

// Operator Overloading
impl Add for RtVec3 {
    type Output = RtVec3;

    fn add(self, other: RtVec3) -> RtVec3 {
        RtVec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<RtVec3> for f64 {
    type Output = RtVec3;

    fn add(self, v: RtVec3) -> RtVec3 {
        RtVec3 {
            x: v.x + self,
            y: v.y + self,
            z: v.z + self,
        }
    }
}

impl Add<RtVec3> for u32 {
    type Output = RtVec3;

    fn add(self, v: RtVec3) -> RtVec3 {
        RtVec3 {
            x: v.x + self as f64,
            y: v.y + self as f64,
            z: v.z + self as f64,
        }
    }
}

impl Sub for RtVec3 {
    type Output = RtVec3;

    fn sub(self, other: RtVec3) -> RtVec3 {
        RtVec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<RtVec3> for f64 {
    type Output = RtVec3;

    fn sub(self, v: RtVec3) -> RtVec3 {
        RtVec3 {
            x: v.x - self,
            y: v.y - self,
            z: v.z - self,
        }
    }
}

impl Sub<RtVec3> for u32 {
    type Output = RtVec3;

    fn sub(self, v: RtVec3) -> RtVec3 {
        RtVec3 {
            x: v.x - self as f64,
            y: v.y - self as f64,
            z: v.z - self as f64,
        }
    }
}

impl Mul for RtVec3 {
    type Output = RtVec3;

    fn mul(self, other: RtVec3) -> RtVec3 {
        RtVec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for RtVec3 {
    type Output = RtVec3;

    fn mul(self, t: f64) -> RtVec3 {
        RtVec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<u32> for RtVec3 {
    type Output = RtVec3;

    fn mul(self, t: u32) -> RtVec3 {
        let t = t as f64;
        RtVec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<RtVec3> for f64 {
    type Output = RtVec3;

    fn mul(self, v: RtVec3) -> RtVec3 {
        v * self // reusing the vector * scalar logic
    }
}

impl Mul<RtVec3> for u32 {
    type Output = RtVec3;

    fn mul(self, v: RtVec3) -> RtVec3 {
        v * self as f64 // reusing the vector * scalar logic
    }
}

impl Div for RtVec3 {
    type Output = RtVec3;

    fn div(self, other: RtVec3) -> RtVec3 {
        RtVec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for RtVec3 {
    type Output = RtVec3;

    fn div(self, t: f64) -> RtVec3 {
        RtVec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Div<u32> for RtVec3 {
    type Output = RtVec3;

    fn div(self, t: u32) -> RtVec3 {
        let t = t as f64;
        RtVec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Div<RtVec3> for f64 {
    type Output = RtVec3;

    fn div(self, v: RtVec3) -> RtVec3 {
        v / self // reusing the vector * scalar logic
    }
}

impl Div<RtVec3> for u32 {
    type Output = RtVec3;

    fn div(self, v: RtVec3) -> RtVec3 {
        v / self as f64 // reusing the vector * scalar logic
    }
}

impl Neg for RtVec3 {
    type Output = RtVec3;

    fn neg(self) -> RtVec3 {
        RtVec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}