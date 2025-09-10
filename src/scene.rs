use crate::vector::Vec3;
use crate::ray::{Ray, HitRecord};
use crate::objects::Object;
use crate::camera::Camera;
use crate::light::Light;
use crate::image::Image;
use rayon::prelude::*;

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub camera: Option<Camera>,
    pub background_color: Vec3,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            camera: None,
            background_color: Vec3::new(0.7, 0.8, 1.0), // Light sky blue
        }
    }
    
    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
    
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Some(camera);
    }
    
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &dyn Object)> {
        let mut closest_hit: Option<(HitRecord, &dyn Object)> = None;
        let mut closest_t = t_max;
        
        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_t) {
                closest_t = hit_record.t;
                closest_hit = Some((hit_record, object.as_ref()));
            }
        }
        
        closest_hit
    }
    
    pub fn render(&self, image: &mut Image, enable_reflection: bool) {
        let camera = self.camera.as_ref().expect("Camera not set");
        let width = image.width;
        let height = image.height;
        
        let pixels: Vec<Vec3> = (0..height).into_par_iter().enumerate().flat_map(|(row_idx, j)| {
            if row_idx % 10 == 0 {
                eprintln!("\rScanlines remaining: {}", height as usize - row_idx - 1);
            }
            (0..width).into_par_iter().map(move |i| {
                let u = i as f64 / (width - 1) as f64;
                let v = (height - 1 - j) as f64 / (height - 1) as f64;
                
                let ray = camera.get_ray(u, v);
                self.ray_color(&ray, 5, enable_reflection) // Max depth of 5
            })
        }).collect();
        
        for (i, pixel) in pixels.into_iter().enumerate() {
            let x = i % width as usize;
            let y = i / width as usize;
            image.set_pixel(x, y, pixel);
        }
        
        eprintln!("\nDone.");
    }
    
    fn ray_color(&self, ray: &Ray, depth: i32, enable_reflection: bool) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        
        if let Some((hit_record, object)) = self.hit(ray, 0.001, f64::INFINITY) {
            let material = object.material();
            let mut color = Vec3::zero();
            
            // Ambient lighting
            color = color + material.color * material.ambient;
            
            // Direct lighting from all light sources
            for light in &self.lights {
                let light_dir = light.direction_from(hit_record.point);
                let light_distance = light.distance_from(hit_record.point);
                
                // Check for shadows
                let shadow_ray = Ray::new(hit_record.point + hit_record.normal * 0.001, light_dir);
                let in_shadow = self.hit(&shadow_ray, 0.001, light_distance).is_some();
                
                if !in_shadow {
                    // Diffuse lighting
                    let diffuse_strength = hit_record.normal.dot(&light_dir).max(0.0);
                    let diffuse = material.color * light.color * material.diffuse * diffuse_strength * light.intensity;
                    
                    // Specular lighting
                    let view_dir = (-ray.direction).normalize();
                    let reflect_dir = (-light_dir).reflect(&hit_record.normal);
                    let spec_strength = view_dir.dot(&reflect_dir).max(0.0).powf(material.shininess);
                    let specular = light.color * material.specular * spec_strength * light.intensity;
                    
                    // Apply attenuation
                    let attenuation = light.attenuation(light_distance);
                    color = color + (diffuse + specular) * attenuation;
                }
            }
            
            // Reflection
            if enable_reflection && material.reflectivity > 0.0 {
                let reflected_dir = ray.direction.reflect(&hit_record.normal);
                let reflected_ray = Ray::new(hit_record.point + hit_record.normal * 0.001, reflected_dir);
                let reflected_color = self.ray_color(&reflected_ray, depth - 1, enable_reflection);
                color = color * (1.0 - material.reflectivity) + reflected_color * material.reflectivity;
            }
            
            // Refraction (transparency)
            if material.transparency > 0.0 {
                let refraction_ratio = if hit_record.front_face {
                    1.0 / material.refractive_index
                } else {
                    material.refractive_index
                };
                
                if let Some(refracted_dir) = ray.direction.refract(&hit_record.normal, refraction_ratio) {
                    let refracted_ray = Ray::new(hit_record.point - hit_record.normal * 0.001, refracted_dir);
                    let refracted_color = self.ray_color(&refracted_ray, depth - 1, enable_reflection);
                    color = color * (1.0 - material.transparency) + refracted_color * material.transparency;
                }
            }
            
            color.clamp(0.0, 1.0)
        } else {
            self.background_color
        }
    }
}
