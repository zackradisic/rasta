use std::{
    iter::{Flatten, Map},
    ops::Index,
    slice::Iter,
};

use crate::{
    light::Shading,
    math::{Mat4, Radians, Vec2, Vec3, Vec4},
    rasterize::Color,
    texture::Texture,
    wavefront::WavefrontObj,
};

pub struct InstanceBuilder<'a, M: Model<'a>> {
    model: &'a M,
    pos: Option<Vec3<f32>>,
    scale: Option<Vec3<f32>>,
    rotation_y: Option<Radians>,
    color: Option<Color>,
    shading: Option<Shading>,
}

impl<'a, M: Model<'a>> InstanceBuilder<'a, M> {
    pub fn new(model: &'a M) -> Self {
        Self {
            model,
            pos: None,
            scale: None,
            rotation_y: None,
            color: None,
            shading: None,
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

    pub fn shading(mut self, s: Shading) -> Self {
        self.shading = Some(s);
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
            shading: self.shading.unwrap_or_default(),
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
    shading: Shading,
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

    pub fn shading(&self) -> Shading {
        self.shading
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
    fn texture(&'a self) -> Option<&'a Texture>;
}

#[derive(Default)]
pub struct ModelOptions {
    pub wireframe: bool,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub p0: Vec3<f32>,
    pub p1: Vec3<f32>,
    pub p2: Vec3<f32>,
    pub color: Color,
    pub uvs: Option<[Vec2<f32>; 3]>,
    pub normals: Option<[Vec3<f32>; 3]>,
}

impl Triangle {
    pub fn new(
        p0: Vec3<f32>,
        p1: Vec3<f32>,
        p2: Vec3<f32>,
        color: Color,
        normals: Option<[Vec3<f32>; 3]>,
    ) -> Self {
        Self {
            p0,
            p1,
            p2,
            color,
            normals: normals,
            uvs: None,
        }
    }

    pub fn new_with_uvs<V: Into<Vec2<f32>> + Clone>(
        p0: Vec3<f32>,
        p1: Vec3<f32>,
        p2: Vec3<f32>,
        uvs: [V; 3],
        normals: Option<[Vec3<f32>; 3]>,
    ) -> Self {
        Self {
            p0,
            p1,
            p2,
            color: Color(0, 0, 0),
            uvs: Some([
                uvs[0].clone().into(),
                uvs[1].clone().into(),
                uvs[2].clone().into(),
            ]),
            normals: normals,
        }
    }

    pub fn transform(&self, m: &Mat4<f32>, view_proj: &Mat4<f32>) -> Self {
        let p0 = m * Vec4(self.p0.0, self.p0.1, self.p0.2, 1.0);
        let p1 = m * Vec4(self.p1.0, self.p1.1, self.p1.2, 1.0);
        let p2 = m * Vec4(self.p2.0, self.p2.1, self.p2.2, 1.0);
        Self {
            p0: Vec3(p0.0, p0.1, p0.2),
            p1: Vec3(p1.0, p1.1, p1.2),
            p2: Vec3(p2.0, p2.1, p2.2),
            color: self.color,
            uvs: self.uvs.clone(),
            normals: self.normals.clone().map(|normals| {
                [
                    (view_proj * normals[0].to_point_vec4()).drop_fourth_component(),
                    (view_proj * normals[1].to_point_vec4()).drop_fourth_component(),
                    (view_proj * normals[2].to_point_vec4()).drop_fourth_component(),
                ]
            }),
            // normals: self.normals.clone(),
        }
    }

    pub fn normal(&self) -> Vec3<f32> {
        let v1 = &self.p1 - &self.p0;
        let v2 = &self.p2 - &self.p0;
        v1.cross(&v2)
    }
}

impl Index<u8> for Triangle {
    type Output = Vec3<f32>;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.p0,
            1 => &self.p1,
            2 => &self.p2,
            otherwise => panic!("Invalid triangle vertex index: {}", otherwise),
        }
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

    fn texture(&'a self) -> Option<&'a Texture> {
        todo!()
    }
}

#[derive(Clone)]
pub struct Cube<'a> {
    front: [Vec3<f32>; 4],
    back: [Vec3<f32>; 4],
    triangles: [Triangle; 12],
    texture: Option<&'a Texture>,
}

fn map_triangle(t: &Triangle) -> [&Vec3<f32>; 3] {
    [&t.p0, &t.p1, &t.p2]
}

impl<'a, 'b> Model<'a> for Cube<'b> {
    type VertexIter =
        Flatten<Map<Iter<'a, Triangle>, for<'r> fn(&'r Triangle) -> [&'r Vec3<f32>; 3]>>;

    type TriangleIter = Iter<'a, Triangle>;

    fn vertices(&'a self) -> Self::VertexIter {
        self.triangles.iter().map(map_triangle as _).flatten()
    }

    fn triangles(&'a self) -> Self::TriangleIter {
        self.triangles.iter()
    }

    fn texture(&'a self) -> Option<&'a Texture> {
        self.texture
    }
}

impl<'a> Cube<'a> {
    pub fn new_with_texture<V: Into<Vec2<f32>> + Clone>(
        ftl: Vec3<f32>,
        fbl: Vec3<f32>,
        fbr: Vec3<f32>,
        ftr: Vec3<f32>,
        btl: Vec3<f32>,
        bbl: Vec3<f32>,
        bbr: Vec3<f32>,
        btr: Vec3<f32>,
        tex_coords: [[V; 3]; 12],
        tex: &'a Texture,
    ) -> Self {
        let triangles = Self::make_triangles_with_uvs(
            &ftl, &fbl, &fbr, &ftr, &btl, &bbl, &bbr, &btr, tex_coords,
        );

        Self {
            front: [ftl, fbl, fbr, ftr],
            back: [btl, bbl, bbr, btr],
            triangles,
            texture: Some(tex),
        }
    }

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
            texture: None,
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
            Triangle::new(ftl.clone(), fbl.clone(), fbr.clone(), front, None),
            Triangle::new(ftr.clone(), ftl.clone(), fbr.clone(), front, None),
            // back
            Triangle::new(btr.clone(), bbr.clone(), bbl.clone(), back, None),
            Triangle::new(btl.clone(), btr.clone(), bbl.clone(), back, None),
            // left side
            Triangle::new(ftl.clone(), bbl.clone(), fbl.clone(), left, None),
            Triangle::new(btl.clone(), bbl.clone(), ftl.clone(), left, None),
            // right side
            Triangle::new(ftr.clone(), fbr.clone(), bbr.clone(), right, None),
            Triangle::new(bbr.clone(), btr.clone(), ftr.clone(), right, None),
            // top
            Triangle::new(ftl.clone(), ftr.clone(), btr.clone(), top, None),
            Triangle::new(btr.clone(), btl.clone(), ftl.clone(), top, None),
            // bot
            Triangle::new(fbr.clone(), fbl.clone(), bbr.clone(), bottom, None),
            Triangle::new(bbl.clone(), bbr.clone(), fbl.clone(), bottom, None),
        ]
    }

    fn make_triangles_with_uvs<V: Into<Vec2<f32>> + Clone>(
        ftl: &Vec3<f32>,
        fbl: &Vec3<f32>,
        fbr: &Vec3<f32>,
        ftr: &Vec3<f32>,
        btl: &Vec3<f32>,
        bbl: &Vec3<f32>,
        bbr: &Vec3<f32>,
        btr: &Vec3<f32>,
        [front, front2, back, back2, left, left2, right, right2, top, top2, bottom, bottom2]: [[V; 3]; 12],
    ) -> [Triangle; 12] {
        [
            // front
            Triangle::new_with_uvs(ftl.clone(), fbl.clone(), fbr.clone(), front, None),
            Triangle::new_with_uvs(ftr.clone(), ftl.clone(), fbr.clone(), front2, None),
            // back
            Triangle::new_with_uvs(btr.clone(), bbr.clone(), bbl.clone(), back, None),
            Triangle::new_with_uvs(btl.clone(), btr.clone(), bbl.clone(), back2, None),
            // left side
            Triangle::new_with_uvs(ftl.clone(), bbl.clone(), fbl.clone(), left, None),
            Triangle::new_with_uvs(btl.clone(), bbl.clone(), ftl.clone(), left2, None),
            // right side
            Triangle::new_with_uvs(ftr.clone(), fbr.clone(), bbr.clone(), right, None),
            Triangle::new_with_uvs(bbr.clone(), btr.clone(), ftr.clone(), right2, None),
            // top
            Triangle::new_with_uvs(ftl.clone(), ftr.clone(), btr.clone(), top, None),
            Triangle::new_with_uvs(btr.clone(), btl.clone(), ftl.clone(), top2, None),
            // bot
            Triangle::new_with_uvs(fbr.clone(), fbl.clone(), bbr.clone(), bottom, None),
            Triangle::new_with_uvs(bbl.clone(), bbr.clone(), fbl.clone(), bottom2, None),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct WavefrontModel<'a> {
    obj: WavefrontObj,
    triangles: Vec<Triangle>,
    texture: Option<&'a Texture>,
}

impl<'a> WavefrontModel<'a> {
    pub fn new(obj: WavefrontObj, outlines: bool) -> Self {
        let triangles = obj.make_triangles(Some(Color(0, 255, 255)), false, false, outlines);
        Self {
            obj,
            triangles,
            texture: None,
        }
    }

    pub fn new_with_tex(obj: WavefrontObj, texture: &'a Texture, normals: bool) -> Self {
        let triangles = obj.make_triangles(None, true, normals, false);
        Self {
            obj,
            triangles,
            texture: Some(texture),
        }
    }
}

impl<'a> Model<'a> for WavefrontModel<'a> {
    type VertexIter = Flatten<
        Map<
            Iter<'a, Triangle>,
            for<'r> fn(
                &'r Triangle,
            ) -> std::iter::Chain<
                std::iter::Chain<std::iter::Once<&Vec3<f32>>, std::iter::Once<&Vec3<f32>>>,
                std::iter::Once<&Vec3<f32>>,
            >,
        >,
    >;

    type TriangleIter = Iter<'a, Triangle>;

    fn vertices(&'a self) -> Self::VertexIter {
        self.triangles.iter().map(triangle_vertices as _).flatten()
    }

    fn triangles(&'a self) -> Self::TriangleIter {
        self.triangles.iter()
    }

    fn texture(&'a self) -> Option<&'a Texture> {
        self.texture
    }
}

fn triangle_vertices(t: &Triangle) -> <Triangle as Model>::VertexIter {
    t.vertices()
}
