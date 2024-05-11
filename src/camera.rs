use crate::ray::Ray;
use crate::raytracer::OutputImageParams;
use crate::vec3::Vec3;

pub struct Camera {
    centre: Vec3,
    pixel_00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
    defocus_angle: f32,
}

impl Camera {
    pub fn new(look_from: Vec3,
               look_at: Vec3,
               vup: Vec3,
               vfov: f32,
               aspect_ratio: f32,
               focus_dist: f32,
               defocus_angle: f32,
               output_image_params: &OutputImageParams,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (output_image_params.image_width as f32);
        let pixel_delta_v = viewport_v / (output_image_params.image_height as f32);

        let viewport_upper_left = look_from - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();

        Self {
            centre: look_from,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            pixel_00_loc: pixel_00_loc,
            defocus_disc_u: u * defocus_radius,
            defocus_disc_v: v * defocus_radius,
            defocus_angle: defocus_angle,
        }
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((i as f32) + offset.x) * self.pixel_delta_u
            + ((j as f32) + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 { self.centre } else { self.defocus_disc_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rand::random::<f32>();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(rand::random::<f32>() - 0.5,
                  rand::random::<f32>() - 0.5,
                  0.0)
    }

    fn defocus_disc_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disc();
        self.centre + (p.x * self.defocus_disc_u) + (p.y * self.defocus_disc_v)
    }
}
