use image::{ImageBuffer, Pixel};

use super::Uint;

use crate::color::Color;
use crate::fractal::Fractal;

pub trait FractalImage<T: Pixel> {
    fn to_img_mono(&self, iters: Uint) -> ImageBuffer<T, Vec<T::Subpixel>>;
    fn to_img_gray(&self, iters: Uint) -> ImageBuffer<T, Vec<T::Subpixel>>;
    fn at_mono(&self, x: u32, y: u32, iters: Uint) -> T;
    fn at_gray(&self, x : u32, y : u32, iters: Uint) -> T;
}

impl<T> FractalImage<T> for Fractal where T: Pixel + Color + 'static {
    fn to_img_mono(&self, iters: Uint) -> ImageBuffer<T, Vec<T::Subpixel>> {
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            self.at_mono(x as u32, y as u32, iters)
        })
    }

    fn to_img_gray(&self, iters: Uint) -> ImageBuffer<T, Vec<T::Subpixel>> {
        ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            self.at_gray(x as u32, y as u32, iters)
        })
    }

    fn at_mono(&self, x: u32, y: u32, iters: Uint) -> T {
        let i = self.at_iter(x, y, iters);
        if i >= iters {
            Color::black()
        }
        else {
            Color::white()
        }

    }

    fn at_gray(&self, x : u32, y : u32, iters: Uint) -> T {
        let i = self.at_iter(x, y, iters);
        Color::gray(1.0 - i as f64 / iters as f64)
    }
}
