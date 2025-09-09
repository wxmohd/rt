use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    
    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
    
    pub fn one() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }
    
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            *self / len
        } else {
            *self
        }
    }
    
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * self.dot(normal)
    }
    
    pub fn refract(&self, normal: &Vec3, eta: f64) -> Option<Vec3> {
        let cos_i = -self.dot(normal);
        let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);
        
        if sin_t2 > 1.0 {
            None // Total internal reflection
        } else {
            let cos_t = (1.0 - sin_t2).sqrt();
            Some(*self * eta + *normal * (eta * cos_i - cos_t))
        }
    }
    
    pub fn lerp(&self, other: &Vec3, t: f64) -> Vec3 {
        *self * (1.0 - t) + *other * t
    }
    
    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    
    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
