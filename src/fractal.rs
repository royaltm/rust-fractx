#![allow(dead_code)]
use iter::iter;
use image::{ImageBuffer, Pixel};

use super::Uint;

use color::Color;

pub const MAXI : Uint = 200;
pub const MAXIF : f64 = MAXI as f64;

pub struct Fractal {
    w : u32,
    h : u32,
    x0 : f64,
    y0 : f64,
    dx : f64,
    dy : f64,
}

pub fn new_fractal(w: u32, h: u32, x0: f64, y0: f64, x1: f64, y1: f64) -> Fractal {
    Fractal {
        w, h,
        x0, y0,
        dx: (x1 - x0) / (w as f64), dy: (y1 - y0) / (h as f64),
    }
}

impl Fractal {
    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn to_img<T>(&self) -> ImageBuffer<T, Vec<T::Subpixel>> where T: Pixel + Color + 'static {
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            self.at(x as Uint, y as Uint)
        })
    }

    pub fn to_img_gray<T>(&self) -> ImageBuffer<T, Vec<T::Subpixel>> where T: Pixel + Color + 'static {
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            self.at_gray(x as Uint, y as Uint)
        })
    }

    pub fn at<T>(&self, x : Uint, y : Uint) -> T where T: Pixel + Color + 'static {
        let i  = iter(self.x0+(x as f64)*self.dx, self.y0+(y as f64)*self.dy);
        if i >= MAXI {
            Color::black()
        }
        else {
            Color::white()
        }

    }

    pub fn at_gray<T>(&self, x : Uint, y : Uint) -> T where T: Pixel + Color + 'static {
        let i  = iter(self.x0+(x as f64)*self.dx, self.y0+(y as f64)*self.dy);
        Color::gray(1.0 - i as f64 / MAXIF)
    }
}
