use super::fractal::MAXI;
use super::Uint;

pub fn iter(x0: f64, y0: f64) -> Uint {
    let (mut x, mut y) = (x0, y0);

    for i in 1..MAXI {
        let (xx, yy) = (x*x, y*y);
        if xx+yy >= 4.0 {
            return i;
        }
        y = 2.0*x*y+y0;
        x = xx-yy+x0;
    }

    MAXI
}
