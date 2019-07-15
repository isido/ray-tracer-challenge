use crate::sphere::Sphere;

#[derive(Debug)]
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

pub fn intersections<'a>(i1: &'a Intersection, i2: &'a Intersection) -> Vec<&'a Intersection<'a>> {
    vec![i1, i2]
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
        let xs = intersections(&i1, &i2);

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }
}
