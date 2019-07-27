use crate::colour::Colour;

pub struct Canvas {
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

    pub fn to_ppm(&self) -> String {
        let header = ppm_header(self.width, self.height);

        let mut data = String::new();

        let mut row_numbers = Vec::with_capacity((self.width * 3) as usize);

        for row in 0..self.height {
            for col in 0..self.width {
                let colour_strings = format_colour(&self.read_pixel(col, row));
                row_numbers.extend_from_slice(&colour_strings);
            }
            data.push_str(&combine_numbers(&row_numbers));
            data.push_str("\n");
            row_numbers.clear();
        }

        header + &data
    }
}

fn ppm_header(width: u32, height: u32) -> String {
    format!("P3\n{width} {height}\n255\n", width=width, height=height)
}

fn format_colour(colour: &Colour) -> [String; 3] {
    let r = convert_pixel(colour.r).to_string();
    let g = convert_pixel(colour.g).to_string();
    let b = convert_pixel(colour.b).to_string();
    [r, g, b]
}

fn convert_pixel(pixel: f32) -> u8 {
    (pixel.min(1.0).max(0.0) * 255.0).round() as u8
}

fn combine_numbers(numbers: &[String]) -> String {
    let mut lines = Vec::new();
    let mut current_line = String::with_capacity(70);
    for n in numbers {
        if current_line.len() + n.len() > 70 {
            lines.push(current_line.trim().to_owned());
            current_line.clear();
        }

        current_line.push_str(n);
        current_line.push_str(" ");
    }

    let trimmed = current_line.trim();
    if trimmed != "" {
        lines.push(trimmed.to_owned());
    }

    lines.join("\n")
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

    #[test]
    fn test_to_ppm_writes_ppm_header() {
        let canvas = Canvas::new(2, 2);
        let ppm = canvas.to_ppm();
        assert!(ppm.starts_with("P3\n2 2\n255\n"))
    }

    #[test]
    fn test_to_ppm_writes_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Colour::new(1.5, 0.0, 0.0);
        let c2 = Colour::new(0.0, 0.5, 0.0);
        let c3 = Colour::new(0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, &c1);
        canvas.write_pixel(2, 1, &c2);
        canvas.write_pixel(4, 2, &c3);

        let ppm = canvas.to_ppm();
        let mut iter = ppm.lines().skip(3);

        let expected_1 = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0";
        let expected_2 = "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0";
        let expected_3 = "0 0 0 0 0 0 0 0 0 0 0 0 128 0 255";

        assert_eq!(iter.next(), Some(expected_1));
        assert_eq!(iter.next(), Some(expected_2));
        assert_eq!(iter.next(), Some(expected_3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_to_ppm_splits_long_lines() {
        let mut canvas = Canvas::new(10, 2);
        let c = Colour::new(1.0, 0.8, 0.6);
        for i in 0..10 {
            for j in 0..2 {
                canvas.write_pixel(i, j, &c);
            }
        }

        let ppm = canvas.to_ppm();
        let mut iter = ppm.lines().skip(3);

        let expected_1 = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204";
        let expected_2 = "153 255 204 153 255 204 153 255 204 153 255 204 153";

        assert_eq!(iter.next(), Some(expected_1));
        assert_eq!(iter.next(), Some(expected_2));
        assert_eq!(iter.next(), Some(expected_1));
        assert_eq!(iter.next(), Some(expected_2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_to_ppm_has_trailing_newline() {
        let canvas = Canvas::new(10, 2);

        let ppm = canvas.to_ppm();
        assert!(ppm.ends_with("\n"));
    }
}
