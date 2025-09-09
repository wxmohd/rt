use crate::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: Vec3, color: Vec3, intensity: f64) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }
    
    pub fn direction_from(&self, point: Vec3) -> Vec3 {
        (self.position - point).normalize()
    }
    
    pub fn distance_from(&self, point: Vec3) -> f64 {
        (self.position - point).length()
    }
    
    pub fn attenuation(&self, distance: f64) -> f64 {
        1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance)
    }
}
