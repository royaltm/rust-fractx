use scoped_threadpool;

use super::Uint;

use fractal::{MAXI, MAXIF, Fractal};

pub trait Multi {
    fn to_img_buffer(&self, concurrency: u32, mono: bool) -> Vec<u8>;
}

impl Multi for Fractal {

    fn to_img_buffer(&self, concurrency: u32, mono: bool) -> Vec<u8> {
        let size = (self.width()*self.height()) as usize;
        let mut pixels = vec![0u8; size];
        let mut pool = scoped_threadpool::Pool::new(concurrency);
        let ref to_pixel = if mono { mono_pixel } else { gray_pixel };

        pool.scoped(|scope| {
            for (y, line) in pixels.chunks_mut(self.width() as usize).enumerate() {
                scope.execute(move || write_line(self, y as u32, line, &to_pixel));
            }
        });
        return pixels;
    }
}

fn gray_pixel(i: Uint) -> u8 {
    (255f64 * (1.0 - i as f64 / MAXIF)) as u8
}

fn mono_pixel(i: Uint) -> u8 {
    (if i >= MAXI { 0 } else { u8::max_value() }) as u8
}

fn write_line(frac: &Fractal, y: u32, line: &mut [u8], to_pixel: &Fn(Uint) -> u8) {
    for (x, p) in line.iter_mut().enumerate() {
        let i  = frac.at_iter(x as u32, y as u32);
        *p = to_pixel(i);
    }
}
