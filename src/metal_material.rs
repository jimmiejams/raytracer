use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct MetalMaterial {
    pub albedo: Colour,
    pub fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Colour, fuzz: f32) -> Self {
        MetalMaterial {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = r_in.direction.reflect(&rec.normal);
        let fuzzy_reflected = reflected.unit_vector() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = Ray::new(rec.p, fuzzy_reflected, r_in.time);
        return if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}