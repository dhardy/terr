//! Generate heightmap using random diamond-square displacement.
//! 
//! Note that parameters are also randomised. If you don't see an interesting
//! result, run it again!

use terr::heightmap::{Heightmap, diamond_square};
use nalgebra::*;
use kiss3d::{window::Window, light::Light};
use rand::prelude::*;
use rand::distributions::*;

fn main() {
    let mut window = Window::new("Terr: fractal");
    window.set_light(Light::StickToCamera);
    
    // Create a height map:
    let size = 128; // must be a power of 2
    let mut heightmap = Heightmap::new(size + 1, size + 1, 0f32);
    
    // Randomise the height of the four corners:
    let distr = LogNormal::new(0.5, 1.5);
    let mut rng = rand::thread_rng();
    for (x, y) in [(0, 0), (0, size), (size, 0), (size, size)].iter() {
        let h = distr.sample(&mut rng) as f32;
        println!("Height[{},{}] = {}", *x, *y, h);
        heightmap.set(*x, *y, h);
    }
    
    // Perform random midpoint displacement with randomised scale.
    let scale = LogNormal::new(-2.0, 1.0).sample(&mut rng) as f32;
    println!("Scale = {}", scale);
    // Note: Normal(0, scale) is possibly better, but not yet available for f32.
    let distr = Uniform::new(-scale, scale);
    diamond_square(&mut heightmap, 0, &mut rng, distr).unwrap();
    
    let mut quad = heightmap.to_trimesh(100., 100.);
    for p in &mut quad.coords {
        // Quad is created with z=height, but y is up in kiss3d's camera.
        // We must rotate all three coords to keep the right side up.
        let temp = p.z;
        p.z = p.x;
        p.x = p.y;
        p.y = temp;
    }
    quad.recompute_normals();
    
    let mut quad = window.add_trimesh(quad, Vector3::from_element(1.0));
    quad.enable_backface_culling(false);
    quad.set_color(0.75, 0.65, 0.4);
    
    let mut sphere = window.add_sphere(1.0);
    sphere.set_local_translation(Translation3::new(0., 5., 0.));
    sphere.set_color(0.6, 0.2, 0.8);
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 50., 50.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
