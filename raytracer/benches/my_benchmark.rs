#![allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use ndarray::Array3;

use image::{DynamicImage, GenericImageView, Pixel};
// use raytracer::fibonacci;
use rand::thread_rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::{
    f64::INFINITY,
    fs::File,
    process::exit,
    sync::{mpsc, Arc},
    thread,
};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lfrom: Point3,
        lat: Point3,
        vup: Vec3,         // view up vector
        vfov: f64,         // 视野范围 [0, 180']
        aspect_ratio: f64, // 比例
        aperture: f64,     // 光圈大小
        focus_dist: f64,   // object distance
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        // let focal_length = 1.;

        let w = (lfrom - lat).unit_vector();
        let u = Vec3::cross(&vup, w).unit_vector();
        let v = Vec3::cross(&w, u);

        let ori = lfrom;
        let hori = focus_dist * viewport_width * u;
        let vert = focus_dist * viewport_height * v;
        let llc = ori - hori / 2. - vert / 2. - focus_dist * w;

        Camera {
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: llc,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y(); // defocus的偏移量

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            tm: random_range(self.time0, self.time1),
        }
    }
}

pub fn degree_to_radians(degree: f64) -> f64 {
    degree * PI / 180.
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

// pub fn random_int(min: isize, max: isize) -> isize {
//     // [min, max]
//     thread_rng().gen_range(min..=max)
// }

pub fn random_range(min: f64, max: f64) -> f64 {
    // [min, max)
    min + (max - min) * random_double()
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64, // Ray 产生的时间
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }
}

use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

const EPS: f64 = 1e-8;

impl Vec3 {
    pub fn new(_x: f64, _y: f64, _z: f64) -> Self {
        Self {
            x: _x,
            y: _y,
            z: _z,
        }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn reflect(&self, n: &Vec3) -> Self {
        *self - 2. * Vec3::dot(self, n) * (*n)
    }
    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = -Vec3::dot(self, n).min(1.);
        let r_out_perp = etai_over_etat * (*self + cos_theta * (*n));
        let r_out_parallel = -(1. - r_out_perp.len_square()).abs().sqrt() * (*n);
        r_out_perp + r_out_parallel
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -self.x * rhs.z + self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
    pub fn len_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f64 {
        self.len_square().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3 {
        *self / (&self).len()
    }

    // ---- random ----
    pub fn random() -> Self {
        Self {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: random_range(min, max),
            y: random_range(min, max),
            z: random_range(min, max),
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        Vec3::random_range(-1., 1.).unit_vector() * random_double()
    }
    pub fn random_in_hemishpere(normal: &Vec3) -> Self {
        let p = Vec3::random_in_unit_sphere();
        if Vec3::dot(&p, normal) > 0. {
            p
        } else {
            -p
        }
    }
    pub fn random_unit_vector() -> Self {
        Vec3::random_range(-1., 1.).unit_vector()
    }
    pub fn random_in_unit_disk() -> Self {
        Vec3::new(random_range(-1., 1.), random_range(-1., 1.), 0.).unit_vector() * random_double()
    }
}

impl Index<u32> for Vec3 {
    type Output = f64;

    fn index(&self, idx: u32) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}
impl IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, idx: u32) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// &mut
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct AABB {
    pub mini: Point3,
    pub maxi: Point3,
}

impl AABB {
    pub fn new(mini: Point3, maxi: Point3) -> Self {
        AABB { mini, maxi }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;
        for a in 0..3 {
            let invD = 1.0 / r.direction()[a];
            let mut t0 = invD * (self.mini[a] - r.origin()[a]);
            let mut t1 = invD * (self.maxi[a] - r.origin()[a]);

            if invD < 0. {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = tmin.max(t0);
            tmax = tmax.min(t1);

            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Point3::new(
        box0.mini[0].min(box1.mini[0]),
        box0.mini[1].min(box1.mini[1]),
        box0.mini[2].min(box1.mini[2]),
    );
    let big = Point3::new(
        box0.maxi[0].max(box1.maxi[0]),
        box0.maxi[1].max(box1.maxi[1]),
        box0.maxi[2].max(box1.maxi[2]),
    );
    AABB {
        mini: small,
        maxi: big,
    }
}

pub struct BvhNode {
    left: Arc<dyn Hittable>, // 指向 Hittable List
    right: Arc<dyn Hittable>,
    box_aabb: AABB,
}

impl BvhNode {
    pub fn new(list: HittableList, time0: f64, time1: f64) -> Self {
        // println!("length = {}", list.objects.len());
        Self::new_from_vec(list.objects, time0, time1)
    }
    pub fn new_node_macro(
        left: Arc<dyn Hittable>,
        right: Arc<dyn Hittable>,
        time0: f64,
        time1: f64,
    ) -> Self {
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        let box_aabb = surrounding_box(box_left, box_right);

        Self {
            left,
            right,
            box_aabb,
        }
    }
    pub fn new_node(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, box_aabb: AABB) -> Self {
        Self {
            left,
            right,
            box_aabb,
        }
    }

    // obj: has moved its ownership !!!
    // to-do: remove "start", "end"
    pub fn new_from_vec(mut obj: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let axis = thread_rng().gen_range(0..=2);
        let span = obj.len();

        let left;
        let right;

        let compare = |x: &Arc<dyn Hittable>, y: &Arc<dyn Hittable>| {
            f64::partial_cmp(
                &x.bounding_box(0., 0.).unwrap().mini[axis],
                &y.bounding_box(0., 0.).unwrap().mini[axis],
            )
            .unwrap()
        };

        // println!("{}", span);
        // for item in obj {
        //     print!("{} ", item);
        // }
        // println!("");

        if span == 0 {
            panic!("src_objects are empty!");
        } else if span == 1 {
            let obj0 = obj.pop().unwrap();
            left = obj0;
            right = left.clone();
        } else if span == 2 {
            let obj0 = obj.pop().unwrap();
            let obj1 = obj.pop().unwrap();
            match compare(&obj0, &obj1) {
                Ordering::Less => {
                    left = obj0;
                    right = obj1;
                }
                _ => {
                    left = obj1;
                    right = obj0;
                }
            }
        } else {
            obj.sort_unstable_by(compare);

            let mut obj_left = obj;
            let obj_rigt = obj_left.split_off(span / 2);

            // let mut flag = 0;
            // if span > 390 {
            //     println!("@");
            //     flag = 1;
            // }
            // println!("{}", flag);
            left = Arc::new(BvhNode::new_from_vec(obj_left, time0, time1));
            right = Arc::new(BvhNode::new_from_vec(obj_rigt, time0, time1));
            // if flag == 1 || span > 390 {
            //     println!("%");
            // }
        }
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        let box_cur = surrounding_box(box_left, box_right);

        // if span > 390 {
        //     println!("{} {:?}", span, box_cur);
        //     exit(0);
        // }

        Self::new_node(left, right, box_cur)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.box_aabb)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.box_aabb.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(
            r,
            t_min,
            match &hit_left {
                None => t_max,
                Some(rec) => rec.t,
            },
        );

        // 注意这里应该能return就先return hit_right 而不是hit_left
        match hit_right {
            Some(_) => hit_right,
            None => hit_left,
        }
    }
}

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self { ir: index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r + (1. - r) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1., 1., 1.); // those kind of material absorbs nothing !

        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        }; // 折射率之比(要判断光线是在光密还是在光疏部分)

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let judnot = refraction_ratio * sin_theta > 1.; // 直接是全反射的情况

        let dir: Vec3;
        if judnot || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() {
            // 比较二者的光强大小决定选哪条射线
            dir = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            dir = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        // let scattered = Ray::new(rec.p, dir, r_in.time());

        Some(ScatterRecord {
            attenuation,
            is_specular: true,
            specular_ray: Ray::new(rec.p, dir, r_in.time()),
            pdf_ptr: None,
        })
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(c: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            emit: SolidColor::new(c.x, c.y, c.z),
        }
    }
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Option<Color> {
        self.emit.value(u, v, p) // 其实本质是直接返回一个solidcolor的颜色
    }
}

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new_color(c: Color) -> Isotropic<SolidColor> {
        Isotropic {
            albedo: SolidColor::new(c.x, c.y, c.z),
        }
    }
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p).unwrap();
        Some(ScatterRecord {
            attenuation,
            is_specular: true,
            specular_ray: scattered,
            pdf_ptr: None,
        })
    }
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T, // albedo 为实现了Texture的一个泛型
}

