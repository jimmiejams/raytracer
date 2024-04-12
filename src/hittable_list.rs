use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub enum HittableObject {
    Sphere(crate::sphere::Sphere),
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<HittableObject>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: Option<f32>) -> Option<HitRecord> {
        let mut best_hit: Option<HitRecord> = None;
        let mut t_max = t_max;
        for i in &self.objects {
            let hit = match i {
                HittableObject::Sphere(s) => s.hit(r, t_min, t_max),
            };
            // if hit() returns Some() then we know it's closer than the previous t_max
            if let Some(hit) = hit {
                t_max = Some(hit.t);
                best_hit = Some(hit);
            }
        }
        best_hit
    }
}