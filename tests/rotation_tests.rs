#[test]
fn rotations_by_type_alias() {
    let angle_by_type = AngleInRotations::new(42.0);
    let angle_by_function = Angle::rotations(42.0);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn rotations_as_radians32() {
    let angle = Angle::rotations(1.0f32);
    assert_ulps_eq!(angle.as_radians().value, std::f32::consts::TAU);
    let angle = Angle::rotations(-2.0f32);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f32::consts::TAU);
}

#[test]
fn rotations_as_radians64() {
    let angle = Angle::rotations(1.0f64);
    assert_ulps_eq!(angle.as_radians().value, std::f64::consts::TAU);
    let angle = Angle::rotations(-2.0f64);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f64::consts::TAU);
}

#[test]
fn rotations_as_degrees32() {
    let angle = Angle::rotations(1.0f32);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = Angle::rotations(-2.0f32);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn rotations_as_degrees64() {
    let angle = Angle::rotations(1.0f64);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = Angle::rotations(-2.0f64);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn rotations_as_percentage32() {
    let angle = Angle::rotations(1.0f32);
    assert_ulps_eq!(angle.as_percentage().value, 100.0);
    let angle = Angle::rotations(-2.0f32);
    assert_ulps_eq!(angle.as_percentage().value, -200.0);
}

#[test]
fn rotations_as_percentage64() {
    let angle = Angle::rotations(1.0f64);
    assert_ulps_eq!(angle.as_percentage().value, 100.0);
    let angle = Angle::rotations(-2.0f64);
    assert_ulps_eq!(angle.as_percentage().value, -200.0);
}

#[test]
fn convert_rotations_to_radians() {
    let angle: AngleInRadians = Angle::rotations(0.25).convert();
    assert_ulps_eq!(angle.value, FRAC_PI_2);
}

#[test]
fn convert_rotations_to_degrees() {
    let angle: AngleInDegrees = Angle::rotations(0.25).convert();
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_rotations_to_rotations() {
    let angle: AngleInRotations = Angle::rotations(90.0).convert();
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_rotations_to_percentages() {
    let angle: AngleInPercentage = Angle::rotations(0.25).convert();
    assert_ulps_eq!(angle.value, 25.0);
}

#[test]
fn convert_rotations_from_radians() {
    let angle = AngleInRotations::from_other(Angle::radians(FRAC_PI_2));
    assert_ulps_eq!(angle.value, 0.25, max_ulps = 13);
}

#[test]
fn convert_rotations_from_degrees() {
    let angle = AngleInRotations::from_other(Angle::degrees(90.0));
    assert_ulps_eq!(angle.value, 0.25);
}

#[test]
fn convert_rotations_from_rotations() {
    let angle = AngleInRotations::from_other(Angle::rotations(0.25));
    assert_ulps_eq!(angle.value, 0.25);
}

#[test]
fn convert_rotations_from_percentages() {
    let angle = AngleInRotations::from_other(Angle::percentage(25.0));
    assert_ulps_eq!(angle.value, 0.25);
}

use approx::assert_ulps_eq;
use bangle::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, ConvertAngle,
    FromOther,
};
use core::f32::consts::FRAC_PI_2;
