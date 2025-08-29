use crate::{
    color::{write_color, Color},
    hit_record::{self, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vec3::{unit_vector, Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,

    image_height: u32,
    center: Point3,
    pixel00_location: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Point3::zero(),
            pixel00_location: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Point3::zero(),
            pixel00_location: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }

    pub fn render(&mut self, world: &mut HittableList) {
        self.initialize();

        eprintln!("P3");
        eprintln!("{} {}", self.image_width, self.image_height);
        eprintln!("255"); // colors range from 0 to 255

        for j in 0..self.image_height {
            print!("\rScanning line {}/{}", j + 1, self.image_height);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_location
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                // AB direction is from A to B = AB = B - A
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };
                let pixel_color = self.ray_color(ray, world);
                write_color(pixel_color);
            }
        }
        print!("\rDone.                           \n");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.center = Point3::zero();

        let focal_length = 1f32;
        //  viewport_width: 3.5555556, viewport_height: 2
        let viewport_height = 2f32;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // edges
        let viewport_u = Vec3::new(viewport_width, 0f32, 0f32);
        let viewport_v = Vec3::new(0f32, -viewport_height, 0f32);

        // delta vector from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // カメラを中心とする場合にviewportの左上の位置
        // shift left by half viewport_u and up by half viewport_v
        let viewport_upper_left = self.center
            - Vec3::new(0f32, 0f32, focal_length)
            - viewport_u / 2f32
            - viewport_v / 2f32;

        self.pixel00_location =
            viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray: Ray, world: &mut HittableList) -> Color {
        let mut record = hit_record::HitRecord {
            p: Point3::zero(),
            normal_vec: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };
        if world.hit(&ray, Interval::new(0.0, f32::INFINITY), &mut record) {
            return 0.5
                * Color::new(
                    record.normal_vec.x + 1.0,
                    record.normal_vec.y + 1.0,
                    record.normal_vec.z + 1.0,
                );
        }

        let unit_direction = unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
