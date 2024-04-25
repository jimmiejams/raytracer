use std::sync::Arc;
use crate::material::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: Option<f32>) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn new(position: &Vec3, t: f32, material: Arc<dyn Material + Send + Sync>) -> Self {
        HitRecord {
            p: *position,
            normal: Vec3::default(),
            t: t,
            front_face: false,
            material: material,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }
}