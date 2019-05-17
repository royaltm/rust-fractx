fractx
======

Rust translation of https://github.com/wkhere/fractx

![Mandelbrot](mandelbrot_gray.png?raw=true "Mandelbrot")


Install
-------

```
git clone https://github.com/royaltm/rust-fractx.git
cd rust-fractx
cargo install --path .
```


```
USAGE:
    fractx [FLAGS] [OPTIONS]

FLAGS:
    -m, --mono       black and white, no greyscales
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --coords <COORDS>      x0,y0,x1,y1
    -o, --output <FILE>        output file name
    -h, --height <HEIGHT>      pixels height
    -i, --iters <ITERS>        number of iterations
    -p, --pixel <PIXEL>        pixel type: gray|rgba
    -t, --threads <THREADS>    number of threads
    -w, --width <WIDTH>        pixels width
```

Library
-------

Add to `Cargo.toml:`:

```toml
[dependencies.fractx]
version = "0.2"
git = "https://github.com/royaltm/rust-fractx"
default-features = false
features = ["image-buffer", "parallel"]
```

Basic example using `Vec` for an array of pixels:

```rust
extern crate fractx;

use fractx::buffer::Buffer;
use fractx::fractal::Fractal;

fn main() {
  /* create struct */
  let frac = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);

  /* draw monochromatic fractal pixels into a new buffer */
  let mut pixels = Buffer::<u8>::to_img_buffer(&frac, 200, true);

  /* draw grayscale fractal pixels using the same buffer */
  Buffer::<u8>::write_img_buffer(&frac, 200, false, &mut pixels);

  /* create rgba pixel buffer using Fractal struct as a guide */
  let pixels_rgba = Buffer::<u32>::create_img_buffer(&frac);

  /* draw grayscale rgba fractal pixels into separately created buffer */
  Buffer::<u32>::write_img_buffer(&frac, 200, false, &mut pixels_rgba);
}
```

The optional `"image-buffer"` feature gives you access to the `FractalImage` trait so you can create fractals into ImageBuffer instances from the [image](https://crates.io/crates/image) crate:

```rust
extern crate fractx;
extern crate image;

use fractx::fractal::Fractal;
use fractx::img::FractalImage;
use image::{Rgb, Rgba, Luma, LumaA};

fn main() {
  let frac = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);
  let img_buffer = FractalImage::<LumaA<u8>>::to_img_mono(&frac, 200);
  img_buffer.save("mandelbrot.png").unwrap();
}
```

Another optional `"parallel"` feature gives you access to `Parallel` trait which renders fractals using [scoped thread pool](https://crates.io/crates/scoped_threadpool):

```rust
extern crate fractx;
extern crate image;

use fractx::fractal::Fractal;
use fractx::parallel::Parallel;
use image::{save_buffer, Gray};

fn main() {
  /* create struct */
  let frac = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);

  /* render using 8 threads */
  let ref buffer = Parallel::<u8>::to_img_buffer(&frac, 200, false, 8);

  /* save as an image */
  save_buffer("mandelbrot.png", buffer, frac.width(), frac.height(), Gray(8)).unwrap();
}
```
