use crate::{
    color::{write_color, Color},
    ray::Ray,
    vec3::{unit_vector, Point3, Vec3},
};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: Point3, radius: f32, ray: Ray) -> f32 {
    // object direction from ray origin to sphere center
    let oc = center - ray.origin;
    let a = ray.direction.length_squared();
    let h = ray.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    // let a = ray.direction.dot(ray.direction);
    // let b = -2.0 * ray.direction.dot(oc);
    // let c = oc.dot(oc) - radius * radius;
    // let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (h - discriminant.sqrt()) / a
    // (-b - discriminant.sqrt()) / (2.0 * a)
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1f32), 0.5, ray);
    if t > 0.0 {
        let n = unit_vector(ray.at(t) - Point3::new(0.0, 0.0, -1f32));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = unit_vector(ray.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16f32 / 9f32;
    let image_width = 400u32;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let focal_length = 1f32;
    //  viewport_width: 3.5555556, viewport_height: 2
    let viewport_height = 2f32;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::zero();

    // edges
    let viewport_u = Vec3::new(viewport_width, 0f32, 0f32);
    let viewport_v = Vec3::new(0f32, -viewport_height, 0f32);

    // delta vector from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // カメラを中心とする場合にviewportの左上の位置
    // shift left by half viewport_u and up by half viewport_v
    let viewport_upper_left =
        camera_center - Vec3::new(0f32, 0f32, focal_length) - viewport_u / 2f32 - viewport_v / 2f32;

    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    eprintln!("P3");
    eprintln!("{} {}", image_width, image_height);
    eprintln!("255"); // colors range from 0 to 255

    for j in 0..image_height {
        print!("\rScanning line {}/{}", j + 1, image_height);
        for i in 0..image_width {
            let pixel_center =
                pixel00_location + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            // AB direction is from A to B = AB = B - A
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };
            let pixel_color = ray_color(ray);
            write_color(pixel_color);
        }
    }
    print!("\rDone.                           \n");
}
