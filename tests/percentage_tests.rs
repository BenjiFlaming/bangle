#[test]
fn percentage_by_type_alias() {
    let angle_by_type = AngleInPercentage::new(42.0);
    let angle_by_function = Angle::percentage(42.0);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn percentage_as_radians32() {
    let angle = Angle::percentage(100.0f32);
    assert_ulps_eq!(angle.as_radians().value, std::f32::consts::TAU);
    let angle = Angle::percentage(-200.0f32);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f32::consts::TAU);
}

#[test]
fn percentage_as_radians64() {
    let angle = Angle::percentage(100.0f64);
    assert_ulps_eq!(angle.as_radians().value, std::f64::consts::TAU);
    let angle = Angle::percentage(-200.0f64);
    assert_ulps_eq!(angle.as_radians().value, 2.0 * -std::f64::consts::TAU);
}

#[test]
fn percentage_as_degrees32() {
    let angle = Angle::percentage(100.0f32);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = Angle::percentage(-200.0f32);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn percentage_as_degrees64() {
    let angle = Angle::percentage(100.0f64);
    assert_ulps_eq!(angle.as_degrees().value, 360.0);
    let angle = Angle::percentage(-200.0f64);
    assert_ulps_eq!(angle.as_degrees().value, -720.0);
}

#[test]
fn percentage_as_rotations32() {
    let angle = Angle::percentage(100.0f32);
    assert_ulps_eq!(angle.as_rotations().value, 1.0);
    let angle = Angle::percentage(-200.0f32);
    assert_ulps_eq!(angle.as_rotations().value, -2.0);
}

#[test]
fn percentage_as_rotations64() {
    let angle = Angle::percentage(100.0f64);
    assert_ulps_eq!(angle.as_rotations().value, 1.0);
    let angle = Angle::percentage(-200.0f64);
    assert_ulps_eq!(angle.as_rotations().value, -2.0);
}

#[test]
fn convert_percentage_to_radians() {
    let angle: AngleInRadians = Angle::percentage(25.0).convert();
    assert_ulps_eq!(angle.value, FRAC_PI_2);
}

#[test]
fn convert_percentage_to_degrees() {
    let angle: AngleInDegrees = Angle::percentage(25.0).convert();
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_percentage_to_rotations() {
    let angle: AngleInRotations = Angle::percentage(25.0).convert();
    assert_ulps_eq!(angle.value, 0.25);
}

#[test]
fn convert_percentage_to_percentages() {
    let angle: AngleInPercentage = Angle::percentage(42.0).convert();
    assert_ulps_eq!(angle.value, 42.0);
}

#[test]
fn convert_percentage_from_radians() {
    let angle = AngleInPercentage::from_other(Angle::radians(FRAC_PI_2));
    assert_ulps_eq!(angle.value, 25.0, max_ulps = 10);
}

#[test]
fn convert_percentage_from_degrees() {
    let angle = AngleInPercentage::from_other(Angle::degrees(90.0));
    assert_ulps_eq!(angle.value, 25.0);
}

#[test]
fn convert_percentage_from_rotations() {
    let angle = AngleInPercentage::from_other(Angle::rotations(0.25));
    assert_ulps_eq!(angle.value, 25.0);
}

#[test]
fn convert_percentage_from_percentages() {
    let angle = AngleInPercentage::from_other(Angle::percentage(42.0));
    assert_ulps_eq!(angle.value, 42.0);
}

use approx::assert_ulps_eq;
use bangle::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, ConvertAngle,
    FromOther,
};
use core::f32::consts::FRAC_PI_2;
