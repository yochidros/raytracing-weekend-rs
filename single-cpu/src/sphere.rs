use std::sync::Arc;

use crate::{
    hit_record::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0f32),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }
        let mut record = HitRecord {
            p: Point3::zero(),
            normal_vec: Point3::zero(),
            material: None,
            t: 0.0,
            front_face: false,
        };

        record.t = root;
        record.p = r.at(record.t);
        record.normal_vec = (record.p - self.center) / self.radius;
        record.material = Some(self.material.clone());

        let outward_normal_vec = (record.p - self.center) / self.radius;
        record.set_face_normal_vec(r, outward_normal_vec);
        Some(record)
    }
}

