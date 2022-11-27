mod color;
mod iter;
mod fractal;
mod img;
mod buffer;
mod parallel;

use clap::Parser;
use std::fmt;

pub type Uint = u32;

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
struct Args {
   /// Output file name
   #[arg(short, long, value_name = "FILE", default_value_t = String::from("mandelbrot.png"))]
   output: String,
   /// Pixels width
   #[arg(short, long, default_value_t = 700)]
   width: u32,
   /// Pixels height
   #[arg(short, long, default_value_t = 400)]
   height: u32,
   /// Number of iterations
   #[arg(short, long, default_value_t = num_cpus::get() as u32)]
   threads: u32,
   /// Number of iterations
   #[arg(short, long, default_value_t = 200)]
   iters: Uint,
   /// No greyscales
   #[arg(short, long, default_value_t = false)]
   mono: bool,
   /// Pixel type: gray|rgba
   #[arg(short, long, default_value_t = PixelType::Gray)]
   pixel: PixelType,
   /// Coords: x0,y0,x1,y1
   #[arg(short, long)]
   coords: Option<String>,
   /// Print help information
   #[arg(short = '?', long, action = clap::ArgAction::Help)]
   help: Option<bool>
}

use std::error::Error;

use img::FractalImage;
use image::{save_buffer, ColorType};
use parallel::Parallel;

use std::str::FromStr;

#[derive(Clone)]
enum PixelType {
    Gray, Rgba
}

impl fmt::Display for PixelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PixelType::Gray => "gray",
            PixelType::Rgba => "rgba",
        })
    }
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

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let filename = args.output;
    let pixel    = args.pixel;
    // let pixel    = PixelType::from_str(pixel)?;
    let width    = args.width;
    let height   = args.height;
    // let threads  = value_t!(opts, "THREADS", u32).unwrap_or_else(|_| num_cpus::get() as u32);
    let threads  = args.threads;
    let iters    = args.iters;
    let mono     = args.mono;
    let mut x0: f64 = -2.5;
    let mut y0: f64 = -1.0;
    let mut x1: f64 = 1.0;
    let mut y1: f64 = 1.0;
    if let Some(coords) = args.coords {
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
                    save_buffer(filename, buffer, img.width(), img.height(), ColorType::Rgba8)?;
                },
                PixelType::Gray => {
                    let ref buffer = Parallel::<u8>::to_img_buffer(&img, iters, mono, threads);
                    save_buffer(filename, buffer, img.width(), img.height(), ColorType::L8)?;
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
