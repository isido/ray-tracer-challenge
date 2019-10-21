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
}

#[cfg(test)]
mod tests {
    use super::*;
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

}
