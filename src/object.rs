use std::{
    iter::{Flatten, Map},
    slice::Iter,
};

use crate::{
    math::{Mat4, Radians, Vec3},
    rasterize::Color,
};

pub struct InstanceBuilder<'a, M: Model<'a>> {
    model: &'a M,
    pos: Option<Vec3<f32>>,
    scale: Option<Vec3<f32>>,
    rotation_y: Option<Radians>,
    color: Option<Color>,
}

impl<'a, M: Model<'a>> InstanceBuilder<'a, M> {
    pub fn new(model: &'a M) -> Self {
        Self {
            model,
            pos: None,
            scale: None,
            rotation_y: None,
            color: None,
        }
    }

    pub fn pos(mut self, pos: Vec3<f32>) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn scale(mut self, scale: Vec3<f32>) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn rotation_y<R: Into<Radians>>(mut self, r: R) -> Self {
        self.rotation_y = Some(r.into());
        self
    }

    pub fn color(mut self, c: Color) -> Self {
        self.color = Some(c);
        self
    }

    pub fn build(self) -> Instance<'a, M> {
        let pos = self.pos.unwrap_or_else(|| Vec3(0.0, 0.0, 0.0));
        let scale = self.scale.unwrap_or_else(|| Vec3(1.0, 1.0, 1.0));
        let rotation_y = self.rotation_y.unwrap_or(Radians::new(0.0));

        let transform_matrix =
            Instance::<M>::build_transform_matrix(pos.clone(), scale.clone(), rotation_y.clone());

        Instance {
            model: self.model,
            pos,
            scale,
            rotation_y,
            color: self.color.unwrap_or(Color(0, 0, 0)),
            transform_matrix,
            matrix_needs_update: false,
        }
    }
}

#[derive(Clone)]
pub struct Instance<'a, M: Model<'a>> {
    pub model: &'a M,
    pos: Vec3<f32>,
    scale: Vec3<f32>,
    rotation_y: Radians,
    color: Color,

    matrix_needs_update: bool,
    pub transform_matrix: Mat4<f32>,
}

impl<'a, M: Model<'a>> Instance<'a, M> {
    pub fn new(model: &'a M) -> InstanceBuilder<'a, M> {
        InstanceBuilder::new(model)
    }

    pub fn update_transform_matrix(&mut self) {
        if self.matrix_needs_update {
            self.transform_matrix = Self::build_transform_matrix(
                self.pos.clone(),
                self.scale.clone(),
                self.rotation_y.clone(),
            );
            self.matrix_needs_update = false;
        }
    }

    pub fn rotation(&self) -> Radians {
        self.rotation_y
    }

    pub fn set_rotation<R: Into<Radians>>(&mut self, r: R) {
        self.rotation_y = r.into();
        self.matrix_needs_update = true;
    }

    pub fn pos(&self) -> Vec3<f32> {
        self.pos.clone()
    }

    pub fn set_pos(&mut self, pos: Vec3<f32>) {
        self.pos = pos;
        self.matrix_needs_update = true;
    }

    fn build_transform_matrix(pos: Vec3<f32>, scale: Vec3<f32>, rotation_y: Radians) -> Mat4<f32> {
        Mat4::translate(pos)
            * Mat4::rotate_y_axis(rotation_y, Vec3(0.0, 0.0, 0.0))
            * Mat4::scale(scale)
    }
}

