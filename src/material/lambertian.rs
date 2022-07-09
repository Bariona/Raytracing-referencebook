use super::{Material, HitRecord, Vec3, Color, Ray, ScatterRecord};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(al: Color) -> Self {
        Self { albedo: al }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let sca = Ray::new(rec.p, scatter_direction);
        let att = self.albedo;

        Some(ScatterRecord{attenuation: att, scattered: sca})
    }
}