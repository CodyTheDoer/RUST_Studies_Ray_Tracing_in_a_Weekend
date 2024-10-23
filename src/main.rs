use raytracing_in_a_weekend::rtvec3::{Point3, RtVec3};
use raytracing_in_a_weekend::ray::{RayColor, Sphere};
use raytracing_in_a_weekend::build_file;

use std::rc::Rc;

fn main() {
    // Image Data
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const INFINITY: f64 = f64::INFINITY;

    let _ray_color: RayColor = RayColor::new_rgb(0.0, 0.0, 0.0);
    
    let image_width: u32 = 1600;
    let mut image_height: u32 = (image_width as f64 / ASPECT_RATIO) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    // Camera Viewport Data
    let focal_length: f64 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = RtVec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = RtVec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - RtVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Call Rc "Reference Counted" data to 
    let double_ptr = Rc::new(0.37);
    let vec3_ptr = Rc::new(RtVec3::new(1.414214, 2.718281, 1.618034));
    let sphere_ptr = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0));

    // Render Data
    let _ = build_file(image_width, image_height, camera_center, pixel_00_loc, pixel_delta_u, pixel_delta_v);
}