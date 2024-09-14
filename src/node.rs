use crate::{
    types::{Color, Image},
    window::Window,
};

// TODO: Only draw nodes that are visible (on screen).
// TODO: Collision detection with the Position and Size trait.

pub trait Node {
    fn draw(&self, window: &mut Window);
}

pub trait Position {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
    fn set_x(&mut self, x: i32);
    fn set_y(&mut self, y: i32);
}

pub trait Size {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn set_width(&mut self, width: i32);
    fn set_height(&mut self, height: i32);
}

pub struct Fill {
    pub color: Color,
}

impl Fill {
    pub fn new(color: Color) -> Fill {
        Fill { color }
    }
}

impl Node for Fill {
    fn draw(&self, window: &mut Window) {
        for x in 0..window.buf.width {
            for y in 0..window.buf.height {
                window.buf.set(x as i32, y as i32, &self.color)
            }
        }
    }
}

pub struct Pixel {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: i32, y: i32, color: Color) -> Pixel {
        Pixel { x, y, color }
    }
}

impl Node for Pixel {
    fn draw(&self, window: &mut Window) {
        window.buf.set(self.x, self.y, &self.color);
    }
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub color: Color,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Node for Rect {
    fn draw(&self, window: &mut Window) {
        for x in self.x..self.x + self.width {
            for y in self.y..self.y + self.height {
                window.buf.set(x, y, &self.color);
            }
        }
    }
}

impl Position for Rect {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y
    }
}

impl Size for Rect {
    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

pub struct Sprite<'a> {
    pub x: i32,
    pub y: i32,
    pub image: &'a Image,
}

impl<'a> Sprite<'a> {
    pub fn new(x: i32, y: i32, image: &Image) -> Sprite {
        Sprite { x, y, image }
    }
}

impl<'a> Node for Sprite<'a> {
    fn draw(&self, window: &mut Window) {
        for x in 0..self.image.width {
            for y in 0..self.image.height {
                let screen_x = x as i32 + self.x;
                let screen_y = y as i32 + self.y;
                let color = self.image.get(x, y);
                window.buf.set(screen_x, screen_y, color);
            }
        }
    }
}

impl<'a> Position for Sprite<'a> {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y
    }
}

impl<'a> Size for Sprite<'a> {
    fn get_width(&self) -> i32 {
        self.image.width as i32
    }

    fn get_height(&self) -> i32 {
        self.image.height as i32
    }

    fn set_width(&mut self, _width: i32) {
        panic!("Cannot set size of Sprite node");
    }

    fn set_height(&mut self, _height: i32) {
        panic!("Cannot set size of Sprite node");
    }
}
