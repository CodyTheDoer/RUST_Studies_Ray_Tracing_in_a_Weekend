use raytracing_in_a_weekend::{RtVec3, Ray};
use raytracing_in_a_weekend::write_color;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Image Data
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    
    let image_width: u32 = 1600;
    let mut image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    println!("{:?} {:?} ", viewport_height, viewport_width);

    // Render Data
    let _ = build_file(image_width, image_height);

    // Define an origin and a direction
    let origin = RtVec3::new(1.0, 2.0, 3.0);
    let direction = RtVec3::new(4.0, 5.0, 6.0);

    // Create a new ray
    let ray = Ray::new(origin, direction);

    // Find the point along the ray at t = 2.0
    let point = ray.at(2.0);
    println!("Point at t=2.0: {:?}", point);
}

fn build_file(
    image_width: u32,
    image_height: u32,
) -> std::io::Result<()> {
    // Setup
    let mut file = File::create("image.ppm")?;
    file.write_all(b"P3\n")?;
    let img_dim = format!("{:?} {:?}\n", image_width, image_height);
    file.write_all(img_dim.as_bytes())?;
    file.write_all(b"255\n")?;

    // Pixel Algo
    for pixel_h in 0..image_height {
        println!("Scanline's remaining: {:?} ", (image_height - pixel_h));
        for pixel_w in 0..image_width {
            let pixel_color = RtVec3::new(pixel_w as f32 / image_width as f32, pixel_h as f32 / image_height as f32, 0.0);
            write_color(pixel_color, &mut file)?;
        }
    }
    println!("Generation finished.");
    Ok(())
}
