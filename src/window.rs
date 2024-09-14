use minifb::{Key, MouseButton};

use crate::types::{Color, GameTime};

pub struct Window {
    pub buf: Buffer,
    pub running: bool,
    pub time: GameTime,
    mini_window: minifb::Window,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Window {
        let buf = Buffer::new(width, height);
        let mini_window =
            minifb::Window::new(&title, width, height, minifb::WindowOptions::default())
                .expect("Failed to create minifb window");
        let time = GameTime::new();

        Window {
            buf,
            mini_window,
            time,
            running: true,
        }
    }

    pub fn key_down(&self, key: Key) -> bool {
        self.mini_window.is_key_down(key)
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        let (x, y) = self
            .mini_window
            .get_mouse_pos(minifb::MouseMode::Clamp)
            .expect("Failed to get mouse position");
        (x as i32, y as i32)
    }

    pub fn mouse_down(&self, btn: MouseButton) -> bool {
        self.mini_window.get_mouse_down(btn)
    }

    fn update(&mut self) {
        if !self.mini_window.is_open() {
            self.running = false;
        }
        self.time.new_frame();
    }

    pub fn flip(&mut self) {
        self.mini_window
            .update_with_buffer(&self.buf.to_u32_buf(), self.buf.width, self.buf.height)
            .expect("Failed to draw frame buffer update to window");
        self.update();
    }
}

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Buffer {
        Buffer {
            width,
            height,
            pixels: vec![Color::BLACK; (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: i32, y: i32, color: &Color) {
        if x >= self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
            return;
        }
        self.pixels[y as usize * self.width + x as usize].add_layer_above(color.clone());
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        self.pixels
            .get(y * self.width + x)
            .expect("Failed to get pixel color (likely out of bounds)")
    }

    pub fn to_u32_buf(&mut self) -> Vec<u32> {
        let mut buf: Vec<u32> = Vec::with_capacity(self.width * self.height);
        for pixel in self.pixels.iter() {
            buf.push(pixel.r as u32 * 65536 + pixel.g as u32 * 256 + pixel.b as u32)
        }
        buf
    }
}
