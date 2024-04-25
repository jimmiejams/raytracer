use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f32,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3,
               look_at: Vec3,
               vup: Vec3,
               vfov: f32,
               aspect_ratio: f32,
               aperture: f32,
               focus_dist: f32
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from.clone();
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin - offset;
        Ray::new(origin, direction)
    }
}
