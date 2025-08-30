use crate::{camera::Camera, hit_record::HittableList, vec3::Point3};

mod camera;
mod color;
mod hit_record;
mod interval;
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
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));
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
