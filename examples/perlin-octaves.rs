//! Generate a flat scene, nothing more.

use terr::{heightmap::Heightmap, unbounded::Perlin};
use nalgebra::{Point3, Vector3};
use kiss3d::{window::Window, light::Light};
use rand::thread_rng;
use rand_distr::{Distribution, UnitCircle, Exp1};

fn main() {
    let mut window = Window::new("Terr: perlin octaves");
    window.set_light(Light::StickToCamera);
    
    let mut rng = thread_rng();
    
    let cells = 256;
    let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));
    let mut ampl = 20.0;
    let mut larc = 1.0 / (cells as f32);
    for _ in 0..7 {
        let sampler = || {
            let g: [f32; 2] = UnitCircle.sample(&mut rng);
            let s: f32 = Exp1.sample(&mut rng);
            [g[0] * s, g[1] * s]
        };
        let surface = Perlin::new(larc, 1024, sampler).unwrap();
        heightmap.add_surface(&surface, ampl);
        ampl *= 0.5;
        larc *= 2.0;
    }
    
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
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(50., 50., 0.), Point3::new(50., 0., 50.));
    
    while window.render_with_camera(&mut camera) {
    }
}
