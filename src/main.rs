extern crate image;
extern crate num;

mod color;
mod iter;
mod fractal;

pub type Uint = u32;

use std::path::Path;

fn main() {
  let img = fractal::new_fractal(700, 400, -2.5, -1.0, 1.0, 1.0);

  let filename = Path::new("mandelbrot.png");
  img.to_img::<image::Rgba<u8>>().save(filename).unwrap();

  let filename = Path::new("mandelbrot_gray.png");
  img.to_img_gray::<image::Rgba<u8>>().save(filename).unwrap();
}
