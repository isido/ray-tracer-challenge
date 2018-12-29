use crate::tuple::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    canvas: Vec<Tuple>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        let c = vec![Tuple::color(0.0, 0.0, 0.0); w * h];
        Canvas {
            width: w,
            height: h,
            canvas: c,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        self.canvas[x + y * self.width]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Tuple) {
        self.canvas[x + y * self.width] = c;
    }

    pub fn to_ppm(&self) -> String {
        fn ppm_color(x: f64) -> i32 {
            let v = (x * 255.0).round() as i32;
            if v < 0 {
                0
            } else if v > 255 {
                255
            } else {
                v
            }
        }

        fn colors_to_ppm_string(v: &[i32]) -> String {
            let mut pos = 0;
            let mut s = String::new();
            for i in v {
                let n = format!("{}", i);
                if pos + n.len() >= 68 {
                    s.push('\n');
                    pos = 0;
                }
                if pos != 0 {
                    s.push(' ');
                    pos += 1;
                }
                s.push_str(&n);
                pos += n.len();
            }
            s
        }

        // header
        let mut s = format!("P3\n{} {}\n255\n", self.width, self.height);

        // content
        for j in 0..self.height {
            let mut v: Vec<i32> = Vec::new();
            for i in 0..self.width {
                let c = self.pixel_at(i, j);
                v.push(ppm_color(c.0));
                v.push(ppm_color(c.1));
                v.push(ppm_color(c.2));
            }
            s.push_str(&colors_to_ppm_string(&v));
            s.push('\n');
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_canvas() {
        let c = Canvas::new(10, 20);
        let col = Tuple::color(0.0, 0.0, 0.0);

        assert_eq!(10, (&c).width);
        assert_eq!(20, (&c).height);
        for i in 0..10 {
            for j in 0..20 {
                assert_eq!(col, c.pixel_at(i, j));
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Tuple::color(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(red, c.pixel_at(2, 3));
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm().lines().take(3).collect::<Vec<&str>>().join("\n");
        assert_eq!("P3\n5 3\n255", ppm);
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Tuple::color(1.5, 0.0, 0.0);
        let c2 = Tuple::color(0.0, 0.5, 0.0);
        let c3 = Tuple::color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c
            .to_ppm()
            .lines()
            .skip(3)
            .take(3)
            .collect::<Vec<&str>>()
            .join("\n");

        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", ppm);
    }

    #[test]
    fn splitting_long_lines_in_pmm_files() {
        let mut c = Canvas::new(10, 2);
        let col = Tuple::color(1.0, 0.8, 0.6);
        for i in 0..10 {
            for j in 0..2 {
                c.write_pixel(i, j, col);
            }
        }

        let ppm = c
            .to_ppm()
            .lines()
            .skip(3)
            .take(4)
            .collect::<Vec<&str>>()
            .join("\n");

        assert_eq!("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153", ppm);
    }

    #[test]
    fn ppm_files_are_terminated_by_newline() {
        let c = Canvas::new(5, 3);
        let mut ppm = c.to_ppm();

        assert_eq!(Some('\n'), ppm.pop());
    }

}