impl<T: Texture> Lambertian<T> {
    pub fn new_texture(albedo: T) -> Self {
        Self { albedo }
    }
    pub fn new(al: Color) -> Lambertian<SolidColor> {
        Lambertian {
            albedo: SolidColor::new(al.x, al.y, al.z),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            is_specular: false,
            specular_ray: Ray::default(),
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p).unwrap(),
            pdf_ptr: Some(CosPDF::new(&rec.normal)),
        })
    }
    fn scatter_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> Option<f64> {
        let cosine = Vec3::dot(&rec.normal, &scattered.direction().unit_vector());
        let consine = cosine.max(0.);

        Some(consine / PI)
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color, // 材质本身的反射率
    pub fuzz: f64,     // 带有哑光效果, fuzz=0表示材质表面为理想金属, 即反射角严格等于入射角
                       // fuzz越大, 反射角和入射角的差异会越大
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
        Some(ScatterRecord {
            attenuation: self.albedo,
            is_specular: true,
            specular_ray: Ray::new(
                rec.p,
                reflected + self.fuzz * Vec3::random_in_unit_sphere(),
                0.,
            ),
            pdf_ptr: None,
        })
    }
}

pub struct ScatterRecord {
    // 1 - attenuation := 光线的被吸收量, attenuation在某种意义上和材料的albedo(反射率)等价
    // 理解: 物体呈现颜色(比如红色) 不是因为本身发光, 而是善于吸收其他颜色光, 而不怎么吸收红光
    //       所以红颜色材质for example: attenuation = (0.8, 0.3, 0.3), i.e. 吸收70%的G和B, 仅吸收20%的R
    pub attenuation: Color,
    pub is_specular: bool,
    pub specular_ray: Ray,
    pub pdf_ptr: Option<CosPDF>,
}

pub struct ONB {
    // 一组正交基
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    // pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
    //     a * self.axis[0] + b * self.axis[1] + c * self.axis[2]
    // }
    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x() * self.axis[0] + a.y() * self.axis[1] + a.z() * self.axis[2]
    }
    pub fn build(n: &Vec3) -> Self {
        let mut axis = [Vec3::default(); 3];
        axis[2] = n.unit_vector();
        let a = if axis[2].x().abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        axis[1] = Vec3::cross(&axis[2], a).unit_vector();
        axis[0] = Vec3::cross(&axis[2], axis[1]);
        Self { axis }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    fn scatter_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> Option<f64> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Option<Color> {
        Some(Color::new(0., 0., 0.))
    }
}

pub struct Cube {
    box_min: Point3,
    box_max: Point3,
    side: HittableList,
}

