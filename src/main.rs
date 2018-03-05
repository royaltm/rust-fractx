extern crate num_traits;
extern crate image;
extern crate num_cpus;
extern crate scoped_threadpool;

#[macro_use(value_t, clap_app, crate_version, crate_authors)]
extern crate clap;

mod color;
mod iter;
mod fractal;
mod img;
mod buffer;
mod parallel;

pub type Uint = u32;

use std::error::Error;
use std::path::Path;
use img::FractalImage;
use image::{save_buffer, Gray, RGBA};
use parallel::Parallel;

use std::str::FromStr;

enum PixelType {
    Gray, Rgba
}

impl FromStr for PixelType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("rgba") {
            Ok(PixelType::Rgba)
        }
        else if s.eq_ignore_ascii_case("gray") || s.eq_ignore_ascii_case("grey") {
            Ok(PixelType::Gray)
        }
        else {
            Err("Expected: gray or rgba pixel type".into())
        }
    }
}

fn run() -> Result<(), Box<Error>> {
    let opts = clap_app!((env!("CARGO_PKG_NAME")) =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@arg FILE: -o --output +takes_value "output file name")
        (@arg WIDTH: -w --width +takes_value "pixels width")
        (@arg HEIGHT: -h --height +takes_value "pixels height")
        (@arg THREADS: -t --threads +takes_value "number of threads")
        (@arg ITERS: -i --iters +takes_value "number of iterations")
        (@arg MONO: -m --mono "black and white, no greyscales")
        (@arg PIXEL: -p --pixel +takes_value "pixel type: gray|rgba")
        (@arg COORDS: -c --coords +takes_value "x0,y0,x1,y1")
    ).get_matches();

    let filename = &Path::new(opts.value_of("FILE").unwrap_or("mandelbrot.png"));
    let pixel    = opts.value_of("PIXEL").unwrap_or("gray");
    let pixel    = PixelType::from_str(pixel)?;
    let width    = value_t!(opts, "WIDTH", u32).unwrap_or(700);
    let height   = value_t!(opts, "HEIGHT", u32).unwrap_or(400);
    let threads  = value_t!(opts, "THREADS", u32).unwrap_or_else(|_| num_cpus::get() as u32);
    let iters    = value_t!(opts, "ITERS", Uint).unwrap_or_else(|_| 200 as Uint);
    let mono     = opts.is_present("MONO");
    let mut x0: f64 = -2.5;
    let mut y0: f64 = -1.0;
    let mut x1: f64 = 1.0;
    let mut y1: f64 = 1.0;
    if let Some(coords) = opts.value_of("COORDS") {
        let mut coords = coords.split(',').map(|v| f64::from_str(v));
        if let Some(v) = coords.next() { x0 = v? }
        if let Some(v) = coords.next() { y0 = v? }
        if let Some(v) = coords.next() { x1 = v? }
        if let Some(v) = coords.next() { y1 = v? }
    }

    let img = fractal::Fractal::from_view_box(width, height, x0, y0, x1, y1);

    match threads {
        0 | 1 => {
            match pixel {
                PixelType::Rgba => {
                    if mono {
                        FractalImage::<image::Rgba<u8>>::to_img_mono(&img, iters).save(filename)?;
                    }
                    else {
                        FractalImage::<image::Rgba<u8>>::to_img_gray(&img, iters).save(filename)?;
                    }
                },
                PixelType::Gray => {
                    if mono {
                        FractalImage::<image::Luma<u8>>::to_img_mono(&img, iters).save(filename)?;
                    }
                    else {
                        FractalImage::<image::Luma<u8>>::to_img_gray(&img, iters).save(filename)?;
                    }
                }
            }
        },
        _     => {
            match pixel {
                PixelType::Rgba => {
                    let ref buffer = vec_u32_as_vec_u8(Parallel::<u32>::to_img_buffer(&img, iters, mono, threads));
                    save_buffer(filename, buffer, img.width(), img.height(), RGBA(8))?;
                },
                PixelType::Gray => {
                    let ref buffer = Parallel::<u8>::to_img_buffer(&img, iters, mono, threads);
                    save_buffer(filename, buffer, img.width(), img.height(), Gray(8))?;
                }
            }
        }
    }
    Ok(())
}

fn vec_u32_as_vec_u8(mut v: Vec<u32>) -> Vec<u8> {
    use std::mem;
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    unsafe {
        // Cast `v` into the void: no destructor run, so we are in
        // complete control of the allocation to which `p` points.
        mem::forget(v);
        let len = len * mem::size_of::<u32>() / mem::size_of::<u8>();
        let cap = cap * mem::size_of::<u32>() / mem::size_of::<u8>();
        // Put everything back together into a Vec
        Vec::from_raw_parts(p as *mut u8, len, cap)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        ::std::process::exit(1);
    }
}
