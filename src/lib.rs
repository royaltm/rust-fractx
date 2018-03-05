extern crate num_traits;

#[cfg(feature = "image-buffer")]
extern crate image;

#[cfg(feature = "parallel")]
extern crate num_cpus;

#[cfg(feature = "parallel")]
extern crate scoped_threadpool;

mod iter;
pub mod fractal;
pub mod buffer;

#[cfg(feature = "image-buffer")]
pub mod color;

#[cfg(feature = "image-buffer")]
pub mod img;

#[cfg(feature = "parallel")]
pub mod parallel;

pub type Uint = u32;
