use std::slice::Iter;

use crate::{math::Vec3, rasterize::Color};

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

pub struct Cube {
    front: [Vec3<f32>; 4],
    back: [Vec3<f32>; 4],
    triangles: [Triangle; 12],
    pub color: [Color; 6],
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

    pub fn triangles(&self) -> Iter<Triangle> {
        self.triangles.iter()
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
