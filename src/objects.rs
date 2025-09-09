use crate::vector::Vec3;
use crate::ray::{Ray, HitRecord};
use crate::material::Material;

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn material(&self) -> &Material;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        
        Some(HitRecord::new(point, outward_normal, root, ray))
    }
    
    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl Object for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-8 {
            return None; // Ray is parallel to plane
        }
        
        let t = (self.point - ray.origin).dot(&self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }
        
        let point = ray.at(t);
        Some(HitRecord::new(point, self.normal, t, ray))
    }
    
    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Cube {
    pub center: Vec3,
    pub size: f64,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f64, material: Material) -> Self {
        Cube { center, size, material }
    }
}

impl Object for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let half_size = self.size / 2.0;
        let min = self.center - Vec3::new(half_size, half_size, half_size);
        let max = self.center + Vec3::new(half_size, half_size, half_size);
        
        let inv_dir = Vec3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        
        let t1 = (min.x - ray.origin.x) * inv_dir.x;
        let t2 = (max.x - ray.origin.x) * inv_dir.x;
        let t3 = (min.y - ray.origin.y) * inv_dir.y;
        let t4 = (max.y - ray.origin.y) * inv_dir.y;
        let t5 = (min.z - ray.origin.z) * inv_dir.z;
        let t6 = (max.z - ray.origin.z) * inv_dir.z;
        
        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
        
        if tmax < 0.0 || tmin > tmax {
            return None;
        }
        
        let t = if tmin < t_min { tmax } else { tmin };
        if t < t_min || t > t_max {
            return None;
        }
        
        let point = ray.at(t);
        let center_to_point = point - self.center;
        
        // Determine which face was hit
        let abs_x = center_to_point.x.abs();
        let abs_y = center_to_point.y.abs();
        let abs_z = center_to_point.z.abs();
        
        let normal = if abs_x > abs_y && abs_x > abs_z {
            Vec3::new(center_to_point.x.signum(), 0.0, 0.0)
        } else if abs_y > abs_z {
            Vec3::new(0.0, center_to_point.y.signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, center_to_point.z.signum())
        };
        
        Some(HitRecord::new(point, normal, t, ray))
    }
    
    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Cylinder { center, radius, height, material }
    }
}

impl Object for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        
        // Check intersection with infinite cylinder (ignoring y)
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let b = 2.0 * (oc.x * ray.direction.x + oc.z * ray.direction.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        
        let half_height = self.height / 2.0;
        
        // Check both intersection points
        for &t in &[t1, t2] {
            if t >= t_min && t <= t_max {
                let point = ray.at(t);
                let y = point.y - self.center.y;
                
                // Check if intersection is within cylinder height
                if y >= -half_height && y <= half_height {
                    let normal = Vec3::new(
                        (point.x - self.center.x) / self.radius,
                        0.0,
                        (point.z - self.center.z) / self.radius,
                    );
                    return Some(HitRecord::new(point, normal, t, ray));
                }
            }
        }
        
        // Check intersection with top and bottom caps
        let y_min = self.center.y - half_height;
        let y_max = self.center.y + half_height;
        
        if ray.direction.y.abs() > 1e-8 {
            // Bottom cap
            let t = (y_min - ray.origin.y) / ray.direction.y;
            if t >= t_min && t <= t_max {
                let point = ray.at(t);
                let dx = point.x - self.center.x;
                let dz = point.z - self.center.z;
                if dx * dx + dz * dz <= self.radius * self.radius {
                    let normal = Vec3::new(0.0, -1.0, 0.0);
                    return Some(HitRecord::new(point, normal, t, ray));
                }
            }
            
            // Top cap
            let t = (y_max - ray.origin.y) / ray.direction.y;
            if t >= t_min && t <= t_max {
                let point = ray.at(t);
                let dx = point.x - self.center.x;
                let dz = point.z - self.center.z;
                if dx * dx + dz * dz <= self.radius * self.radius {
                    let normal = Vec3::new(0.0, 1.0, 0.0);
                    return Some(HitRecord::new(point, normal, t, ray));
                }
            }
        }
        
        None
    }
    
    fn material(&self) -> &Material {
        &self.material
    }
}
