use raytracing_in_a_weekend::RtVec3;
use raytracing_in_a_weekend::write_color;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Image Data
    let image_width: u32 = 1000;
    let image_height: u32 = 1000;

    // Render Data
    let _ = build_file(image_width, image_height);
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
