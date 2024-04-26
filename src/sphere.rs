use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Arc<dyn Material + Sync + Send>) -> Sphere {
        Sphere {
            centre: centre,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: Option<f32>) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let disc_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - disc_sqrt) / a;
        if (root < t_min) || (t_max.is_some_and(|x| x < root)) {
            root = (-half_b + disc_sqrt) / a;
            if (root < t_min) || (t_max.is_some_and(|x| x < root)) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.centre) / self.radius;
        let mut hit = HitRecord::new(p, root, Arc::clone(&self.material));
        hit.set_face_normal(r, &outward_normal);
        Some(hit)
    }
}