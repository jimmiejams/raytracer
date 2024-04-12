use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        const VIEWPORT_HEIGHT: f32 = 2.0;
        let viewport_width: f32 = aspect_ratio * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        Self {
            origin: origin.clone(),
            horizontal: horizontal.clone(),
            vertical: vertical.clone(),
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v - self.origin.clone();
        Ray::new(&self.origin, &direction)
    }
}
