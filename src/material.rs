use crate::{
    color::Color,
    hit_record::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, reflect, unit_vector},
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Color);
}

pub struct Lambertian {
    pub albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Color) {
        let scatter_direction = hit_record.normal_vec + random_unit_vector();

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            hit_record.normal_vec
        } else {
            scatter_direction
        };

        (Ray::new(hit_record.p, scatter_direction), self.albedo)
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Color) {
        let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal_vec);
        (Ray::new(hit_record.p, reflected), self.albedo)
    }
}
