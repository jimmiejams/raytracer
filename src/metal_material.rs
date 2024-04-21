use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct MetalMaterial {
    pub albedo: Vec3,
}

impl MetalMaterial {
    pub fn new(a: &Vec3) -> Self {
        MetalMaterial {
            albedo: *a,
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(&rec.p, &reflected);
        return if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}