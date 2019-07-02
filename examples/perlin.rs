//! Generate a flat scene, nothing more.

use terr::{mesh::SampleMesh, unbounded::Perlin};
use std::f32::consts;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use kiss3d::{window::Window, light::Light};
use rand::thread_rng;
use rand_distr::{Distribution, UnitCircle};

fn main() {
    let mut window = Window::new("Terr: perlin");
    window.set_light(Light::StickToCamera);
    
    let mut rng = thread_rng();
    let sampler = || UnitCircle.sample(&mut rng);
    
    let surface = Perlin::new(0.08615, 256, sampler).unwrap();
    let mesh = surface.sample_mesh((-50., -50.), (100., 100.), (128, 128));
    
    let mut quad = window.add_trimesh(mesh, Vector3::from_element(1.0));
    quad.enable_backface_culling(false);
    quad.set_color(0.75, 0.65, 0.4);
    quad.set_local_rotation(UnitQuaternion::from_euler_angles(-consts::FRAC_PI_2, 0., 0.));
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 1., 15.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
