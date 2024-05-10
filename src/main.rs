use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use indicatif::ProgressBar;
use clap::{Parser, Subcommand};

use raytracer::vec3::Vec3;
use raytracer::camera::Camera;
use raytracer::raytracer::{OutputImageParams, Raytracer};
use raytracer::random_world::random_world;

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
    if output_path.extension().unwrap() != "exr" {
        return Err(format!("output filename {:?} requires an '.exr' extension", output_path).into());
    }

    // image
    let output_image_params = OutputImageParams::new(3.0 / 2.0, 400);
    // world
    let world = random_world();
    // camera
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        output_image_params.aspect_ratio,
        0.1,
        10.0
    );

    // run the raytracer
    let pb = ProgressBar::new(output_image_params.image_height as u64);
    let raytracer = Raytracer::new(camera, world, output_image_params);
    raytracer.run(&pb);
    raytracer.save_image(output_path);

    Ok(())
}

fn dump(output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let world = random_world();
    let mut file = File::create(output_path)?;
    write!(file, "{:#?}", &world)?;
    Ok(())
}
