use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = (pixel_color.x * 255.999) as i32;
    let g = (pixel_color.y * 255.999) as i32;
    let b = (pixel_color.z * 255.999) as i32;
    eprintln!("{r} {g} {b}");
}
