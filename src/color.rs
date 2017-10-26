use image::*;

pub trait Color {
    fn black() -> Self;
    fn white() -> Self;
}

impl<T: Primitive + 'static> Color for Rgba<T> {
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
}

impl<T: Primitive + 'static> Color for Rgb<T> {
    fn black() -> Rgb<T> {
        Rgb([T::zero(),T::zero(),T::zero()])
    }

    fn white() -> Rgb<T> {
        Rgb([T::max_value(),
             T::max_value(),
             T::max_value()])
    }
}

impl<T: Primitive + 'static> Color for Luma<T> {
    fn black() -> Luma<T> {
        Luma([T::zero()])
    }

    fn white() -> Luma<T> {
        Luma([T::max_value()])
    }
}

impl<T: Primitive + 'static> Color for LumaA<T> {
    fn black() -> LumaA<T> {
        LumaA([T::zero(), T::max_value()])
    }

    fn white() -> LumaA<T> {
        LumaA([T::max_value(), T::max_value()])
    }
}
