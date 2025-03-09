use glam::{Quat, UVec2, Vec3, Vec4};
use image::{Rgb, RgbImage};
use crate::ray::Ray;


pub fn rotate_by_quaternion(rotation: Quat, point: Vec3) -> Vec3 {
    let point_quat = Quat::from_xyzw(point.x, point.y, point.z, 0.0);
    (rotation * point_quat * rotation.conjugate()).xyz()
}
pub struct Camera<F>
where F: Fn(Ray) -> Rgb<u8>
{
    pub origin: Vec3,
    pub rotation: Quat,
    pub canvas: Canvas,
    pub focal_length: f32,
    pub viewport: Viewport,
    pub background_generator: F,
    pixel_origin: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl<F> Camera<F>
where F: Fn(Ray) -> Rgb<u8>
{
    pub fn new(origin: Vec3,
               rotation: Quat,
               canvas_size: (u32, u32),
               focal_length: f32,
               viewport_width: f32,
               background_generator: F) -> Camera<F>
    {
        let viewport = Viewport {width: viewport_width, height: viewport_width * (canvas_size.1 as f32)/(canvas_size.0 as f32)};
        println!("Viewport width: {}, height: {}", viewport.width, viewport.height);
        let pixel_delta_u = rotate_by_quaternion(rotation, Vec3::new(0.0, -viewport.width / (canvas_size.0 as f32), 0.0));
        let pixel_delta_v = rotate_by_quaternion(rotation, Vec3::new(0.0, 0.0, -viewport.height / (canvas_size.1 as f32)));
        let pixel_origin = origin +
            rotate_by_quaternion(rotation, Vec3::new(focal_length, viewport.width/2.0, viewport.height/2.0)) +
            0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            origin,
            rotation,
            canvas: Canvas::new(canvas_size.0, canvas_size.1),
            focal_length,
            viewport,
            background_generator,
            pixel_origin,
            pixel_delta_u,
            pixel_delta_v
        }
    }

    pub fn create_ray(&self, pixel: UVec2) -> Ray {
        return Ray {
            origin: self.origin.clone(),
            dir: self.canvas_to_viewport(pixel),
        }
    }

    pub fn canvas_to_viewport(&self, coord: UVec2) -> Vec3 {
        (self.pixel_origin - self.origin) + (coord.x as f32)*self.pixel_delta_u + (coord.y as f32)*self.pixel_delta_v
    }

    pub fn put_pixel(&mut self, i: u32, j: u32, color: Rgb<u8>) {
        // println!("Putting color {:?} to pixel ({:?}, {:?})", color, x, y);
        self.canvas.screen.put_pixel(i, j, color);
    }

}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub screen: RgbImage,
}

impl Canvas {
    fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            screen: RgbImage::new(width, height)
        }
    }
}

pub struct Viewport {
    pub width: f32,
    pub height: f32,
}