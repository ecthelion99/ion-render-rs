mod ray;
mod camera;

use image::{Rgb, RgbImage};
use glam::{UVec2};

fn main() {
    let image_size = UVec2::new(256, 256);
    let mut image = RgbImage::new(image_size.x as u32, image_size.y as u32);
    for i in 0..image_size.x {
        for j in 0..image_size.y {
            let color: Rgb<u8> = Rgb([
                ((i as f32 / image_size.x as f32) * u8::MAX as f32) as u8,
                ((j as f32 / image_size.y as f32) * u8::MAX as f32) as u8,
                0
                ]
            );
            image.put_pixel(i as u32, j as u32, color);
        }
    }
}
