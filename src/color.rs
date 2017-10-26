use image::*;
use num::traits;

pub trait Color {
    fn black() -> Self;
    fn white() -> Self;
    fn gray(value: f64) -> Self;
}

impl<T: Primitive + 'static> Color for Rgba<T>
    where f64: From<T>
{
    fn black() -> Rgba<T> {
        Rgba([T::zero(),T::zero(),T::zero(),
              T::max_value()])
    }

    fn white() -> Rgba<T> {
        Rgba([T::max_value(),
              T::max_value(),
              T::max_value(),
              T::max_value()])
    }

    fn gray(value: f64) -> Rgba<T> {
        let n = traits::cast(value * f64::from(T::max_value())).unwrap();
        Rgba([n,n,n, T::max_value()])
    }
}

impl<T: Primitive + 'static> Color for Rgb<T>
    where f64: From<T>
{
    fn black() -> Rgb<T> {
        Rgb([T::zero(),T::zero(),T::zero()])
    }

    fn white() -> Rgb<T> {
        Rgb([T::max_value(),
             T::max_value(),
             T::max_value()])
    }

    fn gray(value: f64) -> Rgb<T> {
        let n = traits::cast(value * f64::from(T::max_value())).unwrap();
        Rgb([n,n,n])
    }
}

impl<T: Primitive + 'static> Color for Luma<T>
    where f64: From<T>
{
    fn black() -> Luma<T> {
        Luma([T::zero()])
    }

    fn white() -> Luma<T> {
        Luma([T::max_value()])
    }

    fn gray(value: f64) -> Luma<T> {
        let n = traits::cast(value * f64::from(T::max_value())).unwrap();
        Luma([n])
    }
}

impl<T: Primitive + 'static> Color for LumaA<T>
    where f64: From<T>
{
    fn black() -> LumaA<T> {
        LumaA([T::zero(), T::max_value()])
    }

    fn white() -> LumaA<T> {
        LumaA([T::max_value(), T::max_value()])
    }

    fn gray(value: f64) -> LumaA<T> {
        let n = traits::cast(value * f64::from(T::max_value())).unwrap();
        LumaA([n, T::max_value()])
    }
}
