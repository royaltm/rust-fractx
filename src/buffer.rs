use super::Uint;

use num_traits::cast::AsPrimitive;
use num_traits::int::PrimInt;
use crate::fractal::Fractal;

const ALPHA_MAX: u32 = 0xff << 24;

pub trait ExpandPixel: PrimInt + 'static {
    fn expand_pixel8(self) -> Self;
}

impl ExpandPixel for u32 {
    fn expand_pixel8(self) -> u32 {
        (ALPHA_MAX | self | (self << 8) | (self << 16)).to_le()
    }
}

impl ExpandPixel for u8 {
    fn expand_pixel8(self) -> u8 {
        self
    }
}

pub trait Buffer<T: ExpandPixel> {
    fn create_img_buffer(&self) -> Vec<T>;
    fn write_img_buffer(&self, iters: Uint, mono: bool, pixels: &mut [T]);
    fn to_img_buffer(&self, iters: Uint, mono: bool) -> Vec<T>;
}

impl<T> Buffer<T> for Fractal
where T: ExpandPixel + Default,
      f64: AsPrimitive<T>, u8: AsPrimitive<T>
{
    fn create_img_buffer(&self) -> Vec<T> {
        let size = (self.width()*self.height()) as usize;
        vec![Default::default(); size]
    }

    fn write_img_buffer(&self, iters: Uint, mono: bool, pixels: &mut [T]) {
        let to_pixel = if mono { mono_pixel::<T> } else { gray_pixel::<T> };

        for (y, line) in pixels.chunks_mut(self.width() as usize).enumerate() {
            write_line(self, y as u32, line, iters, &to_pixel);
        }
    }

    fn to_img_buffer(&self, iters: Uint, mono: bool) -> Vec<T> {
        let mut pixels = self.create_img_buffer();
        self.write_img_buffer(iters, mono, &mut pixels);
        return pixels;
    }
}

pub fn gray_pixel<T>(v: f64) -> T
where T: ExpandPixel,
      f64: AsPrimitive<T>
{
    let v = (255.0 * (1.0 - v)).as_();
    v.expand_pixel8()
}

pub fn mono_pixel<T>(v: f64) -> T
where T: ExpandPixel,
      u8: AsPrimitive<T>
{
    let v = (if v >= 0.5 { 0 } else { u8::max_value() }).as_();
    v.expand_pixel8()
}

pub fn write_line<T, F>(frac: &Fractal, y: u32, line: &mut [T], iters: Uint, to_pixel: &F)
where F: Fn(f64) -> T
{
    for (x, p) in line.iter_mut().enumerate() {
        let i  = frac.at_iter(x as u32, y as u32, iters);
        *p = to_pixel(i as f64 / iters as f64);
    }
}
