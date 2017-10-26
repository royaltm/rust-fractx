#![feature(test)]

extern crate test;
extern crate image;
extern crate fractx;

use test::Bencher;
use fractx::fractal::new_fractal;
use image::{Rgb, Rgba, Luma, LumaA};

#[bench]
fn bench_rgba(ben: &mut Bencher) {
    let img = new_fractal(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        img.to_img::<Rgba<u8>>();
    });
}

#[bench]
fn bench_rgb(ben: &mut Bencher) {
    let img = new_fractal(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        img.to_img::<Rgb<u8>>();
    });
}

#[bench]
fn bench_luma(ben: &mut Bencher) {
    let img = new_fractal(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        img.to_img::<Luma<u8>>();
    });
}

#[bench]
fn bench_luma_a(ben: &mut Bencher) {
    let img = new_fractal(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        img.to_img::<LumaA<u8>>();
    });
}
