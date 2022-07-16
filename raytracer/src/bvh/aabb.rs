use crate::Hit::{Point3, Ray};

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

            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

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
