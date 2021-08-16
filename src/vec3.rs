
use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);
// pub type Color = Vec3;
pub use Vec3 as Color;
pub use Vec3 as Point3;
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vec(self) -> Self {
        self / self.length()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    #[inline]
    pub fn random(r_min: f64, r_max: f64, rng: &mut ThreadRng) -> Self {
        Self(
            rng.gen_range(r_min..r_max),
            rng.gen_range(r_min..r_max),
            rng.gen_range(r_min..r_max),
        )
    }
    #[inline]   
    pub fn random_in_unit_sphere(rng: &mut ThreadRng)->Vec3{
        loop{
            let p = Vec3::random(-1., 1., rng);
            if p.length_squared() >= 1.{
                continue;
            } else{
                break p;
            }
        }
    }
    pub fn random_unit_vec(rng: &mut ThreadRng)->Vec3{
        Self::unit_vec(Self::random_in_unit_sphere(rng))
    }
    pub fn random_in_hemisphere(normal: Vec3,rng: &mut ThreadRng) ->Vec3{
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0. {
            in_unit_sphere
        }else {
            -in_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool{
        let s = 1e-8;
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }
    pub fn reflect(self,n: Vec3) -> Vec3{
        self - 2.0*self.dot(n)*n
    }
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) ->Vec3{
        let cos_theta = (-uv).dot(n).min(1.);
        let r_out_perp =etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = - (1. - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
    pub fn random_in_untit_disk(rng: &mut ThreadRng) -> Vec3{
        loop {
            let p = Vec3(rng.gen_range(-1. .. 1. ),rng.gen_range(-1. .. 1. ),0.);
            if p.length_squared() >= 1.{
                continue;
            } else{
                break p;
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        // Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
        rhs * self
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.0 * other.0, self.1 * other.1, self.2 * other.2);
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self {
        (1.0 / rhs) * self
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
