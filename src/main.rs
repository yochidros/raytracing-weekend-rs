use std::sync::Arc;

use crate::{
    camera::Camera,
    color::Color,
    hit_record::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};

mod camera;
mod color;
mod hit_record;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    let aspect_ratio = 16f32 / 9f32;
    let image_width = 400u32;
    let mut camera = Camera::new(aspect_ratio, image_width, 80.0, 50);

    // world
    let mut world = HittableList::new();

    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        mat_center,
    )));

    let mat_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));

    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));

    // world.add(Box::new(sphere::Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    // )));
    // let base = Point3::new(0.6, 0.0, -3.0);
    // let dir = unit_vector(Vec3::new(1.0, 1.0, 0.0));
    // let shifted_sphere = base + dir * 1.0;
    // let s = sphere::Sphere::new(base, 0.8);
    // world.add(Box::new(s));
    // world.add(Box::new(sphere::Sphere::new(
    //     Point3::new(1.5, -0.3, -1.5),
    //     0.214,
    // )));
    camera.render(&mut world);
}
