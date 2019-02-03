use std::ops::{Add, Index, IndexMut, Sub};

pub type FPoint = Point<f32>;
pub type IPoint = Point<i32>;

pub type FVec = Vec<f32>;
pub type IVec = Vec<i32>;

pub type FSize = Size<f32>;
pub type ISize = Size<i32>;

pub type FRect = Rect<f32>;
pub type IRect = Rect<i32>;

pub type FMargins = Margins<f32>;
pub type IMargins = Margins<i32>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point<T>(pub T, pub T);

impl<T: Copy> Point<T> {
    pub fn x(&self) -> T {
        self.0
    }
    pub fn y(&self) -> T {
        self.0
    }
}

impl<T: Copy> Index<usize> for Point<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!(),
        }
    }
}

impl<T: Copy> IndexMut<usize> for Point<T> {
    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!(),
        }
    }
}

impl From<FPoint> for [f32; 2] {
    fn from(val: FPoint) -> Self {
        [val.0, val.1]
    }
}

impl From<IPoint> for [i32; 2] {
    fn from(val: IPoint) -> Self {
        [val.0, val.1]
    }
}

impl From<winit::dpi::LogicalPosition> for FPoint {
    fn from(pos: winit::dpi::LogicalPosition) -> Self {
        let (x, y): (f64, f64) = pos.into();
        Point(x as _, y as _)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vec<T>(pub T, pub T);

impl<T: Copy> Vec<T> {
    pub fn x(&self) -> T {
        self.0
    }
    pub fn y(&self) -> T {
        self.0
    }
}

impl<T: Copy> Index<usize> for Vec<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!(),
        }
    }
}

impl<T: Copy> IndexMut<usize> for Vec<T> {
    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!(),
        }
    }
}


impl From<FVec> for [f32; 2] {
    fn from(val: FVec) -> Self {
        [val.0, val.1]
    }
}

impl From<IVec> for [i32; 2] {
    fn from(val: IVec) -> Self {
        [val.0, val.1]
    }
}

impl<T: Add<Output = T>> Add for Vec<T> {
    type Output = Vec<T>;
    fn add(self, other: Vec<T>) -> Vec<T> {
        Vec(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Add<Output = T>> Add<Vec<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Vec<T>) -> Point<T> {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Size<T: Copy>(pub T, pub T);

impl<T: Copy> Size<T> {
    pub fn width(&self) -> T {
        self.0
    }
    pub fn height(&self) -> T {
        self.1
    }
}

impl From<FSize> for [f32; 2] {
    fn from(val: FSize) -> Self {
        [val.0, val.1]
    }
}

impl From<ISize> for [i32; 2] {
    fn from(val: ISize) -> Self {
        [val.0, val.1]
    }
}

impl From<winit::dpi::LogicalSize> for FSize {
    fn from(size: winit::dpi::LogicalSize) -> Self {
        let (w, h): (f64, f64) = size.into();
        Size(w as _, h as _)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rect<T: Copy> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
where
    T: Copy,
    T: Add<Output = T>,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
    pub fn new_s(x: T, y: T, size: Size<T>) -> Rect<T> {
        Rect {
            x,
            y,
            width: size.0,
            height: size.1,
        }
    }
    pub fn new_p(point: Point<T>, width: T, height: T) -> Rect<T> {
        Rect {
            x: point.0,
            y: point.1,
            width,
            height,
        }
    }
    pub fn new_ps(point: Point<T>, size: Size<T>) -> Rect<T> {
        Rect {
            x: point.0,
            y: point.1,
            width: size.0,
            height: size.1,
        }
    }

    pub fn point(&self) -> Point<T> {
        Point(self.x, self.y)
    }
    pub fn size(&self) -> Size<T> {
        Size(self.width, self.height)
    }
    pub fn left(&self) -> T {
        self.x
    }
    pub fn top(&self) -> T {
        self.y
    }
    pub fn right(&self) -> T {
        self.x + self.width
    }
    pub fn bottom(&self) -> T {
        self.y + self.height
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Margins<T>(pub T, pub T, pub T, pub T);

impl<T: Copy + Add<Output = T>> Margins<T> {
    pub fn left(&self) -> T {
        self.0
    }
    pub fn top(&self) -> T {
        self.1
    }
    pub fn right(&self) -> T {
        self.2
    }
    pub fn bottom(&self) -> T {
        self.3
    }
    pub fn horizontal(&self) -> T {
        self.0 + self.2
    }
    pub fn vertical(&self) -> T {
        self.1 + self.3
    }
}

impl From<FMargins> for [f32; 4] {
    fn from(val: FMargins) -> Self {
        [val.0, val.1, val.2, val.3]
    }
}

impl From<IMargins> for [i32; 4] {
    fn from(val: IMargins) -> Self {
        [val.0, val.1, val.2, val.3]
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Add<Margins<T>> for Rect<T> {
    type Output = Rect<T>;
    fn add(self, rhs: Margins<T>) -> Rect<T> {
        Rect {
            x: self.x - rhs.left(),
            y: self.y - rhs.top(),
            width: self.width + rhs.horizontal(),
            height: self.height + rhs.vertical(),
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Sub<Margins<T>> for Rect<T> {
    type Output = Rect<T>;
    fn sub(self, rhs: Margins<T>) -> Rect<T> {
        Rect {
            x: self.x + rhs.left(),
            y: self.y + rhs.top(),
            width: self.width - rhs.horizontal(),
            height: self.height + rhs.vertical(),
        }
    }
}
