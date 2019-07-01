//! Generate heightmap using random midpoint displacement.
//! 
//! Note that parameters are also randomised. If you don't see an interesting
//! result, run it again!

use terr::heightmap::{Heightmap, midpoint_displacement};
use nalgebra::*;
use kiss3d::{window::Window, light::Light};
use rand::prelude::*;
use rand_distr::*;

fn main() {
    let mut window = Window::new("Terr: fractal displacement (midpoint)");
    window.set_light(Light::StickToCamera);
    
    let cells = 129; // must be 2.powi(n) + 1 for some integer n
    let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));
    
    // Randomise the height of the four corners:
    let distr = LogNormal::new(0.5, 1.5).unwrap();
    let mut rng = rand::thread_rng();
    for (x, y) in [(0, 0), (0, cells-1), (cells-1, 0), (cells-1, cells-1)].iter() {
        let h = distr.sample(&mut rng) as f32;
        println!("Height[{},{}] = {}", *x, *y, h);
        heightmap.set(*x, *y, h);
    }
    
    // Perform random midpoint displacement with randomised scale.
    let scale = LogNormal::new(-2.0, 1.0).unwrap().sample(&mut rng) as f32;
    println!("Scale = {}", scale);
    // Note: can use Uniform::new(-scale, scale) instead for speed
    let distr = Normal::new(0.0, scale).unwrap();
    midpoint_displacement(&mut heightmap, 0, &mut rng, distr).unwrap();
    
    let mut quad = heightmap.to_trimesh();
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
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 50., 50.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
