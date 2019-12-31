use ray_tracer::{Canvas, Colour, Tuple, Matrix};
use std::fs::File;
use std::io::prelude::*;
use std::f32::consts::PI;

static WIDTH: u32 = 400;
static HEIGHT: u32 = 400;

fn draw_pixel(canvas: &mut Canvas, pixel: &Tuple) {
    let white = Colour::new(1.0, 1.0, 1.0);
    let x = pixel.x as u32;
    let y = pixel.y as u32;

    canvas.write_pixel(x, y, &white);
    canvas.write_pixel(x + 1, y, &white);
    canvas.write_pixel(x, y + 1, &white);
    canvas.write_pixel(x + 1, y + 1, &white);
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    for i in 0..12 {
        let transformation =
            Matrix::identity()
            .translate(0.0, -150.0, 0.0)
            .rotate_z(PI / 6.0 * i as f32)
            .translate(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 0.0);

        let pixel = transformation * Tuple::point(0.0, 0.0, 0.0);
        draw_pixel(&mut canvas, &pixel);
    }

    let ppm = canvas.to_ppm();

    let mut output_file = File::create("clock.ppm").unwrap();
    output_file.write_all(&ppm.into_bytes()).unwrap();
}
