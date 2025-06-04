/// An angle which has been specified in degrees.
pub type AngleInDegrees<T = f32> = Angle<Degrees, T>;

impl<T> From<&AngleInRadians<T>> for AngleInDegrees<T>
where
    T: AngleValue,
{
    fn from(radians: &AngleInRadians<T>) -> Self {
        Self::from(radians.value.radians_to_degrees())
    }
}

impl<T> From<&AngleInRotations<T>> for AngleInDegrees<T>
where
    T: AngleValue,
{
    fn from(radians: &AngleInRotations<T>) -> Self {
        Self::from(radians.value.rotations_to_degrees())
    }
}

impl<T> From<&AngleInPercentage<T>> for AngleInDegrees<T>
where
    T: AngleValue,
{
    fn from(percentage: &AngleInPercentage<T>) -> Self {
        Self::from(percentage.value.percentage_to_degrees())
    }
}

// This blanket implementation allows converting from an owned value.
impl<Unit, T> From<Angle<Unit, T>> for AngleInDegrees<T>
where
    Unit: AngleUnit,
    T: AngleValue,
    AngleInDegrees<T>: for<'a> From<&'a Angle<Unit, T>>,
{
    fn from(angle: Angle<Unit, T>) -> Self {
        Self::from(&angle)
    }
}

use crate::{
    Angle, AngleInPercentage, AngleInRadians, AngleInRotations, AngleUnit, AngleValue, Degrees,
};
