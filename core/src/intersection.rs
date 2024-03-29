use crate::*;

pub struct Hit<'a> {
    pub intersection: &'a Intersection<'a>,
    pub point: Point,
    pub object_point: Point,
    pub eyev: UnitVector,
    pub normalv: UnitVector,
    pub inside: bool,
    pub reflectv: UnitVector,
    pub n1: f32,
    pub n2: f32,
}

impl Hit<'_> {
    pub fn schlick(&self) -> f32 {
        let mut cos = dot(&self.eyev, &self.normalv);

        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = n_ratio * n_ratio * (1. - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = f32::sqrt(1.0 - sin2_t);
        }

        let r0 = (self.n1 - self.n2) / (self.n1 + self.n2);
        let r0 = r0 * r0;
        r0 + (1. - r0) * f32::powi(1. - cos, 5)
    }
}

pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
    pub uv: Option<(f32, f32)>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a dyn Shape) -> Intersection<'a> {
        Intersection {
            t,
            object,
            uv: None,
        }
    }

    pub fn new_with_uv(t: f32, object: &'a dyn Shape, uv: (f32, f32)) -> Intersection<'a> {
        Intersection {
            t,
            object,
            uv: Some(uv),
        }
    }

    pub fn prepare_hit(&self, ray: &Ray) -> Hit {
        let point = ray.position(self.t);
        let object_point = self.object.get_transform_inverse() * point;
        let eyev = UnitVector::new_normalize(-ray.direction);
        let normalv = self.object.normal_at(&point, self);
        let inside = dot(&normalv, &eyev) < 0.;
        let normalv = if inside { -normalv } else { normalv };
        let reflectv = UnitVector::new_unchecked(reflect(&ray.direction, &normalv));

        // Transparency
        let material = self.object.get_material();
        let (n1, n2) = if material.transparency.is_some() {
            // TODO: nested shapes. It only works in
            // vacuum-material interfaces.
            let n1 = 1.;
            let n2 = material.refractive_index;
            if inside {
                (n2, n1)
            } else {
                (n1, n2)
            }
        } else {
            (1., 1.)
        };

        Hit {
            intersection: self,
            point,
            object_point,
            eyev,
            inside,
            normalv,
            reflectv,
            n1,
            n2,
        }
    }
}

// TODO: Remove
pub type Intersections<'a> = &'a [Intersection<'a>];

// Returns the closest non-negative intersection
pub fn hit<'a>(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
    xs.iter()
        .filter(|&x| x.t > 0.)
        .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
}
