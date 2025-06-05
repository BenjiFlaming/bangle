/// An angle which has been specified in radians.
pub type AngleInRadians<T = f32> = Angle<Radians, T>;

impl<T> Angle<Radians, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in radians, from the specified value.
    pub fn radians(value: T) -> Angle<Radians, T> {
        AngleInRadians::new(value)
    }
}

impl<T> From<&AngleInDegrees<T>> for AngleInRadians<T>
where
    T: AngleValue,
{
    fn from(degrees: &AngleInDegrees<T>) -> Self {
        Self::from(degrees.value.degrees_to_radians())
    }
}

impl<T> From<&AngleInRotations<T>> for AngleInRadians<T>
where
    T: AngleValue,
{
    fn from(rotations: &AngleInRotations<T>) -> Self {
        Self::from(rotations.value.rotations_to_radians())
    }
}

impl<T> From<&AngleInPercentage<T>> for AngleInRadians<T>
where
    T: AngleValue,
{
    fn from(percentage: &AngleInPercentage<T>) -> Self {
        Self::from(percentage.value.percentage_to_radians())
    }
}

// This blanket implementation allows converting from an owned value.
impl<Unit, T> From<Angle<Unit, T>> for AngleInRadians<T>
where
    Unit: AngleUnit,
    T: AngleValue,
    AngleInRadians<T>: for<'a> From<&'a Angle<Unit, T>>,
{
    fn from(angle: Angle<Unit, T>) -> Self {
        Self::from(&angle)
    }
}

use crate::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRotations, AngleUnit, AngleValue, Radians,
};
