pub use std::f64::consts::PI;

use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_real() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: -f64::INFINITY,
};

pub const UNIVERSE: Interval = Interval {
    min: -f64::INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, num: f64) -> f64 {
        match num {
            num if num < self.min => self.min,
            num if num > self.max => self.max,
            _ => num,
        }
    }
}
impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
}
