//! Generate a flat scene, nothing more.

use terr::{heightmap::Heightmap, unbounded::Perlin};
use std::f32::consts;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use kiss3d::{window::Window, light::Light};

fn main() {
    let mut window = Window::new("Terr: perlin");
    window.set_light(Light::StickToCamera);
    
    let cells = 200;
    let mut heightmap = Heightmap::new_flat((cells, cells), (100.0, 100.0));
    let mut ampl = 50.0;
    let mut larc = 1.0 / (cells as f32);
    for _ in 0..7 {
        let surface = Perlin::new(larc, 256, &mut rand::thread_rng()).unwrap();
        heightmap.add(&surface, ampl);
        ampl *= 0.5;
        larc *= 2.0;
    }
    
    let quad = heightmap.to_trimesh();
    let mut quad = window.add_trimesh(quad, Vector3::from_element(1.0));
    quad.enable_backface_culling(false);
    quad.set_color(0.75, 0.65, 0.4);
    quad.set_local_rotation(UnitQuaternion::from_euler_angles(-consts::FRAC_PI_2, 0., 0.));
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 1., 15.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
