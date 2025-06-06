#![doc = include_str!("../README.md")]

//! ## Creating an [`Angle`]
//!
//! The core type is simply called [`Angle`], and it can be instantiated
//! through a variety of functions, e.g.
//!
//! ```ignore
//! let degrees = Angle::degrees(90);
//! let radians = Angle::radians(PI / 2.0);
//! let rotations = Angle::rotations(0.25);
//! let percentage = Angle::percentage(25.0);
//! ```
//!
//!
//! ## Converting between types
//!
//! Once you have an [`Angle`], you can convert it to any other type
//! via similarly named functions:
//!
//! ```ignore
//! let degrees = Angle::degrees(90);
//! let radians = degrees.as_radians();
//! let rotations = degrees.as_rotations();
//! let percentage = degrees.as_percentage();
//! ```
//!
//!
//! ## Adding and subtracting
//!
//! You can add and subtract angles of different types,
//! and the result will be an angle of whatever type is on
//! the lefthand side of the operation:
//!
//! ```ignore
//! let degrees = Angle::degrees(90);
//! let radians = Angle::radians(PI / 2.0);
//! let rotations = Angle::rotations(0.25);
//! let percentage = Angle::percentage(25.0);
//!
//! let full_circle_in_degrees = degrees + radians + rotations + percentage;
//! ```
//!
//!
//! ## As a function argument
//!
//! If you want to ensure that a function receives an [`Angle`] of a particular type,
//! you can specify that type using either an explicit type or one of the crate's type aliases.
//! Alternatively, you can allow callers to provide a raw number,
//! by using the [`Into`] trait:
//!
//! ```ignore
//! fn angle_magic(first_angle: Angle<Radians>, second_angle: AngleInRadians) {
//!   // do stuff
//! }
//!
//! fn flexible_angle(angle: impl Into<AngleInRadians>) {
//!   let radians = angle.into();
//!   // do stuff
//! }
//! ```
//!
//! The first function above requires calling code to either have a value of the appropriate type,
//! or create one on-the-fly, while the second function allows either raw numbers or angles of any type
//! (with conversion being performed automatically).
//!
//! ```ignore
//! let radians = Angle::radians(4.0);
//! let degrees = Angle::degrees(2.0);
//!
//! angle_magic(radians, degrees.into());
//! angle_magic(Angle::radians(4.0), Angle::radians(2.0));
//!
//! flexible_angle(42.0);
//! flexible_angle(radians);
//! flexible_angle(degrees);
//! ```
//!
//!
//! ## High resolution (64-bit) angles
//!
//! You can also specify that 64-bit numbers be used, instead of the default 32-bit.
//! To do this, simply override the second generic argument.  As an example,
//! here is how the functions above could specify 64-bit numbers:
//!
//! ```ignore
//! fn angle_magic(first_angle: Angle<Radians, f64>, second_angle: AngleInRadians<f64>) {
//!   // do stuff
//! }
//!
//! fn flexible_angle(angle: impl Into<AngleInRadians<f64>>) {
//!   let radians = angle.into();
//!   // do stuff
//! }
//! ```
//!
//! ## Running the tests
//!
//! At the time of writing, the code snippets in this documentation are not set up properly
//! to function as tests, so the `cargo test` command will ignore them.
//! The `tests/` directory contains a full suite of tests, however, which [Tarpaulin] confirms
//! provides 100% coverage of all library code.
//! (Run `cargo tarpaulin -o html`, and examine the output HTML file in this crate's root directory.)
//!
//! You may also wish to consider using [cargo-nextest] for faster test running,
//! with more concise reporting of results.
//! (I was late in discovering this tool, so just passing this info along.)
//!
//! [Tarpaulin]: https://github.com/xd009642/tarpaulin
//! [cargo-nextest]: https://nexte.st/
//!
//!

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
