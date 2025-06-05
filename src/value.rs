/// A generic trait for [f32] and [f64] floating point types.
pub trait AngleValue:
    Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Copy
    + Sized
{
    /// Converts this value from radians to degrees.
    fn radians_to_degrees(&self) -> Self;

    /// Converts this value from radians to rotations.
    fn radians_to_rotations(&self) -> Self;

    /// Converts this value from radians to percentage.
    fn radians_to_percentage(&self) -> Self;

    /// Converts this value from degrees to radians.
    fn degrees_to_radians(&self) -> Self;

    /// Converts this value from degrees to rotations.
    fn degrees_to_rotations(&self) -> Self;

    /// Converts this value from degrees to percentage.
    fn degrees_to_percentage(&self) -> Self;

    /// Converts this value from rotations to radians.
    fn rotations_to_radians(&self) -> Self;

    /// Converts this value from rotations to degrees.
    fn rotations_to_degrees(&self) -> Self;

    /// Converts this value from rotations to percentage.
    fn rotations_to_percentage(&self) -> Self;

    /// A constant for converting from percentage to radians.
    fn percentage_to_radians(&self) -> Self;

    /// Converts this value from percentage to degrees.
    fn percentage_to_degrees(&self) -> Self;

    /// Converts this value from percentage to degrees.
    fn percentage_to_rotations(&self) -> Self;
}

impl AngleValue for f32 {
    fn radians_to_degrees(&self) -> Self {
        self.to_degrees()
    }

    fn radians_to_rotations(&self) -> Self {
        self * 0.15915482
    }

    fn radians_to_percentage(&self) -> Self {
        self * 0.15915482 * 100.0
    }

    fn degrees_to_radians(&self) -> Self {
        self.to_radians()
    }

    fn degrees_to_rotations(&self) -> Self {
        self / 360.0
    }

    fn degrees_to_percentage(&self) -> Self {
        self / 360.0 * 100.0
    }

    fn rotations_to_radians(&self) -> Self {
        self * core::f32::consts::TAU
    }

    fn rotations_to_degrees(&self) -> Self {
        self * 360.0
    }

    fn rotations_to_percentage(&self) -> Self {
        self * 100.0
    }

    fn percentage_to_radians(&self) -> Self {
        self / 100.0 * core::f32::consts::TAU
    }

    fn percentage_to_degrees(&self) -> Self {
        self / 100.0 * 360.0
    }

    fn percentage_to_rotations(&self) -> Self {
        self / 100.0
    }
}

impl AngleValue for f64 {
    fn radians_to_degrees(&self) -> Self {
        self.to_degrees()
    }

    fn radians_to_rotations(&self) -> Self {
        self * 0.15915482422145
    }

    fn radians_to_percentage(&self) -> Self {
        self * 0.15915482422145 * 100.0
    }

    fn degrees_to_radians(&self) -> Self {
        self.to_radians()
    }

    fn degrees_to_rotations(&self) -> Self {
        self / 360.0
    }

    fn degrees_to_percentage(&self) -> Self {
        self / 360.0 * 100.0
    }

    fn rotations_to_radians(&self) -> Self {
        self * core::f64::consts::TAU
    }

    fn rotations_to_degrees(&self) -> Self {
        self * 360.0
    }

    fn rotations_to_percentage(&self) -> Self {
        self * 100.0
    }

    fn percentage_to_radians(&self) -> Self {
        self / 100.0 * core::f64::consts::TAU
    }

    fn percentage_to_degrees(&self) -> Self {
        self / 100.0 * 360.0
    }

    fn percentage_to_rotations(&self) -> Self {
        self / 100.0
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
