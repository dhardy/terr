//! Generate heightmap purely from uncorrelated random noise.
//! This cannot generate any features but may be useful to add a little
//! variation on top of other data.

use terr::height::{Height, Plain};
use nalgebra::{Point3, Vector2, Vector3, Translation3};
use ncollide3d::procedural;
use kiss3d::{window::Window, light::Light};
use rand::distributions::Normal;

fn main() {
    let mut window = Window::new("Terr: noise");
    window.set_light(Light::StickToCamera);
    
    let distr = Normal::new(0., 0.2);
    let height = Plain::new().add_noise(distr);
    let mut rng = rand::thread_rng();
    
    let mut quad = procedural::quad::<f32>(100., 100., 100, 100);
    
    for p in &mut quad.coords {
        // Quad is created with z=0, but y is up in kiss3d's camera. We have to
        // rotate all three coords to keep the right side up.
        p.z = p.x;
        p.x = p.y;
        // FIXME: use Normal::<f32> distribution
        p.y = height.sample(Vector2::new(p.x as f64, p.z as f64), &mut rng) as f32;    // 0.0 + noise
    }
    quad.recompute_normals();
    
    let mut quad = window.add_trimesh(quad, Vector3::from_element(1.0));
    quad.enable_backface_culling(true);
    quad.set_color(0.75, 0.65, 0.4);
    
    let mut sphere = window.add_sphere(1.0);
    sphere.set_local_translation(Translation3::new(0., 1., 0.));
    sphere.set_color(0.6, 0.2, 0.8);
    
    let mut camera = kiss3d::camera::ArcBall::new(Point3::new(0., 3., 15.), Point3::new(0., 0., 1.));
    
    while window.render_with_camera(&mut camera) {
    }
}