impl Cube {
    pub fn new<T>(p0: Point3, p1: Point3, ptr: T) -> Self
    where
        T: 'static + Clone + Material,
    {
        let mut side = HittableList::default();

        side.objects.push(Arc::new(Rectanglexy::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        side.objects.push(Arc::new(Rectanglexy::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        side.objects.push(Arc::new(Rectanglexz::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        side.objects.push(Arc::new(Rectanglexz::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        side.objects.push(Arc::new(Rectangleyz::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        side.objects.push(Arc::new(Rectangleyz::new(
            p0.y, p1.y, p0.z, p1.z, p0.x, ptr,
        )));

        Self {
            box_min: p0,
            box_max: p1,
            side,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.side.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let output_box = AABB::new(self.box_min, self.box_max);
        Some(output_box)
    }
}

pub struct ConstantMedium<H: Hittable, M: Material> {
    // 带有smoke效果
    boundary: H,
    phase: M,
    neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn new(
        boundary: H,
        density: f64,
        color: Color,
    ) -> ConstantMedium<H, Isotropic<SolidColor>> {
        ConstantMedium {
            boundary,
            phase: Isotropic::<SolidColor>::new_color(color),
            neg_inv_density: -1. / density,
        }
    }
    pub fn new_tx(boundary: H, density: f64, texture: T) -> Self {
        ConstantMedium {
            boundary,
            phase: Isotropic::new(texture),
            neg_inv_density: -1. / density,
        }
    }
}

impl<H: Hittable, M: Material> Hittable for ConstantMedium<H, M> {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enableDebug = false;
        let debugging = enableDebug && random_double() < 0.00001;

        let rec1 = self.boundary.hit(r, -INFINITY, INFINITY);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1.unwrap();

        let rec2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
        if rec2.is_none() {
            return None;
        }
        let mut rec2 = rec2.unwrap();

        if debugging {
            println!("t_min = {} t_max = {}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = r.direction().len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (random_double().ln());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        Some(HitRecord::new(
            t,
            p,
            Vec3::new(1., 0., 0.),
            true,
            &self.phase,
            0.,
            0.,
        ))
    }
}

pub struct MoveSphere<M: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat: M,
    // pub hit: HitRecord,
}

impl<M: Material> MoveSphere<M> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: M,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            time0,
            time1,
            mat,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MoveSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().len_square();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.len_square() - self.radius * self.radius;

        let discrim = half_b.powi(2) - a * c;
        if discrim < 0. {
            return None;
        }

        let sqrtd = discrim.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord::new(
            root,
            r.at(root),
            Vec3::default(),
            bool::default(),
            &self.mat,
            0.,
            0.,
        );
        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let cub = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(self.center(time0) - cub, self.center(time0) + cub);
        let box1 = AABB::new(self.center(time1) - cub, self.center(time1) + cub);

        Some(surrounding_box(box0, box1))
    }
}

pub struct Rectanglexy<M: Material> {
    mat: M,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl<M: Material> Rectanglexy<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: M) -> Self {
        Self {
            mat,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl<M: Material> Hittable for Rectanglexy<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max || t.is_nan() {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let t = t;
        let outward_normal = Vec3::new(0., 0., 1.);
        let mut rec = HitRecord::new(
            t,
            r.at(t),
            Vec3::default(),
            bool::default(),
            &self.mat,
            u,
            v,
        );
        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mat;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let getn = self.hit(&Ray::new(*origin, *v, 0.), 0.001, INFINITY);

        if getn.is_none() {
            return 0.;
        }
        let rec = getn.unwrap();
        let area = (self.x1 - self.x0) * (self.y1 - self.y0);
        let distance_squared = rec.t.powi(2) * v.len_square();
        let cos = Vec3::dot(v, &rec.normal).abs() / v.len();

        // println!("{} {} {} {}", rec.t, distance_squared, cos, area);
        distance_squared / (cos * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let point = Point3::new(
            random_range(self.x0, self.x1),
            random_range(self.y0, self.y1),
            self.k,
        );
        point - *origin
    }
}

pub struct Rectanglexz<M: Material> {
    mat: M,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<M: Material> Rectanglexz<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: M) -> Self {
        Self {
            mat,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for Rectanglexz<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max || t.is_nan() {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let t = t;
        let outward_normal = Vec3::new(0., 1., 0.);

        let mut rec = HitRecord::new(
            t,
            r.at(t),
            Vec3::default(),
            bool::default(),
            &self.mat,
            u,
            v,
        );

        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mat;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let getn = self.hit(&Ray::new(*origin, *v, 0.), 0.001, INFINITY);

        if getn.is_none() {
            return 0.;
        }
        let rec = getn.unwrap();
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t.powi(2) * v.len_square();
        let cos = Vec3::dot(v, &rec.normal).abs() / v.len();

        // println!("{} {} {} {}", rec.t, distance_squared, cos, area);
        distance_squared / (cos * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let point = Point3::new(
            random_range(self.x0, self.x1),
            self.k,
            random_range(self.z0, self.z1),
        );
        point - *origin
    }
}

pub struct Rectangleyz<M: Material> {
    mat: M,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<M: Material> Rectangleyz<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: M) -> Self {
        Self {
            mat,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for Rectangleyz<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max || t.is_nan() {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let t = t;
        let outward_normal = Vec3::new(1., 0., 0.);

        let mut rec = HitRecord::new(
            t,
            r.at(t),
            Vec3::default(),
            bool::default(),
            &self.mat,
            u,
            v,
        );

        rec.set_face_normal(r, &outward_normal);
        // rec.mat = self.mat;
        // rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        let getn = self.hit(&Ray::new(*origin, *v, 0.), 0.001, INFINITY);

        if getn.is_none() {
            return 0.;
        }
        let rec = getn.unwrap();
        let area = (self.y1 - self.y0) * (self.z1 - self.z0);
        let distance_squared = rec.t.powi(2) * v.len_square();
        let cos = Vec3::dot(v, &rec.normal).abs() / v.len();

        // println!("{} {} {} {}", rec.t, distance_squared, cos, area);
        distance_squared / (cos * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let point = Point3::new(
            self.k,
            random_range(self.y0, self.y1),
            random_range(self.z0, self.z1),
        );
        point - *origin
    }
}

pub struct Rotatey<T: Hittable> {
    ptr: T,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl<T: Hittable> Rotatey<T> {
    // 以垂直与y轴的方向rotate degree度
    pub fn new(ptr: T, angle: f64) -> Self {
        let radians = degree_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = ptr.bounding_box(0., 1.).unwrap();
        let hasbox = true;

        let mut mi = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut mx = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maxi.x + (1 - i) as f64 * bbox.mini.x;
                    let y = j as f64 * bbox.maxi.y + (1 - j) as f64 * bbox.mini.y;
                    let z = k as f64 * bbox.maxi.z + (1 - k) as f64 * bbox.mini.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        mi[c] = mi[c].min(tester[c]);
                        mx[c] = mx[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new(mi, mx);
        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl<T: Hittable> Hittable for Rotatey<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
    pub fn get_sphere_uv(p: &Point3) -> Option<[f64; 2]> {
        /*
            p: a given point on the sphere of radius one, centered at the origin.
            u: returned value [0,1] of angle around the Y axis from X=-1.
            v: returned value [0,1] of angle from Y=-1 to Y=+1.
               <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
               <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
               <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        */

        let theta = (-p.y()).acos();
        let phi = p.x().atan2(-p.z()) + PI;

        Some([phi / 2. / PI, theta / PI])
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().len_square();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.len_square() - self.radius * self.radius;

        let discrim = half_b.powi(2) - a * c;
        if discrim < 0. {
            return None;
        }

        let sqrtd = discrim.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        let tup = Self::get_sphere_uv(&outward_normal).unwrap();

        let mut rec = HitRecord::new(
            root,
            r.at(root),
            Vec3::default(),
            bool::default(),
            &self.mat,
            tup[0],
            tup[1],
        );

        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let cub = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - cub, self.center + cub))
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if self.hit(&Ray::new(*o, *v, 0.), 0.001, INFINITY).is_none() {
            return 0.;
        }

        let cos_theta_max = (1. - self.radius.powi(2) / (self.center - *o).len_square()).sqrt();
        let solid_angle = 2. * PI * (1. - cos_theta_max);

        1. / solid_angle
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;
        let distance_squared = direction.len_square();
        let uvw = ONB::build(&direction);
        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}

// 物体空间平移 offset
#[derive(Debug)]
pub struct Translate<H: Hittable> {
    ptr: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(ptr: H, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            let outward = rec.normal;
            rec.set_face_normal(&moved_r, &outward);

            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            let output_box =
                AABB::new(output_box.mini + self.offset, output_box.maxi + self.offset);
            Some(output_box)
        } else {
            None
        }
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        self.ptr.random(&(*origin - self.offset))
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.ptr.pdf_value(&(*o - self.offset), v)
        // println!("{:?} {:?} res = {}", *o - self.offset, v, res);
    }
}

const EPS_: f64 = 1e-10;

pub struct Triangle<M: Material> {
    pub v0: Point3,
    pub v1: Point3,
    pub v2: Point3,
    pub mat: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, mat: M) -> Self {
        Self { v0, v1, v2, mat }
    }
    pub fn get_normal(&self) -> Vec3 {
        (self.v1 - self.v0).cross(self.v2 - self.v0).unit_vector()
    }
    pub fn inside(&self, p: Point3) -> bool {
        let n = self.get_normal();
        Vec3::dot(&n, &(self.v1 - self.v0).cross(p - self.v0)) >= 0.
            && Vec3::dot(&n, &(self.v2 - self.v1).cross(p - self.v1)) >= 0.
            && Vec3::dot(&n, &(self.v0 - self.v2).cross(p - self.v2)) >= 0.
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let n = self.get_normal();
        let NdotRaydir = n.dot(&r.direction());
        if NdotRaydir.abs() < EPS_ {
            return None;
        }
        let t = Vec3::dot(&(self.v0 - r.orig), &n) / NdotRaydir;

        if t < t_min || t > t_max {
            return None;
        }

        let p = r.at(t);
        if !self.inside(p) {
            return None;
        }

        let a1 = self.v0.x - self.v1.x;
        let b1 = self.v0.x - self.v2.x;
        let c1 = self.v0.x - p.x;
        let a2 = self.v0.y - self.v1.y;
        let b2 = self.v0.y - self.v2.y;
        let c2 = self.v0.y - p.y;

        let beta = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
        let gamma = (a1 * c2 - a2 * c1) / (a1 * b2 - b1 * a2);

        let mut rec = HitRecord::new(
            t,
            p,
            Vec3::default(),
            bool::default(),
            &self.mat,
            beta,
            gamma,
        );
        rec.set_face_normal(r, &n);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let eps = Point3::new(EPS_, EPS_, EPS_);
        Some(AABB::new(
            Point3::new(
                self.v0.x.min(self.v1.x.min(self.v2.x)),
                self.v0.y.min(self.v1.y.min(self.v2.y)),
                self.v0.z.min(self.v1.z.min(self.v2.z)),
            ) - eps,
            Point3::new(
                self.v0.x.max(self.v1.x.max(self.v2.x)),
                self.v0.y.max(self.v1.y.max(self.v2.y)),
                self.v0.z.max(self.v1.z.max(self.v2.z)),
            ) + eps,
        ))
    }
}

pub struct CosPDF {
    uvw: ONB,
}

impl CosPDF {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: ONB::build(w) }
    }
}

impl PDF for CosPDF {
    fn value(&self, direction: &Vec3) -> f64 {
        // 使用蒙特卡罗方法, 计算相应的pdf
        let cos = Vec3::dot(&direction.unit_vector(), &self.uvw.w());
        let cos = cos.max(0.);
        cos / PI
    }
    fn generate(&self) -> Vec3 {
        // 在半球上生成随机
        self.uvw.local(random_cosine_direction())
    }
}

#[derive(Clone)]
pub struct HittablePDF<'a, H: Hittable> {
    origin: Point3,
    ptr: &'a H,
}

impl<'a, H: Hittable> HittablePDF<'a, H> {
    pub fn new(ptr: &'a H, origin: Vec3) -> Self {
        Self { origin, ptr }
    }
}

impl<'a, H: Hittable> PDF for HittablePDF<'a, H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.origin)
    }
}

pub struct MixturePDF<T1: PDF, T2: PDF> {
    p0: T1,
    p1: T2,
}

impl<T1: PDF, T2: PDF> MixturePDF<T1, T2> {
    pub fn new(p0: T1, p1: T2) -> Self {
        Self { p0, p1 }
    }
}

impl<T1: PDF, T2: PDF> PDF for MixturePDF<T1, T2> {
    fn value(&self, direction: &Vec3) -> f64 {
        // println!("{} dir = {:?}", self.p0.value(direction), direction);
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

pub trait PDF {
    fn generate(&self) -> Vec3;
    fn value(&self, direction: &Vec3) -> f64;
}

pub fn random_cosine_direction() -> Vec3 {
    // 在半球内随机, 关于cos(theta)分布, theta为与normal所成角
    let r1 = random_double();
    let r2 = random_double();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = 1. + r2 * ((1. - radius.powi(2) / distance_squared).sqrt() - 1.);

    let phi = 2. * PI * r1;
    let x = phi.cos() * (1. - z.powi(2)).sqrt();
    let y = phi.sin() * (1. - z.powi(2)).sqrt();

    Vec3::new(x, y, z)
}

pub struct Checker<T: Texture> {
    pub odd: T,
    pub even: T,
}

impl<T: Texture> Checker<T> {
    pub fn new(c1: Color, c2: Color) -> Checker<SolidColor> {
        Checker {
            odd: SolidColor::new(c1.x, c1.y, c1.z),
            even: SolidColor::new(c2.x, c2.y, c2.z),
        }
    }
}

impl<T: Texture> Texture for Checker<T> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Option<Color> {
        let sines = (p.x() * 10.).sin() * (p.y() * 10.).sin() * (p.z() * 10.).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    pub data: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let data = image::open(filename).unwrap();
        let width = data.width();
        let height = data.height();
        Self {
            width,
            height,
            data,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Option<Color> {
        let u = clamp(u, 0., 1.);
        let v = 1. - clamp(v, 0., 1.);

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1. / 255.;
        let pixel = self.data.get_pixel(i, j).to_rgb();

        Some(Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        ))
    }
}

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Option<Color>;
}

pub struct ObjTexture {
    pub u1: f64,
    pub v1: f64,
    pub u2: f64,
    pub v2: f64,
    pub u3: f64,
    pub v3: f64,
    pub img: Arc<RgbImage>,
}

impl ObjTexture {
    pub fn new(img: Arc<RgbImage>, u1: f64, v1: f64, u2: f64, v2: f64, u3: f64, v3: f64) -> Self {
        Self {
            u1,
            v1,
            u2,
            v2,
            u3,
            v3,
            img,
        }
    }
}

impl Texture for ObjTexture {
    fn value(&self, beta: f64, gamma: f64, _p: &Point3) -> Option<Color> {
        // if self.u > 1. || self.u < 0. || self.v > 1. || self.v < 0. {
        //     panic!();
        // }
        // let u = clamp(self.u, 0., 1.);
        // let v = clamp(self.v, 0., 1.);

        let alpha = 1. - beta - gamma;
        let u = self.u1 * alpha + self.u2 * beta + self.u3 * gamma;
        let v = self.v1 * alpha + self.v2 * beta + self.v3 * gamma;
        let mut i = (u * ((self.img.width()) as f64)) as u32;
        let mut j = ((1. - v) * ((self.img.height()) as f64)) as u32;

        // let mut i = (self.u * self.img.width() as f64) as u32;
        // let mut j = ((1. - self.v) * self.img.height() as f64) as u32;

        if i >= self.img.width() {
            i = self.img.width() - 1;
        }
        if j >= self.img.height() {
            j = self.img.height() - 1;
        }

        let color_scale = 1. / 255.;
        let pixel = self.img.get_pixel(i, j);

        Some(Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        ))
    }
}

const POINT_COUNT: usize = 256;

#[derive(Default)]
pub struct Perlin {
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
    pub ranvec: Vec<Point3>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = vec![Point3::default(); POINT_COUNT];
        for item in ranvec.iter_mut().take(POINT_COUNT) {
            *item = Point3::random_range(-1., 1.);
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            perm_x,
            perm_y,
            perm_z,
            ranvec,
        }
    }

    fn trilinear_inerp(a3: Array3<Point3>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1. - u))
                        * (j as f64 * v + (1 - j) as f64 * (1. - v))
                        * (k as f64 * w + (1 - k) as f64 * (1. - w))
                        * Point3::dot(
                            &a3[[i, j, k]],
                            &Point3::new(u - i as f64, v - j as f64, w - k as f64),
                        );
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Point3, depth: isize) -> f64 {
        let mut accum = 0.;
        let mut temp = *p;
        let mut weigth = 1.;

        for _i in 0..depth {
            accum += weigth * Self::noise(&self, &temp);
            weigth *= 0.5;
            temp *= 2.;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u.powi(2) * (3. - 2. * u);
        v = v.powi(2) * (3. - 2. * v);
        w = w.powi(2) * (3. - 2. * w);

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;

        let mut a3 = Array3::<Point3>::default((2, 2, 2));

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    a3[[di, dj, dk]] = self.ranvec[self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                }
            }
        }
        Self::trilinear_inerp(a3, u, v, w)
    }

    pub fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![0; POINT_COUNT];

        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as usize;
        }

        Perlin::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            let j = thread_rng().gen_range(0..i);
            p.swap(i, j);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Option<Color> {
        Some(
            Color::new(1., 1., 1.)
                * 0.5
                * (1. + (self.scale * p.z() + 10. * self.noise.turb(p, 7)).sin()),
        )
    }
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    pub color_val: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_val: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Option<Color> {
        Some(self.color_val)
    }
}

pub struct HitRecord<'a> {
    pub p: Point3,        // 碰撞点
    pub normal: Vec3,     // 碰撞点的单 位 法 向 量(与Ray的方向相反)
    pub t: f64,           // 表示 p = Ray(t)
    pub front_face: bool, // 是否Ray来自外侧
    pub mat: &'a dyn Material,
    pub u: f64, // u, v: 用于贴图
    pub v: f64, // u, v \in [0, 1]
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        p: Point3,
        normal: Vec3,
        front_face: bool,
        mat: &'a dyn Material,
        u: f64,
        v: f64,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
            mat,
            u,
            v,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

// ---- Hittable trait ----
pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
}

// ---- Hittable List ----
// 用于存储 Hittable 的 struct

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    // 这里必须要用Arc来实现多态(因为有Vec的存在, 需要实现一个Vector中存多种类型)
}

