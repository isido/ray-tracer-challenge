use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn creating_and_querying_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);
        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn computing_point_from_distance() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(Tuple::point(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Tuple::point(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Tuple::point(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Tuple::point(4.5, 3.0, 4.0), ray.position(2.5))
    }
}
