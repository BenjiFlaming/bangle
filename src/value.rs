/// Allows a numeric type to be converted between angle units.
pub trait AngleValue:
    Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Default
    + PartialEq
    + PartialOrd
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

    /// Negates this value in-place (multiplies it by -1.0).
    fn negate(&mut self);
}

impl AngleValue for f32 {
    fn radians_to_degrees(&self) -> Self {
        self.to_degrees()
    }

    fn radians_to_rotations(&self) -> Self {
        self * 0.15915482
    }

    fn radians_to_percentage(&self) -> Self {
        self * 15.915482
    }

    fn degrees_to_radians(&self) -> Self {
        self.to_radians()
    }

    fn degrees_to_rotations(&self) -> Self {
        self * 0.002_777_777_8
    }

    fn degrees_to_percentage(&self) -> Self {
        self * 0.277_777_8
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
        self * 0.062_831_86
    }

    fn percentage_to_degrees(&self) -> Self {
        self * 3.6
    }

    fn percentage_to_rotations(&self) -> Self {
        self * 0.01
    }

    fn negate(&mut self) {
        *self *= -1.0;
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
        self * 15.915482422145
    }

    fn degrees_to_radians(&self) -> Self {
        self.to_radians()
    }

    fn degrees_to_rotations(&self) -> Self {
        self * 0.002777777777777778
    }

    fn degrees_to_percentage(&self) -> Self {
        self * 0.2777777777777778
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
        self * 0.06283185307179587
    }

    fn percentage_to_degrees(&self) -> Self {
        self * 3.6
    }

    fn percentage_to_rotations(&self) -> Self {
        self * 0.01
    }

    fn negate(&mut self) {
        *self *= -1.0;
    }
}

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
