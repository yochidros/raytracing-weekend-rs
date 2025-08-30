use std::{f32::consts::PI, sync::Arc};

use crate::{
    camera::Camera,
    color::Color,
    hit_record::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
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
    // let r = (PI / 4.0).cos();

    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        20.0,
        80.0,
        30,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    camera.defocus_angle = 10.0;
    camera.focus_distance = 3.4;

    // world
    let mut world = HittableList::new();

    // let mat_left = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    // let mat_right = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    //
    // world.add(Box::new(Sphere::new(Vec3::new(-r, 0.0, -1.0), r, mat_left)));
    // world.add(Box::new(Sphere::new(Vec3::new(r, 0.0, -1.0), r, mat_right)));

    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        mat_center,
    )));

    let mat_left = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));

    let mat_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        mat_bubble,
    )));

    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
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
    camera.render(&world);
}

