pub mod ray;
pub mod vec3;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use image::{Rgb, Rgb32FImage};
use indicatif::ProgressBar;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_colour(r: &Ray) -> Rgb<f32> {
    let sphere_centre = Vec3::new(0.0, 0.0, -1.0);
    let sphere_radius: f32 = 0.5;
    if hit_sphere(&sphere_centre, sphere_radius, &r) {
        return Rgb([1.0, 0.0, 0.0]);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let c: Vec3 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    Rgb([c.x, c.y, c.z])
}

fn hit_sphere(centre: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - *centre;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let args: Vec<String> = env::args().collect();
    let output_filename = Path::new(&args[1]);
    assert_eq!(output_filename.extension(), Some(OsStr::new("exr")));

    let mut output_image = Rgb32FImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let u = (x as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (y as f32) / (IMAGE_HEIGHT - 1) as f32;
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let ray = Ray::new(&origin, &direction);
            let pixel_colour = ray_colour(&ray);
            output_image.put_pixel(x, y, pixel_colour);
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");

    output_image.save(output_filename).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_sphere_test() {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const IMAGE_WIDTH: u32 = 400;
        const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

        // camera
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        let x: u32 = IMAGE_WIDTH / 2;
        let y: u32 = IMAGE_HEIGHT / 2;

        let u = (x as f32) / (IMAGE_WIDTH - 1) as f32;
        let v = (y as f32) / (IMAGE_HEIGHT - 1) as f32;
        let direction = lower_left_corner + horizontal * u + vertical * v - origin;
        let ray = Ray::new(&origin, &direction);

        let sphere_centre = Vec3::new(0.0, 0.0, -1.0);
        let sphere_radius: f32 = 0.5;

        assert!(hit_sphere(&sphere_centre, sphere_radius, &ray));
    }
}