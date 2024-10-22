use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RtVec3{
    x: f32,
    y: f32,
    z: f32,
}

// Type alias for geometric clarity, similar to using in C++
type Point3 = RtVec3;

impl RtVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        RtVec3 {
            x,
            y,
            z,
        }
    }

    // pub fn neg(&self) -> RtVec3 {
    //     RtVec3 {
    //         x: -self.x,
    //         y: -self.y,
    //         z: -self.z,
    //     }
    // }
    
    pub fn x(&self) -> f32 {
        self.x
    }
    
    pub fn y(&self) -> f32 {
        self.y
    }
    
    pub fn z(&self) -> f32 {
        self.z
    }
    
    // Calculating length squared
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Calculating magnitude
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    // Scalar multiplication
    pub fn multiply_scalar(&self, t: f32) -> RtVec3 {
        RtVec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    // Scalar Division
    pub fn divide_scalar(&self, t: f32) -> RtVec3 {
        RtVec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
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

impl Mul<f32> for RtVec3 {
    type Output = RtVec3;

    fn mul(self, t: f32) -> RtVec3 {
        RtVec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
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

impl Div<f32> for RtVec3 {
    type Output = RtVec3;

    fn div(self, t: f32) -> RtVec3 {
        RtVec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
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