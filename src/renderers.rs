pub mod ppm;
use std::ops::{Index, IndexMut};

pub use ppm::PPM;

pub trait Renderer {
    fn draw(self, image: Image);
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Image {
    pixels: Vec<Vec<Color>>,
    width: usize,
    height: usize,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

#[derive(Debug, Clone, Copy)]
/// First index is `y` coordinate, the second is `x`
pub struct Idx2D(pub usize, pub usize);

impl Image {
    /// Creates a black image with the specified width and height
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            pixels: vec![vec![Color::new(0, 0, 0); width]; height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y][x]
    }

    /// Fills rectangle with the upper-left corner at `(x,y)` and with the provided width
    /// and height
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        if y + height >= self.height || x + width >= self.width || x < 0 || y < 0 {
            panic!(
                "Rectangle at ({}, {}) with size ({}, {}) out of bounds",
                x, y, width, height
            );
        }
        self.pixels.iter_mut().skip(y).take(height).for_each(|row| {
            row.iter_mut()
                .skip(x)
                .take(width)
                .for_each(|pixel| *pixel = color);
        });
    }
}
