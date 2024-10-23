use raytracing_in_a_weekend::camera::Camera;
use raytracing_in_a_weekend::rtvec3::Point3;
use raytracing_in_a_weekend::hit::{HittableList, Sphere};

use std::rc::Rc;

fn main() {
    let mut world_objects: HittableList = HittableList::new();

    let sphere = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let ground_sphere = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    world_objects.add(sphere); 
    world_objects.add(ground_sphere);

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 10;

    let cam: Camera = Camera::new(world_objects, aspect_ratio, image_width, samples_per_pixel);

    cam.render();
}