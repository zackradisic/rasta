use std::mem::take;

use crate::{
    canvas::Canvas,
    rasterize::{Color, Point, Vec3},
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

pub fn draw_shaded_line<C: Canvas>(
    canvas: &mut C,
    (mut p0, mut c0): (Point, Color),
    (mut p1, mut c1): (Point, Color),
) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    if dx.abs() > dy.abs() {
        // line is horizontalish, make sure x0 < x1
        if dx < 0 {
            std::mem::swap(&mut p0, &mut p1);
            std::mem::swap(&mut c0, &mut c1);
        }

        let ys = interpolate(p0.x, p0.y, p1.x, p1.y);
        let colors = interpolate_color(p0.x, c0, p1.x, c1);

        for x in p0.x..=p1.x {
            canvas.put_pixel(x, ys[(x - p0.x) as usize], colors[(x - p0.x) as usize]);
        }
    } else {
        // line is verticalish, make sure it's bottom to to
        if dy < 0 {
            std::mem::swap(&mut p0, &mut p1);
            std::mem::swap(&mut c0, &mut c1);
        }

        let xs = interpolate(p0.y, p0.x, p1.y, p1.x);
        let colors = interpolate_color(p0.y, c0, p1.y, c1);
        for y in p0.y..=p1.y {
            canvas.put_pixel(xs[(y - p0.y) as usize], y, colors[(y - p0.y) as usize]);
        }
    }
}

pub fn draw_wireframe_triangle<C: Canvas>(
    canvas: &mut C,
    p0: Point,
    p1: Point,
    p2: Point,
    color: Color,
) {
    draw_line(canvas, p0, p1, color);
    draw_line(canvas, p1, p2, color);
    draw_line(canvas, p2, p0, color)
}

pub fn draw_triangle<C: Canvas>(
    canvas: &mut C,
    mut p0: Point,
    mut p1: Point,
    mut p2: Point,
    color: Color,
) {
    // Sort so y0 <= y1 <= y2
    if p1.y < p0.y {
        std::mem::swap(&mut p0, &mut p1);
    }
    if p2.y < p0.y {
        std::mem::swap(&mut p0, &mut p2);
    }
    if p2.y < p1.y {
        std::mem::swap(&mut p1, &mut p2);
    }

    // Compute x of triangle edges
    let x01 = interpolate(p0.y, p0.x, p1.y, p1.x);
    let x12 = interpolate(p1.y, p1.x, p2.y, p2.x);
    let x02 = interpolate(p0.y, p0.x, p2.y, p2.x);

    // Concatenate short sides
    let take_amount = x01.len() - 1;
    let x012: Vec<_> = x01
        .into_iter()
        .take(take_amount)
        .chain(x12.into_iter())
        .collect();

    // Determine which is left and which is right
    let m = x012.len() / 2;
    let (x_left, x_right) = if x02[m] < x012[m] {
        (x02, x012)
    } else {
        (x012, x02)
    };

    // Draw
    for y in p0.y..=p2.y {
        let mut x = x_left[(y - p0.y) as usize] as i32;
        loop {
            if x > x_right[(y - p0.y) as usize] as i32 {
                break;
            }

            canvas.put_pixel(x, y, color);

            x += 1;
        }
    }
}

pub fn interpolate_color(i0: i32, d0: Color, i1: i32, d1: Color) -> Vec<Color> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut ret = vec![];

    let d0: Vec3<f32> = d0.to_vec3_f32s();
    let d1: Vec3<f32> = d1.to_vec3_f32s();

    let a: Vec3<f32> = (&d1 - &d0) / (i1 as f32 - i0 as f32);
    let a_r = a.0;
    let a_g = a.1;
    let a_b = a.2;

    let mut d = d0;

    for _ in i0..=i1 {
        ret.push(Color::from_vec3_f32s(d.clone()).unwrap());
        d.0 += a_r;
        d.1 += a_g;
        d.2 += a_b;
    }

    ret
}

pub fn draw_shaded_triangle<C: Canvas>(
    canvas: &mut C,
    (mut p0, mut c0): (Point, Color),
    (mut p1, mut c1): (Point, Color),
    (mut p2, mut c2): (Point, Color),
) {
    // Sort so y0 <= y1 <= y2
    if p1.y < p0.y {
        std::mem::swap(&mut p0, &mut p1);
        std::mem::swap(&mut c0, &mut c1);
    }
    if p2.y < p0.y {
        std::mem::swap(&mut p0, &mut p2);
        std::mem::swap(&mut c0, &mut c2);
    }
    if p2.y < p1.y {
        std::mem::swap(&mut p1, &mut p2);
        std::mem::swap(&mut c1, &mut c2);
    }

    // Compute x of triangle edges
    let x01 = interpolate(p0.y, p0.x, p1.y, p1.x);
    let x12 = interpolate(p1.y, p1.x, p2.y, p2.x);
    let x02 = interpolate(p0.y, p0.x, p2.y, p2.x);

    let h01 = interpolate_color(p0.y, c0, p1.y, c1);
    let h12 = interpolate_color(p1.y, c1, p2.y, c2);
    let h02 = interpolate_color(p0.y, c0, p2.y, c2);

    // Concatenate short sides
    let take_amount = x01.len() - 1;
    let x012: Vec<_> = x01
        .into_iter()
        .take(take_amount)
        .chain(x12.into_iter())
        .collect();
    let h012: Vec<_> = h01
        .into_iter()
        .take(take_amount)
        .chain(h12.into_iter())
        .collect();

    // Determine which is left and which is right
    let m = x012.len() / 2;
    let (x_left, x_right, h_left, h_right) = if x02[m] < x012[m] {
        (x02, x012, h02, h012)
    } else {
        (x012, x02, h012, h02)
    };

    // Draw
    for y in p0.y..=p2.y {
        let mut x = x_left[(y - p0.y) as usize] as i32;
        let color_idx = (y - p0.y) as usize;
        let color = if color_idx < h_left.len() {
            h_left[color_idx]
        } else {
            h_right[color_idx]
        };

        loop {
            if x > x_right[(y - p0.y) as usize] as i32 {
                break;
            }

            canvas.put_pixel(x, y, color);

            x += 1;
        }
    }
}
