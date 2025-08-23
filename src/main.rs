use crate::color::write_color;

mod color;
mod vec3;

fn main() {
    let image_width = 256u32;
    let image_height = 256u32;

    eprintln!("P3");
    eprintln!("{} {}", image_width, image_height);
    eprintln!("255"); // colors range from 0 to 255

    for j in 0..image_height {
        print!("\rScanning line {}/{}", j + 1, image_height);
        for i in 0..image_width {
            let pixel_color = color::Color::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0f32,
            );
            write_color(pixel_color);
        }
    }
    print!("\rDone.                           \n");
}
