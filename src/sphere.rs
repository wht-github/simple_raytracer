use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Color, Point3, Vec3},
    material::Scatter
};
use std::rc::Rc;
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Rc<dyn Scatter>,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0. {
            return None;
        }
        let sqrtdelta = delta.sqrt();
        let mut root = (-half_b - sqrtdelta) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtdelta) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let tmp = ray.point_at_parameter(root);
        let mut hr = HitRecord {
            t: root,
            p: tmp,
            normal: (tmp - self.center) / self.radius,
            front_face: true,
            mat:Rc::clone(&(self.mat))
        };
        let outward_normal = (tmp - self.center) / self.radius;
        hr.set_face_normal(ray, &outward_normal);
        Some(hr)
    }
}
