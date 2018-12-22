extern crate ray_tracer_challenge;

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
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize(),
    };
    let mut ticks = 0;

    println!("Initial position {:?} ", proj.position);
    while proj.position.1 > 0.0 {
        proj = tick(&env, &proj);
        ticks += 1;
        println!("Tick #{:?}, position {:?} ", ticks, proj.position);
    }
}
