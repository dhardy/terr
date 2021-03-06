//! Generate a flat scene, nothing more.

use terr::{mesh::SampleMesh, unbounded::Flat};
use std::f32::consts;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use kiss3d::{window::Window, light::Light};

fn main() {
    let mut window = Window::new("Terr: flat");
    window.set_light(Light::StickToCamera);
    
    let surface = Flat::new(0f32);
    let mesh = surface.sample_mesh((-50., -50.), (100., 100.), (1, 1));
    
    let mut quad = window.add_trimesh(mesh, Vector3::from_element(1.0));
    quad.enable_backface_culling(false);
    quad.set_color(0.75, 0.65, 0.4);
    quad.set_local_rotation(UnitQuaternion::from_euler_angles(-consts::FRAC_PI_2, 0., 0.));
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 1., 15.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
