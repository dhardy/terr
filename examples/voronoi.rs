//! Generate a Voronoi diagram as a heightmap.

use terr::heightmap::{Heightmap, Voronoi};
use nalgebra::*;
use kiss3d::{window::Window, light::Light};

fn main() {
    let mut window = Window::new("Terr: fractal");
    window.set_light(Light::StickToCamera);
    
    // Create a height map:
    let size = 128; // must be a power of 2
    let mut heightmap = Heightmap::new(size + 1, size + 1, 0f32);
    
    // Try different weights and numbers of points!
    let w = [-80.0, 20.0, 50.0];
    // let w = [-90.0, 120.0];
    // let w = [70.0, -120.0];
    let voronoi = Voronoi::random(24, &mut rand::thread_rng());
    voronoi.apply_to(&mut heightmap, &w);
    
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
