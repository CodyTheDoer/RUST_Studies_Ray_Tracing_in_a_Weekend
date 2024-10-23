use raytracing_in_a_weekend::camera::Camera;
use raytracing_in_a_weekend::rtvec3::Point3;
use raytracing_in_a_weekend::hit::{HittableList, Sphere};
use raytracing_in_a_weekend::RayColor;

use std::rc::Rc;

fn main() {
    let mut world_objects: HittableList = HittableList::new();
    let ray_color: RayColor = RayColor::new_rgb(0.0, 0.0, 0.0);
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let sample_ray_bounce_max: u32 = 10;

    let sphere = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let ground_sphere = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    world_objects.add(sphere);
    world_objects.add(ground_sphere);

    let cam: Camera = Camera::new(world_objects, ray_color, aspect_ratio, image_width, samples_per_pixel, sample_ray_bounce_max);

    cam.render();
}