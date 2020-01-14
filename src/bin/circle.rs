use ray_tracer::{Canvas, Colour, Tuple, Ray, Sphere, sphere};
use std::fs::File;
use std::io::prelude::*;

static WIDTH: u32 = 400;
static HEIGHT: u32 = 400;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let sphere = Sphere::new();
    let light = Tuple::point(0.0, 0.0, -3.0);

    let ratio = 400.0 / 6.0;

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let direction = Tuple::vector(
                (x as f32) / ratio - 3.0,
                (y as f32) / ratio - 3.0,
                4.0
            );
            let ray = Ray::new(light, direction);
            let xs = sphere.intersect(&ray);

            if xs.len() != 0 {
                let hit = sphere::hit(&xs);
                let t = hit.unwrap().t;
                let colour = Colour::new(t, t, t);
                canvas.write_pixel(x, y, &colour);
            }
        }
    }

    let ppm = canvas.to_ppm();

    let mut output_file = File::create("circle.ppm").unwrap();
    output_file.write_all(&ppm.into_bytes()).unwrap();
}
