use scoped_threadpool;
use num_traits::cast::AsPrimitive;

use super::Uint;

use fractal::Fractal;
use buffer::*;

pub trait Parallel<T: ExpandPixel>: Buffer<T> {
    fn to_img_buffer(&self, iters: Uint, mono: bool, concurrency: u32) -> Vec<T>;
    fn write_img_buffer(&self, iters: Uint, mono: bool, concurrency: u32, pixels: &mut [T]);
}

impl<T> Parallel<T> for Fractal
where T: ExpandPixel + Default + Send,
      f64: AsPrimitive<T>, u8: AsPrimitive<T>
{
    fn write_img_buffer(&self, iters: Uint, mono: bool, concurrency: u32, pixels: &mut [T]) {
        let to_pixel = if mono { mono_pixel::<T> } else { gray_pixel::<T> };
        let mut pool = scoped_threadpool::Pool::new(concurrency);

        pool.scoped(|scope| {
            for (y, line) in pixels.chunks_mut(self.width() as usize).enumerate() {
                scope.execute(move || write_line(self, y as u32, line, iters, &to_pixel));
            }
        });
    }

    fn to_img_buffer(&self, iters: Uint, mono: bool, concurrency: u32) -> Vec<T> {
        let mut pixels = self.create_img_buffer();
        Parallel::<T>::write_img_buffer(self, iters, mono, concurrency, &mut pixels);
        return pixels;
    }
}
