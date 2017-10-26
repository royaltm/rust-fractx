use super::fractal::MAXI;

pub fn iter(x0: f64, y0: f64) -> bool {
    let (mut x, mut y) : (f64, f64) = (x0, y0);

    for _ in 1..MAXI {
        let (xx, yy) = (x*x, y*y);
        if xx+yy >= 4.0 {
            return false;
        }
        y = 2.0*x*y+y0;
        x = xx-yy+x0;
    }

    true
}
