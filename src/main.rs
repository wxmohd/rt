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
    
    #[arg(short, long, default_value = "600")]
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
    
    // Set up camera - simpler setup
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),  // position at origin
        Vec3::new(0.0, 0.0, -1.0), // look down negative z
        Vec3::new(0.0, 1.0, 0.0),  // up
        90.0,                      // wider fov
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
    let material = Material::new(
        Vec3::new(0.8, 0.2, 0.2), // red color
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -2.0), // Place sphere further away
        1.0,
        material,
    )));
}

fn create_plane_cube_scene(scene: &mut Scene) {
    // Lower brightness lighting
    scene.lights.clear();
    scene.add_light(Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(0.5, 0.5, 0.5), // dimmer light
        0.5,
    ));
    
    let plane_material = Material::new(
        Vec3::new(0.5, 0.5, 0.5), // gray
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    let cube_material = Material::new(
        Vec3::new(0.2, 0.8, 0.2), // green
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    scene.add_object(Box::new(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        plane_material,
    )));
    
    scene.add_object(Box::new(Cube::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        cube_material,
    )));
}

fn create_all_objects_scene(scene: &mut Scene) {
    let sphere_material = Material::new(
        Vec3::new(0.8, 0.2, 0.2), // red
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    let cube_material = Material::new(
        Vec3::new(0.2, 0.8, 0.2), // green
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    let cylinder_material = Material::new(
        Vec3::new(0.2, 0.2, 0.8), // blue
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    let plane_material = Material::new(
        Vec3::new(0.5, 0.5, 0.5), // gray
        0.1, 0.7, 0.2, 200.0, 0.0, 0.0, 1.0
    );
    
    scene.add_object(Box::new(Sphere::new(
        Vec3::new(-2.0, 0.0, 0.0),
        1.0,
        sphere_material,
    )));
    
    scene.add_object(Box::new(Cube::new(
        Vec3::new(2.0, 0.0, 0.0),
        1.0,
        cube_material,
    )));
    
    scene.add_object(Box::new(Cylinder::new(
        Vec3::new(0.0, 0.0, -2.0),
        0.5,
        2.0,
        cylinder_material,
    )));
    
    scene.add_object(Box::new(Plane::new(
        Vec3::new(0.0, -2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        plane_material,
    )));
}

fn create_different_perspective_scene(scene: &mut Scene) {
    // Change camera position
    let camera = Camera::new(
        Vec3::new(3.0, 2.0, 5.0),  // different position
        Vec3::new(0.0, 0.0, 0.0),  // still looking at origin
        Vec3::new(0.0, 1.0, 0.0),  // up
        45.0,                       // fov
        800.0 / 600.0,             // aspect ratio
    );
    scene.set_camera(camera);
    
    // Same objects as scene3
    create_all_objects_scene(scene);
}
