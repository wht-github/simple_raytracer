use rand::rngs::ThreadRng;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radius: f64,
}
impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ration: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ration * viewport_height;


        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(w).unit_vec();
        let v = w.cross(u);

        let focal_length = 1.0;
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - focus_dist*w;
        
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,v,w,
            lens_radius: aperture/2.
        }
    }
    pub fn get_ray(&self, s: f64, t: f64,rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_untit_disk(rng);
        let offset = self.u*rd.0 + self.v*rd.1;
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        }
    }
}
