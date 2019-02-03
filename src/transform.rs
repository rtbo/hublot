use crate::geom::{FPoint, FVec, Point, Vec};

use std::ops::{Index, IndexMut, Mul};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Transform(pub [[f32; 3]; 2]);

impl Transform {
    #[rustfmt::skip]
    #[inline(always)]
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32
    ) -> Transform {
        Transform([
            [m00, m01, m02],
            [m10, m11, m12],
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn identity() -> Transform {
        Transform([
            [   1f32, 0f32, 0f32,   ],
            [   0f32, 1f32, 0f32    ],
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn translation(vec: FVec) -> Transform {
        Transform([
            [   1f32, 0f32, vec.x(),    ],
            [   0f32, 1f32, vec.y(),    ],
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn rotation(radians: f32) -> Transform {
        let sin = radians.sin();
        let cos = radians.cos();
        Transform([
            [   cos, sin, 0f32,     ],
            [   -sin, cos, 0f32,    ],
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn homothetie(factor: [f32; 2]) -> Transform {
        Transform([
            [   factor[0], 0f32, 0f32, ],
            [   0f32, factor[1], 0f32, ],
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn translate(&self, vec: FVec) -> Transform {
        Transform::new(
            self[(0, 0)], self[(0, 1)], self[(0, 2)] + vec.x(),
            self[(1, 0)], self[(1, 1)], self[(1, 2)] + vec.y(),
        )
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn rotate(&self, radians: f32) -> Transform {
        let cos = radians.cos();
        let sin = radians.sin();
        Transform::new(
            cos * self[(0, 0)] - sin * self[(1, 0)],
            cos * self[(0, 1)] - sin * self[(1, 1)],
            cos * self[(0, 2)] - sin * self[(1, 2)],

            sin * self[(0, 0)] + cos * self[(1, 0)],
            sin * self[(0, 1)] + cos * self[(1, 1)],
            sin * self[(0, 2)] + cos * self[(1, 2)],
        )
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn scale(&self, factors: [f32; 2]) -> Transform {
        let [x, y] = factors;
        Transform::new(
            x * self[(0, 0)], x * self[(0, 1)], x * self[(0, 2)],
            y * self[(1, 0)], y * self[(1, 1)], y * self[(1, 2)],
        )
    }
}

impl Index<usize> for Transform {
    type Output = [f32; 3];
    /// index a single row
    #[inline(always)]
    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row]
    }
}

impl IndexMut<usize> for Transform {
    /// index a single row
    #[inline(always)]
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.0[row]
    }
}

impl Index<(usize, usize)> for Transform {
    type Output = f32;
    /// index an element
    #[inline(always)]
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.0[idx.0][idx.1]
    }
}

impl IndexMut<(usize, usize)> for Transform {
    /// index an element
    #[inline(always)]
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.0[idx.0][idx.1]
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    #[rustfmt::skip]
    fn mul(self, rhs: Transform) -> Transform {
        Transform::new(
            self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0],
            self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1],
            self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2],
            self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0],
            self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0],
            self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2],
        )
    }
}

impl Mul<FVec> for Transform {
    type Output = FVec;
    /// transform a vector (discards translation)
    #[rustfmt::skip]
    fn mul(self, rhs: FVec) -> FVec {
        Vec(
            self[0][0] * rhs.0 + self[0][1] * rhs.1,
            self[1][0] * rhs.0 + self[1][1] * rhs.1,
        )
    }
}

impl Mul<FPoint> for Transform {
    type Output = FPoint;
    /// transform a point (keeps translation)
    #[rustfmt::skip]
    fn mul(self, rhs: FPoint) -> FPoint {
        Point(
            self[0][0] * rhs.0 + self[0][1] * rhs.1 + self[0][2],
            self[1][0] * rhs.0 + self[1][1] * rhs.1 + self[1][2],
        )
    }
}
