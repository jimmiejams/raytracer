#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min: min,
            max: max,
        }
    }

    pub fn new_from_interval(a: &Interval, b: &Interval) -> Self {
        Self {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    pub fn new_positive_inf() -> Self {
        Self {
            min: 0.0,
            max: f32::INFINITY,
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f32::INFINITY,
            max: -f32::INFINITY,
        }
    }
}