struct Tuple(f64, f64, f64, f64);

impl Tuple {
    fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_point() {
        // A tuple with w=1.0 is a point
        let t = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(t.0, 4.3);
        assert_eq!(t.1, -4.2);
        assert_eq!(t.2, 3.1);
        assert_eq!(t.3, 1.0);
    }

    #[test]
    fn tuple_vector() {
        // A tuple with w=0 is a vector
        let t = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(t.0, 4.3);
        assert_eq!(t.1, -4.2);
        assert_eq!(t.2, 3.1);
        assert_eq!(t.3, 0.0);
    }
}
