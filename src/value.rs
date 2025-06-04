/// A generic trait for [f32] and [f64] floating point types.
pub trait AngleValue: Mul<Self, Output = Self> + Div<Self, Output = Self> + Copy + Sized {
    /// Converts this value from degrees to radians.
    fn degrees_to_radians(&self) -> Self;

    /// Converts this value from radians to degrees.
    fn radians_to_degrees(&self) -> Self;

    /// Converts this value from rotations to radians.
    fn rotations_to_radians(&self) -> Self;

    /// The mathematical constant tau, for this data type.
    fn tau() -> Self;

    /// The number 360, as represented in this type.
    fn degree_full_circle() -> Self;

    /// A constant for converting from radians to rotations.
    fn radians_to_rotations() -> Self;

    /// The number 100, as represented in this type.
    fn full_percentage() -> Self;

    /// Converts this value from percentage to degrees.
    fn percentage_to_degrees(&self) -> Self;

    /// A constant for converting from percentage to radians.
    fn percentage_to_radians(&self) -> Self;

    /// Converts this value from rotations to degrees.
    fn rotations_to_degrees(&self) -> Self;
}

impl AngleValue for f32 {
    fn degrees_to_radians(&self) -> Self {
        self.to_radians()
    }

    fn radians_to_degrees(&self) -> Self {
        self.to_degrees()
    }

    fn tau() -> Self {
        core::f32::consts::TAU
    }

    fn degree_full_circle() -> Self {
        360.0
    }

    fn radians_to_rotations() -> Self {
        0.15915482
    }

    fn full_percentage() -> Self {
        100.0
    }

    fn percentage_to_degrees(&self) -> Self {
        self / 100.0 * 360.0
    }

    fn percentage_to_radians(&self) -> Self {
        self / 100.0 * core::f32::consts::TAU
    }

    fn rotations_to_radians(&self) -> Self {
        self * core::f32::consts::TAU
    }

    fn rotations_to_degrees(&self) -> Self {
        self * 360.0
    }
}

impl AngleValue for f64 {
    fn degrees_to_radians(&self) -> Self {
        self.to_radians()
    }

    fn radians_to_degrees(&self) -> Self {
        self.to_degrees()
    }

    fn tau() -> Self {
        core::f64::consts::TAU
    }

    fn degree_full_circle() -> Self {
        360.0
    }

    fn radians_to_rotations() -> Self {
        0.15915482422145
    }

    fn full_percentage() -> Self {
        100.0
    }

    fn percentage_to_degrees(&self) -> Self {
        self / 100.0 * 360.0
    }

    fn percentage_to_radians(&self) -> Self {
        self / 100.0 * core::f64::consts::TAU
    }

    fn rotations_to_radians(&self) -> Self {
        self * core::f64::consts::TAU
    }

    fn rotations_to_degrees(&self) -> Self {
        self * 360.0
    }
}

use std::ops::{Div, Mul};
