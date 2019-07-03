use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }

}
