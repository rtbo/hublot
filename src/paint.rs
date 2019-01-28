use crate::Color;

pub mod gradient {
    use crate::Color;

    /// A control stop for color gradients
    pub struct Stop(pub f32, pub Color);

    /// Direction of a linear gradient
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

pub enum Paint {
    Solid(Color),
    LinearGradient(Vec<gradient::Stop>, gradient::Direction),
}
