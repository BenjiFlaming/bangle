#[test]
fn degrees_to_radians() {
    let angle = AngleInDegrees::new(90.0);
    assert_ulps_eq!(angle.as_radians().value, FRAC_PI_2);
    let angle = AngleInDegrees::new(-180.0);
    assert_ulps_eq!(angle.as_radians().value, -PI);
}

#[test]
fn degrees_to_rotations32() {
    let angle = AngleInDegrees::new(90.0f32);
    assert_ulps_eq!(angle.as_rotations().value, 0.25);
    let angle = AngleInDegrees::new(-180.0f32);
    assert_ulps_eq!(angle.as_rotations().value, -0.5);
}

#[test]
fn degreess_to_rotations64() {
    let angle = AngleInDegrees::new(90.0f64);
    assert_ulps_eq!(angle.as_rotations().value, 0.25);
    let angle = AngleInDegrees::new(-180.0f64);
    assert_ulps_eq!(angle.as_rotations().value, -0.5);
}

#[test]
fn degrees_to_percentage32() {
    let angle = AngleInDegrees::new(90.0f32);
    assert_ulps_eq!(angle.as_percentage().value, 25.0);
    let angle = AngleInDegrees::new(-180.0f32);
    assert_ulps_eq!(angle.as_percentage().value, -50.0);
}

#[test]
fn degreess_to_percentage64() {
    let angle = AngleInDegrees::new(90.0f64);
    assert_ulps_eq!(angle.as_percentage().value, 25.0);
    let angle = AngleInDegrees::new(-180.0f64);
    assert_ulps_eq!(angle.as_percentage().value, -50.0);
}

use approx::assert_ulps_eq;
use bangle::AngleInDegrees;
use std::f32::consts::{FRAC_PI_2, PI};
