use std::sync::Arc;

use indicatif::ProgressBar;
use clap::Parser;

use raytracer::sphere::Sphere;
use raytracer::vec3::Vec3;
use raytracer::camera::Camera;
use raytracer::hittable_list::*;
use raytracer::material::Material;
use raytracer::lambertian_material::LambertianMaterial;
use raytracer::metal_material::MetalMaterial;
use raytracer::dialectric_material::DialectricMaterial;
use raytracer::colour::Colour;
use raytracer::random::random_range;
use raytracer::raytracer::{OutputImageParams, Raytracer};

#[derive(Parser)]
struct Cli {
    output_path: std::path::PathBuf,
}

fn random_scene(world: &mut HittableList) {
    let material_ground: Arc<dyn Material + Sync + Send> = Arc::new(LambertianMaterial::new(&Colour::new(0.5, 0.5, 0.5)));
    world.objects.push(HittableObject::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::clone(&material_ground))));

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
                    let sphere_material: Arc<dyn Material + Sync + Send> = Arc::new(LambertianMaterial::new(&albedo));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, Arc::clone(&sphere_material))
                        )
                    );
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_range_new(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material: Arc<dyn Material + Sync + Send> = Arc::new(MetalMaterial::new(&albedo, fuzz));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, Arc::clone(&sphere_material))
                        )
                    );
                } else {
                    let sphere_material: Arc<dyn Material + Sync + Send> = Arc::new(DialectricMaterial::new(1.5));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, Arc::clone(&sphere_material))
                        )
                    );
                }
            }
        }
    }

    let material1: Arc<dyn Material + Sync + Send> = Arc::new(DialectricMaterial::new(1.5));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::clone(&material1))
        )
    );

    let material2: Arc<dyn Material + Sync + Send> = Arc::new(LambertianMaterial::new(&Colour::new(0.4, 0.2, 0.1)));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&material2))
        )
    );

    let material3: Arc<dyn Material + Sync + Send> = Arc::new(MetalMaterial::new(&Colour::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&material3))
        )
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command-line arguments
    let args = Cli::parse();
    if args.output_path.extension().unwrap() != "exr" {
        return Err(format!("output filename requires an '.exr' extension").into());
    }

    // image
    let output_image_params = OutputImageParams::new(3.0 / 2.0, 400);
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
        output_image_params.aspect_ratio,
        aperture,
        dist_to_focus
    );

    // run the raytracer
    let pb = ProgressBar::new(output_image_params.image_height as u64);
    let raytracer = Raytracer::new(camera, world, output_image_params);
    raytracer.run(&pb);
    raytracer.save_image(&args.output_path);

    Ok(())
}
