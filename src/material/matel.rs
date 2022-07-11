use super::{Material, HitRecord, Vec3, Color, Ray, ScatterRecord};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(al: Color, fuz: f64) -> Self {
        Self { 
            albedo: al, 
            fuzz: if fuz < 1. { fuz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);

        let sca = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let att = self.albedo;
        
        if Vec3::dot(&sca.direction(), &rec.normal) > 0. {
            Some(ScatterRecord{attenuation: att, scattered: sca})
        } else {
            None
        }
    }
}