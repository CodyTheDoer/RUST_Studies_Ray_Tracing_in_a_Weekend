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
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal {albedo}
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray, 
        rec: HitRecord, 
    ) -> Option<(Color, Ray)> {
        let reflected = RtVec3::reflect(r_in.direction(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

// Material Defaults, considering changing to setup()
pub fn default_material() -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5)))
}

  