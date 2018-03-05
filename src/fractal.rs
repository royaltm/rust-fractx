use iter::iter;

use super::Uint;

pub struct Fractal {
    w : u32,
    h : u32,
    x0 : f64,
    y0 : f64,
    dx : f64,
    dy : f64,
}

#[allow(dead_code)]
impl Fractal {
    pub fn new(w: u32, h: u32, x0: f64, y0: f64, dx: f64, dy: f64) -> Fractal {
        Fractal {
            w, h,
            x0, y0,
            dx, dy
        }
    }

    pub fn from_view_box(w: u32, h: u32, x0: f64, y0: f64, x1: f64, y1: f64) -> Fractal {
        let dx = (x1 - x0).abs() / (w as f64);
        let dy = (y1 - y0).abs() / (h as f64);
        let x0 = if x1 < x0 { x1 } else { x0 };
        let y0 = if y1 < y0 { y1 } else { y0 };
        Fractal {
            w, h,
            x0, y0,
            dx, dy
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn res_width(&self) -> f64 {
        self.dx
    }

    pub fn res_height(&self) -> f64 {
        self.dy
    }

    pub fn view_width(&self) -> f64 {
        self.dx * (self.w as f64)
    }

    pub fn view_height(&self) -> f64 {
        self.dy * (self.h as f64)
    }

    pub fn view_coords(&self) -> (f64, f64) {
        (self.x0, self.y0)
    }

    pub fn view_box(&self) -> (f64, f64, f64, f64) {
        (self.x0, self.y0,
         self.x0 + self.dx * (self.w as f64),
         self.y0 + self.dy * (self.h as f64))
    }

    pub fn as_view_coords(&self, x: u32, y: u32) -> (f64, f64) {
        (self.x0+(x as f64)*self.dx, self.y0+(y as f64)*self.dy)
    }

    pub fn at_iter(&self, x: u32, y: u32, iters: Uint) -> Uint {
        let (xc, yc) = self.as_view_coords(x, y);
        iter(xc, yc, iters)
    }

    pub fn move_by(&mut self, offx: i32, offy: i32) {
        self.x0 = self.dx * (offx as f64) + self.x0;
        self.y0 = self.dy * (offy as f64) + self.y0;
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;
    }

    pub fn set_view_coordinates(&mut self, x0: f64, y0: f64) {
        self.x0 = x0;
        self.y0 = y0;
    }

    pub fn set_view_size(&mut self, vbx: f64, vby: f64) {
        self.dx = vbx / (self.w as f64);
        self.dy = vby / (self.h as f64);
    }

    pub fn zoom_at(&mut self, x: u32, y: u32, mag: f64) {
        let (cx, cy) = self.as_view_coords(x, y);
        let dx = self.dx * mag;
        let dy = self.dy * mag;
        self.x0 = cx - dx * (x as f64);
        self.y0 = cy - dy * (y as f64);
        self.dx = dx;
        self.dy = dy;
    }

    pub fn window(&self, offx: i32, offy: i32, w: u32, h: u32) -> Fractal {
        let dx = self.dx;
        let dy = self.dy;
        let x0 = dx * (offx as f64) + self.x0;
        let y0 = dy * (offy as f64) + self.y0;
        Fractal {
            w, h,
            x0, y0,
            dx, dy
        }
    }
}
