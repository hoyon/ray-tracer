extern crate ray_tracer;

use ray_tracer::point::Tuple;
use std::fmt;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl fmt::Debug for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos: {:?}, vel: {:?}", self.position, self.velocity)
    }
}

fn main() {
    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(100.0, 3.0, 0.0).normalise(),
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0) * 0.01,
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    println!(
        "projectile: {:?}\nenvironment:; {:?}",
        projectile, environment
    );

    while projectile.position.y > 0.0 {
        projectile = tick(&environment, &projectile);
        println!("{:?}", projectile);
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let new_pos = proj.position + proj.velocity;
    let new_velocity = proj.velocity + env.gravity + env.wind;

    Projectile {
        position: new_pos,
        velocity: new_velocity,
    }
}
