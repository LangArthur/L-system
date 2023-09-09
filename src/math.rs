pub enum Rotation {
    ClockWise,
    AntiClockWise,
}

impl Rotation {
    pub fn value(&self) -> f32 {
        match self {
            Self::ClockWise => 1.0,
            Self::AntiClockWise => -1.0,
        }
    }
}
