use std::ops::{Div, Mul};

use crate::{
    canvas::Canvas,
    math::{Vec2, Vec3},
    rasterize::{Color, Point},
};

pub fn viewport_to_canvas<T: Mul<Output = T> + Div<Output = T>>(
    p: Vec2<T>,
    cw: T,
    ch: T,
    vw: T,
    vh: T,
) -> Vec2<T> {
    Vec2(p.0 * cw / vw, p.1 * ch / vh)
}

pub fn project_vertex<T: Mul<Output = T> + Div<Output = T> + Copy>(
    v: Vec3<T>,
    d: T,
    cw: T,
    ch: T,
    vw: T,
    vh: T,
) -> Vec2<T> {
    viewport_to_canvas(Vec2(v.0 * d / v.2, v.1 * d / v.2), cw, ch, vw, vh)
}

pub fn draw_line_broken<C: Canvas>(
    canvas: &mut C,
    mut x0: f32,
    mut y0: f32,
    mut x1: f32,
    mut y1: f32,
    color: Color,
) {
    // make sure x0 < x1
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let a = (y1 - y0) / (x1 - x0);

    let mut y = y0;
    let mut x = x0;
    while x <= x1 {
        canvas.put_pixel(x, y, color);
        y += a;
        x += 1.0;
    }
}

pub fn interpolate(mut i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0 as f32];
    }

    let mut ret = vec![];

    let a: f32 = (d1 - d0) as f32 / (i1 - i0) as f32;
    let mut d: f32 = d0 as f32;

    while i0 <= i1 {
        ret.push(d);
        d += a;
        i0 += 1.0;
    }

    ret
}

pub fn draw_line<C: Canvas>(canvas: &mut C, mut p0: Point, mut p1: Point, color: Color) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    if dx.abs() > dy.abs() {
        // line is horizontalish, make sure x0 < x1
        if dx < 0.0 {
            std::mem::swap(&mut p0, &mut p1);
        }

        let ys = interpolate(p0.x, p0.y, p1.x, p1.y);
        let mut x = p0.x;
        while x <= p1.x {
            canvas.put_pixel(x, ys[(x - p0.x) as usize], color);
            x += 1.0;
        }
    } else {
        // line is verticalish, make sure it's bottom to to
        if dy < 0.0 {
            std::mem::swap(&mut p0, &mut p1);
        }

        let xs = interpolate(p0.y, p0.x, p1.y, p1.x);
        let mut y = p0.y;
        while y <= p1.y {
            canvas.put_pixel(xs[(y - p0.y) as usize], y, color);
            y += 1.0;
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
        if dx < 0.0 {
            std::mem::swap(&mut p0, &mut p1);
            std::mem::swap(&mut c0, &mut c1);
        }

        let ys = interpolate(p0.x, p0.y, p1.x, p1.y);
        let colors = interpolate_color(p0.x, c0, p1.x, c1);

        let mut x = p0.x;
        while x <= p1.x {
            canvas.put_pixel(x, ys[(x - p0.x) as usize], colors[(x - p0.x) as usize]);
            x += 1.0;
        }
    } else {
        // line is verticalish, make sure it's bottom to to
        if dy < 0.0 {
            std::mem::swap(&mut p0, &mut p1);
            std::mem::swap(&mut c0, &mut c1);
        }

        let xs = interpolate(p0.y, p0.x, p1.y, p1.x);
        let colors = interpolate_color(p0.y, c0, p1.y, c1);
        let mut y = p0.y;
        while y <= p1.y {
            canvas.put_pixel(xs[(y - p0.y) as usize], y, colors[(y - p0.y) as usize]);
            y += 1.0;
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
    let mut y = p0.y;
    while y <= p2.y {
        let mut x = x_left[(y - p0.y) as usize] as i32;
        loop {
            if x > x_right[(y - p0.y) as usize] as i32 {
                break;
            }

            canvas.put_pixel(x, y, color);

            x += 1;
        }
        y += 1.0;
    }
}

pub fn interpolate_color(mut i0: f32, d0: Color, i1: f32, d1: Color) -> Vec<Color> {
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

    while i0 <= i1 {
        ret.push(Color::from_vec3_f32s(d.clone()).unwrap());
        d.0 += a_r;
        d.1 += a_g;
        d.2 += a_b;
        i0 += 1.0;
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
    let mut y = p0.y;
    while y <= p2.y {
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
        y += 1.0;
    }
}

pub fn draw_cube_wireframe<C: Canvas>(
    canvas: &mut C,
    front_vertices: [Vec3<f32>; 4],
    back_vertices: [Vec3<f32>; 4],
    (vw, vh): (f32, f32),
    d: f32,
) {
    let front_vertices: Vec<_> = front_vertices
        .into_iter()
        .map(|p| project_vertex(p, d, canvas.width() as f32, canvas.height() as f32, vw, vh).into())
        .collect();
    let back_vertices: Vec<_> = back_vertices
        .into_iter()
        .map(|p| project_vertex(p, d, canvas.width() as f32, canvas.height() as f32, vw, vh).into())
        .collect();

    let f0 = front_vertices[0];
    let f1 = front_vertices[1];
    let f2 = front_vertices[2];
    let f3 = front_vertices[3];

    let b0 = back_vertices[0];
    let b1 = back_vertices[1];
    let b2 = back_vertices[2];
    let b3 = back_vertices[3];

    // back
    draw_line(canvas, b0, b1, Color::RED);
    draw_line(canvas, b1, b2, Color::RED);
    draw_line(canvas, b2, b3, Color::RED);
    draw_line(canvas, b3, b0, Color::RED);

    // front to back lines
    draw_line(canvas, f0, b0, Color::GREEN);
    draw_line(canvas, f1, b1, Color::GREEN);
    draw_line(canvas, f2, b2, Color::GREEN);
    draw_line(canvas, f3, b3, Color::GREEN);

    // front lines
    draw_line(canvas, f0, f1, Color::BLUE);
    draw_line(canvas, f1, f2, Color::BLUE);
    draw_line(canvas, f2, f3, Color::BLUE);
    draw_line(canvas, f3, f0, Color::BLUE);
}
