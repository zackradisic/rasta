pub fn triangle_lerp_and_calculate_left(
    i0: f32,
    i1: f32,
    i2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
) -> (LerpKind, LerpKind, bool) {
    let a01 = Lerp::new(i0, d0, i1, d1);
    let a12 = Lerp::new(i1, d1, i2, d2);
    let a02 = Lerp::new(i0, d0, i2, d2);

    let a012 = PiecewiseLerp::new(a01, a12);

    let a02_is_left = a02.mid_value() < a012.mid_value();

    if a02_is_left {
        (LerpKind::Lerp(a02), LerpKind::Piecewise(a012), true)
    } else {
        (LerpKind::Piecewise(a012), LerpKind::Lerp(a02), false)
    }
}

pub fn triangle_lerp(
    i0: f32,
    i1: f32,
    i2: f32,
    d0: f32,
    d1: f32,
    d2: f32,
    a02_is_left: bool,
) -> (LerpKind, LerpKind) {
    let a01 = Lerp::new(i0, d0, i1, d1);
    let a12 = Lerp::new(i1, d1, i2, d2);
    let a02 = Lerp::new(i0, d0, i2, d2);

    let a012 = PiecewiseLerp::new(a01, a12);

    if a02_is_left {
        (LerpKind::Lerp(a02), LerpKind::Piecewise(a012))
    } else {
        (LerpKind::Piecewise(a012), LerpKind::Lerp(a02))
    }
}

pub enum LerpKind {
    Lerp(Lerp),
    Piecewise(PiecewiseLerp),
}

impl Default for LerpKind {
    fn default() -> Self {
        LerpKind::Lerp(Lerp::default())
    }
}

impl LerpKind {
    pub fn interpolate(&self, i: f32) -> f32 {
        match self {
            LerpKind::Lerp(a) => a.interpolate(i),
            LerpKind::Piecewise(a) => a.interpolate(i),
        }
    }
}

pub struct PiecewiseLerp {
    a: Lerp,
    b: Lerp,
}

impl PiecewiseLerp {
    pub fn new(a: Lerp, b: Lerp) -> Self {
        Self { a, b }
    }

    pub fn interpolate(&self, i: f32) -> f32 {
        if i < self.a.i1 {
            return self.a.interpolate(i);
        }
        self.b.interpolate(i)
    }

    pub fn mid_value(&self) -> f32 {
        let i = self.a.i0 + ((self.b.i1 - self.a.i0) / 2.0);
        self.interpolate(i)
    }
}

#[derive(Default)]
pub struct Lerp {
    i0: f32,
    d0: f32,
    i1: f32,
    d1: f32,
    a: f32,
}

impl Lerp {
    pub fn new(i0: f32, d0: f32, i1: f32, d1: f32) -> Self {
        Lerp {
            i0,
            d0,
            i1,
            d1,
            a: (d1 - d0) / (i1 - i0),
        }
    }

    pub fn interpolate(&self, i: f32) -> f32 {
        let x = i - self.i0;
        self.d0 + (x * self.a)
    }

    pub fn mid_value(&self) -> f32 {
        let i = self.i0 + ((self.i1 - self.i0) / 2.0);
        self.interpolate(i)
    }
}

#[cfg(test)]
mod test {
    use crate::draw::Rasterizer;

    use super::Lerp;

    #[test]
    fn works() {
        let i0 = 20.0;
        let i1 = 420.0;
        let d0 = -50.0;
        let d1 = 69.0;

        let lerp = Rasterizer::interpolate(i0, d0, i1, d1);
        let lerp2 = Lerp::new(i0, d0, i1, d1);

        let mut y = i0;

        while y <= i1 {
            assert_eq!(lerp[(y - i0) as usize], lerp2.interpolate(y));
            y += 1.0;
        }
    }
}
