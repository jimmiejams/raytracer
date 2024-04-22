use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

use image::{Rgb, Rgb32FImage};
use indicatif::ProgressBar;

use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;
use raytracer::camera::Camera;
use raytracer::hittable_list::*;
use raytracer::hittable::Hittable;
use raytracer::material::Material;
use raytracer::lambertian_material::LambertianMaterial;
use raytracer::metal_material::MetalMaterial;
use raytracer::dialectric_material::DialectricMaterial;
use raytracer::colour::Colour;

fn ray_colour(r: &Ray, world: &impl Hittable, depth: i32) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    let hit_record = world.hit(r, 0.001, None);
    if let Some(hit) = hit_record {
        if let Some(mat) = hit.material.scatter(&r, &hit) {
            let (attenuation, scattered) = mat;
            return attenuation * ray_colour(&scattered, world, depth - 1);
        }
        else {
            Colour::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}

fn write_colour(output_image: &mut Rgb32FImage, x: u32, y: u32, pixel_colour: &Colour, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f32;
    let pixel_colour = Colour {
        r: (scale * pixel_colour.r).sqrt(),
        g: (scale * pixel_colour.g).sqrt(),
        b: (scale * pixel_colour.b).sqrt(),
    }.clamp(0.0, 1.0);
    let rgb_colour: Rgb<f32> = pixel_colour.into();
    output_image.put_pixel(x, y, rgb_colour);
}

fn main() {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // world
    let mut world: HittableList = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(LambertianMaterial::new(&Colour::new(0.8, 0.8, 0.0)));
    let material_centre: Rc<dyn Material> = Rc::new(LambertianMaterial::new(&Colour::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(DialectricMaterial::new(1.5));
    let material_right: Rc<dyn Material> = Rc::new(MetalMaterial::new(&Colour::new(0.8, 0.6, 0.2), 0.0));

    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &Rc::clone(&material_ground))));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &Rc::clone(&material_centre))));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &Rc::clone(&material_left))));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, &Rc::clone(&material_left))));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &Rc::clone(&material_right))));

    // camera
    let camera = Camera::new(ASPECT_RATIO);

    let args: Vec<String> = env::args().collect();
    let output_filename = Path::new(&args[1]);
    assert_eq!(output_filename.extension(), Some(OsStr::new("exr")));

    let mut output_image = Rgb32FImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let mut pixel_colour: Colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + rand::random::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = ((IMAGE_HEIGHT - y - 1) as f32 + rand::random::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &world, MAX_DEPTH);
            }
            write_colour(&mut output_image, x, y, &pixel_colour, SAMPLES_PER_PIXEL);
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");

    output_image.save(output_filename).unwrap();
}
