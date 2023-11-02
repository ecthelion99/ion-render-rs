use glam::{Vec3, IVec2};
use image::{Rgb};

use crate::ray::{Ray};


pub struct Sphere<T: Material> {
    pub center: f32,
    pub radius: f32,
    material: T,
    tolerance: f32
}

impl<T> Sphere<T> {
    fn on(&self, point: Vec3) -> bool {
        ((point - self.center).length() - self.radius).abs() <= self.tolerance
    }
}

impl<T: Material> SceneObject for Sphere<T> {
    fn intersection(&self, ray: Ray) -> Option<Vec<(Vec3, f32)>> {
        let co = ray.origin - self.center;
        let a = Vec3::dot(ray.dir, ray.dir);
        let b = 2*Vec3::dot(co, ray.dir);
        let c = Vec3::dot(co, co) - self.radius * self.radius;
        let discriminant = b*b - 4*a*c;
        if discriminant == 0 {
            let t = b/2*a;
            Some(vec![(ray.at(t), t)])
        } else if discriminant > 0 {
            let (t1, t2) = (b-f32::sqrt(discriminant)/2*a, b+f32::sqrt(discriminant)/2*a);
            Some(vec![(ray.at(t1), t1),(ray.at(t2), t2)])
        }
    }

    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>> {
        if self.on(point) {
            self.material.get_color(point)
        } else {
            None
        }
    }
}

pub trait SceneObject {
    fn intersection(&self, ray: Ray) -> Option<Vec<(Vec3, f32)>>;

    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>>;
}

pub trait Material {
    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>>;
}

struct SolidColor{
    color: Rgb<u8>,
}

impl Material for SolidColor {
    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>> {
        Some(self.color.clone())
    }
}

