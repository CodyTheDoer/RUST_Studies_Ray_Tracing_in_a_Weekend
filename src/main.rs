use raytracing_in_a_weekend::{RtVec3, Ray, RayColor, Point3};
use raytracing_in_a_weekend::{write_color, ray_color};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Image Data
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let ray_color: RayColor = RayColor::new_rgb(0.0, 0.0, 0.0);
    
    let image_width: u32 = 1600;
    let mut image_height: u32 = (image_width as f32 / ASPECT_RATIO) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    // Camera Viewport Data
    let focal_length: f32 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = RtVec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = RtVec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - RtVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    let copy_of_pixel_00_loc = &pixel_00_loc;
    println!("{:?}", copy_of_pixel_00_loc);

    // Render Data
    let _ = build_file(image_width, image_height, camera_center, pixel_00_loc, pixel_delta_u, pixel_delta_v);

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
    camera_center: RtVec3,
    pixel_00_loc: RtVec3,
    pixel_delta_u: RtVec3,
    pixel_delta_v: RtVec3,
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
            let pixel_center = pixel_00_loc + pixel_w as f32 * pixel_delta_u + pixel_h as f32 * pixel_delta_v;
            // let pixel_center = pixel_00_loc + (pixel_h as f32 * image_height as f32) + (pixel_w as f32* image_width as f32);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(ray);
            write_color(pixel_color, &mut file)?;
        }
    }
    println!("Generation finished.");
    Ok(())
}
