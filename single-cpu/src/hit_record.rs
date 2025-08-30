use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal_vec: Vec3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(p: Point3, normal_vec: Vec3, t: f32, front_face: bool) -> Self {
        Self {
            p,
            normal_vec,
            material: None,
            t,
            front_face,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal_vec(&mut self, ray: &Ray, outward_normal_vec: Vec3) {
        self.front_face = ray.direction.dot(outward_normal_vec) < 0.0;
        self.normal_vec = if self.front_face {
            outward_normal_vec
        } else {
            -outward_normal_vec
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(interval.min, closest_so_far)) {
                closest_so_far = rec.t;
                return Some(rec);
            }
        }

        None
    }
}

