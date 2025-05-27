use std::io::Write;

use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: &Colour) {
    let r: i32 = (255.999 * pixel_colour.x()) as i32;
    let g: i32 = (255.999 * pixel_colour.y()) as i32;
    let b: i32 = (255.999 * pixel_colour.z()) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing colour");
}
