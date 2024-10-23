use crate::ray::{Ray, RayColor};
use crate::ray::{write_color_to_pixel, color};
use crate::rtvec3::{Point3, RtVec3};
use crate::hit::HittableList;

use std::fs::File;
use std::io::Write;

pub struct Camera{
    world: HittableList,
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    camera_center: Point3,
    viewport_u: RtVec3,
    viewport_v: RtVec3,
    pixel_delta_u: RtVec3,
    pixel_delta_v: RtVec3,
    viewport_upper_left: RtVec3,
    pixel_00_loc: RtVec3,
}

impl Camera {
    pub fn new(
        world: HittableList,
        aspect_ratio: f64,
        image_width: u32,
    ) -> Self {
        let _ray_color: RayColor = RayColor::new_rgb(0.0, 0.0, 0.0);

        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
    
        // Camera Viewport Data
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64  = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);
    
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u: RtVec3 = RtVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: RtVec3 = RtVec3::new(0.0, -viewport_height, 0.0);
    
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u: RtVec3 = viewport_u / image_width as f64;
        let pixel_delta_v: RtVec3 = viewport_v / image_height as f64;
    
        // Calculate the location of the upper left pixel.
        let viewport_upper_left: RtVec3 = camera_center - RtVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc: RtVec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
        Camera{
            world,
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            viewport_width,
            viewport_height,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel_00_loc,
        }
    }

    pub fn render(
        &self,
    ) {
        let _ = Self::build_file(&self.world, self.image_width, self.image_height, self.camera_center, self.pixel_00_loc, self.pixel_delta_u, self.pixel_delta_v);
    }

    fn build_file(
        world: &HittableList,
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
                let pixel_center = pixel_00_loc + pixel_w as f64 * pixel_delta_u + pixel_h as f64 * pixel_delta_v;
                // let pixel_center = pixel_00_loc + (pixel_h as f64 * image_height as f64) + (pixel_w as f64* image_width as f64);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_direction);
    
                let pixel_color = color(ray, world.clone());
                write_color_to_pixel(pixel_color, &mut file)?;
            }
        }
        println!("Generation finished.");
        Ok(())
    }
}