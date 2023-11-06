use glam::{Vec3, Quat, IVec2};
use image::{ImageResult, Rgb, RgbImage};
use crate::ray::Ray;
use crate::objects::{Sphere, SolidColor};


pub struct Scene {
    pub camera: Vec<Camera>,
    pub objects : Vec<Sphere<SolidColor>>,
}

impl Scene {
    pub fn render(&mut self, camera_id: usize, name: &str) -> ImageResult<()> {
        let mut camera = &mut self.camera[camera_id];
        for i in -(camera.canvas.height as i32)/2..((camera.canvas.height as i32)/2) {
            for j in -(camera.canvas.width as i32)/2..((camera.canvas.width as i32)/2 ){
                let ray = camera.create_ray(IVec2::new(i, j));
                // println!("ray at pixel ({},{}), {:?}", i, j, ray);
                let intersections = ray.intersections(&self.objects);
                let min_point = intersections
                    .iter()
                    .fold((camera.background_color, f32::MAX),
                                 |(acc_color, acc_dist), &(color, pos)| {
                                     let distance = (pos - camera.origin).length_squared();
                                     if distance < acc_dist {
                                         (color, distance)
                                     } else {
                                         (color, distance)
                                     }
                                 });
                camera.put_pixel(i, j, min_point.0);
            }
        }
        camera.canvas.screen.save(name)
    }
}
pub struct Camera {
    pub origin: Vec3,
    pub rotation: Quat,
    pub canvas: Canvas,
    pub d: f32,
    pub viewport: Viewport,
    pub background_color: Rgb<u8>
}

impl Camera {
    pub fn new(origin: Vec3,
           rotation: Quat,
           canvas_size: (u32, u32),
           d: f32,
           viewport_size: (f32, f32),
           background_color: Rgb<u8>) -> Camera {
        Camera {
            origin,
            rotation,
            canvas: Canvas::new(canvas_size.0, canvas_size.1),
            d,
            viewport: Viewport {
                width: viewport_size.0,
                height: viewport_size.1,
            },
            background_color,
        }
    }

    pub fn create_ray(&self, pixel: IVec2) -> Ray {
        return Ray {
            origin: self.origin.clone(),
            dir: self.canvas_to_viewport(pixel),
        }
    }

    fn canvas_to_viewport(&self, coord: IVec2) -> Vec3 {
        Vec3 {
            x: (coord.x as f32) * self.viewport.width / (self.canvas.width as f32) ,
            y: (coord.y as f32) * self.viewport.height / (self.canvas.height as f32),
            z: self.d,
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: Rgb<u8>) {
        // println!("Putting color {:?} to pixel ({:?}, {:?})", color, x, y);
        self.canvas.screen.put_pixel((x + (self.canvas.width/2) as i32) as u32,
                                     self.canvas.height - ((y + (self.canvas.height/2) as i32) as u32) -1,
                                        color);
    }

}

pub struct Canvas {
    width: u32,
    height: u32,
    screen: RgbImage,
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
    width: f32,
    height: f32,
}