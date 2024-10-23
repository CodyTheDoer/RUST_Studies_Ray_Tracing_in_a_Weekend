pub mod hit; 
pub mod ray;
pub mod rtvec3; 

use std::fs::File;
use std::io::Write;

pub use hit::{Hittable, HittableList, HitRecord};
pub use ray::{Ray, RayColor};
pub use ray::{write_color_to_pixel, color};
pub use rtvec3::{Point3, RtVec3};

pub fn degrees_to_radians(
    degrees: f64,
) -> f64 {
    const PI: f64 = 3.14159265358979323846264338327950288_f64;
    let res = degrees * PI / 180.0;
    res
}

pub fn build_file(
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

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Custom New interval
    pub fn new(min: f64, max: f64) -> Interval {
        Interval {min, max}
    }

    pub fn new_empty(min: f64, max: f64) -> Interval {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn new_universe(min: f64, max: f64) -> Interval {
        Interval {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}