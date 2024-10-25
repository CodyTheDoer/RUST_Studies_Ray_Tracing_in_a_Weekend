use raytracing_in_a_weekend::camera::Camera;
use raytracing_in_a_weekend::rtvec3::Point3;
use raytracing_in_a_weekend::hit::{HittableList, Sphere};
use raytracing_in_a_weekend::ray::Color;
use raytracing_in_a_weekend::material::{
    default_material_lambertian,    new_material_lambertian,    new_material_lambertian_color,  new_material_lambertian_color_float, 
    default_material_metal,         new_material_metal,         new_material_metal_color,       new_material_metal_color_float,
    default_material_dielectric,    new_material_dielectric,    new_material_dielectric_color,  new_material_dielectric_color_float,
};

use std::rc::Rc;

fn main() {
    let mut world_objects: HittableList = HittableList::new();
    let ray_color: Color = Color::new_rgb(0.0, 0.0, 0.0);
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let samples_per_pixel: u32 = 100;
    let sample_ray_bounce_max: u32 = 10;
    let fov: f64 = 50.0;

    // Build Colors
    let albedo_silver = Color::new_rgb(0.8, 0.8, 0.8);
    let albedo_gold = Color::new_rgb(0.8, 0.6, 0.2);
    let albedo_blue_gray = Color::new_rgb(0.1, 0.2, 0.5);
    let albedo_green_mountain_dew = Color::new_rgb(0.8, 0.8, 0.0);

    // Build materials, apply colors and other variables
    let material_center   = new_material_lambertian_color(albedo_blue_gray);
    let material_ground  = new_material_lambertian_color(albedo_green_mountain_dew);
    let material_left = new_material_dielectric(1.5);
    let material_bubble = new_material_dielectric(1.0 / 1.5);
    let material_right = new_material_metal_color(albedo_gold, 1.0);

    // Build spheres and assign materials
    let sphere_ground = Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::clone(&material_ground)));
    let sphere_center = Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Rc::clone(&material_center)));
    let sphere_left = Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::clone(&material_left)));
    let sphere_bubble = Rc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, Rc::clone(&material_bubble)));
    let sphere_right = Rc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::clone(&material_right)));
    
    // Add the objects to the world
    world_objects.add(sphere_ground);
    world_objects.add(sphere_center);
    world_objects.add(sphere_left);
    world_objects.add(sphere_bubble);
    world_objects.add(sphere_right);

    // Start the camera, passing in world and variables
    let cam: Camera = Camera::new(world_objects, ray_color, aspect_ratio, fov, image_width, samples_per_pixel, sample_ray_bounce_max);

    cam.render();
}