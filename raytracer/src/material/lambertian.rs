use super::{Color, HitRecord, Material, Ray, ScatterRecord, Vec3};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(al: Color) -> Self {
        Self { albedo: al }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo;

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
