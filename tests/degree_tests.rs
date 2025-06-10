#[test]
fn degrees_by_type_alias() {
    let angle_by_type = AngleInDegrees::new(45.0);
    let angle_by_function = Angle::degrees(45.0);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn degrees_as_radians32() {
    let angle = Angle::degrees(90.0f32);
    assert_ulps_eq!(angle.as_radians().value, FRAC_PI_2_32);
    let angle = Angle::degrees(-180.0f32);
    assert_ulps_eq!(angle.as_radians().value, -PI_32);
}

#[test]
fn degrees_as_radians64() {
    let angle = Angle::degrees(90.0f64);
    assert_ulps_eq!(angle.as_radians().value, FRAC_PI_2_64);
    let angle = Angle::degrees(-180.0f64);
    assert_ulps_eq!(angle.as_radians().value, -PI_64);
}

#[test]
fn degrees_as_rotations32() {
    let angle = Angle::degrees(90.0f32);
    assert_ulps_eq!(angle.as_rotations().value, 0.25);
    let angle = Angle::degrees(-180.0f32);
    assert_ulps_eq!(angle.as_rotations().value, -0.5);
}

#[test]
fn degreess_as_rotations64() {
    let angle = Angle::degrees(90.0f64);
    assert_ulps_eq!(angle.as_rotations().value, 0.25);
    let angle = Angle::degrees(-180.0f64);
    assert_ulps_eq!(angle.as_rotations().value, -0.5);
}

#[test]
fn degrees_as_percentage32() {
    let angle = Angle::degrees(90.0f32);
    assert_ulps_eq!(angle.as_percentage().value, 25.0);
    let angle = Angle::degrees(-180.0f32);
    assert_ulps_eq!(angle.as_percentage().value, -50.0);
}

#[test]
fn degreess_as_percentage64() {
    let angle = Angle::degrees(90.0f64);
    assert_ulps_eq!(angle.as_percentage().value, 25.0);
    let angle = Angle::degrees(-180.0f64);
    assert_ulps_eq!(angle.as_percentage().value, -50.0);
}

#[test]
fn convert_degrees_to_radians() {
    let angle: AngleInRadians = Angle::degrees(90.0).convert();
    assert_ulps_eq!(angle.value, FRAC_PI_2_32);
}

#[test]
fn convert_degrees_to_degrees() {
    let angle: AngleInDegrees = Angle::degrees(90.0).convert();
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_degrees_to_rotations() {
    let angle: AngleInRotations = Angle::degrees(90.0).convert();
    assert_ulps_eq!(angle.value, 0.25);
}

#[test]
fn convert_degrees_to_percentages() {
    let angle: AngleInPercentage = Angle::degrees(90.0).convert();
    assert_ulps_eq!(angle.value, 25.0);
}

#[test]
fn convert_degrees_from_radians() {
    let angle = AngleInDegrees::from_other(Angle::radians(FRAC_PI_2_32));
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_degrees_from_degrees() {
    let angle = AngleInDegrees::from_other(Angle::degrees(90.0));
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_degrees_from_rotations() {
    let angle = AngleInDegrees::from_other(Angle::rotations(0.25));
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_degrees_from_percentages() {
    let angle = AngleInDegrees::from_other(Angle::percentage(25.0));
    assert_ulps_eq!(angle.value, 90.0);
}

use approx::assert_ulps_eq;
use bangle::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, ConvertAngle,
    FromOther,
};
use core::f32::consts::{FRAC_PI_2 as FRAC_PI_2_32, PI as PI_32};
use core::f64::consts::{FRAC_PI_2 as FRAC_PI_2_64, PI as PI_64};
