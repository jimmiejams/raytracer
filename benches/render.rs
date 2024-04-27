use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use indicatif::ProgressBar;
use raytracer::benchmark_world::benchmark_world;
use raytracer::camera::Camera;
use raytracer::raytracer::{OutputImageParams, Raytracer};
use raytracer::vec3::Vec3;

fn render() {
    let output_image_params = OutputImageParams::new(3.0 / 2.0, 400);
    let world = benchmark_world();
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        output_image_params.aspect_ratio,
        0.02,
        10.0
    );
    let pb = ProgressBar::new(output_image_params.image_height as u64);
    let raytracer = Raytracer::new(camera, world, output_image_params);
    raytracer.run(&pb);
}

fn bench_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("Raytracer");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(600));
    group.bench_function("raytracer", |b| b.iter(|| render()));
    group.finish();
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
