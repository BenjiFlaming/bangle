//! # Bangle
//!
//! The `bangle` crate provides strongly-typed floating point angles,
//! specified in radians, degrees, rotations, or percentages,
//! with efficient and ergonomic conversion between units.
//! Both `f32` and `f64` numeric types are supported,
//! with `f32` being the implicit default.
//! All conversions, for each unit and numeric type,
//! are covered by tests, and the library is fully `no_std` compatible.
//! `bangle` is especially intended for situations where local code clarity
//! may be enhanced by specifying or modifying an angle in units other than radians.
//!
//!
//! ## Creating an [`Angle`]
//!
//! The core type is simply called [`Angle`], and it has a dedicated constructor
//! named for each supported unit type:
//!
//! ```rust
//! use bangle::Angle;
//! use core::f32::consts::PI;
//!
//! let degrees = Angle::degrees(90.0);
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
//! ```rust
//! # use bangle::Angle;
//! #
//! let degrees = Angle::degrees(90.0);
//! let radians = degrees.as_radians();
//! let rotations = degrees.as_rotations();
//! let percentage = degrees.as_percentage();
//! ```
//!
//!
//! ## Accessing the numeric value
//!
//! The value of an [`Angle`] is stored in the public [`value`][Angle::value] member:
//!
//! ```rust
//! # use bangle::Angle;
//! #
//! let mut degrees = Angle::degrees(90.0);
//! assert_eq!(degrees.value, 90.0);
//!
//! degrees.value = 180.0;
//! assert_eq!(degrees.value, 180.0);
//!
//! degrees.value *= 2.0;
//! assert_eq!(degrees.value, 360.0);
//! ```
//!
//!
//! ## Adding and subtracting
//!
//! You can add and subtract angles of different types,
//! and the result will be an angle of whatever type is on
//! the lefthand side of the operation:
//!
//! ```rust
//! # use bangle::Angle;
//! # use core::f32::consts::PI;
//! # use approx::assert_ulps_eq;
//! #
//! let degrees = Angle::degrees(90.0);
//! let radians = Angle::radians(PI / 2.0);
//! let rotations = Angle::rotations(0.25);
//! let percentage = Angle::percentage(25.0);
//!
//! let full_circle_in_degrees = degrees + radians + rotations + percentage;
//! assert_ulps_eq!(full_circle_in_degrees.value, 360.0);
//! ```
//!
//!
//! ## Multiplying and dividing
//!
//! You can also multiply and divide angles by floating point numbers:
//!
//! ```rust
//! # use bangle::Angle;
//! # use core::f32::consts::PI;
//! # use approx::assert_ulps_eq;
//! #
//! let mut degrees = Angle::degrees(90.0);
//!
//! degrees *= 4.0;
//! assert_ulps_eq!(degrees.value, 360.0);
//!
//! degrees /= 2.0;
//! assert_ulps_eq!(degrees.value, 180.0);
//! ```
//!
//!
//! ## Function arguments
//!
//! If you want to ensure that a function receives an [`Angle`] of a particular type,
//! you can specify that type explicitly:
//!
//! ```rust
//! # use bangle::{Angle, Radians, AngleInRadians};
//! # use approx::assert_ulps_eq;
//! #
//! fn twice(angle: Angle<Radians>) -> Angle<Radians> {
//!   angle * 2.0
//! }
//!
//! let angle = Angle::radians(5.0);
//! assert_ulps_eq!(twice(angle).value, 10.0);
//! ```
//!
//! Or use one of the crate's type aliases:
//!
//! ```rust
//! # use bangle::{Angle, Radians};
//! # use approx::assert_ulps_eq;
//! #
//! use bangle::AngleInRadians;
//!
//! fn twice(angle: AngleInRadians) -> AngleInRadians {
//!   angle * 2.0
//! }
//!
//! let angle = Angle::radians(5.0);
//! assert_ulps_eq!(twice(angle).value, 10.0);
//! ```
//!
//! Alternatively, you could allow callers to provide other angle units,
//! or raw floating point numbers, by using the [`Into`] trait:
//!
//! ```rust
//! # use bangle::{Angle, Radians, AngleInRadians};
//! # use approx::assert_ulps_eq;
//! #
//! fn twice(angle: impl Into<AngleInRadians>) -> AngleInRadians {
//!   let radians = angle.into();
//!   radians * 2.0
//! }
//!
//! let radians = Angle::radians(5.0);
//! let degrees = Angle::degrees(5.0);
//!
//! assert_ulps_eq!(twice(radians).value, 10.0);
//! assert_ulps_eq!(twice(degrees).as_degrees().value, 10.0);
//! assert_ulps_eq!(twice(5.0).value, 10.0);
//! ```
//!
//! As a final option, you could make a function completely generic:
//!
//! ```rust
//! # use bangle::{Angle, AngleUnit};
//! # use approx::assert_ulps_eq;
//! #
//! fn twice<T: AngleUnit>(angle: Angle<T>) -> Angle<T> {
//!   angle * 2.0
//! }
//!
//! let radians = Angle::radians(5.0);
//! let degrees = Angle::degrees(5.0);
//!
//! assert_ulps_eq!(twice(radians).value, 10.0);
//! assert_ulps_eq!(twice(degrees).value, 10.0);
//! ```
//!
//!
//! ## High resolution (64-bit) angles
//!
//! By default, Bangle uses [`f32`] numbers for storing angle values.
//! If your use case requires higher precision,
//! you can use [`f64`] numbers by overriding the appropriate generic argument:
//!
//! ```rust
//! # use bangle::{Angle, Radians, AngleInRadians};
//! # use approx::assert_ulps_eq;
//! fn sum_angles(angle1: Angle<Radians, f64>, angle2: AngleInRadians<f64>) -> AngleInRadians<f64> {
//!   angle1 + angle2
//! }
//!
//! let angle_a = Angle::radians(4.0);
//! let angle_b = Angle::radians(2.0);
//!
//! assert_ulps_eq!(sum_angles(angle_a, angle_b).value, 6.0);
//! ```
//!
//! Note that, in most cases, you don't need to do anything unusual when creating an [`Angle`]
//! to be passed into such functions - Rust will typically infer the use of [`f64`] numbers automatically.
//!
//!
//! ## Running the tests
//!
//! Bangle includes a full suite of tests, which (according to [Tarpaulin]) provides 100% coverage of all library code.
//! (To verify this, install [Tarpaulin], use `cargo tarpaulin -o html` within Bangle's crate directory to run all tests,
//! and examine the HTML output file in the Bangle crate's root directory.)
//!
//! You can also run all tests via the `cargo test` command,
//! or use [cargo-nextest] (recommended) to run the main test suite via the command `cargo nextest run`,
//! and then use `cargo test --doc` to test the examples in this documentation.
//!
//! [Tarpaulin]: https://github.com/xd009642/tarpaulin
//! [cargo-nextest]: https://nexte.st/
//!
//!
//! ## In conclusion
//!
//! If you find this library useful, and you're curious to know what else I might be building or doing,
//! you might enjoy visiting [my website](https://benjiflaming.com), where you'll find content dating back
//! all the way to the late 90s.
//!

#![no_std]
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
