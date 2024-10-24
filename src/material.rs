use crate::hit::HitRecord; 

use crate::ray::{Ray, Color};

use crate::rtvec3::RtVec3;

use std::rc::Rc;

pub trait Material {
    fn scatter (
        &self,
        r_in: Ray, 
        rec: HitRecord, 
        attenuation: Color, 
        scattered: Ray,
    ) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian {albedo}
    }
}

pub fn default_material() -> Rc<dyn Material> {
    Rc::new(Lambertian::new(Color::new_rgb(0.5, 0.5, 0.5)))
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: Ray, 
        rec: HitRecord, 
        attenuation: Color, 
        scattered: Ray,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + RtVec3::random_unit_vector();
        if scatter_direction < RtVec3::new(0.001, 0.001, 0.001) {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

// class lambertian : public material {
// public:
//     lambertian(const color& albedo) : albedo(albedo) {}

//     bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
//     const override {
//         auto scatter_direction = rec.normal + random_unit_vector();
//         scattered = ray(rec.p, scatter_direction);
//         attenuation = albedo;
//         return true;
//     }

// private:
//     color albedo;
// };

  