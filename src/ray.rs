use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    fn new(&self, origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            dir: *direction,
            origin: *origin,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
