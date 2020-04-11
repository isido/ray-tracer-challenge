use crate::intersection::Intersection;
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
