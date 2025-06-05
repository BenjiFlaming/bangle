/// An angle which has been specified in rotations.
pub type AngleInPercentage<T = f32> = Angle<Percentage, T>;

impl<T> Angle<Percentage, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in percentage, from the specified value.
    pub fn percentage(value: T) -> AngleInPercentage<T> {
        AngleInPercentage::new(value)
    }
}

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
impl<U, T> From<Angle<U, T>> for AngleInPercentage<T>
where
    U: AngleUnit,
    T: AngleValue,
    AngleInPercentage<T>: for<'a> From<&'a Angle<U, T>>,
{
    fn from(angle: Angle<U, T>) -> Self {
        Self::from(&angle)
    }
}

use crate::{
    Angle, AngleInDegrees, AngleInRadians, AngleInRotations, AngleUnit, AngleValue, Percentage,
};
