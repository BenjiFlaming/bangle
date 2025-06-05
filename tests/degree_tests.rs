#[test]
fn degrees_by_name() {
    let angle_by_type = AngleInDegrees::new(45.0);
    let angle_by_function = Angle::degrees(45.0);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn degrees_to_radians32() {
    let angle = AngleInDegrees::new(90.0f32);
    assert_ulps_eq!(angle.as_radians().value, FRAC_PI_2_32);
    let angle = AngleInDegrees::new(-180.0f32);
    assert_ulps_eq!(angle.as_radians().value, -PI_32);
}

#[test]
fn degrees_to_radians64() {
    let angle = AngleInDegrees::new(90.0f64);
    assert_ulps_eq!(angle.as_radians().value, FRAC_PI_2_64);
    let angle = AngleInDegrees::new(-180.0f64);
    assert_ulps_eq!(angle.as_radians().value, -PI_64);
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
use bangle::{Angle, AngleInDegrees};
use core::f32::consts::{FRAC_PI_2 as FRAC_PI_2_32, PI as PI_32};
use core::f64::consts::{FRAC_PI_2 as FRAC_PI_2_64, PI as PI_64};
