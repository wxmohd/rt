use crate::vector::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub fov: f64,
    pub aspect_ratio: f64,
    
    // Computed values
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, fov: f64, aspect_ratio: f64) -> Self {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        
        let w = (position - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        
        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;
        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - w;
        
        Camera {
            position,
            look_at,
            up,
            fov,
            aspect_ratio,
            u,
            v,
            w,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let target = self.lower_left_corner + self.horizontal * s + self.vertical * t;
        let direction = target - self.position;
        Ray::new(self.position, direction)
    }
}
