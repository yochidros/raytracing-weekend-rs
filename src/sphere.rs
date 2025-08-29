use crate::{
    hit_record::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}
impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0f32),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.p = r.at(record.t);
        record.normal_vec = (record.p - self.center) / self.radius;

        let outward_normal_vec = (record.p - self.center) / self.radius;
        record.set_face_normal_vec(r, outward_normal_vec);

        true
    }
}
