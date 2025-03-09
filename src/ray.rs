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

    pub fn intersections<'a>(&self, objects : &'a Vec<Sphere<SolidColor>>) -> Vec<(&'a Sphere<SolidColor>, f32)> {
        let mut intersections = Vec::new();
        for obj in objects {
            if let Some(hits) = obj.intersection(self) {
                for t in hits {
                    // println!("point {:?} intersects sphere {:?}", p, obj);
                    intersections.push((obj, t));
                }
            }
        }
        intersections
    }
}
