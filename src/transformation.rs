use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0,
    ];
    Matrix::from_vector(4, &v)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, z, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    Matrix::from_vector(4, &v)
}

pub fn rotation_x(r: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, 0.0, 0.0, 0.0,
        0.0, r.cos(), -r.sin(), 0.0,
        0.0, r.sin(), r.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];
    Matrix::from_vector(4, &v)
}

pub fn rotation_y(r: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        r.cos(), 0.0, r.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -r.sin(), 0.0, r.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0
    ];
    Matrix::from_vector(4, &v)
}

pub fn rotation_z(r: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        r.cos(), -r.sin(), 0.0, 0.0,
        r.sin(), r.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];
    Matrix::from_vector(4, &v)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    #[rustfmt::skip]
    let v = vec![
        1.0, xy, xz, 0.0,
        yx, 1.0, yz, 0.0,
        zx, zy, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    ];
    Matrix::from_vector(4, &v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;
    use std::f64::consts::{PI, SQRT_2};

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::point(2.0, 1.0, 7.0), transform.tuple_prod(p));
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(Tuple::point(-8.0, 7.0, 3.0), inv.tuple_prod(p));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(v, transform.tuple_prod(v));
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::point(-8.0, 18.0, 32.0), transform.tuple_prod(p));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::vector(-8.0, 18.0, 32.0), transform.tuple_prod(v));
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(Tuple::vector(-2.0, 2.0, 2.0), inv.tuple_prod(v));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(-2.0, 3.0, 4.0), transform.tuple_prod(p));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            Tuple::point(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0),
            half_quarter.tuple_prod(p)
        );
        assert_eq!(Tuple::point(0.0, 0.0, 1.0), full_quarter.tuple_prod(p));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        assert_eq!(
            Tuple::point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0),
            inv.tuple_prod(p)
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            Tuple::point(SQRT_2 / 2.0, 0.0, SQRT_2 / 2.0),
            half_quarter.tuple_prod(p)
        );
        assert_eq!(Tuple::point(1.0, 0.0, 0.0), full_quarter.tuple_prod(p));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            Tuple::point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0.0),
            half_quarter.tuple_prod(p)
        );
        assert_eq!(Tuple::point(-1.0, 0.0, 0.0), full_quarter.tuple_prod(p));
    }

    #[test]
    fn shearing_transformation_moves_x_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(5.0, 3.0, 4.0), transform.tuple_prod(p));
    }

    #[test]
    fn shearing_transformation_moves_x_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(6.0, 3.0, 4.0), transform.tuple_prod(p));
    }

    #[test]
    fn shearing_transformation_moves_y_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(2.0, 5.0, 4.0), transform.tuple_prod(p));
    }

    #[test]
    fn shearing_transformation_moves_y_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(2.0, 7.0, 4.0), transform.tuple_prod(p));
    }

    #[test]
    fn shearing_transformation_moves_z_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(2.0, 3.0, 6.0), transform.tuple_prod(p));
    }

    #[test]
    fn shearing_transformation_moves_z_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(Tuple::point(2.0, 3.0, 7.0), transform.tuple_prod(p));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a.tuple_prod(p);
        assert_eq!(Tuple::point(1.0, -1.0, 0.0), p2);

        let p3 = b.tuple_prod(p2);
        assert_eq!(Tuple::point(5.0, -5.0, 0.0), p3);

        let p4 = c.tuple_prod(p3);
        assert_eq!(Tuple::point(15.0, 0.0, 7.0), p4);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        assert_eq!(Tuple::point(15.0, 0.0, 7.0), t.tuple_prod(p));
    }
}
