use crate::hit::HitRecord; 

use crate::ray::{Ray, Color};

use crate::rtvec3::RtVec3;

use std::rc::Rc;

pub trait Material {
    fn scatter (
        &self,
        r_in: Ray, 
        rec: HitRecord, 
    ) -> Option<(Color, Ray)>;
}

// Lambertian Logic (Perfect diffusion)
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray, 
        rec: HitRecord, 
    ) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + RtVec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p, scatter_direction);

        Some((attenuation, scattered))
    }
}

// Metal Logic (Perfect Reflection, I think)
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray, 
        rec: HitRecord, 
    ) -> Option<(Color, Ray)> {
        // Reflect Ray
        let mut reflected = RtVec3::reflect(r_in.direction(), rec.normal);
        // Apply fuzz 
        reflected = reflected + self.fuzz * RtVec3::random_unit_vector();
        
        // Create the scattered Ray
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        // Only scatter if the dot product of the scattered direction and normal is positive
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

// Material Defaults, considering changing to setup()
pub fn default_material() -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5)))
}

pub fn default_material_lambertian() -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5))) 
}

pub fn new_material_lambertian(color: Color) -> Rc<dyn Material> {
    Rc::new(Lambertian::new(color))
}

pub fn new_material_lambertian_float(r: f64, g: f64, b: f64) -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(r, g, b)))
}

pub fn default_material_metal() -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(0.5, 0.5, 0.5), 0.5))
}

pub fn new_material_metal(color: Color, fuzz: f64) -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(color.r, color.g, color.b), fuzz))
}

pub fn new_material_metal_float(r: f64, g: f64, b: f64, fuzz: f64) -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(r, g, b), fuzz))
}

  