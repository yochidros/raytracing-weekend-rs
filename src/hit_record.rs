use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        false
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal_vec: Vec3,
    pub t: f32,
    pub front_face: bool,
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
    fn hit(&self, r: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord {
            p: Point3::zero(),
            normal_vec: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(interval.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = HitRecord {
                    p: temp_record.p,
                    normal_vec: temp_record.normal_vec,
                    t: temp_record.t,
                    front_face: temp_record.front_face,
                };
            }
        }

        hit_anything
    }
}
