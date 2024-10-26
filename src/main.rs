use raytracing_in_a_weekend::camera::Camera;
use raytracing_in_a_weekend::rtvec3::{Point3, RtVec3};
use raytracing_in_a_weekend::hit::{HittableList, Sphere};
use raytracing_in_a_weekend::ray::Color;
use raytracing_in_a_weekend::material::{
    default_material_lambertian,    new_material_lambertian,    new_material_lambertian_color,  new_material_lambertian_color_float, 
    default_material_metal,         new_material_metal,         new_material_metal_color,       new_material_metal_color_float,
    default_material_dielectric,    new_material_dielectric,    new_material_dielectric_color,  new_material_dielectric_color_float,
};
use::raytracing_in_a_weekend::Interval;
use::raytracing_in_a_weekend::{random_float, random_float_range};

use std::rc::Rc;

fn main() {
    // World Parameters
    let mut world_objects: HittableList = HittableList::new();

    // Image Parameters
    let aspect_ratio: f64 = 16.0 / 9.0;                     // Output image aspect ratio
    let image_width: u32 = 1920;
    let samples_per_pixel: u32 = 500;                       // Antialiasing multiplier, rendered x times and then each pixel is averaged
    let sample_bounce_max: u32 = 50;                        // How many times a ray can bounce

    // Ray Parameters
    let ray_color: Color = Color::new_rgb(0.0, 0.0, 0.0);   // Default Color is black

    // Camera Parameters
    let fov: f64 = 20.0;                                    // field of view, smaller zooms in
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);     // Point camera is looking from
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);       // Point camera is looking at
    let vup: RtVec3 = RtVec3::new(0.0, 1.0, 0.0);           // Camera-relative "up" direction
    let defocus_angle: f64 = 0.6;                           // Variation angle of rays through each pixel
    let focus_dist: f64 = 10.0;                             // Distance from camera lookfrom point to plane of perfect focus

    // Complex Implementation Demo
    let material_ground = new_material_lambertian_color_float(0.5, 0.5, 0.5);
    world_objects.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, -1.0), 1000.0, Rc::clone(&material_ground))));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_float();
            let center: Point3 = Point3::new(a as f64 + 0.9 * random_float(), 0.2, b as f64 + 0.9 * random_float());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = new_material_lambertian_color(albedo);
                    world_objects.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&sphere_material))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(Interval::new(0.5, 1.0));
                    let fuzz = random_float_range(Interval::new(0.0, 0.5));
                    let sphere_material = new_material_metal_color(albedo, fuzz);
                    world_objects.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&sphere_material))));
                } else {
                    // glass
                    let sphere_material = new_material_dielectric(1.5);
                    world_objects.add(Rc::new(Sphere::new(center, 0.2, Rc::clone(&sphere_material))));
                }
            }
        }
    }

    let material1 = new_material_dielectric(1.5);
    world_objects.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Rc::clone(&material1))));

    let material2 = new_material_lambertian_color(Color::new_rgb(0.4, 0.2, 0.1));
    world_objects.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Rc::clone(&material2))));

    let material3 = new_material_metal_color_float(0.7, 0.6, 0.5, 0.0);
    world_objects.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Rc::clone(&material3))));

    
    // Dev Demo
/*  
    // Build Colors (albedo)
    let albedo_silver = Color::new_rgb(0.8, 0.8, 0.8);
    let albedo_gold = Color::new_rgb(0.8, 0.6, 0.2);
    let albedo_blue_gray = Color::new_rgb(0.1, 0.2, 0.5);
    let albedo_green_mountain_dew = Color::new_rgb(0.8, 0.8, 0.0);

    // Build materials, apply colors and other variables
    let material_center   = new_material_lambertian_color(albedo_blue_gray);
    let material_ground  = new_material_lambertian_color(albedo_green_mountain_dew);
    let material_left = new_material_dielectric(1.5);
    let material_bubble = new_material_dielectric(1.0 / 1.5);
    let material_right = new_material_metal_color(albedo_gold, 0.0);

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
*/

    // Start the camera, passing in world and variables
    let cam: Camera = Camera::new(
        world_objects, 
        ray_color, 
        aspect_ratio, 
        fov, 
        defocus_angle,
        focus_dist,
        lookfrom, 
        lookat, 
        vup, 
        image_width, 
        samples_per_pixel, 
        sample_bounce_max
    );

    cam.render();
}