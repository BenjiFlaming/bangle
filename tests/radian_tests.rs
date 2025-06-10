#[test]
fn radians_by_type_alias() {
    let angle_by_type = AngleInRadians::new(FRAC_PI_2_32);
    let angle_by_function = Angle::radians(FRAC_PI_2_32);
    assert_ulps_eq!(angle_by_type.value, angle_by_function.value);
}

#[test]
fn radians_as_degrees32() {
    let angle = Angle::radians(FRAC_PI_2_32);
    assert_ulps_eq!(angle.as_degrees().value, 90.0);
}

#[test]
fn radians_as_degrees64() {
    let angle = Angle::radians(FRAC_PI_2_64);
    assert_ulps_eq!(angle.as_degrees().value, 90.0);
}

#[test]
fn radians_as_rotations32() {
    let angle = Angle::radians(1.0f32);
    assert_ulps_eq!(angle.as_rotations().value, 0.15915482);
}

#[test]
fn radians_as_rotations64() {
    let angle = Angle::radians(1.0f64);
    assert_ulps_eq!(angle.as_rotations().value, 0.15915482422145);
}

#[test]
fn radians_as_percentage32() {
    let angle = Angle::radians(1.0f32);
    assert_ulps_eq!(angle.as_percentage().value, 15.915482);
}

#[test]
fn radians_as_percentage64() {
    let angle = Angle::radians(1.0f64);
    assert_ulps_eq!(angle.as_percentage().value, 15.915482422145);
}

#[test]
fn convert_radians_to_radians() {
    let angle: AngleInRadians = Angle::radians(FRAC_PI_2_32).convert();
    assert_ulps_eq!(angle.value, FRAC_PI_2_32);
}

#[test]
fn convert_radians_to_degrees() {
    let angle: AngleInDegrees = Angle::radians(FRAC_PI_2_32).convert();
    assert_ulps_eq!(angle.value, 90.0);
}

#[test]
fn convert_radians_to_rotations() {
    let angle: AngleInRotations = Angle::radians(1.0).convert();
    assert_ulps_eq!(angle.value, 0.15915482);
}

#[test]
fn convert_radians_to_percentages() {
    let angle: AngleInPercentage = Angle::radians(1.0).convert();
    assert_ulps_eq!(angle.value, 15.915482);
}

#[test]
fn convert_radians_from_radians() {
    let angle = AngleInRadians::from_other(Angle::radians(FRAC_PI_2_32));
    assert_ulps_eq!(angle.value, FRAC_PI_2_32);
}

#[test]
fn convert_radians_from_degrees() {
    let angle = AngleInRadians::from_other(Angle::degrees(90.0));
    assert_ulps_eq!(angle.value, FRAC_PI_2_32);
}

#[test]
fn convert_radians_from_rotations() {
    let angle = AngleInRadians::from_other(Angle::rotations(0.25));
    assert_ulps_eq!(angle.value, FRAC_PI_2_32);
}

#[test]
fn convert_radians_from_percentages() {
    let angle = AngleInRadians::from_other(Angle::percentage(25.0));
    assert_ulps_eq!(angle.value, FRAC_PI_2_32);
}

use approx::assert_ulps_eq;
use bangle::{
    Angle, AngleInDegrees, AngleInPercentage, AngleInRadians, AngleInRotations, ConvertAngle,
    FromOther,
};
use core::{f32::consts::FRAC_PI_2 as FRAC_PI_2_32, f64::consts::FRAC_PI_2 as FRAC_PI_2_64};
