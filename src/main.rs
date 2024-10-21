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
            let r: f32 = (pixel_w as f32) / ((image_width - 1) as f32);
            let g: f32 = (pixel_h as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.0;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
            file.write_all(pixel_triplets.as_bytes())?;
        }
    }
    println!("Generation finished.");
    Ok(())
}
