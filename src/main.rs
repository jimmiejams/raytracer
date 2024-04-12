pub mod ray;
pub mod vec3;
mod hittable;
mod sphere;
mod hittable_list;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use image::{Rgb, Rgb32FImage};
use indicatif::ProgressBar;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_list::*;
use crate::sphere::Sphere;

fn ray_colour(r: &Ray, world: &HittableList) -> Rgb<f32> {
    let hit_record = world.hit(r, 0.0, None);
    if let Some(hit) = hit_record {
        let colour = (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        return Rgb([colour.x, colour.y, colour.z]);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let c: Vec3 = Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    Rgb([c.x, c.y, c.z])
}

fn main() {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // world
    let mut world: HittableList = HittableList::new();
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

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
            let pixel_colour = ray_colour(&ray, &world);
            output_image.put_pixel(x, y, pixel_colour);
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");

    output_image.save(output_filename).unwrap();
}
