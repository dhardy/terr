//! Generate a Voronoi diagram as a heightmap.

use terr::heightmap::{Heightmap, Voronoi};
use nalgebra::*;
use kiss3d::{window::Window, light::Light};

fn main() {
    let mut window = Window::new("Terr: voronoi");
    window.set_light(Light::StickToCamera);
    
    let cells = 129; // must be 2.powi(n) + 1 for some integer n
    let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));
    
    // Try different weights and numbers of points!
    let w = [-0.8, 0.2, 0.4];
    // let w = [-0.9, 1.2];
    // let w = [0.7, -1.2];
    let voronoi = Voronoi::random(&heightmap, 24, &mut rand::thread_rng());
    voronoi.apply_to(&mut heightmap, &w, |x,y| (x*x + y*y).sqrt());
    
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
    
    let mut sphere = window.add_sphere(1.0);
    sphere.set_local_translation(Translation3::new(0., 5., 0.));
    sphere.set_color(0.6, 0.2, 0.8);
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 50., 50.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
