use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour,Ray)>;
}