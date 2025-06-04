use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

const RGB_CORRECTION: f64 = 255.9999;

use crate::util::random_real;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    pub const fn y(&self) -> f64 {
        self.e[1]
    }

    pub const fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn random_real() -> Self {
        Vec3::new(random_real(), random_real(), random_real())
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_real();
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn reflect(&self, n: &Vec3) -> Self {
        *self - 2.0 * dot(self, n) * *n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = dot(&self.neg(), n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let abs = (1.0 - r_out_perp.length_squared()).abs().sqrt();
        let r_out_parallel = -abs * *n;
        r_out_perp + r_out_parallel
    }
}

impl Colour {
    pub const fn r(&self) -> f64 {
        self.e[0]
    }

    pub const fn g(&self) -> f64 {
        self.e[1]
    }

    pub const fn b(&self) -> f64 {
        self.e[2]
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.r() * RGB_CORRECTION) as u8,
            (self.g() * RGB_CORRECTION) as u8,
            (self.b() * RGB_CORRECTION) as u8,
        ]
    }
    pub fn to_rgb_gamma_corrected(&self) -> [u8; 3] {
        [
            (self.r().sqrt() * RGB_CORRECTION) as u8,
            (self.g().sqrt() * RGB_CORRECTION) as u8,
            (self.b().sqrt() * RGB_CORRECTION) as u8,
        ]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        let mut output = Vec3::new(0.0, 0.0, 0.0);
        iter.for_each(|v| {
            output += v;
        });

        output
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}
