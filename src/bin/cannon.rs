use ray_tracer::{Canvas, Colour, Tuple};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

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

static WIDTH: u32 = 400;
static HEIGHT: u32 = 200;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut projectile = Projectile {
        position: Tuple::point(1.0, 1.0, 0.0),
        velocity: Tuple::vector(5.0, 6.0, 0.0).normalise() * 1.0,
    };

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.3, 0.0) * 0.01,
        wind: Tuple::vector(-0.001, 0.0, 0.0),
    };

    let colour = Colour::new(0.1, 1.0, 0.0);

    while projectile.position.y >= 0.0 && projectile.position.x >= 0.0 {
        projectile = tick(&environment, &projectile);

        let x = projectile.position.x as u32;
        let y = HEIGHT - (projectile.position.y as u32).min(HEIGHT);

        if x < WIDTH && y < HEIGHT {
            canvas.write_pixel(x, y, &colour);
        }
    }

    let ppm = canvas.to_ppm();

    let mut output_file = File::create("cannon.ppm").unwrap();
    output_file.write_all(&ppm.into_bytes()).unwrap();
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let new_pos = proj.position + proj.velocity;
    let new_velocity = proj.velocity + env.gravity + env.wind;

    Projectile {
        position: new_pos,
        velocity: new_velocity,
    }
}
