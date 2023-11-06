use glam::{Vec3, IVec2};
use image::{Rgb};

use crate::ray::{Ray};


#[derive(Debug)]
pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f32,
    pub material: T,
    pub tolerance: f32
}

impl<T: Material> Sphere<T> {
    fn on(&self, point: Vec3) -> bool {
        let dist = (point - self.center).length();
        // println!("point {:?} is {:?} from the sphere center", point, dist);
        (dist - self.radius).abs() <= self.tolerance
    }
}

impl<T: Material + std::fmt::Debug> SceneObject for Sphere<T> {
    fn intersection(&self, ray: &Ray) -> Option<Vec<Vec3>> {
        let co = ray.origin - self.center;
        let a = Vec3::dot(ray.dir, ray.dir);
        let b = 2.0*Vec3::dot(co, ray.dir);
        let c = Vec3::dot(co, co) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant == 0.0 {
            let t = -b/(2.0*a);
            Some(vec![ray.at(t)])
        } else if discriminant > 0.0 {
            let (t1, t2) = ((-b-f32::sqrt(discriminant))/(2.0*a),
                            (-b+f32::sqrt(discriminant))/(2.0*a));
            Some(vec![ray.at(t1),ray.at(t2)])
        } else {
            None
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
    fn intersection(&self, ray: &Ray) -> Option<Vec<Vec3>>;

    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>>;
}

pub trait Material {
    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>>;
}

#[derive(Debug)]
pub struct SolidColor{
    pub(crate) color: Rgb<u8>,
}

impl Material for SolidColor {
    fn get_color(&self, point: Vec3) -> Option<Rgb<u8>> {
        Some(self.color.clone())
    }
}

