use std::cmp::Ordering;

use crate::sphere::Sphere;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Intersection<'a> {
        Intersection {
            t: t,
            object: object,
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
    use crate::sphere::Sphere;

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

}
