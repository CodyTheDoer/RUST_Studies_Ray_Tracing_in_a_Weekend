use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RtVec3{
    x: f32,
    y: f32,
    z: f32,
}

impl RtVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        RtVec3 {
            x,
            y,
            z,
        }
    }
    
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

// Type alias for geometric clarity, similar to using in C++
type Point3 = RtVec3;

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