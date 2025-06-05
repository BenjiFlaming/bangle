/// An angle which has been specified in rotations.
pub type AngleInRotations<T = f32> = Angle<Rotations, T>;

impl<T> Angle<Rotations, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in rotations, from the specified value.
    pub fn rotations(value: T) -> AngleInRotations<T> {
        AngleInRotations::new(value)
    }
}

impl<T> From<&AngleInRadians<T>> for AngleInRotations<T>
where
    T: AngleValue,
{
    fn from(radians: &AngleInRadians<T>) -> Self {
        Self::from(radians.value.radians_to_rotations())
    }
}

impl<T> From<&AngleInDegrees<T>> for AngleInRotations<T>
where
    T: AngleValue,
{
    fn from(degrees: &AngleInDegrees<T>) -> Self {
        Self::from(degrees.value.degrees_to_rotations())
    }
}

impl<T> From<&AngleInPercentage<T>> for AngleInRotations<T>
where
    T: AngleValue,
{
    fn from(percentage: &AngleInPercentage<T>) -> Self {
        Self::from(percentage.value.percentage_to_rotations())
    }
}

// This blanket implementation allows converting from an owned value.
impl<U, T> From<Angle<U, T>> for AngleInRotations<T>
where
    U: AngleUnit,
    T: AngleValue,
    AngleInRotations<T>: for<'a> From<&'a Angle<U, T>>,
{
    fn from(angle: Angle<U, T>) -> Self {
        Self::from(&angle)
    }
}

use crate::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleUnit, AngleValue, Rotations,
};
