/// An angle, of generic unit `U` and generic floating point type `T`.
#[derive(Clone, Copy, Debug)]
pub struct Angle<U: AngleUnit, T: AngleValue = f32> {
    /// The value of this angle, specified
    pub value: T,

    /// The type of unit in which this angle is specified.
    pub angle_unit: PhantomData<U>,
}

impl<U, T> From<T> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<U, T> Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
{
    /// Constructs a new angle from the specified value.
    pub const fn new(value: T) -> Self {
        Self {
            value,
            angle_unit: PhantomData,
        }
    }

    /// Constructs a new angle, represented in radians, from the specified value.
    pub fn radians(value: T) -> AngleInRadians<T>
    where
        AngleInRadians<T>: for<'a> From<&'a Self>,
    {
        AngleInRadians::new(value)
    }

    /// Constructs a new angle, represented in degrees, from the specified value.
    pub fn degrees(value: T) -> AngleInDegrees<T>
    where
        AngleInDegrees<T>: for<'a> From<&'a Self>,
    {
        AngleInDegrees::new(value)
    }

    /// Constructs a new angle, represented in rotations, from the specified value.
    pub fn rotations(value: T) -> AngleInRotations<T>
    where
        AngleInRotations<T>: for<'a> From<&'a Self>,
    {
        AngleInRotations::new(value)
    }

    /// Constructs a new angle, represented in percentage, from the specified value.
    pub fn percentage(value: T) -> AngleInPercentage<T>
    where
        AngleInPercentage<T>: for<'a> From<&'a Self>,
    {
        AngleInPercentage::new(value)
    }

    /// This angle, as represented in radians.
    pub fn as_radians(&self) -> AngleInRadians<T>
    where
        AngleInRadians<T>: for<'a> From<&'a Self>,
    {
        AngleInRadians::from(self)
    }

    /// This angle, as represented in degrees.
    pub fn as_degrees(&self) -> AngleInDegrees<T>
    where
        AngleInDegrees<T>: for<'a> From<&'a Self>,
    {
        AngleInDegrees::from(self)
    }

    /// This angle, as represented in rotations.
    pub fn as_rotations(&self) -> AngleInRotations<T>
    where
        AngleInRotations<T>: for<'a> From<&'a Self>,
    {
        AngleInRotations::from(self)
    }

    /// This angle, as represented in percentage.
    pub fn as_percentage(&self) -> AngleInPercentage<T>
    where
        AngleInPercentage<T>: for<'a> From<&'a Self>,
    {
        AngleInPercentage::from(self)
    }
}

use crate::{
    AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, AngleUnit, AngleValue,
};
use std::{fmt::Debug, marker::PhantomData};
