use crate::{
    canvas::Canvas,
    rasterize::{Color, Point},
};

pub fn draw_line_broken<C: Canvas>(
    canvas: &mut C,
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    color: Color,
) {
    // make sure x0 < x1
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let a = (y1 - y0) / (x1 - x0);

    let mut y = y0;
    for x in x0..=x1 {
        canvas.put_pixel(x, y, color);
        y += a;
    }
}

pub fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0 as f32];
    }

    let mut ret = vec![];

    let a: f32 = (d1 - d0) as f32 / (i1 - i0) as f32;
    let mut d: f32 = d0 as f32;

    for _ in i0..=i1 {
        ret.push(d);
        d += a;
    }

    ret
}

pub fn draw_line<C: Canvas>(canvas: &mut C, mut p0: Point, mut p1: Point, color: Color) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    if dx.abs() > dy.abs() {
        // line is horizontalish, make sure x0 < x1
        if dx < 0 {
            std::mem::swap(&mut p0, &mut p1);
        }

        let ys = interpolate(p0.x, p0.y, p1.x, p1.y);
        for x in p0.x..=p1.x {
            canvas.put_pixel(x, ys[(x - p0.x) as usize], color);
        }
    } else {
        // line is verticalish, make sure it's bottom to to
        if dy < 0 {
            std::mem::swap(&mut p0, &mut p1);
        }

        let xs = interpolate(p0.y, p0.x, p1.y, p1.x);
        for y in p0.y..=p1.y {
            canvas.put_pixel(xs[(y - p0.y) as usize], y, color);
        }
    }
}
