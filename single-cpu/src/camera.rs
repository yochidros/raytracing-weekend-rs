use crate::{
    color::{write_color, Color},
    hit_record::{self, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    utils::f32_random,
    vec3::{
        random_in_unit_disk, random_on_hemisphere, random_unit_vector, unit_vector, Point3, Vec3,
    },
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: f32,
    pub vfov: f32, // angle
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_distance: f32,

    image_height: u32,
    center: Point3,
    pixel00_location: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32,
    max_depth: u32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        vfov: f32,
        samples_per_pixel: f32,
        max_depth: u32,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle: 0.0,
            focus_distance: 10.0,
            image_height: 0,
            center: Point3::zero(),
            pixel00_location: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 0.0,
            max_depth,
            v: Vec3::zero(),
            u: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10.0,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            image_height: 0,
            center: Point3::zero(),
            pixel00_location: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 0.0,
            max_depth: 10,
            v: Vec3::zero(),
            u: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        eprintln!("P3");
        eprintln!("{} {}", self.image_width, self.image_height);
        eprintln!("255"); // colors range from 0 to 255

        for j in 0..self.image_height {
            print!("\rScanning line {}/{}", j + 1, self.image_height);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel as u32 {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(ray, self.max_depth, world);
                }
                write_color(pixel_color * self.pixel_samples_scale);
            }
        }
        print!("\rDone.                           \n");
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_location
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
    fn sample_square(&self) -> Vec3 {
        Vec3::new(f32_random() - 0.5, f32_random(), 0.0)
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.center = self.lookfrom;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel;

        // let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta / 2f32).tan();
        //  viewport_width: 3.5555556, viewport_height: 2
        let viewport_height = 2f32 * h * self.focus_distance;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(self.vup.cross(self.w));
        self.v = self.w.cross(self.u);

        // edges
        let viewport_u = viewport_width * self.u; // Vec3::new(viewport_width, 0f32, 0f32);
        let viewport_v = viewport_height * -self.v; //Vec3::new(0f32, -viewport_height, 0f32);

        // delta vector from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // カメラを中心とする場合にviewportの左上の位置
        // shift left by half viewport_u and up by half viewport_v
        let viewport_upper_left =
            self.center - (self.focus_distance * self.w) - viewport_u / 2f32 - viewport_v / 2f32;

        self.pixel00_location =
            viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, ray: Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::zero();
        }
        // let mut record = hit_record::HitRecord {
        //     p: Point3::zero(),
        //     normal_vec: Vec3::zero(),
        //     t: 0.0,
        //     front_face: false,
        // };
        if let Some(record) = world.hit(&ray, Interval::new(0.0001, f32::INFINITY)) {
            if let Some(material) = &record.material {
                let (scattered, attenuation) = material.scatter(&ray, &record);
                return attenuation * self.ray_color(scattered, depth - 1, world);
            }
            return Color::zero();
            // let direction = record.normal_vec + random_unit_vector();
            // // let direction = random_on_hemisphere(record.normal_vec);
            // return 0.5 * self.ray_color(Ray::new(record.p, direction), depth - 1, world);
        }

        let unit_direction = unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

