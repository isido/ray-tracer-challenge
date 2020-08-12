use std::cmp::Ordering;

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Intersection<'a> {
        Intersection {
            t: t,
            object: object,
        }
    }

    pub fn prepare_computations(&self, r: &Ray) -> Computations {
        let eyev_ = -r.direction;
        let mut normalv_ = self.object.normal_at(r.position(self.t));
        let mut inside_ = false;
        if normalv_.dot(eyev_) < 0.0 {
            inside_ = true;
            normalv_ = -normalv_;
        }

        Computations {
            t: self.t,
            object: self.object,
            point: r.position(self.t),
            eyev: eyev_,
            normalv: normalv_,
            inside: inside_,
        }
    }
}

pub fn intersections<'a>(xs: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
    let mut ys = xs.to_owned();
    ys.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));
    ys.to_vec()
}

pub fn hit<'a>(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
    for x in xs {
        if x.t >= 0.0 {
            return Some(x);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(3.5, i.t);
        assert_eq!(&s, i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&vec![i1, i2]);

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&vec![i1, i2]);
        let i = hit(&xs);

        assert_eq!(Some(&i1), i);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections(&vec![i1, i2]);
        let i = hit(&xs);

        assert_eq!(Some(&i2), i);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections(&vec![i1, i2]);
        let i = hit(&xs);

        assert_eq!(None, i);
    }

    #[test]
    fn hit_is_always_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = intersections(&vec![i1, i2, i3, i4]);
        let i = hit(&xs);

        assert_eq!(Some(&i4), i);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);
        assert_eq!(i.t, comps.t);
        assert_eq!(i.object, comps.object);
        assert_eq!(Tuple::point(0.0, 0.0, -1.0), comps.point);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normalv);
    }

    #[test]
    fn hit_when_intersection_occurs_on_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);

        let comps = i.prepare_computations(&r);
        assert_eq!(i.t, comps.t);
        assert_eq!(i.object, comps.object);
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), comps.point);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.eyev);
        assert_eq!(true, comps.inside);
        assert_eq!(Tuple::vector(0.0, 0.0, -1.0), comps.normalv);
    }
}
