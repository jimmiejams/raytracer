use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material: std::fmt::Debug {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour,Ray)>;
}
