pub mod ray;
pub mod vec3;
mod hittable;
mod sphere;
mod hittable_list;
mod random;
mod camera;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use image::{Rgb, Rgb32FImage};
use indicatif::ProgressBar;
use ray::Ray;
use vec3::Vec3;
use hittable_list::*;
use sphere::Sphere;
use crate::camera::Camera;
use crate::hittable::Hittable;

fn ray_colour(r: &Ray, world: &impl Hittable) -> Rgb<f32> {
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
    let camera = Camera::new(ASPECT_RATIO);

    let args: Vec<String> = env::args().collect();
    let output_filename = Path::new(&args[1]);
    assert_eq!(output_filename.extension(), Some(OsStr::new("exr")));

    let mut output_image = Rgb32FImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let u = (x as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (y as f32) / (IMAGE_HEIGHT - 1) as f32;
            let ray = camera.get_ray(u, v);
            let pixel_colour = ray_colour(&ray, &world);
            output_image.put_pixel(x, y, pixel_colour);
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");

    output_image.save(output_filename).unwrap();
}
