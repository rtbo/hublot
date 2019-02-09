use crate::Color;

pub mod gradient {
    use crate::Color;

    /// A control stop for color gradients
    #[derive(Copy, Clone, Debug)]
    pub struct Stop(pub f32, pub Color);

    /// Direction of a linear gradient
    #[derive(Copy, Clone, Debug)]
    pub enum Direction {
        Angle(f32),
        N,
        NE,
        E,
        SE,
        S,
        SW,
        W,
        NW,
    }
}

#[derive(Clone, Debug)]
pub enum Paint {
    Solid(Color),
    LinearGradient(Vec<gradient::Stop>, gradient::Direction),
}
