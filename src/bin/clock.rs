extern crate ray_tracer_challenge;

use std::f64::consts::PI;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::transformation;
use ray_tracer_challenge::tuple::Tuple;

// z-axis is up, x-axis is right, canvas takes points from ]-1.0, 1.0[
fn point_to_canvas(canvas: &Canvas, point: &Tuple) -> (usize, usize) {
    assert!(point.0 < 1.0 && point.0 > -1.0);
    assert!(point.2 < 1.0 && point.2 > -1.0);

    let w = canvas.width as f64 / 2.0;
    let h = canvas.width as f64 / 2.0;

    let x = (w * point.0) + w;
    let y = (h * point.2) + h;
    (x as usize, y as usize)
}

fn main() {
    let width = 40;
    let height = 40;

    let mut canvas = Canvas::new(width, height);
    let twelve = Tuple::point(0.0, 0.0, 3.0 / 4.0);
    let color = Tuple::color(0.5, 0.5, 0.5);

    let (x, y) = point_to_canvas(&canvas, &twelve);
    canvas.write_pixel(x, y, color);

    for i in 1..12 {
        let r = transformation::rotation_y(i as f64 * (PI / 6.0));
        let p = r.tuple_prod(twelve);
        let (x, y) = point_to_canvas(&canvas, &p);
        //        println!("x: {}, y: {}", x, y);
        canvas.write_pixel(x, y, color);
    }

    println!("{}", canvas.to_ppm());
}
