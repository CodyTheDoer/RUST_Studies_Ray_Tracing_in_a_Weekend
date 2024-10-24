use raytracing_in_a_weekend::camera::Camera;
use raytracing_in_a_weekend::rtvec3::Point3;
use raytracing_in_a_weekend::hit::{HittableList, Sphere};
use raytracing_in_a_weekend::ray::Color;
use raytracing_in_a_weekend::material::{default_material, default_material_lambertian, default_material_metal, new_material_lambertian, new_material_metal};

use std::rc::Rc;

fn main() {
    let mut world_objects: HittableList = HittableList::new();
    let ray_color: Color = Color::new_rgb(0.0, 0.0, 0.0);
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let sample_ray_bounce_max: u32 = 10;

    let material_ground  = new_material_lambertian(0.8, 0.8, 0.0);
    let material_center   = new_material_lambertian(0.1, 0.2, 0.5);
    let material_left = new_material_metal(0.8, 0.8, 0.8);
    let material_right = new_material_metal(0.8, 0.6, 0.2);

    let sphere_ground = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::clone(&material_ground)));
    let sphere_center = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Rc::clone(&material_center)));
    let sphere_left = Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::clone(&material_left)));
    let sphere_right = Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::clone(&material_right)));
    
    world_objects.add(sphere_ground);
    world_objects.add(sphere_center);
    world_objects.add(sphere_left);
    world_objects.add(sphere_right);

    let cam: Camera = Camera::new(world_objects, ray_color, aspect_ratio, image_width, samples_per_pixel, sample_ray_bounce_max);

    cam.render();
}