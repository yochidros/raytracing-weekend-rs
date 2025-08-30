use once_cell::sync::Lazy;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

static INTENSITY: Lazy<Interval> = Lazy::new(|| Interval::new(0.0, 0.999));

#[inline]
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(pixel_color: Color) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let rbyte = (256f32 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256f32 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256f32 * INTENSITY.clamp(b)) as i32;

    eprintln!("{rbyte} {gbyte} {bbyte}");
}
