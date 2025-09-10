use clap::Parser;

mod vector;
mod ray;
mod objects;
mod camera;
mod scene;
mod material;
mod light;
mod image;

use vector::Vec3;
use camera::Camera;
use scene::Scene;
use objects::{Sphere, Plane, Cube, Cylinder};
use material::Material;
use light::Light;
use image::Image;

#[derive(Parser)]
#[command(name = "rt")]
#[command(about = "A ray tracer that renders 3D scenes to PPM images")]
struct Args {
    #[arg(short, long, default_value = "800")]
    width: u32,
    
    #[arg(long, default_value = "600")]
    height: u32,
    
    #[arg(short, long, default_value = "scene1")]
    scene: String,
    
    #[arg(short = 'r', long)]
    reflection: bool,
    
    #[arg(short = 't', long)]
    textures: bool,
}

fn main() {
    let args = Args::parse();
    
    let mut scene = Scene::new();
    
    // Set up camera for proper perspective
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),  // camera at origin
        Vec3::new(0.0, 0.0, -1.0), // looking down negative z
        Vec3::new(0.0, 1.0, 0.0),  // up vector
        45.0,                      // field of view
        args.width as f64 / args.height as f64, // aspect ratio
    );
    scene.set_camera(camera);
    
    // Add lighting
    scene.add_light(Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
    ));
    
    // Create scenes based on argument
    match args.scene.as_str() {
        "scene1" => create_sphere_scene(&mut scene),
        "scene2" => create_plane_cube_scene(&mut scene),
        "scene3" => create_all_objects_scene(&mut scene),
        "scene4" => create_different_perspective_scene(&mut scene),
        _ => create_sphere_scene(&mut scene),
    }
    
    // Render the scene
    let mut image = Image::new(args.width, args.height);
    scene.render(&mut image, args.reflection);
    
    // Output PPM format
    image.output_ppm();
}

fn create_sphere_scene(scene: &mut Scene) {
    // Simple red sphere - test basic rendering
    let sphere_material = Material::new(
        Vec3::new(0.8, 0.2, 0.2), // bright red
        0.2, 0.8, 0.3, 100.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -5.0), // Place sphere in front of camera
        2.0,                       // Large sphere
        sphere_material,
    )));
}

fn create_plane_cube_scene(scene: &mut Scene) {
    // Lower brightness lighting as required
    scene.lights.clear();
    scene.add_light(Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(0.3, 0.3, 0.3), // dimmer light than sphere scene
        0.3,
    ));
    
    let plane_material = Material::new(
        Vec3::new(0.9, 0.8, 0.95), // light lavender
        0.3, 0.8, 0.3, 200.0, 0.0, 0.0, 1.0
    );
    
    let cube_material = Material::new(
        Vec3::new(0.7, 0.9, 1.0), // light blue
        0.3, 0.8, 0.4, 200.0, 0.0, 0.0, 1.0
    );
    
    scene.add_object(Box::new(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        plane_material,
    )));
    
    scene.add_object(Box::new(Cube::new(
        Vec3::new(0.0, 0.0, -5.0),
        1.0,
        cube_material,
    )));
}

fn create_all_objects_scene(scene: &mut Scene) {
    // Ground plane
    let plane_material = Material::new(
        Vec3::new(0.5, 0.5, 0.5), // gray
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        plane_material,
    )));
    
    // Sphere
    let sphere_material = Material::new(
        Vec3::new(0.8, 0.2, 0.2), // red
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-2.0, 0.0, -5.0),
        1.0,
        sphere_material,
    )));
    
    // Cube
    let cube_material = Material::new(
        Vec3::new(0.2, 0.8, 0.2), // green
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Cube::new(
        Vec3::new(2.0, 0.0, -5.0),
        1.0,
        cube_material,
    )));
    
    // Cylinder
    let cylinder_material = Material::new(
        Vec3::new(0.2, 0.2, 0.8), // blue
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Cylinder::new(
        Vec3::new(0.0, 0.0, -7.0),
        0.5,
        2.0,
        cylinder_material,
    )));
}

fn create_different_perspective_scene(scene: &mut Scene) {
    // Change camera position for different perspective
    let camera = Camera::new(
        Vec3::new(-3.0, 3.0, 2.0),  // different elevated position
        Vec3::new(0.0, 0.0, -5.0),  // looking at the objects
        Vec3::new(0.0, 1.0, 0.0),   // up
        45.0,                       // fov
        800.0 / 600.0,              // aspect ratio
    );
    scene.set_camera(camera);
    
    // Same objects as scene3 but from different angle
    // Ground plane
    let plane_material = Material::new(
        Vec3::new(0.5, 0.5, 0.5), // gray
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        plane_material,
    )));
    
    // Sphere
    let sphere_material = Material::new(
        Vec3::new(0.8, 0.2, 0.2), // red
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-2.0, 0.0, -5.0),
        1.0,
        sphere_material,
    )));
    
    // Cube
    let cube_material = Material::new(
        Vec3::new(0.2, 0.8, 0.2), // green
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Cube::new(
        Vec3::new(2.0, 0.0, -5.0),
        1.0,
        cube_material,
    )));
    
    // Cylinder
    let cylinder_material = Material::new(
        Vec3::new(0.2, 0.2, 0.8), // blue
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    scene.add_object(Box::new(Cylinder::new(
        Vec3::new(0.0, 0.0, -7.0),
        0.5,
        2.0,
        cylinder_material,
    )));
}
