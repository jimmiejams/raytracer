use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32) -> Sphere {
        Sphere {
            centre: centre,
            radius: radius,
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
        let mut hit = HitRecord {
            t: root,
            p: p,
            ..Default::default()
        };
        hit.set_face_normal(r, &outward_normal);
        Some(hit)
    }
}