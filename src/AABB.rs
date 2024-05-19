use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default, Debug, Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }

    pub fn new_from_points(a: &Vec3, b: &Vec3) -> Self {
        Self {
            x: if a.x <= b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) },
            y: if a.y <= b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) },
            z: if a.z <= b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) },
        }
    }

    pub fn new_from_aabb(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::new_from_interval(&box0.x, &box1.x),
            y: Interval::new_from_interval(&box0.y, &box1.y),
            z: Interval::new_from_interval(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        match n {
            0 => return self.x,
            1 => return self.y,
            2 => return self.z,
            _ => panic!("axis interval {}", n),
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut ray_t = *ray_t;
        let ray_orig = r.origin;
        let ray_dir = r.direction;

        for axis in 0..3 {
            let (ray_dir, ray_orig) = match axis {
                0 => (ray_dir.x, ray_orig.x),
                1 => (ray_dir.y, ray_orig.y),
                2 => (ray_dir.z, ray_orig.z),
                _ => panic!("ray dir axis {}", axis),
            };

            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir;

            let t0 = (ax.min - ray_orig) * adinv;
            let t1 = (ax.max - ray_orig) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            }
            else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }
}
