use std::sync::Arc;
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub enum HittableObject {
    Sphere(crate::sphere::Sphere),
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            ..Default::default()
        }
    }

    pub fn add(&mut self, object: &Arc<dyn Hittable>) {
        self.objects.push(Arc::clone(object));
        self.bbox = AABB::new_from_aabb(&self.bbox, &object.bounding_box());
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

    fn bounding_box(&self) -> AABB {
        *(self.bbox)
    }
}