use std::sync::Arc;
use rand::{Rng, thread_rng};

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::{HittableList, HittableObject};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(world: &mut HittableList) -> Self {
        return BVHNode::new_slice(world, 0, world.objects.len());
    }

    pub fn new_slice(world: &mut HittableList, start: usize, end: usize) -> Self {
        let mut new_node: BVHNode = Default::default();
        let object_span = end - start;
        if object_span == 1 {
            new_node.left = Arc::clone(&world.objects[start]);
            new_node.right = Arc::clone(&world.objects[start]);
        }
        else if object_span == 2 {
            new_node.left = Arc::clone(&world.objects[start]);
            new_node.right = Arc::clone(&world.objects[start + 1]);
        }
        else {
            let mut rng = thread_rng();
            let axis = rng.gen_range(0..2);
            let sort_by = |a: Arc<HittableObject>, b: Arc<HittableObject>| {
                let a_axis_interval = a.bounding_box().axis_interval(axis);
                let b_axis_interval = b.bounding_box().axis_interval(axis);
                a_axis_interval.min < b_axis_interval.min
            };
            let objects_slice: &mut [Arc<HittableObject>] = &mut world.objects[start..end];
            objects_slice.sort_by(sort_by);
            let mid = start + object_span / 2;
            new_node.left = Arc::new(BVHNode::new_slice(world, start, mid));
            new_node.right = Arc::new(BVHNode::new_slice(world, mid, end));
        }
        new_node.bounding_box = AABB::new_from_aabb(&new_node.left.bounding_box(), &new_node.right.bounding_box());
        new_node
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(&r, &ray_t) {
            return None;
        }
        let hit_left = self.left(&r, &ray_t);
        let right_ray = if let Some(hl) = hit_left { Interval::new(ray_t.min, hl.t) } else { Interval::new(ray_t.min, ray_t.max) };
        let hit_right = self.right(&r, &right_ray);
        if Some(hit_left) {
            return hit_left;
        }
        else if Some(hit_right) {
            return hit_right;
        }
        else {
            return None;
        }
    }

    fn bounding_box(&self) -> AABB {
        *(self.bounding_box)
    }
}