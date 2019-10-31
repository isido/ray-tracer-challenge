use crate::lights::PointLight;
use crate::tuple;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lightning(self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Tuple {
        let effective_color = self.color.hadamard(light.intensity);
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);
        let diffuse;
        let specular;

        if light_dot_normal < 0.0 {
            diffuse = Tuple::color(0.0, 0.0, 0.0);
            specular = Tuple::color(0.0, 0.0, 0.0);
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = tuple::reflect(-lightv, normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                specular = Tuple::color(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lights;
    use crate::tuple::Tuple;

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(Tuple::color(1.0, 1.0, 1.0), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200.0, m.shininess);
    }

    #[test]
    fn lightning_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = lights::point_light(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lightning(light, position, eyev, normalv);

        assert_eq!(Tuple::color(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lightning_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light =
            lights::point_light(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lightning(light, position, eyev, normalv);

        assert_eq!(Tuple::color(0.7364, 0.7364, 0.7364), result);
    }

    #[test]
    fn lightning_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light =
            lights::point_light(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lightning(light, position, eyev, normalv);

        assert_eq!(Tuple::color(1.6364, 1.6364, 1.6364), result);
    }

    #[test]
    fn lightning_with_light_behind_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = lights::point_light(Tuple::point(0.0, 0.0, 10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lightning(light, position, eyev, normalv);

        assert_eq!(Tuple::color(0.1, 0.1, 0.1), result);
    }

}
