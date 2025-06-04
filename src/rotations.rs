/// An angle which has been specified in rotations.
pub type AngleInRotations<T = f32> = Angle<Rotations, T>;

impl<T> From<&AngleInRadians<T>> for AngleInRotations<T>
where
    T: AngleValue,
{
    fn from(radians: &AngleInRadians<T>) -> Self {
        Self::from(radians.value * T::radians_to_rotations())
    }
}

impl<T> From<&AngleInDegrees<T>> for AngleInRotations<T>
where
    T: AngleValue,
{
    fn from(degrees: &AngleInDegrees<T>) -> Self {
        Self::from(degrees.value / T::degree_full_circle())
    }
}

impl<T> From<&AngleInPercentage<T>> for AngleInRotations<T>
where
    T: AngleValue,
{
    fn from(percentage: &AngleInPercentage<T>) -> Self {
        Self::from(percentage.value / T::full_percentage())
    }
}

// This blanket implementation allows converting from an owned value.
impl<Unit, T> From<Angle<Unit, T>> for AngleInRotations<T>
where
    Unit: AngleUnit,
    T: AngleValue,
    AngleInRotations<T>: for<'a> From<&'a Angle<Unit, T>>,
{
    fn from(angle: Angle<Unit, T>) -> Self {
        Self::from(&angle)
    }
}

use crate::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleUnit, AngleValue, Rotations,
};