pub trait Model<'a> {
    type VertexIter: Iterator<Item = &'a Vec3<f32>>;
    type TriangleIter: Iterator<Item = &'a Triangle>;

    fn vertices(&'a self) -> Self::VertexIter;
    fn triangles(&'a self) -> Self::TriangleIter;
}

#[derive(Default)]
pub struct ModelOptions {
    pub wireframe: bool,
}

#[derive(Clone)]
pub struct Triangle {
    pub p0: Vec3<f32>,
    pub p1: Vec3<f32>,
    pub p2: Vec3<f32>,
    pub color: Color,
}

impl Triangle {
    pub fn new(p0: Vec3<f32>, p1: Vec3<f32>, p2: Vec3<f32>, color: Color) -> Self {
        Self { p0, p1, p2, color }
    }
}

impl<'a> Model<'a> for Triangle {
    type VertexIter = std::iter::Chain<
        std::iter::Chain<std::iter::Once<&'a Vec3<f32>>, std::iter::Once<&'a Vec3<f32>>>,
        std::iter::Once<&'a Vec3<f32>>,
    >;

    type TriangleIter = std::iter::Once<&'a Self>;

    fn vertices(&'a self) -> Self::VertexIter {
        std::iter::once(&self.p0)
            .chain(std::iter::once(&self.p1))
            .chain(std::iter::once(&self.p2))
    }

    fn triangles(&'a self) -> Self::TriangleIter {
        std::iter::once(&self)
    }
}

#[derive(Clone)]
pub struct Cube {
    front: [Vec3<f32>; 4],
    back: [Vec3<f32>; 4],
    triangles: [Triangle; 12],
    pub color: [Color; 6],
}

fn map_triangle(t: &Triangle) -> [&Vec3<f32>; 3] {
    [&t.p0, &t.p1, &t.p2]
}

impl<'a> Model<'a> for Cube {
    type VertexIter =
        Flatten<Map<Iter<'a, Triangle>, for<'r> fn(&'r Triangle) -> [&'r Vec3<f32>; 3]>>;

    type TriangleIter = Iter<'a, Triangle>;

    fn vertices(&'a self) -> Self::VertexIter {
        self.triangles.iter().map(map_triangle as _).flatten()
    }

    fn triangles(&'a self) -> Self::TriangleIter {
        self.triangles.iter()
    }
}

impl Cube {
    pub fn new(
        ftl: Vec3<f32>,
        fbl: Vec3<f32>,
        fbr: Vec3<f32>,
        ftr: Vec3<f32>,
        btl: Vec3<f32>,
        bbl: Vec3<f32>,
        bbr: Vec3<f32>,
        btr: Vec3<f32>,
        // (front, back, top, bottom, left, right): (Color, Color, Color, Color, Color, Color),
        [front, back, left, right, top, bottom]: [Color; 6],
    ) -> Self {
        let triangles = Self::make_triangles(
            &ftl,
            &fbl,
            &fbr,
            &ftr,
            &btl,
            &bbl,
            &bbr,
            &btr,
            [front, back, left, right, top, bottom],
        );

        Self {
            front: [ftl, fbl, fbr, ftr],
            back: [btl, bbl, bbr, btr],
            triangles,
            color: [front, back, left, right, top, bottom],
        }
    }

    fn make_triangles(
        ftl: &Vec3<f32>,
        fbl: &Vec3<f32>,
        fbr: &Vec3<f32>,
        ftr: &Vec3<f32>,
        btl: &Vec3<f32>,
        bbl: &Vec3<f32>,
        bbr: &Vec3<f32>,
        btr: &Vec3<f32>,
        [front, back, left, right, top, bottom]: [Color; 6],
    ) -> [Triangle; 12] {
        [
            // front
            Triangle::new(ftl.clone(), fbl.clone(), fbr.clone(), front),
            Triangle::new(ftr.clone(), ftl.clone(), fbr.clone(), front),
            // back
            Triangle::new(btl.clone(), bbl.clone(), bbr.clone(), back),
            Triangle::new(btr.clone(), btl.clone(), bbr.clone(), back),
            // left side
            Triangle::new(ftl.clone(), fbl.clone(), bbl.clone(), left),
            Triangle::new(bbl.clone(), btl.clone(), ftl.clone(), left),
            // right side
            Triangle::new(ftr.clone(), fbr.clone(), bbr.clone(), right),
            Triangle::new(bbr.clone(), btr.clone(), ftr.clone(), right),
            // top
            Triangle::new(ftl.clone(), ftr.clone(), btr.clone(), top),
            Triangle::new(btr.clone(), btl.clone(), ftl.clone(), top),
            // bot
            Triangle::new(fbl.clone(), fbr.clone(), bbr.clone(), bottom),
            Triangle::new(bbr.clone(), bbl.clone(), fbl.clone(), bottom),
        ]
    }
}
