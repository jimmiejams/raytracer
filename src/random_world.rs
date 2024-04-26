use std::sync::Arc;
use crate::colour::Colour;
use crate::dialectric_material::DialectricMaterial;
use crate::hittable_list::{HittableList, HittableObject};
use crate::lambertian_material::LambertianMaterial;
use crate::material::Material;
use crate::metal_material::MetalMaterial;
use crate::random::random_range;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub fn random_world() -> HittableList {
    let mut world = HittableList::new();
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

    world
}
