/// An angle which has been specified in rotations.
pub type AngleInPercentage<T = f32> = Angle<Percentage, T>;

impl From<&AngleInRadians<f32>> for AngleInPercentage<f32> {
    fn from(radians: &AngleInRadians<f32>) -> Self {
        Self::from(radians.value * 0.15915482 * 100.0)
    }
}

impl From<&AngleInRadians<f64>> for AngleInPercentage<f64> {
    fn from(radians: &AngleInRadians<f64>) -> Self {
        Self::from(radians.value * 0.15915482422145 * 100.0)
    }
}

impl From<&AngleInDegrees<f32>> for AngleInPercentage<f32> {
    fn from(degrees: &AngleInDegrees<f32>) -> Self {
        Self::from(degrees.value / 360.0 * 100.0)
    }
}

impl From<&AngleInDegrees<f64>> for AngleInPercentage<f64> {
    fn from(degrees: &AngleInDegrees<f64>) -> Self {
        Self::from(degrees.value / 360.0 * 100.0)
    }
}

impl From<&AngleInRotations<f32>> for AngleInPercentage<f32> {
    fn from(rotations: &AngleInRotations<f32>) -> Self {
        Self::from(rotations.value * 100.0)
    }
}

impl From<&AngleInRotations<f64>> for AngleInPercentage<f64> {
    fn from(rotations: &AngleInRotations<f64>) -> Self {
        Self::from(rotations.value * 100.0)
    }
}

// This blanket implementation allows converting from an owned value.
impl<Unit, T> From<Angle<Unit, T>> for AngleInPercentage<T>
where
    Unit: AngleUnit,
    T: AngleValue,
    AngleInPercentage<T>: for<'a> From<&'a Angle<Unit, T>>,
{
    fn from(angle: Angle<Unit, T>) -> Self {
        Self::from(&angle)
    }
}

use crate::{
    Angle, AngleInDegrees, AngleInRadians, AngleInRotations, AngleUnit, AngleValue, Percentage,
};
