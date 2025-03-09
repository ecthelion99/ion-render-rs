mod ray;
mod scene;
mod objects;
mod camera;

use std::error::Error;
use std::f32::consts::{FRAC_PI_4, PI};
use image::{Rgb, RgbImage};
use glam::{Quat, UVec2, Vec3};
use show_image::create_window;
use crate::scene::*;
use crate::objects::*;
use crate::camera::*;

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let mut scene = Scene {
        camera: vec![Camera::new(Vec3::new(0.0, 0.0, 0.0), Quat::from_rotation_z(0.0),(256, 256),
        1.0,2.0 , lerp(Rgb([255, 255, 255]), Rgb([128, 179,255])))],
        objects: vec![
            Sphere{
                center: Vec3::new(2.5, 0., 0.0),
                radius: 1.,
                material: SolidColor{
                    color: Rgb([255, 0, 0]),
                },
                tolerance: 1e-1
            },
            // Sphere{
            //     center: Vec3::new(2., 0., 4.),
            //     radius: 1.,
            //     material: SolidColor{
            //         color: Rgb([0, 0, 255]),
            //     },
            //     tolerance: 1e-5
            // },
            // Sphere{
            //     center: Vec3::new(-2., 0., 4.),
            //     radius: 1.,
            //     material: SolidColor{
            //         color: Rgb([0, 255, 0]),
            //     },
            //     tolerance: 1e-5
            // }
        ]
    };

    let window = create_window("Render", Default::default())?;
    scene.render(&window, 0, "test1")?;
    window.wait_until_destroyed()?;
    Ok(())
}