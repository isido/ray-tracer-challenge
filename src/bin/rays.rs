extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::intersection;
use ray_tracer_challenge::lights::PointLight;
use ray_tracer_challenge::ray::Ray;
use ray_tracer_challenge::sphere::Sphere;
use ray_tracer_challenge::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::new();
    shape.material.color = Tuple::color(1.0, 0.2, 1.0);

    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);

            if let Some(hit) = intersection::hit(&xs) {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -r.direction;
                let color = hit.object.material.lightning(light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }
    println!("{}", canvas.to_ppm());
}
