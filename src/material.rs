use crate::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflectivity: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Vec3,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflectivity: f64,
        transparency: f64,
        refractive_index: f64,
    ) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            reflectivity,
            transparency,
            refractive_index,
        }
    }
    
    pub fn default() -> Self {
        Material::new(
            Vec3::new(0.5, 0.5, 0.5), // gray
            0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
        )
    }
    
    pub fn reflective(color: Vec3, reflectivity: f64) -> Self {
        Material::new(
            color,
            0.1, 0.3, 0.6, 200.0, reflectivity, 0.0, 1.0
        )
    }
    
    pub fn transparent(color: Vec3, transparency: f64, refractive_index: f64) -> Self {
        Material::new(
            color,
            0.1, 0.1, 0.8, 200.0, 0.1, transparency, refractive_index
        )
    }
}
