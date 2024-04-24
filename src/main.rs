use std::rc::Rc;

use image::{Rgb, Rgb32FImage};
use indicatif::ProgressBar;
use clap::Parser;

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
use raytracer::random::random_range;

#[derive(Parser)]
struct Cli {
    output_path: std::path::PathBuf,
}

fn random_scene(world: &mut HittableList) {
    let material_ground: Rc<dyn Material> = Rc::new(LambertianMaterial::new(&Colour::new(0.5, 0.5, 0.5)));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &Rc::clone(&material_ground))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rand::random::<f32>();
            let centre = Vec3::new(
                (a as f32) + 0.9 * rand::random::<f32>(),
                0.2,
                (b as f32) + 0.9 * rand::random::<f32>()
            );
            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Colour::random_new() * Colour::random_new();
                    let sphere_material: Rc<dyn Material> = Rc::new(LambertianMaterial::new(&albedo));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, &Rc::clone(&sphere_material))
                        )
                    );
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_range_new(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material: Rc<dyn Material> = Rc::new(MetalMaterial::new(&albedo, fuzz));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, &Rc::clone(&sphere_material))
                        )
                    );
                } else {
                    let sphere_material: Rc<dyn Material> = Rc::new(DialectricMaterial::new(1.5));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, &Rc::clone(&sphere_material))
                        )
                    );
                }
            }
        }
    }

    let material1: Rc<dyn Material> = Rc::new(DialectricMaterial::new(1.5));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &Rc::clone(&material1))
        )
    );

    let material2: Rc<dyn Material> = Rc::new(LambertianMaterial::new(&Colour::new(0.4, 0.2, 0.1)));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &Rc::clone(&material2))
        )
    );

    let material3: Rc<dyn Material> = Rc::new(MetalMaterial::new(&Colour::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &Rc::clone(&material3))
        )
    );
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command-line arguments
    let args = Cli::parse();
    if args.output_path.extension().unwrap() != "exr" {
        return Err(format!("output filename requires an '.exr' extension").into());
    }

    // image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: i32 = 50;

    // world
    let mut world: HittableList = HittableList::new();
    random_scene(&mut world);

    // camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.02;
    let camera = Camera::new(
        &look_from,
        &look_at,
        &vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus
    );

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

    output_image.save(args.output_path).unwrap();

    Ok(())
}
