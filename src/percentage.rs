/// An angle which has been specified in rotations.
pub type AngleInPercentage<T = f32> = Angle<Percentage, T>;

impl<T> From<&AngleInRadians<T>> for AngleInPercentage<T>
where
    T: AngleValue,
{
    fn from(radians: &AngleInRadians<T>) -> Self {
        Self::from(radians.value.radians_to_percentage())
    }
}

impl<T> From<&AngleInDegrees<T>> for AngleInPercentage<T>
where
    T: AngleValue,
{
    fn from(degrees: &AngleInDegrees<T>) -> Self {
        Self::from(degrees.value.degrees_to_percentage())
    }
}

impl<T> From<&AngleInRotations<T>> for AngleInPercentage<T>
where
    T: AngleValue,
{
    fn from(rotations: &AngleInRotations<T>) -> Self {
        Self::from(rotations.value.rotations_to_percentage())
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
