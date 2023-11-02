use glam::{Vec3, Quat, IVec2};
use image::{RgbImage};

struct Camera {
    origin: Vec3,
    rotation: Quat,
    canvas: Canvas,
    d: f32,
    viewport: Viewport,
}

impl Camera {
    pub fn new(origin: Vec3,
           rotation: Quat,
           canvas_size: (u32, u32),
           d: f32,
           viewport_size: (f32, f32)) -> Camera {
        Camera {
            origin,
            rotation,
            canvas: Canvas::new(canvas_size.0, canvas_size.1),
            d,
            viewport: Viewport {
                width: viewport_size.0,
                height: viewport_size.1,
            }
        }
    }

    fn canvas_to_viewport(&self, coord: IVec2) -> Vec3 {
        Vec3 {
            x: coord.x * self.viewport.width / self.canvas.width as f32 ,
            y: coord.x * self.viewport.height / self.canvas.height as f32,
            z: self.d,
        }
    }

}

struct Canvas {
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

struct Viewport {
    width: f32,
    height: f32,
}