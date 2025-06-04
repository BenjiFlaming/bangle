//! This crate implements statically-typed angles,
//! and provides conversions between different representations.

#![warn(missing_docs, clippy::missing_docs_in_private_items)]

pub use self::{
    angle::Angle,
    degrees::AngleInDegrees,
    percentage::AngleInPercentage,
    radians::AngleInRadians,
    rotations::AngleInRotations,
    unit::{AngleUnit, Degrees, Percentage, Radians, Rotations},
    value::AngleValue,
};

/// The top-level Angle type.
mod angle;

/// Various units in which angles may be specified.
mod unit;

/// Provides a trait which allows [f32] and [f64] to be used interchangeably.
mod value;

/// Angles which are specified in radians.
mod radians;

/// Angles which are specified in degrees.
mod degrees;

/// Angles which are specified in rotations.
mod rotations;

/// Angles which are specified in percent.
mod percentage;
