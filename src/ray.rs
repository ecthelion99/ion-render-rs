use glam::{Vec3};
use image::Rgb;
use crate::objects::{Sphere, SolidColor, SceneObject};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.dir*t + self.origin
    }

    pub fn intersections(&self, objects : &Vec<Sphere<SolidColor>>) -> Vec<(Rgb<u8>, Vec3)> {
        let mut intersection_points = Vec::new();
        for obj in objects {
            if let Some(points) = obj.intersection(self) {
                for p in points {
                    // println!("point {:?} intersects sphere {:?}", p, obj);
                    if let Some(color) = obj.get_color(p) {
                        intersection_points.push((color, p));
                    }
                }
            }

        }
        return intersection_points
    }
}
