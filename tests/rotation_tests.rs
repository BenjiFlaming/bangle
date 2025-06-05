#[test]
fn rotations_by_name() {
    let angle_by_type = AngleInRotations::new(42.0);
    let angle_by_function = Angle::rotations(42.0);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn rotations_to_radians32() {
    let angle = AngleInRotations::new(1.0f32);
    assert_ulps_eq!(angle.as_radians().value, std::f32::consts::TAU);
    let angle = AngleInRotations::new(-2.0f32);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f32::consts::TAU);
}

#[test]
fn rotations_to_radians64() {
    let angle = AngleInRotations::new(1.0f64);
    assert_ulps_eq!(angle.as_radians().value, std::f64::consts::TAU);
    let angle = AngleInRotations::new(-2.0f64);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f64::consts::TAU);
}

#[test]
fn rotations_to_degrees32() {
    let angle = AngleInRotations::new(1.0f32);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = AngleInRotations::new(-2.0f32);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn rotations_to_degrees64() {
    let angle = AngleInRotations::new(1.0f64);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = AngleInRotations::new(-2.0f64);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn rotations_to_percentage32() {
    let angle = AngleInRotations::new(1.0f32);
    assert_ulps_eq!(angle.as_percentage().value, 100.0);
    let angle = AngleInRotations::new(-2.0f32);
    assert_ulps_eq!(angle.as_percentage().value, -200.0);
}

#[test]
fn rotations_to_percentage64() {
    let angle = AngleInRotations::new(1.0f64);
    assert_ulps_eq!(angle.as_percentage().value, 100.0);
    let angle = AngleInRotations::new(-2.0f64);
    assert_ulps_eq!(angle.as_percentage().value, -200.0);
}

use approx::assert_ulps_eq;
use bangle::{Angle, AngleInRotations};
