use image::{Primitive, Rgba, Rgb, Luma, LumaA};
use num_traits::cast::AsPrimitive;

pub trait Color {
    fn black() -> Self;
    fn white() -> Self;
    fn gray(value: f64) -> Self;
}

impl<T: Primitive + 'static> Color for Rgba<T>
    where f64: From<T> + AsPrimitive<T>
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
        let n: T = (value * f64::from(T::max_value())).as_();
        Rgba([n,n,n, T::max_value()])
    }
}

impl<T: Primitive + 'static> Color for Rgb<T>
    where f64: From<T> + AsPrimitive<T>
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
        let n = (value * f64::from(T::max_value())).as_();
        Rgb([n,n,n])
    }
}

impl<T: Primitive + 'static> Color for Luma<T>
    where f64: From<T> + AsPrimitive<T>
{
    fn black() -> Luma<T> {
        Luma([T::zero()])
    }

    fn white() -> Luma<T> {
        Luma([T::max_value()])
    }

    fn gray(value: f64) -> Luma<T> {
        let n = (value * f64::from(T::max_value())).as_();
        Luma([n])
    }
}

impl<T: Primitive + 'static> Color for LumaA<T>
    where f64: From<T> + AsPrimitive<T>
{
    fn black() -> LumaA<T> {
        LumaA([T::zero(), T::max_value()])
    }

    fn white() -> LumaA<T> {
        LumaA([T::max_value(), T::max_value()])
    }

    fn gray(value: f64) -> LumaA<T> {
        let n = (value * f64::from(T::max_value())).as_();
        LumaA([n, T::max_value()])
    }
}
