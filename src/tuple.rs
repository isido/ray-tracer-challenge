use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Tuple(pub f64, pub f64, pub f64, pub f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 0.0)
    }

    pub fn color(red: f64, green: f64, blue: f64) -> Tuple {
        Tuple(red, green, blue, 0.0)
    }

    pub fn magnitude(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        Tuple(
            self.0 / self.magnitude(),
            self.1 / self.magnitude(),
            self.2 / self.magnitude(),
            self.3 / self.magnitude(),
        )
    }

    pub fn dot(self, other: Tuple) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn cross(self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn hadamard(self, other: Tuple) -> Tuple {
        Tuple(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        let eps = 1e-5;
        (self.0 - other.0).abs() < eps
            && (self.1 - other.1).abs() < eps
            && (self.2 - other.2).abs() < eps
            && (self.3 - other.3).abs() < eps
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        Tuple(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        Tuple(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_point() {
        // A tuple with w=1.0 is a point
        let p = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(p.0, 4.3);
        assert_eq!(p.1, -4.2);
        assert_eq!(p.2, 3.1);
        assert_eq!(p.3, 1.0);
    }

    #[test]
    fn tuple_vector() {
        // A tuple with w=0 is a vector
        let v = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(v.0, 4.3);
        assert_eq!(v.1, -4.2);
        assert_eq!(v.2, 3.1);
        assert_eq!(v.3, 0.0);
    }

    #[test]
    fn equality_vector() {
        let t1 = Tuple::vector(1.0, 1.0, 1.0);
        let t2 = Tuple::vector(1.0, 1.0, 1.0);

        assert_eq!(t1, t2);
    }

    #[test]
    fn inequality_vector_point() {
        let t = Tuple::vector(1.0, 1.0, 1.0);
        let p = Tuple::point(1.0, 1.0, 1.0);

        assert_ne!(t, p);
    }

    #[test]
    fn inequality_vector_vector() {
        let v1 = Tuple::vector(0.0, 1.0, 0.0);
        let v2 = Tuple::vector(1.0, 0.0, 0.0);

        assert_ne!(v1, v2);
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(Tuple(1.0, 1.0, 6.0, 1.0), t1 + t2);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(Tuple::vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(Tuple::point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(Tuple::vector(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(Tuple::vector(-1.0, 2.0, -3.0), zero - v);
    }

    #[test]
    fn negating_tuple() {
        let t = Tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple(-1.0, 2.0, -3.0, 4.0), -t);
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let t = Tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple(3.5, -7.0, 10.5, -14.0), t * 3.5);
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let t = Tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple(0.5, -1.0, 1.5, -2.0), t * 0.5);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let t = Tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple(0.5, -1.0, 1.5, -2.0), t / 2.0);
    }

    #[test]
    fn magnitude_of_vectors() {
        assert_eq!(1.0, Tuple::vector(1.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, Tuple::vector(0.0, 1.0, 0.0).magnitude());
        assert_eq!(1.0, Tuple::vector(0.0, 0.0, 1.0).magnitude());
        assert_eq!(14.0_f64.sqrt(), Tuple::vector(1.0, 2.0, 3.0).magnitude());
        assert_eq!(14.0_f64.sqrt(), Tuple::vector(-1.0, -2.0, -3.0).magnitude());
    }

    #[test]
    fn normalizing_vectors() {
        assert_eq!(
            Tuple::vector(1.0, 0.0, 0.0),
            Tuple::vector(4.0, 0.0, 0.0).normalize()
        );
        assert_eq!(
            Tuple::vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            ),
            Tuple::vector(1.0, 2.0, 3.0).normalize()
        );
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        assert_eq!(1.0, Tuple::vector(1.0, 2.0, 3.0).normalize().magnitude());
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(20.0, v1.dot(v2));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::vector(-1.0, 2.0, -1.0), v1.cross(v2));
        assert_eq!(Tuple::vector(1.0, -2.0, 1.0), v2.cross(v1));
    }

    #[test]
    fn colors_are_tuples() {
        let c = Tuple::color(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, c.0);
        assert_eq!(0.4, c.1);
        assert_eq!(1.7, c.2);
    }

    #[test]
    fn adding_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);

        assert_eq!(Tuple::color(1.6, 0.7, 1.0), c1 + c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);

        assert_eq!(Tuple::color(0.2, 0.5, 0.5), c1 - c2); // TODO fix floating point rounding error error
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Tuple::color(0.2, 0.3, 0.4);

        assert_eq!(Tuple::color(0.4, 0.6, 0.8), c * 2.0);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Tuple::color(1.0, 0.2, 0.4);
        let c2 = Tuple::color(0.9, 1.0, 0.1);

        assert_eq!(Tuple::color(0.9, 0.2, 0.04), c1.hadamard(c2));
    }

}
