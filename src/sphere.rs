use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn intersect(&self, orig_ray: &Ray) -> Vec<Intersection> {
        let ray = orig_ray.transform(&self.transform.inverse());
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
            vec![Intersection::new(t1, self), Intersection::new(t2, self)]
        }
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse().tuple_prod(world_point);
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let world_normal = self
            .transform
            .inverse()
            .transpose()
            .tuple_prod(object_normal);
        let world_normal2 = Tuple::vector(world_normal.0, world_normal.1, world_normal.2);
        world_normal2.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::transformation;
    use crate::tuple::Tuple;

    use std::f64::consts::PI;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
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
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let xs = sphere.intersect(&ray);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(&r);

        assert_eq!(2, xs.len());
        assert_eq!(&s, xs[0].object);
        assert_eq!(&s, xs[1].object);
    }

    #[test]
    fn spheres_default_transformation() {
        let s = Sphere::new();

        assert_eq!(Matrix::identity(), s.transform);
    }

    #[test]
    fn changing_spheres_transformation() {
        let mut s = Sphere::new();
        let t = transformation::translation(2.0, 3.0, 4.0);
        let tt = transformation::translation(2.0, 3.0, 4.0); // TODO make matrices copyable
        s.transform = t;

        assert_eq!(tt, s.transform);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = transformation::scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(&r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = transformation::translation(5.0, 0.0, 0.0);
        let xs = s.intersect(&r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(Tuple::vector(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));

        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(Tuple::vector(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn normal_on_sphere_at_non_axial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));

        assert_eq!(
            Tuple::vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            ),
            n
        )
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = transformation::translation(0.0, 1.0, 0.0);
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));

        assert_eq!(Tuple::vector(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform = transformation::scaling(1.0, 0.5, 1.0) * transformation::rotation_z(PI / 5.0);
        let n = s.normal_at(Tuple::point(
            0.0,
            f64::sqrt(2.0) / 2.0,
            -f64::sqrt(2.0) / 2.0,
        ));

        assert_eq!(Tuple::vector(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        let m = s.material;

        assert_eq!(Material::new(), m);
    }

    #[test]
    fn sphere_may_be_assigned_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;

        assert_eq!(m, s.material);
    }

}