impl HittableList {
    pub fn add(&mut self, element: Arc<dyn Hittable>) {
        self.objects.push(element);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(tmp_rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = tmp_rec.t;
                hit_rec = Some(tmp_rec);
            }
        }
        hit_rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;
        let mut tmp_box = AABB::default();

        for obj in &self.objects {
            match obj.bounding_box(time0, time1) {
                Some(tmp_AABB) => {
                    if first_box {
                        tmp_box = tmp_AABB;
                        first_box = false;
                    } else {
                        tmp_box = surrounding_box(tmp_box, tmp_AABB);
                    }
                }
                None => {
                    return None;
                }
            }
        }
        Some(tmp_box)
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weigh = 1. / self.objects.len() as f64;
        let mut sum = 0.;

        for object in self.objects.iter() {
            sum += weigh * object.pdf_value(o, v);
        }

        sum
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.objects[thread_rng().gen_range(0..self.objects.len())].random(o)
    }
}

pub fn load_obj(
    world: &mut HittableList,
    rootfile: &str,
    rate: f64,
    objfile: &str,
    offset: Vec3,
    rotate_angle: f64,
) {
    // rate: 物体放大倍数 objfile: obj格式文件名 imgfile: 贴图文件名
    let obj = tobj::load_obj(
        //"obj_material/10483_baseball_v1_L3.obj",
        String::from(rootfile) + objfile,
        //"obj_material/patrick.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );

    assert!(obj.is_ok());

    let (models, materials) = obj.expect("Failed to load OBJ file");
    let materials = materials.unwrap();

    // Materials might report a separate loading error if the MTL file wasn't found.
    // If you don't need the materials, you can generate a default here and use that instead.
    // let materials = materials.expect("Failed to load MTL file");

    for m in models.iter() {
        let mesh = &m.mesh;
        let mat_id = mesh.material_id.unwrap();

        let mat_file_name = String::from(rootfile) + materials[mat_id].diffuse_texture.as_str();

        // println!("{}", mat_file_name);
        // Todo: 这里重复的png/jpg可能会被多次load, 可以用一个Hash来优化一下
        let tex = Arc::new(
            image::open(mat_file_name)
                .expect("load image failed")
                .into_rgb8(),
        );

        assert!(!mesh.texcoords.is_empty());

        let mut vertices: Vec<Point3> = Vec::default(); // 存储所用到的点集
        for id in 0..mesh.positions.len() / 3 {
            let x = mesh.positions[3 * id] as f64;
            let y = mesh.positions[3 * id + 1] as f64;
            let z = mesh.positions[3 * id + 2] as f64;
            vertices.push(Point3::new(x, y, z));
        }

        let mut object = HittableList::default();
        for i in 0..mesh.indices.len() / 3 {
            // [idx_x, idx_y, idx_z, ... ] 三个点为一个triangle

            let idx_x = mesh.indices[i * 3] as usize;
            let idx_y = mesh.indices[i * 3 + 1] as usize;
            let idx_z = mesh.indices[i * 3 + 2] as usize;

            let u1 = mesh.texcoords[2 * idx_x] as f64;
            let v1 = mesh.texcoords[2 * idx_x + 1] as f64;

            let u2 = mesh.texcoords[2 * idx_y] as f64;
            let v2 = mesh.texcoords[2 * idx_y + 1] as f64;

            let u3 = mesh.texcoords[2 * idx_z] as f64;
            let v3 = mesh.texcoords[2 * idx_z + 1] as f64;

            let mat = ObjTexture::new(tex.clone(), u1, v1, u2, v2, u3, v3);
            //let mut col = mat1.value(0.5, 0., &Point3::default()).unwrap();

            let tri = Triangle::new(
                rate * vertices[idx_x],
                rate * vertices[idx_y],
                rate * vertices[idx_z],
                Lambertian::new_texture(mat),
            );
            object.add(Arc::new(tri));
            // println!("{}", i);
        }

        //std::process::exit(0);
        // println!("{}", object.objects.len());
        let object = BvhNode::new_from_vec(object.objects, 0., 1.);
        let object = Rotatey::new(object, rotate_angle);
        let object = Translate::new(object, offset);
        world.add(Arc::new(object));
    }
}

pub fn cornell_box() -> (HittableList, HittableList) {
    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let red = Lambertian::<SolidColor>::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::<SolidColor>::new(Color::new(12., 12., 12.));

    world.add(Arc::new(Rectangleyz::new(0., 555., 0., 555., 555., green)));
    world.add(Arc::new(Rectangleyz::new(0., 555., 0., 555., 0., red)));
    world.add(Arc::new(Rectanglexz::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(Arc::new(Rectanglexz::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
    )));
    world.add(Arc::new(Rectanglexy::new(0., 555., 0., 555., 555., white)));

    let light1 = Arc::new(
        //    Translate::new(
        Rectangleyz::new(213., 343., 227., 332., 554., light.clone()),
        //     Point3::new(0., 100., 0.,)
        // ),
    );
    world.add(light1.clone());
    lights.add(light1);

    let light2 = Arc::new(
        //    Translate::new(
        Rectangleyz::new(213., 343., 227., 332., 1., light.clone()),
        //     Point3::new(0., 100., 0.,)
        // ),
    );
    world.add(light2.clone());
    lights.add(light2);

    let light3 = Arc::new(
        //    Translate::new(
        Rectanglexz::new(213., 343., 227., 332., 554., light.clone()),
        //     Point3::new(0., 100., 0.,)
        // ),
    );
    world.add(light3.clone());
    lights.add(light3);

    let light4 = Arc::new(
        //    Translate::new(
        Rectanglexz::new(213., 343., 227., 332., 1., light),
        //     Point3::new(0., 100., 0.,)
        // ),
    );
    world.add(light4.clone());
    lights.add(light4);

    // let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.);
    // let box1 = Cube::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 330., 165.),
    //     aluminum,
    // );
    // let box1 = Rotatey::new(box1, 15.);
    // let box1 = Translate::new(box1, Vec3::new(265., 0., 295.));
    // world.add(Arc::new(box1));

    // let glass = Dielectric::new(1.5);
    // let ball1 = Arc::new(Sphere::new(Point3::new(190., 90., 190.), 90., glass));
    // world.add(ball1.clone());
    // lights.add(ball1);

    // let tri = Triangle::new(
    //     Point3::new(170., 50., 0.),
    //     Point3::new(300., 50., -50.),
    //     Point3::new(250., 205., 200.),
    //     red,
    // );
    // world.add(Arc::new(tri));

    // let box2 = Arc::new(Cube::new(
    //     Point3::new(0., 0., 0.),
    //     Point3::new(165., 165., 165.),
    //     white,
    // ));
    // let box2 = Arc::new(Rotatey::new(box2, -18.));
    // let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    // world.add(box2);

    // let tmp = add_bvh_static();

    // world.add(Arc::new(Translate::new(tmp, Vec3::new(270., 170., 450.))));
    // load_obj(
    //     &mut world,
    //     "obj_material/",
    //     100.,
    //     "patrick.obj",
    //     Vec3::new(270., 70., 450.),
    //     180.,
    // );

    (world, lights)
}

fn ray_color(
    r: Ray,
    background: Color,
    world: &HittableList,
    lights: &HittableList,
    depth: i32,
) -> Color {
    if depth <= 0 {
        // 反射过多次, 可认为碰到了一个corner, 直接返回(0,0,0)无光
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(&r, 0.001, INFINITY) {
        let emitted = rec.mat.emitted(rec.u, rec.v, &rec.p).unwrap(); // 击中物体本身发光程度(目前只有diffuse材质会emit light)
        if let Some(ScatterRecord) = (rec.mat).scatter(&r, &rec) {
            if ScatterRecord.is_specular {
                return ScatterRecord.attenuation
                    * ray_color(
                        ScatterRecord.specular_ray,
                        background,
                        world,
                        lights,
                        depth - 1,
                    );
            }

            // 这部分目前就是Lambertian材质的Tracer
            let light_ptr = HittablePDF::new(lights, rec.p); // 按光源位置分布的pdf

            let mix_pdf = MixturePDF::new(light_ptr, ScatterRecord.pdf_ptr.unwrap()); // 将light_pdf 与 cos分布(如lambertian)进行mixture

            let scattered = Ray::new(rec.p, mix_pdf.generate(), r.time());
            let pdf_val = mix_pdf.value(&scattered.direction());
            // println!("$ {}", light_ptr.value(&scattered.direction()));

            //println!("{:?} {}", ScatterRecord.attenuation, pdf_val);
            emitted
                + ScatterRecord.attenuation
                    * (rec.mat).scatter_pdf(&r, &rec, &scattered).unwrap()
                    * ray_color(scattered, background, world, lights, depth - 1)
                    / pdf_val
        } else {
            emitted
        }
    } else {
        background
    }
}
// 在每个像素点周围(小范围)内采样sample_per_pixel次, 暴力取平均值
fn write_color(pixel_color: Color, samples_per_pixel: usize) -> [u8; 3] {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    [
        (256. * clamp(r, 0., 0.999)) as u8,
        (256. * clamp(g, 0., 0.999)) as u8,
        (256. * clamp(b, 0., 0.999)) as u8,
    ]
}

fn ray_benchmark(c: &mut Criterion) {
    const THREAD_NUMBER: usize = 8;

    // Image
    const RATIO: f64 = 1.;
    const IMAGE_WIDTH: usize = 600;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 10;
    const MAX_DEPTH: i32 = 50;

    let quality = 100;
    let path = "output/output.jpg";

    // World

    let aperture = 0.;
    let background = Color::new(0., 0., 0.);
    let lf = Point3::new(278., 278., -800.);
    let la = Point3::new(278., 278., 0.);
    let vfov = 40.;

    let (world, lights) = cornell_box();

    /*
    let switch = 6;
    match switch {
        0 => {
            world = HittableList::two_sphere();
        }
        1 => {
            world = HittableList::two_perlin_sphere();
        }
        2 => {
            world = HittableList::load_image();
        }
        3 => {
            world = HittableList::simple_light();
            background = Color::new(0., 0., 0.);
            lf = Point3::new(26., 3., 6.);
            la = Point3::new(0., 2., 0.);
        }
        4 => {
            world = HittableList::cornell_box_smoke();
            background = Color::new(0., 0., 0.);
            lf = Point3::new(278., 278., -800.);
            la = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        5 => {
            world = HittableList::final_scene();
            background = Color::new(0., 0., 0.);
            lf = Point3::new(478., 278., -600.);
            la = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        6 => {
            world = HittableList::cornell_box();
            background = Color::new(0., 0., 0.);
            lf = Point3::new(278., 278., -800.);
            la = Point3::new(278., 278., 0.);
            vfov = 40.;
        }
        _ => {
            world = HittableList::random_scene();
            aperture = 0.1;
        }
    }
    */

    // Camera

    let cam = Camera::new(
        lf,
        la,
        Vec3::new(0., 1., 0.),
        vfov,
        RATIO,
        aperture,
        10.,
        0.,
        1.,
    );

    // let i = IMAGE_HEIGHT / 2;
    // let j = IMAGE_WIDTH / 2;
    // let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.);
    // let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.);
    // let r = cam.get_ray(u, v);
    // // ---- benchmark test ----
    // c.bench_function("Ray test", |b| { // b: Bencher 类型
    //     b.iter(|| ray_color(
    //         black_box(r),
    //         black_box(background),
    //         black_box(&world),
    //         black_box(&lights),
    //         black_box(MAX_DEPTH),
    //     ))
    // });

    c.bench_function("Ray test", |b| {
        // b: Bencher 类型
        b.iter(|| {
            for a in 0..=5 {
                for b in 0..=5 {
                    let i = IMAGE_HEIGHT / 2 - a;
                    let j = IMAGE_WIDTH / 2 - b;
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.);
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.);
                    let r = cam.get_ray(u, v);
                    ray_color(
                        black_box(r),
                        black_box(background),
                        black_box(&world),
                        black_box(&lights),
                        black_box(MAX_DEPTH),
                    );
                }
            }
        })
    });

    // Render

    // println!(
    //     "         Image size:                {}",
    //     style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow()
    // );
    // println!(
    //     "         Sample number per pixel:   {}",
    //     style(SAMPLES_PER_PIXEL.to_string()).yellow()
    // );
    // println!(
    //     "         Reflection max depth:      {}",
    //     style(MAX_DEPTH.to_string()).yellow()
    // );

    // const SECTION_LINE_NUM: usize = IMAGE_HEIGHT / THREAD_NUMBER;

    // let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    // let mut output_pixel_color = Vec::<Color>::new(); // store pixels
    // let mut thread_pool = Vec::<_>::new(); // store closures

    // let multiprogress = Arc::new(MultiProgress::new());
    // multiprogress.set_move_cursor(true);

    // for thread_id in 0..THREAD_NUMBER {
    //     let line_beg = SECTION_LINE_NUM * thread_id;
    //     let mut line_end = line_beg + SECTION_LINE_NUM;
    //     if line_end > IMAGE_HEIGHT || (thread_id == THREAD_NUMBER - 1 && line_end < IMAGE_HEIGHT) {
    //         line_end = IMAGE_HEIGHT;
    //     }

    //     let mp = multiprogress.clone();
    //     let progress_bar = mp.add(ProgressBar::new(
    //         ((line_end - line_beg) * IMAGE_WIDTH) as u64,
    //     ));
    //     progress_bar.set_style(ProgressStyle::default_bar()
    //     .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
    //     .progress_chars("#>-"));

    //     let (tx, rx) = mpsc::channel();

    //     let clone_world = world.clone(); // due to multithread's ownership problem
    //     let clone_lights = lights.clone();

    //     thread_pool.push((
    //         thread::spawn(move || {
    //             let mut progress = 0;
    //             progress_bar.set_position(0);

    //             let channel_send = tx;

    //             let mut section_pixel_color = Vec::<Color>::new();

    //             for j in line_beg..line_end {
    //                 for i in 0..IMAGE_WIDTH {
    //                     let mut pixel_color: Color = Color::new(0., 0., 0.);
    //                     for _s in 0..SAMPLES_PER_PIXEL {
    //                         let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.);
    //                         let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.);
    //                         let r = cam.get_ray(u, v);

    //                         pixel_color +=
    //                             ray_color(r, background, &clone_world, &clone_lights, MAX_DEPTH);
    //                     }
    //                     section_pixel_color.push(pixel_color);
    //                     progress += 1;
    //                     progress_bar.set_position(progress);
    //                 }
    //             }
    //             channel_send.send(section_pixel_color).unwrap();
    //             progress_bar.finish_with_message("Finished.");
    //         }),
    //         rx,
    //     ));
    // }

    // multiprogress.join().unwrap();

    // let mut thread_progress_finish: bool = true;
    // let collecting_progress_bar = ProgressBar::new(THREAD_NUMBER as u64);

    // for thread_id in 0..THREAD_NUMBER {
    //     let thread = thread_pool.remove(0);
    //     match thread.0.join() {
    //         Ok(_) => {
    //             let mut received = thread.1.recv().unwrap();
    //             output_pixel_color.append(&mut received);
    //             collecting_progress_bar.inc(1);
    //         }
    //         Err(_) => {
    //             thread_progress_finish = false;
    //             println!(
    //                 "      ⚠️ {}{}{}",
    //                 style("Joining the ").red(),
    //                 style(thread_id.to_string()).yellow(),
    //                 style("th thread failed!").red(),
    //             );
    //         }
    //     }
    // }
    // if !thread_progress_finish {
    //     println!("{}", style("RE").bold().red());
    //     exit(1);
    // }
    // collecting_progress_bar.finish_and_clear();

    // let mut pixel_id = 0;
    // for j in 0..IMAGE_HEIGHT as u32 {
    //     for i in 0..IMAGE_WIDTH as u32 {
    //         let pixel_color = output_pixel_color[pixel_id];
    //         let pixel = img.get_pixel_mut(i, IMAGE_HEIGHT as u32 - j - 1);
    //         *pixel = image::Rgb(write_color(pixel_color, SAMPLES_PER_PIXEL));
    //         pixel_id += 1;
    //     }
    // }
    // println!(
    //     "Image size: {}\nJPEG quality: {}",
    //     style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow(),
    //     style(quality.to_string()).yellow(),
    // );

    // // Output image to file
    // println!("Ouput image as \"{}\"", style(path).yellow());
    // let output_image = image::DynamicImage::ImageRgb8(img);
    // let mut output_file = File::create(path).unwrap();
    // match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
    //     Ok(_) => {}
    //     // Err(_) => panic!("Outputting image fails."),
    //     Err(_) => println!("{}", style("Outputting image fails.").red()),
    // }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

criterion_group!(benches, ray_benchmark);
criterion_main!(benches);

/*
#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[inline]
fn concat_with_format_macro(s1: &str, s2: &str, s3: &str) -> String {
    format!("{}{}{}", s1, s2, s3)
}

#[inline]
fn concat_with_push_str(s1: &str, s2: &str, s3: &str) -> String {
    let mut s = s1.to_string();
    s.push_str(s2);
    s.push_str(s3);
    s
}

macro_rules! call {
    ($fn:ident) => {
        $fn(
            black_box("2014-11-28"),
            black_box("T"),
            black_box("12:00:09Z"),
        )
    };
}

fn concat_benchmark(c: &mut Criterion) {
    #![allow(unused_mut)]
    let mut group = c.benchmark_group("num_elements");
    macro_rules! bench {
        ($tp:expr $(,)*) => {};
        ($tp:expr, $st: literal $($strs: literal)*, $id: ident $($ids: ident)*, $p: literal $($ps: literal)*) => {
            group.throughput(Throughput::Elements($tp));
            group.bench_with_input(format!("concat_with_push_str with {} elements", $tp), &($st, $($strs,)*), |b, &($id, $($ids,)*)| b.iter(|| {
                let mut s = $id.to_string();
                $(
                    s.push_str($ids);
                )*
                s
            }));
            group.bench_with_input(format!("concat_with_format_macro with {} elements", $tp), &($st, $($strs,)*), |b, &($id, $($ids,)*)| b.iter(|| {
                format!($p, $id, $($ids),*)
            }));
            bench!({($tp) - 1u64}, $($strs)*, $($ids)*, $($ps)*);
        };
    }

    bench!(
        10u64,
        "x" "xx" "xxx" "xxxx" "xxxxx" "xxxxxx" "xxxxxxx" "xxxxxxxx" "xxxxxxxxx" "xxxxxxxxx",
        s1 s2 s3 s4 s5 s6 s7 s8 s9 s10,
        "{}{}{}{}{}{}{}{}{}{}" "{}{}{}{}{}{}{}{}{}" "{}{}{}{}{}{}{}{}" "{}{}{}{}{}{}{}"
        "{}{}{}{}{}{}" "{}{}{}{}{}" "{}{}{}{}" "{}{}{}" "{}{}" "{}"
    );
}

// pub fn criterion_benchmark(c: &mut Criterion) {
//     // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
//     c.bench_function("concat_with_push_str", |b| {
//         b.iter(|| call!(concat_with_push_str))
//     })
//     .bench_function("concat_with_format_macro", |b| {
//         b.iter(|| call!(concat_with_format_macro))
//     });
// }

criterion_group!(benches, concat_benchmark);
criterion_main!(benches);

*/
