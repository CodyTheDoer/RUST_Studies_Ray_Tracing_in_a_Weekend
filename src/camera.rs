use crate::ray::{Ray, RayColor};
use crate::ray::{write_color_to_pixel, color};
use crate::rtvec3::{Point3, RtVec3};
use crate::hit::HittableList;
use crate::sample_square;

use std::fs::File;
use std::io::Write;

pub struct Camera{
    world: HittableList,
    ray_color: RayColor,
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
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
        ray_color: RayColor,
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
    ) -> Self {
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
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
            ray_color,
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale, 
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

    pub fn render(&self) {
        let _ = Self::build_file(&self);
    }

    fn get_ray(
        &self,
        i: u32,
        j: u32,
    ) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;
        let ray: Ray = Ray::new(ray_origin, ray_direction);
        ray
    }

    fn build_file(&self) -> std::io::Result<()> {
        // Setup
        let mut file = File::create("image.ppm")?;
        file.write_all(b"P3\n")?;
        let img_dim = format!("{:?} {:?}\n", self.image_width, self.image_height);
        file.write_all(img_dim.as_bytes())?;
        file.write_all(b"255\n")?;
        
        // Pixel Algo
        for pixel_h in 0..self.image_height {
            println!("Scanline's remaining: {:?} ", (self.image_height - pixel_h));
            for pixel_w in 0..self.image_width {
                let mut pixel_color_samples: Vec<RtVec3> = Vec::new();
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(pixel_w, pixel_h);
                    let pixel_color = color(ray, self.world.clone());
                    pixel_color_samples.push(pixel_color);
                }
                let mut average_pixel_color_sum: RtVec3 = RtVec3::new(0.0, 0.0, 0.0);
                for sample in pixel_color_samples {
                    average_pixel_color_sum = average_pixel_color_sum + sample;
                }
                let average_pixel_color = average_pixel_color_sum * self.pixel_samples_scale;
                write_color_to_pixel(average_pixel_color, &mut file)?;
            }
        }
        println!("Generation finished.");
        Ok(())
    }
}