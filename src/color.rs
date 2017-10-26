use image::*;

pub trait Color {
    fn black() -> Self;
    fn white() -> Self;
}

impl<T: Primitive + 'static> Color for Rgba<T> {
    fn black() -> Rgba<T> {
        Rgba([T::zero(),T::zero(),T::zero(),
              T::from(T::from(0xff).unwrap()).unwrap()])
    }

    fn white() -> Rgba<T> {
        Rgba([T::from(0xff).unwrap(),
              T::from(0xff).unwrap(),
              T::from(0xff).unwrap(),
              T::from(0xff).unwrap()])
    }
}

impl<T: Primitive + 'static> Color for Rgb<T> {
    fn black() -> Rgb<T> {
        Rgb([T::zero(),T::zero(),T::zero()])
    }

    fn white() -> Rgb<T> {
        Rgb([T::from(0xff).unwrap(),
             T::from(0xff).unwrap(),
             T::from(0xff).unwrap()])
    }
}

impl<T: Primitive + 'static> Color for Luma<T> {
    fn black() -> Luma<T> {
        Luma([T::zero()])
    }

    fn white() -> Luma<T> {
        Luma([T::from(0xff).unwrap()])
    }
}

impl<T: Primitive + 'static> Color for LumaA<T> {
    fn black() -> LumaA<T> {
        LumaA([T::zero(), T::from(0xff).unwrap()])
    }

    fn white() -> LumaA<T> {
        LumaA([T::from(0xff).unwrap(), T::from(0xff).unwrap()])
    }
}
