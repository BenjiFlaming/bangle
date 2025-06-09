/// This marker trait allows different "types" of angles to be defined.
pub trait AngleUnit: Clone + Copy + Debug {}

#[derive(Clone, Copy, Debug)]
/// An angle specified in degrees.
pub struct Degrees;
impl AngleUnit for Degrees {}

#[derive(Clone, Copy, Debug)]
/// An angle specified in radians.
pub struct Radians;
impl AngleUnit for Radians {}

#[derive(Clone, Copy, Debug)]
/// An angle specified in rotations.
pub struct Rotations;
impl AngleUnit for Rotations {}

#[derive(Clone, Copy, Debug)]
/// An angle specified in percentage.
pub struct Percentage;
impl AngleUnit for Percentage {}

use core::fmt::Debug;
