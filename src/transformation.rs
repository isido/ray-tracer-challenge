use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let v = vec![
        1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
    ];
    Matrix::from_vector(4, &v)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
