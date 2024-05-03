use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material: std::fmt::Debug + Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour,Ray)>;
}
