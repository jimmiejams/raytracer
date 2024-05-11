use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    pub is_moving: bool,
    centre_vec: Vec3,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            centre: centre,
            radius: radius,
            material: material,
            is_moving: false,
            centre_vec: Vec3::default(),
        }
    }

    pub fn new_moving(centre1: Vec3, centre2: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            centre: centre1,
            radius: radius,
            material: material,
            is_moving: true,
            centre_vec: centre2 - centre1,
        }
    }

    pub fn sphere_centre(&self, time: f32) -> Vec3 {
        self.centre + time * self.centre_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let centre = if self.is_moving { self.sphere_centre(r.time) } else { self.centre };
        let oc = centre - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let disc_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - disc_sqrt) / a;
        if !ray_t.surrounds(root) {
            let root = (h + disc_sqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - centre) / self.radius;
        let mut hit = HitRecord::new(p, root, Arc::clone(&self.material));
        hit.set_face_normal(r, &outward_normal);
        Some(hit)
    }
}