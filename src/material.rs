use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use rand::prelude::*;
pub trait Scatter {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord, rng: &mut ThreadRng)
        -> Option<(Ray, Color)>;
}
pub struct Lambertian {
    pub albedo: Color,
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
pub struct Dielectric {
    pub ir: f64, //index of refraction
}
impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vec(rng);
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        let scattered = Ray {
            origin: hit_rec.p,
            dir: scatter_direction,
        };
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {
        let reflected = r_in.dir.unit_vec().reflect(hit_rec.normal);
        let scattered = Ray {
            origin: hit_rec.p,
            dir: reflected + self.fuzz.min(1.0) * Vec3::random_in_unit_sphere(rng),
        };
        let attenuation = self.albedo;
        if scattered.dir.dot(hit_rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Color)> {

        let attenuation = Color(1.,1.,1.);
        let refraction_ratio = if hit_rec.front_face {1.0/ self.ir} else{self.ir};
        let unit_direction = r_in.dir.unit_vec();
        let cos_theta = (-unit_direction).dot(hit_rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();
        let mut direction = Vec3(0.,0.,0.);
        if refraction_ratio * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0. .. 1.){
            direction = unit_direction.reflect(hit_rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, hit_rec.normal, refraction_ratio);
        }
        let scattered = Ray {
            origin: hit_rec.p,
            dir: direction,
        };
        Some((scattered,attenuation))
    }
}
impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64{
        let r0 = ((1. -ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. -r0)*(1. -cosine).powi(5)
    }
}