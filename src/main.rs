mod ray;
mod scene;
mod objects;

use image::{Rgb, RgbImage};
use glam::{Quat, UVec2, Vec3};
use crate::scene::*;
use crate::objects::*;

fn main() {
    let mut scene = Scene {
        camera: vec![Camera::new(Vec3::ZERO, Quat::default(),(1000, 1000),
        1.0, (1.0, 1.0), Rgb([255, 255, 255]))],
        objects: vec![
            Sphere{
                center: Vec3::new(0., -1., 3.),
                radius: 1.,
                material: SolidColor{
                    color: Rgb([255, 0, 0]),
                },
                tolerance: 1e-1
            },
            Sphere{
                center: Vec3::new(2., 0., 4.),
                radius: 1.,
                material: SolidColor{
                    color: Rgb([0, 0, 255]),
                },
                tolerance: 1e-5
            },
            Sphere{
                center: Vec3::new(-2., 0., 4.),
                radius: 1.,
                material: SolidColor{
                    color: Rgb([0, 255, 0]),
                },
                tolerance: 1e-5
            }
        ]
    };
    scene.render(0, "test1.png").expect("Error saving image");
}
