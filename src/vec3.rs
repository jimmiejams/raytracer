use std::ops::{AddAssign, DivAssign, MulAssign, Neg, SubAssign, Mul, Add, Div, Sub};
use std::convert::Into;
use image::Rgb;
use crate::random;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn random_new() -> Vec3 {
        Vec3 {
            x: rand::random::<f32>(),
            y: rand::random::<f32>(),
            z: rand::random::<f32>(),
        }
    }

    pub fn random_range_new(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random::random_range(min, max),
            y: random::random_range(min, max),
            z: random::random_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range_new(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn clamp(&self, min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: if self.x < min { min } else if self.x > max { max } else { self.x },
            y: if self.y < min { min } else if self.y > max { max } else { self.y },
            z: if self.z < min { min } else if self.z > max { max } else { self.z },
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Into<Rgb<f32>> for Vec3 {
    fn into(self) -> Rgb<f32> {
        Rgb([self.x, self.y, self.z])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let v:Vec3 = Vec3::default();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }
    #[test]
    fn new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn random_new_range() {
        let v = Vec3::random_range_new(-2.0, 2.0);
        assert!(v.x >= -2.0);
        assert!(v.x <= 2.0);
        assert!(v.y >= -2.0);
        assert!(v.y <= 2.0);
        assert!(v.z >= -2.0);
        assert!(v.z <= 2.0);
    }

    #[test]
    fn random_in_unit_sphere() {
        let v = Vec3::random_in_unit_sphere();
        assert!(v.length() < 1.0);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn length_squared() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length_squared(), 9.0);
    }

    #[test]
    fn add_assign() {
        let mut u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        u += v;
        assert_eq!(u.x, 1.0 + 4.0);
        assert_eq!(u.y, 2.0 + 5.0);
        assert_eq!(u.z, 3.0 + 6.0);
    }

    #[test]
    fn add() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let w = u + v;
        assert_eq!(w.x, 1.0 + 4.0);
        assert_eq!(w.y, 2.0 + 5.0);
        assert_eq!(w.z, 3.0 + 6.0);
    }

    #[test]
    fn sub_assign() {
        let mut u = Vec3::new(3.0, 3.0, 3.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        u -= v;
        assert_eq!(u.x, 3.0 - 1.0);
        assert_eq!(u.y, 3.0 - 2.0);
        assert_eq!(u.z, 3.0 - 3.0);
    }

    #[test]
    fn sub() {
        let u = Vec3::new(3.0, 3.0, 3.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = u - v;
        assert_eq!(w.x, 3.0 - 1.0);
        assert_eq!(w.y, 3.0 - 2.0);
        assert_eq!(w.z, 3.0 - 3.0);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v.x, 1.0 * 2.0);
        assert_eq!(v.y, 2.0 * 2.0);
        assert_eq!(v.z, 3.0 * 2.0);
    }

    #[test]
    fn mul() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = u * 2.0;
        assert_eq!(v.x, 1.0 * 2.0);
        assert_eq!(v.y, 2.0 * 2.0);
        assert_eq!(v.z, 3.0 * 2.0);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(3.0, 2.0, 1.0);
        v /= 2.0;
        assert_eq!(v.x, 3.0 / 2.0);
        assert_eq!(v.y, 2.0 / 2.0);
        assert_eq!(v.z, 1.0 / 2.0);
    }

    #[test]
    fn div() {
        let u = Vec3::new(3.0, 2.0, 1.0);
        let v = u / 2.0;
        assert_eq!(v.x, 3.0 / 2.0);
        assert_eq!(v.y, 2.0 / 2.0);
        assert_eq!(v.z, 1.0 / 2.0);
    }

    #[test]
    fn neg() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = -u;
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -2.0);
        assert_eq!(v.z, -3.0);
    }

    #[test]
    fn dot_product() {
        let u = Vec3::new(1.0, 1.0, 1.0);
        let v = Vec3::new(-2.0, 0.5, -0.5);
        let uv = u.dot(&v);
        assert_eq!(uv, 1.0 * -2.0 + 1.0 * 0.5 + 1.0 * -0.5);
    }

    #[test]
    fn cross_product() {
        let u = Vec3::new(1.0, 1.0, 1.0);
        let v = Vec3::new(-2.0, 0.5, -0.5);
        let uv = u.cross(&v);
        assert_eq!(uv.x, u.y * v.z - u.z * v.y);
        assert_eq!(uv.y, u.z * v.x - u.x * v.z);
        assert_eq!(uv.z, u.x * v.y - u.y * v.x);
    }

    #[test]
    fn unit_vector() {
        let u = Vec3::new(1.0, 2.0, 2.0);
        let v = u.unit_vector();
        assert_eq!(v.x, 1.0 / 3.0);
        assert_eq!(v.y, 2.0 / 3.0);
        assert_eq!(v.z, 2.0 / 3.0);
    }

    #[test]
    fn clamp() {
        let u = Vec3::new(-1.0, 0.5, 1.5);
        let v = u.clamp(0.0, 1.0);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.5);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn into_rgb() {
        let u = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(Rgb([0.1, 0.2, 0.3]), u.into());
    }
}
