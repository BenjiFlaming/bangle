/// An angle which has been specified in degrees.
pub type AngleInDegrees<T = f32> = Angle<Degrees, T>;

impl<T> Angle<Degrees, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in degrees, from the specified value.
    pub fn degrees(value: T) -> AngleInDegrees<T> {
        AngleInDegrees::new(value)
    }
}

impl<T> FromOther<T> for Angle<Degrees, T>
where
    T: AngleValue,
{
    fn from_other<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_degrees(angle.borrow().value)
    }
}

impl<T> ConvertAngle<T> for Angle<Degrees, T>
where
    T: AngleValue,
{
    fn convert<U>(self) -> Angle<U, T>
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::from_degrees(self.value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, ConvertAngle, Degrees, FromOther};
use core::borrow::Borrow;
