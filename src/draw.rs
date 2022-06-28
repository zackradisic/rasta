use std::ops::{Add, Mul};

use crate::{
    canvas::{canvas_coords_to_screen_coords, Canvas, IntoPixelValue},
    lerp::{triangle_lerp, triangle_lerp_and_calculate_left, Lerp},
    light::{Light, Shading},
    math::{Degrees, Mat4, Vec2, Vec3, Vec4},
    object::{Cube, Instance, Model, Triangle},
    rasterize::{Color, Point},
    texture::Texture,
};

pub struct Rasterizer {
    cw: f32,
    ch: f32,
    vw: f32,
    vh: f32,
    d: f32,
    depth_buffer: Vec<f32>,
    pub view_projection_matrix: Mat4<f32>,
    pub view_matrix: Mat4<f32>,
    unproject_matrix: Mat4<f32>,
    lights: Vec<Light>,
    shading: Shading,
}

impl Rasterizer {
    pub fn new(
        cw: f32,
        ch: f32,
        vw: f32,
        vh: f32,
        d: f32,
        view_matrix: Mat4<f32>,
        projection_matrix: Mat4<f32>,
        lights: Vec<Light>,
    ) -> Self {
        Self {
            cw,
            ch,
            vw,
            vh,
            d,
            depth_buffer: vec![f32::INFINITY; cw as usize * ch as usize],
            view_projection_matrix: &projection_matrix * &view_matrix,
            unproject_matrix: &view_matrix.invert().unwrap() * projection_matrix.invert().unwrap(),
            view_matrix,
            lights,
            shading: Shading::Phong,
        }
    }

    fn unproject_point(&self, x: f32, y: f32, z: f32) -> Vec3<f32> {
        let mut unprojected_point = &self.unproject_matrix * Vec4(x, y, z, 1.0);
        // divide by z
        unprojected_point.0 = unprojected_point.3;
        unprojected_point.1 = unprojected_point.3;
        unprojected_point.2 = unprojected_point.3;
        Vec3(
            unprojected_point.0,
            unprojected_point.1,
            unprojected_point.2,
        )
    }

    pub fn should_cull(&self, t: &Triangle, camera: Vec3<f32>) -> bool {
        let triangle_center = (&t.p0 + &t.p1 + &t.p2) * (1.0 / 3.0);
        let center = &camera - &triangle_center;
        let normal = t.normal();
        let angle = normal.angle(&center);

        angle < 0.0
    }

    pub fn clear<C: Canvas>(&mut self, canvas: &mut C, color: Color) {
        for i in 0..self.depth_buffer.len() {
            self.depth_buffer[i] = f32::INFINITY;
        }
        canvas.clear(color);
    }

    fn put_pixel<C, X, Y>(&mut self, canvas: &mut C, x: X, y: Y, inv_z: f32, color: Color)
    where
        C: Canvas,
        X: IntoPixelValue,
        Y: IntoPixelValue,
    {
        match canvas_coords_to_screen_coords(x, y, canvas.width(), canvas.height()) {
            Some((x_screen, y_screen)) => {
                let depth_buffer_idx = (y_screen * canvas.width() as u32) + x_screen;

                if inv_z < self.depth_buffer[depth_buffer_idx as usize] {
                    canvas.put_pixel(x, y, color);
                    self.depth_buffer[depth_buffer_idx as usize] = inv_z;
                }
            }
            None => (),
        }
    }

    pub fn viewport_to_canvas(&self, p: Vec2<f32>) -> Vec2<f32> {
        Vec2(
            (p.0 * self.cw / self.vw).floor(),
            (p.1 * self.ch / self.vh).floor(),
        )
    }

