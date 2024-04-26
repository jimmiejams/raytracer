use rand::random;
use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug)]
pub struct DialectricMaterial {
    pub ir: f32,
}

impl DialectricMaterial {
    pub fn new(index_of_refraction: f32) -> DialectricMaterial {
        DialectricMaterial {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DialectricMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || DialectricMaterial::reflectance(cos_theta, refraction_ratio) > random::<f32>() {
            unit_direction.reflect(&rec.normal)
        }
        else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}