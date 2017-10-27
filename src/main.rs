#![feature(asm)]
extern crate image;
extern crate num;
extern crate num_cpus;
extern crate scoped_threadpool;

#[macro_use(value_t, clap_app)]
extern crate clap;

mod color;
mod iter;
mod fractal;
mod multi;

pub type Uint = u32;

use image::{save_buffer, Gray};
use multi::Multi;

fn main() {
    let opts = clap_app!(fractx =>
        (version: "1.0")
        (about: "All your base are belong to z*z + c.")
        (@arg FILE: -o --output +takes_value "output file name")
        (@arg WIDTH: -w --width +takes_value "pixels width")
        (@arg HEIGHT: -h --height +takes_value "pixels height")
        (@arg THREADS: -t --threads +takes_value "number of threads")
        (@arg MONO: -m --mono "black and white, no greyscales")
    ).get_matches();

    let filename = opts.value_of("FILE").unwrap_or("mandelbrot.png");
    let width    = value_t!(opts, "WIDTH", u32).unwrap_or(700);
    let height   = value_t!(opts, "HEIGHT", u32).unwrap_or(400);
    let threads  = value_t!(opts, "THREADS", u32).unwrap_or_else(|_| num_cpus::get() as u32);
    let mono     = opts.is_present("MONO");

    let img = fractal::new_fractal(width, height, -2.5, -1.0, 1.0, 1.0);

    match threads {
        0 | 1 => {
            if mono {
                img.to_img::<image::Luma<u8>>().save(filename).unwrap();
            }
            else {
                img.to_img_gray::<image::Luma<u8>>().save(filename).unwrap();
            }
        },
        _     => {
            let ref buffer = img.to_img_buffer(threads, mono);
            save_buffer(filename, buffer, img.width(), img.height(), Gray(8)).unwrap()
        }
    }
}
