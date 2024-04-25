use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

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
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
        let fuzzy_reflected = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        let scattered = Ray::new(rec.p, fuzzy_reflected);
        return if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}