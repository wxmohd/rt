# Ray Tracer (RT)

A high-performance ray tracer implemented in Rust that renders 3D scenes into 2D PPM images. This ray tracer supports multiple geometric objects, realistic lighting with shadows, and optional reflection effects.

## Features

- **Geometric Objects**: Sphere, Cube, Plane, and Cylinder
- **Realistic Lighting**: Ambient, diffuse, and specular lighting with shadow casting
- **Camera System**: Configurable camera position, orientation, and field of view
- **Material System**: Customizable materials with color, reflectivity, and transparency
- **Parallel Rendering**: Multi-threaded rendering for improved performance
- **PPM Output**: Standard PPM image format output
- **Command Line Interface**: Easy-to-use CLI with various options

## Installation

1. Ensure you have Rust installed on your system
2. Clone or download this project
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Usage

Generate a scene and output to PPM format:

```bash
cargo run --release -- --scene scene1 --width 800 --height 600 > output.ppm
```

### Command Line Options

- `--width, -w`: Image width in pixels (default: 800)
- `--height, -h`: Image height in pixels (default: 600)
- `--scene, -s`: Scene to render (scene1, scene2, scene3, scene4)
- `--reflection, -r`: Enable reflection effects
- `--textures, -t`: Enable texture rendering (bonus feature)

### Examples

```bash
# Render a sphere scene
cargo run --release -- --scene scene1 --width 800 --height 600 > sphere.ppm

# Render with reflections enabled
cargo run --release -- --scene scene3 --reflection --width 800 --height 600 > reflective_scene.ppm

# Render at lower resolution for faster testing
cargo run --release -- --scene scene2 --width 400 --height 300 > test.ppm
```

## Scene Descriptions

### Scene 1: Single Sphere
- Contains a red sphere with standard lighting
- Good for testing basic ray-sphere intersection

### Scene 2: Plane and Cube (Lower Brightness)
- Contains a gray plane and green cube
- Uses dimmer lighting compared to Scene 1
- Demonstrates plane and cube rendering

### Scene 3: All Objects
- Contains all four object types: sphere, cube, cylinder, and plane
- Shows the full capabilities of the ray tracer
- Objects are positioned to avoid overlap

### Scene 4: Different Perspective
- Same objects as Scene 3 but from a different camera angle
- Demonstrates camera positioning and orientation

## Creating Custom Scenes

To create your own scenes, modify the scene creation functions in `src/main.rs`:

### Creating Objects

#### Sphere
```rust
let material = Material::new(
    Vec3::new(0.8, 0.2, 0.2), // RGB color (red)
    0.1, 0.7, 0.2, 200.0,     // ambient, diffuse, specular, shininess
    0.0, 0.0, 1.0             // reflectivity, transparency, refractive_index
);

scene.add_object(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -2.0), // center position
    1.0,                       // radius
    material,
)));
```

#### Cube
```rust
scene.add_object(Box::new(Cube::new(
    Vec3::new(2.0, 0.0, -2.0), // center position
    1.0,                       // size
    material,
)));
```

#### Plane
```rust
scene.add_object(Box::new(Plane::new(
    Vec3::new(0.0, -2.0, 0.0), // point on plane
    Vec3::new(0.0, 1.0, 0.0),  // normal vector
    material,
)));
```

#### Cylinder
```rust
scene.add_object(Box::new(Cylinder::new(
    Vec3::new(0.0, 0.0, -2.0), // center position
    0.5,                       // radius
    2.0,                       // height
    material,
)));
```

### Adjusting Camera Position

```rust
let camera = Camera::new(
    Vec3::new(3.0, 2.0, 5.0),  // camera position
    Vec3::new(0.0, 0.0, 0.0),  // look at point
    Vec3::new(0.0, 1.0, 0.0),  // up vector
    45.0,                      // field of view (degrees)
    width as f64 / height as f64, // aspect ratio
);
scene.set_camera(camera);
```

### Modifying Lighting

```rust
// Add a light source
scene.add_light(Light::new(
    Vec3::new(5.0, 5.0, 5.0),  // position
    Vec3::new(1.0, 1.0, 1.0),  // color (white)
    1.0,                       // intensity
));

// For dimmer lighting
scene.add_light(Light::new(
    Vec3::new(5.0, 5.0, 5.0),
    Vec3::new(0.5, 0.5, 0.5),  // dimmer color
    0.5,                       // lower intensity
));
```

### Changing Brightness

Brightness can be controlled through:
1. **Light intensity**: Lower the intensity value (0.0 to 1.0)
2. **Light color**: Use darker RGB values
3. **Material properties**: Adjust ambient, diffuse, and specular values

Example for lower brightness:
```rust
// Dim light
scene.add_light(Light::new(
    Vec3::new(5.0, 5.0, 5.0),
    Vec3::new(0.3, 0.3, 0.3),  // Very dim
    0.3,
));

// Or adjust material
let dim_material = Material::new(
    Vec3::new(0.5, 0.5, 0.5),
    0.05, 0.4, 0.1, 200.0,     // Lower ambient and diffuse
    0.0, 0.0, 1.0
);
```

## Material Properties

Materials control how objects appear:

- **Color**: RGB values (0.0 to 1.0)
- **Ambient**: Base lighting level (typically 0.1)
- **Diffuse**: How much the surface scatters light (0.0 to 1.0)
- **Specular**: Shininess/highlight intensity (0.0 to 1.0)
- **Shininess**: Tightness of specular highlights (higher = tighter)
- **Reflectivity**: Mirror-like reflection (0.0 to 1.0)
- **Transparency**: See-through effect (0.0 to 1.0)
- **Refractive Index**: Light bending (1.0 = no bending, 1.5 = glass)

## Performance Tips

1. **Use lower resolutions** for testing (e.g., 200x150)
2. **Disable reflections** for faster rendering
3. **Limit scene complexity** - fewer objects render faster
4. **Use release builds** (`--release` flag) for optimal performance

## Output Format

The ray tracer outputs PPM (Portable Pixmap) format images. PPM files can be:
- Viewed with image viewers that support PPM
- Converted to other formats using tools like ImageMagick:
  ```bash
  convert output.ppm output.png
  ```

## Troubleshooting

### Common Issues

1. **Black/background-only images**: Check object positions relative to camera
2. **Slow rendering**: Use smaller image dimensions or disable reflections
3. **Compilation errors**: Ensure Rust is up to date

### Debugging Tips

- Start with simple scenes (single sphere)
- Use smaller image dimensions for testing
- Check that objects are positioned in front of the camera
- Verify lighting is present and positioned correctly

## Technical Details

### Architecture

- **Vector Math**: Custom 3D vector implementation with standard operations
- **Ray Casting**: Rays are cast from camera through each pixel
- **Object Intersection**: Each object type implements ray intersection algorithms
- **Lighting Model**: Phong lighting with ambient, diffuse, and specular components
- **Parallel Processing**: Uses Rayon for multi-threaded pixel rendering

### Coordinate System

- **X-axis**: Right
- **Y-axis**: Up  
- **Z-axis**: Into the screen (negative Z is away from camera)

