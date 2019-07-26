use crate::colour::Colour;

struct Canvas {
    pixels: Vec<Colour>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height;
        let black = Colour::new(0.0, 0.0, 0.0);
        Canvas {
            pixels: vec![black; pixel_count as usize],
            width,
            height,
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, colour: &Colour) {
        let index = y * self.width + x;
        self.pixels[index as usize] = *colour;
    }

    pub fn read_pixel(&self, x: u32, y: u32) -> Colour {
        let index = y * self.width + x;
        self.pixels[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_makes_all_black_canvas() {
        let canvas = Canvas::new(10, 20);
        let black = Colour::new(0.0, 0.0, 0.0);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        assert_eq!(canvas.pixels.len(), 200);

        for pixel in canvas.pixels {
            assert_eq!(pixel, black);
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::new(2, 2);
        let red = Colour::new(1.0, 0.0, 0.0);

        canvas.write_pixel(1, 1, &red);
        assert_eq!(canvas.pixels[3], red);

        canvas.write_pixel(0, 1, &red);
        assert_eq!(canvas.pixels[2], red);
    }

    #[test]
    fn test_read_pixel() {
        let mut canvas = Canvas::new(2, 2);
        let red = Colour::new(1.0, 0.0, 0.0);

        canvas.pixels[3] = red;
        assert_eq!(canvas.read_pixel(1, 1), red);

        canvas.pixels[2] = red;
        assert_eq!(canvas.read_pixel(0, 1), red);
    }
}
