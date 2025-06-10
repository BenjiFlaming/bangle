/// An angle, of generic unit `U` and generic floating point type `T`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Angle<U: AngleUnit, T: AngleValue = f32> {
    /// The value of this angle, as a floating point number of type `T`.
    pub value: T,

    /// The type of unit in which this angle is specified.
    pub angle_unit: PhantomData<U>,
}

// Allows the construction of an angle from a raw numeric type.
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

    /// This angle, as represented in radians.
    pub fn as_radians(&self) -> AngleInRadians<T> {
        AngleInRadians::from_other(self)
    }

    /// This angle, as represented in degrees.
    pub fn as_degrees(&self) -> AngleInDegrees<T> {
        AngleInDegrees::from_other(self)
    }

    /// This angle, as represented in rotations.
    pub fn as_rotations(&self) -> AngleInRotations<T> {
        AngleInRotations::from_other(self)
    }

    /// This angle, as represented in percentage.
    pub fn as_percentage(&self) -> AngleInPercentage<T> {
        AngleInPercentage::from_other(self)
    }
}

impl<U, T, O> Add<Angle<O, T>> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
    O: AngleUnit,
    Self: FromOther<T>,
{
    type Output = Self;

    fn add(self, other: Angle<O, T>) -> Self::Output {
        Self::new(self.value + Self::from_other(other).value)
    }
}

impl<U, T, O> AddAssign<Angle<O, T>> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
    O: AngleUnit,
    Self: FromOther<T>,
{
    fn add_assign(&mut self, other: Angle<O, T>) {
        self.value += Self::from_other(other).value
    }
}

impl<U, T, O> Sub<Angle<O, T>> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
    O: AngleUnit,
    Self: FromOther<T>,
{
    type Output = Self;

    fn sub(self, other: Angle<O, T>) -> Self::Output {
        Self::new(self.value - Self::from_other(other).value)
    }
}

impl<U, T, O> SubAssign<Angle<O, T>> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
    O: AngleUnit,
    Self: FromOther<T>,
{
    fn sub_assign(&mut self, other: Angle<O, T>) {
        self.value -= Self::from_other(other).value
    }
}

impl<U, T> Mul<T> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

impl<U, T> MulAssign<T> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
{
    fn mul_assign(&mut self, rhs: T) {
        self.value *= rhs;
    }
}

impl<U, T> Div<T> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

impl<U, T> DivAssign<T> for Angle<U, T>
where
    U: AngleUnit,
    T: AngleValue,
{
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
    }
}

use crate::{
    AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, AngleUnit, AngleValue,
    FromOther,
};
use core::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
