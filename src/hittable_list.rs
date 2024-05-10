use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Debug)]
pub enum HittableObject {
    Sphere(crate::sphere::Sphere),
}

#[derive(Default,Debug)]
pub struct HittableList {
    pub objects: Vec<HittableObject>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut best_hit: Option<HitRecord> = None;
        let mut closest_so_far: f32 = ray_t.max;
        for i in &self.objects {
            let hit = match i {
                HittableObject::Sphere(s) => s.hit(r, &Interval::new(ray_t.min, closest_so_far))
            };
            // if hit() returns Some() then we know it's closer than the previous t_max
            if let Some(hit) = hit {
                closest_so_far = hit.t;
                best_hit = Some(hit);
            }
        }
        best_hit
    }
}