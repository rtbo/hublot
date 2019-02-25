use crate::Color;

pub mod gradient {
    use crate::Color;
    use crate::geom;

    /// A control stop for color gradients
    #[derive(Copy, Clone, Debug)]
    pub struct Stop(pub f32, pub Color);

    /// Direction of a linear gradient
    #[derive(Copy, Clone, Debug)]
    pub enum Direction {
        /// North (angle 0 degrees)
        N,
        /// North-East
        NE,
        /// East (angle 90 degrees)
        E,
        /// South-East
        SE,
        /// South (angle 180 degrees)
        S,
        /// South-West
        SW,
        /// West (angle 270 degrees)
        W,
        /// North-West
        NW,
        /// A provided angle in degrees
        Angle(f32),
    }

    impl Direction {
        /// Compute the angle in radians of the gradient line if applied on the given size.
        pub fn compute_angle(&self, size: geom::FSize) -> f32 {
            use std::f32::consts::PI;
            match self {
                Direction::N => 0f32,
                Direction::NE => (size.width() / size.height()).atan(),
                Direction::E => PI / 2f32,
                Direction::SE => PI - (size.width() / size.height()).atan(),
                Direction::S => PI,
                Direction::SW => PI + (size.width() / size.height()).atan(),
                Direction::W => 3f32 * PI / 2f32,
                Direction::NW => 2f32 * PI - (size.width() / size.height()).atan(),
                Direction::Angle(angle) => angle * PI / 180f32,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Paint {
    Solid(Color),
    LinearGradient(Vec<gradient::Stop>, gradient::Direction),
}
