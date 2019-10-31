use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub intensity: Tuple,
    pub position: Tuple,
}

pub fn point_light(pos: Tuple, inte: Tuple) -> PointLight {
    PointLight {
        position: pos,
        intensity: inte,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Tuple::color(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = point_light(position, intensity);

        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
