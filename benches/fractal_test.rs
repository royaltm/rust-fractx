#![feature(test)]

extern crate test;
extern crate image;
extern crate fractx;

use test::Bencher;
use fractx::fractal::Fractal;
use fractx::img::FractalImage;
use image::{Rgb, Rgba, Luma, LumaA};

#[bench]
fn bench_rgba(ben: &mut Bencher) {
    let img = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        FractalImage::<Rgba<u8>>::to_img_mono(&img, 200);
    });
}

#[bench]
fn bench_rgb(ben: &mut Bencher) {
    let img = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        FractalImage::<Rgb<u8>>::to_img_mono(&img, 200);
    });
}

#[bench]
fn bench_luma(ben: &mut Bencher) {
    let img = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        FractalImage::<Luma<u8>>::to_img_mono(&img, 200);
    });
}

#[bench]
fn bench_luma_a(ben: &mut Bencher) {
    let img = Fractal::from_view_box(700, 400, -2.5, -1.0, 1.0, 1.0);
    ben.iter(|| {
        FractalImage::<LumaA<u8>>::to_img_mono(&img, 200);
    });
}
