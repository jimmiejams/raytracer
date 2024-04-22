use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct LambertianMaterial {
    pub albedo: Colour,
}

impl LambertianMaterial {
    pub fn new(albedo: &Colour) -> Self {
        LambertianMaterial {
            albedo: *albedo,
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(&rec.p, &scatter_direction);
        Some((self.albedo, scattered))
    }
}