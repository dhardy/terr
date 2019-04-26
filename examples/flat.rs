//! Generate a flat scene, nothing more.

use std::f32::consts;
use nalgebra::{Point3, UnitQuaternion, Translation3};
use kiss3d::{window::Window, light::Light};

fn main() {
    let mut window = Window::new("Terr: flat");
    window.set_light(Light::StickToCamera);
    
    let mut quad = window.add_quad(100., 100., 1, 1);
    quad.set_color(0.75, 0.65, 0.4);
    quad.set_local_rotation(UnitQuaternion::from_euler_angles(consts::FRAC_PI_2, 0., 0.));
    
    let mut sphere = window.add_sphere(1.0);
    sphere.set_local_translation(Translation3::new(0., 1., 0.));
    sphere.set_color(0.6, 0.2, 0.8);
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 1., 15.), Point3::new(0., 0., 0.));
    
    while window.render_with_camera(&mut camera) {
    }
}
