use super::{Material, HitRecord, Vec3, Color, Ray, ScatterRecord};

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(al: Color) -> Self {
        Self { albedo: al }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);

        let sca = Ray::new(rec.p, reflected);
        let att = self.albedo;
        
        if Vec3::dot(&sca.direction(), &rec.normal) > 0. {
            Some(ScatterRecord{attenuation: att, scattered: sca})
        } else {
            None
        }
    }
}
