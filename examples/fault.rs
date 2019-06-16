//! Displace terrain via multiple fault-lines

use terr::heightmap::{Heightmap, fault_displacement};
use nalgebra::*;
use kiss3d::{window::Window, light::Light};
use rand::prelude::*;
use rand::distributions::*;

fn main() {
    let mut window = Window::new("Terr: voronoi");
    window.set_light(Light::StickToCamera);
    
    // Create a height map:
    let size = 128; // must be a power of 2
    let mut heightmap = Heightmap::new(size + 1, size + 1, 0f32);
    
    let mut rng = rand::thread_rng();
    let r_dist = LogNormal::new(-2.5, 0.5);
    for _ in 0..50 {
        let r = rng.sample(r_dist) as f32;
        fault_displacement(&mut heightmap, &mut rng, (0.0, r), |d| {
            // Only affect points on one side of fault at distance < r:
            if d >= 0.0 && d < r {
                // A smooth function with derivative 0 at d=r.
                // We multiply by 10*r to make height relate to width.
                10.0 * r * (1.0 - (d / r).powi(2)).powi(2)
            } else {
                0.0
            }
        });
    }
    
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
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 50., 50.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
