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

// Metal Logic
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

// Metal Logic
pub struct Dielectric {
    pub albedo: Color,
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(albedo: Color, refraction_index: f64) -> Self {
        Dielectric {
            albedo,
            refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray, 
        rec: HitRecord, 
    ) -> Option<(Color, Ray)> {
        let attenuation = self.albedo;
        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction: RtVec3 = RtVec3::unit_vector(&r_in.direction());
        let refracted: RtVec3 = RtVec3::refract(unit_direction, rec.normal, ri);

        let scattered = Ray::new(rec.p, refracted);
        Some((attenuation, scattered))
    }
}

// Material Defaults, considering changing to setup()
pub fn default_material() -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5)))
}

// Material: Lambertian
pub fn new_material_lambertian() -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5))) 
}

pub fn new_material_lambertian_color(color: Color) -> Rc<dyn Material> {
    Rc::new(Lambertian::new(color))
}

pub fn new_material_lambertian_color_float(r: f64, g: f64, b: f64) -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(r, g, b)))
}

// Material Metal
pub fn default_material_metal() -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(0.5, 0.5, 0.5), 0.5))
}

pub fn new_material_metal(fuzz: f64) -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(0.5, 0.5, 0.5), fuzz))
}

pub fn new_material_metal_color(color: Color, fuzz: f64) -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(color.r, color.g, color.b), fuzz))
}

pub fn new_material_metal_float(r: f64, g: f64, b: f64, fuzz: f64) -> Rc<dyn Material> {
    Rc::new(Metal::new(Color::new_rgb(r, g, b), fuzz))
}

// Material Dielectric
pub fn default_material_dielectric() -> Rc<dyn Material> { // Full refraction
    Rc::new(Dielectric::new(Color::new_rgb(1.0, 1.0, 1.0), 1.5))
}

pub fn new_material_dielectric(refraction: f64) -> Rc<dyn Material> {
    Rc::new(Dielectric::new(Color::new_rgb(1.0, 1.0, 1.0), refraction))
}

pub fn new_material_dielectric_color(color: Color, refraction: f64) -> Rc<dyn Material> {
    Rc::new(Dielectric::new(color, refraction))
}

pub fn new_material_dielectric_color_float(r: f64, g: f64, b: f64, refraction: f64) -> Rc<dyn Material> {
    Rc::new(Dielectric::new(Color::new_rgb(r, g, b), refraction))
}

  