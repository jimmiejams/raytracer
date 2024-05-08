use std::ops::{AddAssign, Mul, Add, DivAssign};
use std::iter::Sum;
use std::convert::Into;
use image::Rgb;
use crate::random;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Colour {
        Colour { r, g, b, }
    }

    pub fn random_new() -> Colour {
        Colour {
            r: rand::random::<f32>(),
            g: rand::random::<f32>(),
            b: rand::random::<f32>(),
        }
    }

    pub fn random_range_new(min: f32, max: f32) -> Colour {
        Colour {
            r: random::random_range(min, max),
            g: random::random_range(min, max),
            b: random::random_range(min, max),
        }
    }

    pub fn clamp(&self, min: f32, max: f32) -> Colour {
        Colour {
            r: if self.r < min { min } else if self.r > max { max } else { self.r },
            g: if self.g < min { min } else if self.g > max { max } else { self.g },
            b: if self.b < min { min } else if self.b > max { max } else { self.b },
        }
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Into<Rgb<f32>> for Colour {
    fn into(self) -> Rgb<f32> {
        Rgb([self.r, self.g, self.b])
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Add<Colour> for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Sum for Colour {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self { r: 0.0, g: 0.0, b: 0.0 }, |a, b| Self {
            r: a.r + b.r,
            g: a.g + b.g,
            b: a.b + b.b,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let c = Colour::new(0.1, 0.2, 0.3);
        assert_eq!(c.r, 0.1);
        assert_eq!(c.g, 0.2);
        assert_eq!(c.b, 0.3);
    }

    #[test]
    fn default() {
        let c = Colour::default();
        assert_eq!(c.r, 0.0);
        assert_eq!(c.g, 0.0);
        assert_eq!(c.b, 0.0);
    }

    #[test]
    fn clamp() {
        let c1 = Colour::new(-0.1, 0.0, 1.1);
        let c2 = c1.clamp(0.0, 1.0);
        assert_eq!(c2.r, 0.0);
        assert_eq!(c2.g, 0.0);
        assert_eq!(c2.b, 1.0);
    }

    #[test]
    fn into_rgb_f32() {
        let c = Colour::new(0.1, 0.2, 0.3);
        assert_eq!(Rgb([0.1, 0.2, 0.3]), c.into());
    }

    #[test]
    fn add_assign() {
        let mut c1 = Colour::new(0.1, 0.2, 0.3);
        let c2 = Colour::new(0.2, 0.3, 0.4);
        c1 += c2;
        assert_eq!(c1.r, 0.1 + 0.2);
        assert_eq!(c1.g, 0.2 + 0.3);
        assert_eq!(c1.b, 0.3 + 0.4);
    }

    #[test]
    fn add() {
        let c1 = Colour::new(0.1, 0.2, 0.3);
        let c2 = Colour::new(0.2, 0.3, 0.4);
        let c3 = c1 + c2;
        assert_eq!(c3.r, 0.1 + 0.2);
        assert_eq!(c3.g, 0.2 + 0.3);
        assert_eq!(c3.b, 0.3 + 0.4);
    }

    #[test]
    fn mul_colour() {
        let c1 = Colour::new(0.1, 0.2, 0.3);
        let c2 = Colour::new(0.2, 0.3, 0.4);
        let c3 = c1 * c2;
        assert_eq!(c3.r, 0.1 * 0.2);
        assert_eq!(c3.g, 0.2 * 0.3);
        assert_eq!(c3.b, 0.3 * 0.4);
    }

    #[test]
    fn mul_f32() {
        let c1 = Colour::new(0.1, 0.2, 0.3);
        let c2 = c1 * 0.3;
        assert_eq!(c2.r, 0.1 * 0.3);
        assert_eq!(c2.g, 0.2 * 0.3);
        assert_eq!(c2.b, 0.3 * 0.3);
    }

    #[test]
    fn div_assign_f32() {
        let mut c1 = Colour::new(0.1, 0.2, 0.3);
        c1 /= 2.0;
        assert_eq!(c1.r, 0.1 / 2.0);
        assert_eq!(c1.g, 0.2 / 2.0);
        assert_eq!(c1.b, 0.3 / 2.0);
    }
}