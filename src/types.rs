use std::time::Instant;

#[derive(Debug)]
pub struct GameTime {
    last_frame: Instant,
    pub delta: f32,
}

impl GameTime {
    pub fn new() -> GameTime {
        GameTime {
            last_frame: Instant::now(),
            delta: 0.0,
        }
    }

    pub fn new_frame(&mut self) {
        self.delta = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    pub fn add_layer_above(&mut self, other: Color) {
        let other_alpha = other.a as f64 / 255.0;
        let old_alpha = 1.0 - other_alpha;
        self.r = (self.r as f64 * old_alpha + other.r as f64 * other_alpha) as u8;
        self.g = (self.g as f64 * old_alpha + other.g as f64 * other_alpha) as u8;
        self.b = (self.b as f64 * old_alpha + other.b as f64 * other_alpha) as u8;
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(path: &str, width: usize, height: usize) -> Image {
        let mut ril_image =
            ril::Image::<ril::Rgba>::open(path).expect(&format!("Failed to load image {}", path));
        ril_image.resize(width as u32, height as u32, ril::ResizeAlgorithm::Nearest);

        let mut pixels: Vec<Color> = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let ril_pixel = ril_image
                    .get_pixel(x as u32, y as u32)
                    .expect(&format!("Failed to get pixel of ril::Image {}", path));
                let pixel = Color::rgba(ril_pixel.r, ril_pixel.g, ril_pixel.b, ril_pixel.a);
                pixels.push(pixel);
            }
        }

        Image {
            width,
            height,
            pixels,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        self.pixels.get(y * self.width + x).expect(&format!(
            "Could not get pixel x={}, y={} of image (likely out of bounds)",
            x, y
        ))
    }
}
