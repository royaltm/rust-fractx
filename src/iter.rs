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

// regs:
// X0L - x, xx
// X1L - y
// X2L - backup x
// X3L - yy
// X4L - const x0
// X5L - const y0
// X6L - backup acc
// X7L - const pbound = 4

#[cfg(any(target_arch = "x86_64"))]
pub fn iter(x0: f64, y0: f64) -> Uint {
    let i: Uint;

    unsafe {
        asm!("
    movupd  xmm4, xmm0
    movupd  xmm5, xmm1
    mov     cx, ax
    jmp     2f

1:
    movupd  xmm2, xmm0
    movupd  xmm3, xmm1

    mulsd   xmm0, xmm0
    mulsd   xmm3, xmm3
    movupd  xmm6, xmm3
    addsd   xmm6, xmm0
    ucomisd xmm6, xmm7
    jae     3f

    subsd   xmm0, xmm3
    addsd   xmm0, xmm4

    mulsd   xmm1, xmm2
    addsd   xmm1, xmm1
    addsd   xmm1, xmm5
2:
    dec     cx
    jnz     1b
3:
    sub     ax, cx
"
    : "={ax}"(i)    // output

    : "{xmm0}"(x0), // input
      "{xmm1}"(y0),
      "{xmm7}"(4.0),
      "{ax}"(MAXI)

    : "cc",         // clobber
      "cx",
      "xmm2",
      "xmm3",
      "xmm4",
      "xmm5",
      "xmm6"

    : "volatile",   // flags
      "alignstack",
      "intel")
    }

    i
}
