use crate::vec3::{Color, Point3, Vec3};
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}
impl Ray {
    #[inline]
    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}