use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct LambertianMaterial {
    pub albedo: Vec3,
}

impl LambertianMaterial {
    pub fn new(a: &Vec3) -> Self {
        LambertianMaterial {
            albedo: *a,
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(&rec.p, &scatter_direction);
        Some((self.albedo, scattered))
    }
}