    pub fn project_vertex(&self, v: Vec3<f32>) -> Vec2<f32> {
        self.viewport_to_canvas(Vec2(v.0 * self.d / v.2, v.1 * self.d / v.2))
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

    /// TODO: returning a vec is really inefficient
    pub fn interpolate(mut i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
        if i0 == i1 {
            return vec![d0 as f32];
        }

        let mut ret = Vec::with_capacity((i1 - i0).ceil() as usize);

        let a = (d1 - d0) / (i1 - i0);
        let mut d: f32 = d0 as f32;

        while i0 <= i1 {
            ret.push(d);
            d += a;
            i0 += 1.0;
        }

        ret
    }

    pub fn triangle_interpolate(
        y0: f32,
        y1: f32,
        y2: f32,
        d0: f32,
        d1: f32,
        d2: f32,
    ) -> (Vec<f32>, Vec<f32>) {
        let x01 = Self::interpolate(y0, d0, y1, d1);
        let x12 = Self::interpolate(y1, d1, y2, d2);
        let x02 = Self::interpolate(y0, d0, y2, d2);

        let take = x01.len() - 1;
        let x012: Vec<_> = x01.into_iter().take(take).chain(x12.into_iter()).collect();

        (x02, x012)
    }

    pub fn draw_line<C: Canvas>(canvas: &mut C, mut p0: Point, mut p1: Point, color: Color) {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;

        if dx.abs() > dy.abs() {
            // line is horizontalish, make sure x0 < x1
            if dx < 0.0 {
                std::mem::swap(&mut p0, &mut p1);
            }

            let ys = Self::interpolate(p0.x, p0.y, p1.x, p1.y);
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

            let xs = Self::interpolate(p0.y, p0.x, p1.y, p1.x);
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

            let ys = Self::interpolate(p0.x, p0.y, p1.x, p1.y);
            let colors = Self::interpolate_color(p0.x, c0, p1.x, c1);

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

            let xs = Self::interpolate(p0.y, p0.x, p1.y, p1.x);
            let colors = Self::interpolate_color(p0.y, c0, p1.y, c1);
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
        Self::draw_line(canvas, p0, p1, color);
        Self::draw_line(canvas, p1, p2, color);
        Self::draw_line(canvas, p2, p0, color)
    }

    pub fn compute_illumination(
        center: Vec3<f32>,
        camera_pos: &Vec3<f32>,
        normal: Vec3<f32>,
        light: &Light,
    ) -> f32 {
        let mut illumination = 0.2;
        match light {
            Light::Ambient(_) => todo!(),
            Light::Directional(intensity, dir) => {
                // Diffuse component
                let cos_alpha = f32::max(
                    0.0,
                    dir.dot(&normal) / (dir.magnitude() * normal.magnitude()),
                );
                illumination += cos_alpha * intensity;

                // Specular component
                let reflected = &normal * (2.0 * normal.dot(&dir)) + (dir * -1.0);
                let view = Vec3(0.0, 0.0, 0.0) - center;
                let cos_beta = reflected.dot(&view) / (reflected.magnitude() * view.magnitude());
                if cos_beta > 0.0 {
                    let specular = 50;
                    illumination += cos_beta.powi(specular) * intensity;
                }
            }
            Light::Point(_, _) => todo!(),
        };
        illumination
    }

    /// Draw triangle by passing in the triangle object and its projected vertices
    /// TODO: refactor this
    pub fn draw_triangle<C: Canvas>(
        &mut self,
        canvas: &mut C,
        center: Vec3<f32>,
        light_direction: Vec3<f32>,
        camera_pos: &Vec3<f32>,
        triangle: &Triangle,
        texture: Option<&Texture>,
        mut p0: Point,
        mut p1: Point,
        mut p2: Point,
    ) {
        if self.should_cull(triangle, center) {
            return;
        }
        let color = triangle.color;

        let (mut i0, mut i1, mut i2) = (0, 1, 2);
        // Sort so y0 <= y1 <= y2
        if p1.y < p0.y {
            std::mem::swap(&mut p0, &mut p1);
            std::mem::swap(&mut i0, &mut i1);
        }
        if p2.y < p0.y {
            std::mem::swap(&mut p0, &mut p2);
            std::mem::swap(&mut i0, &mut i2);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p1, &mut p2);
            std::mem::swap(&mut i1, &mut i2);
        }
        let (v0, v1, v2) = (&triangle[i0], &triangle[i1], &triangle[i2]);

        let (z0, z1, z2) = (triangle[i0].2, triangle[i1].2, triangle[i2].2);

        let normal = triangle.normal().normalize();
        let (n0, n1, n2) = triangle
            .normals
            .as_ref()
            .map(|normals| {
                (
                    normals[i0 as usize].clone(),
                    normals[i1 as usize].clone(),
                    normals[i2 as usize].clone(),
                )
            })
            .unwrap_or((normal.clone(), normal.clone(), normal.clone()));

        let light = Light::Directional(
            if matches!(self.shading, Shading::Phong) {
                1.9
            } else {
                0.9
            },
            light_direction,
        );

        // Compute x of triangle edges
        let (x_left, x_right, x02_is_left) =
            triangle_lerp_and_calculate_left(p0.y, p1.y, p2.y, p0.x, p1.x, p2.x);
        // Compute interpolated z coordinates for depth buffer
        let (z_left, z_right) =
            triangle_lerp(p0.y, p1.y, p2.y, 1.0 / z0, 1.0 / z1, 1.0 / z2, x02_is_left);
        // Compute points for Gouraud shading
        let illu0 = Self::compute_illumination(v0.clone(), camera_pos, normal.clone(), &light);
        let illu1 = Self::compute_illumination(v1.clone(), camera_pos, normal.clone(), &light);
        let illu2 = Self::compute_illumination(v2.clone(), camera_pos, normal.clone(), &light);
        let (i_left, i_right) = triangle_lerp(p0.y, p1.y, p2.y, illu0, illu1, illu2, x02_is_left);
        // Compute points for phong shading
        let (nx_left, nx_right) = triangle_lerp(p0.y, p1.y, p2.y, n0.0, n1.0, n2.0, x02_is_left);
        let (ny_left, ny_right) = triangle_lerp(p0.y, p1.y, p2.y, n0.1, n1.1, n2.1, x02_is_left);
        let (nz_left, nz_right) = triangle_lerp(p0.y, p1.y, p2.y, n0.2, n1.2, n2.2, x02_is_left);
        // Compute uv for texture
        let (u_left, u_right, v_left, v_right) = if let Some(uvs) = &triangle.uvs {
            let i0 = i0 as usize;
            let i1 = i1 as usize;
            let i2 = i2 as usize;
            let (u02, u012) = triangle_lerp(
                p0.y,
                p1.y,
                p2.y,
                uvs[i0].0 / z0,
                uvs[i1].0 / z1,
                uvs[i2].0 / z2,
                x02_is_left,
            );
            let (v02, v012) = triangle_lerp(
                p0.y,
                p1.y,
                p2.y,
                uvs[i0].1 * (1.0 / z0),
                uvs[i1].1 * (1.0 / z1),
                uvs[i2].1 * (1.0 / z2),
                x02_is_left,
            );
            (u02, u012, v02, v012)
        } else {
            (
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        };

        // Draw
        let mut y = p0.y;
        while y <= p2.y {
            let (xl, xr) = (x_left.interpolate(y), x_right.interpolate(y));

            let zscan = Lerp::new(xl, z_left.interpolate(y), xr, z_right.interpolate(y));
            let iscan = Lerp::new(xl, i_left.interpolate(y), xr, i_right.interpolate(y));
            let uscan = triangle
                .uvs
                .as_ref()
                .map(|_| Lerp::new(xl, u_left.interpolate(y), xr, u_right.interpolate(y)));
            let vscan = triangle
                .uvs
                .as_ref()
                .map(|_| Lerp::new(xl, v_left.interpolate(y), xr, v_right.interpolate(y)));

            let nxscan = Lerp::new(xl, nx_left.interpolate(y), xr, nx_right.interpolate(y));
            let nyscan = Lerp::new(xl, ny_left.interpolate(y), xr, ny_right.interpolate(y));
            let nzscan = Lerp::new(xl, nz_left.interpolate(y), xr, nz_right.interpolate(y));

            let mut x = xl;
            while x <= xr {
                let inverse_z = zscan.interpolate(x);
                let color =
                    if let (Some(uscan), Some(vscan), Some(texture)) = (&uscan, &vscan, texture) {
                        texture.texel(
                            uscan.interpolate(x) / inverse_z,
                            vscan.interpolate(x) / inverse_z,
                        )
                    } else {
                        color
                    };
                let intensity = match self.shading {
                    Shading::Gourad => iscan.interpolate(x),
                    Shading::Phong => {
                        let vertex = self.unproject_point(x, y, 1.0 / inverse_z);
                        let normal = Vec3(
                            nxscan.interpolate(x),
                            nyscan.interpolate(x),
                            nzscan.interpolate(x),
                        );
                        Self::compute_illumination(vertex, camera_pos, normal, &light)
                    }
                };
                let illuminated_color = Color::from_vec3_f32s(&color.to_vec3_f32s() * intensity);

                self.put_pixel(canvas, x, y, inverse_z, illuminated_color);
                x += 1.0;
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
            ret.push(Color::from_vec3_f32s(d.clone()));
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
        let x01 = Self::interpolate(p0.y, p0.x, p1.y, p1.x);
        let x12 = Self::interpolate(p1.y, p1.x, p2.y, p2.x);
        let x02 = Self::interpolate(p0.y, p0.x, p2.y, p2.x);

        let h01 = Self::interpolate_color(p0.y, c0, p1.y, c1);
        let h12 = Self::interpolate_color(p1.y, c1, p2.y, c2);
        let h02 = Self::interpolate_color(p0.y, c0, p2.y, c2);

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
        let m = x02.len() / 2;
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
        &self,
        canvas: &mut C,
        front_vertices: [Vec3<f32>; 4],
        back_vertices: [Vec3<f32>; 4],
    ) {
        let front_vertices: Vec<_> = front_vertices
            .into_iter()
            .map(|p| self.project_vertex(p).into())
            .collect();
        let back_vertices: Vec<_> = back_vertices
            .into_iter()
            .map(|p| self.project_vertex(p).into())
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
        Self::draw_line(canvas, b0, b1, Color::RED);
        Self::draw_line(canvas, b1, b2, Color::RED);
        Self::draw_line(canvas, b2, b3, Color::RED);
        Self::draw_line(canvas, b3, b0, Color::RED);

        // front to back lines
        Self::draw_line(canvas, f0, b0, Color::GREEN);
        Self::draw_line(canvas, f1, b1, Color::GREEN);
        Self::draw_line(canvas, f2, b2, Color::GREEN);
        Self::draw_line(canvas, f3, b3, Color::GREEN);

        // front lines
        Self::draw_line(canvas, f0, f1, Color::BLUE);
        Self::draw_line(canvas, f1, f2, Color::BLUE);
        Self::draw_line(canvas, f2, f3, Color::BLUE);
        Self::draw_line(canvas, f3, f0, Color::BLUE);
    }

    pub fn draw_animated_cube_wireframe<C: Canvas>(
        &self,
        canvas: &mut C,
        front_vertices: [Vec3<f32>; 4],
        back_vertices: [Vec3<f32>; 4],
        t: usize,
        point: Vec3<f32>,
    ) {
        let rotation_matrix = Mat4::rotate_y_axis(Degrees((t as f32 / 60.0) * 15.0), point);
        let translation_matrix = Mat4::translate(Vec3(0.0, (t as f32 / 120.0).sin() * 0.5, 0.0));
        let transform = translation_matrix * rotation_matrix;

        let front_vertices: Vec<_> = front_vertices
            .into_iter()
            .map(|p| &transform * Vec4(p.0, p.1, p.2, 1.0))
            .map(|p| Vec3(p.0, p.1, p.2))
            .collect();
        let back_vertices: Vec<_> = back_vertices
            .into_iter()
            .map(|p| &transform * Vec4(p.0, p.1, p.2, 1.0))
            .map(|p| Vec3(p.0, p.1, p.2))
            .collect();

        self.draw_cube_wireframe(
            canvas,
            [
                front_vertices[0].clone(),
                front_vertices[1].clone(),
                front_vertices[2].clone(),
                front_vertices[3].clone(),
            ],
            [
                back_vertices[0].clone(),
                back_vertices[1].clone(),
                back_vertices[2].clone(),
                back_vertices[3].clone(),
            ],
        );
    }

    pub fn draw_cube<C: Canvas>(&mut self, canvas: &mut C, cube: &Cube) {
        for triangle in cube.triangles() {
            self.draw_triangle(
                canvas,
                Vec3(0.0, 0.0, 0.0),
                Vec3(0.0, 0.0, 0.0),
                &Vec3(0.0, 0.0, 0.0),
                triangle,
                None,
                self.project_vertex(triangle.p0.clone()).into(),
                self.project_vertex(triangle.p1.clone()).into(),
                self.project_vertex(triangle.p2.clone()).into(),
            )
        }
    }

    pub fn draw_cube_wireframe_obj<C: Canvas>(&self, canvas: &mut C, cube: &Cube) {
        for triangle in cube.triangles() {
            Self::draw_wireframe_triangle(
                canvas,
                self.project_vertex(triangle.p0.clone()).into(),
                self.project_vertex(triangle.p1.clone()).into(),
                self.project_vertex(triangle.p2.clone()).into(),
                triangle.color,
            )
        }
    }

    pub fn render_instance<'a, 'b, C: Canvas, M: Model<'a>>(
        &mut self,
        canvas: &mut C,
        instance: &Instance<'a, M>,
        texture: Option<&'b Texture>,
    ) {
        self.shading = instance.shading();
        let instance_matrix = &instance.transform_matrix;
        self.render_model(canvas, instance.model, instance_matrix, texture);
    }

    pub fn render_model<'a, 'b, C, M>(
        &mut self,
        canvas: &mut C,
        model: &'a M,
        transform_matrix: &Mat4<f32>,
        texture: Option<&'b Texture>,
    ) where
        C: Canvas,
        M: Model<'a>,
    {
        let final_transform = &self.view_projection_matrix * transform_matrix;
        let mut projected = vec![];
        for v in model.vertices() {
            let projected_vertex = &final_transform * Vec4(v.0, v.1, v.2, 1.0);
            let divide_by_w = Point {
                x: (projected_vertex.0 / projected_vertex.3).floor(),
                y: (projected_vertex.1 / projected_vertex.3).floor(),
            };
            projected.push(divide_by_w);
        }

        // let transformed_center = &self.view_matrix * Vec4(0.0, 0.0, 0.0, 1.0);
        let transformed_center = Vec4(0.0, 0.0, 0.0, 1.0);
        // let light_direction = &self.view_matrix * Vec4(0.6, -0.1, 1.0, 0.0);
        let light_direction = if self.shading == Shading::Phong {
            Vec4(-1.5, -1.0, 1.0, 0.0)
        } else {
            Vec4(0.0, 0.4, 1.0, 0.0)
        };

        let mut i = 0;
        for t in model.triangles() {
            self.draw_triangle(
                canvas,
                Vec3(
                    transformed_center.0,
                    transformed_center.1,
                    transformed_center.2,
                ),
                Vec3(light_direction.0, light_direction.1, light_direction.2),
                &(&self.view_matrix * Vec4(0.0, 0.0, 0.0, 1.0)).drop_fourth_component(),
                // &t.transform(&(&final_transform)),
                &t.transform(&(&self.view_matrix * transform_matrix), &final_transform),
                texture,
                projected[i].into(),
                projected[i + 1].into(),
                projected[i + 2].into(),
            );
            i += 3;
        }
    }
}
