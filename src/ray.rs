use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray { origin: origin, direction: direction, time: time }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        let o = Vec3::new(0.0, 0.0, 0.0);
        let d = Vec3::new(1.0, 1.0, 1.0);
        let r = Ray::new(o, d, 0.0);
        let a1 = r.at(1.0);
        assert_eq!(a1.x, 0.0 + 1.0);
        assert_eq!(a1.y, 0.0 + 1.0);
        assert_eq!(a1.z, 0.0 + 1.0);
        let a2 = r.at(2.0);
        assert_eq!(a2.x, 0.0 + 2.0);
        assert_eq!(a2.y, 0.0 + 2.0);
        assert_eq!(a2.z, 0.0 + 2.0);
        let a3 = r.at(-1.0);
        assert_eq!(a3.x, 0.0 + -1.0);
        assert_eq!(a3.y, 0.0 + -1.0);
        assert_eq!(a3.z, 0.0 + -1.0);
    }
}