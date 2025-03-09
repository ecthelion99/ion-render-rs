use std::error::Error;
use glam::{Vec3, Quat, IVec2, Vec2, UVec2};
use image::{EncodableLayout, ImageResult, Rgb, RgbImage};
use show_image::{create_window, ImageInfo, ImageView, WindowProxy};
use crate::ray::Ray;
use crate::objects::{Sphere, SolidColor, SceneObject};
use crate::camera::{Camera, };

pub fn lerp(start_value: Rgb<u8>, end_value: Rgb<u8>) -> impl Fn(Ray) -> Rgb<u8>
{
    move |ray: Ray| {
        let a = 0.5*(ray.dir.normalize().z + 1.0);
        let r = (1.0 - a)*(start_value[0] as f32) + a*(end_value[0] as f32);
        let g = (1.0 - a)*(start_value[1] as f32) + a*(end_value[1] as f32);
        let b = (1.0 - a)*(start_value[2] as f32) + a*(end_value[2] as f32);
        Rgb([r as u8, g as u8, b as u8])
    }
}

fn normal_to_color(normal: Vec3) -> Rgb<u8> {
    let r = ((normal.x + 1.) * 127.) as u8;
    let g = ((normal.y + 1.) * 127.) as u8;
    let b = ((normal.z + 1.) * 127.) as u8;
    Rgb([r, g, b])
}
pub struct Scene<F>
where F: Fn(Ray) -> Rgb<u8> {
    pub camera: Vec<Camera<F>>,
    pub objects : Vec<Sphere<SolidColor>>,
}

impl<F> Scene<F>
where F: Fn(Ray) -> Rgb<u8>
{
    pub fn render(&mut self, window: &WindowProxy, camera_id: usize, name: &str) -> Result<(), Box<dyn Error>> {
        let mut camera = &mut self.camera[camera_id];
        // println!("Pixel Origin: {}, du: {}, dv: {}",camera.pixel_origin, camera.pixel_delta_u, camera.pixel_delta_v);
        for i in 0..camera.canvas.width {
            for j in 0..camera.canvas.height{
                let ray = camera.create_ray(UVec2::new(i, j));
                let intersections = ray.intersections(&self.objects);
                if let Some((obj, min_t)) = intersections
                    .iter()
                    .fold(None, |acc, &(sphere, t)| {
                        if t > 0.0 {
                            if let Some((closest_obj, min_t)) = acc {
                                if t < min_t {
                                    Some((sphere, t))
                                } else {
                                    acc
                                }
                            } else {
                                Some((sphere, t))
                            }
                        } else {
                            acc
                        }
                    })
                {
                    let point = ray.at(min_t);
                    if obj.on(point) {
                        if (j == camera.canvas.height/2) {
                            let normal = obj.get_normal(point).unwrap();
                        }
                        camera.put_pixel(i, j, normal_to_color(obj.get_normal(point).unwrap()));
                    }
                } else {
                    camera.put_pixel(i, j, (camera.background_generator)(ray));
                }
            }
        }
        let image_view = ImageView::new(
            ImageInfo::rgb8(camera.canvas.width, camera.canvas.height), // TODO: don't hardcode image type
            camera.canvas.screen.as_bytes()
        );
        window.set_image(name, image_view)?;
        Ok(())
    }
}

