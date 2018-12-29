extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() {
    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);
    let col = Tuple::color(0.5, 0.5, 0.5);

    while proj.position.1 > 0.0 {
        let x = proj.position.0.round() as usize;
        let y = canvas.height - (proj.position.1.round() as usize);

        if x < canvas.width && y < canvas.height {
            canvas.write_pixel(x, y, col);
        }

        proj = tick(&env, &proj);
    }
    println!("{}", canvas.to_ppm());
}
