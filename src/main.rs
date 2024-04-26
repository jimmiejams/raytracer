use std::sync::Arc;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use indicatif::ProgressBar;
use clap::{Parser, Subcommand};

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
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Render {
        output_path: PathBuf,
    },
    Dump {
        output_path: PathBuf,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command-line arguments
    let args = Cli::parse();
    match &args.command {
        Some(Commands::Render { output_path }) => {
            render(output_path)?;
        },
        Some(Commands::Dump { output_path }) => {
            dump(&output_path)?;
        },
        None => {},
    }
    Ok(())
}

fn render(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if output_path.extension().unwrap() != ".exr" {
        return Err(format!("output filename requires an '.exr' extension").into());
    }

    // image
    let output_image_params = OutputImageParams::new(3.0 / 2.0, 400);
    // world
    let mut world: HittableList = HittableList::new();
    random_scene(&mut world);
    // camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        output_image_params.aspect_ratio,
        0.02,
        10.0
    );

    // run the raytracer
    let pb = ProgressBar::new(output_image_params.image_height as u64);
    let raytracer = Raytracer::new(camera, world, output_image_params);
    raytracer.run(&pb);
    raytracer.save_image(output_path);

    Ok(())
}

fn random_scene(world: &mut HittableList) {
    let material_ground: Arc<dyn Material + Sync + Send> = Arc::new(LambertianMaterial::new(Colour::new(0.5, 0.5, 0.5)));
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
                    let sphere_material: Arc<dyn Material + Sync + Send> = Arc::new(LambertianMaterial::new(albedo));
                    world.objects.push(
                        HittableObject::Sphere(
                            Sphere::new(centre, 0.2, Arc::clone(&sphere_material))
                        )
                    );
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_range_new(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material: Arc<dyn Material + Sync + Send> = Arc::new(MetalMaterial::new(albedo, fuzz));
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

    let material2: Arc<dyn Material + Sync + Send> = Arc::new(LambertianMaterial::new(Colour::new(0.4, 0.2, 0.1)));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::clone(&material2))
        )
    );

    let material3: Arc<dyn Material + Sync + Send> = Arc::new(MetalMaterial::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(
        HittableObject::Sphere(
            Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::clone(&material3))
        )
    );
}

fn dump(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut world = HittableList::new();
    random_scene(&mut world);
    let mut file = File::create(output_path)?;
    write!(file, "{:#?}", &world)?;
    Ok(())
}