use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use image::{Rgb, Rgb32FImage};
use indicatif::ProgressBar;
use rand::{Rng, thread_rng};
use rayon::prelude::*;

use crate::camera::Camera;
use crate::colour::Colour;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;

pub struct OutputImageParams {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
}

impl OutputImageParams {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let mut p = OutputImageParams::default();
        p.aspect_ratio = aspect_ratio;
        p.image_width = image_width;
        p.image_height = (image_width as f32 / aspect_ratio) as u32;
        p
    }
}

impl Default for OutputImageParams {
    fn default() -> Self {
        const ASPECT_RATIO: f32 = 3.0 / 2.0;
        const IMAGE_WIDTH: u32 = 400;
        const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
        const SAMPLES_PER_PIXEL: u32 = 500;
        const MAX_DEPTH: i32 = 50;
        Self {
            aspect_ratio: ASPECT_RATIO,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
            samples_per_pixel: SAMPLES_PER_PIXEL,
            max_depth: MAX_DEPTH,
        }
    }
}

pub struct Raytracer {
    pub camera: Camera,
    pub world: HittableList,
    pub output_image: Arc<Mutex<Rgb32FImage>>,
    pub output_image_params: OutputImageParams,
}

impl Raytracer {
    pub fn new(camera: Camera, world: HittableList, output_image_params: OutputImageParams) -> Self {
        let output_image = Arc::new(Mutex::new(Rgb32FImage::new(output_image_params.image_width, output_image_params.image_height)));
        Self {
            camera: camera,
            world: world,
            output_image_params: output_image_params,
            output_image: output_image,
        }
    }

    pub fn run(&self, pb: &ProgressBar) {
        (0..self.output_image_params.image_height).into_par_iter().for_each(|y| {
            let mut rng = thread_rng();
            for x in 0..self.output_image_params.image_width {
                let mut pixel_colour: Colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..self.output_image_params.samples_per_pixel {
                    let u = (x as f32 + rng.gen::<f32>()) / (self.output_image_params.image_width - 1) as f32;
                    let v = ((self.output_image_params.image_height - y - 1) as f32 + rng.gen::<f32>()) / (self.output_image_params.image_height - 1) as f32;
                    let ray = self.camera.get_ray(u, v);
                    pixel_colour += self.ray_colour(&ray, self.output_image_params.max_depth);
                }
                let mut locked_output_image = self.output_image.lock().unwrap();
                self.write_colour(&mut locked_output_image, x, y, &pixel_colour, self.output_image_params.samples_per_pixel);
            }
            pb.inc(1);
        });
        pb.finish_with_message("done");
    }

    fn ray_colour(&self, r: &Ray, depth: i32) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }
        let hit_record = self.world.hit(r, 0.001, None);
        if let Some(hit) = hit_record {
            if let Some(mat) = hit.material.scatter(&r, &hit) {
                let (attenuation, scattered) = mat;
                return attenuation * self.ray_colour(&scattered, depth - 1);
            }
            else {
                Colour::new(0.0, 0.0, 0.0);
            }
        }
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
    }

    fn write_colour(&self, output_image: &mut Rgb32FImage, x: u32, y: u32, pixel_colour: &Colour, samples_per_pixel: u32) {
        let scale = 1.0 / samples_per_pixel as f32;
        let pixel_colour = Colour {
            r: (scale * pixel_colour.r).sqrt(),
            g: (scale * pixel_colour.g).sqrt(),
            b: (scale * pixel_colour.b).sqrt(),
        }.clamp(0.0, 1.0);
        let rgb_colour: Rgb<f32> = pixel_colour.into();
        output_image.put_pixel(x, y, rgb_colour);
    }

    pub fn save_image(&self, output_path: &PathBuf) {
        self.output_image.lock().unwrap().save(output_path).unwrap();
    }
}