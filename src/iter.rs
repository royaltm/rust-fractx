use super::fractal::MAXI;
use super::Uint;

#[cfg(not(any(target_arch = "x86_64")))]
pub fn iter(x0: f64, y0: f64) -> Uint {
    let (mut x, mut y) = (x0, y0);

    for i in 1..MAXI+1 {
        let (xx, yy) = (x*x, y*y);
        if xx+yy >= 4.0 {
            return i;
        }
        y = 2.0*x*y+y0;
        x = xx-yy+x0;
    }

    MAXI
}

#[cfg(any(target_arch = "x86_64"))]
pub fn iter(x0: f64, y0: f64) -> Uint {
    let numiter: Uint;

    unsafe {
        asm!("
                           //       [  HI:LO  ]
                           // xmm4: [   _, 4.0]
                           // xmm0: [   _,  x0]
                           // xmm1: [   _,  y0]
    mov      ax, cx
    unpcklpd xmm0, xmm1    // xmm0: [  y0,  x0]
    movapd   xmm1, xmm0    // xmm1: [   y,   x]
    jmp      2f
1:
    movsd    xmm2, xmm1    // xmm2: [   _,    x]
    unpcklpd xmm2, xmm1    // xmm2: [   x,    x]
    mulpd    xmm2, xmm1    // xmm2: [ x*y,  x*x]
    unpckhpd xmm1, xmm2    // xmm1: [ x*y,    y]
    mulsd    xmm1, xmm1    // xmm1: [ x*y,  y*y]
    movsd    xmm3, xmm1    // xmm3: [   _,  y*y]
    addpd    xmm1, xmm2    // xmm1: [2xy ,yy+xx]
    ucomisd  xmm1, xmm4    // compare yy + xx > 4.0
    jae      3f
    subsd    xmm2, xmm3    // xmm2: [   _, xx-yy]
    movsd    xmm1, xmm2    // xmm1: [ 2xy, xx-yy]
    addpd    xmm1, xmm0    // xmm1: [2xy+y0,xx-yy+x0]
2:
    dec     cx
    jnz     1b
3:
    sub     ax, cx
"
    : "={ax}"(numiter) // output

    : "{xmm0}"(x0),    // input
      "{xmm1}"(y0),
      "{xmm4}"(4.0),
      "{cx}"(MAXI)

    : "cc",            // clobber
      "xmm2",
      "xmm3"

    : "intel")         // flags
    }

    numiter
}
