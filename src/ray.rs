use crate::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
    
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, outward_normal: Vec3, t: f64, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}
