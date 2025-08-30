use crate::{
    color::Color,
    hit_record::HitRecord,
    ray::Ray,
    utils::f32_random,
    vec3::{random_unit_vector, reflect, refract, unit_vector},
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
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1f32 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Color) {
        let mut reflected = reflect(unit_vector(ray_in.direction), hit_record.normal_vec);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        (Ray::new(hit_record.p, reflected), self.albedo)
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Ray, Color) {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(ray_in.direction);
        let cos_theta = (-unit_direction).dot(hit_record.normal_vec).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > f32_random() {
                reflect(unit_direction, hit_record.normal_vec)
            } else {
                refract(unit_direction, hit_record.normal_vec, refraction_ratio)
            };

        (Ray::new(hit_record.p, direction), attenuation)
    }
}
