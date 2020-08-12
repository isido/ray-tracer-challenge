use crate::intersection;
use crate::intersection::{Computations, Intersection};
use crate::lights::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformation;
use crate::tuple::Tuple;

pub struct World {
    light: Option<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> World {
        World {
            light: None,
            objects: vec![],
        }
    }
    pub fn default() -> World {
        let mut s1 = Sphere::new();
        let m = Material {
            color: Tuple::color(0.8, 1.0, 0.6),
            ambient: 0.0,
            shininess: 0.0,
            diffuse: 0.7,
            specular: 0.2,
        };
        s1.material = m;

        let mut s2 = Sphere::new();
        let t = transformation::scaling(0.5, 0.5, 0.5);
        s2.transform = t;

        World {
            light: Some(PointLight::new(
                Tuple::point(-10.0, 10.0, -10.0),
                Tuple::color(1.0, 1.0, 1.0),
            )),
            objects: vec![s1, s2],
        }
    }
    pub fn contains(&self, s: &Sphere) -> bool {
        self.objects.contains(s)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let vecs: Vec<_> = self.objects.iter().map(|x| x.intersect(ray)).collect();
        let mut vv = vec![];
        for v in vecs {
            vv.extend(v);
        }
        vv.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        vv
    }

    pub fn shade_hit(&self, c: Computations) -> Tuple {
        c.object
            .material
            .lightning(self.light.unwrap(), c.point, c.eyev, c.normalv)
    }

    pub fn color_at(&self, r: &Ray) -> Tuple {
        let is = self.intersect(r);
        match intersection::hit(&is) {
            None => Tuple::color(0.0, 0.0, 0.0),
            Some(i) => {
                let comps = i.prepare_computations(r);
                self.shade_hit(comps)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lights::PointLight;
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transformation;
    use crate::tuple::Tuple;

    #[test]
    fn creating_world() {
        let w = World::new();
        assert_eq!(None, w.light);
        assert_eq!(0, w.objects.len())
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        let mut s1 = Sphere::new();
        let m = Material {
            color: Tuple::color(0.8, 1.0, 0.6),
            ambient: 0.0,
            shininess: 0.0,
            diffuse: 0.7,
            specular: 0.2,
        };
        s1.material = m;

        let mut s2 = Sphere::new();
        let t = transformation::scaling(0.5, 0.5, 0.5);
        s2.transform = t;

        let w = World::default();
        assert_eq!(light, w.light.unwrap());
        assert!(w.contains(&s1));
        assert!(w.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersect(&r);
        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }

    #[test]
    fn shading_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);
        assert_eq!(Tuple::color(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(
            Tuple::point(0.0, 0.25, 0.0),
            Tuple::color(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, &shape);

        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(comps);
        assert_eq!(Tuple::color(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(Tuple::color(0.0, 0.0, 0.0), c);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(Tuple::color(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        let expected = w.objects[1].material.color;
        let mut outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(expected, w.color_at(&r));
    }
}
