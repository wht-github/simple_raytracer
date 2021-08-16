use crate::{
    ray::Ray,
    vec3::{Color, Point3, Vec3},
    material::Scatter
};
use std::rc::Rc;
// #[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Scatter>,
}
impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> HittableList {
        Self {
            objects: Vec::new(),
        }
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hr: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    hr = Some(record);
                }
                None => {}
            }
        }
        hr
    }
}
